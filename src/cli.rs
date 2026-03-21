//! Command-line interface definitions.

use anyhow::{Result, bail};
use clap::{Args, Parser, Subcommand};

use crate::ir::Version;

// ---------------------------------------------------------------------------
// Top-level CLI
// ---------------------------------------------------------------------------

#[derive(Parser, Debug)]
#[command(
    name = "gloam",
    version,
    about = "Vulkan/OpenGL/GLES/EGL/GLX/WGL loader generator"
)]
pub struct Cli {
    /// Automatically include any extension whose commands or enums were
    /// promoted into the requested core version, even if not listed in
    /// --extensions.
    #[arg(long)]
    pub promoted: bool,

    /// Automatically include any extension that is a predecessor of an
    /// explicitly selected extension (i.e. its commands are aliases of commands
    /// in the selected set).  For example, if GL_KHR_parallel_shader_compile is
    /// selected, GL_ARB_parallel_shader_compile is included automatically.
    #[arg(long)]
    pub predecessors: bool,

    /// API specifiers: comma-separated name[:profile]=version pairs.  Profile
    /// is required for GL (core|compat). Version is optional (latest if
    /// omitted).  Example: gl:core=3.3,gles2=3.0
    #[arg(long, required = true)]
    pub api: String,

    /// Extension filter: path to a file (one per line) or a comma-separated
    /// list of extension names. Omit to include all possible extensions.
    #[arg(long)]
    pub extensions: Option<String>,

    /// Merge multiple APIs of the same spec into a single output file.
    /// Required when combining gl and gles2; behaviour is undefined otherwise.
    #[arg(long)]
    pub merge: bool,

    /// Directory for generated output files.
    #[arg(long, default_value = ".")]
    pub out_path: String,

    /// Suppress informational messages on stderr.
    #[arg(long)]
    pub quiet: bool,

    /// Fetch XML specs from Khronos remote URLs instead of bundled copies.
    #[arg(long)]
    pub fetch: bool,

    #[command(subcommand)]
    pub generator: Generator,
}

#[derive(Subcommand, Debug)]
pub enum Generator {
    /// Generate a C loader.
    C(CArgs),
    /// Generate a Rust loader.
    Rust(RustArgs),
}

#[derive(Args, Debug)]
pub struct CArgs {
    /// Enable bijective function-pointer alias resolution.
    #[arg(long)]
    pub alias: bool,

    /// Include a built-in dlopen/LoadLibrary convenience loader layer.
    #[arg(long)]
    pub loader: bool,
}

#[derive(Args, Debug)]
pub struct RustArgs {
    /// Enable bijective function-pointer alias resolution.
    #[arg(long)]
    pub alias: bool,
}

impl Cli {
    pub fn api_requests(&self) -> Result<Vec<ApiRequest>> {
        self.api
            .split(',')
            .map(|s| ApiRequest::parse(s.trim()))
            .collect()
    }

    /// Returns None (include all) or Some(list of extension names).
    pub fn extension_filter(&self) -> Result<Option<Vec<String>>> {
        let Some(ref spec) = self.extensions else {
            return Ok(None);
        };

        if std::path::Path::new(spec).exists() {
            let text = std::fs::read_to_string(spec)?;
            let list = text
                .lines()
                .map(str::trim)
                .filter(|l| !l.is_empty() && !l.starts_with('#'))
                .map(str::to_string)
                .collect();
            return Ok(Some(list));
        }

        // Treat as an inline comma-separated list.
        let list = spec
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect();
        Ok(Some(list))
    }
}

// ---------------------------------------------------------------------------
// ApiRequest
// ---------------------------------------------------------------------------

/// Normalize an API name to its canonical short form.
///
/// The Khronos XML uses `"vulkan"` in feature and extension `api=`
/// / `supported=` attributes, but the CLI convention is `"vk"`.  This function
/// maps the long form to the short form so the rest of the codebase can use
/// a single canonical name.  All other API names pass through unchanged.
pub fn canonical_api_name(name: &str) -> &str {
    match name {
        "vulkan" => "vk",
        other => other,
    }
}

/// One parsed entry from the `--api` argument.
#[derive(Debug, Clone)]
pub struct ApiRequest {
    /// Canonical API name: "gl", "gles1", "gles2", "egl", "glx", "wgl", "vk".
    pub name: String,
    /// Only meaningful for GL: "core" or "compat".
    pub profile: Option<String>,
    /// Maximum version to include. None means "latest available".
    pub version: Option<Version>,
}

