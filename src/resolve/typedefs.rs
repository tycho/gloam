//! Type list building, topological sorting, and include protection inference.
//!
//! Transforms raw type definitions from the spec into `TypeDef` entries with
//! correct dependency ordering and platform protection, plus auxiliary header
//! collection.

use std::collections::{HashMap, HashSet, VecDeque};

use indexmap::IndexMap;

use crate::ir::RawSpec;
use crate::parse::types::ident_words;

use super::protect::{Protection, is_gl_auto_excluded};
use super::selection::SelectedExt;
use super::types::TypeDef;

// ---------------------------------------------------------------------------
// Build type list
// ---------------------------------------------------------------------------

pub(super) fn build_type_list(
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
                // protect= attribute.
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

// ---------------------------------------------------------------------------
// Topological sort
// ---------------------------------------------------------------------------

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

    let mut name_to_idx: HashMap<&str, usize> = HashMap::new();
    for (i, t) in types.iter().enumerate() {
        name_to_idx.insert(t.name.as_str(), i);
    }

    let scan_cats: &[&str] = &["struct", "union", "funcpointer"];

    let deps: Vec<Vec<usize>> = types
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let mut d: Vec<usize> = Vec::new();
            if scan_cats.contains(&t.category.as_str()) {
                for word in ident_words(&t.raw_c) {
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
    // B's name, B comes before A.
    if order.len() < n {
        let stranded: Vec<usize> = (0..n).filter(|&i| in_degree[i] != 0).collect();
        let stranded_set: HashSet<usize> = stranded.iter().copied().collect();

        let s_deps: HashMap<usize, HashSet<usize>> = stranded
            .iter()
            .map(|&i| {
                let deps_i: HashSet<usize> = ident_words(&types[i].raw_c)
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
// Extension-derived type protections
// ---------------------------------------------------------------------------

/// Build a map from type name → protection macros derived purely from the
/// extensions that require that type.
///
/// This covers the common Vulkan pattern where a struct has no `protect=`
/// attribute on its `<type>` element but is required only inside an extension
/// with `platform="win32"` (or similar), making its protection implicit.
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

// ---------------------------------------------------------------------------
// Include protection inference
// ---------------------------------------------------------------------------

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
fn infer_include_protections(
    raw: &RawSpec,
    selected_exts: &[SelectedExt<'_>],
) -> HashMap<String, Vec<String>> {
    // Step 1: include_name → set of type names that `requires=` it.
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
    let mut type_protect: HashMap<&str, Protection> = HashMap::new();

    let all_dep_names: HashSet<&str> = include_to_deps
        .values()
        .flat_map(|s| s.iter().copied())
        .collect();

    // Source (a): extension require blocks.
    for ext in selected_exts {
        for require in &ext.raw.requires {
            for type_name in &require.types {
                record_protect(
                    type_name.as_str(),
                    &ext.raw.protect,
                    &all_dep_names,
                    &mut type_protect,
                );
            }
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
            None => continue,
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
pub(super) fn collect_required_headers(
    raw: &RawSpec,
    req_types: &HashSet<String>,
    spec_name: &str,
) -> Vec<String> {
    let mut headers: IndexMap<String, ()> = IndexMap::new();

    for t in &raw.types {
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

    if spec_name == "vk" {
        headers.insert("vk_platform.h".to_string(), ());

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

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Map a `requires=` value to a *bundled* header path we own and copy to the
/// output tree.
fn requires_to_bundled_header(requires: &str) -> Option<&'static str> {
    match requires {
        "khrplatform" => Some("KHR/khrplatform.h"),
        "eglplatform" => Some("EGL/eglplatform.h"),
        "vk_platform" => Some("vk_platform.h"),
        _ => None,
    }
}

/// True if an include-category type name refers to a header that we bundle.
fn is_bundled_include_type(name: &str) -> bool {
    matches!(name, "vk_platform" | "khrplatform" | "eglplatform") || name.starts_with("vk_video/")
}

pub(super) fn normalize_raw_c(raw: &str) -> String {
    raw.trim().to_string()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn topo_sort_typedefs_simple_dependency_order() {
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
        assert_eq!(sorted.len(), 2);
        assert!(sorted.iter().any(|t| t.name == "A"));
        assert!(sorted.iter().any(|t| t.name == "B"));
    }

    #[test]
    fn topo_sort_typedefs_non_scannable_categories_ignored() {
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
        assert_eq!(sorted[0].name, "D");
        assert_eq!(sorted[1].name, "C");
    }
}
