//! Unified fetch engine: combines GitHub acquisition with the SQLite cache to
//! resolve a set of registry files to provenance pins + content.
//!
//! Two resolution modes:
//!   - [`Engine::resolve_head`] — no-lock `--fetch`: resolve each cluster's
//!     branch HEAD (cache-first, governed by the HEAD TTL) and fetch the needed
//!     files at that commit.
//!   - [`Engine::resolve_pinned`] — `--lock`: fetch each file by its pinned blob
//!     SHA (content-addressed, cache-first), never touching HEAD.
//!
//! [`Engine::seed_from_bundle`] preloads the cache from the embedded bundle so
//! bundled content is found via the single cache lookup path (it records
//! commits/blobs/tree-entries but not HEAD — the bundled commit isn't HEAD).

use std::collections::BTreeMap;

use anyhow::{Context, Result, anyhow, bail};
use indexmap::IndexMap;

use super::acquire::{Github, HeadRef};
use super::cache::{self, Cache};
use super::manifest::{BundledProvenance, ProvenancePin, git_blob_sha1};
use super::{Cluster, find};

/// A resolved file: its provenance pin plus content bytes.
#[derive(Debug)]
pub struct Resolved {
    pub pin: ProvenancePin,
    pub content: Vec<u8>,
}

pub struct Engine {
    gh: Github,
    cache: Cache,
    head_ttl: i64,
    object_ttl: i64,
}

impl Engine {
    /// Build an engine using the platform-default cache and `$GITHUB_TOKEN`.
    pub fn new() -> Result<Self> {
        Ok(Self::from_parts(Github::new()?, Cache::open_default()?))
    }

    /// Build from explicit parts (used in tests with an in-memory cache).
    pub fn from_parts(gh: Github, cache: Cache) -> Self {
        Self {
            gh,
            cache,
            head_ttl: cache::HEAD_TTL_SECS,
            object_ttl: cache::OBJECT_TTL_SECS,
        }
    }

    /// Seed the cache from an embedded bundle: `content_for(key)` yields the
    /// embedded bytes for a registry key.  Records commit/describe/tree/blob
    /// rows (not HEAD).  Skips any pin whose content is unavailable.
    ///
    /// `put_blob` is idempotent, but binding multi-MB content per call is the
    /// dominant cost of repeated seeding — blobs already present (they're
    /// content-addressed, so presence is sufficient) get only the cheap
    /// metadata refresh.  The skipped `last_used` bump is harmless: any blob
    /// actually read is bumped by the read, and an evicted-but-needed blob
    /// reseeds on the next pass.
    pub fn seed_from_bundle<F>(&self, bundle: &BundledProvenance, content_for: F) -> Result<()>
    where
        F: Fn(&str) -> Option<Vec<u8>>,
    {
        let now = cache::now();
        // One transaction for the whole seed: the metadata refreshes are
        // dozens of small writes, and one WAL commit per statement is the
        // slow path.
        let tx = self.cache.transaction()?;
        for (key, pin) in &bundle.provenance {
            if self.cache.has_blob(&pin.blob)? {
                self.cache.put_commit(&pin.commit, &pin.repo, now)?;
                self.cache
                    .put_tree_entry(&pin.commit, &pin.path_in_repo, &pin.blob)?;
                continue;
            }
            let Some(content) = content_for(key) else {
                continue;
            };
            self.cache.put_commit(&pin.commit, &pin.repo, now)?;
            self.cache
                .put_tree_entry(&pin.commit, &pin.path_in_repo, &pin.blob)?;
            self.cache.put_blob(&pin.blob, &content, now)?;
        }
        tx.commit()?;
        Ok(())
    }

