//! Enum group assembly and flat enum construction.
//!
//! Builds the flat `#define` constants and Vulkan typed enum groups from the
//! raw spec, with value-dependency sorting to ensure forward references are
//! resolved before use.

use std::collections::{HashMap, HashSet, VecDeque};

use crate::ir::RawSpec;

use super::protect::Protection;
use super::types::{EnumGroup, FlatEnum};

// ---------------------------------------------------------------------------
// Flat enums
// ---------------------------------------------------------------------------

pub(super) fn build_flat_enums(
    raw: &RawSpec,
    req_enums: &HashSet<String>,
    is_vulkan: bool,
) -> Vec<FlatEnum> {
    let enum_protect = build_ext_enum_protections(raw);
    raw.flat_enums
        .iter()
        // For Vulkan, all flat enums are API constants (VK_MAX_DESCRIPTION_SIZE
        // etc.) that are never explicitly listed in <require> blocks but are
        // always needed.  For GL, only emit constants selected by the feature set.
        .filter(|(name, _)| is_vulkan || req_enums.contains(*name))
        .filter_map(|(_, e)| {
            let value = e.value.as_deref().or(e.alias.as_deref())?;
            let protect = enum_protect
                .get(e.name.as_str())
                .cloned()
                .unwrap_or_default();
            Some(FlatEnum {
                name: e.name.clone(),
                literal_value: value.to_string(),
                value: value.to_string(),
                comment: e.comment.clone(),
                protect,
            })
        })
        .collect()
}

/// Build a map from enum constant name → platform protection macros.
fn build_ext_enum_protections(raw: &RawSpec) -> HashMap<String, Vec<String>> {
    let mut tmp: HashMap<&str, Protection> = HashMap::new();

    for ext in &raw.extensions {
        for require in &ext.requires {
            for enum_name in &require.enums {
                tmp.entry(enum_name.as_str())
                    .or_insert_with(Protection::new_guarded)
                    .add_extension(&ext.protect);
            }
        }
    }

    tmp.into_iter()
        .map(|(name, prot)| (name.to_string(), prot.into_vec()))
        .collect()
}

// ---------------------------------------------------------------------------
// Enum groups (Vulkan)
// ---------------------------------------------------------------------------

pub(super) fn build_enum_groups(raw: &RawSpec) -> Vec<EnumGroup> {
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
                        literal_value: String::new(), // resolved below
                        comment: v.comment.clone(),
                        protect: vec![],
                    })
                })
                .collect();

            let mut sorted = sort_enum_values(raw_values);
            resolve_literal_values(&mut sorted);

            EnumGroup {
                name: g.name.clone(),
                is_bitmask: false,
                bitwidth: g.bitwidth.unwrap_or(32),
                values: sorted,
            }
        })
        .collect()
}

/// Fill `literal_value` for each entry.  For numeric literals, it's the value
/// itself.  For aliases (value is another enum name), look up the target's
/// `literal_value`.  Assumes topological order (canonical before alias).
fn resolve_literal_values(values: &mut [FlatEnum]) {
    // Build a name→literal map as we go (topo order guarantees targets are resolved first).
    let mut literals: HashMap<String, String> = HashMap::new();
    for v in values.iter_mut() {
        let resolved = literals
            .get(&v.value)
            .cloned()
            .unwrap_or_else(|| v.value.clone());
        v.literal_value = resolved.clone();
        literals.insert(v.name.clone(), resolved);
    }
}

// ---------------------------------------------------------------------------
// Value-dependency sort
// ---------------------------------------------------------------------------

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

    let name_to_idx: HashMap<&str, usize> = values
        .iter()
        .enumerate()
        .map(|(i, v)| (v.name.as_str(), i))
        .collect();

    let deps: Vec<Option<usize>> = values
        .iter()
        .map(|v| {
            let val = v.value.trim();
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

    let mut in_degree: Vec<usize> = deps.iter().map(|d| d.is_some() as usize).collect();

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
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_enum(name: &str, value: &str) -> FlatEnum {
        FlatEnum {
            name: name.to_string(),
            value: value.to_string(),
            literal_value: String::new(),
            comment: String::new(),
            protect: vec![],
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
        let input = vec![make_enum("VK_MAX", "-1"), make_enum("VK_ZERO", "0")];
        let out = sort_enum_values(input);
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
}
