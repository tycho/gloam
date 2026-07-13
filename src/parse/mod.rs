//! XML parsing. Takes a primary XML document plus zero or more supplemental
//! documents and produces a `RawSpec`.
//!
//! Rather than mutating the DOM, `SpecDocs` holds all documents simultaneously
//! and every collection pass iterates them in order — primary first, then
//! supplementals.  This is semantically identical to the "merge then parse"
//! approach in the original Python but avoids any DOM mutation.

pub mod commands;
pub mod enums;
pub mod features;
pub mod types;

use anyhow::{Context, Result};
use indexmap::IndexMap;

use crate::diag::Diag;
use crate::fetch::SpecSources;
use crate::identity::Spec;
use crate::ir::{RawSpec, Version};

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

pub fn parse(sources: &SpecSources, spec: Spec, diag: Diag) -> Result<RawSpec> {
    // Parse all documents up-front.  Lifetimes: each Document<'src> borrows
    // from its source String.  We keep (source, doc) pairs together so the
    // borrow is valid for the whole function body.
    let primary_doc = roxmltree::Document::parse(&sources.primary)
        .with_context(|| format!("parsing primary {} XML", spec))?;

    let supp_docs: Vec<roxmltree::Document<'_>> = sources
        .supplementals
        .iter()
        .enumerate()
        .map(|(i, s)| {
            roxmltree::Document::parse(s)
                .with_context(|| format!("parsing supplemental {} XML #{}", spec, i))
        })
        .collect::<Result<_>>()?;

    let docs = SpecDocs {
        primary: &primary_doc,
        supplementals: &supp_docs,
    };

    let platforms = parse_platforms(&docs);
    let raw_types = types::parse_types(&docs, diag);
    let (enum_groups, flat_enums) = enums::parse_enums(&docs, spec)?;
    let commands = commands::parse_commands(&docs, diag);
    let (features, extensions) =
        features::parse_features_extensions(&docs, spec, &platforms, diag)?;

    Ok(RawSpec {
        spec,
        platforms,
        types: raw_types,
        enum_groups,
        flat_enums,
        commands,
        features,
        extensions,
    })
}

// ---------------------------------------------------------------------------
// SpecDocs — multi-document view
// ---------------------------------------------------------------------------

/// Holds the primary XML document and all supplemental documents.
/// Provides iteration helpers that transparently span all documents.
pub struct SpecDocs<'a, 'input> {
    pub primary: &'a roxmltree::Document<'input>,
    pub supplementals: &'a [roxmltree::Document<'input>],
}

impl<'a, 'input> SpecDocs<'a, 'input> {
    fn all_docs(&self) -> impl Iterator<Item = &'a roxmltree::Document<'input>> {
        std::iter::once(self.primary).chain(self.supplementals.iter())
    }

    /// All direct children of `<section_tag>` top-level elements across all docs.
    pub fn section_children(&self, section_tag: &str) -> Vec<roxmltree::Node<'a, 'input>> {
        let mut nodes = Vec::new();
        for doc in self.all_docs() {
            for root_child in doc.root_element().children() {
                if root_child.tag_name().name() == section_tag {
                    nodes.extend(root_child.children().filter(|n| n.is_element()));
                }
            }
        }
        nodes
    }

    /// All `<enums>` elements that are direct children of the root (GL style:
    /// multiple `<enums>` blocks, each potentially with namespace/group attrs).
    pub fn all_enums_blocks(&self) -> Vec<roxmltree::Node<'a, 'input>> {
        let mut nodes = Vec::new();
        for doc in self.all_docs() {
            for child in doc.root_element().children() {
                if child.is_element() && child.tag_name().name() == "enums" {
                    nodes.push(child);
                }
            }
        }
        nodes
    }

    /// All `<feature>` elements (direct root children) across all docs.
    pub fn all_features(&self) -> Vec<roxmltree::Node<'a, 'input>> {
        let mut nodes = Vec::new();
        for doc in self.all_docs() {
            for child in doc.root_element().children() {
                if child.is_element() && child.tag_name().name() == "feature" {
                    nodes.push(child);
                }
            }
        }
        nodes
    }

    /// All `<platform>` elements inside any `<platforms>` block.
    pub fn all_platforms(&self) -> Vec<roxmltree::Node<'a, 'input>> {
        self.section_children("platforms")
    }

    /// All `<extension>` elements inside any `<extensions>` block.
    pub fn all_extensions(&self) -> Vec<roxmltree::Node<'a, 'input>> {
        self.section_children("extensions")
    }
}

/// Parse all `<platform>` elements into a name → protect macro map.
/// e.g. "xlib" → "VK_USE_PLATFORM_XLIB_KHR"
fn parse_platforms(docs: &SpecDocs<'_, '_>) -> IndexMap<String, String> {
    let mut map = IndexMap::new();
    for node in docs.all_platforms() {
        if node.tag_name().name() != "platform" {
            continue;
        }
        let Some(name) = node.attribute("name") else {
            continue;
        };
        let Some(protect) = node.attribute("protect") else {
            continue;
        };
        map.insert(name.to_string(), protect.to_string());
    }
    map
}

// ---------------------------------------------------------------------------
// Shared XML helpers
// ---------------------------------------------------------------------------

