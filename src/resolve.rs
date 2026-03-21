//! Feature-set resolution.
//!
//! Takes a `RawSpec` plus the user's API requests and produces a `FeatureSet`
//! with all arrays indexed, range tables built, and alias pairs computed.
//! This is the bridge between the parser and the code generators.

use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::Result;
use indexmap::IndexMap;
use serde::Serialize;

use crate::cli::{ApiRequest, Cli, canonical_api_name, xml_api_name};
use crate::fetch;
use crate::ir::{RawCommand, RawSpec};
use crate::parse;
use crate::parse::commands::infer_vulkan_scope;
use crate::parse::types::ident_words;

// ---------------------------------------------------------------------------
// FeatureSet — the resolved, indexed output
// ---------------------------------------------------------------------------

/// Everything a code generator needs, fully indexed and sorted.
#[derive(Debug, Serialize)]
pub struct FeatureSet {
    /// "gl", "egl", "glx", "wgl", "vk"
    pub spec_name: String,
    /// Display name: "GL", "EGL", "GLX", "WGL", "Vulkan"
    pub display_name: String,
    /// API names active in this feature set (may be multiple for merged builds).
    pub apis: Vec<String>,
    pub is_merged: bool,
    pub is_vulkan: bool,
    pub is_gl_family: bool,
    /// C context struct name, e.g. "GloamGLContext", "GloamGLES2Context".
    /// Precomputed so templates can use it directly without a filter.
    pub context_name: String,

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
    /// PFN typedef name e.g. "PFNGLCULLFACEPROC"
    pub pfn_type: String,
    /// C return type text e.g. "void", "const GLubyte *"
    pub return_type: String,
    /// Formatted parameter list for PFN typedef (empty → "void").
    pub params_str: String,
    /// Full params for IntelliSense prototypes.
    pub params: Vec<Param>,
    /// Vulkan scope name (empty string for non-Vulkan).
    pub scope: String,
    /// Platform guard macro (if any).
    pub protect: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Param {
    pub type_raw: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct TypeDef {
    /// The canonical type name as declared in the spec.
    pub name: String,
    pub raw_c: String,
    pub category: String,
    /// Platform protection macros.  Empty = unconditional.
    pub protect: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct FlatEnum {
    pub name: String,
    pub value: String,
    pub comment: String,
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
// Protection lattice
// ---------------------------------------------------------------------------

/// Tracks the platform-protection state for a type or include.
///
/// The lattice has two states:
///   - `Guarded(macros)` — protected by a set of `#if defined(...)` macros.
///   - `Unconditional` — required by at least one unprotected context, so no
///     guard is needed.
///
/// Once a value reaches `Unconditional` it stays there (absorbing element).
#[derive(Debug, Clone)]
enum Protection {
    Unconditional,
    Guarded(Vec<String>),
}

impl Protection {
    fn new_guarded() -> Self {
        Self::Guarded(Vec::new())
    }

    /// Merge protection information from one extension.  If the extension is
    /// unprotected, the result becomes unconditional.  Otherwise the
    /// extension's guards are unioned in.
    fn add_extension(&mut self, ext_protect: &[String]) {
        match self {
            Self::Unconditional => {} // absorbing — nothing to do
            Self::Guarded(guards) => {
                if ext_protect.is_empty() {
                    *self = Self::Unconditional;
                } else {
                    for p in ext_protect {
                        if !guards.contains(p) {
                            guards.push(p.clone());
                        }
                    }
                }
            }
        }
    }

    /// Convert to the final representation: empty Vec = unconditional.
    fn into_vec(self) -> Vec<String> {
        match self {
            Self::Unconditional => Vec::new(),
            Self::Guarded(v) if v.is_empty() => Vec::new(),
            Self::Guarded(mut v) => {
                v.sort();
                v
            }
        }
    }

    #[allow(dead_code)]
    fn is_unconditional(&self) -> bool {
        matches!(self, Self::Unconditional)
    }
}

// ---------------------------------------------------------------------------
// GL auto-exclude set
// ---------------------------------------------------------------------------

/// Type names that GL specs auto-include but that we never emit (they map to
/// system or bundled headers instead).
const GL_AUTO_EXCLUDE: &[&str] = &["stddef", "khrplatform", "inttypes"];

fn is_gl_auto_excluded(name: &str) -> bool {
    GL_AUTO_EXCLUDE.contains(&name)
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

pub fn build_feature_sets(cli: &Cli) -> Result<Vec<FeatureSet>> {
    let requests = cli.api_requests()?;
    let ext_filter = cli.extension_filter()?;
    let promoted = cli.promoted;
    let predecessors = cli.predecessors;

    let alias = match &cli.generator {
        crate::cli::Generator::C(c) => c.alias,
        crate::cli::Generator::Rust(r) => r.alias,
    };

    // Group requests by spec family only for merged builds.
    // For non-merged builds, resolve each API request independently
    // so the generator produces separate files per API.
    let mut feature_sets = Vec::new();

    if cli.merge {
        let mut by_spec: IndexMap<String, Vec<ApiRequest>> = IndexMap::new();
        for req in requests {
            by_spec
                .entry(req.spec_name().to_string())
                .or_default()
                .push(req);
        }
        for (spec_name, reqs) in &by_spec {
            let sources = fetch::load_spec(spec_name, cli.fetch)?;
            let raw = parse::parse(&sources, spec_name)?;
            let fs =
                resolve_feature_set(&raw, reqs, &ext_filter, true, alias, promoted, predecessors)?;
            feature_sets.push(fs);
        }
    } else {
        for req in &requests {
            let spec_name = req.spec_name();
            let sources = fetch::load_spec(spec_name, cli.fetch)?;
            let raw = parse::parse(&sources, spec_name)?;
            let fs = resolve_feature_set(
                &raw,
                std::slice::from_ref(req),
                &ext_filter,
                false,
                alias,
                promoted,
                predecessors,
            )?;
            feature_sets.push(fs);
        }
    }

    Ok(feature_sets)
}

// ---------------------------------------------------------------------------
// Core resolution
// ---------------------------------------------------------------------------

fn resolve_feature_set(
    raw: &RawSpec,
    requests: &[ApiRequest],
    ext_filter: &Option<Vec<String>>,
    is_merged: bool,
    want_aliases: bool,
    want_promoted: bool,
    want_predecessors: bool,
) -> Result<FeatureSet> {
    let spec_name = &raw.spec_name;
    let is_vulkan = spec_name == "vk";
    let is_gl_family = matches!(spec_name.as_str(), "gl" | "egl" | "glx" | "wgl");

    let display_name = match spec_name.as_str() {
        "gl" => "GL",
        "egl" => "EGL",
        "glx" => "GLX",
        "wgl" => "WGL",
        "vk" => "Vulkan",
        _ => spec_name.as_str(),
    };

    // api_names uses the XML-canonical form ("vulkan" not "vk") because
    // these flow into generated C symbol suffixes (kExtIdx_vulkan, etc.)
    // and IndexMap keys used by the templates.  File stems come from
    // spec_name ("vk"), not from here.
    let api_names: Vec<String> = requests
        .iter()
        .map(|r| xml_api_name(&r.name).to_string())
        .collect();

    // ------------------------------------------------------------------
    // Step 1: Determine which features (versions) are selected.
    // ------------------------------------------------------------------
    let selected_features = select_features(raw, requests);

    // ------------------------------------------------------------------
    // Step 2: Collect required names from selected features.
    // ------------------------------------------------------------------
    let mut req_types: HashSet<String> = HashSet::new();
    let mut req_enums: HashSet<String> = HashSet::new();
    let mut req_commands: IndexMap<String, ()> = IndexMap::new(); // preserves order
    let mut removed_commands: HashSet<String> = HashSet::new();
    let mut removed_enums: HashSet<String> = HashSet::new();
    // Per-API core command sets used by --promoted to scope promotion checks.
    // Keyed by API name (e.g. "gl", "gles2"); values exclude profile-removed commands.
    let mut per_api_core_cmds: HashMap<String, HashSet<String>> = HashMap::new();

    for feat in &selected_features {
        let req_for_api = requests.iter().find(|r| r.name == feat.api);
        let profile = req_for_api.and_then(|r| r.profile.as_deref());
        let api_cmds = per_api_core_cmds.entry(feat.api.clone()).or_default();

        for require in &feat.raw.requires {
            if !api_profile_matches(
                require.api.as_deref(),
                require.profile.as_deref(),
                &feat.api,
                profile,
            ) {
                continue;
            }
            req_types.extend(require.types.iter().cloned());
            req_enums.extend(require.enums.iter().cloned());
            for cmd in &require.commands {
                req_commands.entry(cmd.clone()).or_insert(());
                api_cmds.insert(cmd.clone());
            }
        }
        for remove in &feat.raw.removes {
            if !profile_matches(remove.profile.as_deref(), profile) {
                continue;
            }
            removed_commands.extend(remove.commands.iter().cloned());
            removed_enums.extend(remove.enums.iter().cloned());
            // Apply removes inline — features are processed in version order so
            // each version's removes are applied immediately after its requires.
            for cmd in &remove.commands {
                api_cmds.remove(cmd.as_str());
            }
        }
    }
    for cmd in &removed_commands {
        req_commands.shift_remove(cmd.as_str());
    }

    // ------------------------------------------------------------------
    // Step 3: Determine which extensions are selected.
    // ------------------------------------------------------------------
    let selected_exts = select_extensions(
        raw,
        requests,
        ext_filter,
        spec_name,
        &per_api_core_cmds,
        want_promoted,
        want_predecessors,
    );

    // ------------------------------------------------------------------
    // Step 4: Collect additional required names from extensions.
    // ------------------------------------------------------------------
    let mut ext_commands: IndexMap<String, usize> = IndexMap::new(); // cmd -> ext_index

    for (ext_idx, ext) in selected_exts.iter().enumerate() {
        for require in &ext.raw.requires {
            for api in &api_names {
                if !api_profile_matches(require.api.as_deref(), None, api, None) {
                    continue;
                }
                req_types.extend(require.types.iter().cloned());
                req_enums.extend(require.enums.iter().cloned());
                for e in &require.enums {
                    removed_enums.remove(e.as_str());
                }
                for cmd in &require.commands {
                    // Core commands already in req_commands stay there.
                    if !req_commands.contains_key(cmd.as_str()) {
                        ext_commands.entry(cmd.clone()).or_insert(ext_idx);
                    }
                }
            }
        }
    }

    for e in &removed_enums {
        req_enums.remove(e.as_str());
    }

    // ------------------------------------------------------------------
    // Step 5: Build the indexed command list.
    // Core functions first (in req_commands order), then extension functions.
    // ------------------------------------------------------------------
    let core_cmd_names: Vec<String> = req_commands.keys().cloned().collect();
    let ext_cmd_names: Vec<String> = ext_commands.keys().cloned().collect();

    let all_cmd_names: Vec<&str> = core_cmd_names
        .iter()
        .chain(ext_cmd_names.iter())
        .map(String::as_str)
        .collect();

    let pfn_prefix = api_pfn_prefix(spec_name);
    let name_prefix = api_name_prefix(spec_name);
    let cmd_protect_map = build_command_protect_map(&selected_exts);

    let mut commands: Vec<Command> = Vec::with_capacity(all_cmd_names.len());
    for (idx, &cmd_name) in all_cmd_names.iter().enumerate() {
        let raw_cmd = match raw.commands.get(cmd_name) {
            Some(c) => c,
            None => {
                eprintln!("warning: command '{}' required but not in spec", cmd_name);
                continue;
            }
        };

        let scope = if is_vulkan {
            infer_vulkan_scope(raw_cmd).c_name().to_string()
        } else {
            String::new()
        };

        let protect = cmd_protect_map.get(cmd_name).cloned();

        commands.push(build_command(
            idx as u16,
            raw_cmd,
            &scope,
            protect,
            pfn_prefix,
            name_prefix,
        ));
    }

    // ------------------------------------------------------------------
    // Step 6: Build indexed feature and extension lists.
    // ------------------------------------------------------------------
    let features: Vec<Feature> = selected_features
        .iter()
        .enumerate()
        .map(|(i, sf)| {
            let ver = &sf.raw.version;
            let short = version_short_name(&sf.raw.name, &sf.api);
            Feature {
                index: i as u16,
                full_name: sf.raw.name.clone(),
                short_name: short,
                version: SerVersion {
                    major: ver.major,
                    minor: ver.minor,
                },
                packed: ver.packed(),
                api: sf.api.clone(),
            }
        })
        .collect();

    // Extensions are sorted alphabetically.
    let mut sorted_exts: Vec<_> = selected_exts.iter().collect();
    sorted_exts.sort_by_key(|e| e.raw.name.as_str());

    let extensions: Vec<Extension> = sorted_exts
        .iter()
        .enumerate()
        .map(|(i, e)| build_extension(i as u16, e))
        .collect();

    // Ext name → sorted index (for building range tables).
    let ext_index_map: HashMap<&str, u16> = extensions
        .iter()
        .map(|e| (e.name.as_str(), e.index))
        .collect();

    // ------------------------------------------------------------------
    // Step 7: Build PFN range tables.
    // ------------------------------------------------------------------
    let feature_pfn_ranges = build_feature_pfn_ranges(&selected_features, &features, &commands);

    // Per-API extension PFN ranges and subset indices.
    let mut ext_pfn_ranges: IndexMap<String, Vec<PfnRange>> = IndexMap::new();
    let mut ext_subset_indices: IndexMap<String, Vec<u16>> = IndexMap::new();

    for api in &api_names {
        let (ranges, indices) = build_ext_pfn_ranges(
            api,
            &selected_exts,
            &ext_commands,
            &ext_index_map,
            &commands,
        );
        ext_pfn_ranges.insert(api.clone(), ranges);
        ext_subset_indices.insert(api.clone(), indices);
    }

    // ------------------------------------------------------------------
    // Step 8: Types to emit.
    // ------------------------------------------------------------------
    // Iteratively expand req_types to a fixed point: any type referenced
    // in the raw_c of a selected type must itself be selected.  This catches
    // member pointer types like VkPipelineLibraryCreateInfoKHR that are used
    // inside required structs but never appear in any <require><type> block.
    // Auto-included categories (define/basetype/bitmask/funcpointer/enum/
    // handle) are also treated as seeds for the expansion.
    if is_vulkan {
        let type_names: HashSet<&str> = raw.types.iter().map(|t| t.name.as_str()).collect();

        // Seed req_types with parameter and return types from all selected commands.
        // This catches types that are only referenced as command parameters rather
        // than being explicitly listed in <require><type> blocks — e.g.
        // VkViewportSwizzleNV appears only as a parameter of vkCmdSetViewportSwizzleNV,
        // which VK_EXT_shader_object pulls in without also selecting VK_NV_viewport_swizzle.
        for &cmd_name in &all_cmd_names {
            if let Some(raw_cmd) = raw.commands.get(cmd_name) {
                for param in &raw_cmd.params {
                    if !param.type_name.is_empty() {
                        req_types.insert(param.type_name.clone());
                    }
                }
            }
        }

        loop {
            let mut added = false;
            for t in &raw.types {
                if t.raw_c.is_empty() {
                    continue;
                }
                let auto = matches!(
                    t.category.as_str(),
                    "define" | "basetype" | "bitmask" | "funcpointer" | "enum" | "handle"
                );
                if !auto && !req_types.contains(&t.name) {
                    continue;
                }
                for word in crate::parse::types::ident_words(&t.raw_c) {
                    if type_names.contains(word) && req_types.insert(word.to_string()) {
                        added = true;
                    }
                }
            }
            if !added {
                break;
            }
        }
    }

    let types = build_type_list(raw, &req_types, spec_name, is_vulkan, &selected_exts);

    // ------------------------------------------------------------------
    // Step 9: Flat enums and enum groups.
    // ------------------------------------------------------------------
    let flat_enums = build_flat_enums(raw, &req_enums, is_vulkan);
    let enum_groups = build_enum_groups(raw);

    // ------------------------------------------------------------------
    // Step 10: Alias pairs (optional).
    // ------------------------------------------------------------------
    let alias_pairs = if want_aliases {
        build_alias_pairs(raw, &commands)
    } else {
        Vec::new()
    };

    let required_headers = collect_required_headers(raw, &req_types, spec_name);

    let context_name = build_context_name(spec_name);

    Ok(FeatureSet {
        spec_name: spec_name.clone(),
        display_name: display_name.to_string(),
        apis: api_names,
        is_merged,
        is_vulkan,
        is_gl_family,
        context_name,
        features,
        extensions,
        commands,
        types,
        flat_enums,
        enum_groups,
        feature_pfn_ranges,
        ext_pfn_ranges,
        ext_subset_indices,
        alias_pairs,
        required_headers,
    })
}

// ---------------------------------------------------------------------------
// Feature selection
// ---------------------------------------------------------------------------

struct SelectedFeature<'a> {
    api: String,
    raw: &'a crate::ir::RawFeature,
}

fn select_features<'a>(raw: &'a RawSpec, requests: &[ApiRequest]) -> Vec<SelectedFeature<'a>> {
    let mut selected = Vec::new();
    for req in requests {
        let max_ver = req.version.clone();
        for feat in &raw.features {
            if canonical_api_name(&feat.api) != canonical_api_name(&req.name) {
                continue;
            }
            if let Some(ref mv) = max_ver
                && feat.version > *mv
            {
                continue;
            }
            selected.push(SelectedFeature {
                api: req.name.clone(),
                raw: feat,
            });
        }
    }
    // Sort: GL versions first, then GLES, matching the spec's ordering rule.
    selected.sort_by(|a, b| {
        api_order(&a.api)
            .cmp(&api_order(&b.api))
            .then_with(|| a.raw.version.cmp(&b.raw.version))
    });
    selected
}

fn api_order(api: &str) -> u8 {
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

// ---------------------------------------------------------------------------
// Extension selection
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

struct SelectedExt<'a> {
    raw: &'a crate::ir::RawExtension,
    reason: SelectionReason,
}

fn select_extensions<'a>(
    raw: &'a RawSpec,
    requests: &[ApiRequest],
    filter: &Option<Vec<String>>,
    spec_name: &str,
    per_api_core_cmds: &HashMap<String, HashSet<String>>,
    want_promoted: bool,
    want_predecessors: bool,
) -> Vec<SelectedExt<'a>> {
    let mut api_set: HashSet<&str> = requests.iter().map(|r| r.name.as_str()).collect();
    // The Khronos XML uses "vulkan" in supported= attributes, but our
    // canonical name is "vk".  Insert the XML form so contains() lookups
    // against XML-sourced strings succeed.
    if api_set.contains("vk") {
        api_set.insert("vulkan");
    }
    // WGL mandatory extensions (spec gotcha #9).
    let wgl_mandatory: HashSet<&str> = if spec_name == "wgl" {
        ["WGL_ARB_extensions_string", "WGL_EXT_extensions_string"]
            .iter()
            .copied()
            .collect()
    } else {
        HashSet::new()
    };

