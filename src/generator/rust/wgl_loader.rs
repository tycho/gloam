//! WGL loader emission for the Rust backend.
//!
//! The generated module mirrors the C backend's WGL contract:
//! `Wgl::load_wgl(hdc, loader)` uses a faux 1.0 version (WGL has no version
//! query), loads **every** PFN upfront (availability, not version, gates
//! WGL), additionally marks features whose every PFN resolved, detects
//! extensions from the space-separated string (`wglGetExtensionsStringARB`
//! for the HDC, falling back to `wglGetExtensionsStringEXT`), loads the
//! enabled extension ranges, and resolves aliases.  The C loader's redundant
//! pre-load of the two extensions-string entry points is folded into the
//! load-all pass.
//!
//! Windows/GDI types come from `<windows.h>` in C; the generated module has
//! no headers, so the referenced subset is declared in the platform table
//! below with the documented Win32 ABI shapes.  The GL scalar types WGL
//! commands borrow (`GLenum`, `GLuint`, ...) are plain aliases here — the
//! `gl` module's newtypes belong to that module.

use std::fmt::Write as _;

use anyhow::Result;

use crate::resolve::FeatureSet;

use super::abi::{emit_base_types, rust_type};
use super::common::{
    cmd_index, emit_alias_pairs_table, emit_command_tables, emit_extension_tables, emit_flag_query,
    emit_hash_ext_words_method, emit_load_range_method, emit_method, emit_missing_helpers,
    emit_resolve_aliases_method, method_name,
};