/// Recursively extract C text from a `<type>` element, handling:
///   - plain text nodes — included as-is
///   - `<apientry/>` — replaced with "APIENTRY"
///   - `<name>`, `<type>`, `<ptype>` sub-elements — their text is inlined
///   - `//` line comments — rewritten to `/* */` (for C99 compat)
pub fn extract_raw_c(node: roxmltree::Node<'_, '_>) -> String {
    let raw = extract_raw_c_inner(node);
    // Rewrite C++ line comments to C block comments (spec gotcha #6).
    rewrite_line_comments(&raw)
}

fn extract_raw_c_inner(node: roxmltree::Node<'_, '_>) -> String {
    let mut out = String::new();
    for child in node.children() {
        if child.is_text() {
            out.push_str(child.text().unwrap_or(""));
        } else if child.is_element() {
            match child.tag_name().name() {
                "apientry" => out.push_str("APIENTRY"),
                "comment" => {} // skip XML comments embedded in type defs
                _ => out.push_str(&extract_raw_c_inner(child)),
            }
        }
    }
    out
}

/// Rewrite C++ `// comment` style to `/* comment */` (spec gotcha #6).
///
/// Two guards keep this from corrupting valid C:
///   - text inside an existing `/* ... */` block passes through untouched
///     (a URL like `https://...` in a block comment would otherwise gain an
///     injected `*/` and leave the original terminator dangling);
///   - `//` immediately preceded by `:` is treated as part of a URL, not a
///     comment.
pub fn rewrite_line_comments(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    let mut prev: Option<char> = None;
    while let Some(c) = chars.next() {
        // Copy an existing block comment verbatim.
        if c == '/' && chars.peek() == Some(&'*') {
            out.push(c);
            out.push(chars.next().unwrap()); // '*'
            let mut star = false;
            for c2 in chars.by_ref() {
                out.push(c2);
                if star && c2 == '/' {
                    break;
                }
                star = c2 == '*';
            }
            prev = Some('/');
            continue;
        }

        if c == '/' && chars.peek() == Some(&'/') && prev != Some(':') {
            chars.next(); // consume second '/'
            let mut comment = String::new();
            for c2 in chars.by_ref() {
                if c2 == '\n' {
                    break;
                }
                comment.push(c2);
            }
            out.push_str("/* ");
            out.push_str(comment.trim());
            out.push_str(" */\n");
            prev = Some('\n');
            continue;
        }

        out.push(c);
        prev = Some(c);
    }
    out
}

/// Parse a version string "major.minor" into a `Version`.
pub fn parse_version(s: &str) -> Result<Version> {
    let (maj, min) = s
        .split_once('.')
        .ok_or_else(|| anyhow::anyhow!("invalid version '{}', expected major.minor", s))?;
    Ok(Version::new(maj.trim().parse()?, min.trim().parse()?))
}

/// Compute the value of an enum that uses the extension offset formula:
///   1_000_000_000 + 1_000 * (extnumber - 1) + offset
/// Negated if `dir="-"`.
pub fn compute_ext_enum_value(extnumber: u32, offset: u32, dir: Option<&str>) -> i64 {
    let base: i64 = 1_000_000_000 + 1_000 * (extnumber as i64 - 1) + offset as i64;
    if dir == Some("-") { -base } else { base }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- rewrite_line_comments ----

    #[test]
    fn line_comment_becomes_block_comment() {
        assert_eq!(
            rewrite_line_comments("int x; // hello\nint y;"),
            "int x; /* hello */\nint y;"
        );
    }

    #[test]
    fn line_comment_at_eof_without_newline() {
        assert_eq!(rewrite_line_comments("// end"), "/* end */\n");
    }

    #[test]
    fn url_in_block_comment_is_untouched() {
        let s = "/* see https://registry.khronos.org for details */\nint x;";
        assert_eq!(rewrite_line_comments(s), s);
    }

    #[test]
    fn double_slash_in_block_comment_is_untouched() {
        let s = "/* a // b */\nint x;";
        assert_eq!(rewrite_line_comments(s), s);
    }

    #[test]
    fn url_in_plain_text_is_untouched() {
        let s = "#define SPEC_URL \"https://example.com/path\"";
        assert_eq!(rewrite_line_comments(s), s);
    }

    #[test]
    fn text_without_comments_is_identity() {
        let s = "typedef unsigned int GLenum;\n";
        assert_eq!(rewrite_line_comments(s), s);
    }

    // ---- parse_version ----

    #[test]
    fn parse_version_accepts_major_minor() {
        assert_eq!(parse_version("3.3").unwrap(), Version::new(3, 3));
    }

    #[test]
    fn parse_version_rejects_missing_minor() {
        assert!(parse_version("3").is_err());
    }

    // ---- compute_ext_enum_value ----

    #[test]
    fn ext_enum_offset_formula() {
        // 1_000_000_000 + 1_000 * (extnumber - 1) + offset
        assert_eq!(compute_ext_enum_value(1, 0, None), 1_000_000_000);
        assert_eq!(compute_ext_enum_value(2, 3, None), 1_000_001_003);
        assert_eq!(compute_ext_enum_value(2, 3, Some("-")), -1_000_001_003);
    }
}