    let mut selected: Vec<SelectedExt<'a>> = raw
        .extensions
        .iter()
        .filter_map(|e| {
            let supported = e.supported.iter().any(|s| api_set.contains(s.as_str()));
            if !supported {
                return None;
            }
            if wgl_mandatory.contains(e.name.as_str()) {
                return Some(SelectedExt {
                    raw: e,
                    reason: SelectionReason::Mandatory,
                });
            }
            match filter {
                None => Some(SelectedExt {
                    raw: e,
                    reason: SelectionReason::AllExtensions,
                }),
                Some(list) => {
                    if list.contains(&e.name) {
                        Some(SelectedExt {
                            raw: e,
                            reason: SelectionReason::Explicit,
                        })
                    } else {
                        None
                    }
                }
            }
        })
        .collect();

    // Build an extension name → index lookup for dependency resolution.
    let ext_by_name: HashMap<&str, usize> = raw
        .extensions
        .iter()
        .enumerate()
        .map(|(i, e)| (e.name.as_str(), i))
        .collect();

    // Dependency-following pass: walk the `depends` field of each selected
    // extension and pull in any prerequisite extensions not already selected.
    // Fixed-point loop because dependencies can be transitive — pulling in
    // extension A may require extension B which requires extension C.
    // Runs before --promoted and --predecessors so that dependency-pulled
    // extensions' commands are visible to those passes.
    loop {
        let already: HashSet<&str> = selected.iter().map(|e| e.raw.name.as_str()).collect();
        let prev_len = selected.len();

        // Collect unique dependency names from all currently selected extensions.
        let needed: HashSet<&str> = selected
            .iter()
            .flat_map(|e| e.raw.depends.iter().map(String::as_str))
            .filter(|dep| {
                !already.contains(dep)
                    && ext_by_name.contains_key(dep)
                    // Only pull in extensions that support a requested API.
                    && raw.extensions[ext_by_name[dep]]
                        .supported
                        .iter()
                        .any(|s| api_set.contains(s.as_str()))
            })
            .collect();

        for dep_name in needed {
            if let Some(&idx) = ext_by_name.get(dep_name) {
                selected.push(SelectedExt {
                    raw: &raw.extensions[idx],
                    reason: SelectionReason::Dependency,
                });
            }
        }

        if selected.len() == prev_len {
            break;
        }
    }

