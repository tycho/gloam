//! Spec-level constants and naming helpers.
//!
//! `SpecInfo` precomputes per-spec constants (display name, boolean flags,
//! prefix strings) from the [`Spec`] identity; the tables themselves live on
//! the `Api`/`Spec` enums in `crate::identity`.
//!
//! `ResolveConfig` bundles the configuration parameters that were previously
//! passed as 6+ separate arguments through `resolve_feature_set`.

use std::collections::HashSet;

use crate::cli::{ApiRequest, ExtensionFilter};
use crate::identity::{Api, Spec};

// ---------------------------------------------------------------------------
// SpecInfo
// ---------------------------------------------------------------------------

/// Precomputed spec-level constants derived from the spec identity.
pub(super) struct SpecInfo {
    pub display_name: &'static str,
    pub is_vulkan: bool,
    pub is_gl_family: bool,
    pub name_prefix: &'static str,
}

impl SpecInfo {
    pub fn new(spec: Spec) -> Self {
        Self {
            display_name: spec.display(),
            is_vulkan: spec.is_vulkan(),
            is_gl_family: !spec.is_vulkan(),
            name_prefix: spec.name_prefix(),
        }
    }
}

// ---------------------------------------------------------------------------
// ResolveConfig
// ---------------------------------------------------------------------------

/// Bundles the loose parameters that were passed through multiple function
/// signatures in the original monolithic `resolve_feature_set`.
pub(super) struct ResolveConfig<'a> {
    pub ext_filter: &'a ExtensionFilter,
    pub baseline: &'a [ApiRequest],
    pub is_merged: bool,
    pub want_aliases: bool,
    pub want_promoted: bool,
    pub want_predecessors: bool,
}

// ---------------------------------------------------------------------------
// Naming helpers
// ---------------------------------------------------------------------------

/// Strip API prefix from extension name: "GL_ARB_sync" → "ARB_sync".
pub(super) fn ext_short_name(name: &str) -> String {
    for prefix in &["GL_", "EGL_", "GLX_", "WGL_", "VK_"] {
        if let Some(s) = name.strip_prefix(prefix) {
            return s.to_string();
        }
    }
    name.to_string()
}

/// Strip version prefix: "GL_VERSION_3_3" → "VERSION_3_3".
pub(super) fn version_short_name(name: &str, api: Api) -> String {
    name.strip_prefix(api.version_prefix())
        .unwrap_or(name)
        .to_string()
}

/// Build the list of API names in CLI-canonical form for the given requests.
///
/// These become `FeatureSet.apis` — the template-visible API identities that
/// flow into generated C symbol suffixes (`kExtIdx_vk`, etc.), the per-API
/// IndexMap keys, and (for non-merged builds) the output file stem.
pub(super) fn api_names(requests: &[ApiRequest]) -> Vec<String> {
    requests
        .iter()
        .map(|r| r.api.as_str().to_string())
        .collect()
}

/// Build the set of API name strings for fast membership testing against
/// XML-sourced tokens in extension selection.  Contains each request's
/// canonical name plus its XML form (they differ only for Vulkan, so
/// `contains()` succeeds for both spellings).
pub(super) fn build_api_set(requests: &[ApiRequest]) -> HashSet<&'static str> {
    let mut api_set: HashSet<&'static str> = HashSet::new();
    for r in requests {
        api_set.insert(r.api.as_str());
        api_set.insert(r.api.xml_name());
    }
    api_set
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ---- ext_short_name / version_short_name ----

    #[test]
    fn ext_short_name_strips_gl_prefix() {
        assert_eq!(ext_short_name("GL_ARB_sync"), "ARB_sync");
    }

    #[test]
    fn ext_short_name_strips_vk_prefix() {
        assert_eq!(ext_short_name("VK_KHR_swapchain"), "KHR_swapchain");
    }

    #[test]
    fn ext_short_name_strips_egl_prefix() {
        assert_eq!(
            ext_short_name("EGL_KHR_platform_wayland"),
            "KHR_platform_wayland"
        );
    }

    #[test]
    fn ext_short_name_unknown_prefix_unchanged() {
        assert_eq!(ext_short_name("UNKNOWN_foo_bar"), "UNKNOWN_foo_bar");
    }

    #[test]
    fn version_short_name_gl() {
        assert_eq!(version_short_name("GL_VERSION_3_3", Api::Gl), "VERSION_3_3");
    }

    #[test]
    fn version_short_name_gles2() {
        // GLES uses "GL_" prefix in the XML feature name.
        assert_eq!(
            version_short_name("GL_ES_VERSION_3_0", Api::Gles2),
            "ES_VERSION_3_0"
        );
    }

    #[test]
    fn version_short_name_vk() {
        assert_eq!(version_short_name("VK_VERSION_1_3", Api::Vk), "VERSION_1_3");
    }

    // ---- SpecInfo ----

    #[test]
    fn spec_info_vulkan() {
        let s = SpecInfo::new(Spec::Vk);
        assert!(s.is_vulkan);
        assert!(!s.is_gl_family);
        assert_eq!(s.name_prefix, "vk");
    }

    #[test]
    fn spec_info_gl() {
        let s = SpecInfo::new(Spec::Gl);
        assert!(!s.is_vulkan);
        assert!(s.is_gl_family);
        assert_eq!(s.display_name, "GL");
        assert_eq!(s.name_prefix, "gl");
    }
}
