//! GLX loader emission for the Rust backend.
//!
//! The generated module mirrors the C backend's GLX contract:
//! `Glx::load_glx(display, screen, loader)` bootstraps `glXQueryVersion`,
//! queries the version and sets feature flags from it, loads **every** PFN
//! upfront, additionally marks features whose every PFN resolved, detects
//! extensions from the space-separated `glXQueryExtensionsString(display,
//! screen)` string, loads the enabled extension ranges, and resolves
//! aliases.
//!
//! One deliberate divergence: the C loader opens the default X display via
//! `XOpenDisplay` when handed NULL, which links libX11.  The generated Rust
//! crate is `#![no_std]` with no link dependencies, so `display` must be a
//! real connection (Rust callers get one from winit/x11rb/xlib anyway); a
//! null display fails with [`LoadError::NoDisplay`].
//!
//! The X11 types GLX references come from `<X11/Xlib.h>` in C; the generated
//! module has no headers, so the referenced subset is declared in the
//! platform table below (XIDs are `unsigned long`, connection/visual types
//! are opaque).  The GL scalars glx.xml borrows are plain aliases here — the
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

/// Declarations for the Xlib types GLX references.  XIDs are
/// `unsigned long` per Xlib (32-bit on ILP32, 64-bit on LP64 — `c_ulong`
/// is Rust's spelling of exactly that); the connection and visual types
/// are opaque, pointer-only structs.
fn glx_platform_decl(name: &str) -> Option<&'static str> {
    Some(match name {
        // Xlib scalars.
        "Bool" => "pub type Bool = i32;",
        "Status" => "pub type Status = i32;",
        // XIDs.
        "XID" => "pub type XID = core::ffi::c_ulong;",
        "Window" => "pub type Window = core::ffi::c_ulong;",
        "Pixmap" => "pub type Pixmap = core::ffi::c_ulong;",
        "Font" => "pub type Font = core::ffi::c_ulong;",
        "Colormap" => "pub type Colormap = core::ffi::c_ulong;",
        // Opaque Xlib records (pointer-only in the GLX API surface; the
        // real definitions belong to Xlib bindings).
        "Display" => "#[repr(C)]\npub struct Display {\n    _opaque: [u8; 0],\n}",
        "Screen" => "#[repr(C)]\npub struct Screen {\n    _opaque: [u8; 0],\n}",
        "XVisualInfo" => "#[repr(C)]\npub struct XVisualInfo {\n    _opaque: [u8; 0],\n}",
        // GL scalars (plain aliases; see the module doc).  GLintptr and
        // GLsizeiptr are absent here deliberately: glx.xml carries its own
        // guarded typedefs for them, so they are derived, not declared.
        "GLenum" => "pub type GLenum = u32;",
        "GLbitfield" => "pub type GLbitfield = u32;",
        "GLboolean" => "pub type GLboolean = u8;",
        "GLfloat" => "pub type GLfloat = f32;",
        "GLint" => "pub type GLint = i32;",
        "GLsizei" => "pub type GLsizei = i32;",
        "GLubyte" => "pub type GLubyte = u8;",
        "GLuint" => "pub type GLuint = u32;",
        _ => return None,
    })
}

/// Build the whole GLX module body (everything after the preamble).
pub(super) fn emit_glx_module(fs: &FeatureSet, preamble: &str) -> Result<String> {
    let mut s = String::with_capacity(512 * 1024);

    s.push_str(preamble);
    s.push('\n');
    s.push_str(
        "#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, clippy::all)]\n\
         #![deny(unsafe_op_in_unsafe_fn)]\n\n\
         use core::ffi::{CStr, c_char, c_void};\n\n",
    );

    s.push_str("// ── GLX base types ──────────────────────────────────────────\n");
    emit_base_types(fs, &mut s, &[], &glx_platform_decl)?;
    s.push('\n');

    emit_constants(fs, &mut s)?;
    emit_command_tables(fs, &mut s);
    emit_extension_tables(fs, &mut s)?;
    emit_missing_helpers(&mut s, "GLX");
    emit_context(fs, &mut s);

    Ok(s)
}

