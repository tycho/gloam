//! Render model — the C backend's view of a resolved `FeatureSet`.
//!
//! `FeatureSet` records spec facts (which features, extensions, commands,
//! types, and enums are in the build); `RenderModel` owns everything that is
//! a C-emission concern layered on top of those facts: protection-grouped
//! views that coalesce consecutive same-guard items into one `#if`/`#endif`
//! pair, and the packed function-name blob layout.  A future non-C backend
//! would build its own model from the same `FeatureSet`.

use serde::Serialize;

use crate::identity::Spec;
use crate::ir::TypeCategory;
use crate::resolve::{FeatureSet, FlatEnum, Param, TypeDef};

// ---------------------------------------------------------------------------
// Render model
// ---------------------------------------------------------------------------

/// Per-render precomputed template data.  Exposed to templates as `m`.
#[derive(Debug, Serialize)]
pub struct RenderModel {
    /// C context struct name, e.g. "GloamGLContext", "GloamVulkanContext".
    pub context_name: String,
    /// Include-category types, grouped by consecutive protection.
    pub include_type_groups: Vec<ProtectedGroup<TypeDef>>,
    /// Non-include types, grouped by consecutive protection.
    pub type_groups: Vec<ProtectedGroup<TypeDef>>,
    /// Extensions, grouped by consecutive protection (for #define guards and
    /// presence macros).
    pub ext_guard_groups: Vec<ProtectedGroup<ExtGuardEntry>>,
    /// Commands, grouped by consecutive protection (for PFN typedefs,
    /// IntelliSense prototypes, and dispatch macros).
    pub cmd_pfn_groups: Vec<ProtectedGroup<CmdPfnEntry>>,
    /// Flat enum constants grouped by consecutive protection, for the
    /// constants section of the header.
    pub flat_enum_groups: Vec<ProtectedGroup<FlatEnum>>,
    /// Commands the per-spec load functions must resolve by hand before the
    /// version/extension detection machinery can run (see
    /// [`bootstrap_names`]), in pfnArray order.
    pub bootstrap_cmds: Vec<BootstrapCmd>,
    /// Packed function-name blob layout (offsets passed to templates as
    /// separate context keys — the table loops index it by cmd.index).
    #[serde(skip)]
    pub fn_names: FnNameLayout,
}

impl RenderModel {
    pub fn new(fs: &FeatureSet) -> Self {
        let include_type_groups = group_by_protection(
            fs.types
                .iter()
                .filter(|t| t.category == TypeCategory::Include && !t.raw_c.is_empty())
                .cloned(),
            |t| t.protect.clone(),
        );

        let type_groups = group_by_protection(
            fs.types
                .iter()
                .filter(|t| t.category != TypeCategory::Include && !t.raw_c.is_empty())
                .cloned(),
            |t| t.protect.clone(),
        );

        let ext_guard_groups = group_by_protection_pairs(fs.extensions.iter().map(|e| {
            (
                e.protect.clone(),
                ExtGuardEntry {
                    name: e.name.clone(),
                    short_name: e.short_name.clone(),
                },
            )
        }));

        let cmd_pfn_groups = group_by_protection_pairs(fs.commands.iter().map(|c| {
            let protect = c
                .protect
                .as_ref()
                .map(|p| vec![p.clone()])
                .unwrap_or_default();
            (
                protect,
                CmdPfnEntry {
                    index: c.index,
                    name: c.name.clone(),
                    short_name: c.short_name.clone(),
                    pfn_type: pfn_type_name(fs.spec, &c.name),
                    return_type: c.return_type.clone(),
                    params_str: params_str(&c.params),
                    params: c.params.clone(),
                },
            )
        }));

        let flat_enum_groups =
            group_by_protection(fs.flat_enums.iter().cloned(), |e| e.protect.clone());

        let names = bootstrap_names(fs.spec);
        let bootstrap_cmds = fs
            .commands
            .iter()
            .filter(|c| names.contains(&c.name.as_str()))
            .map(|c| BootstrapCmd {
                name: c.name.clone(),
                short_name: c.short_name.clone(),
                pfn_type: pfn_type_name(fs.spec, &c.name),
            })
            .collect();

        Self {
            context_name: fs.spec.context_name(),
            include_type_groups,
            type_groups,
            ext_guard_groups,
            cmd_pfn_groups,
            flat_enum_groups,
            bootstrap_cmds,
            fn_names: FnNameLayout::build(fs),
        }
    }
}

