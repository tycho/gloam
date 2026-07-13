//! Unified source loading: resolve registry keys to provenance pins **and**
//! content, from one consistent place.
//!
//! [`SourceStore`] is the run-scoped resolver: one is constructed per
//! invocation and threaded everywhere sources are needed.  It memoizes —
//! the fetch engine (SQLite cache + bundle seed) is constructed at most once
//! per run, and each source is resolved, read, and hash-verified at most
//! once.  Content is memoized by **blob SHA**, so swapping the lock set
//! mid-run (the implicit-pin settlement in `run()`) re-resolves pins but
//! never re-reads content.  This also makes snapshot coherence structural:
//! every phase of a run sees the same bytes for a key, by construction.
//!
//! Modes:
//! - Bundled (default / no `--fetch`): pins from the embedded
//!   `bundled/provenance.json`, content from the embedded files.
//! - `--fetch`: through the [`engine`](super::engine), which seeds the cache
//!   from the bundle and resolves each cluster's HEAD.
//! - Locked (`--lock` or the implicit baseline): pins come from the lock;
//!   content from the bundle (blob must match) or the cache-backed engine.

#[cfg(feature = "fetch")]
use std::cell::OnceCell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{Result, anyhow, bail};
use indexmap::IndexMap;

use super::manifest::{ProvenancePin, git_blob_sha1};
use crate::bundled;

/// A loaded source: its provenance pin plus shared content bytes.
/// Cloning is cheap (the content is `Arc`-shared with the store's memo).
#[derive(Debug, Clone)]
pub struct LoadedSource {
    pub pin: ProvenancePin,
    pub content: Arc<Vec<u8>>,
}

/// Run-scoped source resolver.  See the module docs.
pub struct SourceStore {
    use_fetch: bool,
    /// Current lock set: pins are taken from here when present.
    lock: Option<IndexMap<String, ProvenancePin>>,
    /// Lazily constructed fetch engine — opened and bundle-seeded once.
    #[cfg(feature = "fetch")]
    engine: OnceCell<super::engine::Engine>,
    /// key → loaded source, valid for the current lock set (cleared by
    /// [`Self::set_lock`]).
    resolved: RefCell<IndexMap<String, LoadedSource>>,
    /// blob SHA → content.  Content-addressed, so it survives lock swaps.
    content: RefCell<HashMap<String, Arc<Vec<u8>>>>,
}

impl SourceStore {
    pub fn new(use_fetch: bool, lock: Option<IndexMap<String, ProvenancePin>>) -> Self {
        Self {
            use_fetch,
            lock,
            #[cfg(feature = "fetch")]
            engine: OnceCell::new(),
            resolved: RefCell::new(IndexMap::new()),
            content: RefCell::new(HashMap::new()),
        }
    }

    /// Bundled, no lock — the default and the test default.
    pub fn bundled() -> Self {
        Self::new(false, None)
    }

    /// Replace the lock set (the implicit-pin settlement in `run()`).
    /// Clears the per-key memo — pins may now differ — but keeps the
    /// content memo: blobs are content-addressed and cannot change meaning.
    pub fn set_lock(&mut self, lock: Option<IndexMap<String, ProvenancePin>>) {
        self.lock = lock;
        self.resolved.borrow_mut().clear();
    }

    /// Resolve registry `keys` to pins + content.  Memoized: repeat keys are
    /// map hits and emit no debug tracing (only actual resolution work does).
    pub fn resolve(&self, keys: &[&str]) -> Result<IndexMap<String, LoadedSource>> {
        let mut out: IndexMap<String, LoadedSource> = IndexMap::new();
        let mut misses: Vec<&str> = Vec::new();
        {
            let resolved = self.resolved.borrow();
            for &key in keys {
                match resolved.get(key) {
                    Some(src) => {
                        out.insert(key.to_string(), src.clone());
                    }
                    None => misses.push(key),
                }
            }
        }
        if misses.is_empty() {
            return Ok(out);
        }

        let started = std::time::Instant::now();
        let mode = match (self.lock.is_some(), self.use_fetch) {
            (true, true) => "locked+fetch",
            (true, false) => "locked+bundled",
            (false, true) => "fetch",
            (false, false) => "bundled",
        };

        let fresh = if let Some(pins) = &self.lock {
            self.resolve_locked(&misses, pins)?
        } else {
            #[cfg(feature = "fetch")]
            if self.use_fetch {
                self.resolve_fetch(&misses)?
            } else {
                self.resolve_bundled(&misses)?
            }
            #[cfg(not(feature = "fetch"))]
            self.resolve_bundled(&misses)?
        };

        crate::diag::debug(format_args!(
            "resolve[{mode}] {} key(s) in {}ms ({} memoized; first: {})",
            misses.len(),
            started.elapsed().as_millis(),
            keys.len() - misses.len(),
            misses.first().unwrap_or(&"<none>"),
        ));

        let mut resolved = self.resolved.borrow_mut();
        for (key, src) in fresh {
            resolved.insert(key.clone(), src.clone());
            out.insert(key, src);
        }
        Ok(out)
    }