    // Build the bidirectional alias maps once — they're used by both the
    // --promoted and --predecessors passes.
    let cmd_to_alias: HashMap<&str, &str> = if want_promoted || want_predecessors {
        let mut m = HashMap::new();
        for (name, cmd) in &raw.commands {
            if let Some(ref alias) = cmd.alias {
                m.insert(name.as_str(), alias.as_str());
                m.insert(alias.as_str(), name.as_str());
            }
        }
        m
    } else {
        HashMap::new()
    };
    let enum_to_alias: HashMap<&str, &str> = if want_predecessors {
        let mut m = HashMap::new();
        for (name, e) in &raw.flat_enums {
            if let Some(ref alias) = e.alias {
                m.insert(name.as_str(), alias.as_str());
                m.insert(alias.as_str(), name.as_str());
            }
        }
        m
    } else {
        HashMap::new()
    };

    if want_promoted {
        // Snapshot names already selected so we don't duplicate them.
        let already: HashSet<&str> = selected.iter().map(|e| e.raw.name.as_str()).collect();

        for ext in &raw.extensions {
            if already.contains(ext.name.as_str()) {
                continue;
            }

            // An extension is considered promoted if, for at least one API A that:
            //   (a) the extension claims to support, and
            //   (b) we are generating,
            // any of the extension's commands for that API appear in A's core
            // command set — either directly (same-name promotion) or via the
            // alias graph (renamed promotion, e.g. glActiveTextureARB → glActiveTexture).
            //
            // Checking per-API (rather than against the unified req_commands) prevents
            // cross-contamination in merged builds: a GLES2-only extension whose
            // commands happen to match GLES2 core will not be auto-included for gl:core.
            let is_promoted = ext
                .supported
                .iter()
                .filter(|s| api_set.contains(s.as_str()))
                .any(|api| {
                    let Some(core_cmds) = per_api_core_cmds.get(canonical_api_name(api.as_str()))
                    else {
                        return false;
                    };
                    ext.requires
                        .iter()
                        // Only consider require blocks that apply to this API.
                        .filter(|req| api_profile_matches(req.api.as_deref(), None, api, None))
                        .any(|req| {
                            req.commands.iter().any(|c| {
                                // Same-name promotion: the command landed in core
                                // with the same name (e.g. ARB_copy_buffer →
                                // glCopyBufferSubData is unchanged).
                                core_cmds.contains(c.as_str())
                                    // Renamed promotion: the command has an alias
                                    // that is in core (e.g. glActiveTextureARB →
                                    // glActiveTexture).
                                    || cmd_to_alias
                                        .get(c.as_str())
                                        .is_some_and(|a| core_cmds.contains(*a))
                            })
                        })
                });

            if is_promoted {
                selected.push(SelectedExt {
                    raw: ext,
                    reason: SelectionReason::Promoted,
                });
            }
        }
    }

    if want_predecessors {
        // Build the set of all commands and enums contributed by the currently
        // selected extensions (after --promoted may have expanded the set).
        // An unselected extension is a "predecessor" of the selected set if any
        // of its commands or enums are aliases of items in this set — i.e. the
        // extension was superseded by one already selected.
        //
        // We iterate to a fixed point because adding a predecessor may itself
        // have predecessors not yet in the set.  The sets are maintained
        // incrementally: each iteration only adds items from newly-selected
        // extensions rather than re-scanning the entire selected list.
        let mut selected_ext_cmds: HashSet<&str> = selected
            .iter()
            .flat_map(|e| {
                e.raw
                    .requires
                    .iter()
                    .flat_map(|req| req.commands.iter().map(String::as_str))
            })
            .collect();

        let mut selected_ext_enums: HashSet<&str> = selected
            .iter()
            .flat_map(|e| {
                e.raw
                    .requires
                    .iter()
                    .flat_map(|req| req.enums.iter().map(String::as_str))
            })
            .collect();

        loop {
            let already: HashSet<&str> = selected.iter().map(|e| e.raw.name.as_str()).collect();
            let prev_len = selected.len();

            for ext in &raw.extensions {
                if already.contains(ext.name.as_str()) {
                    continue;
                }
                let supported = ext.supported.iter().any(|s| api_set.contains(s.as_str()));
                if !supported {
                    continue;
                }
                let is_predecessor = ext.requires.iter().any(|req| {
                    req.commands.iter().any(|c| {
                        selected_ext_cmds.contains(c.as_str())
                            || cmd_to_alias
                                .get(c.as_str())
                                .is_some_and(|a| selected_ext_cmds.contains(*a))
                    }) || req.enums.iter().any(|e| {
                        selected_ext_enums.contains(e.as_str())
                            || enum_to_alias
                                .get(e.as_str())
                                .is_some_and(|a| selected_ext_enums.contains(*a))
                    })
                });
                if is_predecessor {
                    selected.push(SelectedExt {
                        raw: ext,
                        reason: SelectionReason::Predecessor,
                    });
                }
            }

            if selected.len() == prev_len {
                break;
            }

            // Incrementally add commands/enums from only the newly selected extensions.
            for ext in &selected[prev_len..] {
                for req in &ext.raw.requires {
                    selected_ext_cmds.extend(req.commands.iter().map(String::as_str));
                    selected_ext_enums.extend(req.enums.iter().map(String::as_str));
                }
            }
        }
    }

