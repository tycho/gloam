//! EGL loader emission for the Rust backend.
//!
//! The generated crate mirrors the C backend's EGL contract:
//! `Egl::load_egl(display, loader)` bootstraps `eglQueryString`, parses the
//! version from `eglQueryString(display, EGL_VERSION)`, sets feature flags
//! from it, loads enabled feature PFNs, detects extensions (the client
//! extension string at `EGL_NO_DISPLAY` plus the display extension string
//! when `display` is real), loads enabled extension PFNs, and resolves
//! aliases.
//!
//! Passing `EGL_NO_DISPLAY` is a supported phase-0 (EGL 1.5 semantics):
//! the version and extension strings then describe the *client* library,
//! which is exactly what's needed to detect `EGL_ANGLE_platform_angle` and
//! friends before any display exists.  Re-run `load_egl` with the real
//! display afterwards for the full picture.

use std::fmt::Write as _;

use anyhow::Result;

use super::{
    emit_base_types, emit_command_tables, emit_extension_tables, emit_flag_query, emit_method,
    emit_missing_helpers, method_name, rust_type,
};
use crate::resolve::FeatureSet;

/// Per-target declarations for the EGL native handle types, mirroring
/// eglplatform.h's platform arms.  In C these come from `EGL/eglplatform.h`;
/// the generated crate has no headers, so each target's ABI shape is
/// declared here.  (Windows: HDC/HWND/HBITMAP; Apple: int display, pointer
/// window/pixmap; Android: ANativeWindow; other unix: Xlib, where
/// Window/Pixmap are XIDs — `unsigned long`.)
fn egl_platform_decl(name: &str) -> Option<&'static str> {
    Some(match name {
        // Defined in eglplatform.h (khronos_int32_t on every platform arm),
        // so egl.xml's own entry is empty-bodied.
        "EGLint" => "pub type EGLint = i32;",
        "EGLNativeDisplayType" => {
            "#[cfg(windows)]\npub type EGLNativeDisplayType = *mut c_void; // HDC\n\
             #[cfg(target_vendor = \"apple\")]\npub type EGLNativeDisplayType = i32;\n\
             #[cfg(target_os = \"android\")]\npub type EGLNativeDisplayType = *mut c_void;\n\
             #[cfg(not(any(windows, target_vendor = \"apple\", target_os = \"android\")))]\n\
             pub type EGLNativeDisplayType = *mut c_void; // Display *"
        }
        "EGLNativeWindowType" => {
            "#[cfg(windows)]\npub type EGLNativeWindowType = *mut c_void; // HWND\n\
             #[cfg(target_vendor = \"apple\")]\npub type EGLNativeWindowType = *mut c_void;\n\
             #[cfg(target_os = \"android\")]\npub type EGLNativeWindowType = *mut ANativeWindow;\n\
             #[cfg(not(any(windows, target_vendor = \"apple\", target_os = \"android\")))]\n\
             pub type EGLNativeWindowType = core::ffi::c_ulong; // Window (XID)"
        }
        "EGLNativePixmapType" => {
            "#[cfg(windows)]\npub type EGLNativePixmapType = *mut c_void; // HBITMAP\n\
             #[cfg(target_vendor = \"apple\")]\npub type EGLNativePixmapType = *mut c_void;\n\
             #[cfg(target_os = \"android\")]\npub type EGLNativePixmapType = *mut c_void;\n\
             #[cfg(not(any(windows, target_vendor = \"apple\", target_os = \"android\")))]\n\
             pub type EGLNativePixmapType = core::ffi::c_ulong; // Pixmap (XID)"
        }
        _ => return None,
    })
}

/// Build the whole EGL module body (everything after the preamble).
pub(super) fn emit_egl_module(fs: &FeatureSet, preamble: &str) -> Result<String> {
    let mut s = String::with_capacity(512 * 1024);

    s.push_str(preamble);
    s.push('\n');
    s.push_str(
        "#![no_std]\n\
         #![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, clippy::all)]\n\
         #![deny(unsafe_op_in_unsafe_fn)]\n\n\
         use core::ffi::{CStr, c_char, c_void};\n\n",
    );

    s.push_str("// ── EGL base types ──────────────────────────────────────────\n");
    emit_base_types(fs, &mut s, &[], &egl_platform_decl)?;
    s.push('\n');

    emit_constants(fs, &mut s)?;
    emit_command_tables(fs, &mut s);
    emit_extension_tables(fs, &mut s)?;
    emit_missing_helpers(&mut s, "EGL");
    emit_context(fs, &mut s);

    Ok(s)
}

