//! Rust loader generator (experimental; GL/GLES only).
//!
//! Emits a self-contained crate (`Cargo.toml` + `src/lib.rs`): GL base-type
//! aliases, `GLenum`/`GLbitfield` newtypes, the flat enum constants, a packed
//! function-name blob, per-API `load_<api>` functions (version detection,
//! feature/extension-gated loading, alias resolution), extension detection via
//! an XXH3 hash table (reusing the resolver's `Extension.hash`, so the Rust and
//! C loaders detect identically), an inline `[*const c_void; K]` context, an
//! `#![no_std]` crate root (everything is `core` + the caller's loader), and an
//! `#[inline]` dispatch method per command plus feature/extension presence
//! queries.  Hand-formatted tables carry `#[rustfmt::skip]` so a downstream
//! `cargo fmt` leaves their alignment intact.  Unsafe operations sit in
//! explicit `unsafe {}` blocks (enforced by `#![deny(unsafe_op_in_unsafe_fn)]`),
//! so the output is valid under any edition; the pinned `edition = 2021` is
//! just a stable default, not load-bearing.
//!
//! See `docs/rust-backend.md` for the design.  The one remaining optional mode
//! is mx-global dispatch via a `OnceLock` global.

use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Context, Result, bail};
use indexmap::IndexMap;

use super::GeneratedTree;
use crate::identity::Spec;
use crate::preamble;
use crate::provenance::load::SourceStore;
use crate::provenance::manifest::{OutputEntry, ProvenancePin, git_blob_sha1};
use crate::resolve::{Command, FeatureSet};

/// GL base-type aliases, covering the whole GL/GLES command surface.  The
/// translator maps each C base type to one of these names (or a `GLenum`/
/// `GLbitfield` newtype), so pointer/const wrapping is all it has to compute.
/// Item order is irrelevant in Rust, so the callback aliases may reference the
/// `GLenum` newtype defined later.
const BASE_TYPES: &str = "\
pub type GLvoid = c_void;
pub type GLboolean = u8;
pub type GLbyte = i8;
pub type GLubyte = u8;
pub type GLchar = c_char;
pub type GLcharARB = c_char;
pub type GLshort = i16;
pub type GLushort = u16;
pub type GLint = i32;
pub type GLuint = u32;
pub type GLfixed = i32;
pub type GLint64 = i64;
pub type GLuint64 = u64;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;
pub type GLintptr = isize;
pub type GLsizeiptr = isize;
pub type GLintptrARB = isize;
pub type GLsizeiptrARB = isize;
pub type GLsizei = i32;
pub type GLfloat = f32;
pub type GLclampf = f32;
pub type GLdouble = f64;
pub type GLclampd = f64;
pub type GLhalf = u16;
pub type GLhalfARB = u16;
pub type GLhalfNV = u16;
pub type GLhandleARB = u32;
pub type GLvdpauSurfaceNV = GLintptr;

// Opaque handle types.  `GLsync` is a pointer to an incomplete struct in C,
// so it gets its own zero-sized opaque type to keep the pointer distinct
// (a GLsync is not interchangeable with any other pointer, as in C).
// `GLeglImageOES`/`GLeglClientBufferEXT` are literally `void *` in the spec.
#[repr(C)]
pub struct __GLsync {
    _opaque: [u8; 0],
}
pub type GLsync = *mut __GLsync;
pub type GLeglImageOES = *mut c_void;
pub type GLeglClientBufferEXT = *mut c_void;
#[repr(C)]
pub struct _cl_context {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct _cl_event {
    _opaque: [u8; 0],
}

// Callback function-pointer types.
pub type GLDEBUGPROC =
    Option<unsafe extern \"system\" fn(GLenum, GLenum, GLuint, GLenum, GLsizei, *const GLchar, *const c_void)>;
pub type GLDEBUGPROCARB = GLDEBUGPROC;
pub type GLDEBUGPROCKHR = GLDEBUGPROC;
pub type GLDEBUGPROCAMD =
    Option<unsafe extern \"system\" fn(GLuint, GLenum, GLenum, GLsizei, *const GLchar, *mut c_void)>;
pub type GLVULKANPROCNV = Option<unsafe extern \"system\" fn()>;
pub type GLSETBLOBPROCANGLE =
    Option<unsafe extern \"system\" fn(*const c_void, GLsizeiptr, *const c_void, GLsizeiptr, *const c_void)>;
pub type GLGETBLOBPROCANGLE = Option<
    unsafe extern \"system\" fn(*const c_void, GLsizeiptr, *mut c_void, GLsizeiptr, *const c_void) -> GLsizeiptr,
>;
";

/// The two GL scalar-enum newtypes.  `#[repr(transparent)]` keeps them
/// ABI-identical to `u32`, so they pass through `extern \"system\"` fn pointers
/// exactly like the C `GLenum`/`GLbitfield`.
const NEWTYPES: &str = "\
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct GLenum(pub u32);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct GLbitfield(pub u32);

impl core::ops::BitOr for GLbitfield {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        GLbitfield(self.0 | rhs.0)
    }
}

impl core::ops::BitAnd for GLbitfield {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        GLbitfield(self.0 & rhs.0)
    }
}
";