    selected
}

// ---------------------------------------------------------------------------
// Building the Command entry
// ---------------------------------------------------------------------------

fn build_command(
    index: u16,
    raw: &RawCommand,
    scope: &str,
    protect: Option<String>,
    pfn_prefix: &str,
    name_prefix: &str,
) -> Command {
    let short_name = raw
        .name
        .strip_prefix(name_prefix)
        .unwrap_or(&raw.name)
        .to_string();

    let pfn_type = if pfn_prefix == "PFN_" {
        // Vulkan: PFN_vkFoo
        format!("PFN_{}", raw.name)
    } else {
        // GL family: PFNGLFOOPROC — strip the lowercase api prefix (e.g. "gl")
        // before uppercasing so we don't get PFNGLGLFOOPROC.
        let stem = raw.name.strip_prefix(name_prefix).unwrap_or(&raw.name);
        format!("{}{}PROC", pfn_prefix, stem.to_uppercase())
    };

    let params: Vec<Param> = raw
        .params
        .iter()
        .map(|p| Param {
            type_raw: p.type_raw.clone(),
            name: p.name.clone(),
        })
        .collect();

    let params_str = if params.is_empty() {
        "void".to_string()
    } else {
        params
            .iter()
            .map(|p| {
                if p.name.is_empty() {
                    p.type_raw.clone()
                } else if p.type_raw.trim_end().ends_with(']') {
                    // Array param: type_raw already contains the name and
                    // array suffix, e.g. "float blendConstants[4]".
                    // Emit verbatim — don't append the name again.
                    p.type_raw.trim().to_string()
                } else {
                    format!("{} {}", p.type_raw, p.name)
                }
            })
            .collect::<Vec<_>>()
            .join(", ")
    };

    Command {
        index,
        name: raw.name.clone(),
        short_name,
        pfn_type,
        return_type: raw.return_type.clone(),
        params_str,
        params,
        scope: scope.to_string(),
        protect,
    }
}

// ---------------------------------------------------------------------------
// Building the Extension entry
// ---------------------------------------------------------------------------

fn build_extension(index: u16, e: &SelectedExt<'_>) -> Extension {
    use xxhash_rust::xxh3::xxh3_64;

    let hash_val = xxh3_64(e.raw.name.as_bytes());
    // No ULL suffix — templates append it at the point of use.
    let hash = format!("0x{:016x}", hash_val);
    let short = ext_short_name(&e.raw.name);

    Extension {
        index,
        name: e.raw.name.clone(),
        short_name: short,
        hash,
        protect: e.raw.protect.clone(),
        reason: e.reason,
    }
}

// ---------------------------------------------------------------------------
// Context struct name
// ---------------------------------------------------------------------------

fn build_context_name(spec: &str) -> String {
    let display = match spec {
        "gl" => "GL",
        "egl" => "EGL",
        "glx" => "GLX",
        "wgl" => "WGL",
        "vk" => "Vulkan",
        other => other,
    };
    format!("Gloam{}Context", display)
}

/// Strip API prefix from extension name: "GL_ARB_sync" → "ARB_sync".
fn ext_short_name(name: &str) -> String {
    for prefix in &["GL_", "EGL_", "GLX_", "WGL_", "VK_"] {
        if let Some(s) = name.strip_prefix(prefix) {
            return s.to_string();
        }
    }
    name.to_string()
}

