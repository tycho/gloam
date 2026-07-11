//! Typed identity for APIs and spec families.
//!
//! Every name an API or spec family goes by — CLI-canonical short name,
//! XML-canonical name, display name, C symbol prefixes, sort order — lives
//! here, on two enums.  Everywhere else in the pipeline holds an [`Api`] or
//! [`Spec`] value and asks it for the string form it needs at the boundary
//! (CLI parsing on the way in, template/serialization strings on the way
//! out).  Adding an API means adding a variant and letting exhaustive
//! matches point at every table that needs a row.
//!
//! Naming conventions, because Khronos has several:
//!   - **canonical** — the CLI short form and gloam's internal identity:
//!     `"gl"`, `"gles2"`, `"vk"`, ...
//!   - **XML** — the form used by `api=`/`supported=` attributes in the
//!     Khronos XML: identical to canonical except Vulkan, which the XML
//!     calls `"vulkan"`.  XML attribute *tokens* are an open set (e.g.
//!     `"vulkansc"`, `"glsc2"`) — they are matched against a typed [`Api`],
//!     never parsed into one.

use anyhow::{Result, bail};

// ---------------------------------------------------------------------------
// Api
// ---------------------------------------------------------------------------

/// One requestable API. This is what `--api gl:core=3.3,gles2=3.0` parses
/// into, and what feature/extension selection is keyed by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Api {
    Gl,
    Glcore,
    Gles1,
    Gles2,
    Egl,
    Glx,
    Wgl,
    Vk,
}

impl Api {
    /// Parse a CLI-supplied API name.  Accepts the XML-canonical `"vulkan"`
    /// as an alias for `"vk"`; anything else unknown is a hard error.
    pub fn from_cli(name: &str) -> Result<Api> {
        Ok(match name {
            "gl" => Api::Gl,
            "glcore" => Api::Glcore,
            "gles1" => Api::Gles1,
            "gles2" => Api::Gles2,
            "egl" => Api::Egl,
            "glx" => Api::Glx,
            "wgl" => Api::Wgl,
            "vk" | "vulkan" => Api::Vk,
            other => bail!(
                "unknown API '{other}' (expected gl, glcore, gles1, gles2, egl, glx, wgl, or vk)"
            ),
        })
    }

    /// CLI-canonical short name — gloam's internal identity string.
    pub fn as_str(self) -> &'static str {
        match self {
            Api::Gl => "gl",
            Api::Glcore => "glcore",
            Api::Gles1 => "gles1",
            Api::Gles2 => "gles2",
            Api::Egl => "egl",
            Api::Glx => "glx",
            Api::Wgl => "wgl",
            Api::Vk => "vk",
        }
    }

    /// XML-canonical name, as used in `api=`/`supported=` attributes.
    /// Differs from [`Self::as_str`] only for Vulkan (`"vulkan"`).
    pub fn xml_name(self) -> &'static str {
        match self {
            Api::Vk => "vulkan",
            other => other.as_str(),
        }
    }

    /// Display name for public C symbols, e.g. `gloamLoadGLES2Context`.
    pub fn display(self) -> &'static str {
        match self {
            Api::Gl | Api::Glcore => "GL",
            Api::Gles1 => "GLES1",
            Api::Gles2 => "GLES2",
            Api::Egl => "EGL",
            Api::Glx => "GLX",
            Api::Wgl => "WGL",
            Api::Vk => "Vulkan",
        }
    }

    /// The spec family this API's definitions live in.
    pub fn spec(self) -> Spec {
        match self {
            Api::Gl | Api::Glcore | Api::Gles1 | Api::Gles2 => Spec::Gl,
            Api::Egl => Spec::Egl,
            Api::Glx => Spec::Glx,
            Api::Wgl => Spec::Wgl,
            Api::Vk => Spec::Vk,
        }
    }

    /// Sorting key for API ordering in merged builds: GL first, then GLES, etc.
    pub fn sort_order(self) -> u8 {
        match self {
            Api::Gl | Api::Glcore => 0,
            Api::Gles1 => 1,
            Api::Gles2 => 2,
            Api::Egl => 3,
            Api::Glx => 4,
            Api::Wgl => 5,
            Api::Vk => 6,
        }
    }

    /// Prefix stripped from feature names to get the short version name,
    /// e.g. `"GL_VERSION_3_3"` → `"VERSION_3_3"`.
    pub fn version_prefix(self) -> &'static str {
        match self {
            Api::Gl | Api::Glcore | Api::Gles1 | Api::Gles2 => "GL_",
            Api::Egl => "EGL_",
            Api::Glx => "GLX_",
            Api::Wgl => "WGL_",
            Api::Vk => "VK_",
        }
    }
}

impl std::fmt::Display for Api {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Normalize an XML api token to its CLI-canonical form: `"vulkan"` → `"vk"`,
/// everything else unchanged.  For matching XML-sourced strings (an open set
/// that includes tokens gloam never requests, e.g. `"vulkansc"`) against
/// canonical names without parsing them into [`Api`].
pub fn canonical_api_name(token: &str) -> &str {
    match token {
        "vulkan" => "vk",
        other => other,
    }
}

// ---------------------------------------------------------------------------
// Spec
// ---------------------------------------------------------------------------

/// One spec family — a set of Khronos XML documents parsed together and
/// emitted as one loader.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Spec {
    Gl,
    Egl,
    Glx,
    Wgl,
    Vk,
}