/// Declarations for the `<windows.h>` types WGL references, with their
/// documented Win32 ABI shapes (handles are opaque pointers, matching
/// windows-sys), plus the GL scalars wgl.xml borrows from gl.xml.
fn wgl_platform_decl(name: &str) -> Option<&'static str> {
    Some(match name {
        // Win32 scalars.
        "BOOL" => "pub type BOOL = i32;",
        "CHAR" => "pub type CHAR = c_char;",
        "DWORD" => "pub type DWORD = u32;",
        "FLOAT" => "pub type FLOAT = f32;",
        "INT" => "pub type INT = i32;",
        "INT32" => "pub type INT32 = i32;",
        "INT64" => "pub type INT64 = i64;",
        "UINT" => "pub type UINT = u32;",
        "USHORT" => "pub type USHORT = u16;",
        "COLORREF" => "pub type COLORREF = u32;",
        // Win32 handles and pointers.
        "HANDLE" => "pub type HANDLE = *mut c_void;",
        "HDC" => "pub type HDC = *mut c_void;",
        "HGLRC" => "pub type HGLRC = *mut c_void;",
        "HENHMETAFILE" => "pub type HENHMETAFILE = *mut c_void;",
        "LPCSTR" => "pub type LPCSTR = *const c_char;",
        "LPVOID" => "pub type LPVOID = *mut c_void;",
        "PROC" => "pub type PROC = Option<unsafe extern \"system\" fn()>;",
        // GL scalars (plain aliases; see the module doc).
        "GLenum" => "pub type GLenum = u32;",
        "GLbitfield" => "pub type GLbitfield = u32;",
        "GLboolean" => "pub type GLboolean = u8;",
        "GLfloat" => "pub type GLfloat = f32;",
        "GLint" => "pub type GLint = i32;",
        "GLsizei" => "pub type GLsizei = i32;",
        "GLuint" => "pub type GLuint = u32;",
        "GLushort" => "pub type GLushort = u16;",
        // GDI structs, laid out per their Win32 documentation.
        "RECT" => {
            "#[repr(C)]\n\
                   #[derive(Copy, Clone)]\n\
                   pub struct RECT {\n\
                   \x20   pub left: i32,\n\
                   \x20   pub top: i32,\n\
                   \x20   pub right: i32,\n\
                   \x20   pub bottom: i32,\n\
                   }"
        }
        "PIXELFORMATDESCRIPTOR" => {
            "#[repr(C)]\n\
                   #[derive(Copy, Clone)]\n\
                   pub struct PIXELFORMATDESCRIPTOR {\n\
                   \x20   pub nSize: u16,\n\
                   \x20   pub nVersion: u16,\n\
                   \x20   pub dwFlags: u32,\n\
                   \x20   pub iPixelType: u8,\n\
                   \x20   pub cColorBits: u8,\n\
                   \x20   pub cRedBits: u8,\n\
                   \x20   pub cRedShift: u8,\n\
                   \x20   pub cGreenBits: u8,\n\
                   \x20   pub cGreenShift: u8,\n\
                   \x20   pub cBlueBits: u8,\n\
                   \x20   pub cBlueShift: u8,\n\
                   \x20   pub cAlphaBits: u8,\n\
                   \x20   pub cAlphaShift: u8,\n\
                   \x20   pub cAccumBits: u8,\n\
                   \x20   pub cAccumRedBits: u8,\n\
                   \x20   pub cAccumGreenBits: u8,\n\
                   \x20   pub cAccumBlueBits: u8,\n\
                   \x20   pub cAccumAlphaBits: u8,\n\
                   \x20   pub cDepthBits: u8,\n\
                   \x20   pub cStencilBits: u8,\n\
                   \x20   pub cAuxBuffers: u8,\n\
                   \x20   pub iLayerType: u8,\n\
                   \x20   pub bReserved: u8,\n\
                   \x20   pub dwLayerMask: u32,\n\
                   \x20   pub dwVisibleMask: u32,\n\
                   \x20   pub dwDamageMask: u32,\n\
                   }"
        }
        "LAYERPLANEDESCRIPTOR" => {
            "#[repr(C)]\n\
                   #[derive(Copy, Clone)]\n\
                   pub struct LAYERPLANEDESCRIPTOR {\n\
                   \x20   pub nSize: u16,\n\
                   \x20   pub nVersion: u16,\n\
                   \x20   pub dwFlags: u32,\n\
                   \x20   pub iPixelType: u8,\n\
                   \x20   pub cColorBits: u8,\n\
                   \x20   pub cRedBits: u8,\n\
                   \x20   pub cRedShift: u8,\n\
                   \x20   pub cGreenBits: u8,\n\
                   \x20   pub cGreenShift: u8,\n\
                   \x20   pub cBlueBits: u8,\n\
                   \x20   pub cBlueShift: u8,\n\
                   \x20   pub cAlphaBits: u8,\n\
                   \x20   pub cAlphaShift: u8,\n\
                   \x20   pub cAccumBits: u8,\n\
                   \x20   pub cAccumRedBits: u8,\n\
                   \x20   pub cAccumGreenBits: u8,\n\
                   \x20   pub cAccumBlueBits: u8,\n\
                   \x20   pub cAccumAlphaBits: u8,\n\
                   \x20   pub cDepthBits: u8,\n\
                   \x20   pub cStencilBits: u8,\n\
                   \x20   pub cAuxBuffers: u8,\n\
                   \x20   pub iLayerPlane: u8,\n\
                   \x20   pub bReserved: u8,\n\
                   \x20   pub crTransparent: COLORREF,\n\
                   }"
        }
        "LPGLYPHMETRICSFLOAT" => {
            "#[repr(C)]\n\
                   #[derive(Copy, Clone)]\n\
                   pub struct POINTFLOAT {\n\
                   \x20   pub x: f32,\n\
                   \x20   pub y: f32,\n\
                   }\n\
                   #[repr(C)]\n\
                   #[derive(Copy, Clone)]\n\
                   pub struct GLYPHMETRICSFLOAT {\n\
                   \x20   pub gmfBlackBoxX: f32,\n\
                   \x20   pub gmfBlackBoxY: f32,\n\
                   \x20   pub gmfptGlyphOrigin: POINTFLOAT,\n\
                   \x20   pub gmfCellIncX: f32,\n\
                   \x20   pub gmfCellIncY: f32,\n\
                   }\n\
                   pub type LPGLYPHMETRICSFLOAT = *mut GLYPHMETRICSFLOAT;"
        }
        _ => return None,
    })
}

/// Build the whole WGL module body (everything after the preamble).
pub(super) fn emit_wgl_module(fs: &FeatureSet, preamble: &str) -> Result<String> {
    let mut s = String::with_capacity(512 * 1024);

    s.push_str(preamble);
    s.push('\n');
    s.push_str(
        "#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, clippy::all)]\n\
         #![deny(unsafe_op_in_unsafe_fn)]\n\n\
         use core::ffi::{CStr, c_char, c_void};\n\n",
    );

    s.push_str("// ── WGL base types ──────────────────────────────────────────\n");
    emit_base_types(fs, &mut s, &[], &wgl_platform_decl)?;
    s.push('\n');

    emit_constants(fs, &mut s)?;
    emit_command_tables(fs, &mut s);
    emit_extension_tables(fs, &mut s)?;
    emit_missing_helpers(&mut s, "WGL");
    emit_context(fs, &mut s);

    Ok(s)
}