impl ApiRequest {
    /// Parse a single `name[:profile][=major.minor]` token.
    pub fn parse(s: &str) -> Result<Self> {
        let (name_profile, ver_str) = match s.find('=') {
            Some(i) => (&s[..i], Some(&s[i + 1..])),
            None => (s, None),
        };

        let (name, profile) = match name_profile.find(':') {
            Some(i) => (&name_profile[..i], Some(&name_profile[i + 1..])),
            None => (name_profile, None),
        };

        if name.is_empty() {
            bail!("empty API name in --api argument");
        }

        let version = ver_str
            .map(|v| {
                let (maj, min) = v.split_once('.').ok_or_else(|| {
                    anyhow::anyhow!("invalid version '{}', expected major.minor", v)
                })?;
                Ok::<_, anyhow::Error>(Version::new(maj.parse()?, min.parse()?))
            })
            .transpose()?;

        Ok(Self {
            name: canonical_api_name(name).to_string(),
            profile: profile.map(str::to_string),
            version,
        })
    }

    /// Maps the API name to its spec family: "gl", "egl", "glx", "wgl", "vk".
    pub fn spec_name(&self) -> &str {
        match self.name.as_str() {
            "gl" | "gles1" | "gles2" | "glcore" => "gl",
            "egl" => "egl",
            "glx" => "glx",
            "wgl" => "wgl",
            "vk" | "vulkan" => "vk",
            other => other,
        }
    }

    /// True if this request targets GL (desktop or ES).
    #[allow(dead_code)]
    pub fn is_gl_family(&self) -> bool {
        matches!(self.name.as_str(), "gl" | "gles1" | "gles2" | "glcore")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- ApiRequest::parse ----

    #[test]
    fn parse_gl_core_versioned() {
        let r = ApiRequest::parse("gl:core=3.3").unwrap();
        assert_eq!(r.name, "gl");
        assert_eq!(r.profile.as_deref(), Some("core"));
        assert_eq!(r.version, Some(Version::new(3, 3)));
    }

    #[test]
    fn parse_gl_compat_no_version() {
        let r = ApiRequest::parse("gl:compat").unwrap();
        assert_eq!(r.name, "gl");
        assert_eq!(r.profile.as_deref(), Some("compat"));
        assert!(r.version.is_none());
    }

    #[test]
    fn parse_gles2_versioned() {
        let r = ApiRequest::parse("gles2=3.0").unwrap();
        assert_eq!(r.name, "gles2");
        assert!(r.profile.is_none());
        assert_eq!(r.version, Some(Version::new(3, 0)));
    }

    #[test]
    fn parse_vk_versioned() {
        let r = ApiRequest::parse("vk=1.3").unwrap();
        assert_eq!(r.name, "vk");
        assert_eq!(r.version, Some(Version::new(1, 3)));
    }

    #[test]
    fn parse_vulkan_normalizes_to_vk() {
        // "vulkan" is the XML-canonical name; "vk" is the CLI-canonical name.
        // Both must produce the same ApiRequest.
        let r = ApiRequest::parse("vulkan=1.3").unwrap();
        assert_eq!(r.name, "vk", "vulkan should normalize to vk");
        assert_eq!(r.version, Some(Version::new(1, 3)));
    }

    #[test]
    fn parse_vulkan_bare_normalizes_to_vk() {
        let r = ApiRequest::parse("vulkan").unwrap();
        assert_eq!(r.name, "vk");
        assert!(r.version.is_none());
    }

    #[test]
    fn parse_bare_name_no_version() {
        let r = ApiRequest::parse("egl").unwrap();
        assert_eq!(r.name, "egl");
        assert!(r.profile.is_none());
        assert!(r.version.is_none());
    }

    #[test]
    fn parse_empty_name_errors() {
        assert!(ApiRequest::parse("=1.0").is_err());
    }

    #[test]
    fn parse_version_missing_minor_errors() {
        assert!(ApiRequest::parse("gl:core=3").is_err());
    }

    #[test]
    fn parse_version_non_numeric_errors() {
        assert!(ApiRequest::parse("gl:core=three.three").is_err());
    }

    // ---- spec_name() mapping ----

    #[test]
    fn spec_name_gl_family_maps_to_gl() {
        for name in &["gl", "gles1", "gles2", "glcore"] {
            let r = ApiRequest::parse(name).unwrap();
            assert_eq!(r.spec_name(), "gl", "failed for api name '{name}'");
        }
    }

    #[test]
    fn spec_name_passthrough() {
        for name in &["egl", "glx", "wgl"] {
            let r = ApiRequest::parse(name).unwrap();
            assert_eq!(r.spec_name(), *name);
        }
    }

    #[test]
    fn spec_name_vulkan_alias() {
        // Both "vk" and "vulkan" should map to "vk".
        assert_eq!(ApiRequest::parse("vk").unwrap().spec_name(), "vk");
        assert_eq!(ApiRequest::parse("vulkan").unwrap().spec_name(), "vk");
    }
}
