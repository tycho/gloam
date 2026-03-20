//! All IR types produced by the XML parser. These are "raw" — direct
//! representations of what the XML says, before feature-set resolution or
//! index assignment.

use indexmap::IndexMap;

// ---------------------------------------------------------------------------
// Version
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32) -> Self {
        Self { major, minor }
    }
    /// Pack into a u16 for cheap comparisons: (major << 8) | minor.
    pub fn packed(&self) -> u16 {
        ((self.major as u16) << 8) | (self.minor as u16)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.packed().cmp(&other.packed())
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// A single `<type>` element from the XML, before any deduplication.
/// Multiple variants of the same name (differing by `api=`) are kept as
/// separate entries; the resolver picks the right one later.
#[derive(Debug, Clone)]
pub struct RawType {
    pub name: String,
    pub api: Option<String>,
    /// "basetype", "bitmask", "define", "enum", "funcpointer",
    /// "group", "handle", "include", "struct", "union", or empty.
    pub category: String,
    /// Name of another type this one depends on (from `requires=` attr).
    pub requires: Option<String>,
    /// If present, this type is an alias of another.
    pub alias: Option<String>,
    /// 32 or 64 for Vulkan bitmask types; None otherwise.
    pub bitwidth: Option<u32>,
    /// The fully-assembled C text (apientry substituted, name/type sub-elements
    /// inlined). Ready to emit verbatim into a C header.
    pub raw_c: String,
    /// Platform protection macro (e.g. "VK_USE_PLATFORM_WIN32_KHR").
    pub protect: Option<String>,
}

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

/// A single enum value, either from a flat `<enums>` block or from an
/// `<enum extends=...>` inside an `<extension>`.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RawEnum {
    pub name: String,
    /// Computed string value (decimal or 0x-hex). None for pure alias enums.
    pub value: Option<String>,
    pub api: Option<String>,
    /// "u" or "ull" etc., as present in the XML `type=` attr.
    pub type_suffix: Option<String>,
    /// This enum is an alias of another enum name.
    pub alias: Option<String>,
    pub comment: String,
    /// For Vulkan typed enums: the enum type this value belongs to.
    pub parent_type: Option<String>,
}

/// A Vulkan typed enum group (the `<enums type="enum"|"bitmask">` element).
#[derive(Debug, Clone)]
pub struct RawEnumGroup {
    pub name: String,
    pub bitwidth: Option<u32>,
    pub values: Vec<RawEnum>,
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

/// A single parsed command parameter.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RawParam {
    pub name: String,
    /// Full C type text for the parameter (e.g. "const GLubyte *", "GLenum").
    pub type_raw: String,
    /// The base type name extracted from `<ptype>` or from the text,
    /// used for Vulkan command-scope inference.
    pub type_name: String,
    /// Only set for multi-API commands where a param differs between APIs.
    pub api: Option<String>,
}

/// A single parsed command.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RawCommand {
    pub name: String,
    pub api: Option<String>,
    /// C return type text (e.g. "void", "const GLubyte *", "VkResult").
    pub return_type: String,
    pub params: Vec<RawParam>,
    /// If non-None, this command is an alias of the named command.
    pub alias: Option<String>,
}

/// Vulkan command scope, inferred from the first parameter type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandScope {
    /// Used exclusively for `vkGetInstanceProcAddr` (must use dlsym).
    Unknown,
    /// First param is absent or not a dispatchable handle.
    Global,
    /// First param is VkInstance or VkPhysicalDevice.
    Instance,
    /// First param is VkDevice, VkQueue, or VkCommandBuffer.
    Device,
}

impl CommandScope {
    pub fn c_name(self) -> &'static str {
        match self {
            Self::Unknown => "GloamCommandScopeUnknown",
            Self::Global => "GloamCommandScopeGlobal",
            Self::Instance => "GloamCommandScopeInstance",
            Self::Device => "GloamCommandScopeDevice",
        }
    }
}

// ---------------------------------------------------------------------------
// Features and extensions
// ---------------------------------------------------------------------------