pub fn generate(
    fs: &FeatureSet,
    args: &crate::cli::RustArgs,
    out: &Path,
    store: &SourceStore,
    command_line: &str,
) -> Result<GeneratedTree> {
    if fs.spec != Spec::Gl {
        bail!(
            "the rust backend currently supports only the GL/GLES API; \
             {} is not implemented yet",
            fs.display_name
        );
    }

    let stem = output_stem(fs);

    // Provenance pins for every contributing source (same as the C backend).
    let source_key_refs: Vec<&str> = fs.source_keys.iter().map(String::as_str).collect();
    let pins: IndexMap<String, ProvenancePin> = store
        .resolve(&source_key_refs)
        .context("resolving source provenance")?
        .into_iter()
        .map(|(key, src)| (key, src.pin))
        .collect();

    let preamble = rustify_preamble(&preamble::build_preamble(fs, &pins, command_line));
    let body = normalize_eof(emit_module(fs, &preamble, args.mx_global)?);

    // Emit a self-contained crate: Cargo.toml (pins the edition + the
    // xxhash-rust dependency) plus src/lib.rs (the loader).
    let crate_name = format!("gloam_{stem}");
    let cargo_toml = emit_cargo_toml(&crate_name);
    let rustfmt_toml = emit_rustfmt_toml();
    let src_dir = out.join("src");
    std::fs::create_dir_all(&src_dir)?;
    std::fs::write(out.join("Cargo.toml"), &cargo_toml)?;
    std::fs::write(out.join("rustfmt.toml"), &rustfmt_toml)?;
    std::fs::write(src_dir.join("lib.rs"), &body)?;

    Ok(GeneratedTree {
        pins,
        files: vec![
            OutputEntry {
                path: "Cargo.toml".to_string(),
                blob: git_blob_sha1(cargo_toml.as_bytes()),
                verbatim: false,
                derived_from: Vec::new(),
            },
            OutputEntry {
                path: "src/lib.rs".to_string(),
                blob: git_blob_sha1(body.as_bytes()),
                verbatim: false,
                derived_from: fs.source_keys.clone(),
            },
        ],
    })
}

/// Emit the generated loader crate's `Cargo.toml`.  The pinned `edition = 2021`
/// is a conservative default (the code uses explicit `unsafe {}` blocks, so any
/// edition works); the crate encapsulates the `xxhash-rust` dependency it uses
/// for extension-name hashing.
fn emit_cargo_toml(crate_name: &str) -> String {
    format!(
        "# @generated by gloam.\n\
         [package]\n\
         name = \"{crate_name}\"\n\
         version = \"0.0.0\"\n\
         edition = \"2021\"\n\
         # c\"...\" literals need 1.77; core::error::Error needs 1.81.\n\
         rust-version = \"1.81\"\n\
         \n\
         [features]\n\
         # Analogue of a KHR_no_error GL context, applied at compile time:\n\
         # removes the per-call is-loaded check, so calling an unloaded\n\
         # function is undefined behavior (as in C) instead of a panic.\n\
         no-error = []\n\
         \n\
         [dependencies]\n\
         xxhash-rust = {{ version = \"0.8\", features = [\"xxh3\"] }}\n"
    )
}

/// Emit the generated loader crate's `rustfmt.toml`. This file directs rustfmt
/// to not re-format the generated lib.rs code.
fn emit_rustfmt_toml() -> String {
    format!(
        "# @generated by gloam.\n\
         format_generated_files = false\n"
    )
}

/// Merged builds use the spec name (`gl`); non-merged use the single API name.
fn output_stem(fs: &FeatureSet) -> String {
    if fs.is_merged {
        fs.spec_name.clone()
    } else {
        fs.apis
            .first()
            .cloned()
            .unwrap_or_else(|| fs.spec_name.clone())
    }
}

/// Turn the C block-comment preamble (`/* ... */` with ` * ` line prefixes)
/// into Rust line comments.  Stopgap: a cleaner refactor would have
/// `build_preamble` yield raw lines and format per language.
fn rustify_preamble(c_preamble: &str) -> String {
    let mut out = String::new();
    for line in c_preamble.lines() {
        let t = line.trim_end();
        if t == "/*" || t == " */" {
            continue;
        }
        let rest = t.strip_prefix(" *").unwrap_or(t);
        let _ = writeln!(out, "//{rest}");
    }
    out
}

/// Build the whole module body (everything after the preamble).
fn emit_module(fs: &FeatureSet, preamble: &str, mx_global: bool) -> Result<String> {
    let mut s = String::with_capacity(256 * 1024);

    s.push_str(preamble);
    s.push('\n');
    s.push_str(
        "#![no_std]\n\
         #![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, clippy::all)]\n\
         #![deny(unsafe_op_in_unsafe_fn)]\n\n\
         use core::ffi::{CStr, c_char, c_void};\n\n",
    );

    s.push_str("// ── GL base types ───────────────────────────────────────────\n");
    s.push_str(BASE_TYPES);
    s.push('\n');
    s.push_str("// ── Enum newtypes ───────────────────────────────────────────\n");
    s.push_str(NEWTYPES);
    s.push('\n');

    emit_constants(fs, &mut s);
    emit_command_tables(fs, &mut s);
    emit_extension_tables(fs, &mut s)?;
    emit_missing_helpers(&mut s);
    emit_version_helpers(&mut s);
    emit_context(fs, &mut s);
    if mx_global {
        emit_global(fs, &mut s);
    }

    Ok(s)
}

/// Emit the two `__missing` variants behind the `no-error` cargo feature: the
/// default panics by name (looked up in the existing name blob — no new data),
/// the `no-error` build collapses the dispatch match back to an unchecked call.
fn emit_missing_helpers(s: &mut String) {
    s.push_str(
        "// ── Unloaded-call handling ──────────────────────────────────\n\
         /// Reached when a dispatch wrapper finds a null PFN: the function is not\n\
         /// loaded (feature/extension absent, or the context was never loaded).\n\
         /// Panics with the function's name from the name blob.\n\
         #[cfg(not(feature = \"no-error\"))]\n\
         #[cold]\n\
         #[inline(never)]\n\
         unsafe fn __missing(idx: usize) -> ! {\n\
         \x20   let off = FN_NAME_OFFSETS[idx] as usize;\n\
         \x20   let name = CStr::from_bytes_until_nul(&FN_NAME_DATA[off..]).unwrap_or(c\"?\");\n\
         \x20   panic!(\n\
         \x20       \"{} is not loaded (unsupported by this context, or called before load)\",\n\
         \x20       name.to_str().unwrap_or(\"?\")\n\
         \x20   )\n\
         }\n\n\
         /// `no-error` build (the KHR_no_error analogue): promise the compiler the\n\
         /// null case is impossible, so the dispatch match compiles to an\n\
         /// unchecked call.  Calling an unloaded function is undefined behavior,\n\
         /// exactly as in C.\n\
         #[cfg(feature = \"no-error\")]\n\
         #[inline(always)]\n\
         unsafe fn __missing(_idx: usize) -> ! {\n\
         \x20   debug_assert!(false, \"unloaded GL function called in a no-error build\");\n\
         \x20   unsafe { core::hint::unreachable_unchecked() }\n\
         }\n\n",
    );
}