    // -- content memo ---------------------------------------------------------

    /// Content for `blob`, from the memo or via `fetch` (result is memoized).
    fn content_for_blob<F>(&self, blob: &str, fetch: F) -> Result<Arc<Vec<u8>>>
    where
        F: FnOnce() -> Result<Vec<u8>>,
    {
        if let Some(c) = self.content.borrow().get(blob) {
            return Ok(c.clone());
        }
        let c = Arc::new(fetch()?);
        self.content
            .borrow_mut()
            .insert(blob.to_string(), c.clone());
        Ok(c)
    }

    // -- mode implementations ---------------------------------------------------

    /// Locked: pin every key from the lock set.  Content comes from the
    /// bundle when its blob matches, else the cache-backed engine (--fetch).
    /// This is the single owner of the missing-pin refusal.
    fn resolve_locked(
        &self,
        keys: &[&str],
        pins: &IndexMap<String, ProvenancePin>,
    ) -> Result<IndexMap<String, LoadedSource>> {
        // Refuse up front if the lock lacks provenance for anything we need.
        for &key in keys {
            if !pins.contains_key(key) {
                bail!(
                    "manifest (--lock) has no provenance for required file '{key}'; \
                     regenerate without --lock"
                );
            }
        }

        let mut out = IndexMap::new();
        for &key in keys {
            let pin = pins.get(key).unwrap().clone();
            let content = self.content_for_blob(&pin.blob, || {
                // Bundled content satisfies the lock only if its blob matches.
                if let Some(text) = bundled::content_by_key(key)
                    && git_blob_sha1(text.as_bytes()) == pin.blob
                {
                    return Ok(text.as_bytes().to_vec());
                }
                #[cfg(feature = "fetch")]
                if self.use_fetch {
                    let mut single = IndexMap::new();
                    single.insert(key.to_string(), pin.clone());
                    let mut resolved = self.engine()?.resolve_pinned(&single, &[key])?;
                    return Ok(resolved
                        .remove(key)
                        .ok_or_else(|| anyhow!("engine did not resolve '{key}'"))?
                        .content);
                }
                if bundled::content_by_key(key).is_some() {
                    bail!(
                        "--lock without --fetch: bundled '{key}' does not match the \
                         locked blob ({}); use --fetch",
                        &pin.blob[..7.min(pin.blob.len())]
                    );
                }
                bail!(
                    "--lock without --fetch: '{key}' is not in this gloam build's bundle; \
                     use --fetch"
                )
            })?;
            out.insert(key.to_string(), LoadedSource { pin, content });
        }
        Ok(out)
    }

    fn resolve_bundled(&self, keys: &[&str]) -> Result<IndexMap<String, LoadedSource>> {
        let bundle = bundled::bundled_provenance()?;
        let mut out = IndexMap::new();
        for &key in keys {
            let pin = bundle.provenance.get(key).cloned().ok_or_else(|| {
                anyhow!(
                    "bundled/provenance.json has no entry for '{key}' — run `cargo xtask bundle`"
                )
            })?;
            let content = self.content_for_blob(&pin.blob, || {
                Ok(bundled::content_by_key(key)
                    .ok_or_else(|| {
                        anyhow!("bundled content for '{key}' is empty — run `cargo xtask bundle`")
                    })?
                    .as_bytes()
                    .to_vec())
            })?;
            out.insert(key.to_string(), LoadedSource { pin, content });
        }
        Ok(out)
    }

    #[cfg(feature = "fetch")]
    fn resolve_fetch(&self, keys: &[&str]) -> Result<IndexMap<String, LoadedSource>> {
        let resolved = self.engine()?.resolve_head(keys)?;
        let mut out = IndexMap::new();
        for (key, r) in resolved {
            // The engine just read (and verified) this content; memoize it
            // under its blob so later locked-mode resolves reuse it.
            let content = Arc::new(r.content);
            self.content
                .borrow_mut()
                .insert(r.pin.blob.clone(), content.clone());
            out.insert(
                key,
                LoadedSource {
                    pin: r.pin,
                    content,
                },
            );
        }
        Ok(out)
    }

