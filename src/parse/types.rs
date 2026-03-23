//! Parsing of `<types>` sections into `RawType` records, with topological
//! dependency ordering.

use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::Result;

use super::{SpecDocs, extract_raw_c};
use crate::ir::RawType;

// GL pointer types that need the macOS ptrdiff_t guard (spec gotcha #7).
const MACOS_PTRDIFF_TYPES: &[&str] = &["GLsizeiptr", "GLintptr", "GLsizeiptrARB", "GLintptrARB"];

pub fn parse_types(docs: &SpecDocs<'_, '_>, _spec_name: &str) -> Result<Vec<RawType>> {
    let type_nodes = docs.section_children("types");

    // Collect all RawType entries.  Multiple entries can share a name (api variants).
    let mut raw: Vec<RawType> = Vec::with_capacity(type_nodes.len());

    for node in &type_nodes {
        if node.tag_name().name() != "type" {
            continue;
        }

        // Determine the name: prefer `name=` attribute, then direct <n>
        // child, then <proto><n> for the structured funcpointer format
        // where the name lives inside <proto> rather than at top level.
        let name = if let Some(n) = node.attribute("name") {
            n.to_string()
        } else if let Some(name_elem) = node
            .children()
            .find(|n| n.is_element() && n.tag_name().name() == "name")
        {
            name_elem.text().unwrap_or("").to_string()
        } else if let Some(proto) = node
            .children()
            .find(|n| n.is_element() && n.tag_name().name() == "proto")
        {
            if let Some(name_elem) = proto
                .children()
                .find(|n| n.is_element() && n.tag_name().name() == "name")
            {
                name_elem.text().unwrap_or("").to_string()
            } else {
                eprintln!("warning: <type> with no discernible name, skipping");
                continue;
            }
        } else {
            eprintln!("warning: <type> with no discernible name, skipping");
            continue;
        };

        if name.is_empty() {
            continue;
        }

        let api = node.attribute("api").map(str::to_string);
        let category = node.attribute("category").unwrap_or("").to_string();
        let requires = node.attribute("requires").map(str::to_string);
        let alias = node.attribute("alias").map(str::to_string);
        let protect = node.attribute("protect").map(str::to_string);

        // Bitwidth: explicit attr, or inherit later from alias chain.
        let bitwidth = node
            .attribute("bitwidth")
            .and_then(|s| s.parse::<u32>().ok());

        // For enum-category types, we emit nothing from this node directly —
        // the actual enum group is built in enums.rs.  We still record the
        // entry so the alias chain and bitwidth propagation can work.
        let raw_c = if category == "include" {
            // Emit as a verbatim #include directive — roxmltree decodes XML
            // entities so &lt;X11/Xlib.h&gt; arrives as <X11/Xlib.h>.
            let text = extract_raw_c(*node).trim().to_string();
            if text.is_empty() {
                // Empty body (e.g. `<type category="include" name="X11/Xlib.h"/>`):
                // the name attribute IS the header path — synthesize the directive.
                // Platform system headers use angle-bracket form.
                if name.ends_with(".h") && !name.starts_with("vk") {
                    format!("#include <{}>", name)
                } else {
                    // vk_platform and similar: quoted form.
                    format!("#include \"{}\"", name)
                }
            } else {
                text
            }
        } else if category == "enum" {
            // Enum aliases (e.g. VkComponentTypeNV = VkComponentTypeKHR) need
            // a typedef emission.  Plain enum types have no direct C emission
            // — their values are handled by enum groups in enums.rs.
            //
            // We deliberately omit the `enum` keyword: `typedef X Y` rather
            // than `typedef enum X Y`.  For 32-bit enums, both forms are
            // equivalent since `typedef enum X { ... } X` creates both the
            // enum tag and the typedef.  For 64-bit enums, however, the
            // pre-C23 C path emits `typedef uint64_t X` (no enum tag), so
            // `typedef enum X Y` would be a forward reference to a
            // non-existent enum — triggering clang's
            // -Wmicrosoft-enum-forward-reference diagnostic.  The plain
            // `typedef X Y` form works in all C/C++ versions regardless of
            // bitwidth, because the topo sort guarantees X is already defined.
            if let Some(ref al) = alias {
                format!("typedef {} {};", al, name)
            } else {
                String::new()
            }
        } else if category == "struct" || category == "union" {
            if alias.is_some() {
                // Alias: `typedef AliasedName NewName` (semicolon added by
                // normalize_raw_c in the resolver).
                format!("typedef {} {};", alias.as_deref().unwrap(), name)
            } else {
                // Build a proper `typedef struct Name { ... } Name` from the
                // <member> children.  extract_raw_c_inner would concatenate all
                // member text as a flat blob, producing incorrect output.
                extract_struct_c(*node, &name, &category)
            }
        } else if category == "funcpointer" {
            // Vulkan funcpointers come in two XML formats:
            //
            // Old (inline text): `typedef void* (VKAPI_PTR *NAME)(params...);`
            //   extract_raw_c handles this correctly.
            //
            // New (VulkanBase-era): structured <proto> and <param> children,
            //   analogous to <command> elements.  extract_raw_c would produce
            //   garbled output here since it concatenates all child text naively.
            if node
                .children()
                .any(|n| n.is_element() && n.tag_name().name() == "proto")
            {
                extract_funcpointer_c(*node, &name)
            } else {
                extract_raw_c(*node).trim().to_string()
            }
        } else {
            // For alias-only entries (bitmask, handle, basetype aliases) where
            // extract_raw_c returns empty, emit a #define.  These are types
            // like VkPipelineStageFlags2KHR (alias of VkPipelineStageFlags2)
            // where a typedef would be ill-formed because the aliased type is
            // already a typedef'd integer — #define is the correct form.
            let mut c = extract_raw_c(*node).trim().to_string();
            if c.is_empty()
                && let Some(ref al) = alias
            {
                c = format!("#define {} {}", name, al);
            }
            // Apply macOS ptrdiff_t guard for the affected GL types (gotcha #7).
            if MACOS_PTRDIFF_TYPES.contains(&name.as_str()) {
                c = macos_ptrdiff_guard(&name, &c);
            }
            c
        };

        raw.push(RawType {
            name,
            api,
            category,
            requires,
            alias,
            bitwidth,
            raw_c,
            protect,
        });
    }

    // Propagate bitwidth=64 through alias chains (spec gotcha #4).
    // This metadata is retained on RawType for correctness; it is no longer
    // needed for alias raw_c fixup since enum aliases are now always emitted
    // as plain `typedef X Y` (no `enum` keyword), which is valid regardless
    // of bitwidth.
    propagate_bitwidth(&mut raw);

    // Topological sort by dependency order (spec gotcha #2).
    let sorted = topological_sort(raw);

    Ok(sorted)
}