/// Emit the module-level GL_VERSION parsing helpers used by each `load_<api>`.
fn emit_version_helpers(s: &mut String) {
    s.push_str(
        "// ── Version parsing ─────────────────────────────────────────\n\
         /// True if the NUL-terminated string at `p` begins with `pre`.\n\
         unsafe fn __starts_with(p: *const GLubyte, pre: &[u8]) -> bool {\n\
         \x20   let mut i = 0;\n\
         \x20   while i < pre.len() {\n\
         \x20       if unsafe { *p.add(i) } != pre[i] {\n\
         \x20           return false;\n\
         \x20       }\n\
         \x20       i += 1;\n\
         \x20   }\n\
         \x20   true\n\
         }\n\n\
         /// Parse a GL_VERSION string (e.g. `\"3.3.0 ...\"` or `\"OpenGL ES 3.0\"`)\n\
         /// into a packed `major << 8 | minor`, or 0 if unparseable.\n\
         unsafe fn __parse_gl_version(mut p: *const GLubyte) -> u32 {\n\
         \x20   if p.is_null() {\n\
         \x20       return 0;\n\
         \x20   }\n\
         \x20   const PREFIXES: [&[u8]; 5] = [\n\
         \x20       b\"OpenGL ES-CM \",\n\
         \x20       b\"OpenGL ES-CL \",\n\
         \x20       b\"OpenGL ES \",\n\
         \x20       b\"OpenGL SC \",\n\
         \x20       b\"OpenGL \",\n\
         \x20   ];\n\
         \x20   unsafe {\n\
         \x20       let mut k = 0;\n\
         \x20       while k < PREFIXES.len() {\n\
         \x20           if __starts_with(p, PREFIXES[k]) {\n\
         \x20               p = p.add(PREFIXES[k].len());\n\
         \x20               break;\n\
         \x20           }\n\
         \x20           k += 1;\n\
         \x20       }\n\
         \x20       let mut major: u32 = 0;\n\
         \x20       while (*p).is_ascii_digit() {\n\
         \x20           major = major * 10 + (*p - b'0') as u32;\n\
         \x20           p = p.add(1);\n\
         \x20       }\n\
         \x20       let mut minor: u32 = 0;\n\
         \x20       if *p == b'.' {\n\
         \x20           p = p.add(1);\n\
         \x20           while (*p).is_ascii_digit() {\n\
         \x20               minor = minor * 10 + (*p - b'0') as u32;\n\
         \x20               p = p.add(1);\n\
         \x20           }\n\
         \x20       }\n\
         \x20       (major << 8) | minor\n\
         \x20   }\n\
         }\n\n",
    );
}

/// Parse a pre-baked hash literal (`"0x…"`, as the resolver stores in
/// `Extension.hash`) into a `u64`.
fn parse_hash(literal: &str) -> u64 {
    let t = literal.trim();
    let hex = t
        .strip_prefix("0x")
        .or_else(|| t.strip_prefix("0X"))
        .unwrap_or(t);
    let hex: String = hex.chars().take_while(|c| c.is_ascii_hexdigit()).collect();
    u64::from_str_radix(&hex, 16).unwrap_or(0)
}

/// Emit the extension count, the pre-baked `(hash, extArray-index)` table sorted
/// by hash for binary search, and the runtime `__ext_hash`.  Reuses the XXH3-64
/// hashes the resolver already computed (`Extension.hash`), so the Rust and C
/// loaders detect identically.  Errors on a collision (a correctness guard).
fn emit_extension_tables(fs: &FeatureSet, s: &mut String) -> Result<()> {
    let m = fs.extensions.len();
    let _ = writeln!(
        s,
        "// ── Extensions ──────────────────────────────────────────────\n\
         pub const EXT_COUNT: usize = {m};\n"
    );

    let mut table: Vec<(u64, u16)> = fs
        .extensions
        .iter()
        .map(|e| (parse_hash(&e.hash), e.index))
        .collect();
    table.sort_by_key(|&(h, _)| h);
    for w in table.windows(2) {
        if w[0].0 == w[1].0 {
            bail!(
                "XXH3-64 collision between extensions (extArray indices {} and {})",
                w[0].1,
                w[1].1
            );
        }
    }

    // Parallel arrays rather than `[(u64, u16); N]`: a tuple would pad each
    // entry to 16 bytes (6 wasted), which for ~1000 extensions is several KiB.
    // Parallel arrays (not `[(u64, u16); N]`, which would pad each entry to 16
    // bytes): the sorted hashes, and the extArray index each hash belongs to.
    s.push_str("// XXH3-64 of each extension name, sorted for binary search.\n");
    s.push_str("#[rustfmt::skip]\n");
    let _ = writeln!(s, "static EXT_HASH_KEYS: [u64; EXT_COUNT] = [");
    for (h, idx) in &table {
        let name = fs
            .extensions
            .get(*idx as usize)
            .map_or("", |e| e.name.as_str());
        let _ = writeln!(s, "    0x{h:016x}, // {name}");
    }
    s.push_str("];\n");
    s.push_str("// extArray index for the correspondingly-ranked EXT_HASH_KEYS entry.\n");
    s.push_str("#[rustfmt::skip]\n");
    let _ = writeln!(s, "static EXT_HASH_IDX: [u16; EXT_COUNT] = [");
    for chunk in table.chunks(20) {
        let line: Vec<String> = chunk.iter().map(|(_, idx)| idx.to_string()).collect();
        let _ = writeln!(s, "    {},", line.join(", "));
    }
    s.push_str("];\n\n");

    s.push_str(
        "/// XXH3-64 of a NUL-terminated driver extension string — the same hash\n\
         /// gloam pre-baked into EXT_HASH_KEYS, so driver names match the table.\n\
         #[inline]\n\
         unsafe fn __ext_hash(p: *const GLubyte) -> u64 {\n\
         \x20   unsafe {\n\
         \x20       let mut len = 0usize;\n\
         \x20       while *p.add(len) != 0 {\n\
         \x20           len += 1;\n\
         \x20       }\n\
         \x20       xxhash_rust::xxh3::xxh3_64(core::slice::from_raw_parts(p, len))\n\
         \x20   }\n\
         }\n\n",
    );
    Ok(())
}