    /// Resolve registry keys at upstream HEAD (no-lock `--fetch`).
    pub fn resolve_head(&self, keys: &[&str]) -> Result<BTreeMap<String, Resolved>> {
        let by_cluster = group_by_cluster(keys)?;
        let mut out = BTreeMap::new();
        let now = cache::now();

        for (cluster, cluster_keys) in by_cluster {
            // Resolve the cluster's HEAD commit (cache-first, then a
            // conditional request).
            let commit = match self.cache.fresh_head(cluster.repo, now, self.head_ttl)? {
                Some(c) => c,
                None => self.resolve_stale_head(cluster, now)?,
            };
            self.cache.put_commit(&commit, cluster.repo, now)?;

            // Cache-first per file: a tree entry at this commit plus verified
            // blob content means zero network.  Everything else is a miss.
            let mut misses: Vec<&super::FileSpec> = Vec::new();
            for key in cluster_keys {
                let spec = cluster.files.iter().find(|f| f.key == key).unwrap();
                let hit = match self.cache.blob_for_path(&commit, spec.path_in_repo)? {
                    Some(blob) => self
                        .verified_cached_blob(&blob, now)?
                        .map(|content| (blob, content)),
                    None => None,
                };
                match hit {
                    Some((blob, content)) => {
                        out.insert(
                            key.to_string(),
                            Resolved {
                                pin: pin_for(cluster, spec.path_in_repo, &commit, &blob),
                                content,
                            },
                        );
                    }
                    None => misses.push(spec),
                }
            }
            if !misses.is_empty() {
                self.fetch_misses(cluster, &commit, &misses, now, &mut out)?;
            }
        }

        self.cache.evict(now, self.object_ttl)?;
        Ok(out)
    }

    /// Resolve registry keys against an explicit pin set (`--lock`).  Errors if
    /// a requested key has no pin (the caller surfaces the "regenerate without
    /// --lock" guidance).
    pub fn resolve_pinned(
        &self,
        pins: &IndexMap<String, ProvenancePin>,
        keys: &[&str],
    ) -> Result<BTreeMap<String, Resolved>> {
        let mut out = BTreeMap::new();
        let now = cache::now();

        for &key in keys {
            let pin = pins
                .get(key)
                .ok_or_else(|| anyhow!("manifest has no provenance for required file '{key}'"))?;

            let content = match self.verified_cached_blob(&pin.blob, now)? {
                Some(c) => c,
                None => {
                    // Raw host first (unmetered): the pin names the commit
                    // and path, and the bytes are acceptable only if they
                    // hash to the pinned blob.  A failed or mismatching raw
                    // fetch (e.g. the pinned commit was garbage-collected
                    // upstream) falls back to the content-addressed blobs
                    // API.
                    let raw = self
                        .gh
                        .raw_content(&pin.repo, &pin.commit, &pin.path_in_repo)
                        .ok()
                        .filter(|c| git_blob_sha1(c) == pin.blob);
                    let c = match raw {
                        Some(c) => c,
                        None => {
                            let c = self
                                .gh
                                .blob_content(&pin.repo, &pin.blob)
                                .with_context(|| format!("fetching pinned blob for '{key}'"))?;
                            // Never launder wrong content into "verified"
                            // provenance: the fetched bytes must hash to the
                            // pinned blob SHA.
                            let got = git_blob_sha1(&c);
                            if got != pin.blob {
                                bail!(
                                    "pinned blob for '{key}' from {} hashes to {got}, \
                                     expected {} — refusing corrupt or mismatched content",
                                    pin.repo,
                                    pin.blob
                                );
                            }
                            c
                        }
                    };
                    self.cache.put_blob(&pin.blob, &c, now)?;
                    c
                }
            };
            // Opportunistically keep commit/tree metadata warm.
            self.cache.put_commit(&pin.commit, &pin.repo, now)?;
            self.cache
                .put_tree_entry(&pin.commit, &pin.path_in_repo, &pin.blob)?;

            out.insert(
                key.to_string(),
                Resolved {
                    pin: pin.clone(),
                    content,
                },
            );
        }

        Ok(out)
    }

    /// Re-resolve a cluster's HEAD after the TTL lapsed, using a conditional
    /// request so an unchanged upstream costs no metered API call.
    ///
    /// On 304 the stored commit is re-`set_head` with `now` — a
    /// rate-limit-free TTL heartbeat.  On 200 the (possibly new) commit and
    /// the response ETag are recorded for the next round trip.
    fn resolve_stale_head(&self, cluster: &Cluster, now: i64) -> Result<String> {
        let url = self.gh.ref_url(cluster.repo, cluster.branch);
        // A 304 is only actionable if we still know what we derived from this
        // URL last time — never send If-None-Match without a stored commit.
        let stored = self.cache.stored_head(cluster.repo)?;
        let etag = match &stored {
            Some(_) => self.cache.etag_for(&url)?,
            None => None,
        };
        match self
            .gh
            .head_commit_conditional(cluster.repo, cluster.branch, etag.as_deref())?
        {
            HeadRef::NotModified => {
                let c = stored.expect("If-None-Match sent only with a stored head");
                self.cache.set_head(cluster.repo, cluster.branch, &c, now)?;
                Ok(c)
            }
            HeadRef::Resolved { commit, etag } => {
                self.cache
                    .set_head(cluster.repo, cluster.branch, &commit, now)?;
                if let Some(etag) = &etag {
                    self.cache.set_etag(&url, etag, now)?;
                }
                Ok(commit)
            }
        }
    }

