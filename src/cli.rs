//! Command-line interface definitions.

use std::collections::HashSet;

use anyhow::{Result, bail};
use clap::{Args, Parser, Subcommand};

use crate::identity::{Api, Spec};
use crate::ir::Version;

// ---------------------------------------------------------------------------
// Top-level CLI
// ---------------------------------------------------------------------------

#[derive(Parser, Debug)]
#[command(
    name = "gloam",
    version = crate::build_info::VERSION,
    long_version = crate::version::long_version(),
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
    /// is required for GL (core|compatibility). Version is optional (latest if
    /// omitted).  Example: gl:core=3.3,gles2=3.0
    /// Required for generation; ignored by `gloam lock`.
    #[arg(long)]
    pub api: Option<String>,

    /// Extension filter: path to a file (one per line), a comma-separated
    /// list of extension names, or "all" (the default if omitted).  Prefix a
    /// name with `-` to exclude it.  Examples:
    ///   --extensions all,-GL_EXT_direct_state_access
    ///   --extensions GL_KHR_debug,GL_ARB_sync
    ///   --extensions ""              (include no extensions)
    #[arg(long)]
    pub extensions: Option<String>,

    /// Baseline API versions.  Extensions that are fully promoted into these
    /// versions or earlier are excluded — they're guaranteed to be present
    /// in a context of at least the baseline version.  Format matches --api:
    ///   --baseline gl:core=3.3,gles2=3.0
    #[arg(long)]
    pub baseline: Option<String>,

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
    #[cfg(feature = "fetch")]
    #[arg(long)]
    pub fetch: bool,

    /// Pin upstream sources to the provenance recorded in a previous
    /// `.gloam/manifest.json` (or a `gloam lock` snapshot), for reproducible
    /// output.  Requires either --fetch or a gloam build whose bundled files
    /// match the locked blobs.
    #[arg(long, value_name = "MANIFEST")]
    pub lock: Option<std::path::PathBuf>,

    #[command(subcommand)]
    pub generator: Generator,
}

#[derive(Subcommand, Debug)]
pub enum Generator {
    /// Generate a C loader.
    C(CArgs),
    /// Write a provenance-only snapshot manifest (no loader output) pinning
    /// every supported upstream source at the current bundle (or, with --fetch,
    /// upstream HEAD).  Reuse it later with --lock for reproducible generation.
    Lock(LockArgs),
}

#[derive(Args, Debug)]
pub struct LockArgs {
    /// Output path for the snapshot manifest.  If the file already exists,
    /// repos whose pinned files are all byte-identical to it keep their
    /// previously recorded commit/describe, so upstream commits that don't
    /// touch any pinned file don't churn the manifest.  Delete the file to
    /// force a full re-snapshot.
    #[arg(long, default_value = "manifest.json")]
    pub out: String,
}

#[derive(Args, Debug)]
pub struct CArgs {
    /// Enable bijective function-pointer alias resolution.
    #[arg(long)]
    pub alias: bool,

    /// Include a built-in dlopen/LoadLibrary convenience loader layer.
    #[arg(long)]
    pub loader: bool,

    /// Use upstream Vulkan-Headers instead of embedding type definitions.
    /// When set, the generated header includes <vulkan/vulkan_core.h> and
    /// platform-specific headers rather than emitting its own types, enums,
    /// and PFN typedefs.  Only meaningful for Vulkan builds.
    #[arg(long)]
    pub external_headers: bool,
}