/// Emit the flat `#define`-style constants as free consts typed by the newtype,
/// so `use gl::*;` gives bare `GL_TRIANGLES` with call-site type safety.
fn emit_constants(fs: &FeatureSet, s: &mut String) {
    s.push_str("// ── Constants ───────────────────────────────────────────────\n");
    let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for e in &fs.flat_enums {
        if !seen.insert(e.name.as_str()) {
            continue; // merged GL+GLES2 can repeat a name; first wins
        }
        if let Some((ty, val)) = special_const(&e.name, &e.literal_value)
            .or_else(|| classify_const(&e.literal_value, e.is_bitmask))
        {
            let _ = writeln!(s, "pub const {}: {ty} = {val};", e.name);
        }
    }
    s.push('\n');
}

/// Emit the packed function-name blob (one NUL-separated `&[u8]` in `.rodata`,
/// like the C backend's `kFnNameData`), a parallel offset table, and the PFN
/// range tables that `load()` walks.  Packing avoids a per-name pointer +
/// relocation, which is where the naive `[&CStr; K]` table was heaviest.
fn emit_command_tables(fs: &FeatureSet, s: &mut String) {
    let k = fs.commands.len();
    let f = fs.features.len();
    let _ = writeln!(
        s,
        "// ── Command table ───────────────────────────────────────────\n\
         pub const COMMAND_COUNT: usize = {k};\n\
         pub const FEATURE_COUNT: usize = {f};\n"
    );

    // Record each command name's start offset in the blob.  The offset type is
    // the narrowest that indexes it.
    let mut offsets: Vec<usize> = Vec::with_capacity(k);
    let mut total = 0usize;
    for c in &fs.commands {
        offsets.push(total);
        total += c.name.len() + 1;
    }
    let off_ty = if offsets.last().copied().unwrap_or(0) <= u16::MAX as usize {
        "u16"
    } else {
        "u32"
    };

    // One NUL-terminated name per source line, joined at compile time by the
    // `\`-continuations (each strips the newline + indent of the next line).
    // Rust can't interleave comments inside a literal like C's blob, so the
    // name↔index↔offset annotations live on FN_NAME_OFFSETS below.
    s.push_str("#[rustfmt::skip]\n");
    s.push_str("static FN_NAME_DATA: &[u8] = b\"\\\n");
    for c in &fs.commands {
        let _ = writeln!(s, "    {}\\0\\", c.name);
    }
    s.push_str("\";\n\n");

    s.push_str("// Byte offset of each command name in FN_NAME_DATA, indexed in\n");
    s.push_str("// lockstep with the pfn table (slot [i] == command i).\n");
    s.push_str("#[rustfmt::skip]\n");
    let _ = writeln!(s, "static FN_NAME_OFFSETS: [{off_ty}; COMMAND_COUNT] = [");
    for (i, c) in fs.commands.iter().enumerate() {
        let _ = writeln!(s, "    {:>7}, // [{i}] {}", offsets[i], c.name);
    }
    s.push_str("];\n\n");

    // (feature/extension index, first pfn slot, count) triples.  Extension
    // ranges are per-API so each `load_<api>` loads only its own.
    emit_range_table(s, "FEATURE_RANGES", &fs.feature_pfn_ranges, |i| {
        fs.features
            .get(i as usize)
            .map_or_else(String::new, |f| f.full_name.clone())
    });
    for (api, ranges) in &fs.ext_pfn_ranges {
        emit_range_table(s, &format!("EXT_RANGES_{api}"), ranges, |i| {
            fs.extensions
                .get(i as usize)
                .map_or_else(String::new, |e| e.name.clone())
        });
    }
}

/// Emit a `(feature/ext index, first pfn slot, count)` range table, one entry
/// per line with a `// <name>` comment (`label` maps the index to its
/// feature/extension name), under `#[rustfmt::skip]` to keep the alignment.
fn emit_range_table(
    s: &mut String,
    name: &str,
    ranges: &[crate::resolve::PfnRange],
    label: impl Fn(u16) -> String,
) {
    s.push_str("#[rustfmt::skip]\n");
    let _ = writeln!(s, "static {name}: [(u16, u16, u16); {}] = [", ranges.len());
    for r in ranges {
        let _ = writeln!(
            s,
            "    ({:>4}, {:>4}, {:>4}), // {}",
            r.extension,
            r.start,
            r.count,
            label(r.extension)
        );
    }
    s.push_str("];\n\n");
}