    /// Resolve cache misses at `commit`: one Contents-API directory listing
    /// per parent directory yields the blob SHAs, then content comes from the
    /// cache (unchanged blob at a new commit — zero download) or the
    /// unmetered raw host, verified against the listed SHA.
    fn fetch_misses(
        &self,
        cluster: &Cluster,
        commit: &str,
        misses: &[&super::FileSpec],
        now: i64,
        out: &mut BTreeMap<String, Resolved>,
    ) -> Result<()> {
        // One listing per distinct parent directory, in miss order.
        let mut dirs: Vec<&str> = Vec::new();
        for spec in misses {
            let dir = parent_dir(spec.path_in_repo);
            if !dirs.contains(&dir) {
                dirs.push(dir);
            }
        }
        // path_in_repo -> blob SHA learned from the listings.
        let mut listed: BTreeMap<String, String> = BTreeMap::new();
        for dir in dirs {
            let entries = self
                .gh
                .contents_dir(cluster.repo, dir, commit)
                .with_context(|| {
                    format!("listing {}:{dir} at {commit}", cluster.repo)
                })?;
            for (path, sha) in entries {
                // Record tree entries only for files this cluster tracks —
                // rows for untracked listing entries would bloat the cache
                // for nothing.
                if cluster.files.iter().any(|f| f.path_in_repo == path) {
                    self.cache.put_tree_entry(commit, &path, &sha)?;
                    listed.insert(path, sha);
                }
            }
        }

        for spec in misses {
            let blob = listed.get(spec.path_in_repo).ok_or_else(|| {
                anyhow!(
                    "{} is missing from the directory listing of {} at {commit} \
                     — tracked file absent upstream",
                    spec.path_in_repo,
                    cluster.repo
                )
            })?;
            // An advanced commit usually leaves blobs untouched, so check
            // the cache by SHA before paying for the download — re-fetching
            // unchanged content is exactly the redundant transfer this layer
            // exists to avoid.
            let content = match self.verified_cached_blob(blob, now)? {
                Some(c) => c,
                None => {
                    let c = self
                        .gh
                        .raw_content(cluster.repo, commit, spec.path_in_repo)
                        .with_context(|| {
                            format!("fetching {} from {}", spec.path_in_repo, cluster.repo)
                        })?;
                    let got = git_blob_sha1(&c);
                    if got != *blob {
                        bail!(
                            "content of {} from {} hashes to {got}, but the directory \
                             listing reported blob {blob} — refusing corrupt or \
                             mismatched content",
                            spec.path_in_repo,
                            cluster.repo
                        );
                    }
                    self.cache.put_blob(blob, &c, now)?;
                    c
                }
            };
            out.insert(
                spec.key.to_string(),
                Resolved {
                    pin: pin_for(cluster, spec.path_in_repo, commit, blob),
                    content,
                },
            );
        }
        Ok(())
    }

    /// Return cached content for `blob_sha` only if it still hashes to that
    /// SHA.  A mismatch (cache corruption) is treated as a miss; the caller
    /// refetches and `put_blob` overwrites the bad row.  Content correctness
    /// therefore never depends on the storage engine being bug-free.
    fn verified_cached_blob(&self, blob_sha: &str, now: i64) -> Result<Option<Vec<u8>>> {
        Ok(self
            .cache
            .blob(blob_sha, now)?
            .filter(|content| git_blob_sha1(content) == blob_sha))
    }
}

/// The parent directory of a path within a repository (`""` for root-level
/// files) — the unit at which misses are grouped for directory listings.
fn parent_dir(path: &str) -> &str {
    match path.rfind('/') {
        Some(i) => &path[..i],
        None => "",
    }
}

fn pin_for(cluster: &Cluster, path_in_repo: &str, commit: &str, blob: &str) -> ProvenancePin {
    ProvenancePin {
        repo: cluster.repo.to_string(),
        repo_url: cluster.repo_url.to_string(),
        path_in_repo: path_in_repo.to_string(),
        commit: commit.to_string(),
        blob: blob.to_string(),
    }
}