/// The items required by one `<require>` block inside a feature or extension.
#[derive(Debug, Clone, Default)]
pub struct Require {
    /// API restriction on this block (None = applies to all).
    pub api: Option<String>,
    /// Profile restriction ("core", "compat"), None = all profiles.
    pub profile: Option<String>,
    pub types: Vec<String>,
    pub enums: Vec<String>,
    pub commands: Vec<String>,
}

/// The items removed by one `<remove>` block inside a feature.
#[derive(Debug, Clone, Default)]
pub struct Remove {
    pub profile: Option<String>,
    pub commands: Vec<String>,
    pub enums: Vec<String>,
}

/// A `<feature>` element (a versioned API level).
#[derive(Debug, Clone)]
pub struct RawFeature {
    /// e.g. "GL_VERSION_3_3", "VK_VERSION_1_0"
    pub name: String,
    /// Single API name (already split; a feature with api="gl,gles2" becomes two entries).
    pub api: String,
    pub version: Version,
    pub requires: Vec<Require>,
    pub removes: Vec<Remove>,
}

/// An `<extension>` element.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RawExtension {
    /// e.g. "GL_ARB_sync"
    pub name: String,
    /// Which API names this extension supports.
    pub supported: Vec<String>,
    pub requires: Vec<Require>,
    /// Platform protection macros (may be multiple).
    pub protect: Vec<String>,
    /// Extension registry number, used for enum offset calculation.
    pub number: Option<u32>,
}

// ---------------------------------------------------------------------------
// Top-level raw spec
// ---------------------------------------------------------------------------

/// Everything parsed from one specification's XML (primary + all supplementals),
/// before any feature-set resolution or indexing.
#[derive(Debug)]
pub struct RawSpec {
    /// The canonical spec name: "gl", "egl", "glx", "wgl", "vk".
    pub spec_name: String,

    /// Vulkan platform registry: platform name → protect macro.
    /// e.g. "xlib" → "VK_USE_PLATFORM_XLIB_KHR".
    /// Empty for non-Vulkan specs.
    /// Stored for completeness; the protect strings are propagated onto
    /// RawExtension.protect during parsing and not re-read from here.
    #[allow(dead_code)]
    pub platforms: IndexMap<String, String>,

    /// All type definitions, in topological dependency order.
    /// Multiple entries may share a name if they have different `api=` values.
    pub types: Vec<RawType>,

    /// Vulkan typed enum groups (from `<enums type="enum"|"bitmask">`).
    pub enum_groups: Vec<RawEnumGroup>,

    /// GL-style flat `#define` constants.
    /// Insertion order is preserved (for deterministic output).
    pub flat_enums: IndexMap<String, RawEnum>,

    /// All commands, keyed by name. For commands that exist in multiple API
    /// variants, only one representative is stored (api=None where applicable,
    /// else the most general variant). Alias fixup has already been applied.
    pub commands: IndexMap<String, RawCommand>,

    /// Feature version blocks, sorted by api then ascending version.
    pub features: Vec<RawFeature>,

    /// All extensions from the combined XML.
    pub extensions: Vec<RawExtension>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_ordering() {
        assert!(Version::new(3, 3) > Version::new(3, 2));
        assert!(Version::new(4, 0) > Version::new(3, 9));
        assert!(Version::new(1, 1) > Version::new(1, 0));
        assert_eq!(Version::new(2, 0), Version::new(2, 0));
    }

    #[test]
    fn version_packed_encodes_correctly() {
        assert_eq!(Version::new(3, 3).packed(), 0x0303);
        assert_eq!(Version::new(1, 0).packed(), 0x0100);
        assert_eq!(Version::new(4, 6).packed(), 0x0406);
    }

    #[test]
    fn version_packed_ordering_matches_semantic_ordering() {
        // packed() ordering must agree with Ord so comparisons in generated
        // C are correct (the generated code compares packed versions directly).
        let pairs = [
            (Version::new(1, 0), Version::new(1, 1)),
            (Version::new(3, 2), Version::new(3, 3)),
            (Version::new(3, 9), Version::new(4, 0)),
        ];
        for (a, b) in &pairs {
            assert!(a.packed() < b.packed(), "{a} < {b}");
        }
    }

    #[test]
    fn version_display() {
        assert_eq!(Version::new(3, 3).to_string(), "3.3");
        assert_eq!(Version::new(1, 0).to_string(), "1.0");
    }
}