/// Emit the context struct, per-API `load_<api>` functions (version detection,
/// version/extension-gated loading, alias resolution), the dispatch methods,
/// and the feature/extension presence queries.
fn emit_context(fs: &FeatureSet, s: &mut String) {
    let idx_getstring = cmd_index(fs, "glGetString");
    let idx_gi = cmd_index(fs, "glGetIntegerv");
    let idx_gsi = cmd_index(fs, "glGetStringi");
    let has_alias = !fs.alias_pairs.is_empty();

    if has_alias {
        let _ = writeln!(
            s,
            "// (canonical, secondary) command indices propagated by --alias.\n\
             #[rustfmt::skip]\n\
             static ALIAS_PAIRS: [(u16, u16); {}] = [",
            fs.alias_pairs.len()
        );
        for p in &fs.alias_pairs {
            let cn = fs
                .commands
                .get(p.canonical as usize)
                .map_or("", |c| c.name.as_str());
            let sn = fs
                .commands
                .get(p.secondary as usize)
                .map_or("", |c| c.name.as_str());
            let _ = writeln!(
                s,
                "    ({:>4}, {:>4}), // {cn} <-> {sn}",
                p.canonical, p.secondary
            );
        }
        s.push_str("];\n\n");
    }

    s.push_str(
        "// ── Context ─────────────────────────────────────────────────\n\
         /// Why a `load_<api>` constructor failed.\n\
         #[derive(Copy, Clone, PartialEq, Eq, Debug)]\n\
         pub enum LoadError {\n\
         \x20   /// `loader` returned null for `glGetString` — the current context\n\
         \x20   /// exposes no GL at all (or `loader` is not a GL proc-address source).\n\
         \x20   MissingGetString,\n\
         \x20   /// `GL_VERSION` was null or unparseable, so no API level could be\n\
         \x20   /// detected.\n\
         \x20   UnparseableVersion,\n\
         }\n\n\
         impl core::fmt::Display for LoadError {\n\
         \x20   fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {\n\
         \x20       f.write_str(match self {\n\
         \x20           LoadError::MissingGetString => \"glGetString is not available in this context\",\n\
         \x20           LoadError::UnparseableVersion => \"GL_VERSION missing or unparseable\",\n\
         \x20       })\n\
         \x20   }\n\
         }\n\n\
         impl core::error::Error for LoadError {}\n\n\
         /// Loaded GL entry points plus detected feature/extension presence.\n\
         /// The PFN table is inline (not boxed) for single-indirection dispatch;\n\
         /// not `Clone`/`Copy` to avoid copying the whole table.\n\
         pub struct Gl {\n\
         \x20   pfns: [*const c_void; COMMAND_COUNT],\n\
         \x20   feat: [bool; FEATURE_COUNT],\n\
         \x20   ext: [bool; EXT_COUNT],\n\
         \x20   version: u32,\n\
         }\n\n\
         impl Gl {\n",
    );

    for api in &fs.apis {
        emit_load(s, fs, api, idx_getstring);
    }

    // Shared range loader.
    s.push_str(
        "\n    #[inline]\n\
         \x20   unsafe fn load_range(\n\
         \x20       &mut self,\n\
         \x20       loader: &mut dyn FnMut(&CStr) -> *const c_void,\n\
         \x20       start: u16,\n\
         \x20       count: u16,\n\
         \x20   ) {\n\
         \x20       for i in start..start + count {\n\
         \x20           let idx = i as usize;\n\
         \x20           let off = FN_NAME_OFFSETS[idx] as usize;\n\
         \x20           let name =\n\
         \x20               unsafe { CStr::from_bytes_until_nul(&FN_NAME_DATA[off..]).unwrap_unchecked() };\n\
         \x20           self.pfns[idx] = loader(name);\n\
         \x20       }\n\
         \x20   }\n",
    );

    emit_detect(s, idx_gi, idx_gsi);

    if has_alias {
        s.push_str(
            "\n    /// Propagate each loaded pointer to its unloaded alias slot.\n\
             \x20   fn resolve_aliases(&mut self) {\n\
             \x20       for &(ci, si) in ALIAS_PAIRS.iter() {\n\
             \x20           let (c, d) = (self.pfns[ci as usize], self.pfns[si as usize]);\n\
             \x20           if c.is_null() && !d.is_null() {\n\
             \x20               self.pfns[ci as usize] = d;\n\
             \x20           } else if !c.is_null() && d.is_null() {\n\
             \x20               self.pfns[si as usize] = c;\n\
             \x20           }\n\
             \x20       }\n\
             \x20   }\n",
        );
    } else {
        s.push_str("\n    fn resolve_aliases(&mut self) {}\n");
    }

    s.push_str(
        "\n    /// Detected API version, packed as `major << 8 | minor`.\n\
         \x20   #[inline]\n\
         \x20   pub fn version(&self) -> u32 {\n\
         \x20       self.version\n\
         \x20   }\n",
    );

    // Dispatch method per command, then bool query per extension and per
    // feature.  One name set guards against any collision across the three.
    let first_api = fs.apis.first().map(String::as_str).unwrap_or("gl");
    s.push_str(
        "\n    // Dispatch wrappers.  The pointer local is named `__pfn` because GL\n\
         \x20   // parameter names (`f`, `n`, ...) could otherwise collide with it.\n",
    );
    let mut used: std::collections::HashSet<String> = std::collections::HashSet::new();
    for cmd in &fs.commands {
        if used.insert(cmd.short_name.clone()) {
            s.push('\n');
            s.push_str(&emit_method(cmd, first_api));
            s.push('\n');
        }
    }
    for e in &fs.extensions {
        let name = method_name(&e.short_name);
        if used.insert(name.clone()) {
            emit_flag_query(s, &name, &e.name, "ext", e.index, "advertises");
        }
    }
    for feat in &fs.features {
        let name = method_name(&feat.short_name);
        if used.insert(name.clone()) {
            emit_flag_query(s, &name, &feat.full_name, "feat", feat.index, "supports");
        }
    }

    s.push_str("}\n");
}

