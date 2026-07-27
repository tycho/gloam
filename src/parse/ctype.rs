//! Typed C declarator parsing — `TypeRef`, the structured view of a
//! parameter, return, or struct-member type.
//!
//! The registries express types as fragments of C declarator syntax
//! (`const GLchar *const*`, `float blendConstants[4]`,
//! `uint32_t instanceCustomIndex:24`).  The grammar those fragments draw
//! from is closed and small, so this module parses it exactly and refuses
//! anything outside it: a construct this parser has never seen fails
//! generation loudly instead of silently passing through as text a
//! non-C backend would then mistranslate.
//!
//! Grammar (no function pointers — the registries only ever reference those
//! through named typedefs, and funcpointer *definitions* are handled by the
//! structured `<type>` parsing, not here):
//!
//! ```text
//! declarator := {qualifier} base {qualifier} {pointer} [name] {array} [bitfield]
//! qualifier  := "const" | "struct"
//! base       := builtin-word+ | identifier
//! pointer    := "*" ["const"]
//! array      := "[" (integer | identifier) "]"
//! bitfield   := ":" integer
//! ```
//!
//! `builtin-word` covers the multi-word C scalar spellings (`unsigned int`,
//! `unsigned long`, ...).  A `const` before any `*` binds to the base; a
//! `const` after a `*` binds to that pointer level, exactly as in C.

use anyhow::{Result, bail};

/// C scalar keywords that may combine into a multi-word base type.
const BUILTIN_WORDS: &[&str] = &[
    "void", "char", "short", "int", "long", "float", "double", "signed", "unsigned",
];

/// A parsed C declarator.  Language-neutral: printers (C, Rust) decide how
/// each piece is spelled in their target.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TypeRef {
    /// Whitespace-normalized base type name: `"GLenum"`, `"unsigned int"`,
    /// `"void"`.
    pub base: String,
    /// The base (the deepest pointee) is const-qualified.
    pub base_const: bool,
    /// The base was written with the `struct` keyword (`struct wl_display`).
    pub struct_kw: bool,
    /// Pointer levels, innermost (adjacent to the base) first.  `true` means
    /// that pointer itself is const-qualified (`* const`).
    pub pointers: Vec<bool>,
    /// Array dimensions as written (integer literal or identifier constant),
    /// outermost first.
    pub array: Vec<String>,
    /// Bitfield width (struct members only).
    pub bitfield: Option<u32>,
    /// Declarator name embedded in the fragment, when present (array
    /// parameters and struct members carry one; plain parameter types do
    /// not).
    pub decl_name: Option<String>,
}

impl TypeRef {
    /// Parse a C declarator fragment.  Errors on anything outside the
    /// registry grammar, quoting the offending input.
    pub fn parse(input: &str) -> Result<TypeRef> {
        let mut ty = TypeRef::default();
        let mut lex = Lexer::new(input);

        while let Some(tok) = lex.next()? {
            match tok {
                Token::Ident("const") => {
                    if let Some(level) = ty.pointers.last_mut() {
                        *level = true;
                    } else {
                        ty.base_const = true;
                    }
                }
                Token::Ident("struct") => {
                    if !ty.base.is_empty() {
                        bail!("'struct' after base type in C declarator '{input}'");
                    }
                    ty.struct_kw = true;
                }
                Token::Ident(word) => {
                    if ty.base.is_empty() {
                        // First identifier: the base.  Builtin scalar words
                        // may chain (`unsigned int`); anything else is a
                        // one-word base name.
                        ty.base.push_str(word);
                        if BUILTIN_WORDS.contains(&word) {
                            while let Some(next) = lex.peek_builtin()? {
                                ty.base.push(' ');
                                ty.base.push_str(next);
                            }
                        }
                    } else if ty.decl_name.is_none() {
                        ty.decl_name = Some(word.to_string());
                    } else {
                        bail!(
                            "unexpected identifier '{word}' after declarator name \
                             in C declarator '{input}'"
                        );
                    }
                }
                Token::Star => {
                    if ty.base.is_empty() {
                        bail!("'*' before base type in C declarator '{input}'");
                    }
                    if ty.decl_name.is_some() || !ty.array.is_empty() {
                        bail!("'*' after declarator name in C declarator '{input}'");
                    }
                    ty.pointers.push(false);
                }
                Token::ArrayDim(dim) => {
                    if ty.base.is_empty() {
                        bail!("array dimension before base type in C declarator '{input}'");
                    }
                    ty.array.push(dim.to_string());
                }
                Token::Bitfield(width) => {
                    if ty.base.is_empty() || ty.bitfield.is_some() {
                        bail!("misplaced bitfield width in C declarator '{input}'");
                    }
                    ty.bitfield = Some(width);
                }
            }
        }

        if ty.base.is_empty() {
            bail!("no base type in C declarator '{input}'");
        }
        Ok(ty)
    }

