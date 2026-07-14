//! Render model — the C backend's view of a resolved `FeatureSet`.
//!
//! `FeatureSet` records spec facts (which features, extensions, commands,
//! types, and enums are in the build); `RenderModel` owns everything that is
//! a C-emission concern layered on top of those facts: protection-grouped
//! views that coalesce consecutive same-guard items into one `#if`/`#endif`
//! pair, and the packed function-name blob layout.  A future non-C backend
//! would build its own model from the same `FeatureSet`.

use serde::Serialize;

use crate::ir::TypeCategory;
use crate::resolve::{FeatureSet, FlatEnum, Param, TypeDef};

// ---------------------------------------------------------------------------
// Render model
// ---------------------------------------------------------------------------

/// Per-render precomputed template data.  Exposed to templates as `m`.
#[derive(Debug, Serialize)]
pub struct RenderModel {
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
                    pfn_type: c.pfn_type.clone(),
                    return_type: c.return_type.clone(),
                    params_str: c.params_str.clone(),
                    params: c.params.clone(),
                },
            )
        }));

        let flat_enum_groups =
            group_by_protection(fs.flat_enums.iter().cloned(), |e| e.protect.clone());

        Self {
            include_type_groups,
            type_groups,
            ext_guard_groups,
            cmd_pfn_groups,
            flat_enum_groups,
            fn_names: FnNameLayout::build(fs),
        }
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