/// Emit one `load_<api>` function: bootstrap glGetString, detect the version and
/// set this API's feature flags, load enabled feature PFNs, detect extensions,
/// load enabled extension PFNs, resolve aliases.
fn emit_load(s: &mut String, fs: &FeatureSet, api: &str, idx_getstring: Option<u16>) {
    let _ = write!(
        s,
        "    /// Load the {api} API against `loader` (a GetProcAddress-style\n\
         \x20   /// callback), detecting version then extensions.  `Err` when the\n\
         \x20   /// current context has no usable {api}.\n\
         \x20   ///\n\
         \x20   /// # Safety\n\
         \x20   /// A matching GL context must be current and `loader` valid.\n\
         \x20   #[inline]\n\
         \x20   pub unsafe fn load_{api}(\n\
         \x20       mut loader: impl FnMut(&CStr) -> *const c_void,\n\
         \x20   ) -> Result<Self, LoadError> {{\n\
         \x20       // Immediately erase to `&mut dyn` — the real loader is compiled\n\
         \x20       // once, not once per closure type.\n\
         \x20       unsafe {{ Self::load_{api}_dyn(&mut loader) }}\n\
         \x20   }}\n\n\
         \x20   unsafe fn load_{api}_dyn(\n\
         \x20       loader: &mut dyn FnMut(&CStr) -> *const c_void,\n\
         \x20   ) -> Result<Self, LoadError> {{\n\
         \x20       let mut gl = Self {{\n\
         \x20           pfns: [core::ptr::null(); COMMAND_COUNT],\n\
         \x20           feat: [false; FEATURE_COUNT],\n\
         \x20           ext: [false; EXT_COUNT],\n\
         \x20           version: 0,\n\
         \x20       }};\n"
    );

    let Some(gs) = idx_getstring else {
        // No glGetString in this build — can't version-detect.
        s.push_str("        Err(LoadError::MissingGetString)\n    }\n");
        return;
    };

    let _ = write!(
        s,
        "        gl.pfns[{gs}] = loader(c\"glGetString\");\n\
         \x20       if gl.pfns[{gs}].is_null() {{\n\
         \x20           return Err(LoadError::MissingGetString);\n\
         \x20       }}\n\
         \x20       gl.version = unsafe {{ __parse_gl_version(gl.GetString(GL_VERSION)) }};\n\
         \x20       if gl.version == 0 {{\n\
         \x20           return Err(LoadError::UnparseableVersion);\n\
         \x20       }}\n\
         \x20       // Feature presence for this API, from the parsed version.\n"
    );
    for feat in fs.features.iter().filter(|f| f.api == api) {
        let _ = writeln!(
            s,
            "        gl.feat[{idx}] = gl.version >= 0x{packed:04x};",
            idx = feat.index,
            packed = feat.packed
        );
    }
    let _ = write!(
        s,
        "        for &(fi, start, count) in FEATURE_RANGES.iter() {{\n\
         \x20           if gl.feat[fi as usize] {{\n\
         \x20               unsafe {{ gl.load_range(loader, start, count) }};\n\
         \x20           }}\n\
         \x20       }}\n\
         \x20       unsafe {{ gl.detect_extensions() }};\n\
         \x20       for &(ei, start, count) in EXT_RANGES_{api}.iter() {{\n\
         \x20           if gl.ext[ei as usize] {{\n\
         \x20               unsafe {{ gl.load_range(loader, start, count) }};\n\
         \x20           }}\n\
         \x20       }}\n\
         \x20       gl.resolve_aliases();\n\
         \x20       Ok(gl)\n\
         \x20   }}\n"
    );
}

/// Emit an `#[inline] pub fn <name>(&self) -> bool` presence query reading
/// `self.<field>[idx]`.  Plain indexing: the index is a literal below the
/// array length, so the bounds check is statically elided — no `unsafe`.
fn emit_flag_query(s: &mut String, name: &str, full: &str, field: &str, idx: u16, verb: &str) {
    let _ = write!(
        s,
        "\n    /// Whether the driver {verb} `{full}`.\n\
         \x20   #[inline]\n\
         \x20   pub fn {name}(&self) -> bool {{\n\
         \x20       self.{field}[{idx}]\n\
         \x20   }}\n"
    );
}

/// Index of a command by full name (`glGetString`, ...), if present.
fn cmd_index(fs: &FeatureSet, name: &str) -> Option<u16> {
    fs.commands.iter().find(|c| c.name == name).map(|c| c.index)
}

/// Emit `detect_extensions`: enumerate the driver's extensions, hash each, and
/// flag matches against the pre-baked table via binary search.  Degrades to a
/// no-op when the required entry points aren't in the build.
fn emit_detect(s: &mut String, idx_gi: Option<u16>, idx_gsi: Option<u16>) {
    match (idx_gi, idx_gsi) {
        (Some(gi), Some(gsi)) => {
            let _ = write!(
                s,
                "\n    /// # Safety: the loaded entry points must be callable.\n\
                 \x20   unsafe fn detect_extensions(&mut self) {{\n\
                 \x20       if self.pfns[{gi}].is_null() || self.pfns[{gsi}].is_null() {{\n\
                 \x20           return;\n\
                 \x20       }}\n\
                 \x20       let mut n: GLint = 0;\n\
                 \x20       unsafe {{ self.GetIntegerv(GL_NUM_EXTENSIONS, &mut n) }};\n\
                 \x20       let mut i: GLuint = 0;\n\
                 \x20       while (i as GLint) < n {{\n\
                 \x20           let name = unsafe {{ self.GetStringi(GL_EXTENSIONS, i) }};\n\
                 \x20           if !name.is_null() {{\n\
                 \x20               let h = unsafe {{ __ext_hash(name) }};\n\
                 \x20               if let Ok(pos) = EXT_HASH_KEYS.binary_search(&h) {{\n\
                 \x20                   self.ext[EXT_HASH_IDX[pos] as usize] = true;\n\
                 \x20               }}\n\
                 \x20           }}\n\
                 \x20           i += 1;\n\
                 \x20       }}\n\
                 \x20   }}\n",
            );
        }
        _ => {
            s.push_str(
                "\n    /// Extension detection unavailable (glGetIntegerv/glGetStringi\n\
                 \x20   /// absent from this build).\n\
                 \x20   unsafe fn detect_extensions(&mut self) {}\n",
            );
        }
    }
}

/// Method name for an extension presence query: the short name, `_`-prefixed if
/// it would otherwise start with a digit (e.g. `3DFX_multisample`).
fn method_name(short: &str) -> String {
    if short.starts_with(|c: char| c.is_ascii_digit()) {
        format!("_{short}")
    } else {
        short.to_string()
    }
}