/// Strip version prefix: "GL_VERSION_3_3" → "VERSION_3_3".
fn version_short_name(name: &str, api: &str) -> String {
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

// ---------------------------------------------------------------------------
// PFN range table construction
// ---------------------------------------------------------------------------

fn build_feature_pfn_ranges(
    features: &[SelectedFeature<'_>],
    feat_entries: &[Feature],
    commands: &[Command],
) -> Vec<PfnRange> {
    debug_assert_eq!(
        features.len(),
        feat_entries.len(),
        "SelectedFeature and Feature slices must be built in the same order"
    );

    // Build a map: command name → pfnArray index.
    let cmd_index: HashMap<&str, u16> = commands
        .iter()
        .map(|c| (c.name.as_str(), c.index))
        .collect();

    let mut ranges: Vec<PfnRange> = Vec::new();

    // features[] and feat_entries[] are built from the same source in the same
    // order, so we zip rather than doing O(n) string searches per feature.
    for (sf, feat) in features.iter().zip(feat_entries.iter()) {
        debug_assert_eq!(sf.raw.name, feat.full_name);

        let mut cmd_indices: Vec<u16> = Vec::new();
        for require in &sf.raw.requires {
            for cmd_name in &require.commands {
                if let Some(&idx) = cmd_index.get(cmd_name.as_str()) {
                    cmd_indices.push(idx);
                }
            }
        }
        cmd_indices.sort_unstable();
        cmd_indices.dedup();

        ranges.extend(indices_to_ranges(feat.index, &cmd_indices));
    }

    ranges
}

fn build_ext_pfn_ranges(
    api: &str,
    exts: &[SelectedExt<'_>],
    _ext_commands: &IndexMap<String, usize>, // cmd -> ext index in `exts` (reserved)
    ext_index_map: &HashMap<&str, u16>,      // ext name -> sorted ext index
    commands: &[Command],
) -> (Vec<PfnRange>, Vec<u16>) {
    let cmd_index: HashMap<&str, u16> = commands
        .iter()
        .map(|c| (c.name.as_str(), c.index))
        .collect();

    let mut ranges: Vec<PfnRange> = Vec::new();
    let mut subset_indices: Vec<u16> = Vec::new();

    // Collect extensions relevant to this API.
    let relevant_exts: Vec<(usize, &SelectedExt)> = exts
        .iter()
        .enumerate()
        .filter(|(_, e)| {
            e.raw
                .supported
                .iter()
                .any(|s| canonical_api_name(s) == canonical_api_name(api))
        })
        .collect();

    for (_orig_idx, ext) in &relevant_exts {
        let sorted_ext_idx = match ext_index_map.get(ext.raw.name.as_str()) {
            Some(&i) => i,
            None => continue,
        };

        subset_indices.push(sorted_ext_idx);

        // Commands belonging to this extension for this API.
        let mut cmd_indices: Vec<u16> = Vec::new();
        for require in &ext.raw.requires {
            if !api_profile_matches(require.api.as_deref(), None, api, None) {
                continue;
            }
            for cmd_name in &require.commands {
                if let Some(&pfn_idx) = cmd_index.get(cmd_name.as_str()) {
                    cmd_indices.push(pfn_idx);
                }
            }
        }
        cmd_indices.sort_unstable();
        cmd_indices.dedup();

        ranges.extend(indices_to_ranges(sorted_ext_idx, &cmd_indices));
    }

    subset_indices.sort_unstable();
    (ranges, subset_indices)
}

/// Convert a sorted list of pfnArray indices belonging to the same feature/ext
/// into one or more PfnRange entries (one per contiguous run).
fn indices_to_ranges(ext_idx: u16, sorted: &[u16]) -> Vec<PfnRange> {
    if sorted.is_empty() {
        return Vec::new();
    }
    let mut ranges = Vec::new();
    let mut start = sorted[0];
    let mut count = 1u16;

    for &idx in &sorted[1..] {
        if idx == start + count {
            count += 1;
        } else {
            ranges.push(PfnRange {
                extension: ext_idx,
                start,
                count,
            });
            start = idx;
            count = 1;
        }
    }
    ranges.push(PfnRange {
        extension: ext_idx,
        start,
        count,
    });
    ranges
}

// ---------------------------------------------------------------------------
// Types list
// ---------------------------------------------------------------------------

fn build_type_list(
    raw: &RawSpec,
    req_types: &HashSet<String>,
    spec_name: &str,
    is_vulkan: bool,
    selected_exts: &[SelectedExt<'_>],
) -> Vec<TypeDef> {
    // Always infer include protections — Vulkan needs it for WSI headers,
    // and GL needs it to correctly guard khrplatform and eglplatform includes.
    // Scoped to selected extensions so that includes are only emitted when
    // an extension actually in the feature set depends on them.
    let include_protections = infer_include_protections(raw, selected_exts);
    let ext_type_protect = build_ext_type_protections(raw);

    let type_list: Vec<TypeDef> = raw
        .types
        .iter()
        .filter(|t| {
            // Empty raw_c → nothing to emit.
            if t.raw_c.is_empty() {
                return false;
            }
            // Enum-category types: plain enums have no direct C emission
            // (their values are emitted via enum groups).  Alias-only enum
            // types (e.g. VkComponentTypeNV = VkComponentTypeKHR) DO need a
            // typedef emission and must pass through the filter.
            if t.category == "enum" && t.raw_c.is_empty() {
                return false;
            }
            // Include-category types: emit only for system/WSI headers where
            // infer_include_protections decided they're needed.  Bundled headers
            // (vk_platform, vk_video/*, etc.) are already emitted by the
            // required_headers template loop and must not appear here too.
            if t.category == "include" {
                if is_bundled_include_type(&t.name) {
                    return false;
                }
                return include_protections.contains_key(&t.name);
            }
            // `define` and `basetype` types (VK_DEFINE_HANDLE, VkFlags,
            // VkBool32, etc.) are not listed in any <require> block but must
            // always be emitted for the matching API, like GL auto-includes.
            // `bitmask` typedefs are also thin wrappers never explicitly
            // required.  Struct/union/handle/funcpointer/enum are only emitted
            // when a feature or extension explicitly requires them.
            if is_vulkan {
                let auto = matches!(
                    t.category.as_str(),
                    "define" | "basetype" | "bitmask" | "funcpointer" | "enum" | "handle"
                );
                if auto {
                    return t
                        .api
                        .as_deref()
                        .is_none_or(|a| a.split(',').any(|s| s.trim() == "vulkan"));
                }
                return req_types.contains(&t.name);
            }
            // GL family: auto-include all API-compatible types except the
            // excluded ones (spec gotcha #5 exclusions).
            if is_gl_auto_excluded(&t.name) {
                return false;
            }
            req_types.contains(&t.name) || t.api.as_deref().is_none_or(|a| a == spec_name)
        })
        .map(|t| {
            // For include-category types, use the inferred protection list.
            // For all others, use the type's own protect attribute.
            let protect = if t.category == "include" {
                include_protections
                    .get(&t.name)
                    .cloned()
                    .unwrap_or_default()
            } else {
                // Prefer extension-derived protection over the type's own
                // protect= attribute.  Many Vulkan structs are protected only
                // via the extension that introduces them (e.g. platform="win32"
                // on the extension) and have no protect= on the <type> element.
                if let Some(p) = ext_type_protect.get(t.name.as_str()) {
                    p.clone()
                } else {
                    t.protect.iter().cloned().collect()
                }
            };
            TypeDef {
                name: t.name.clone(),
                raw_c: normalize_raw_c(&t.raw_c),
                category: t.category.clone(),
                protect,
            }
        })
        .collect::<Vec<TypeDef>>();
    topo_sort_typedefs(type_list)
}

/// Topological sort on a `Vec<TypeDef>`.
///
/// A type A depends on B if B's name appears as a word in A's raw_c.
/// We only create dep edges when scanning struct/union/funcpointer raw_c —
/// other categories (define, basetype, etc.) don't have bodies that reference
/// other types in ordering-relevant ways, and scanning them can create false
/// cycle edges.
///
/// Cycle fallback: stranded types are sorted among themselves (types used by
/// others in the same cycle come first) before being appended.
fn topo_sort_typedefs(types: Vec<TypeDef>) -> Vec<TypeDef> {
    let n = types.len();
    if n < 2 {
        return types;
    }

    // name → index map using TypeDef.name (reliable, no raw_c parsing needed).
    let mut name_to_idx: HashMap<&str, usize> = HashMap::new();
    for (i, t) in types.iter().enumerate() {
        name_to_idx.insert(t.name.as_str(), i);
    }

    // Only scan struct/union/funcpointer bodies for deps.  Other categories
    // either have no meaningful body (define is a macro, basetype/bitmask are
    // simple typedefs) or would introduce false cycles.
    let scan_cats: &[&str] = &["struct", "union", "funcpointer"];

    let deps: Vec<Vec<usize>> = types
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let mut d: Vec<usize> = Vec::new();
            if scan_cats.contains(&t.category.as_str()) {
                for word in crate::parse::types::ident_words(&t.raw_c) {
                    if word == t.name.as_str() {
                        continue;
                    }
                    if let Some(&dep_idx) = name_to_idx.get(word)
                        && dep_idx != i
                    {
                        d.push(dep_idx);
                    }
                }
                d.sort_unstable();
                d.dedup();
            }
            d
        })
        .collect();

    let mut in_degree: Vec<usize> = deps.iter().map(|d| d.len()).collect();
    let mut rev: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (i, dep_list) in deps.iter().enumerate() {
        for &dep in dep_list {
            rev[dep].push(i);
        }
    }

    let mut queue: VecDeque<usize> = (0..n).filter(|&i| in_degree[i] == 0).collect();
    let mut order: Vec<usize> = Vec::with_capacity(n);
    while let Some(node) = queue.pop_front() {
        order.push(node);
        for &dependent in &rev[node] {
            in_degree[dependent] -= 1;
            if in_degree[dependent] == 0 {
                queue.push_back(dependent);
            }
        }
    }

    // Cycle fallback: sort stranded nodes so that if A's raw_c references
    // B's name, B comes before A.  Uses a second topo sort scoped to just
    // the stranded subset.
    if order.len() < n {
        let stranded: Vec<usize> = (0..n).filter(|&i| in_degree[i] != 0).collect();
        let stranded_set: HashSet<usize> = stranded.iter().copied().collect();

        // Build per-node dependency sets (deduplicated — ident_words may
        // yield the same word more than once for a given type body).
        let s_deps: HashMap<usize, HashSet<usize>> = stranded
            .iter()
            .map(|&i| {
                let deps_i: HashSet<usize> = crate::parse::types::ident_words(&types[i].raw_c)
                    .filter_map(|word| {
                        name_to_idx
                            .get(word)
                            .copied()
                            .filter(|&j| j != i && stranded_set.contains(&j))
                    })
                    .collect();
                (i, deps_i)
            })
            .collect();

        let mut s_in: HashMap<usize, usize> =
            stranded.iter().map(|&i| (i, s_deps[&i].len())).collect();
        let mut s_rev: HashMap<usize, Vec<usize>> =
            stranded.iter().map(|&i| (i, Vec::new())).collect();
        for &i in &stranded {
            for &j in &s_deps[&i] {
                s_rev.get_mut(&j).unwrap().push(i);
            }
        }

        let mut s_queue: VecDeque<usize> = stranded
            .iter()
            .filter(|&&i| s_in[&i] == 0)
            .copied()
            .collect();
        let mut s_order: Vec<usize> = Vec::new();
        while let Some(node) = s_queue.pop_front() {
            s_order.push(node);
            for &dep in &s_rev[&node] {
                let e = s_in.get_mut(&dep).unwrap();
                *e -= 1;
                if *e == 0 {
                    s_queue.push_back(dep);
                }
            }
        }
        // Any still-stranded types (true cycles) append in original index order.
        let processed: HashSet<usize> = s_order.iter().copied().collect();
        for &i in &stranded {
            if !processed.contains(&i) {
                s_order.push(i);
            }
        }
        order.extend(s_order);
    }

    let mut out: Vec<Option<TypeDef>> = types.into_iter().map(Some).collect();
    order.into_iter().map(|i| out[i].take().unwrap()).collect()
}

// ---------------------------------------------------------------------------
// Include protection inference
// ---------------------------------------------------------------------------

/// Build a map from type name → protection macros derived purely from the
/// extensions that require that type.
///
/// This covers the common Vulkan pattern where a struct has no `protect=`
/// attribute on its `<type>` element but is required only inside an extension
/// with `platform="win32"` (or similar), making its protection implicit.
///
/// Missing from the map = not required by any extension (use the type's
/// own `protect=` attribute instead, or no guard).
fn build_ext_type_protections(raw: &RawSpec) -> HashMap<String, Vec<String>> {
    let mut tmp: HashMap<&str, Protection> = HashMap::new();

    for ext in &raw.extensions {
        for require in &ext.requires {
            for type_name in &require.types {
                tmp.entry(type_name.as_str())
                    .or_insert_with(Protection::new_guarded)
                    .add_extension(&ext.protect);
            }
        }
    }

    tmp.into_iter()
        .map(|(name, prot)| (name.to_string(), prot.into_vec()))
        .collect()
}

/// Record protection guards for a single type name if it's an include
/// dependency.  Uses the `Protection` lattice for clean state merging.
fn record_protect<'a>(
    name: &'a str,
    ext_protect: &[String],
    all_dep_names: &HashSet<&str>,
    map: &mut HashMap<&'a str, Protection>,
) {
    if !all_dep_names.contains(name) {
        return;
    }
    map.entry(name)
        .or_insert_with(Protection::new_guarded)
        .add_extension(ext_protect);
}

/// For each `category="include"` type in the spec, determine what `#if`
/// protection it needs based on which extensions require types that depend
/// on that include file.
///
/// Algorithm (mirrors GLAD's `protections()` method):
///
///   - Collect all types that have `requires=<include_name>`.
///   - For each such type, find every selected extension that requires it.
///   - The include's protection = union of those extensions' protections.
///   - If any requiring extension is unprotected, the include is unconditional
///     (empty protection list).
///   - If no extension requires the type at all, the include is omitted.
fn infer_include_protections(
    raw: &RawSpec,
    selected_exts: &[SelectedExt<'_>],
) -> HashMap<String, Vec<String>> {
    // Step 1: include_name → set of type names that `requires=` it.
    // e.g. "X11/Xlib.h" → {"Display", "VisualID", "Window"}
    let include_names: HashSet<&str> = raw
        .types
        .iter()
        .filter(|t| t.category == "include")
        .map(|t| t.name.as_str())
        .collect();

    let mut include_to_deps: HashMap<&str, HashSet<&str>> = HashMap::new();
    for t in &raw.types {
        if t.category == "include" {
            continue;
        }
        if let Some(ref req) = t.requires
            && include_names.contains(req.as_str())
        {
            include_to_deps
                .entry(req.as_str())
                .or_default()
                .insert(t.name.as_str());
        }
    }

    // Step 2: dep_type_name → protection.
    //
    // Two sources:
    //   (a) Extensions that directly list the dep type in their <require> block
    //       (e.g. VK_KHR_xlib_surface requires "Display" explicitly).
    //   (b) Protected struct/union types whose raw_c TEXT contains the dep type
    //       name (e.g. VkXlibSurfaceCreateInfoKHR has a Display* member and
    //       lives under VK_USE_PLATFORM_XLIB_KHR).  This catches the case
    //       where the dep type is never listed in any <require> block.
    let mut type_protect: HashMap<&str, Protection> = HashMap::new();

    // Collect all dep names for fast membership tests in (b).
    let all_dep_names: HashSet<&str> = include_to_deps
        .values()
        .flat_map(|s| s.iter().copied())
        .collect();

    // Source (a): extension require blocks — type names and command parameter
    // types.  Platform types like `Display` and `RROutput` often only appear
    // as command parameters, never as explicit <type> entries in <require>.
    // Scoped to selected extensions so includes not needed by the feature set
    // are not emitted.
    for ext in selected_exts {
        for require in &ext.raw.requires {
            // (a1) Explicitly listed type names.
            for type_name in &require.types {
                record_protect(
                    type_name.as_str(),
                    &ext.raw.protect,
                    &all_dep_names,
                    &mut type_protect,
                );
            }
            // (a2) Parameter types of required commands.
            for cmd_name in &require.commands {
                if let Some(cmd) = raw.commands.get(cmd_name.as_str()) {
                    for param in &cmd.params {
                        record_protect(
                            param.type_name.as_str(),
                            &ext.raw.protect,
                            &all_dep_names,
                            &mut type_protect,
                        );
                    }
                }
            }
        }
    }

    // Source (b): scan raw_c of every type that has a known protection.
    // Protection comes from the type's own protect= attr, OR from extensions
    // that require that type.
    let mut type_own_protect: HashMap<&str, Protection> = HashMap::new();
    for t in &raw.types {
        if t.category == "include" || t.raw_c.is_empty() {
            continue;
        }
        if let Some(ref p) = t.protect {
            type_own_protect
                .entry(t.name.as_str())
                .or_insert_with(|| Protection::Guarded(vec![p.clone()]));
        }
    }
    // Also derive struct protection from selected extension context.
    for ext in selected_exts {
        for require in &ext.raw.requires {
            for type_name in &require.types {
                type_own_protect
                    .entry(type_name.as_str())
                    .or_insert_with(|| {
                        if ext.raw.protect.is_empty() {
                            Protection::Unconditional
                        } else {
                            Protection::Guarded(ext.raw.protect.clone())
                        }
                    });
            }
        }
    }

    for t in &raw.types {
        if t.raw_c.is_empty() || t.category == "include" {
            continue;
        }
        let struct_protect = match type_own_protect.get(t.name.as_str()) {
            None => continue, // unprotected type, skip
            Some(prot) => prot,
        };
        for word in ident_words(&t.raw_c) {
            if !all_dep_names.contains(word) {
                continue;
            }
            let entry = type_protect
                .entry(word)
                .or_insert_with(Protection::new_guarded);
            match struct_protect {
                Protection::Unconditional => *entry = Protection::Unconditional,
                Protection::Guarded(ps) => entry.add_extension(ps),
            }
        }
    }

    // Step 3: for each include, union its dep types' protections.
    let mut result: HashMap<String, Vec<String>> = HashMap::new();

    for (include_name, dep_names) in &include_to_deps {
        let mut merged = Protection::new_guarded();
        let mut any_found = false;

        for &dep_name in dep_names {
            if let Some(prot) = type_protect.get(dep_name) {
                any_found = true;
                match prot {
                    Protection::Unconditional => {
                        merged = Protection::Unconditional;
                        break;
                    }
                    Protection::Guarded(guards) => merged.add_extension(guards),
                }
            }
        }

        if any_found {
            result.insert(include_name.to_string(), merged.into_vec());
        }
    }

    result
}

// ---------------------------------------------------------------------------
// Required auxiliary headers
// ---------------------------------------------------------------------------

/// Scan the selected types for `requires=` attributes that map to auxiliary
/// header files that must be copied to the output include tree.
///
/// Returns paths relative to the include root, e.g. `"KHR/khrplatform.h"`.
/// Deduplication and insertion order are preserved via IndexMap.
fn collect_required_headers(
    raw: &RawSpec,
    req_types: &HashSet<String>,
    spec_name: &str,
) -> Vec<String> {
    let mut headers: IndexMap<String, ()> = IndexMap::new();

    for t in &raw.types {
        // Only consider types that were selected (same logic as build_type_list).
        let selected = if spec_name == "vk" {
            req_types.contains(&t.name)
        } else {
            !is_gl_auto_excluded(&t.name)
                && (req_types.contains(&t.name) || t.api.as_deref().is_none_or(|a| a == spec_name))
        };
        if !selected {
            continue;
        }

        if let Some(ref req) = t.requires
            && let Some(hdr) = requires_to_bundled_header(req)
        {
            headers.insert(hdr.to_string(), ());
        }
    }

    // Vulkan: vk_platform.h is always needed; vk_video headers are bundled
    // and must be copied when any *selected* type requires them.
    // Include-category types are never in req_types directly — check whether
    // any selected non-include type has `requires=` pointing to the header.
    if spec_name == "vk" {
        headers.insert("vk_platform.h".to_string(), ());

        // Build set of vk_video include names for fast lookup.
        let vk_video_includes: HashSet<&str> = raw
            .types
            .iter()
            .filter(|t| t.category == "include" && t.name.starts_with("vk_video/"))
            .map(|t| t.name.as_str())
            .collect();

        for t in &raw.types {
            if t.category == "include" {
                continue;
            }
            if let Some(ref req) = t.requires
                && vk_video_includes.contains(req.as_str())
            {
                // This type is selected if it's in req_types OR is auto-included.
                let auto = matches!(
                    t.category.as_str(),
                    "define" | "basetype" | "bitmask" | "funcpointer" | "enum" | "handle"
                );
                if auto || req_types.contains(&t.name) {
                    headers.insert(req.clone(), ());
                }
            }
        }
    }

    headers.into_keys().collect()
}

/// Map a `requires=` value to a *bundled* header path we own and copy to the
/// output tree.  Returns None for system/WSI headers (X11/Xlib.h, windows.h,
/// etc.) which the user must provide themselves.
fn requires_to_bundled_header(requires: &str) -> Option<&'static str> {
    match requires {
        "khrplatform" => Some("KHR/khrplatform.h"),
        "eglplatform" => Some("EGL/eglplatform.h"),
        "vk_platform" => Some("vk_platform.h"),
        _ => None,
    }
}

/// True if an include-category type name refers to a header that we bundle
/// and copy to the output tree.  These are emitted via the `required_headers`
/// template loop and must NOT also appear as include-category types, or the
/// generated header `#include`s them twice.
fn is_bundled_include_type(name: &str) -> bool {
    matches!(name, "vk_platform" | "khrplatform" | "eglplatform") || name.starts_with("vk_video/")
}

fn normalize_raw_c(raw: &str) -> String {
    // Semicolons are added at construction time in types.rs; XML-sourced
    // content already includes them where required.  We only trim whitespace.
    raw.trim().to_string()
}

fn build_flat_enums(raw: &RawSpec, req_enums: &HashSet<String>, is_vulkan: bool) -> Vec<FlatEnum> {
    raw.flat_enums
        .iter()
        // For Vulkan, all flat enums are API constants (VK_MAX_DESCRIPTION_SIZE
        // etc.) that are never explicitly listed in <require> blocks but are
        // always needed.  For GL, only emit constants selected by the feature set.
        .filter(|(name, _)| is_vulkan || req_enums.contains(*name))
        .filter_map(|(_, e)| {
            let value = e.value.as_deref().or(e.alias.as_deref())?;
            Some(FlatEnum {
                name: e.name.clone(),
                value: value.to_string(),
                comment: e.comment.clone(),
            })
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Enum groups (Vulkan)
// ---------------------------------------------------------------------------

fn build_enum_groups(raw: &RawSpec) -> Vec<EnumGroup> {
    raw.enum_groups
        .iter()
        .map(|g| {
            let raw_values: Vec<FlatEnum> = g
                .values
                .iter()
                .filter_map(|v| {
                    let val = v.value.as_deref().or(v.alias.as_deref())?;
                    Some(FlatEnum {
                        name: v.name.clone(),
                        value: val.to_string(),
                        comment: v.comment.clone(),
                    })
                })
                .collect();

            EnumGroup {
                name: g.name.clone(),
                is_bitmask: false,
                bitwidth: g.bitwidth.unwrap_or(32),
                values: sort_enum_values(raw_values),
            }
        })
        .collect()
}

/// Sort enum values so that any member whose value is a reference to another
/// member name is emitted after the member it references.
///
/// A value is a "reference" if it is not a numeric literal (decimal, hex,
/// or negative).  We do a single-pass Kahn topological sort; the input order
/// is preserved for values with no inter-dependencies.
fn sort_enum_values(values: Vec<FlatEnum>) -> Vec<FlatEnum> {
    let n = values.len();
    if n == 0 {
        return values;
    }

    // Build a name → index map.
    let name_to_idx: HashMap<&str, usize> = values
        .iter()
        .enumerate()
        .map(|(i, v)| (v.name.as_str(), i))
        .collect();

    // For each value, find the index of the value it depends on (if any).
    // A dependency exists when `value` is itself a member name — i.e. it
    // starts with VK_ (or any non-numeric, non-minus character) and is
    // present in name_to_idx.
    let deps: Vec<Option<usize>> = values
        .iter()
        .map(|v| {
            let val = v.value.trim();
            // Numeric literals: decimal, negative decimal, hex.
            let is_numeric = val.starts_with(|c: char| c.is_ascii_digit())
                || val.starts_with("0x")
                || val.starts_with("0X")
                || (val.starts_with('-') && val.len() > 1);
            if is_numeric {
                return None;
            }
            name_to_idx.get(val).copied()
        })
        .collect();

    // Kahn's algorithm: in-degree = 1 if the value has a dep, else 0.
    let mut in_degree: Vec<usize> = deps.iter().map(|d| d.is_some() as usize).collect();

    // rev[i] = list of values that depend on i.
    let mut rev: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (i, dep) in deps.iter().enumerate() {
        if let Some(d) = dep {
            rev[*d].push(i);
        }
    }

    let mut queue: VecDeque<usize> = (0..n).filter(|&i| in_degree[i] == 0).collect();
    let mut order: Vec<usize> = Vec::with_capacity(n);

    while let Some(node) = queue.pop_front() {
        order.push(node);
        for &dep in &rev[node] {
            in_degree[dep] -= 1;
            if in_degree[dep] == 0 {
                queue.push_back(dep);
            }
        }
    }

    // Append any remaining nodes (cycles — shouldn't happen in practice).
    for (i, item) in in_degree.iter().enumerate().take(n) {
        if *item != 0 {
            order.push(i);
        }
    }

    let mut out: Vec<Option<FlatEnum>> = values.into_iter().map(Some).collect();
    order.into_iter().map(|i| out[i].take().unwrap()).collect()
}

// ---------------------------------------------------------------------------
// Alias pairs
// ---------------------------------------------------------------------------

fn build_alias_pairs(raw: &RawSpec, commands: &[Command]) -> Vec<AliasPair> {
    // Build name -> index map for quick lookup.
    let idx: HashMap<&str, u16> = commands
        .iter()
        .map(|c| (c.name.as_str(), c.index))
        .collect();

    // Group by canonical (shortest name).
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();
    for (name, cmd) in &raw.commands {
        if let Some(ref alias) = cmd.alias {
            // Both must be in the selected command set.
            if !idx.contains_key(name.as_str()) || !idx.contains_key(alias.as_str()) {
                continue;
            }
            // Canonical = shortest name; if equal, alphabetical.
            let (canonical, secondary) = if alias.len() < name.len()
                || (alias.len() == name.len() && alias.as_str() < name.as_str())
            {
                (alias.clone(), name.clone())
            } else {
                (name.clone(), alias.clone())
            };
            groups.entry(canonical).or_default().push(secondary);
        }
    }

    let mut pairs: Vec<AliasPair> = Vec::new();
    for (canonical, secondaries) in groups {
        let Some(&ci) = idx.get(canonical.as_str()) else {
            continue;
        };
        for secondary in secondaries {
            let Some(&si) = idx.get(secondary.as_str()) else {
                continue;
            };
            pairs.push(AliasPair {
                canonical: ci,
                secondary: si,
            });
        }
    }

    // Sort by canonical index (the load loop depends on consecutive ordering).
    pairs.sort_by_key(|p| (p.canonical, p.secondary));
    pairs
}

// ---------------------------------------------------------------------------
// Utility: prebake command → platform protect mapping.
// ---------------------------------------------------------------------------

/// Build a map from command name → platform protection macro, derived from
/// extensions.  A single pass over all extensions replaces the previous
/// per-command linear scan (O(cmds × exts × requires) → O(exts × requires)).
fn build_command_protect_map<'a>(exts: &[SelectedExt<'a>]) -> HashMap<&'a str, String> {
    let mut map = HashMap::new();
    for ext in exts {
        if let Some(protect) = ext.raw.protect.first() {
            for require in &ext.raw.requires {
                for cmd in &require.commands {
                    map.entry(cmd.as_str()).or_insert_with(|| protect.clone());
                }
            }
        }
    }
    map
}

// ---------------------------------------------------------------------------
// API naming helpers
// ---------------------------------------------------------------------------

/// Prefix for PFN type names.
fn api_pfn_prefix(spec: &str) -> &'static str {
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
fn api_name_prefix(spec: &str) -> &'static str {
    match spec {
        "gl" | "gles1" | "gles2" | "glcore" => "gl",
        "egl" => "egl",
        "glx" => "glX",
        "wgl" => "wgl",
        "vk" | "vulkan" => "vk",
        _ => "",
    }
}