/// Emit the flat constants.  EGL's `#define`s are untyped in C; here
/// enum-shaped values become `EGLenum` (a plain `u32` alias, so they
/// compare and assign freely against `EGLBoolean`/`EGLint` expressions
/// after the usual `as` casts), 64-bit values become `u64`
/// (`EGL_FOREVER`), and `EGL_CAST(T,V)` forms become `V as T` — the Rust
/// spelling of the header's own cast macro.
fn emit_constants(fs: &FeatureSet, s: &mut String) -> Result<()> {
    s.push_str("// ── Constants ───────────────────────────────────────────────\n");
    let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for e in &fs.flat_enums {
        if !seen.insert(e.name.as_str()) {
            continue;
        }
        let v = e.literal_value.trim();
        // EGL_CAST(EGLDisplay,0) and friends.
        if let Some(inner) = v
            .strip_prefix("EGL_CAST(")
            .and_then(|r| r.strip_suffix(')'))
        {
            let Some((ty, val)) = inner.split_once(',') else {
                anyhow::bail!("malformed EGL_CAST in constant '{}': '{v}'", e.name);
            };
            let _ = writeln!(
                s,
                "pub const {}: {} = {} as {};",
                e.name,
                ty.trim(),
                val.trim(),
                ty.trim()
            );
            continue;
        }
        let stripped = v.trim_end_matches(['u', 'U', 'l', 'L']);
        let (neg, body) = match stripped.strip_prefix('-') {
            Some(b) => (true, b),
            None => (false, stripped),
        };
        let parsed: Option<u64> =
            if let Some(hex) = body.strip_prefix("0x").or_else(|| body.strip_prefix("0X")) {
                u64::from_str_radix(hex, 16).ok()
            } else {
                body.parse::<u64>().ok()
            };
        let Some(value) = parsed else {
            anyhow::bail!("unrecognized EGL constant literal '{v}' ({})", e.name);
        };
        let ty = if neg {
            "EGLint"
        } else if value > u32::MAX as u64 {
            "u64"
        } else {
            "EGLenum"
        };
        let _ = writeln!(s, "pub const {}: {ty} = {stripped};", e.name);
    }
    s.push('\n');
    Ok(())
}