/// Emit one `#[inline] unsafe fn` dispatch wrapper for `cmd`.  The PFN slot is
/// transmuted to `Option<fn>` (guaranteed null-pointer-optimized), never to a
/// bare `fn`: a bare fn pointer has a non-null validity invariant, so the
/// transmute itself would be UB for an unloaded slot, even if never called.
/// `first_api` names the `load_<api>` the safety docs point at.
fn emit_method(cmd: &Command, first_api: &str) -> String {
    let params: Vec<(String, String)> = cmd
        .params
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let name = if p.name.is_empty() {
                format!("arg{i}")
            } else {
                sanitize_ident(&p.name)
            };
            (name, translate_type(&p.type_raw))
        })
        .collect();

    let ret = if cmd.return_type.trim() == "void" {
        None
    } else {
        Some(translate_type(&cmd.return_type))
    };
    let ret_sig = ret.as_ref().map(|r| format!(" -> {r}")).unwrap_or_default();

    let recv_and_args = if params.is_empty() {
        "&self".to_string()
    } else {
        let list = params
            .iter()
            .map(|(n, t)| format!("{n}: {t}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("&self, {list}")
    };
    let fn_param_types = params
        .iter()
        .map(|(_, t)| t.clone())
        .collect::<Vec<_>>()
        .join(", ");
    let call_args = params
        .iter()
        .map(|(n, _)| n.clone())
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        "    /// # Safety\n\
         \x20   /// The context must be loaded and current; see [`Gl::load_{first_api}`].\n\
         \x20   #[inline]\n\
         \x20   pub unsafe fn {short}({recv_and_args}){ret_sig} {{\n\
         \x20       let __pfn: Option<unsafe extern \"system\" fn({fn_param_types}){ret_sig}> =\n\
         \x20           unsafe {{ core::mem::transmute(*self.pfns.get_unchecked({idx})) }};\n\
         \x20       let __pfn = match __pfn {{\n\
         \x20           Some(__f) => __f,\n\
         \x20           None => unsafe {{ __missing({idx}) }},\n\
         \x20       }};\n\
         \x20       unsafe {{ __pfn({call_args}) }}\n\
         \x20   }}",
        short = cmd.short_name,
        idx = cmd.index,
    )
}

/// Emit the `--mx-global` layer: a process-global context in an `UnsafeCell`
/// (written once by `init_global`, read-only afterwards) plus one free
/// function per command / presence query delegating to it.  This is the
/// analogue of the C loader's `gloam_gl_context` global + dispatch macros:
/// dispatch is `[link-time-fixed base + offset] -> PFN`, no branch and no
/// atomic on the hot path.  The global starts as a fully zeroed context, so
/// reading flags before `init_global` is defined (everything absent) — only
/// dispatching is a contract violation, caught by the dispatch debug_asserts.
fn emit_global(fs: &FeatureSet, s: &mut String) {
    s.push_str(
        "\n// ── Global context (--mx-global) ───────────────────────────\n\
         impl Gl {\n\
         \x20   /// Every PFN null, every flag false, version 0.\n\
         \x20   const EMPTY: Gl = Gl {\n\
         \x20       pfns: [core::ptr::null(); COMMAND_COUNT],\n\
         \x20       feat: [false; FEATURE_COUNT],\n\
         \x20       ext: [false; EXT_COUNT],\n\
         \x20       version: 0,\n\
         \x20   };\n\
         }\n\n\
         struct GlobalCell(core::cell::UnsafeCell<Gl>);\n\
         // SAFETY: written only by `init_global` (whose contract forbids racing\n\
         // it against any other access), read-only afterwards.\n\
         unsafe impl Sync for GlobalCell {}\n\n\
         static GLOBAL: GlobalCell = GlobalCell(core::cell::UnsafeCell::new(Gl::EMPTY));\n\n\
         /// Install `gl` as the process-global context behind the free functions.\n\
         ///\n\
         /// # Safety\n\
         /// Must complete before — and must never run concurrently with — any\n\
         /// other use of the global (free functions, [`global`]).  The write is\n\
         /// unsynchronized, so publish it to other threads through an ordinary\n\
         /// happens-before edge (spawn the render thread after this returns, send\n\
         /// a message, etc.), exactly as with the C loader's global context.\n\
         pub unsafe fn init_global(gl: Gl) {\n\
         \x20   unsafe { *GLOBAL.0.get() = gl };\n\
         }\n\n\
         /// The process-global context installed by [`init_global`] (a zeroed\n\
         /// context beforehand: flags read false, dispatch is a contract\n\
         /// violation).\n\
         #[inline]\n\
         pub fn global() -> &'static Gl {\n\
         \x20   // SAFETY: no `&mut` can exist outside `init_global`, whose contract\n\
         \x20   // forbids concurrent calls to this.\n\
         \x20   unsafe { &*GLOBAL.0.get() }\n\
         }\n",
    );

    // One-shot per-API loaders straight into the global (C: gloamLoadGL).
    for api in &fs.apis {
        let _ = write!(
            s,
            "\n/// Load the {api} API (see [`Gl::load_{api}`]) and install it as the\n\
             /// process-global context.\n\
             ///\n\
             /// # Safety\n\
             /// [`Gl::load_{api}`]'s contract plus [`init_global`]'s.\n\
             pub unsafe fn load_{api}_global(\n\
             \x20   loader: impl FnMut(&CStr) -> *const c_void,\n\
             ) -> Result<(), LoadError> {{\n\
             \x20   unsafe {{ init_global(Gl::load_{api}(loader)?) }};\n\
             \x20   Ok(())\n\
             }}\n"
        );
    }

    // Free-function mirrors: commands, then extension / feature queries, with
    // the same collision-guard order as the methods.
    let mut used: std::collections::HashSet<String> = std::collections::HashSet::new();
    for cmd in &fs.commands {
        if used.insert(cmd.short_name.clone()) {
            s.push('\n');
            s.push_str(&emit_global_fn(cmd));
            s.push('\n');
        }
    }
    for e in &fs.extensions {
        let name = method_name(&e.short_name);
        if used.insert(name.clone()) {
            emit_global_flag_query(s, &name, &e.name, "advertises");
        }
    }
    for feat in &fs.features {
        let name = method_name(&feat.short_name);
        if used.insert(name.clone()) {
            emit_global_flag_query(s, &name, &feat.full_name, "supports");
        }
    }

    s.push_str(
        "\n/// Detected API version of the global context (`major << 8 | minor`;\n\
         /// 0 before [`init_global`]).\n\
         #[inline]\n\
         pub fn version() -> u32 {\n\
         \x20   global().version()\n\
         }\n",
    );
}