    /// True for a plain `void` (no pointers, no array): the C spelling of
    /// "no value".
    pub fn is_void(&self) -> bool {
        self.base == "void" && self.pointers.is_empty() && self.array.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Lexer
// ---------------------------------------------------------------------------

#[derive(Debug)]
enum Token<'a> {
    Ident(&'a str),
    Star,
    /// `[dim]`, dimension text (integer or identifier constant).
    ArrayDim(&'a str),
    /// `:width`.
    Bitfield(u32),
}

struct Lexer<'a> {
    input: &'a str,
    rest: &'a str,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer { input, rest: input }
    }

    fn next(&mut self) -> Result<Option<Token<'a>>> {
        self.rest = self.rest.trim_start();
        let Some(c) = self.rest.chars().next() else {
            return Ok(None);
        };
        match c {
            '*' => {
                self.rest = &self.rest[1..];
                Ok(Some(Token::Star))
            }
            '[' => {
                let Some(close) = self.rest.find(']') else {
                    bail!("unterminated '[' in C declarator '{}'", self.input);
                };
                let dim = self.rest[1..close].trim();
                if dim.is_empty() || !dim.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                    bail!(
                        "array dimension '{dim}' is not an integer or identifier \
                         in C declarator '{}'",
                        self.input
                    );
                }
                self.rest = &self.rest[close + 1..];
                Ok(Some(Token::ArrayDim(dim)))
            }
            ':' => {
                self.rest = self.rest[1..].trim_start();
                let end = self
                    .rest
                    .find(|c: char| !c.is_ascii_digit())
                    .unwrap_or(self.rest.len());
                let digits = &self.rest[..end];
                let Ok(width) = digits.parse::<u32>() else {
                    bail!(
                        "bitfield width '{digits}' is not an integer in C declarator '{}'",
                        self.input
                    );
                };
                self.rest = &self.rest[end..];
                Ok(Some(Token::Bitfield(width)))
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                let end = self
                    .rest
                    .find(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                    .unwrap_or(self.rest.len());
                let (word, rest) = self.rest.split_at(end);
                self.rest = rest;
                Ok(Some(Token::Ident(word)))
            }
            other => bail!(
                "unexpected character '{other}' in C declarator '{}'",
                self.input
            ),
        }
    }