/// Emit the flat constants.  WGL's `#define`s are untyped in C; here the
/// attrib-list values become `i32` (WGL attribute arrays are `const int *`,
/// so building one is cast-free) and `<enums type="bitmask">` values become
/// `u32` (they feed `UINT`/`DWORD` flag parameters).
fn emit_constants(fs: &FeatureSet, s: &mut String) -> Result<()> {
    s.push_str("// ── Constants ───────────────────────────────────────────────\n");
    let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for e in &fs.flat_enums {
        if !seen.insert(e.name.as_str()) {
            continue;
        }
        let v = e.literal_value.trim();
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
            anyhow::bail!("unrecognized WGL constant literal '{v}' ({})", e.name);
        };
        let ty = if neg {
            "i32"
        } else if value > u32::MAX as u64 {
            "u64"
        } else if e.is_bitmask || value > i32::MAX as u64 {
            "u32"
        } else {
            "i32"
        };
        let _ = writeln!(s, "pub const {}: {ty} = {stripped};", e.name);
    }
    s.push('\n');
    Ok(())
}

/// Emit the `Wgl` context struct, `load_wgl`, extension detection over the
/// space-separated extensions string, dispatch methods, and presence queries.
fn emit_context(fs: &FeatureSet, s: &mut String) {
    let has_alias = !fs.alias_pairs.is_empty();
    emit_alias_pairs_table(fs, s);

    s.push_str(
        "// ── Context ─────────────────────────────────────────────────\n\
         /// Why [`Wgl::load_wgl`] failed.\n\
         #[derive(Copy, Clone, PartialEq, Eq, Debug)]\n\
         pub enum LoadError {\n\
         \x20   /// Neither `wglGetExtensionsStringARB` nor\n\
         \x20   /// `wglGetExtensionsStringEXT` produced an extension string —\n\
         \x20   /// `loader` is not a WGL proc-address source, or no context is\n\
         \x20   /// current on the HDC.\n\
         \x20   MissingExtensionsString,\n\
         }\n\n\
         impl core::fmt::Display for LoadError {\n\
         \x20   fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {\n\
         \x20       f.write_str(match self {\n\
         \x20           LoadError::MissingExtensionsString => \"WGL extensions string missing\",\n\
         \x20       })\n\
         \x20   }\n\
         }\n\n\
         impl core::error::Error for LoadError {}\n\n\
         /// Loaded WGL entry points plus detected feature/extension presence.\n\
         pub struct Wgl {\n\
         \x20   pfns: [*const c_void; COMMAND_COUNT],\n\
         \x20   feat: [bool; FEATURE_COUNT],\n\
         \x20   ext: [bool; EXT_COUNT],\n\
         \x20   version: u32,\n\
         }\n\n\
         impl Wgl {\n\
         \x20   /// Load WGL against `loader` (a wglGetProcAddress-style callback)\n\
         \x20   /// and detect extensions for `hdc`.\n\
         \x20   ///\n\
         \x20   /// WGL has no version query, so the version is a faux 1.0 and\n\
         \x20   /// every PFN is loaded upfront — availability, not version, gates\n\
         \x20   /// WGL entry points (as in the C loader).\n\
         \x20   ///\n\
         \x20   /// # Safety\n\
         \x20   /// A WGL context must be current on the calling thread, `hdc`\n\
         \x20   /// must be a valid device context, and `loader` must yield\n\
         \x20   /// pointers callable as the named WGL functions.\n\
         \x20   #[inline]\n\
         \x20   pub unsafe fn load_wgl(\n\
         \x20       hdc: HDC,\n\
         \x20       mut loader: impl FnMut(&CStr) -> *const c_void,\n\
         \x20   ) -> Result<Self, LoadError> {\n\
         \x20       // Immediately erase to `&mut dyn` — the real loader is compiled\n\
         \x20       // once, not once per closure type.\n\
         \x20       unsafe { Self::load_wgl_dyn(hdc, &mut loader) }\n\
         \x20   }\n\n\
         \x20   unsafe fn load_wgl_dyn(\n\
         \x20       hdc: HDC,\n\
         \x20       loader: &mut dyn FnMut(&CStr) -> *const c_void,\n\
         \x20   ) -> Result<Self, LoadError> {\n\
         \x20       let mut wgl = Self {\n\
         \x20           pfns: [core::ptr::null(); COMMAND_COUNT],\n\
         \x20           feat: [false; FEATURE_COUNT],\n\
         \x20           ext: [false; EXT_COUNT],\n\
         \x20           version: 0x0100, // faux WGL 1.0\n\
         \x20       };\n\
         \x20       // Feature presence from the faux version.\n",
    );
    for feat in &fs.features {
        let _ = writeln!(
            s,
            "        wgl.feat[{idx}] = wgl.version >= 0x{packed:04x};",
            idx = feat.index,
            packed = feat.packed
        );
    }
    s.push_str(
        "        // Load every PFN upfront, then additionally mark features whose\n\
         \x20       // every PFN resolved (set-only, mirroring the C loader).\n\
         \x20       unsafe { wgl.load_range(loader, 0, COMMAND_COUNT as u16) };\n\
         \x20       for &(fi, start, count) in FEATURE_RANGES.iter() {\n\
         \x20           let mut ok = true;\n\
         \x20           for i in start..start + count {\n\
         \x20               ok &= !wgl.pfns[i as usize].is_null();\n\
         \x20           }\n\
         \x20           if ok {\n\
         \x20               wgl.feat[fi as usize] = true;\n\
         \x20           }\n\
         \x20       }\n\
         \x20       unsafe { wgl.detect_extensions(hdc)? };\n\
         \x20       for &(ei, start, count) in EXT_RANGES_wgl.iter() {\n\
         \x20           if wgl.ext[ei as usize] {\n\
         \x20               unsafe { wgl.load_range(loader, start, count) };\n\
         \x20           }\n\
         \x20       }\n\
         \x20       wgl.resolve_aliases();\n\
         \x20       Ok(wgl)\n\
         \x20   }\n",
    );

    emit_load_range_method(s);
    emit_detect(fs, s);
    emit_hash_ext_words_method(s);
    emit_resolve_aliases_method(s, has_alias);

    s.push_str(
        "\n    /// The faux WGL version (always `1 << 8 | 0` — WGL has no version\n\
         \x20   /// query; capability lives in the extension flags).\n\
         \x20   #[inline]\n\
         \x20   pub fn version(&self) -> u32 {\n\
         \x20       self.version\n\
         \x20   }\n",
    );

    s.push_str(
        "\n    // Dispatch wrappers.  The pointer local is named `__pfn` because\n\
         \x20   // parameter names could otherwise collide with it.\n",
    );
    let safety = "A WGL context must be loaded and current; see [`Wgl::load_wgl`].";
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
    s.push_str("}\n");
}