/// Emit the flat constants.  GLX's `#define`s are untyped in C; here the
/// attrib-list values become `i32` (GLX attribute arrays are `int`/`const
/// int *`, so building one is cast-free), `<enums type="bitmask">` values
/// become `u32` (they feed `unsigned long`/mask parameters), and the
/// sentinels that don't fit `i32` (`GLX_DONT_CARE`) fall back to `u32`.
fn emit_constants(fs: &FeatureSet, s: &mut String) -> Result<()> {
    s.push_str("// ── Constants ───────────────────────────────────────────────\n");
    let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for e in &fs.flat_enums {
        if !seen.insert(e.name.as_str()) {
            continue;
        }
        let v = e.literal_value.trim();
        // String constants (GLX_EXTENSION_NAME): emit as &CStr, matching the
        // Vulkan printer's treatment of *_EXTENSION_NAME strings.
        if let Some(inner) = v.strip_prefix('"').and_then(|r| r.strip_suffix('"')) {
            let _ = writeln!(s, "pub const {}: &CStr = c\"{inner}\";", e.name);
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
            anyhow::bail!("unrecognized GLX constant literal '{v}' ({})", e.name);
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

/// Emit the `Glx` context struct, `load_glx`, extension detection over the
/// space-separated extensions string, dispatch methods, and presence queries.
fn emit_context(fs: &FeatureSet, s: &mut String) {
    let has_alias = !fs.alias_pairs.is_empty();
    emit_alias_pairs_table(fs, s);

    s.push_str(
        "// ── Context ─────────────────────────────────────────────────\n\
         /// Why [`Glx::load_glx`] failed.\n\
         #[derive(Copy, Clone, PartialEq, Eq, Debug)]\n\
         pub enum LoadError {\n\
         \x20   /// `display` was null.  (The C loader opens the default display\n\
         \x20   /// here, but that links libX11; this crate takes no link\n\
         \x20   /// dependencies, so the caller supplies the connection.)\n\
         \x20   NoDisplay,\n\
         \x20   /// `loader` returned null for `glXQueryVersion` — not a GLX\n\
         \x20   /// proc-address source.\n\
         \x20   MissingQueryVersion,\n\
         \x20   /// `glXQueryVersion` reported no usable version.\n\
         \x20   QueryVersionFailed,\n\
         \x20   /// `glXQueryExtensionsString` was unavailable or returned null.\n\
         \x20   MissingExtensionsString,\n\
         }\n\n\
         impl core::fmt::Display for LoadError {\n\
         \x20   fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {\n\
         \x20       f.write_str(match self {\n\
         \x20           LoadError::NoDisplay => \"display is null\",\n\
         \x20           LoadError::MissingQueryVersion => \"glXQueryVersion is not available\",\n\
         \x20           LoadError::QueryVersionFailed => \"glXQueryVersion reported no version\",\n\
         \x20           LoadError::MissingExtensionsString => \"GLX extensions string missing\",\n\
         \x20       })\n\
         \x20   }\n\
         }\n\n\
         impl core::error::Error for LoadError {}\n\n\
         /// Loaded GLX entry points plus detected feature/extension presence.\n\
         pub struct Glx {\n\
         \x20   pfns: [*const c_void; COMMAND_COUNT],\n\
         \x20   feat: [bool; FEATURE_COUNT],\n\
         \x20   ext: [bool; EXT_COUNT],\n\
         \x20   version: u32,\n\
         }\n\n\
         impl Glx {\n\
         \x20   /// Load GLX against `loader` (a glXGetProcAddress-style callback)\n\
         \x20   /// and detect the version, then extensions, for `display` +\n\
         \x20   /// `screen`.\n\
         \x20   ///\n\
         \x20   /// # Safety\n\
         \x20   /// `display` must be a valid X connection, `screen` a valid screen\n\
         \x20   /// number on it, and `loader` must yield pointers callable as the\n\
         \x20   /// named GLX functions.\n\
         \x20   #[inline]\n\
         \x20   pub unsafe fn load_glx(\n\
         \x20       display: *mut Display,\n\
         \x20       screen: i32,\n\
         \x20       mut loader: impl FnMut(&CStr) -> *const c_void,\n\
         \x20   ) -> Result<Self, LoadError> {\n\
         \x20       // Immediately erase to `&mut dyn` — the real loader is compiled\n\
         \x20       // once, not once per closure type.\n\
         \x20       unsafe { Self::load_glx_dyn(display, screen, &mut loader) }\n\
         \x20   }\n\n\
         \x20   unsafe fn load_glx_dyn(\n\
         \x20       display: *mut Display,\n\
         \x20       screen: i32,\n\
         \x20       loader: &mut dyn FnMut(&CStr) -> *const c_void,\n\
         \x20   ) -> Result<Self, LoadError> {\n\
         \x20       let mut glx = Self {\n\
         \x20           pfns: [core::ptr::null(); COMMAND_COUNT],\n\
         \x20           feat: [false; FEATURE_COUNT],\n\
         \x20           ext: [false; EXT_COUNT],\n\
         \x20           version: 0,\n\
         \x20       };\n\
         \x20       if display.is_null() {\n\
         \x20           return Err(LoadError::NoDisplay);\n\
         \x20       }\n",
    );

    let idx_qv = cmd_index(fs, "glXQueryVersion");
    let Some(qv) = idx_qv else {
        s.push_str("        Err(LoadError::MissingQueryVersion)\n    }\n}\n");
        return;
    };

    let _ = write!(
        s,
        "        glx.pfns[{qv}] = loader(c\"glXQueryVersion\");\n\
         \x20       if glx.pfns[{qv}].is_null() {{\n\
         \x20           return Err(LoadError::MissingQueryVersion);\n\
         \x20       }}\n\
         \x20       let (mut major, mut minor): (i32, i32) = (0, 0);\n\
         \x20       unsafe {{ glx.QueryVersion(display, &mut major, &mut minor) }};\n\
         \x20       glx.version = ((major as u32) << 8) | (minor as u32 & 0xff);\n\
         \x20       if glx.version == 0 {{\n\
         \x20           return Err(LoadError::QueryVersionFailed);\n\
         \x20       }}\n\
         \x20       // Feature presence from the queried version.\n"
    );
    for feat in &fs.features {
        let _ = writeln!(
            s,
            "        glx.feat[{idx}] = glx.version >= 0x{packed:04x};",
            idx = feat.index,
            packed = feat.packed
        );
    }
    s.push_str(
        "        // Load every PFN upfront, then additionally mark features whose\n\
         \x20       // every PFN resolved (set-only, mirroring the C loader).\n\
         \x20       unsafe { glx.load_range(loader, 0, COMMAND_COUNT as u16) };\n\
         \x20       for &(fi, start, count) in FEATURE_RANGES.iter() {\n\
         \x20           let mut ok = true;\n\
         \x20           for i in start..start + count {\n\
         \x20               ok &= !glx.pfns[i as usize].is_null();\n\
         \x20           }\n\
         \x20           if ok {\n\
         \x20               glx.feat[fi as usize] = true;\n\
         \x20           }\n\
         \x20       }\n\
         \x20       unsafe { glx.detect_extensions(display, screen)? };\n\
         \x20       for &(ei, start, count) in EXT_RANGES_glx.iter() {\n\
         \x20           if glx.ext[ei as usize] {\n\
         \x20               unsafe { glx.load_range(loader, start, count) };\n\
         \x20           }\n\
         \x20       }\n\
         \x20       glx.resolve_aliases();\n\
         \x20       Ok(glx)\n\
         \x20   }\n",
    );

    emit_load_range_method(s);
    emit_detect(fs, s);
    emit_hash_ext_words_method(s);
    emit_resolve_aliases_method(s, has_alias);

    s.push_str(
        "\n    /// Detected GLX version, packed as `major << 8 | minor`.\n\
         \x20   #[inline]\n\
         \x20   pub fn version(&self) -> u32 {\n\
         \x20       self.version\n\
         \x20   }\n",
    );

    s.push_str(
        "\n    // Dispatch wrappers.  The pointer local is named `__pfn` because\n\
         \x20   // parameter names could otherwise collide with it.\n",
    );
    let safety = "GLX must be loaded for a current display; see [`Glx::load_glx`].";
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
/// `glXQueryExtensionsString(display, screen)`; an unresolved entry point or
/// a null string fails the load (as in the C loader).
fn emit_detect(fs: &FeatureSet, s: &mut String) {
    let Some(qes) = cmd_index(fs, "glXQueryExtensionsString") else {
        s.push_str(
            "\n    /// Extension detection unavailable (glXQueryExtensionsString\n\
             \x20   /// absent from this build).\n\
             \x20   unsafe fn detect_extensions(&mut self, _display: *mut Display, _screen: i32) -> Result<(), LoadError> {\n\
             \x20       Err(LoadError::MissingExtensionsString)\n\
             \x20   }\n",
        );
        return;
    };
    let _ = write!(
        s,
        "\n    /// The space-separated extensions string, hashed word-by-word\n\
         \x20   /// against the pre-baked table.\n\
         \x20   unsafe fn detect_extensions(&mut self, display: *mut Display, screen: i32) -> Result<(), LoadError> {{\n\
         \x20       if self.pfns[{qes}].is_null() {{\n\
         \x20           return Err(LoadError::MissingExtensionsString);\n\
         \x20       }}\n\
         \x20       let ext_str = unsafe {{ self.QueryExtensionsString(display, screen) }};\n\
         \x20       if ext_str.is_null() {{\n\
         \x20           return Err(LoadError::MissingExtensionsString);\n\
         \x20       }}\n\
         \x20       unsafe {{ self.hash_ext_words(ext_str) }};\n\
         \x20       Ok(())\n\
         \x20   }}\n"
    );
}