    /// The fetch engine, constructed (cache opened, bundle seeded) at most
    /// once per run.
    #[cfg(feature = "fetch")]
    fn engine(&self) -> Result<&super::engine::Engine> {
        if self.engine.get().is_none() {
            let started = std::time::Instant::now();
            let engine = super::engine::Engine::new()?;
            let opened = started.elapsed().as_millis();
            // Seed from the embedded bundle so unchanged content resolves
            // from cache.
            engine.seed_from_bundle(&bundled::bundled_provenance()?, |k| {
                bundled::content_by_key(k).map(|s| s.as_bytes().to_vec())
            })?;
            crate::diag::debug(format_args!(
                "engine: cache opened in {opened}ms, bundle seeded in {}ms (once per run)",
                started.elapsed().as_millis() - opened
            ));
            let _ = self.engine.set(engine);
        }
        Ok(self.engine.get().expect("engine just initialized"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_resolve_returns_pins_and_content() {
        // Uses the populated bundled/provenance.json; offline.
        let store = SourceStore::bundled();
        let resolved = store
            .resolve(&["gl.xml", "xxhash.h"])
            .expect("bundled resolve");

        let gl = &resolved["gl.xml"];
        assert_eq!(gl.pin.repo, "KhronosGroup/OpenGL-Registry");
        assert_eq!(gl.pin.path_in_repo, "xml/gl.xml");
        assert_eq!(gl.pin.commit.len(), 40);
        assert_eq!(gl.pin.blob.len(), 40);
        assert!(gl.content.windows(9).any(|w| w == b"<registry"));

        let xxh = &resolved["xxhash.h"];
        assert_eq!(xxh.pin.repo, "Cyan4973/xxHash");
        assert!(!xxh.content.is_empty());
    }

    #[test]
    fn bundled_resolve_errors_on_unknown_key() {
        let err = SourceStore::bundled()
            .resolve(&["nope.xml"])
            .unwrap_err()
            .to_string();
        assert!(err.contains("nope.xml"), "{err}");
    }

    #[test]
    fn repeat_resolves_share_memoized_content() {
        let store = SourceStore::bundled();
        let a = store.resolve(&["gl.xml"]).unwrap();
        let b = store.resolve(&["gl.xml"]).unwrap();
        assert!(
            Arc::ptr_eq(&a["gl.xml"].content, &b["gl.xml"].content),
            "second resolve must be a memo hit sharing the same Arc"
        );
    }

    #[test]
    fn set_lock_keeps_content_memo_but_rereads_pins() {
        let bundle = bundled::bundled_provenance().unwrap();
        let mut store = SourceStore::bundled();
        let before = store.resolve(&["gl.xml"]).unwrap()["gl.xml"].clone();

        // Lock to the same blob but a sentinel commit: the pin must change,
        // the content Arc must be reused (content-addressed memo).
        let mut pin = bundle.provenance["gl.xml"].clone();
        pin.commit = "f".repeat(40);
        let mut pins = IndexMap::new();
        pins.insert("gl.xml".to_string(), pin);
        store.set_lock(Some(pins));

        let after = store.resolve(&["gl.xml"]).unwrap()["gl.xml"].clone();
        assert_eq!(after.pin.commit, "f".repeat(40), "pin re-read from lock");
        assert!(
            Arc::ptr_eq(&before.content, &after.content),
            "content memo survives the lock swap"
        );
    }

    #[test]
    fn locked_bundled_match_succeeds() {
        // A lock whose blob equals the bundled blob resolves offline (this also
        // checks bundle integrity: file content hashes to the recorded blob).
        let bundle = bundled::bundled_provenance().unwrap();
        let mut pins = IndexMap::new();
        pins.insert("gl.xml".to_string(), bundle.provenance["gl.xml"].clone());
        let store = SourceStore::new(false, Some(pins));
        let resolved = store.resolve(&["gl.xml"]).unwrap();
        assert!(!resolved["gl.xml"].content.is_empty());
    }

    #[test]
    fn locked_bundled_mismatch_is_refused() {
        let mut pin = bundled::bundled_provenance().unwrap().provenance["gl.xml"].clone();
        pin.blob = "0".repeat(40);
        let mut pins = IndexMap::new();
        pins.insert("gl.xml".to_string(), pin);
        let store = SourceStore::new(false, Some(pins));
        let err = store.resolve(&["gl.xml"]).unwrap_err().to_string();
        assert!(err.contains("does not match the locked blob"), "{err}");
    }

    #[test]
    fn locked_missing_key_is_refused() {
        let store = SourceStore::new(false, Some(IndexMap::new()));
        let err = store.resolve(&["gl.xml"]).unwrap_err().to_string();
        assert!(
            err.contains("no provenance for required file 'gl.xml'"),
            "{err}"
        );
    }
}