/// Emit the `Egl` context struct, `load_egl`, version parsing, extension
/// detection over the space-separated client/display strings, dispatch
/// methods, and presence queries.
fn emit_context(fs: &FeatureSet, s: &mut String) {
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
         /// Why [`Egl::load_egl`] failed.\n\
         #[derive(Copy, Clone, PartialEq, Eq, Debug)]\n\
         pub enum LoadError {\n\
         \x20   /// `loader` returned null for `eglQueryString` — not an EGL\n\
         \x20   /// proc-address source.\n\
         \x20   MissingQueryString,\n\
         \x20   /// `EGL_VERSION` was null or unparseable for this display.\n\
         \x20   UnparseableVersion,\n\
         \x20   /// An extension string the C loader requires came back null\n\
         \x20   /// (client string, or display string for a real display).\n\
         \x20   MissingExtensionString,\n\
         }\n\n\
         impl core::fmt::Display for LoadError {\n\
         \x20   fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {\n\
         \x20       f.write_str(match self {\n\
         \x20           LoadError::MissingQueryString => \"eglQueryString is not available\",\n\
         \x20           LoadError::UnparseableVersion => \"EGL_VERSION missing or unparseable\",\n\
         \x20           LoadError::MissingExtensionString => \"EGL extension string missing\",\n\
         \x20       })\n\
         \x20   }\n\
         }\n\n\
         impl core::error::Error for LoadError {}\n\n\
         /// Loaded EGL entry points plus detected feature/extension presence.\n\
         pub struct Egl {\n\
         \x20   pfns: [*const c_void; COMMAND_COUNT],\n\
         \x20   feat: [bool; FEATURE_COUNT],\n\
         \x20   ext: [bool; EXT_COUNT],\n\
         \x20   version: u32,\n\
         }\n\n\
         impl Egl {\n\
         \x20   /// Load EGL against `loader` (a GetProcAddress-style callback),\n\
         \x20   /// detecting the version then extensions for `display`.\n\
         \x20   ///\n\
         \x20   /// `EGL_NO_DISPLAY` is a supported phase-0: the version and\n\
         \x20   /// extensions then describe the *client* library (EGL 1.5\n\
         \x20   /// semantics) — enough to detect client extensions such as\n\
         \x20   /// `EGL_ANGLE_platform_angle` before any display exists.  Load\n\
         \x20   /// again with the real display for the full picture.\n\
         \x20   ///\n\
         \x20   /// # Safety\n\
         \x20   /// `loader` must yield pointers callable as the named EGL\n\
         \x20   /// functions; `display` must be `EGL_NO_DISPLAY` or valid.\n\
         \x20   pub unsafe fn load_egl(\n\
         \x20       display: EGLDisplay,\n\
         \x20       mut loader: impl FnMut(&CStr) -> *const c_void,\n\
         \x20   ) -> Result<Self, LoadError> {\n\
         \x20       unsafe { Self::load_egl_dyn(display, &mut loader) }\n\
         \x20   }\n\n\
         \x20   unsafe fn load_egl_dyn(\n\
         \x20       display: EGLDisplay,\n\
         \x20       loader: &mut dyn FnMut(&CStr) -> *const c_void,\n\
         \x20   ) -> Result<Self, LoadError> {\n\
         \x20       let mut egl = Self {\n\
         \x20           pfns: [core::ptr::null(); COMMAND_COUNT],\n\
         \x20           feat: [false; FEATURE_COUNT],\n\
         \x20           ext: [false; EXT_COUNT],\n\
         \x20           version: 0,\n\
         \x20       };\n",
    );

    let idx_qs = fs
        .commands
        .iter()
        .find(|c| c.name == "eglQueryString")
        .map(|c| c.index);
    let Some(qs) = idx_qs else {
        s.push_str("        Err(LoadError::MissingQueryString)\n    }\n}\n");
        return;
    };

    let _ = write!(
        s,
        "        egl.pfns[{qs}] = loader(c\"eglQueryString\");\n\
         \x20       if egl.pfns[{qs}].is_null() {{\n\
         \x20           return Err(LoadError::MissingQueryString);\n\
         \x20       }}\n\
         \x20       let version = unsafe {{ egl.QueryString(display, EGL_VERSION as EGLint) }};\n\
         \x20       egl.version = __parse_egl_version(version);\n\
         \x20       if egl.version == 0 {{\n\
         \x20           return Err(LoadError::UnparseableVersion);\n\
         \x20       }}\n\
         \x20       // Feature presence from the parsed version.\n"
    );
    for feat in &fs.features {
        let _ = writeln!(
            s,
            "        egl.feat[{idx}] = egl.version >= 0x{packed:04x};",
            idx = feat.index,
            packed = feat.packed
        );
    }
    s.push_str(
        "        for &(fi, start, count) in FEATURE_RANGES.iter() {\n\
         \x20           if egl.feat[fi as usize] {\n\
         \x20               unsafe { egl.load_range(loader, start, count) };\n\
         \x20           }\n\
         \x20       }\n\
         \x20       unsafe { egl.detect_extensions(display)? };\n\
         \x20       for &(ei, start, count) in EXT_RANGES_egl.iter() {\n\
         \x20           if egl.ext[ei as usize] {\n\
         \x20               unsafe { egl.load_range(loader, start, count) };\n\
         \x20           }\n\
         \x20       }\n\
         \x20       egl.resolve_aliases();\n\
         \x20       Ok(egl)\n\
         \x20   }\n\n\
         \x20   #[inline]\n\
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
         \x20   }\n\n\
         \x20   /// Hash every space-separated extension name in the client string\n\
         \x20   /// — and the display string, for a real display — and flag matches\n\
         \x20   /// against the pre-baked table.  Mirrors the C loader's failure\n\
         \x20   /// rules: a null client string, or a null display string when\n\
         \x20   /// `display` is real, fails the load.\n\
         \x20   unsafe fn detect_extensions(&mut self, display: EGLDisplay) -> Result<(), LoadError> {\n\
         \x20       let client = unsafe { self.QueryString(EGL_NO_DISPLAY, EGL_EXTENSIONS as EGLint) };\n\
         \x20       if client.is_null() {\n\
         \x20           return Err(LoadError::MissingExtensionString);\n\
         \x20       }\n\
         \x20       unsafe { self.hash_ext_words(client) };\n\
         \x20       if display != EGL_NO_DISPLAY {\n\
         \x20           let disp = unsafe { self.QueryString(display, EGL_EXTENSIONS as EGLint) };\n\
         \x20           if disp.is_null() {\n\
         \x20               return Err(LoadError::MissingExtensionString);\n\
         \x20           }\n\
         \x20           unsafe { self.hash_ext_words(disp) };\n\
         \x20       }\n\
         \x20       Ok(())\n\
         \x20   }\n\n\
         \x20   /// Tokenize a NUL-terminated, space-separated extension list and\n\
         \x20   /// set the flag for every known name (XXH3 + binary search — the\n\
         \x20   /// same pre-baked hashes the C loader uses).\n\
         \x20   unsafe fn hash_ext_words(&mut self, p: *const c_char) {\n\
         \x20       let bytes = unsafe { CStr::from_ptr(p) }.to_bytes();\n\
         \x20       for word in bytes.split(|&b| b == b' ') {\n\
         \x20           if word.is_empty() {\n\
         \x20               continue;\n\
         \x20           }\n\
         \x20           let h = xxhash_rust::xxh3::xxh3_64(word);\n\
         \x20           if let Ok(pos) = EXT_HASH_KEYS.binary_search(&h) {\n\
         \x20               self.ext[EXT_HASH_IDX[pos] as usize] = true;\n\
         \x20           }\n\
         \x20       }\n\
         \x20   }\n",
    );

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
        "\n    /// Detected EGL version, packed as `major << 8 | minor`.\n\
         \x20   #[inline]\n\
         \x20   pub fn version(&self) -> u32 {\n\
         \x20       self.version\n\
         \x20   }\n",
    );

    s.push_str(
        "\n    // Dispatch wrappers.  The pointer local is named `__pfn` because\n\
         \x20   // parameter names could otherwise collide with it.\n",
    );
    let safety = "The context must be loaded; see [`Egl::load_egl`].";
    let mut used: std::collections::HashSet<String> = std::collections::HashSet::new();
    for cmd in &fs.commands {
        if used.insert(cmd.short_name.clone()) {
            s.push('\n');
            s.push_str(&emit_method(cmd, safety, &rust_type));
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
    s.push_str("}\n\n");

    // Version-string parser: "1.5 (ANGLE ...)" -> 0x0105.
    s.push_str(
        "/// Parse an EGL_VERSION string (`\"1.5 (ANGLE ...)\"`) into a packed\n\
         /// `major << 8 | minor`, or 0 if unparseable.\n\
         fn __parse_egl_version(p: *const c_char) -> u32 {\n\
         \x20   if p.is_null() {\n\
         \x20       return 0;\n\
         \x20   }\n\
         \x20   let bytes = unsafe { CStr::from_ptr(p) }.to_bytes();\n\
         \x20   let mut it = bytes.iter().copied().peekable();\n\
         \x20   let mut major: u32 = 0;\n\
         \x20   while let Some(c) = it.peek().copied().filter(|c| c.is_ascii_digit()) {\n\
         \x20       major = major * 10 + (c - b'0') as u32;\n\
         \x20       it.next();\n\
         \x20   }\n\
         \x20   let mut minor: u32 = 0;\n\
         \x20   if it.peek() == Some(&b'.') {\n\
         \x20       it.next();\n\
         \x20       while let Some(c) = it.peek().copied().filter(|c| c.is_ascii_digit()) {\n\
         \x20           minor = minor * 10 + (c - b'0') as u32;\n\
         \x20           it.next();\n\
         \x20       }\n\
         \x20   }\n\
         \x20   (major << 8) | minor\n\
         }\n",
    );
}