impl Cli {
    pub fn api_requests(&self) -> Result<Vec<ApiRequest>> {
        let api = self
            .api
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("--api is required for generation"))?;
        api.split(',')
            .map(|s| ApiRequest::parse(s.trim()))
            .collect()
    }

    pub fn use_fetch(&self) -> bool {
        #[cfg(feature = "fetch")]
        {
            self.fetch
        }
        #[cfg(not(feature = "fetch"))]
        {
            false
        }
    }

    /// Parse the --extensions argument into an `ExtensionFilter`.
    pub fn extension_filter(&self) -> Result<ExtensionFilter> {
        let Some(ref spec) = self.extensions else {
            return Ok(ExtensionFilter::all());
        };

        // Read names from a file or inline comma-separated list.
        let raw_names: Vec<String> = if std::path::Path::new(spec).exists() {
            let text = std::fs::read_to_string(spec)?;
            text.lines()
                .map(str::trim)
                .filter(|l| !l.is_empty() && !l.starts_with('#'))
                .map(str::to_string)
                .collect()
        } else {
            spec.split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect()
        };

        // Split into includes and excludes based on `-` prefix.
        let mut include_all = false;
        let mut includes: Vec<String> = Vec::new();
        let mut excludes: HashSet<String> = HashSet::new();

        for name in raw_names {
            if name.eq_ignore_ascii_case("all") {
                include_all = true;
            } else if let Some(stripped) = name.strip_prefix('-') {
                if !stripped.is_empty() {
                    excludes.insert(stripped.to_string());
                }
            } else {
                includes.push(name);
            }
        }

        // When "all" is combined with explicit names, the explicit names act as
        // baseline-override pins — they survive --baseline exclusion even though
        // "all" means we don't use them for initial inclusion filtering.
        let (include, keep) = if include_all {
            (None, includes.into_iter().collect())
        } else {
            (Some(includes), HashSet::new())
        };
        Ok(ExtensionFilter {
            include,
            exclude: excludes,
            keep,
        })
    }

    /// Parse the --baseline argument into API requests (same format as --api).
    pub fn baseline_requests(&self) -> Result<Vec<ApiRequest>> {
        let Some(ref spec) = self.baseline else {
            return Ok(Vec::new());
        };
        spec.split(',')
            .map(|s| ApiRequest::parse(s.trim()))
            .collect()
    }
}

// ---------------------------------------------------------------------------
// ExtensionFilter
// ---------------------------------------------------------------------------

/// Parsed extension filter from --extensions.
///
/// `include` is `None` for "all extensions" or `Some(list)` for an explicit set.
/// `exclude` is always a set of names to unconditionally remove — applied as a
/// final veto after all selection passes (explicit, dependency, promoted,
/// predecessor, baseline).
/// `keep` is a set of names that override baseline exclusion — used when the
/// user writes `--extensions all,GL_ARB_foo` to pin specific extensions even
/// if they'd otherwise be excluded by --baseline.
#[derive(Debug)]
pub struct ExtensionFilter {
    pub include: Option<Vec<String>>,
    pub exclude: HashSet<String>,
    pub keep: HashSet<String>,
}

