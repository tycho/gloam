//! XML and auxiliary source loading, unified through the provenance loader
//! ([`crate::provenance::load`]): compile-time-embedded bundled copies by
//! default, or the cache-backed GitHub-API engine under `--fetch`.  Both paths
//! yield identical content (and, via the loader, provenance pins).

use anyhow::{Context, Result, anyhow};
use indexmap::IndexMap;

use crate::provenance;
use crate::provenance::load::{self, LoadedSource};

// ---------------------------------------------------------------------------
// SpecSources
// ---------------------------------------------------------------------------

/// XML text for one spec family: the primary doc plus any supplementals.  The
/// parser iterates all of them in order, treating supplementals as if merged
/// into the primary before parsing.
pub struct SpecSources {
    pub primary: String,
    pub supplementals: Vec<String>,
    /// Registry keys of the files actually merged for this generation — the
    /// primary spec followed by the request-aware supplementals — in order.
    /// Drives provenance/attribution so output reflects what truly contributed.
    pub source_keys: Vec<String>,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Load a spec's XML sources.  `apis` is the set of canonical API names in
/// scope (e.g. `["gl", "gles2"]`), which selects request-aware supplementals.
pub fn load_spec(spec_name: &str, apis: &[&str], use_fetch: bool) -> Result<SpecSources> {
    let primary_key = provenance::primary_key(spec_name)
        .ok_or_else(|| anyhow!("unknown spec name '{}'", spec_name))?;
    let supp_keys = provenance::supplemental_keys(spec_name, apis);

    let mut keys: Vec<&str> = vec![primary_key];
    keys.extend(supp_keys.iter().copied());

    let resolved = load::resolve(&keys, use_fetch)?;

    let primary = take_text(&resolved, primary_key)?;
    let supplementals = supp_keys
        .iter()
        .map(|k| take_text(&resolved, k))
        .collect::<Result<Vec<_>>>()?;
    let source_keys = keys.iter().map(|k| k.to_string()).collect();

    Ok(SpecSources {
        primary,
        supplementals,
        source_keys,
    })
}

/// Load a single auxiliary header's text by registry key (e.g.
/// "KHR/khrplatform.h", "xxhash.h", "vk_video/...").
pub fn load_auxiliary_header(path: &str, use_fetch: bool) -> Result<String> {
    let resolved = load::resolve(&[path], use_fetch)?;
    take_text(&resolved, path)
}

fn take_text(resolved: &IndexMap<String, LoadedSource>, key: &str) -> Result<String> {
    let src = resolved
        .get(key)
        .ok_or_else(|| anyhow!("source '{}' was not resolved", key))?;
    String::from_utf8(src.content.clone())
        .with_context(|| format!("source '{}' is not valid UTF-8", key))
}
