//! Parsing of `<types>` sections into `RawType` records, with topological
//! dependency ordering.

use std::collections::{HashMap, HashSet, VecDeque};

use super::ctype::TypeRef;
use super::xml::NodeExt;
use super::{SpecDocs, extract_raw_c};
use crate::diag::Diag;
use crate::ir::{RawFnSig, RawMember, RawType, TypeCategory, TypePayload, TypedefArm};

// GL pointer types that need the macOS ptrdiff_t guard (spec gotcha #7).
const MACOS_PTRDIFF_TYPES: &[&str] = &["GLsizeiptr", "GLintptr", "GLsizeiptrARB", "GLintptrARB"];

/// Determine a `<type>` element's name: prefer the `name=` attribute, then a
/// direct `<name>` child, then `<proto><name>` for the structured funcpointer
/// format where the name lives inside `<proto>` rather than at top level.
fn type_name(node: roxmltree::Node<'_, '_>) -> Option<String> {
    if let Some(n) = node.attribute("name") {
        return Some(n.to_string());
    }
    if let Some(name_elem) = node.child("name") {
        return Some(name_elem.text().unwrap_or("").to_string());
    }
    node.child("proto")?
        .child("name")
        .map(|n| n.text().unwrap_or("").to_string())
}

pub fn parse_types(docs: &SpecDocs<'_, '_>, diag: Diag) -> Vec<RawType> {
    let type_nodes = docs.section_children("types");

    // Collect all RawType entries.  Multiple entries can share a name (api variants).
    let mut raw: Vec<RawType> = Vec::with_capacity(type_nodes.len());

    for node in &type_nodes {
        if node.tag_name().name() != "type" {
            continue;
        }

        // An unnameable <type> is warn-and-skip, not an error: it may belong
        // to content the build never selects, and if something selected does
        // depend on it, resolution fails loudly on the missing type.
        let Some(name) = type_name(*node).filter(|n| !n.is_empty()) else {
            diag.warn("<type> with no discernible name, skipping");
            continue;
        };

        let api = node.attribute("api").map(str::to_string);
        let category = TypeCategory::from_attr(node.attribute("category"));
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
        //
        // Each branch yields the assembled C text plus the structured
        // `TypePayload` view of the same body.  Structured branches
        // (struct/union members, new-format funcpointers) build both from
        // one XML walk; text branches parse the finished C fragment, so a
        // body outside the known shapes degrades to `Opaque` rather than
        // erroring here (the corpus test pins full structured coverage for
        // the categories that need it).
        let (raw_c, payload) = if category == TypeCategory::Include {
            // Emit as a verbatim #include directive — roxmltree decodes XML
            // entities so &lt;X11/Xlib.h&gt; arrives as <X11/Xlib.h>.
            let text = extract_raw_c(*node).trim().to_string();
            let c = if text.is_empty() {
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
            };
            (c, TypePayload::Opaque)
        } else if category == TypeCategory::Enum {
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
                let c = format!("typedef {} {};", al, name);
                let payload = text_payload(&c);
                (c, payload)
            } else {
                (String::new(), TypePayload::Opaque)
            }
        } else if matches!(category, TypeCategory::Struct | TypeCategory::Union) {
            if alias.is_some() {
                // Alias: `typedef AliasedName NewName` (semicolon added by
                // normalize_raw_c in the resolver).
                let c = format!("typedef {} {};", alias.as_deref().unwrap(), name);
                let payload = text_payload(&c);
                (c, payload)
            } else {
                // Build a proper `typedef struct Name { ... } Name` from the
                // <member> children.  extract_raw_c_inner would concatenate all
                // member text as a flat blob, producing incorrect output.
                extract_struct(*node, &name, category)
            }
        } else if category == TypeCategory::Funcpointer {
            // Vulkan funcpointers come in two XML formats:
            //
            // Old (inline text): `typedef void* (VKAPI_PTR *NAME)(params...);`
            //   extract_raw_c handles this correctly.
            //
            // New (VulkanBase-era): structured <proto> and <param> children,
            //   analogous to <command> elements.  extract_raw_c would produce
            //   garbled output here since it concatenates all child text naively.
            if node.child("proto").is_some() {
                extract_funcpointer(*node, &name)
            } else {
                let c = extract_raw_c(*node).trim().to_string();
                let payload = text_payload(&c);
                (c, payload)
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
            let payload = if category == TypeCategory::Handle {
                handle_payload(&c)
            } else {
                text_payload(&c)
            };
            (c, payload)
        };

        raw.push(RawType {
            name,
            api,
            category,
            requires,
            alias,
            bitwidth,
            raw_c,
            payload,
            protect,
        });
    }

    // Propagate bitwidth=64 through alias chains (spec gotcha #4).
    // The metadata rides on RawType; alias raw_c needs no bitwidth fixup
    // because enum aliases are emitted as plain `typedef X Y` (no `enum`
    // keyword), which is valid regardless of bitwidth.
    propagate_bitwidth(&mut raw);

    // Topological sort by dependency order (spec gotcha #2).
    topological_sort(raw)
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

/// Build a `typedef struct Name { ... } Name` declaration — plus the
/// structured member list — from a Vulkan `<type category="struct">` or
/// `<type category="union">` element.
///
/// `extract_raw_c_inner` is not usable here because it would concatenate
/// all `<member>` sub-element text into a single flat string with no
/// separators, losing the per-member line boundaries.
///
/// The C text and the payload come from the same walk over the same member
/// strings, so the two views cannot drift.  If any member's declarator falls
/// outside the `TypeRef` grammar the payload degrades to `Opaque` (the C
/// text is unaffected); the corpus test pins full structured coverage.
fn extract_struct(
    node: roxmltree::Node<'_, '_>,
    name: &str,
    category: TypeCategory,
) -> (String, TypePayload) {
    let kw = if category == TypeCategory::Union {
        "union"
    } else {
        "struct"
    };

    let mut members: Vec<String> = Vec::new();
    let mut records: Option<Vec<RawMember>> = Some(Vec::new());
    for child in node.children_named("member") {
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

        // Structured record for the same member.  The declarator text is the
        // C fragment minus any trailing ';'.
        if let Some(recs) = records.as_mut() {
            let decl = trimmed.trim_end_matches(';').trim_end();
            let member_name = child
                .child("name")
                .and_then(|n| n.text())
                .unwrap_or("")
                .to_string();
            match TypeRef::parse(decl) {
                Ok(ty) if !member_name.is_empty() => recs.push(RawMember {
                    raw_c: decl.to_string(),
                    name: member_name,
                    ty,
                    values: child.attribute("values").map(str::to_string),
                }),
                _ => records = None,
            }
        }
    }

    let payload = records.map_or(TypePayload::Opaque, TypePayload::Members);

    if members.is_empty() {
        return (format!("typedef {} {} {{}};", kw, name), payload);
    }

    (
        format!(
            "typedef {} {} {{\n{}\n}} {};",
            kw,
            name,
            members.join("\n"),
            name
        ),
        payload,
    )
}

/// Build a `typedef RET (VKAPI_PTR *NAME)(params)` declaration — plus the
/// structured signature — from a structured `<type category="funcpointer">`
/// element that uses `<proto>` and `<param>` children (VulkanBase-era
/// format).
fn extract_funcpointer(node: roxmltree::Node<'_, '_>, name: &str) -> (String, TypePayload) {
    // Extract return type from <proto>: everything before <name>.
    let mut ret = String::new();
    if let Some(proto) = node.child("proto") {
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
    for param in node.children_named("param") {
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

    // Structured signature from the same fragments the C text embeds.
    let payload = fn_sig(ret, &params).map_or(TypePayload::Opaque, TypePayload::Funcpointer);

    (
        format!("typedef {} (VKAPI_PTR *{})({});", ret, name, params_str),
        payload,
    )
}

// ---------------------------------------------------------------------------
// Structured payload helpers
// ---------------------------------------------------------------------------

/// Structured signature from a return-type fragment and parameter fragments
/// (each `"const void* pUserData"`-shaped, carrying its name).
fn fn_sig(ret: &str, params: &[String]) -> Option<RawFnSig> {
    let ret = TypeRef::parse(ret).ok()?;
    let params = params
        .iter()
        .filter(|p| p.trim() != "void")
        .map(|p| {
            let ty = TypeRef::parse(p.trim()).ok()?;
            Some((ty.decl_name.clone().unwrap_or_default(), ty))
        })
        .collect::<Option<Vec<_>>>()?;
    Some(RawFnSig { ret, params })
}

/// Structured payload for a finished C fragment, dispatched on shape:
/// function-pointer typedefs, inline record definitions (`struct Name {...};`
/// like EGL's EGLClientPixmapHI and WGL's GPU_DEVICE, or GLX's
/// `typedef struct/union {...} Name;`), plain or preprocessor-conditional
/// typedefs, anything else `Opaque`.  The record shapes are tested before
/// the function-pointer `(` heuristic so a parenthesis inside a member
/// comment cannot misroute a record body.
fn text_payload(raw: &str) -> TypePayload {
    let t = raw.trim_start();
    let record =
        (t.starts_with("typedef struct") || t.starts_with("typedef union")) && t.contains('{');
    if record || (t.starts_with("struct") && t.contains('{')) {
        record_text_payload(raw)
    } else if t.starts_with("typedef") && t.contains('(') {
        funcpointer_text_payload(raw)
    } else if t.starts_with("typedef") || t.starts_with("#if") {
        typedef_text_payload(raw)
    } else {
        TypePayload::Opaque
    }
}

/// Parse an inline record body — `struct Name { decl; ... };` or
/// `typedef struct/union [tag] { decl; ... } Name;` — into member records
/// (the GL-family registries write these as raw text, unlike vk.xml's
/// structured `<member>` elements).  Whether the record is a struct or a
/// union stays visible in `raw_c` (`TypePayload::Members` carries only the
/// members; the Rust emitter keys union-ness off the raw text).
fn record_text_payload(raw: &str) -> TypePayload {
    let (Some(open), Some(close)) = (raw.find('{'), raw.rfind('}')) else {
        return TypePayload::Opaque;
    };
    if close < open {
        return TypePayload::Opaque;
    }
    // Strip `/* ... */` comments: GLX's record bodies annotate members with
    // trailing comments, which would otherwise glue onto the next declarator
    // when splitting on `;`.
    let body = strip_block_comments(&raw[open + 1..close]);

    let mut members: Vec<RawMember> = Vec::new();
    for decl in body.split(';') {
        let decl = decl.trim();
        if decl.is_empty() {
            continue;
        }
        // Multi-declarator lines (`int x, y;`): the first piece is a full
        // declarator; each further piece must be a bare identifier sharing
        // the first one's type (the registries never put pointers or arrays
        // on the continuation declarators).
        let mut pieces = decl.split(',');
        let first = pieces.next().unwrap_or("").trim();
        let Ok(ty) = TypeRef::parse(first) else {
            return TypePayload::Opaque;
        };
        let Some(name) = ty.decl_name.clone() else {
            return TypePayload::Opaque;
        };
        members.push(RawMember {
            raw_c: first.to_string(),
            name,
            ty: ty.clone(),
            values: None,
        });
        for extra in pieces {
            let extra = extra.trim();
            if extra.is_empty()
                || !extra.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                || extra.starts_with(|c: char| c.is_ascii_digit())
            {
                return TypePayload::Opaque;
            }
            let mut extra_ty = ty.clone();
            extra_ty.decl_name = Some(extra.to_string());
            members.push(RawMember {
                raw_c: format!(
                    "{} {extra}",
                    first.rsplit_once(' ').map_or(first, |(t, _)| t)
                ),
                name: extra.to_string(),
                ty: extra_ty,
                values: None,
            });
        }
    }
    if members.is_empty() {
        TypePayload::Opaque
    } else {
        TypePayload::Members(members)
    }
}

/// Remove every `/* ... */` region from `s` (replacing each with a single
/// space, so token boundaries survive).
fn strip_block_comments(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(start) = rest.find("/*") {
        out.push_str(&rest[..start]);
        out.push(' ');
        match rest[start + 2..].find("*/") {
            Some(end) => rest = &rest[start + 2 + end + 2..],
            None => return out, // unterminated: drop the tail
        }
    }
    out.push_str(rest);
    out
}

/// Parse an inline-text function-pointer typedef:
/// `typedef RET (CALLCONV *NAME)(PARAMS);` (calling convention optional).
fn funcpointer_text_payload(raw: &str) -> TypePayload {
    parse_funcpointer_text(raw).map_or(TypePayload::Opaque, TypePayload::Funcpointer)
}

fn parse_funcpointer_text(raw: &str) -> Option<RawFnSig> {
    let s = raw.trim().strip_prefix("typedef")?;
    let open = s.find('(')?;
    let ret = s[..open].trim();
    let rest = &s[open + 1..];
    let close = rest.find(')')?;
    // The declarator between the parens must actually be a pointer-to-
    // function name; a paren for any other reason is not this shape.
    if !rest[..close].contains('*') {
        return None;
    }
    let after = &rest[close + 1..];
    let popen = after.find('(')?;
    let pclose = after.rfind(')')?;
    if pclose <= popen {
        return None;
    }
    let params: Vec<String> = {
        let text = after[popen + 1..pclose].trim();
        if text.is_empty() {
            Vec::new()
        } else {
            text.split(',').map(|p| p.trim().to_string()).collect()
        }
    };
    fn_sig(ret, &params)
}

/// Parse one `typedef <decl> <name>;` per line, optionally split across
/// `#if`/`#ifdef`/`#else`/`#endif` arms (GLhandleARB, the macOS ptrdiff_t
/// guard).  Any line outside that shape makes the whole body `Opaque`.
fn typedef_text_payload(raw: &str) -> TypePayload {
    // Join preprocessor line continuations (the macOS guard wraps its
    // condition) before walking lines.
    let joined = raw.replace("\\\n", " ");
    let mut arms: Vec<TypedefArm> = Vec::new();
    let mut cond: Option<String> = None;
    let mut in_else = false;

    for line in joined.lines() {
        let l = line.trim();
        if l.is_empty() {
            continue;
        }
        if let Some(rest) = l.strip_prefix("#ifdef") {
            cond = Some(format!("defined({})", rest.trim()));
            in_else = false;
        } else if let Some(rest) = l.strip_prefix("#if ") {
            cond = Some(rest.trim().to_string());
            in_else = false;
        } else if l == "#else" {
            in_else = true;
        } else if l == "#endif" {
            cond = None;
            in_else = false;
        } else if let Some(body) = l.strip_prefix("typedef") {
            let Some(decl) = body.trim().strip_suffix(';') else {
                return TypePayload::Opaque;
            };
            let Ok(ty) = TypeRef::parse(decl.trim()) else {
                return TypePayload::Opaque;
            };
            if ty.decl_name.is_none() {
                return TypePayload::Opaque;
            }
            arms.push(TypedefArm {
                condition: if in_else { None } else { cond.clone() },
                ty,
            });
        } else {
            return TypePayload::Opaque;
        }
    }

    if arms.is_empty() {
        TypePayload::Opaque
    } else {
        TypePayload::Typedef(arms)
    }
}

/// Payload for `category="handle"` bodies: the `VK_DEFINE_*HANDLE` macro
/// invocations.  Alias handles (`#define X Y`) stay `Opaque` — the alias
/// field carries their target.
fn handle_payload(raw: &str) -> TypePayload {
    let t = raw.trim_start();
    if t.starts_with("VK_DEFINE_NON_DISPATCHABLE_HANDLE") {
        TypePayload::Handle {
            dispatchable: false,
        }
    } else if t.starts_with("VK_DEFINE_HANDLE") {
        TypePayload::Handle { dispatchable: true }
    } else {
        TypePayload::Opaque
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::TypePayload;

    fn parse_bundled(file: &str) -> Vec<RawType> {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("bundled/xml")
            .join(file);
        let text = std::fs::read_to_string(path).unwrap();
        let doc = roxmltree::Document::parse(&text).unwrap();
        let docs = super::super::SpecDocs {
            primary: &doc,
            supplementals: &[],
        };
        parse_types(&docs, Diag::new(true))
    }

    fn find<'a>(types: &'a [RawType], name: &str) -> &'a RawType {
        types
            .iter()
            .find(|t| t.name == name)
            .unwrap_or_else(|| panic!("type '{name}' not found"))
    }

    /// Every non-alias struct/union and every funcpointer in every bundled
    /// XML must carry a structured payload, and each structured member must
    /// agree with its XML-declared name.  The type-body counterpart of
    /// the command-declarator corpus test: a registry update whose type
    /// bodies fall outside the grammar fails here with a listing, instead
    /// of silently degrading to `Opaque` for the Rust backend to trip over.
    #[test]
    fn corpus_struct_and_funcpointer_payloads_structured() {
        let mut structs = 0usize;
        let mut funcpointers = 0usize;
        let mut failures: Vec<String> = Vec::new();

        for file in [
            "gl.xml",
            "gl_angle_ext.xml",
            "egl.xml",
            "egl_angle_ext.xml",
            "glx.xml",
            "wgl.xml",
            "vk.xml",
            "glsl_exts.xml",
        ] {
            for t in &parse_bundled(file) {
                match t.category {
                    TypeCategory::Struct | TypeCategory::Union if t.alias.is_none() => {
                        structs += 1;
                        match &t.payload {
                            TypePayload::Members(ms) => {
                                for m in ms {
                                    if m.ty.decl_name.as_deref() != Some(m.name.as_str()) {
                                        failures.push(format!(
                                            "{file}: {}.{}: declarator name {:?} != member \
                                             name (from '{}')",
                                            t.name, m.name, m.ty.decl_name, m.raw_c
                                        ));
                                    }
                                }
                            }
                            _ => failures.push(format!(
                                "{file}: struct/union '{}' has no structured members",
                                t.name
                            )),
                        }
                    }
                    TypeCategory::Funcpointer => {
                        funcpointers += 1;
                        if !matches!(t.payload, TypePayload::Funcpointer(_)) {
                            failures.push(format!(
                                "{file}: funcpointer '{}' has no structured signature",
                                t.name
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }

        assert!(
            structs > 700 && funcpointers > 10,
            "corpus suspiciously small ({structs} structs, {funcpointers} funcpointers)"
        );
        assert!(
            failures.is_empty(),
            "{} unstructured type bodies:\n{}",
            failures.len(),
            failures.join("\n")
        );
    }

    #[test]
    fn vk_payload_spot_checks() {
        let types = parse_bundled("vk.xml");

        // Dispatchable vs non-dispatchable handles.
        assert!(matches!(
            find(&types, "VkInstance").payload,
            TypePayload::Handle { dispatchable: true }
        ));
        assert!(matches!(
            find(&types, "VkBuffer").payload,
            TypePayload::Handle {
                dispatchable: false
            }
        ));

        // Old-format inline funcpointer: void* return, 4 named params.
        let TypePayload::Funcpointer(sig) = &find(&types, "PFN_vkAllocationFunction").payload
        else {
            panic!("PFN_vkAllocationFunction not structured");
        };
        assert_eq!(sig.ret.base, "void");
        assert_eq!(sig.ret.pointers.len(), 1);
        assert_eq!(sig.params.len(), 4);
        assert_eq!(sig.params[0].0, "pUserData");

        // sType member defaults arrive via values=.
        let TypePayload::Members(ms) = &find(&types, "VkInstanceCreateInfo").payload else {
            panic!("VkInstanceCreateInfo not structured");
        };
        let stype = &ms[0];
        assert_eq!(stype.name, "sType");
        assert_eq!(
            stype.values.as_deref(),
            Some("VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO")
        );

        // Bitfield members parse with widths.
        let TypePayload::Members(ms) = &find(&types, "VkAccelerationStructureInstanceKHR").payload
        else {
            panic!("VkAccelerationStructureInstanceKHR not structured");
        };
        assert_eq!(ms[1].ty.bitfield, Some(24)); // instanceCustomIndex:24
    }

    #[test]
    fn gl_family_typedef_payloads() {
        let types = parse_bundled("gl.xml");

        // GLhandleARB: the #ifdef __APPLE__ / #else pair — the exact case
        // hand-maintained base-type tables get wrong.
        let TypePayload::Typedef(arms) = &find(&types, "GLhandleARB").payload else {
            panic!("GLhandleARB not structured");
        };
        assert_eq!(arms.len(), 2);
        assert_eq!(arms[0].condition.as_deref(), Some("defined(__APPLE__)"));
        assert_eq!(arms[0].ty.base, "void");
        assert_eq!(arms[0].ty.pointers.len(), 1);
        assert_eq!(arms[1].condition, None);
        assert_eq!(arms[1].ty.base, "unsigned int");

        // GLsizeiptr: the synthesized macOS ptrdiff_t guard splits into two
        // conditional arms.
        let TypePayload::Typedef(arms) = &find(&types, "GLsizeiptr").payload else {
            panic!("GLsizeiptr not structured");
        };
        assert_eq!(arms.len(), 2);
        assert!(
            arms[0]
                .condition
                .as_deref()
                .is_some_and(|c| c.contains("__ENVIRONMENT_MAC_OS_X_VERSION_MIN_REQUIRED__"))
        );
        assert_eq!(arms[0].ty.base, "long");
        assert_eq!(arms[1].ty.base, "ptrdiff_t");

        // Plain unconditional typedef.
        let TypePayload::Typedef(arms) = &find(&types, "GLenum").payload else {
            panic!("GLenum not structured");
        };
        assert_eq!(arms.len(), 1);
        assert_eq!(arms[0].condition, None);
        assert_eq!(arms[0].ty.base, "unsigned int");

        // Opaque-struct-pointer typedef.
        let TypePayload::Typedef(arms) = &find(&types, "GLsync").payload else {
            panic!("GLsync not structured");
        };
        assert!(arms[0].ty.struct_kw);
        assert_eq!(arms[0].ty.base, "__GLsync");
        assert_eq!(arms[0].ty.pointers.len(), 1);

        // Category-less GL funcpointer typedef (text shape detection).
        let TypePayload::Funcpointer(sig) = &find(&types, "GLDEBUGPROC").payload else {
            panic!("GLDEBUGPROC not structured");
        };
        assert!(sig.ret.is_void());
        assert_eq!(sig.params.len(), 7);
        assert_eq!(sig.params.last().unwrap().0, "userParam");
    }
}