    /// Consume and return the next token only if it is a builtin scalar word
    /// (used to assemble multi-word bases without lookahead buffering).
    fn peek_builtin(&mut self) -> Result<Option<&'a str>> {
        let saved = self.rest;
        match self.next()? {
            Some(Token::Ident(word)) if BUILTIN_WORDS.contains(&word) => Ok(Some(word)),
            _ => {
                self.rest = saved;
                Ok(None)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(s: &str) -> TypeRef {
        TypeRef::parse(s).unwrap()
    }

    #[test]
    fn plain_base() {
        let t = parse("GLenum");
        assert_eq!(t.base, "GLenum");
        assert!(!t.base_const && t.pointers.is_empty() && t.array.is_empty());
        assert_eq!(t.decl_name, None);
    }

    #[test]
    fn void_detection() {
        assert!(parse("void").is_void());
        assert!(!parse("void *").is_void());
        assert!(!parse("GLenum").is_void());
    }

    #[test]
    fn multi_word_builtin_base() {
        assert_eq!(parse("unsigned int").base, "unsigned int");
        assert_eq!(parse("unsigned long").base, "unsigned long");
        // A non-builtin word after a builtin base is a declarator name,
        // not part of the base.
        let t = parse("unsigned int mask");
        assert_eq!(t.base, "unsigned int");
        assert_eq!(t.decl_name.as_deref(), Some("mask"));
    }

    #[test]
    fn const_binds_to_base_before_any_pointer() {
        for s in ["const GLubyte *", "GLubyte const *"] {
            let t = parse(s);
            assert!(t.base_const, "{s}");
            assert_eq!(t.pointers, vec![false], "{s}");
        }
    }

    #[test]
    fn const_after_star_binds_to_that_pointer() {
        // const GLchar *const*: const pointer (level 0) to const GLchar,
        // under a mutable outer pointer (level 1).
        let t = parse("const GLchar *const*");
        assert!(t.base_const);
        assert_eq!(t.pointers, vec![true, false]);
    }

    #[test]
    fn double_pointer_inner_const_only() {
        // const GLcharARB **: pointer to pointer-to-const — the outer
        // pointer is mutable.  (The old string-munging translator got
        // this wrong by applying one const to every level.)
        let t = parse("const GLcharARB **");
        assert!(t.base_const);
        assert_eq!(t.pointers, vec![false, false]);
    }

    #[test]
    fn struct_keyword() {
        let t = parse("struct AHardwareBuffer*");
        assert!(t.struct_kw);
        assert_eq!(t.base, "AHardwareBuffer");
        assert_eq!(t.pointers, vec![false]);
        let t = parse("const struct AHardwareBuffer *");
        assert!(t.struct_kw && t.base_const);
    }

    #[test]
    fn array_param_with_embedded_name() {
        let t = parse("float blendConstants[4]");
        assert_eq!(t.base, "float");
        assert_eq!(t.decl_name.as_deref(), Some("blendConstants"));
        assert_eq!(t.array, vec!["4"]);
    }

    #[test]
    fn array_dims_identifier_and_multi() {
        let t = parse("uint8_t deviceUUID[VK_UUID_SIZE]");
        assert_eq!(t.array, vec!["VK_UUID_SIZE"]);
        let t = parse("float matrix[3][4]");
        assert_eq!(t.array, vec!["3", "4"]);
    }

    #[test]
    fn bitfield_member() {
        let t = parse("uint32_t instanceCustomIndex:24");
        assert_eq!(t.base, "uint32_t");
        assert_eq!(t.decl_name.as_deref(), Some("instanceCustomIndex"));
        assert_eq!(t.bitfield, Some(24));
        let t = parse("VkGeometryInstanceFlagsKHR flags : 8");
        assert_eq!(t.bitfield, Some(8));
    }

    #[test]
    fn no_space_before_star() {
        let t = parse("const VkDeviceCreateInfo*");
        assert!(t.base_const);
        assert_eq!(t.base, "VkDeviceCreateInfo");
        assert_eq!(t.pointers, vec![false]);
    }

    #[test]
    fn rejects_outside_grammar() {
        assert!(TypeRef::parse("").is_err());
        assert!(TypeRef::parse("*").is_err());
        assert!(TypeRef::parse("const").is_err());
        assert!(TypeRef::parse("int (*fp)(void)").is_err()); // funcpointers: not this grammar
        assert!(TypeRef::parse("int a b").is_err());
        assert!(TypeRef::parse("int x[3").is_err());
        assert!(TypeRef::parse("int x:y").is_err());
        assert!(TypeRef::parse("int &r").is_err());
        assert!(TypeRef::parse("GLenum struct x").is_err());
    }

    #[test]
    fn pointer_position_constraints() {
        assert!(TypeRef::parse("float blendConstants[4] *").is_err());
        assert!(TypeRef::parse("float name *").is_err());
    }

    /// Every parameter and return type of every command in every bundled
    /// spec XML must parse.  This is the semantic-awareness guarantee: a
    /// registry update that introduces a construct outside the grammar
    /// fails here with a listing of the offending declarators, instead of
    /// flowing through as opaque text for a non-C backend to mistranslate.
    #[test]
    fn corpus_every_bundled_command_type_parses() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("bundled/xml");
        let mut checked = 0usize;
        let mut failures: Vec<String> = Vec::new();

        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|e| e.to_str()) != Some("xml") {
                continue;
            }
            let file = path.file_name().unwrap().to_string_lossy().into_owned();
            let text = std::fs::read_to_string(&path).unwrap();
            let doc = roxmltree::Document::parse(&text).unwrap();
            let docs = crate::parse::SpecDocs {
                primary: &doc,
                supplementals: &[],
            };
            let commands =
                crate::parse::commands::parse_commands(&docs, crate::diag::Diag::new(true));

            let mut check = |what: &str, raw: &str| {
                checked += 1;
                if let Err(e) = TypeRef::parse(raw) {
                    failures.push(format!("{file}: {what}: {e:#}"));
                }
            };
            for cmd in commands.values() {
                // Alias-only commands with unresolvable chains keep an empty
                // return type; the resolver rejects them if ever selected.
                if cmd.return_type.is_empty() {
                    continue;
                }
                check(&format!("{} return", cmd.name), &cmd.return_type);
                for p in &cmd.params {
                    check(&format!("{} param '{}'", cmd.name, p.name), &p.type_raw);
                }
            }
        }

        assert!(
            checked > 10_000,
            "corpus suspiciously small ({checked} declarators) — did bundled/xml move?"
        );
        assert!(
            failures.is_empty(),
            "{} unparseable declarators:\n{}",
            failures.len(),
            failures.join("\n")
        );
    }
}