// ---------------------------------------------------------------------------
// C naming policy
// ---------------------------------------------------------------------------

/// PFN typedef name for a command: `PFN_vkFoo` for Vulkan, `PFNGLFOOPROC`
/// for the GL family (the lowercase api prefix is stripped before
/// uppercasing so we don't get PFNGLGLFOOPROC).
fn pfn_type_name(spec: Spec, cmd_name: &str) -> String {
    if spec == Spec::Vk {
        format!("PFN_{cmd_name}")
    } else {
        let stem = cmd_name
            .strip_prefix(spec.name_prefix())
            .unwrap_or(cmd_name);
        format!("{}{}PROC", spec.pfn_prefix(), stem.to_uppercase())
    }
}

/// C parameter list text for PFN typedefs and prototypes (empty → "void").
fn params_str(params: &[Param]) -> String {
    if params.is_empty() {
        return "void".to_string();
    }
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
}

// ---------------------------------------------------------------------------
// Bootstrap commands
// ---------------------------------------------------------------------------

/// A command the generated load function assigns by hand from getProcAddr
/// before the generic range loader can run.
#[derive(Debug, Serialize)]
pub struct BootstrapCmd {
    pub name: String,
    pub short_name: String,
    pub pfn_type: String,
}

/// The commands each spec's load function must resolve up front: version
/// detection (find_core) reads the version through them, and WGL needs its
/// extensions-string entry points before extension detection.  These are
/// C loader policy, not spec facts — a command listed here that isn't in the
/// feature set is simply absent from the build (e.g. a filtered build that
/// somehow drops glGetString generates a load function that returns 0).
fn bootstrap_names(spec: Spec) -> &'static [&'static str] {
    match spec {
        Spec::Gl => &["glGetString"],
        Spec::Egl => &["eglGetString", "eglQueryString"],
        Spec::Glx => &["glXQueryVersion"],
        Spec::Wgl => &["wglGetExtensionsStringARB", "wglGetExtensionsStringEXT"],
        // Vulkan bootstraps through GetInstanceProcAddr, spelled out directly
        // in the template — no name-table lookups.
        Spec::Vk => &[],
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pfn_type_vulkan_keeps_command_case() {
        assert_eq!(
            pfn_type_name(Spec::Vk, "vkCreateInstance"),
            "PFN_vkCreateInstance"
        );
    }

    #[test]
    fn pfn_type_gl_family_strips_prefix_before_uppercasing() {
        assert_eq!(pfn_type_name(Spec::Gl, "glCullFace"), "PFNGLCULLFACEPROC");
        assert_eq!(
            pfn_type_name(Spec::Glx, "glXQueryVersion"),
            "PFNGLXQUERYVERSIONPROC"
        );
    }

    fn param(type_raw: &str, name: &str) -> Param {
        Param {
            type_raw: type_raw.to_string(),
            name: name.to_string(),
        }
    }

    #[test]
    fn params_str_empty_is_void() {
        assert_eq!(params_str(&[]), "void");
    }

    #[test]
    fn params_str_joins_type_and_name() {
        assert_eq!(
            params_str(&[param("GLenum", "mode"), param("const GLuint *", "ids")]),
            "GLenum mode, const GLuint * ids"
        );
    }

    #[test]
    fn params_str_array_param_emitted_verbatim() {
        // type_raw already carries the name and array suffix.
        assert_eq!(
            params_str(&[param("float blendConstants[4]", "blendConstants")]),
            "float blendConstants[4]"
        );
    }
}

// ---------------------------------------------------------------------------
// Protection-grouped view types
// ---------------------------------------------------------------------------

/// A group of items that share the same platform protection macros.
///
/// Adjacent items with identical protection are coalesced into a single
/// group so that the generated header emits one `#ifdef`/`#endif` pair per
/// run of identically-protected items, rather than one per item.
#[derive(Debug, Serialize)]
pub struct ProtectedGroup<T: std::fmt::Debug + Serialize> {
    /// Protection macros for this group.  Empty = unconditional (no guard).
    pub protect: Vec<String>,
    /// Items in this group, in their original order.
    pub items: Vec<T>,
}

/// Lightweight extension entry for protection-grouped header sections.
/// Carries only the fields needed by the `#define` guard and presence macro
/// sections, avoiding a full `Extension` clone.
#[derive(Debug, Serialize)]
pub struct ExtGuardEntry {
    pub name: String,
    pub short_name: String,
}

/// Lightweight command entry for protection-grouped header sections.
/// Carries only the fields needed by PFN typedefs, IntelliSense prototypes,
/// dispatch macros, and the context struct (which needs `index` for pad slot
/// naming).
#[derive(Debug, Serialize)]
pub struct CmdPfnEntry {
    pub index: u16,
    pub name: String,
    pub short_name: String,
    pub pfn_type: String,
    pub return_type: String,
    pub params_str: String,
    /// Individual parameters for inline function dispatch wrappers.
    pub params: Vec<Param>,
}

// ---------------------------------------------------------------------------
// Group-by-protection coalescing
// ---------------------------------------------------------------------------

/// Coalesce items into groups of consecutive items that share the same
/// protection macro set.  A single linear pass — O(n) in the item count.
///
/// Takes `(Vec<String>, T)` pairs where the first element is the protection
/// list for that item.  This signature avoids a closure parameter and
/// eliminates the duplicated manual grouping loops that existed for
/// ext_guard_groups and cmd_pfn_groups.
fn group_by_protection_pairs<T>(
    items: impl IntoIterator<Item = (Vec<String>, T)>,
) -> Vec<ProtectedGroup<T>>
where
    T: std::fmt::Debug + Serialize,
{
    let mut groups: Vec<ProtectedGroup<T>> = Vec::new();
    for (protect, item) in items {
        if let Some(last) = groups.last_mut()
            && last.protect == protect
        {
            last.items.push(item);
            continue;
        }
        groups.push(ProtectedGroup {
            protect,
            items: vec![item],
        });
    }
    groups
}

/// Convenience wrapper: coalesce items using a closure to extract protection.
fn group_by_protection<T, F>(
    items: impl IntoIterator<Item = T>,
    get_protect: F,
) -> Vec<ProtectedGroup<T>>
where
    T: std::fmt::Debug + Serialize,
    F: Fn(&T) -> Vec<String>,
{
    let mut groups: Vec<ProtectedGroup<T>> = Vec::new();
    for item in items {
        let protect = get_protect(&item);
        if let Some(last) = groups.last_mut()
            && last.protect == protect
        {
            last.items.push(item);
            continue;
        }
        groups.push(ProtectedGroup {
            protect,
            items: vec![item],
        });
    }
    groups
}

// ---------------------------------------------------------------------------
// Function name blob layout
// ---------------------------------------------------------------------------

/// Pre-computed function name string blob layout.
///
/// Each command name is stored as a NUL-terminated string in a single
/// contiguous char array, with a parallel offset table for O(1) indexing.
/// This avoids one pointer + relocation per command (~30 bytes/command on
/// PIC builds).
#[derive(Debug)]
pub struct FnNameLayout {
    /// Byte offset of each command's name within the blob.
    pub offsets: Vec<u32>,
    /// C type for the offset table: "uint16_t" or "uint32_t".
    pub offset_type: &'static str,
}

impl FnNameLayout {
    fn build(fs: &FeatureSet) -> Self {
        let mut offsets = Vec::with_capacity(fs.commands.len());
        let mut pos = 0u32;
        for cmd in &fs.commands {
            offsets.push(pos);
            pos += cmd.name.len() as u32 + 1; // +1 for NUL
        }
        let offset_type = if pos <= u16::MAX as u32 {
            "uint16_t"
        } else {
            "uint32_t"
        };
        Self {
            offsets,
            offset_type,
        }
    }
}