/// Emit `detect_extensions`: the extensions string from
/// `wglGetExtensionsStringARB(hdc)`, falling back to
/// `wglGetExtensionsStringEXT()`; null from both fails the load (as in the C
/// loader).  Each entry point is consulted only if it resolved.
fn emit_detect(fs: &FeatureSet, s: &mut String) {
    let idx_arb = cmd_index(fs, "wglGetExtensionsStringARB");
    let idx_ext = cmd_index(fs, "wglGetExtensionsStringEXT");

    s.push_str(
        "\n    /// The space-separated extensions string, hashed word-by-word\n\
         \x20   /// against the pre-baked table.\n\
         \x20   unsafe fn detect_extensions(&mut self, hdc: HDC) -> Result<(), LoadError> {\n\
         \x20       let mut ext_str: *const c_char = core::ptr::null();\n",
    );
    if let Some(arb) = idx_arb {
        let _ = write!(
            s,
            "        if !self.pfns[{arb}].is_null() {{\n\
             \x20           ext_str = unsafe {{ self.GetExtensionsStringARB(hdc) }};\n\
             \x20       }}\n"
        );
    } else {
        s.push_str("        let _ = hdc;\n");
    }
    if let Some(ext) = idx_ext {
        let _ = write!(
            s,
            "        if ext_str.is_null() && !self.pfns[{ext}].is_null() {{\n\
             \x20           ext_str = unsafe {{ self.GetExtensionsStringEXT() }};\n\
             \x20       }}\n"
        );
    }
    s.push_str(
        "        if ext_str.is_null() {\n\
         \x20           return Err(LoadError::MissingExtensionsString);\n\
         \x20       }\n\
         \x20       unsafe { self.hash_ext_words(ext_str) };\n\
         \x20       Ok(())\n\
         \x20   }\n",
    );
}
