//! Provenance acquisition via the GitHub REST API.
//!
//! For each repository cluster we resolve a consistent snapshot:
//!   1. branch HEAD → commit SHA,
//!   2. per file, its blob SHA + content, pinned to that commit.
//!
//! Everything is pinned to the resolved commit, so content is race-free even if
//! upstream moves mid-resolution.  For `--lock`, blobs are fetched directly by
//! their pinned SHA (content-addressed), skipping steps 1–2.
//!
//! Files are resolved with the **Contents API** (one call per file, returning
//! the blob SHA and — for files ≤1 MB — inline content) rather than a recursive
//! tree walk, because large repos (`google/angle`, `Vulkan-Docs`) can truncate
//! recursive trees and silently drop entries.

use anyhow::{Context, Result, anyhow, bail};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use serde_json::Value;

use super::{Cluster, ResolvedFile, ResolvedRepo};

const API_BASE: &str = "https://api.github.com";

/// One cluster resolved at a snapshot: repo provenance plus each requested
/// file's provenance and content.
pub struct ClusterFetch {
    pub repo: ResolvedRepo,
    pub files: Vec<(ResolvedFile, Vec<u8>)>,
}

/// A thin GitHub REST API client.
pub struct Github {
    client: reqwest::blocking::Client,
    token: Option<String>,
    /// API base URL (overridable in tests to point at a mock server).
    base: String,
}

impl Github {
    /// Construct a client, reading an auth token from `$GITHUB_TOKEN` when set.
    pub fn new() -> Result<Self> {
        Self::build(API_BASE.to_string())
    }

    /// Construct a client against an alternate API base URL — for tests that
    /// point at a local mock GitHub server.
    pub fn with_base_url(base: impl Into<String>) -> Result<Self> {
        Self::build(base.into())
    }

    fn build(base: String) -> Result<Self> {
        let token = std::env::var("GITHUB_TOKEN").ok().filter(|t| !t.is_empty());
        let client = reqwest::blocking::Client::builder()
            .user_agent(concat!("gloam/", env!("CARGO_PKG_VERSION")))
            .build()
            .context("building HTTP client")?;
        Ok(Self {
            client,
            token,
            base,
        })
    }

    // -- low-level requests --------------------------------------------------

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

    // -- snapshot resolution -------------------------------------------------

    /// Resolve the HEAD commit SHA of a branch.
    pub fn head_commit(&self, repo: &str, branch: &str) -> Result<String> {
        let base = &self.base;
        let url = format!("{base}/repos/{repo}/git/ref/heads/{branch}");
        let v = self.get_json(&url)?;
        v["object"]["sha"]
            .as_str()
            .map(str::to_string)
            .ok_or_else(|| anyhow!("no object.sha in ref response for {repo}@{branch}"))
    }

    /// Resolve a file's blob SHA (and inline content if the API returned it)
    /// at a specific commit, via the Contents API.  Content is `None` for
    /// files over the API's 1 MB inline limit — callers decide whether the
    /// blob is worth downloading (the engine checks its cache by SHA first).
    pub(crate) fn contents(
        &self,
        repo: &str,
        path: &str,
        commit: &str,
    ) -> Result<(String, Option<Vec<u8>>)> {
        let base = &self.base;
        let url = format!("{base}/repos/{repo}/contents/{path}?ref={commit}");
        let v = self.get_json(&url)?;
        let sha = v["sha"]
            .as_str()
            .ok_or_else(|| anyhow!("no sha for {repo}:{path}@{commit}"))?
            .to_string();
        let content = match v["encoding"].as_str() {
            Some("base64") => Some(decode_b64(v["content"].as_str().unwrap_or(""))?),
            _ => None, // e.g. "none" for files > 1 MB; fetch the blob separately
        };
        Ok((sha, content))
    }

    /// Fetch a blob's content by its SHA (content-addressed; used for large
    /// files and for `--lock`).
    pub fn blob_content(&self, repo: &str, blob_sha: &str) -> Result<Vec<u8>> {
        let base = &self.base;
        let url = format!("{base}/repos/{repo}/git/blobs/{blob_sha}");
        let v = self.get_json(&url)?;
        match v["encoding"].as_str() {
            Some("base64") => decode_b64(v["content"].as_str().unwrap_or("")),
            other => bail!("unexpected blob encoding {other:?} for {repo}:{blob_sha}"),
        }
    }

    /// Resolve a whole cluster at HEAD: repo provenance plus the requested files
    /// (by registry key) with content.  `keys` selects which of the cluster's
    /// files to fetch.
    pub fn resolve_cluster_head(&self, cluster: &Cluster, keys: &[&str]) -> Result<ClusterFetch> {
        let commit = self
            .head_commit(cluster.repo, cluster.branch)
            .with_context(|| format!("resolving HEAD of {}", cluster.repo))?;

        let repo = ResolvedRepo {
            repo: cluster.repo.to_string(),
            repo_url: cluster.repo_url.to_string(),
            branch: cluster.branch.to_string(),
            commit: commit.clone(),
        };

        let mut files = Vec::new();
        for &key in keys {
            let spec = cluster
                .files
                .iter()
                .find(|f| f.key == key)
                .ok_or_else(|| anyhow!("key {key} not in cluster {}", cluster.repo))?;
            let (blob, inline) = self
                .contents(cluster.repo, spec.path_in_repo, &commit)
                .with_context(|| format!("resolving {}", spec.path_in_repo))?;
            let content = match inline {
                Some(c) => c,
                None => self
                    .blob_content(cluster.repo, &blob)
                    .with_context(|| format!("fetching blob for {}", spec.path_in_repo))?,
            };
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
/// returns).
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
