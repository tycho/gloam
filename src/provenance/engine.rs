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

use anyhow::{Context, Result, anyhow};
use indexmap::IndexMap;

use super::acquire::Github;
use super::cache::{self, Cache};
use super::manifest::{BundledProvenance, ProvenancePin};
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
    pub fn seed_from_bundle<F>(&self, bundle: &BundledProvenance, content_for: F) -> Result<()>
    where
        F: Fn(&str) -> Option<Vec<u8>>,
    {
        let now = cache::now();
        for (key, pin) in &bundle.provenance {
            let Some(content) = content_for(key) else { continue };
            self.cache.put_commit(&pin.commit, &pin.repo, &pin.describe, now)?;
            self.cache
                .put_tree_entry(&pin.commit, &pin.path_in_repo, &pin.blob)?;
            self.cache.put_blob(&pin.blob, &content, now)?;
        }
        Ok(())
    }

    /// Resolve registry keys at upstream HEAD (no-lock `--fetch`).
    pub fn resolve_head(&self, keys: &[&str]) -> Result<BTreeMap<String, Resolved>> {
        let by_cluster = group_by_cluster(keys)?;
        let mut out = BTreeMap::new();
        let now = cache::now();

        for (cluster, cluster_keys) in by_cluster {
            // Resolve the cluster's HEAD commit + describe (cache-first).
            let commit = match self.cache.fresh_head(cluster.repo, now, self.head_ttl)? {
                Some(c) => c,
                None => {
                    let c = self.gh.head_commit(cluster.repo, cluster.branch)?;
                    self.cache.set_head(cluster.repo, cluster.branch, &c, now)?;
                    c
                }
            };
            let describe = match self.cache.commit_describe(&commit, now)? {
                Some(d) => d,
                None => {
                    let d = self.gh.describe(cluster.repo, &commit)?;
                    self.cache.put_commit(&commit, cluster.repo, &d, now)?;
                    d
                }
            };

            for key in cluster_keys {
                let spec = cluster.files.iter().find(|f| f.key == key).unwrap();
                // blob SHA + content, cache-first.
                let (blob, content) = match self.cache.blob_for_path(&commit, spec.path_in_repo)? {
                    Some(b) => match self.cache.blob(&b, now)? {
                        Some(content) => (b, content),
                        None => self.fetch_and_cache(cluster, spec.path_in_repo, &commit, now)?,
                    },
                    None => self.fetch_and_cache(cluster, spec.path_in_repo, &commit, now)?,
                };
                out.insert(
                    key.to_string(),
                    Resolved {
                        pin: pin_for(cluster, spec.path_in_repo, &commit, &describe, &blob),
                        content,
                    },
                );
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

            let content = match self.cache.blob(&pin.blob, now)? {
                Some(c) => c,
                None => {
                    let c = self
                        .gh
                        .blob_content(&pin.repo, &pin.blob)
                        .with_context(|| format!("fetching pinned blob for '{key}'"))?;
                    self.cache.put_blob(&pin.blob, &c, now)?;
                    c
                }
            };
            // Opportunistically keep commit/tree metadata warm.
            self.cache
                .put_commit(&pin.commit, &pin.repo, &pin.describe, now)?;
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

    fn fetch_and_cache(
        &self,
        cluster: &Cluster,
        path: &str,
        commit: &str,
        now: i64,
    ) -> Result<(String, Vec<u8>)> {
        let (blob, content) = self
            .gh
            .file_at_commit(cluster.repo, path, commit)
            .with_context(|| format!("fetching {} from {}", path, cluster.repo))?;
        self.cache.put_tree_entry(commit, path, &blob)?;
        self.cache.put_blob(&blob, &content, now)?;
        Ok((blob, content))
    }
}

fn pin_for(
    cluster: &Cluster,
    path_in_repo: &str,
    commit: &str,
    describe: &str,
    blob: &str,
) -> ProvenancePin {
    ProvenancePin {
        repo: cluster.repo.to_string(),
        repo_url: cluster.repo_url.to_string(),
        path_in_repo: path_in_repo.to_string(),
        commit: commit.to_string(),
        describe: describe.to_string(),
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

    fn sample_bundle() -> BundledProvenance {
        let mut provenance = IndexMap::new();
        provenance.insert(
            "xxhash.h".to_string(),
            ProvenancePin {
                repo: "Cyan4973/xxHash".to_string(),
                repo_url: "https://github.com/Cyan4973/xxHash".to_string(),
                path_in_repo: "xxhash.h".to_string(),
                commit: "c0ffee00".to_string(),
                describe: "v0.8.2".to_string(),
                blob: "blobxxh".to_string(),
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
                (key == "xxhash.h").then(|| b"/* xxHash */".to_vec())
            })
            .unwrap();

        // The blob is cached, so resolution must not touch the network.
        let resolved = engine
            .resolve_pinned(&bundle.provenance, &["xxhash.h"])
            .unwrap();
        let r = &resolved["xxhash.h"];
        assert_eq!(r.content, b"/* xxHash */");
        assert_eq!(r.pin.blob, "blobxxh");
        assert_eq!(r.pin.describe, "v0.8.2");
    }

    #[test]
    fn resolve_pinned_rejects_missing_pin() {
        let engine = engine_in_memory();
        let bundle = sample_bundle();
        let err = engine
            .resolve_pinned(&bundle.provenance, &["vk.xml"])
            .unwrap_err()
            .to_string();
        assert!(err.contains("no provenance for required file 'vk.xml'"), "{err}");
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