// ---------------------------------------------------------------------------
// macOS ptrdiff_t guard (spec gotcha #7)
// ---------------------------------------------------------------------------

fn macos_ptrdiff_guard(name: &str, _original: &str) -> String {
    format!(
        "#if defined(__ENVIRONMENT_MAC_OS_X_VERSION_MIN_REQUIRED__) \\\n\
         && (__ENVIRONMENT_MAC_OS_X_VERSION_MIN_REQUIRED__ > 1060)\n\
         typedef long {name};\n\
         #else\n\
         typedef ptrdiff_t {name};\n\
         #endif",
        name = name
    )
}

// ---------------------------------------------------------------------------
// Bitwidth propagation (spec gotcha #4)
// ---------------------------------------------------------------------------

fn propagate_bitwidth(types: &mut [RawType]) {
    // Build a map name -> bitwidth for 64-bit entries.
    let bw64: HashSet<String> = types
        .iter()
        .filter(|t| t.bitwidth == Some(64))
        .map(|t| t.name.clone())
        .collect();

    for t in types.iter_mut() {
        if t.bitwidth.is_none()
            && let Some(ref alias) = t.alias
            && bw64.contains(alias.as_str())
        {
            t.bitwidth = Some(64);
        }
    }
}

// ---------------------------------------------------------------------------
// Struct / union C reconstruction
// ---------------------------------------------------------------------------

/// Build a `typedef struct Name { ... } Name` declaration from a Vulkan
/// `<type category="struct">` or `<type category="union">` element.
///
/// `extract_raw_c_inner` is not usable here because it would concatenate
/// all `<member>` sub-element text into a single flat string with no
/// separators, losing the per-member line boundaries.
fn extract_struct_c(node: roxmltree::Node<'_, '_>, name: &str, category: &str) -> String {
    let kw = if category == "union" {
        "union"
    } else {
        "struct"
    };

    let mut members: Vec<String> = Vec::new();
    for child in node.children().filter(|n| n.is_element()) {
        if child.tag_name().name() != "member" {
            continue;
        }
        // Skip members restricted to a non-Vulkan API variant.
        // e.g. api="vulkansc" members must not appear in the vulkan header.
        if let Some(api) = child.attribute("api") {
            // Keep only if "vulkan" is among the comma-separated api values.
            if !api.split(',').any(|a| a.trim() == "vulkan") {
                continue;
            }
        }
        let text = super::extract_raw_c(child);
        let trimmed = text.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.ends_with(';') {
            members.push(format!("    {}", trimmed));
        } else {
            members.push(format!("    {};", trimmed));
        }
    }

    if members.is_empty() {
        return format!("typedef {} {} {{}};", kw, name);
    }

    format!(
        "typedef {} {} {{\n{}\n}} {};",
        kw,
        name,
        members.join("\n"),
        name
    )
}