/// Group requested keys by their cluster, preserving registry order.
fn group_by_cluster<'a>(keys: &[&'a str]) -> Result<Vec<(&'static Cluster, Vec<&'a str>)>> {
    let mut groups: Vec<(&'static Cluster, Vec<&'a str>)> = Vec::new();
    for &key in keys {
        let (cluster, _) = find(key).ok_or_else(|| anyhow!("unknown registry key '{key}'"))?;
        match groups.iter_mut().find(|(c, _)| std::ptr::eq(*c, cluster)) {
            Some((_, v)) => v.push(key),
            None => groups.push((cluster, vec![key])),
        }
    }
    Ok(groups)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn engine_in_memory() -> Engine {
        let cache = Cache::from_connection(Connection::open_in_memory().unwrap()).unwrap();
        Engine::from_parts(Github::new().unwrap(), cache)
    }

    const SAMPLE_CONTENT: &[u8] = b"/* xxHash */";

    fn sample_bundle() -> BundledProvenance {
        let mut provenance = IndexMap::new();
        provenance.insert(
            "xxhash.h".to_string(),
            ProvenancePin {
                repo: "Cyan4973/xxHash".to_string(),
                repo_url: "https://github.com/Cyan4973/xxHash".to_string(),
                path_in_repo: "xxhash.h".to_string(),
                commit: "c0ffee00".to_string(),
                // Must be the real hash: the engine re-verifies cached blobs.
                blob: git_blob_sha1(SAMPLE_CONTENT),
            },
        );
        BundledProvenance {
            schema_version: super::super::manifest::SCHEMA_VERSION,
            provenance,
        }
    }

    #[test]
    fn seed_then_resolve_pinned_is_offline() {
        let engine = engine_in_memory();
        let bundle = sample_bundle();
        engine
            .seed_from_bundle(&bundle, |key| {
                (key == "xxhash.h").then(|| SAMPLE_CONTENT.to_vec())
            })
            .unwrap();

        // The blob is cached, so resolution must not touch the network.
        let resolved = engine
            .resolve_pinned(&bundle.provenance, &["xxhash.h"])
            .unwrap();
        let r = &resolved["xxhash.h"];
        assert_eq!(r.content, SAMPLE_CONTENT);
        assert_eq!(r.pin.blob, git_blob_sha1(SAMPLE_CONTENT));
        assert_eq!(r.pin.commit, "c0ffee00");
    }

    #[test]
    fn corrupted_cached_blob_is_not_served() {
        // Seed the cache with content that does NOT hash to the pin's blob
        // SHA (simulating cache corruption).  Resolution must treat it as a
        // miss and attempt a refetch — which fails here because the client
        // points at an unroutable address — rather than silently serving the
        // corrupt bytes.
        let cache = Cache::from_connection(Connection::open_in_memory().unwrap()).unwrap();
        let gh = Github::with_base_urls("http://127.0.0.1:1", "http://127.0.0.1:1").unwrap();
        let engine = Engine::from_parts(gh, cache);

        let bundle = sample_bundle();
        engine
            .seed_from_bundle(&bundle, |key| {
                (key == "xxhash.h").then(|| b"CORRUPTED".to_vec())
            })
            .unwrap();

        let err = engine.resolve_pinned(&bundle.provenance, &["xxhash.h"]);
        assert!(
            err.is_err(),
            "corrupted cache content must not be served as verified"
        );
    }

    #[test]
    fn resolve_pinned_rejects_missing_pin() {
        let engine = engine_in_memory();
        let bundle = sample_bundle();
        let err = engine
            .resolve_pinned(&bundle.provenance, &["vk.xml"])
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("no provenance for required file 'vk.xml'"),
            "{err}"
        );
    }

    #[test]
    fn group_by_cluster_coalesces_same_repo() {
        let groups = group_by_cluster(&["gl.xml", "glx.xml", "xxhash.h"]).unwrap();
        // gl.xml + glx.xml share OpenGL-Registry; xxhash.h is its own cluster.
        assert_eq!(groups.len(), 2);
        let opengl = groups
            .iter()
            .find(|(c, _)| c.repo == "KhronosGroup/OpenGL-Registry")
            .unwrap();
        assert_eq!(opengl.1, vec!["gl.xml", "glx.xml"]);
    }
}
