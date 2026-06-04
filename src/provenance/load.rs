//! Unified source loading: resolve a set of registry keys to provenance pins
//! **and** content, from one consistent place.
//!
//! - Bundled mode (default / no `--fetch`): pins from the embedded
//!   `bundled/provenance.json`, content from the embedded files.
//! - `--fetch`: through the [`engine`](super::engine), which seeds the cache
//!   from the bundle and resolves each cluster's HEAD — so content and pins come
//!   from the same snapshot, with no separate provenance round-trip.

use anyhow::{Result, anyhow, bail};
use indexmap::IndexMap;

use super::manifest::{ProvenancePin, git_blob_sha1};
use crate::bundled;

/// A loaded source: its provenance pin plus content bytes.
#[derive(Debug)]
pub struct LoadedSource {
    pub pin: ProvenancePin,
    pub content: Vec<u8>,
}

/// How sources are resolved for a run.
pub struct LoadCtx<'a> {
    /// Resolve from upstream (cache-backed) rather than the embedded bundle.
    pub use_fetch: bool,
    /// When set (`--lock`), pin sources to this manifest's provenance instead of
    /// resolving HEAD.
    pub lock: Option<&'a IndexMap<String, ProvenancePin>>,
}

impl LoadCtx<'_> {
    /// Bundled, no lock — the default and the test default.
    pub fn bundled() -> Self {
        Self {
            use_fetch: false,
            lock: None,
        }
    }
}

/// Resolve registry `keys` to pins + content per the load context.
pub fn resolve(keys: &[&str], ctx: &LoadCtx) -> Result<IndexMap<String, LoadedSource>> {
    if let Some(pins) = ctx.lock {
        return resolve_locked(keys, pins, ctx.use_fetch);
    }
    #[cfg(feature = "fetch")]
    if ctx.use_fetch {
        return resolve_fetch(keys);
    }
    resolve_bundled(keys)
}

/// `--lock`: pin every key to the manifest's provenance.  Content comes from the
/// cache-backed engine (with `--fetch`) or from the bundle when its blob matches
/// the lock; a mismatch or missing key is refused.
fn resolve_locked(
    keys: &[&str],
    pins: &IndexMap<String, ProvenancePin>,
    use_fetch: bool,
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

    #[cfg(feature = "fetch")]
    if use_fetch {
        let engine = super::engine::Engine::new()?;
        engine.seed_from_bundle(&bundled::bundled_provenance()?, |k| {
            bundled::content_by_key(k).map(|s| s.as_bytes().to_vec())
        })?;
        let resolved = engine.resolve_pinned(pins, keys)?;
        return Ok(resolved
            .into_iter()
            .map(|(key, r)| {
                (
                    key,
                    LoadedSource {
                        pin: r.pin,
                        content: r.content,
                    },
                )
            })
            .collect());
    }
    let _ = use_fetch;

    // No --fetch: serve from the bundle only if its blob matches the lock.
    let mut out = IndexMap::new();
    for &key in keys {
        let pin = pins.get(key).unwrap().clone();
        let content = bundled::content_by_key(key).ok_or_else(|| {
            anyhow!(
                "--lock without --fetch: '{key}' is not in this gloam build's bundle; use --fetch"
            )
        })?;
        let bundled_blob = git_blob_sha1(content.as_bytes());
        if bundled_blob != pin.blob {
            bail!(
                "--lock without --fetch: bundled '{key}' (blob {}) does not match the \
                 locked blob ({}); use --fetch",
                &bundled_blob[..7.min(bundled_blob.len())],
                &pin.blob[..7.min(pin.blob.len())]
            );
        }
        out.insert(
            key.to_string(),
            LoadedSource {
                pin,
                content: content.as_bytes().to_vec(),
            },
        );
    }
    Ok(out)
}

fn resolve_bundled(keys: &[&str]) -> Result<IndexMap<String, LoadedSource>> {
    let bundle = bundled::bundled_provenance()?;
    let mut out = IndexMap::new();
    for &key in keys {
        let pin = bundle.provenance.get(key).cloned().ok_or_else(|| {
            anyhow!("bundled/provenance.json has no entry for '{key}' — run `cargo xtask bundle`")
        })?;
        let content = bundled::content_by_key(key)
            .ok_or_else(|| {
                anyhow!("bundled content for '{key}' is empty — run `cargo xtask bundle`")
            })?
            .as_bytes()
            .to_vec();
        out.insert(key.to_string(), LoadedSource { pin, content });
    }
    Ok(out)
}

#[cfg(feature = "fetch")]
fn resolve_fetch(keys: &[&str]) -> Result<IndexMap<String, LoadedSource>> {
    let engine = super::engine::Engine::new()?;
    // Seed from the embedded bundle so unchanged content resolves from cache.
    engine.seed_from_bundle(&bundled::bundled_provenance()?, |k| {
        bundled::content_by_key(k).map(|s| s.as_bytes().to_vec())
    })?;
    let resolved = engine.resolve_head(keys)?;
    let mut out = IndexMap::new();
    for (key, r) in resolved {
        out.insert(
            key,
            LoadedSource {
                pin: r.pin,
                content: r.content,
            },
        );
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_resolve_returns_pins_and_content() {
        // Uses the populated bundled/provenance.json; offline.
        let resolved =
            resolve(&["gl.xml", "xxhash.h"], &LoadCtx::bundled()).expect("bundled resolve");

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
        let err = resolve(&["nope.xml"], &LoadCtx::bundled())
            .unwrap_err()
            .to_string();
        assert!(err.contains("nope.xml"), "{err}");
    }

    #[test]
    fn locked_bundled_match_succeeds() {
        // A lock whose blob equals the bundled blob resolves offline (this also
        // checks bundle integrity: file content hashes to the recorded blob).
        let bundle = bundled::bundled_provenance().unwrap();
        let mut pins = IndexMap::new();
        pins.insert("gl.xml".to_string(), bundle.provenance["gl.xml"].clone());
        let ctx = LoadCtx {
            use_fetch: false,
            lock: Some(&pins),
        };
        let resolved = resolve(&["gl.xml"], &ctx).unwrap();
        assert!(!resolved["gl.xml"].content.is_empty());
    }

    #[test]
    fn locked_bundled_mismatch_is_refused() {
        let mut pin = bundled::bundled_provenance().unwrap().provenance["gl.xml"].clone();
        pin.blob = "0".repeat(40);
        let mut pins = IndexMap::new();
        pins.insert("gl.xml".to_string(), pin);
        let ctx = LoadCtx {
            use_fetch: false,
            lock: Some(&pins),
        };
        let err = resolve(&["gl.xml"], &ctx).unwrap_err().to_string();
        assert!(err.contains("does not match the locked blob"), "{err}");
    }

    #[test]
    fn locked_missing_key_is_refused() {
        let pins = IndexMap::new();
        let ctx = LoadCtx {
            use_fetch: false,
            lock: Some(&pins),
        };
        let err = resolve(&["gl.xml"], &ctx).unwrap_err().to_string();
        assert!(
            err.contains("no provenance for required file 'gl.xml'"),
            "{err}"
        );
    }
}
