//! GL / GLES loader emission for the Rust backend.
//!
//! The generated module mirrors the C backend's GL contract: per-API
//! `load_<api>` constructors bootstrap `glGetString`, parse `GL_VERSION` into
//! a packed version, set that API's feature flags, load the enabled feature
//! PFN ranges, detect extensions (`glGetIntegerv(GL_NUM_EXTENSIONS)` +
//! `glGetStringi` hashed against the pre-baked XXH3 table), load the enabled
//! per-API extension ranges, and resolve `--alias` pairs.  Dispatch is
//! `#[inline]` methods on the owned `Gl` context; `--mx-global` additionally
//! emits a process-global context with free-function dispatch (the analogue
//! of the C loader's `gloam_gl_context` + macros).

use std::fmt::Write as _;

use anyhow::Result;

use crate::resolve::FeatureSet;

use super::abi::{emit_base_types, rust_type};
use super::common::{
    cmd_index, emit_alias_pairs_table, emit_command_tables, emit_extension_tables, emit_flag_query,
    emit_global_flag_query, emit_global_fn, emit_load_range_method, emit_method,
    emit_missing_helpers, emit_resolve_aliases_method, method_name,
};

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

/// Build the whole module body (everything after the preamble).
pub(super) fn emit_gl_module(fs: &FeatureSet, preamble: &str, mx_global: bool) -> Result<String> {
    let mut s = String::with_capacity(256 * 1024);

    s.push_str(preamble);
    s.push('\n');
    s.push_str(
        "#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, clippy::all)]\n\
         #![deny(unsafe_op_in_unsafe_fn)]\n\n\
         use core::ffi::{CStr, c_char, c_void};\n\n",
    );

    s.push_str("// ── GL base types ───────────────────────────────────────────\n");
    emit_base_types(fs, &mut s, &["GLenum", "GLbitfield"], &|_| None)?;
    s.push('\n');
    s.push_str("// ── Enum newtypes ───────────────────────────────────────────\n");
    s.push_str(NEWTYPES);
    s.push('\n');

    emit_constants(fs, &mut s);
    emit_command_tables(fs, &mut s);
    emit_extension_tables(fs, &mut s)?;
    emit_gl_ext_hash(&mut s);
    emit_missing_helpers(&mut s, "GL");
    emit_version_helpers(&mut s);
    emit_context(fs, &mut s);
    if mx_global {
        emit_global(fs, &mut s);
    }

    Ok(s)
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

/// Emit the GL-flavored runtime hash helper (NUL-terminated `GLubyte`
/// strings, as `glGetStringi` returns them).  The Vulkan loader emits its
/// own over `VkExtensionProperties` names.
fn emit_gl_ext_hash(s: &mut String) {
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

/// Emit the context struct, per-API `load_<api>` functions (version detection,
/// version/extension-gated loading, alias resolution), the dispatch methods,
/// and the feature/extension presence queries.
fn emit_context(fs: &FeatureSet, s: &mut String) {
    let idx_getstring = cmd_index(fs, "glGetString");
    let idx_gi = cmd_index(fs, "glGetIntegerv");
    let idx_gsi = cmd_index(fs, "glGetStringi");
    let has_alias = !fs.alias_pairs.is_empty();

    // Per-API extension index subsets (the C backend's kExtIdx_<api>): in a
    // merged build, load_<api> claims only the extensions its API supports,
    // even when the driver advertises a name the other API's table knows.
    for (api, subset) in &fs.ext_subset_indices {
        s.push_str("// extArray indices this API supports.\n#[rustfmt::skip]\n");
        let _ = writeln!(s, "static EXT_IDX_{api}: [u16; {}] = [", subset.len());
        for &idx in subset {
            let name = fs
                .extensions
                .get(idx as usize)
                .map_or("", |e| e.name.as_str());
            let _ = writeln!(s, "    {idx:>4}, // {name}");
        }
        s.push_str("];\n\n");
    }

    emit_alias_pairs_table(fs, s);

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

    emit_load_range_method(s);

    emit_detect(s, idx_gi, idx_gsi);

    emit_resolve_aliases_method(s, has_alias);

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
    let safety = format!("The context must be loaded and current; see [`Gl::load_{first_api}`].");
    let mut used: std::collections::HashSet<String> = std::collections::HashSet::new();
    for cmd in &fs.commands {
        if used.insert(cmd.short_name.clone()) {
            s.push('\n');
            s.push_str(&emit_method(cmd, &safety, &rust_type));
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
         \x20       unsafe {{ gl.detect_extensions(&EXT_IDX_{api}) }};\n\
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

/// Emit `detect_extensions`: enumerate the driver's extensions, hash each into
/// a scratch match set, then assign exactly the calling API's subset indices
/// from it — the C loader's `kExtIdx_<api>` semantics, so a merged build's
/// `load_gl` never claims a GLES-only extension (or vice versa).  Degrades to
/// a no-op when the required entry points aren't in the build.
fn emit_detect(s: &mut String, idx_gi: Option<u16>, idx_gsi: Option<u16>) {
    match (idx_gi, idx_gsi) {
        (Some(gi), Some(gsi)) => {
            let _ = write!(
                s,
                "\n    /// # Safety: the loaded entry points must be callable.\n\
                 \x20   unsafe fn detect_extensions(&mut self, subset: &[u16]) {{\n\
                 \x20       if self.pfns[{gi}].is_null() || self.pfns[{gsi}].is_null() {{\n\
                 \x20           return;\n\
                 \x20       }}\n\
                 \x20       let mut seen = [false; EXT_COUNT];\n\
                 \x20       let mut n: GLint = 0;\n\
                 \x20       unsafe {{ self.GetIntegerv(GL_NUM_EXTENSIONS, &mut n) }};\n\
                 \x20       let mut i: GLuint = 0;\n\
                 \x20       while (i as GLint) < n {{\n\
                 \x20           let name = unsafe {{ self.GetStringi(GL_EXTENSIONS, i) }};\n\
                 \x20           if !name.is_null() {{\n\
                 \x20               let h = unsafe {{ __ext_hash(name) }};\n\
                 \x20               if let Ok(pos) = EXT_HASH_KEYS.binary_search(&h) {{\n\
                 \x20                   seen[EXT_HASH_IDX[pos] as usize] = true;\n\
                 \x20               }}\n\
                 \x20           }}\n\
                 \x20           i += 1;\n\
                 \x20       }}\n\
                 \x20       // Only this API's subset: the extension flags an API never\n\
                 \x20       // declared stay false even if the driver advertises the name.\n\
                 \x20       for &idx in subset {{\n\
                 \x20           self.ext[idx as usize] = seen[idx as usize];\n\
                 \x20       }}\n\
                 \x20   }}\n",
            );
        }
        _ => {
            s.push_str(
                "\n    /// Extension detection unavailable (glGetIntegerv/glGetStringi\n\
                 \x20   /// absent from this build).\n\
                 \x20   unsafe fn detect_extensions(&mut self, _subset: &[u16]) {}\n",
            );
        }
    }
}

/// Emit the `--mx-global` layer: a process-global context in an `UnsafeCell`
/// (every write exclusive via `init_global`; re-initialization is supported)
/// plus one free function per command / presence query delegating to it.  This is the
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
         // SAFETY: every write goes through `init_global`, whose contract makes\n\
         // the write exclusive (nothing else touches the global while it runs,\n\
         // and no reference from `global()` outlives into it); all other access\n\
         // is read-only.\n\
         unsafe impl Sync for GlobalCell {}\n\n\
         static GLOBAL: GlobalCell = GlobalCell(core::cell::UnsafeCell::new(Gl::EMPTY));\n\n\
         /// Install `gl` as the process-global context behind the free functions.\n\
         ///\n\
         /// The global is not write-once: calling this again replaces the\n\
         /// context, which is exactly what a GL context swap needs (destroy the\n\
         /// old context, create the new one — possibly on a different driver —\n\
         /// make it current, reload, re-install).  That moment is naturally\n\
         /// quiescent: with the old context gone, no thread may legally be\n\
         /// calling GL anyway.\n\
         ///\n\
         /// # Safety\n\
         /// Each call must be exclusive: nothing may use the global while it\n\
         /// runs — no free-function calls, no [`global`] calls — and no `&Gl`\n\
         /// previously returned by [`global`] may still be held (re-installing\n\
         /// while such a reference lives is undefined behavior even if it is\n\
         /// never read again).  The write is unsynchronized, so publish it to\n\
         /// other threads through an ordinary happens-before edge (spawn the\n\
         /// render thread after this returns, send a message, etc.), exactly as\n\
         /// with the C loader's global context.\n\
         pub unsafe fn init_global(gl: Gl) {\n\
         \x20   unsafe { *GLOBAL.0.get() = gl };\n\
         }\n\n\
         /// The process-global context installed by [`init_global`] (a zeroed\n\
         /// context beforehand: flags read false, dispatch is a contract\n\
         /// violation).  Do not hold the returned reference across a later\n\
         /// [`init_global`] call — its `'static` lifetime is a loan against the\n\
         /// global staying untouched, exactly like keeping a pointer to the C\n\
         /// loader's global context across a reload.\n\
         #[inline]\n\
         pub fn global() -> &'static Gl {\n\
         \x20   // SAFETY: no `&mut` can exist outside `init_global`, whose contract\n\
         \x20   // forbids running while this reference (or any use of it) lives.\n\
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
            s.push_str(&emit_global_fn(cmd, "Gl", &rust_type));
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

/// Name-based typing overrides for GL's polymorphic "special numbers", whose
/// spec block carries no type: booleans must be passable to `GLboolean`
/// parameters, and the sentinels must match the parameter/return types they
/// are compared against (`GLuint64 timeout`, `GLuint` block indices).
/// `GL_ZERO`/`GL_ONE`/`GL_NONE`/`GL_NO_ERROR` deliberately stay `GLenum` —
/// their enum uses dominate their integer uses.
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
