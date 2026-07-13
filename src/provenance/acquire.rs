//! Provenance acquisition over a cluster's fetch endpoints.
//!
//! Two endpoint dialects are spoken (see [`Endpoint`]):
//!
//! - **GitHub**: the REST API for metadata (branch ref → commit, Contents
//!   directory listings → blob SHAs, blobs-by-SHA) plus
//!   `raw.githubusercontent.com` for content (unmetered).  Ref resolution is
//!   conditional: an `If-None-Match` ETag turns an unchanged ref into a
//!   rate-limit-free 304.
//! - **Gitiles** (e.g. `chromium.googlesource.com`): `+log?n=1&format=JSON`
//!   for the branch tip, `+/{commit}/{dir}?format=JSON` for listings (entries
//!   carry git object types — files are `"blob"` — and `id` is the git blob
//!   SHA-1), and `+/{commit}/{path}?format=TEXT` for base64 content.  Every
//!   JSON response starts with the `)]}'` anti-XSSI line, no ETags are
//!   served, and no auth of any kind is sent.
//!
//! For each repository cluster we resolve a consistent snapshot:
//!   1. branch HEAD → commit SHA,
//!   2. per changed directory, one listing yields every file's blob SHA
//!      pinned to that commit,
//!   3. content is fetched at that commit and verified locally against the
//!      listed blob SHA.
//!
//! Everything is pinned to the resolved commit, so content is race-free even
//! if upstream moves mid-resolution.  For `--lock`, content is fetched by
//! pinned commit + path (GitHub blobs API as final fallback), skipping steps
//! 1–2.
//!
//! Directory listings are used rather than a recursive tree walk because
//! large repos (`google/angle`, `Vulkan-Docs`) can truncate recursive trees
//! and silently drop entries; per-directory listings cap at 1000 entries, far
//! above any tracked directory.

use std::collections::BTreeMap;

use anyhow::{Context, Result, anyhow, bail};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use serde_json::Value;

use super::manifest::git_blob_sha1;
use super::{Cluster, Endpoint, ResolvedFile, ResolvedRepo};

const API_BASE: &str = "https://api.github.com";
const RAW_BASE: &str = "https://raw.githubusercontent.com";

/// One cluster resolved at a snapshot: repo provenance plus each requested
/// file's provenance and content.
pub struct ClusterFetch {
    pub repo: ResolvedRepo,
    pub files: Vec<(ResolvedFile, Vec<u8>)>,
}

/// Outcome of a conditional branch-ref resolution.
pub enum HeadRef {
    /// HTTP 304: whatever commit the caller derived from this URL last time
    /// still holds (and the round trip did not count against the rate limit).
    NotModified,
    /// HTTP 200: a (possibly new) tip commit, plus the response ETag to send
    /// as `If-None-Match` next time.  Gitiles serves no ETag, so its
    /// resolutions always carry `etag: None`.
    Resolved {
        commit: String,
        etag: Option<String>,
    },
}

