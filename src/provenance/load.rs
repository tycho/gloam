//! Unified source loading: resolve a set of registry keys to provenance pins
//! **and** content, from one consistent place.
//!
//! - Bundled mode (default / no `--fetch`): pins from the embedded
//!   `bundled/provenance.json`, content from the embedded files.
//! - `--fetch`: through the [`engine`](super::engine), which seeds the cache
//!   from the bundle and resolves each cluster's HEAD — so content and pins come
//!   from the same snapshot, with no separate provenance round-trip.

use anyhow::{Result, anyhow};
use indexmap::IndexMap;

use super::manifest::ProvenancePin;
use crate::bundled;

/// A loaded source: its provenance pin plus content bytes.
#[derive(Debug)]
pub struct LoadedSource {
    pub pin: ProvenancePin,
    pub content: Vec<u8>,
}

/// Resolve registry `keys` to pins + content.  `use_fetch` selects live
/// resolution (cache-backed) over the embedded bundle.
pub fn resolve(keys: &[&str], use_fetch: bool) -> Result<IndexMap<String, LoadedSource>> {
    #[cfg(feature = "fetch")]
    if use_fetch {
        return resolve_fetch(keys);
    }
    let _ = use_fetch;
    resolve_bundled(keys)
}

fn resolve_bundled(keys: &[&str]) -> Result<IndexMap<String, LoadedSource>> {
    let bundle = bundled::bundled_provenance()?;
    let mut out = IndexMap::new();
    for &key in keys {
        let pin = bundle.provenance.get(key).cloned().ok_or_else(|| {
            anyhow!("bundled/provenance.json has no entry for '{key}' — run `cargo xtask bundle`")
        })?;
        let content = bundled::content_by_key(key)
            .ok_or_else(|| anyhow!("bundled content for '{key}' is empty — run `cargo xtask bundle`"))?
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
        let resolved = resolve(&["gl.xml", "xxhash.h"], false).expect("bundled resolve");

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
        let err = resolve(&["nope.xml"], false).unwrap_err().to_string();
        assert!(err.contains("nope.xml"), "{err}");
    }
}
