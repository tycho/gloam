//! Public output types for the resolver.
//!
//! These are pure data types with no logic — only derives.  They constitute
//! the public interface of the resolve module: `FeatureSet` and all its
//! constituent types are consumed by generators, the preamble builder, and
//! templates (via Serde serialization).

use indexmap::IndexMap;
use serde::Serialize;

use crate::identity::Spec;

// ---------------------------------------------------------------------------
// FeatureSet — the resolved, indexed output
// ---------------------------------------------------------------------------

/// Everything a code generator needs, fully indexed and sorted.
#[derive(Debug, Serialize)]
pub struct FeatureSet {
    /// Typed spec identity, for Rust-side consumers (render models).
    /// Not serialized — templates use the stringly fields below.
    #[serde(skip)]
    pub spec: Spec,
    /// "gl", "egl", "glx", "wgl", "vk"
    pub spec_name: String,
    /// Display name: "GL", "EGL", "GLX", "WGL", "Vulkan"
    pub display_name: String,
    /// API names active in this feature set (may be multiple for merged builds).
    pub apis: Vec<String>,
    pub is_merged: bool,
    pub is_vulkan: bool,
    pub is_gl_family: bool,

    /// Version features, in ascending version order.
    /// featArray index = position in this Vec.
    pub features: Vec<Feature>,

    /// Extensions, alphabetically sorted by name.
    /// extArray index = position in this Vec.
    pub extensions: Vec<Extension>,

    /// All commands, in declaration order (core version order then ext order).
    /// pfnArray index = position in this Vec.
    pub commands: Vec<Command>,

    /// Types to emit (in dependency order).
    pub types: Vec<TypeDef>,

    /// GL-style flat #define constants.
    pub flat_enums: Vec<FlatEnum>,

    /// Vulkan typed enum groups.
    pub enum_groups: Vec<EnumGroup>,

    /// Feature PFN range table (shared across APIs in a merged build).
    pub feature_pfn_ranges: Vec<PfnRange>,

    /// Per-API extension PFN range tables.
    /// Key = api name (e.g. "gl", "gles2").
    pub ext_pfn_ranges: IndexMap<String, Vec<PfnRange>>,

    /// Per-API extension index subsets for s_extIdx[] in find_extensions_*.
    /// Key = api name.  Value = extArray indices relevant to that API.
    pub ext_subset_indices: IndexMap<String, Vec<u16>>,

    /// Bijective alias pairs.  Empty unless --alias was requested.
    pub alias_pairs: Vec<AliasPair>,

    /// Auxiliary headers that must be copied to the output include tree.
    /// Derived from the `requires=` attributes of selected types.
    /// Paths are relative to the include root, e.g. "KHR/khrplatform.h".
    pub required_headers: Vec<String>,

    /// Provenance registry keys of every upstream source that contributed to
    /// this loader: the primary spec XML, the request-aware supplementals
    /// actually merged, the required auxiliary headers, and xxhash.h (always
    /// emitted).  Sorted and deduplicated.  Drives attribution and the manifest.
    pub source_keys: Vec<String>,

    /// Extensions excluded by explicit `-` prefix in --extensions.
    pub excluded_explicit: Vec<String>,
    /// Extensions excluded because they are fully promoted into --baseline versions.
    pub excluded_baseline: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Feature {
    pub index: u16,
    /// Short name with no API prefix, e.g. "VERSION_3_3".
    pub short_name: String,
    /// Full feature name, e.g. "GL_VERSION_3_3".
    pub full_name: String,
    pub version: SerVersion,
    /// Packed (major << 8 | minor) — used for version comparison in find_core_*.
    pub packed: u16,
    /// Which API this feature belongs to, e.g. "gl", "gles2".
    /// Used in templates to filter features per-API in merged builds.
    pub api: String,
}

#[derive(Debug, Serialize)]
pub struct SerVersion {
    pub major: u32,
    pub minor: u32,
}

#[derive(Debug, Serialize)]
pub struct Extension {
    pub index: u16,
    /// Full extension name e.g. "GL_ANGLE_framebuffer_blit".
    pub name: String,
    /// Short name (no API prefix) e.g. "ANGLE_framebuffer_blit".
    pub short_name: String,
    /// Pre-baked XXH3_64 hash as "0x...ULL" literal.
    pub hash: String,
    /// Platform protection macros (if any).
    pub protect: Vec<String>,
    /// Why this extension was included in the feature set.
    pub reason: SelectionReason,
}

#[derive(Debug, Serialize)]
pub struct Command {
    pub index: u16,
    /// Full name e.g. "glCullFace"
    pub name: String,
    /// Member name in the context struct e.g. "CullFace"
    pub short_name: String,
    /// C return type text e.g. "void", "const GLubyte *"
    pub return_type: String,
    /// Parameters as declared in the spec (raw C type text + name).
    pub params: Vec<Param>,
    /// Vulkan scope name (empty string for non-Vulkan).
    pub scope: String,
    /// Platform guard macro (if any).
    pub protect: Option<String>,
    /// Byte offset of this command's name within the packed name blob.
    /// Computed after the command list is finalized.
    pub name_offset: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct Param {
    pub type_raw: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TypeDef {
    /// The canonical type name as declared in the spec.
    pub name: String,
    pub raw_c: String,
    /// Serialized as the XML category string (`"include"`, `"struct"`, ...).
    pub category: crate::ir::TypeCategory,
    /// Platform protection macros.  Empty = unconditional.
    pub protect: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FlatEnum {
    pub name: String,
    pub value: String,
    /// Always a numeric literal, even for aliases.  Used in the pre-C23
    /// `static const` path where referencing another variable is not a
    /// constant expression on some compilers (MSVC C2099).
    pub literal_value: String,
    pub comment: String,
    /// Platform protection macros.  Empty = unconditional.
    pub protect: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct EnumGroup {
    pub name: String,
    pub is_bitmask: bool,
    pub bitwidth: u32,
    pub values: Vec<FlatEnum>,
}

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
pub struct PfnRange {
    /// Index into featArray or extArray.
    pub extension: u16,
    /// First pfnArray index covered by this range.
    pub start: u16,
    /// Number of consecutive pfnArray slots.
    pub count: u16,
}

#[derive(Debug, Serialize)]
pub struct AliasPair {
    pub canonical: u16,
    pub secondary: u16,
}

// ---------------------------------------------------------------------------
// SelectionReason
// ---------------------------------------------------------------------------

/// Why an extension was included in the feature set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SelectionReason {
    /// Explicitly listed in --extensions or included because no filter was set.
    Explicit,
    /// No --extensions filter was given — all supported extensions are included.
    AllExtensions,
    /// WGL mandatory extensions (always required for WGL to function).
    Mandatory,
    /// Auto-included because a selected extension declares it as a dependency
    /// (via the `requires=` or `depends=` XML attribute).
    Dependency,
    /// Auto-included because its commands were promoted into a requested core version.
    Promoted,
    /// Auto-included as a predecessor of an already-selected extension.
    Predecessor,
}