/// Try `op` against each endpoint in order; a failure warns (naming the
/// cluster, the endpoint dialect, and the error) and falls over to the next
/// endpoint.  When every endpoint failed, the last error is returned —
/// callers add operation context.  The warning is skipped when there is no
/// next endpoint to fall over to (single-endpoint clusters fail exactly as
/// they did before endpoints existed).
pub(crate) fn try_endpoints<'e, T, I, F>(repo: &str, what: &str, endpoints: I, mut op: F) -> Result<T>
where
    I: IntoIterator<Item = &'e Endpoint>,
    F: FnMut(&'e Endpoint) -> Result<T>,
{
    let mut last: Option<anyhow::Error> = None;
    let mut it = endpoints.into_iter().peekable();
    while let Some(ep) = it.next() {
        match op(ep) {
            Ok(v) => return Ok(v),
            Err(e) => {
                if it.peek().is_some() {
                    crate::diag::warn(format_args!(
                        "{repo}: {} endpoint failed while {what} ({e:#}); \
                         trying the next endpoint",
                        ep.kind()
                    ));
                }
                last = Some(e);
            }
        }
    }
    Err(last.unwrap_or_else(|| anyhow!("cluster {repo} has no fetch endpoints")))
}

/// The parent directory of a path within a repository (`""` for root-level
/// files) — the unit at which content lookups are grouped into directory
/// listings.
pub(crate) fn parent_dir(path: &str) -> &str {
    match path.rfind('/') {
        Some(i) => &path[..i],
        None => "",
    }
}

/// A thin HTTP client speaking the GitHub REST API / raw-content-host and
/// Gitiles dialects.
pub struct Github {
    client: reqwest::blocking::Client,
    token: Option<String>,
    /// GitHub API base URL (overridable in tests to point at a mock server).
    base: String,
    /// Raw-content host base URL (`raw.githubusercontent.com`; overridable in
    /// tests).  Serves plain file bytes without touching the metered API.
    raw_base: String,
}

impl Github {
    /// Construct a client, reading an auth token from `$GITHUB_TOKEN` when set.
    pub fn new() -> Result<Self> {
        Self::build(API_BASE.to_string(), RAW_BASE.to_string())
    }

    /// Construct a client against alternate API and raw-content base URLs —
    /// for tests that point at a local mock GitHub server.  (Gitiles endpoint
    /// URLs live in the cluster's endpoint list, so tests point those at the
    /// mock via synthetic clusters instead.)
    pub fn with_base_urls(
        api_base: impl Into<String>,
        raw_base: impl Into<String>,
    ) -> Result<Self> {
        Self::build(api_base.into(), raw_base.into())
    }

    fn build(base: String, raw_base: String) -> Result<Self> {
        let token = std::env::var("GITHUB_TOKEN").ok().filter(|t| !t.is_empty());
        let client = reqwest::blocking::Client::builder()
            .user_agent(concat!("gloam/", env!("CARGO_PKG_VERSION")))
            .build()
            .context("building HTTP client")?;
        Ok(Self {
            client,
            token,
            base,
            raw_base,
        })
    }

    // -- low-level requests --------------------------------------------------

    /// GET with GitHub API headers and the auth token (metered API host only).
    fn get(&self, url: &str) -> Result<reqwest::blocking::Response> {
        let started = std::time::Instant::now();
        let mut req = self
            .client
            .get(url)
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28");
        if let Some(tok) = &self.token {
            req = req.bearer_auth(tok);
        }
        let resp = req.send().with_context(|| format!("GET {url}"))?;
        crate::diag::debug(format_args!(
            "HTTP GET {url} -> {} in {}ms",
            resp.status(),
            started.elapsed().as_millis()
        ));
        let resp = resp
            .error_for_status()
            .with_context(|| format!("HTTP error from {url}"))?;
        Ok(resp)
    }

    fn get_json(&self, url: &str) -> Result<Value> {
        let text = self
            .get(url)?
            .text()
            .with_context(|| format!("body of {url}"))?;
        serde_json::from_str(&text).with_context(|| format!("parsing JSON from {url}"))
    }

    /// Auth-less GET: no GitHub API headers and — deliberately — no token.
    /// Used for the raw-content host and for every Gitiles request: the API
    /// credential must never be sent cross-host, and no auth of any kind is
    /// sent to Gitiles.
    fn get_plain(&self, url: &str) -> Result<reqwest::blocking::Response> {
        let started = std::time::Instant::now();
        let resp = self
            .client
            .get(url)
            .send()
            .with_context(|| format!("GET {url}"))?;
        crate::diag::debug(format_args!(
            "HTTP GET {url} -> {} in {}ms",
            resp.status(),
            started.elapsed().as_millis()
        ));
        resp.error_for_status()
            .with_context(|| format!("HTTP error from {url}"))
    }

    /// GET a Gitiles JSON response: strip the mandatory `)]}'` anti-XSSI
    /// prefix line, then parse.
    fn gitiles_json(&self, url: &str) -> Result<Value> {
        let text = self
            .get_plain(url)?
            .text()
            .with_context(|| format!("body of {url}"))?;
        let Some(json) = text.strip_prefix(")]}'") else {
            bail!("Gitiles response from {url} lacks the )]}}' anti-XSSI prefix");
        };
        serde_json::from_str(json).with_context(|| format!("parsing JSON from {url}"))
    }

    // -- endpoint primitives ---------------------------------------------------

    /// The exact ref-resolution request URL for a branch at an endpoint —
    /// also the key under which the engine stores this request's ETag (an
    /// ETag belongs to a request, so cache rows are keyed by the URL the
    /// request actually used, which is naturally per-endpoint).
    pub fn ref_url(&self, ep: &Endpoint, branch: &str) -> String {
        match ep {
            Endpoint::GitHub { slug } => {
                // Must match the URL github_head() actually requests.
                let base = &self.base;
                format!("{base}/repos/{slug}/git/ref/heads/{branch}")
            }
            Endpoint::Gitiles { base } => {
                format!("{base}/+log/refs/heads/{branch}?n=1&format=JSON")
            }
        }
    }

    /// Resolve the HEAD commit SHA of a branch at an endpoint, conditionally.
    ///
    /// For GitHub endpoints, when `etag` is given it is sent as
    /// `If-None-Match`; a 304 response (which GitHub does not count against
    /// the rate limit) yields [`HeadRef::NotModified`] — the caller's
    /// previously derived commit still holds.  A 200 yields the commit plus
    /// the response's ETag for the next conditional round trip.
    ///
    /// Gitiles serves no ETag header, so its resolution is always
    /// unconditional and always yields `etag: None`.
    pub fn head_commit_conditional(
        &self,
        ep: &Endpoint,
        branch: &str,
        etag: Option<&str>,
    ) -> Result<HeadRef> {
        match ep {
            Endpoint::GitHub { slug } => self.github_head(slug, branch, etag),
            Endpoint::Gitiles { .. } => {
                let url = self.ref_url(ep, branch);
                let v = self.gitiles_json(&url)?;
                let commit = v["log"][0]["commit"]
                    .as_str()
                    .map(str::to_string)
                    .ok_or_else(|| anyhow!("no log[0].commit in Gitiles log response from {url}"))?;
                Ok(HeadRef::Resolved { commit, etag: None })
            }
        }
    }

    fn github_head(&self, slug: &str, branch: &str, etag: Option<&str>) -> Result<HeadRef> {
        let base = &self.base;
        let url = format!("{base}/repos/{slug}/git/ref/heads/{branch}");
        let started = std::time::Instant::now();
        let mut req = self
            .client
            .get(&url)
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28");
        if let Some(tok) = &self.token {
            req = req.bearer_auth(tok);
        }
        if let Some(etag) = etag {
            req = req.header("If-None-Match", etag);
        }
        let resp = req.send().with_context(|| format!("GET {url}"))?;
        crate::diag::debug(format_args!(
            "HTTP GET {url} -> {} in {}ms",
            resp.status(),
            started.elapsed().as_millis()
        ));
        if resp.status() == reqwest::StatusCode::NOT_MODIFIED {
            return Ok(HeadRef::NotModified);
        }
        let new_etag = resp
            .headers()
            .get(reqwest::header::ETAG)
            .and_then(|v| v.to_str().ok())
            .map(str::to_string);
        let resp = resp
            .error_for_status()
            .with_context(|| format!("HTTP error from {url}"))?;
        let text = resp.text().with_context(|| format!("body of {url}"))?;
        let v: Value =
            serde_json::from_str(&text).with_context(|| format!("parsing JSON from {url}"))?;
        let commit = v["object"]["sha"]
            .as_str()
            .map(str::to_string)
            .ok_or_else(|| anyhow!("no object.sha in ref response for {slug}@{branch}"))?;
        Ok(HeadRef::Resolved {
            commit,
            etag: new_etag,
        })
    }

    /// Resolve the HEAD commit SHA of a branch at an endpoint (unconditional).
    pub fn head_commit(&self, ep: &Endpoint, branch: &str) -> Result<String> {
        match self.head_commit_conditional(ep, branch, None)? {
            HeadRef::Resolved { commit, .. } => Ok(commit),
            HeadRef::NotModified => {
                bail!("unexpected 304 for unconditional ref request at branch {branch}")
            }
        }
    }

    /// List a directory at a specific commit: one call yields
    /// `(path_in_repo, blob_sha)` for every file entry.  `dir` may be `""`
    /// for the repository root.  Subdirectory entries are skipped — callers
    /// list each directory they need individually.
    pub fn contents_dir(
        &self,
        ep: &Endpoint,
        dir: &str,
        commit: &str,
    ) -> Result<Vec<(String, String)>> {
        match ep {
            Endpoint::GitHub { slug } => {
                let base = &self.base;
                let url = if dir.is_empty() {
                    format!("{base}/repos/{slug}/contents?ref={commit}")
                } else {
                    format!("{base}/repos/{slug}/contents/{dir}?ref={commit}")
                };
                let v = self.get_json(&url)?;
                let entries = v.as_array().ok_or_else(|| {
                    anyhow!("directory listing for {slug}:{dir}@{commit} is not a JSON array")
                })?;
                let mut out = Vec::new();
                for entry in entries {
                    if entry["type"].as_str() != Some("file") {
                        continue;
                    }
                    let (Some(path), Some(sha)) = (entry["path"].as_str(), entry["sha"].as_str())
                    else {
                        continue;
                    };
                    out.push((path.to_string(), sha.to_string()));
                }
                Ok(out)
            }
            Endpoint::Gitiles { base } => {
                let url = if dir.is_empty() {
                    format!("{base}/+/{commit}/?format=JSON")
                } else {
                    format!("{base}/+/{commit}/{dir}?format=JSON")
                };
                let v = self.gitiles_json(&url)?;
                let entries = v["entries"]
                    .as_array()
                    .ok_or_else(|| anyhow!("no entries array in Gitiles listing from {url}"))?;
                let mut out = Vec::new();
                for entry in entries {
                    // Gitiles speaks git object types: file entries are
                    // "blob" (not the GitHub Contents dialect's "file"), and
                    // `id` is the git blob SHA-1.
                    if entry["type"].as_str() != Some("blob") {
                        continue;
                    }
                    let (Some(name), Some(id)) = (entry["name"].as_str(), entry["id"].as_str())
                    else {
                        continue;
                    };
                    // Entry names are bare filenames; join with the listed
                    // directory to form the path within the repository.
                    let path = if dir.is_empty() {
                        name.to_string()
                    } else {
                        format!("{dir}/{name}")
                    };
                    out.push((path, id.to_string()));
                }
                Ok(out)
            }
        }
    }

    /// Fetch a file's bytes at a specific commit from an endpoint's content
    /// route (GitHub's unmetered raw host, or Gitiles `?format=TEXT`).
    /// Callers must verify the bytes against a blob SHA learned from a
    /// listing (or a pin) before trusting them.
    pub fn content_at(&self, ep: &Endpoint, commit: &str, path: &str) -> Result<Vec<u8>> {
        match ep {
            Endpoint::GitHub { slug } => self.raw_content(slug, commit, path),
            Endpoint::Gitiles { base } => {
                let url = format!("{base}/+/{commit}/{path}?format=TEXT");
                let text = self
                    .get_plain(&url)?
                    .text()
                    .with_context(|| format!("body of {url}"))?;
                // format=TEXT bodies are base64 of the raw file bytes (with
                // embedded newlines).
                decode_b64(&text)
            }
        }
    }

    /// Fetch a file's bytes at a specific commit from the raw-content host
    /// (`raw.githubusercontent.com`) — unmetered, so it never counts against
    /// the API rate limit.
    pub fn raw_content(&self, slug: &str, commit: &str, path: &str) -> Result<Vec<u8>> {
        let raw_base = &self.raw_base;
        let url = format!("{raw_base}/{slug}/{commit}/{path}");
        Ok(self
            .get_plain(&url)?
            .bytes()
            .with_context(|| format!("body of {url}"))?
            .to_vec())
    }

    /// Fetch a blob's content by its SHA via the GitHub blobs API
    /// (content-addressed; the `--lock` fallback when no endpoint can serve
    /// the pinned commit).  GitHub dialect only — Gitiles has no blob-by-SHA
    /// endpoint.
    pub fn blob_content(&self, slug: &str, blob_sha: &str) -> Result<Vec<u8>> {
        let base = &self.base;
        let url = format!("{base}/repos/{slug}/git/blobs/{blob_sha}");
        let v = self.get_json(&url)?;
        match v["encoding"].as_str() {
            Some("base64") => decode_b64(v["content"].as_str().unwrap_or("")),
            other => bail!("unexpected blob encoding {other:?} for {slug}:{blob_sha}"),
        }
    }

    /// Resolve a whole cluster at HEAD: repo provenance plus the requested
    /// files (by registry key) with content, trying the cluster's endpoints
    /// in order for every operation.  `keys` selects which of the cluster's
    /// files to fetch.  Used by the xtask bundler (no cache; bundle runs are
    /// rare) — the engine resolves through the cache instead.
    pub fn resolve_cluster_head(&self, cluster: &Cluster, keys: &[&str]) -> Result<ClusterFetch> {
        let commit = try_endpoints(
            cluster.repo,
            "resolving branch HEAD",
            cluster.endpoints,
            |ep| self.head_commit(ep, cluster.branch),
        )
        .with_context(|| format!("resolving HEAD of {}", cluster.repo))?;

        let repo = ResolvedRepo {
            repo: cluster.repo.to_string(),
            repo_url: cluster.repo_url.to_string(),
            branch: cluster.branch.to_string(),
            commit: commit.clone(),
        };

        // Validate the requested keys and collect their specs.
        let mut specs = Vec::new();
        for &key in keys {
            let spec = cluster
                .files
                .iter()
                .find(|f| f.key == key)
                .ok_or_else(|| anyhow!("key {key} not in cluster {}", cluster.repo))?;
            specs.push(spec);
        }

        // One listing per distinct parent directory → blob SHA per path.
        let mut dirs: Vec<&str> = Vec::new();
        for spec in &specs {
            let dir = parent_dir(spec.path_in_repo);
            if !dirs.contains(&dir) {
                dirs.push(dir);
            }
        }
        let mut listed: BTreeMap<String, String> = BTreeMap::new();
        for dir in dirs {
            let entries = try_endpoints(
                cluster.repo,
                "listing a directory",
                cluster.endpoints,
                |ep| self.contents_dir(ep, dir, &commit),
            )
            .with_context(|| format!("listing {}:{dir} at {commit}", cluster.repo))?;
            listed.extend(entries);
        }

        let mut files = Vec::new();
        for spec in specs {
            let blob = listed.get(spec.path_in_repo).cloned().ok_or_else(|| {
                anyhow!(
                    "{} is missing from the directory listing of {} at {commit} \
                     — tracked file absent upstream",
                    spec.path_in_repo,
                    cluster.repo
                )
            })?;
            let content = try_endpoints(
                cluster.repo,
                "fetching content",
                cluster.endpoints,
                |ep| self.content_at(ep, &commit, spec.path_in_repo),
            )
            .with_context(|| format!("fetching {} from {}", spec.path_in_repo, cluster.repo))?;
            let got = git_blob_sha1(&content);
            if got != blob {
                bail!(
                    "content of {} from {} hashes to {got}, but the directory \
                     listing reported blob {blob} — refusing corrupt or \
                     mismatched content",
                    spec.path_in_repo,
                    cluster.repo
                );
            }
            files.push((
                ResolvedFile {
                    key: spec.key.to_string(),
                    path_in_repo: spec.path_in_repo.to_string(),
                    blob,
                },
                content,
            ));
        }

        Ok(ClusterFetch { repo, files })
    }
}

/// Decode base64 that may contain embedded newlines (as the git blobs API
/// and Gitiles `format=TEXT` return).
fn decode_b64(s: &str) -> Result<Vec<u8>> {
    let cleaned: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    BASE64
        .decode(cleaned.as_bytes())
        .context("decoding base64 content")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Live end-to-end check against the GitHub API.  Network + ideally a
    /// `GITHUB_TOKEN` (to avoid the 60/hr unauthenticated limit) are required,
    /// so it is `#[ignore]`d by default.  Run with:
    ///   cargo test --features fetch -- --ignored resolves_xxhash_cluster
    #[test]
    #[ignore]
    fn resolves_xxhash_cluster() {
        let gh = Github::new().expect("client");
        let (cluster, _) = super::super::find("xxhash.h").unwrap();
        let fetched = gh
            .resolve_cluster_head(cluster, &["xxhash.h"])
            .expect("resolve xxHash");

        assert_eq!(fetched.repo.repo, "Cyan4973/xxHash");
        assert_eq!(fetched.repo.commit.len(), 40, "full commit SHA-1");

        let (file, content) = &fetched.files[0];
        assert_eq!(file.key, "xxhash.h");
        assert_eq!(file.blob.len(), 40, "full blob SHA-1");
        assert!(
            content.windows(6).any(|w| w == b"xxHash"),
            "fetched xxhash.h should mention xxHash"
        );
    }
}