/// Emit one free-function dispatch wrapper delegating to the method on the
/// global context.
fn emit_global_fn(cmd: &Command) -> String {
    let params: Vec<(String, String)> = cmd
        .params
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let name = if p.name.is_empty() {
                format!("arg{i}")
            } else {
                sanitize_ident(&p.name)
            };
            (name, translate_type(&p.type_raw))
        })
        .collect();

    let ret = if cmd.return_type.trim() == "void" {
        None
    } else {
        Some(translate_type(&cmd.return_type))
    };
    let ret_sig = ret.as_ref().map(|r| format!(" -> {r}")).unwrap_or_default();
    let sig_params = params
        .iter()
        .map(|(n, t)| format!("{n}: {t}"))
        .collect::<Vec<_>>()
        .join(", ");
    let call_args = params
        .iter()
        .map(|(n, _)| n.clone())
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        "/// # Safety\n\
         /// See [`Gl::{short}`]; the global context must be initialized.\n\
         #[inline]\n\
         pub unsafe fn {short}({sig_params}){ret_sig} {{\n\
         \x20   unsafe {{ global().{short}({call_args}) }}\n\
         }}",
        short = cmd.short_name,
    )
}

/// Emit a free presence query over the global context.  Safe: the zeroed
/// pre-init global reads as "nothing present".
fn emit_global_flag_query(s: &mut String, name: &str, full: &str, verb: &str) {
    let _ = write!(
        s,
        "\n/// Whether the global context's driver {verb} `{full}`.\n\
         #[inline]\n\
         pub fn {name}() -> bool {{\n\
         \x20   global().{name}()\n\
         }}\n"
    );
}

/// Translate a C type string from the spec (`Param.type_raw` /
/// `Command.return_type`) into a Rust type, relying on the emitted base-type
/// aliases and newtypes for the base name.  Handles `const` and pointer levels;
/// GL's handful of exotic types are out of scope for this slice.
fn translate_type(raw: &str) -> String {
    let stars = raw.matches('*').count();
    // GL pointer params are uniformly read-only (`const T *`, `const T *const*`)
    // or read-write (`T *`), so const-ness applies to every pointer level: e.g.
    // `const GLchar *const *` -> `*const *const GLchar`, `GLuint *` -> `*mut GLuint`.
    let ptr = if raw.contains("const") {
        "*const "
    } else {
        "*mut "
    };
    let base: String = raw
        .replace("const", " ")
        .replace("struct", " ")
        .replace('*', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let base_rust = match base.as_str() {
        "void" | "GLvoid" => "c_void",
        other => other,
    };
    let mut t = base_rust.to_string();
    for _ in 0..stars {
        t = format!("{ptr}{t}");
    }
    t
}

/// Append `_` to Rust keywords used as GL parameter names (`type`, `ref`, ...).
fn sanitize_ident(name: &str) -> String {
    const KEYWORDS: &[&str] = &[
        "type", "ref", "in", "fn", "match", "box", "move", "become", "final", "override", "priv",
        "as", "loop", "self", "async", "await", "dyn",
    ];
    if KEYWORDS.contains(&name) {
        format!("{name}_")
    } else {
        name.to_string()
    }
}

/// Name-based typing overrides for GL's polymorphic "special numbers", whose
/// spec block carries no type: booleans must be passable to `GLboolean`
/// parameters, and the sentinels must match the parameter/return types they
/// are compared against (`GLuint64 timeout`, `GLuint` block indices).
/// `GL_ZERO`/`GL_ONE`/`GL_NONE`/`GL_NO_ERROR` deliberately stay `GLenum` (the
/// design-doc default) — their enum uses dominate their integer uses.
fn special_const(name: &str, literal: &str) -> Option<(String, String)> {
    let ty = match name {
        "GL_TRUE" | "GL_FALSE" => "GLboolean",
        "GL_TIMEOUT_IGNORED" | "GL_TIMEOUT_IGNORED_APPLE" => "GLuint64",
        "GL_INVALID_INDEX" => "GLuint",
        _ => return None,
    };
    Some((ty.to_string(), literal.trim().to_string()))
}

/// Classify a flat-enum literal into `(rust_type, value_expr)`.  Values that fit
/// `u32` become `GLenum` newtype constants — or `GLbitfield` when the spec
/// defined them in a `<enums type="bitmask">` block, so `GL_*_BIT` constants
/// pass directly to `GLbitfield` parameters and combine with `|`.  Wider values
/// fall back to `u64`, negatives to `GLint`.
fn classify_const(literal: &str, is_bitmask: bool) -> Option<(String, String)> {
    let s = literal.trim();
    let s = s.trim_end_matches(['u', 'U', 'l', 'L']);
    if s.is_empty() {
        return None;
    }
    let (neg, body) = match s.strip_prefix('-') {
        Some(b) => (true, b),
        None => (false, s),
    };
    let value: u64 = if let Some(hex) = body.strip_prefix("0x").or_else(|| body.strip_prefix("0X"))
    {
        u64::from_str_radix(hex, 16).ok()?
    } else {
        body.parse::<u64>().ok()?
    };

    Some(if neg {
        ("GLint".to_string(), s.to_string())
    } else if value > u32::MAX as u64 {
        ("u64".to_string(), s.to_string())
    } else if is_bitmask {
        ("GLbitfield".to_string(), format!("GLbitfield({s})"))
    } else {
        ("GLenum".to_string(), format!("GLenum({s})"))
    })
}

/// Rendered files end with exactly one newline.
fn normalize_eof(mut rendered: String) -> String {
    rendered.truncate(rendered.trim_end_matches('\n').len());
    rendered.push('\n');
    rendered
}
