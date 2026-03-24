//! Spec-level constants and naming helpers.
//!
//! `SpecInfo` precomputes per-spec constants (display name, boolean flags,
//! prefix strings) from the spec name alone, replacing duplicated
//! `match spec_name { ... }` blocks that appeared in multiple places.
//!
//! `ResolveConfig` bundles the configuration parameters that were previously
//! passed as 6+ separate arguments through `resolve_feature_set`.

use std::collections::HashSet;

use crate::cli::{ApiRequest, ExtensionFilter};

// ---------------------------------------------------------------------------
// SpecInfo
// ---------------------------------------------------------------------------

/// Precomputed spec-level constants derived from the spec name alone.
pub(super) struct SpecInfo {
    pub display_name: &'static str,
    pub is_vulkan: bool,
    pub is_gl_family: bool,
    pub pfn_prefix: &'static str,
    pub name_prefix: &'static str,
    pub context_name: String,
}

impl SpecInfo {
    pub fn new(spec_name: &str) -> Self {
        Self {
            display_name: spec_display_name(spec_name),
            is_vulkan: spec_name == "vk",
            is_gl_family: matches!(spec_name, "gl" | "egl" | "glx" | "wgl"),
            pfn_prefix: api_pfn_prefix(spec_name),
            name_prefix: api_name_prefix(spec_name),
            context_name: build_context_name(spec_name),
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
    pub unchecked: bool,
}

// ---------------------------------------------------------------------------
// Naming helpers
// ---------------------------------------------------------------------------

/// Human-readable display name for a spec family.
fn spec_display_name(spec: &str) -> &'static str {
    match spec {
        "gl" => "GL",
        "egl" => "EGL",
        "glx" => "GLX",
        "wgl" => "WGL",
        "vk" => "Vulkan",
        _ => "Unknown",
    }
}

/// C context struct name, e.g. "GloamGLContext", "GloamVulkanContext".
fn build_context_name(spec: &str) -> String {
    format!("Gloam{}Context", spec_display_name(spec))
}

/// Prefix for PFN type names.
pub(super) fn api_pfn_prefix(spec: &str) -> &'static str {
    match spec {
        "vk" => "PFN_",
        "gl" | "gles1" | "gles2" | "glcore" => "PFNGL",
        "egl" => "PFNEGL",
        "glx" => "PFNGLX",
        "wgl" => "PFNWGL",
        _ => "PFN",
    }
}

/// Prefix stripped from command names to get the short (struct member) name.
pub(super) fn api_name_prefix(spec: &str) -> &'static str {
    match spec {
        "gl" | "gles1" | "gles2" | "glcore" => "gl",
        "egl" => "egl",
        "glx" => "glX",
        "wgl" => "wgl",
        "vk" | "vulkan" => "vk",
        _ => "",
    }
}

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
pub(super) fn version_short_name(name: &str, api: &str) -> String {
    let prefix = match api {
        "gl" | "glcore" => "GL_",
        "gles1" | "gles2" => "GL_",
        "egl" => "EGL_",
        "glx" => "GLX_",
        "wgl" => "WGL_",
        "vk" | "vulkan" => "VK_",
        _ => "",
    };
    name.strip_prefix(prefix).unwrap_or(name).to_string()
}

/// Sorting key for API ordering in merged builds: GL first, then GLES, etc.
pub(super) fn api_order(api: &str) -> u8 {
    match api {
        "gl" | "glcore" => 0,
        "gles1" => 1,
        "gles2" => 2,
        "egl" => 3,
        "glx" => 4,
        "wgl" => 5,
        "vk" | "vulkan" => 6,
        _ => 7,
    }
}

/// Build the set of API names in XML-canonical form for the given requests.
///
/// Uses the XML-canonical form ("vulkan" not "vk") because these flow into
/// generated C symbol suffixes (kExtIdx_vulkan, etc.) and IndexMap keys used
/// by the templates.
pub(super) fn xml_api_names(requests: &[ApiRequest]) -> Vec<String> {
    requests
        .iter()
        .map(|r| crate::cli::xml_api_name(&r.name).to_string())
        .collect()
}

/// Build the set of canonical API name strings for fast membership testing
/// in extension selection.
pub(super) fn build_api_set(requests: &[ApiRequest]) -> HashSet<&str> {
    let mut api_set: HashSet<&str> = requests.iter().map(|r| r.name.as_str()).collect();
    // The Khronos XML uses "vulkan" in supported= attributes, but our
    // canonical name is "vk".  Insert the XML form so contains() lookups
    // against XML-sourced strings succeed.
    if api_set.contains("vk") {
        api_set.insert("vulkan");
    }
    api_set
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ---- api_pfn_prefix / api_name_prefix ----

    #[test]
    fn pfn_prefix_gl_family() {
        for api in &["gl", "gles1", "gles2", "glcore"] {
            assert_eq!(api_pfn_prefix(api), "PFNGL", "failed for '{api}'");
        }
    }

    #[test]
    fn pfn_prefix_vulkan() {
        assert_eq!(api_pfn_prefix("vk"), "PFN_");
    }

    #[test]
    fn name_prefix_gl_family() {
        assert_eq!(api_name_prefix("gl"), "gl");
        assert_eq!(api_name_prefix("gles1"), "gl");
        assert_eq!(api_name_prefix("gles2"), "gl");
    }

    #[test]
    fn name_prefix_glx_is_case_sensitive() {
        // glX — capital X matters for generated member names.
        assert_eq!(api_name_prefix("glx"), "glX");
    }

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
        assert_eq!(version_short_name("GL_VERSION_3_3", "gl"), "VERSION_3_3");
    }

    #[test]
    fn version_short_name_gles2() {
        // GLES uses "GL_" prefix in the XML feature name.
        assert_eq!(
            version_short_name("GL_ES_VERSION_3_0", "gles2"),
            "ES_VERSION_3_0"
        );
    }

    #[test]
    fn version_short_name_vk() {
        assert_eq!(version_short_name("VK_VERSION_1_3", "vk"), "VERSION_1_3");
    }

    #[test]
    fn version_short_name_unknown_api_no_strip() {
        assert_eq!(
            version_short_name("CUSTOM_VERSION_1_0", "custom"),
            "CUSTOM_VERSION_1_0"
        );
    }
}