/// Build a `typedef RET (VKAPI_PTR *NAME)(params)` declaration from a
/// structured `<type category="funcpointer">` element that uses `<proto>`
/// and `<param>` children (VulkanBase-era format).
fn extract_funcpointer_c(node: roxmltree::Node<'_, '_>, name: &str) -> String {
    // Extract return type from <proto>: everything before <name>.
    let mut ret = String::new();
    if let Some(proto) = node
        .children()
        .find(|n| n.is_element() && n.tag_name().name() == "proto")
    {
        for child in proto.children() {
            if child.is_text() {
                ret.push_str(child.text().unwrap_or(""));
            } else if child.is_element() {
                match child.tag_name().name() {
                    "name" => break, // stop before the name
                    "type" => ret.push_str(child.text().unwrap_or("")),
                    _ => ret.push_str(&super::extract_raw_c(child)),
                }
            }
        }
    }
    let ret = ret.trim();

    // Build parameter list from <param> children.
    let mut params: Vec<String> = Vec::new();
    for param in node
        .children()
        .filter(|n| n.is_element() && n.tag_name().name() == "param")
    {
        let param_text = super::extract_raw_c(param);
        let trimmed = param_text.trim();
        if !trimmed.is_empty() {
            params.push(trimmed.to_string());
        }
    }
    let params_str = if params.is_empty() {
        "void".to_string()
    } else {
        params.join(", ")
    };

    format!("typedef {} (VKAPI_PTR *{})({});", ret, name, params_str)
}

// ---------------------------------------------------------------------------
// Topological sort (Kahn's algorithm)
// ---------------------------------------------------------------------------
//
// A type A depends on type B if:
//   - A.requires == B.name  (explicit attribute dep)
//   - A.alias    == B.name  (alias chain dep)
//   - A is a funcpointer and B's name appears in A's raw C text (param types)
//
// We sort so that every type appears after all its dependencies.

/// Iterate over every identifier-shaped word in `s`.
/// Splits on anything that isn't `[A-Za-z0-9_]` and yields non-empty words.
pub(crate) fn ident_words(s: &str) -> impl Iterator<Item = &str> {
    s.split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
        .filter(|w| !w.is_empty())
}

fn topological_sort(types: Vec<RawType>) -> Vec<RawType> {
    // Build an index: name -> list of positions in `types`.
    // (Multiple variants of the same name can exist with different api values.)
    let mut name_to_indices: HashMap<&str, Vec<usize>> = HashMap::new();
    for (i, t) in types.iter().enumerate() {
        name_to_indices.entry(&t.name).or_default().push(i);
    }

    // For each type, collect the indices of its dependencies.
    // Dependencies come from three sources:
    //   1. The `requires=` attribute (explicit prerequisite).
    //   2. The `alias=` attribute (must emit the aliased type first).
    //   3. Any GL type name appearing in the raw C text.  We scan ALL types
    //      unconditionally — not just funcpointer category — because supplemental
    //      XMLs (e.g. gl_angle_ext.xml) may use a different or absent category
    //      attribute on their function pointer typedefs.  False matches are
    //      impossible: C keywords and parameter names never appear as GL type
    //      names in name_to_indices.
    let deps: Vec<Vec<usize>> = types
        .iter()
        .map(|t| {
            let mut d = Vec::new();
            if let Some(ref req) = t.requires
                && let Some(idxs) = name_to_indices.get(req.as_str())
            {
                d.extend_from_slice(idxs);
            }
            if let Some(ref alias) = t.alias
                && let Some(idxs) = name_to_indices.get(alias.as_str())
            {
                d.extend_from_slice(idxs);
            }
            for word in ident_words(&t.raw_c) {
                if word == t.name {
                    continue;
                }
                if let Some(idxs) = name_to_indices.get(word) {
                    d.extend_from_slice(idxs);
                }
            }
            // Deduplicate: the same dep can appear from multiple sources.
            // Duplicates would inflate in_degree and strand nodes in the
            // cycle fallback path.
            d.sort_unstable();
            d.dedup();
            d
        })
        .collect();

    // In-degree: for each node, how many prerequisites must come before it.
    // This is simply the length of its own dependency list.
    let mut in_degree: Vec<usize> = deps.iter().map(|d| d.len()).collect();

    // Reverse adjacency: for each node, which nodes depend on it?
    // Used to decrement dependents when a node is processed.
    let mut rev: Vec<Vec<usize>> = vec![Vec::new(); types.len()];
    for (i, dep_list) in deps.iter().enumerate() {
        for &dep in dep_list {
            rev[dep].push(i);
        }
    }

    let mut queue: VecDeque<usize> = (0..types.len()).filter(|&i| in_degree[i] == 0).collect();
    let mut order = Vec::with_capacity(types.len());

    while let Some(node) = queue.pop_front() {
        order.push(node);
        for &dependent in &rev[node] {
            in_degree[dependent] -= 1;
            if in_degree[dependent] == 0 {
                queue.push_back(dependent);
            }
        }
    }

    // If there are remaining unvisited nodes (cycles), append them last.
    if order.len() < types.len() {
        for (i, item) in in_degree.iter().enumerate().take(types.len()) {
            if *item != 0 {
                order.push(i);
            }
        }
    }

    // Consume `types` and reorder.
    let mut types_opt: Vec<Option<RawType>> = types.into_iter().map(Some).collect();
    order
        .into_iter()
        .map(|i| types_opt[i].take().unwrap())
        .collect()
}