// ---------------------------------------------------------------------------
// Profile / API matching helpers
// ---------------------------------------------------------------------------

fn api_profile_matches(
    elem_api: Option<&str>,
    elem_profile: Option<&str>,
    target_api: &str,
    target_prof: Option<&str>,
) -> bool {
    if let Some(a) = elem_api
        && !a
            .split(',')
            .any(|x| canonical_api_name(x.trim()) == canonical_api_name(target_api))
    {
        return false;
    }
    profile_matches(elem_profile, target_prof)
}

fn profile_matches(elem_profile: Option<&str>, target_profile: Option<&str>) -> bool {
    match (elem_profile, target_profile) {
        (None, _) => true,
        (Some(_), None) => true,
        (Some(ep), Some(tp)) => ep == tp,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- sort_enum_values ----

    fn make_enum(name: &str, value: &str) -> FlatEnum {
        FlatEnum {
            name: name.to_string(),
            value: value.to_string(),
            comment: String::new(),
        }
    }

    #[test]
    fn sort_enum_values_numeric_only_preserves_order() {
        let input = vec![
            make_enum("VK_FOO", "0"),
            make_enum("VK_BAR", "1"),
            make_enum("VK_BAZ", "2"),
        ];
        let out = sort_enum_values(input);
        assert_eq!(out[0].name, "VK_FOO");
        assert_eq!(out[1].name, "VK_BAR");
        assert_eq!(out[2].name, "VK_BAZ");
    }

    #[test]
    fn sort_enum_values_alias_placed_after_target() {
        // VK_ALIAS references VK_ORIGINAL; ALIAS must appear after ORIGINAL.
        let input = vec![
            make_enum("VK_ALIAS", "VK_ORIGINAL"),
            make_enum("VK_ORIGINAL", "42"),
        ];
        let out = sort_enum_values(input);
        let original_pos = out.iter().position(|e| e.name == "VK_ORIGINAL").unwrap();
        let alias_pos = out.iter().position(|e| e.name == "VK_ALIAS").unwrap();
        assert!(original_pos < alias_pos, "alias must come after its target");
    }

    #[test]
    fn sort_enum_values_empty_input() {
        assert!(sort_enum_values(vec![]).is_empty());
    }

    #[test]
    fn sort_enum_values_negative_numeric_not_treated_as_alias() {
        // A negative literal like "-1" must not be treated as a name reference.
        let input = vec![make_enum("VK_MAX", "-1"), make_enum("VK_ZERO", "0")];
        let out = sort_enum_values(input);
        // Both are numeric; original order preserved.
        assert_eq!(out[0].name, "VK_MAX");
        assert_eq!(out[1].name, "VK_ZERO");
    }

    #[test]
    fn sort_enum_values_hex_literal_not_treated_as_alias() {
        let input = vec![make_enum("VK_HEX", "0xFF"), make_enum("VK_OTHER", "0x00")];
        let out = sort_enum_values(input);
        assert_eq!(out[0].name, "VK_HEX");
        assert_eq!(out[1].name, "VK_OTHER");
    }

    // ---- profile_matches ----

    #[test]
    fn profile_matches_both_none() {
        assert!(profile_matches(None, None));
    }

    #[test]
    fn profile_matches_element_none_always_matches() {
        assert!(profile_matches(None, Some("core")));
        assert!(profile_matches(None, Some("compat")));
    }

    #[test]
    fn profile_matches_target_none_always_matches() {
        assert!(profile_matches(Some("core"), None));
    }

    #[test]
    fn profile_matches_same() {
        assert!(profile_matches(Some("core"), Some("core")));
    }

    #[test]
    fn profile_matches_different() {
        assert!(!profile_matches(Some("core"), Some("compat")));
    }

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

    // ---- Protection lattice ----

    #[test]
    fn protection_guarded_absorbs_unconditional() {
        // A guarded state that encounters an unprotected extension becomes
        // unconditional (absorbing element of the lattice).
        let mut p = Protection::Guarded(vec!["VK_USE_PLATFORM_WIN32_KHR".to_string()]);
        p.add_extension(&[]); // unprotected extension
        assert!(p.is_unconditional());
    }

    #[test]
    fn protection_unconditional_stays_unconditional() {
        // Once unconditional, adding a guarded extension can't re-guard it.
        let mut p = Protection::Unconditional;
        p.add_extension(&["VK_USE_PLATFORM_XLIB_KHR".to_string()]);
        assert!(p.is_unconditional());
    }

    #[test]
    fn protection_guarded_unions_guards() {
        let mut p = Protection::new_guarded();
        p.add_extension(&["VK_USE_PLATFORM_WIN32_KHR".to_string()]);
        p.add_extension(&["VK_USE_PLATFORM_XLIB_KHR".to_string()]);
        let v = p.into_vec();
        assert_eq!(v.len(), 2);
        // into_vec sorts
        assert_eq!(v[0], "VK_USE_PLATFORM_WIN32_KHR");
        assert_eq!(v[1], "VK_USE_PLATFORM_XLIB_KHR");
    }

    #[test]
    fn protection_guarded_deduplicates() {
        let mut p = Protection::new_guarded();
        p.add_extension(&["VK_USE_PLATFORM_WIN32_KHR".to_string()]);
        p.add_extension(&["VK_USE_PLATFORM_WIN32_KHR".to_string()]);
        let v = p.into_vec();
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn protection_empty_guarded_into_vec_is_empty() {
        // A Guarded with no guards is treated as unconditional (empty Vec).
        let p = Protection::new_guarded();
        assert!(p.into_vec().is_empty());
    }

    #[test]
    fn protection_unconditional_into_vec_is_empty() {
        let p = Protection::Unconditional;
        assert!(p.into_vec().is_empty());
    }

    // ---- indices_to_ranges ----

    #[test]
    fn indices_to_ranges_empty() {
        assert!(indices_to_ranges(0, &[]).is_empty());
    }

    #[test]
    fn indices_to_ranges_single_element() {
        let r = indices_to_ranges(5, &[42]);
        assert_eq!(
            r,
            vec![PfnRange {
                extension: 5,
                start: 42,
                count: 1
            }]
        );
    }

    #[test]
    fn indices_to_ranges_fully_contiguous() {
        // A contiguous run should produce a single range.
        let r = indices_to_ranges(0, &[10, 11, 12, 13, 14]);
        assert_eq!(
            r,
            vec![PfnRange {
                extension: 0,
                start: 10,
                count: 5
            }]
        );
    }

    #[test]
    fn indices_to_ranges_single_gap() {
        // [3, 4, 5, 10, 11] → two ranges.
        let r = indices_to_ranges(1, &[3, 4, 5, 10, 11]);
        assert_eq!(
            r,
            vec![
                PfnRange {
                    extension: 1,
                    start: 3,
                    count: 3
                },
                PfnRange {
                    extension: 1,
                    start: 10,
                    count: 2
                },
            ]
        );
    }

    #[test]
    fn indices_to_ranges_all_disjoint() {
        // No contiguous pairs → one range per element.
        let r = indices_to_ranges(2, &[0, 5, 10]);
        assert_eq!(
            r,
            vec![
                PfnRange {
                    extension: 2,
                    start: 0,
                    count: 1
                },
                PfnRange {
                    extension: 2,
                    start: 5,
                    count: 1
                },
                PfnRange {
                    extension: 2,
                    start: 10,
                    count: 1
                },
            ]
        );
    }

    #[test]
    fn indices_to_ranges_multiple_gaps() {
        let r = indices_to_ranges(0, &[1, 2, 5, 6, 7, 20]);
        assert_eq!(
            r,
            vec![
                PfnRange {
                    extension: 0,
                    start: 1,
                    count: 2
                },
                PfnRange {
                    extension: 0,
                    start: 5,
                    count: 3
                },
                PfnRange {
                    extension: 0,
                    start: 20,
                    count: 1
                },
            ]
        );
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

    // ---- is_gl_auto_excluded ----

    #[test]
    fn gl_auto_excluded_known_names() {
        assert!(is_gl_auto_excluded("stddef"));
        assert!(is_gl_auto_excluded("khrplatform"));
        assert!(is_gl_auto_excluded("inttypes"));
    }

    #[test]
    fn gl_auto_excluded_other_names() {
        assert!(!is_gl_auto_excluded("GLuint"));
        assert!(!is_gl_auto_excluded("GLenum"));
    }

    // ---- topo_sort_typedefs: cycle fallback ----

    #[test]
    fn topo_sort_typedefs_simple_dependency_order() {
        // B depends on A (A appears in B's raw_c), so A should come first.
        let types = vec![
            TypeDef {
                name: "B".to_string(),
                raw_c: "typedef struct { A member; } B;".to_string(),
                category: "struct".to_string(),
                protect: vec![],
            },
            TypeDef {
                name: "A".to_string(),
                raw_c: "typedef struct { int x; } A;".to_string(),
                category: "struct".to_string(),
                protect: vec![],
            },
        ];
        let sorted = topo_sort_typedefs(types);
        let a_pos = sorted.iter().position(|t| t.name == "A").unwrap();
        let b_pos = sorted.iter().position(|t| t.name == "B").unwrap();
        assert!(a_pos < b_pos, "A must precede B");
    }

    #[test]
    fn topo_sort_typedefs_cycle_does_not_panic() {
        // A references B, B references A — a cycle.  The fallback path
        // must produce *some* valid output without panicking.
        let types = vec![
            TypeDef {
                name: "A".to_string(),
                raw_c: "typedef struct { B* ptr; } A;".to_string(),
                category: "struct".to_string(),
                protect: vec![],
            },
            TypeDef {
                name: "B".to_string(),
                raw_c: "typedef struct { A* ptr; } B;".to_string(),
                category: "struct".to_string(),
                protect: vec![],
            },
        ];
        let sorted = topo_sort_typedefs(types);
        // Both must appear exactly once.
        assert_eq!(sorted.len(), 2);
        assert!(sorted.iter().any(|t| t.name == "A"));
        assert!(sorted.iter().any(|t| t.name == "B"));
    }

    #[test]
    fn topo_sort_typedefs_non_scannable_categories_ignored() {
        // "define" category bodies are not scanned for deps, so even though
        // D's raw_c mentions C, no edge is created and insertion order is kept.
        let types = vec![
            TypeDef {
                name: "D".to_string(),
                raw_c: "#define D C".to_string(),
                category: "define".to_string(),
                protect: vec![],
            },
            TypeDef {
                name: "C".to_string(),
                raw_c: "typedef int C;".to_string(),
                category: "basetype".to_string(),
                protect: vec![],
            },
        ];
        let sorted = topo_sort_typedefs(types);
        // No dep edge was created, so original order is preserved.
        assert_eq!(sorted[0].name, "D");
        assert_eq!(sorted[1].name, "C");
    }
}