impl ExtensionFilter {
    /// No filter — include everything, exclude nothing.
    pub fn all() -> Self {
        Self {
            include: None,
            exclude: HashSet::new(),
            keep: HashSet::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// ApiRequest
// ---------------------------------------------------------------------------

/// One parsed entry from the `--api` argument.
#[derive(Debug, Clone)]
pub struct ApiRequest {
    pub api: Api,
    /// Only meaningful for desktop GL: "core" or "compatibility".  These are
    /// the exact profile tokens the Khronos XML uses in `profile=` attributes;
    /// no aliases are accepted, because profile matching is by exact string
    /// and a non-canonical spelling would silently miss profile-conditional
    /// requires/removes.
    pub profile: Option<String>,
    /// Maximum version to include. None means "latest available".
    pub version: Option<Version>,
}

impl ApiRequest {
    /// Parse and validate a single `name[:profile][=major.minor]` token.
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
        let api = Api::from_cli(name)?;

        match (api, profile) {
            // Desktop GL is the only profiled API, and it is ambiguous
            // without one: XML requires/removes are profile-conditional, so
            // no profile silently yields a core/compat hybrid.
            (Api::Gl, None) => {
                bail!(
                    "API 'gl' requires a profile: use gl:core or gl:compatibility \
                     (e.g. gl:core=3.3)"
                )
            }
            (Api::Gl, Some(p)) if !matches!(p, "core" | "compatibility") => {
                bail!("unknown GL profile '{p}' (expected core or compatibility)")
            }
            (Api::Gl, Some(_)) | (_, None) => {}
            (_, Some(p)) => bail!("API '{api}' does not take a profile (got ':{p}')"),
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
            api,
            profile: profile.map(str::to_string),
            version,
        })
    }

    /// The spec family this request's definitions live in.
    pub fn spec(&self) -> Spec {
        self.api.spec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- ApiRequest::parse ----

    #[test]
    fn parse_gl_core_versioned() {
        let r = ApiRequest::parse("gl:core=3.3").unwrap();
        assert_eq!(r.api, Api::Gl);
        assert_eq!(r.profile.as_deref(), Some("core"));
        assert_eq!(r.version, Some(Version::new(3, 3)));
    }

    #[test]
    fn parse_gl_compatibility_no_version() {
        let r = ApiRequest::parse("gl:compatibility").unwrap();
        assert_eq!(r.api, Api::Gl);
        assert_eq!(r.profile.as_deref(), Some("compatibility"));
        assert!(r.version.is_none());
    }

    #[test]
    fn parse_gl_compat_shortcut_is_rejected() {
        // "compat" is not a Khronos profile token; profile matching is by
        // exact string, so accepting it would silently miss
        // profile="compatibility" requires/removes.
        let err = ApiRequest::parse("gl:compat=3.3").unwrap_err().to_string();
        assert!(err.contains("unknown GL profile 'compat'"), "{err}");
    }

    #[test]
    fn parse_gles2_versioned() {
        let r = ApiRequest::parse("gles2=3.0").unwrap();
        assert_eq!(r.api, Api::Gles2);
        assert!(r.profile.is_none());
        assert_eq!(r.version, Some(Version::new(3, 0)));
    }

    #[test]
    fn parse_vk_versioned() {
        let r = ApiRequest::parse("vk=1.3").unwrap();
        assert_eq!(r.api, Api::Vk);
        assert_eq!(r.version, Some(Version::new(1, 3)));
    }

    #[test]
    fn parse_vulkan_normalizes_to_vk() {
        // "vulkan" is the XML-canonical name; "vk" is the CLI-canonical name.
        // Both must produce the same ApiRequest.
        let r = ApiRequest::parse("vulkan=1.3").unwrap();
        assert_eq!(r.api, Api::Vk, "vulkan should normalize to vk");
        assert_eq!(r.version, Some(Version::new(1, 3)));
    }

    #[test]
    fn parse_bare_name_no_version() {
        let r = ApiRequest::parse("egl").unwrap();
        assert_eq!(r.api, Api::Egl);
        assert!(r.profile.is_none());
        assert!(r.version.is_none());
    }

    #[test]
    fn parse_empty_name_errors() {
        assert!(ApiRequest::parse("=1.0").is_err());
    }

    #[test]
    fn parse_unknown_api_errors_at_parse_time() {
        let err = ApiRequest::parse("dx12").unwrap_err().to_string();
        assert!(err.contains("unknown API 'dx12'"), "{err}");
    }

    #[test]
    fn parse_gl_without_profile_errors() {
        let err = ApiRequest::parse("gl=3.3").unwrap_err().to_string();
        assert!(err.contains("requires a profile"), "{err}");
    }

    #[test]
    fn parse_gl_bad_profile_errors() {
        let err = ApiRequest::parse("gl:corr=3.3").unwrap_err().to_string();
        assert!(err.contains("unknown GL profile 'corr'"), "{err}");
    }

    #[test]
    fn parse_profile_on_non_gl_errors() {
        let err = ApiRequest::parse("vk:core=1.3").unwrap_err().to_string();
        assert!(err.contains("does not take a profile"), "{err}");
    }

    #[test]
    fn parse_version_missing_minor_errors() {
        assert!(ApiRequest::parse("gl:core=3").is_err());
    }

    #[test]
    fn parse_version_non_numeric_errors() {
        assert!(ApiRequest::parse("gl:core=three.three").is_err());
    }

    // ---- spec() mapping ----

    #[test]
    fn spec_gl_family_maps_to_gl() {
        for name in &["gl:core", "gles1", "gles2", "glcore"] {
            let r = ApiRequest::parse(name).unwrap();
            assert_eq!(r.spec(), Spec::Gl, "failed for api request '{name}'");
        }
    }

    #[test]
    fn spec_passthrough() {
        assert_eq!(ApiRequest::parse("egl").unwrap().spec(), Spec::Egl);
        assert_eq!(ApiRequest::parse("glx").unwrap().spec(), Spec::Glx);
        assert_eq!(ApiRequest::parse("wgl").unwrap().spec(), Spec::Wgl);
    }

    #[test]
    fn spec_vulkan_alias() {
        assert_eq!(ApiRequest::parse("vk").unwrap().spec(), Spec::Vk);
        assert_eq!(ApiRequest::parse("vulkan").unwrap().spec(), Spec::Vk);
    }
}