impl Spec {
    pub fn from_name(name: &str) -> Option<Spec> {
        Some(match name {
            "gl" => Spec::Gl,
            "egl" => Spec::Egl,
            "glx" => Spec::Glx,
            "wgl" => Spec::Wgl,
            "vk" => Spec::Vk,
            _ => return None,
        })
    }

    /// Canonical spec name: file stems, registry lookups, `RawSpec.spec_name`.
    pub fn as_str(self) -> &'static str {
        match self {
            Spec::Gl => "gl",
            Spec::Egl => "egl",
            Spec::Glx => "glx",
            Spec::Wgl => "wgl",
            Spec::Vk => "vk",
        }
    }

    /// Human-readable display name, e.g. `"Vulkan"`.
    pub fn display(self) -> &'static str {
        match self {
            Spec::Gl => "GL",
            Spec::Egl => "EGL",
            Spec::Glx => "GLX",
            Spec::Wgl => "WGL",
            Spec::Vk => "Vulkan",
        }
    }

    /// C context struct name, e.g. `"GloamGLContext"`, `"GloamVulkanContext"`.
    pub fn context_name(self) -> String {
        format!("Gloam{}Context", self.display())
    }

    /// Prefix for PFN typedef names.
    pub fn pfn_prefix(self) -> &'static str {
        match self {
            Spec::Gl => "PFNGL",
            Spec::Egl => "PFNEGL",
            Spec::Glx => "PFNGLX",
            Spec::Wgl => "PFNWGL",
            Spec::Vk => "PFN_",
        }
    }

    /// Prefix stripped from command names to get the short (struct member)
    /// name.  Case matters: GLX commands are `glX*`.
    pub fn name_prefix(self) -> &'static str {
        match self {
            Spec::Gl => "gl",
            Spec::Egl => "egl",
            Spec::Glx => "glX",
            Spec::Wgl => "wgl",
            Spec::Vk => "vk",
        }
    }

    pub fn is_vulkan(self) -> bool {
        self == Spec::Vk
    }
}

impl std::fmt::Display for Spec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_APIS: [Api; 8] = [
        Api::Gl,
        Api::Glcore,
        Api::Gles1,
        Api::Gles2,
        Api::Egl,
        Api::Glx,
        Api::Wgl,
        Api::Vk,
    ];

    #[test]
    fn api_from_cli_roundtrips_canonical_names() {
        for api in ALL_APIS {
            assert_eq!(Api::from_cli(api.as_str()).unwrap(), api);
        }
    }

    #[test]
    fn api_from_cli_accepts_vulkan_alias() {
        assert_eq!(Api::from_cli("vulkan").unwrap(), Api::Vk);
    }

    #[test]
    fn api_from_cli_rejects_unknown() {
        let err = Api::from_cli("dx12").unwrap_err().to_string();
        assert!(err.contains("unknown API 'dx12'"), "{err}");
    }

    #[test]
    fn xml_name_differs_only_for_vulkan() {
        for api in ALL_APIS {
            if api == Api::Vk {
                assert_eq!(api.xml_name(), "vulkan");
            } else {
                assert_eq!(api.xml_name(), api.as_str());
            }
        }
    }

    #[test]
    fn gl_family_maps_to_gl_spec() {
        for api in [Api::Gl, Api::Glcore, Api::Gles1, Api::Gles2] {
            assert_eq!(api.spec(), Spec::Gl);
        }
        assert_eq!(Api::Vk.spec(), Spec::Vk);
    }

    #[test]
    fn canonical_api_name_normalizes_vulkan_only() {
        assert_eq!(canonical_api_name("vulkan"), "vk");
        assert_eq!(canonical_api_name("vk"), "vk");
        assert_eq!(canonical_api_name("gl"), "gl");
        // Open-set tokens pass through untouched.
        assert_eq!(canonical_api_name("vulkansc"), "vulkansc");
    }

    #[test]
    fn spec_naming_tables() {
        assert_eq!(Spec::Vk.context_name(), "GloamVulkanContext");
        assert_eq!(Spec::Gl.context_name(), "GloamGLContext");
        assert_eq!(Spec::Glx.name_prefix(), "glX", "capital X matters");
        assert_eq!(Spec::Vk.pfn_prefix(), "PFN_");
        assert_eq!(Spec::Gl.pfn_prefix(), "PFNGL");
    }

    #[test]
    fn spec_from_name_covers_all_and_rejects_unknown() {
        for spec in [Spec::Gl, Spec::Egl, Spec::Glx, Spec::Wgl, Spec::Vk] {
            assert_eq!(Spec::from_name(spec.as_str()), Some(spec));
        }
        assert_eq!(Spec::from_name("gles2"), None, "api name is not a spec");
        assert_eq!(Spec::from_name("vulkan"), None, "xml name is not a spec");
    }

    #[test]
    fn version_prefix_per_api() {
        assert_eq!(Api::Gles2.version_prefix(), "GL_");
        assert_eq!(Api::Vk.version_prefix(), "VK_");
        assert_eq!(Api::Egl.version_prefix(), "EGL_");
    }
}
