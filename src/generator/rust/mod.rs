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

mod egl_loader;
mod vk_loader;
mod vk_types;

use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Context, Result, bail};
use indexmap::IndexMap;

use super::GeneratedTree;
use crate::identity::Spec;
use crate::parse::ctype::TypeRef;
use crate::preamble;
use crate::provenance::load::SourceStore;
use crate::provenance::manifest::{OutputEntry, ProvenancePin, git_blob_sha1};
use crate::resolve::{Command, FeatureSet};

/// Layer-1 ABI map: C scalar spellings — and the `khronos_*` aliases, whose
/// definitions live in khrplatform.h, outside any registry — to Rust.  This
/// is the one deliberately hand-maintained table: it encodes platform-ABI
/// facts no XML records.  Everything above it (the GL type aliases,
/// callbacks, opaque handles) is derived from the registry's structured
/// payloads, so an unknown base name here is a hard generation error rather
/// than a silently wrong hand-copied alias.
fn abi_scalar(base: &str) -> Option<&'static str> {
    Some(match base {
        "void" => "c_void",
        "char" => "c_char",
        "unsigned char" => "u8",
        "short" => "i16",
        "unsigned short" => "u16",
        "int" => "i32",
        "unsigned int" => "u32",
        "float" => "f32",
        "double" => "f64",
        // `long` appears only inside the Apple-conditioned arm of the macOS
        // ptrdiff_t guard (LP64 there, so pointer-sized); ptrdiff_t is the
        // portable spelling of the same width.
        "long" => "isize",
        "ptrdiff_t" | "intptr_t" | "ssize_t" => "isize",
        "size_t" | "uintptr_t" => "usize",
        "int8_t" | "khronos_int8_t" => "i8",
        "uint8_t" | "khronos_uint8_t" => "u8",
        "int16_t" | "khronos_int16_t" => "i16",
        "uint16_t" | "khronos_uint16_t" => "u16",
        "int32_t" | "khronos_int32_t" => "i32",
        "uint32_t" | "khronos_uint32_t" => "u32",
        "int64_t" | "khronos_int64_t" => "i64",
        "uint64_t" | "khronos_uint64_t" => "u64",
        "khronos_float_t" => "f32",
        "khronos_intptr_t" | "khronos_ssize_t" => "isize",
        "khronos_uintptr_t" | "khronos_usize_t" => "usize",
        // Nanosecond-timestamp types (EGL_KHR_reusable_sync and friends).
        "khronos_utime_nanoseconds_t" => "u64",
        "khronos_stime_nanoseconds_t" => "i64",
        _ => return None,
    })
}

/// Map a C preprocessor condition (from a conditional typedef arm) to a Rust
/// `cfg` predicate.  Only conditions that actually occur in the registries
/// are mapped; anything new is a hard error so it gets a deliberate mapping
/// instead of a silent mistranslation.
fn cond_to_cfg(cond: &str) -> Result<&'static str> {
    if cond == "defined(__APPLE__)"
        || cond.contains("__ENVIRONMENT_MAC_OS_X_VERSION_MIN_REQUIRED__")
    {
        Ok("target_vendor = \"apple\"")
    } else {
        bail!("no cfg mapping for C preprocessor condition '{cond}'")
    }
}

/// Print a typedef-target `TypeRef` as a Rust type: base through the ABI map
/// (or verbatim when it names another emitted type), pointer levels wrapped
/// per constness.  `defined` is the full set of names this module defines;
/// `opaque` accumulates `struct`-keyword bases (incomplete C struct types)
/// that need a zero-sized `#[repr(C)]` struct synthesized; `platform`
/// accumulates names satisfied by the spec's platform-type table.
fn rust_target_type(
    ty: &TypeRef,
    defined: &std::collections::HashSet<&str>,
    opaque: &mut indexmap::IndexSet<String>,
    platform: &mut PlatformSweep<'_>,
) -> Result<String> {
    let base: String = if ty.struct_kw {
        // Pointer to an incomplete struct (e.g. `struct __GLsync *`): give
        // the tag a synthesized opaque type so the pointer stays distinct.
        opaque.insert(ty.base.clone());
        ty.base.clone()
    } else if let Some(scalar) = abi_scalar(&ty.base) {
        scalar.to_string()
    } else if defined.contains(ty.base.as_str()) || platform.record(&ty.base) {
        ty.base.clone()
    } else {
        bail!(
            "typedef target '{}' is neither an ABI scalar, a type defined \
             in this module, nor a known platform type",
            ty.base
        );
    };
    let mut t = base;
    for i in 0..ty.pointers.len() {
        let pointee_const = if i == 0 {
            ty.base_const
        } else {
            ty.pointers[i - 1]
        };
        t = format!("{}{t}", if pointee_const { "*const " } else { "*mut " });
    }
    Ok(t)
}

/// Collects the platform-table names a spec's emission actually references,
/// so only used declarations are emitted.
struct PlatformSweep<'a> {
    decl: &'a dyn Fn(&str) -> Option<&'static str>,
    used: indexmap::IndexSet<String>,
}

impl PlatformSweep<'_> {
    fn record(&mut self, name: &str) -> bool {
        if (self.decl)(name).is_some() {
            self.used.insert(name.to_string());
            true
        } else {
            false
        }
    }
}

/// Emit a spec's base-type aliases, callback function-pointer types, opaque
/// handle structs, and referenced platform types — derived from the
/// registry's structured type payloads (`TypeDef.payload`), not
/// hand-maintained.  Only the ABI scalar map and the per-spec platform
/// table are hand-written; everything else follows the spec, including
/// platform-conditional typedefs (GLhandleARB is `void *` on Apple
/// platforms and `unsigned int` elsewhere — a distinction a hand-copied
/// table misses).  `newtype_skip` names typedefs replaced by hand-emitted
/// newtypes (GL's GLenum/GLbitfield); `platform_decl` maps platform type
/// names (EGL's native window/display types) to their declarations.
fn emit_base_types(
    fs: &FeatureSet,
    s: &mut String,
    newtype_skip: &[&str],
    platform_decl: &dyn Fn(&str) -> Option<&'static str>,
) -> Result<()> {
    use crate::ir::{TypeCategory, TypePayload};

    // Full set of names this module defines, for validating alias targets
    // that reference other spec types (e.g. GLvdpauSurfaceNV = GLintptr).
    let mut defined: std::collections::HashSet<&str> =
        fs.types.iter().map(|t| t.name.as_str()).collect();
    defined.extend(newtype_skip);

    let mut opaque: indexmap::IndexSet<String> = indexmap::IndexSet::new();
    let mut platform = PlatformSweep {
        decl: platform_decl,
        used: indexmap::IndexSet::new(),
    };
    let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();

    for t in &fs.types {
        if t.category == TypeCategory::Include || !seen.insert(t.name.as_str()) {
            continue;
        }
        match &t.payload {
            // Typedefs replaced by hand-emitted newtypes.
            TypePayload::Typedef(_) if newtype_skip.contains(&t.name.as_str()) => {}
            TypePayload::Typedef(arms) => {
                emit_typedef_arms(s, &t.name, arms, &defined, &mut opaque, &mut platform)?;
            }
            TypePayload::Funcpointer(sig) => {
                let params = sig
                    .params
                    .iter()
                    .map(|(_, ty)| rust_type(ty))
                    .collect::<Vec<_>>()
                    .join(", ");
                let ret = if sig.ret.is_void() {
                    String::new()
                } else {
                    format!(" -> {}", rust_type(&sig.ret))
                };
                let _ = writeln!(
                    s,
                    "pub type {} = Option<unsafe extern \"system\" fn({params}){ret}>;",
                    t.name
                );
            }
            TypePayload::Opaque => {
                // `#define PFN... PFN...PROC` aliases are C-header naming
                // shims for per-command function-pointer typedefs, a concept
                // this backend replaces with dispatch methods — skip them.
                // `#include` carriers (egl.xml's eglplatform entry has no
                // category attribute, so the Include filter above misses it)
                // are satisfied by the ABI map + platform table instead.
                if t.raw_c.starts_with("#define PFN") || t.raw_c.starts_with("#include") {
                    continue;
                }
                // Forward struct declarations (`struct _cl_context;`) become
                // synthesized opaque types below.  Anything else opaque is a
                // spec construct this backend has never seen: fail loudly.
                let trimmed = t.raw_c.trim();
                let fwd = trimmed
                    .strip_prefix("struct ")
                    .and_then(|r| r.strip_suffix(';'))
                    .map(str::trim)
                    .filter(|tag| tag.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'));
                match fwd {
                    Some(tag) => {
                        opaque.insert(tag.to_string());
                    }
                    None => bail!(
                        "type '{}' has no structured form for Rust emission \
                         (raw C: '{}')",
                        t.name,
                        t.raw_c
                    ),
                }
            }
            // Inline struct definitions (EGLClientPixmapHI, WGL's
            // GPU_DEVICE): plain #[repr(C)] structs.
            TypePayload::Members(members) => {
                let _ = writeln!(
                    s,
                    "#[repr(C)]\n#[derive(Copy, Clone)]\npub struct {} {{",
                    t.name
                );
                for m in members {
                    if m.ty.bitfield.is_some() {
                        bail!(
                            "struct '{}' member '{}': bitfields are not supported \
                             in GL-family structs",
                            t.name,
                            m.name
                        );
                    }
                    let mut field =
                        rust_target_type(&m.ty, &defined, &mut opaque, &mut platform)
                            .map_err(|e| e.context(format!("{} member '{}'", t.name, m.name)))?;
                    for dim in m.ty.array.iter().rev() {
                        field = format!("[{field}; {dim}]");
                    }
                    let _ = writeln!(s, "    pub {}: {field},", sanitize_ident(&m.name));
                }
                s.push_str("}\n");
            }
            TypePayload::Handle { .. } => bail!(
                "type '{}': unexpected handle payload in a GL-family build",
                t.name
            ),
        }
    }

    // Commands can reference platform types the typedefs never mention
    // (EGL's native display/window/pixmap parameters).
    for cmd in &fs.commands {
        for ty in cmd.params.iter().map(|p| &p.ty).chain([&cmd.return_ty]) {
            if !ty.struct_kw
                && abi_scalar(&ty.base).is_none()
                && !defined.contains(ty.base.as_str())
            {
                platform.record(&ty.base);
            }
        }
    }

    if !opaque.is_empty() {
        s.push_str(
            "\n// Opaque C struct types (incomplete in the spec).  Zero-sized so\n\
             // pointers to them stay distinct types, exactly as in C.\n",
        );
        for tag in &opaque {
            let _ = writeln!(
                s,
                "#[repr(C)]\npub struct {tag} {{\n    _opaque: [u8; 0],\n}}"
            );
        }
    }
    if !platform.used.is_empty() {
        s.push_str(
            "\n// Platform types (in C these come from platform headers via\n\
             // khrplatform/eglplatform); declared here with their per-target\n\
             // ABI shapes.\n",
        );
        for name in &platform.used {
            let _ = writeln!(s, "{}", (platform.decl)(name).unwrap());
        }
    }
    Ok(())
}

/// Emit one derived type alias, collapsing preprocessor arms whose Rust
/// mapping agrees (the macOS ptrdiff_t guard: `long` and `ptrdiff_t` are
/// both `isize`) and emitting a `cfg`/`cfg(not)` pair when they differ
/// (GLhandleARB).
fn emit_typedef_arms(
    s: &mut String,
    name: &str,
    arms: &[crate::ir::TypedefArm],
    defined: &std::collections::HashSet<&str>,
    opaque: &mut indexmap::IndexSet<String>,
    platform: &mut PlatformSweep<'_>,
) -> Result<()> {
    let targets = arms
        .iter()
        .map(|arm| rust_target_type(&arm.ty, defined, opaque, platform))
        .collect::<Result<Vec<_>>>()?;

    if targets.windows(2).all(|w| w[0] == w[1]) {
        let _ = writeln!(s, "pub type {name} = {};", targets[0]);
        return Ok(());
    }
    // Arms differ: a guarded arm followed by its #else arm.
    let [ref guarded, ref fallback] = targets[..] else {
        bail!("typedef '{name}' has {} arms; expected 2", targets.len());
    };
    let Some(cond) = arms[0].condition.as_deref() else {
        bail!("typedef '{name}': first arm of a differing pair has no condition");
    };
    let cfg = cond_to_cfg(cond)?;
    let _ = writeln!(s, "#[cfg({cfg})]\npub type {name} = {guarded};");
    let _ = writeln!(s, "#[cfg(not({cfg}))]\npub type {name} = {fallback};");
    Ok(())
}

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
    if !matches!(fs.spec, Spec::Gl | Spec::Vk | Spec::Egl) {
        bail!(
            "the rust backend currently supports the GL/GLES, Vulkan, and EGL \
             APIs; {} is not implemented yet",
            fs.display_name
        );
    }
    if fs.spec != Spec::Gl && args.mx_global {
        bail!(
            "--mx-global is only implemented for the GL rust backend so far \
             (requested for {})",
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
    let body = match fs.spec {
        Spec::Vk => normalize_eof(vk_loader::emit_vk_module(fs, &preamble)?),
        Spec::Egl => normalize_eof(egl_loader::emit_egl_module(fs, &preamble)?),
        _ => normalize_eof(emit_module(fs, &preamble, args.mx_global)?),
    };

    // Emit a self-contained crate: Cargo.toml (pins the edition + the
    // xxhash-rust dependency) plus src/lib.rs (the loader).
    let crate_name = format!("gloam_{stem}");
    // One cargo feature per platform-protection macro appearing anywhere in
    // the build (sorted for determinism); the guarded items carry matching
    // #[cfg(feature = ...)] attributes.  Vulkan-only: the GL backend does
    // not emit protection guards.
    let mut platform_features: Vec<String> = if fs.spec == Spec::Vk {
        fs.types
            .iter()
            .flat_map(|t| t.protect.0.iter())
            .chain(fs.extensions.iter().flat_map(|e| e.protect.0.iter()))
            .chain(fs.commands.iter().flat_map(|c| c.protect.0.iter()))
            .cloned()
            .collect()
    } else {
        Vec::new()
    };
    platform_features.sort_unstable();
    platform_features.dedup();
    let cargo_toml = emit_cargo_toml(&crate_name, &platform_features);
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
/// for extension-name hashing.  `platform_features` declares one cargo feature
/// per platform-protection macro (the Rust analogue of Vulkan's
/// `VK_USE_PLATFORM_*` defines); empty for the GL family.
fn emit_cargo_toml(crate_name: &str, platform_features: &[String]) -> String {
    let mut features = String::new();
    if !platform_features.is_empty() {
        features.push_str(
            "# Platform-windowing surface opt-ins, mirroring the C headers'\n\
             # VK_USE_PLATFORM_* defines.\n",
        );
        for f in platform_features {
            let _ = writeln!(features, "{f} = []");
        }
    }
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
         {features}\
         \n\
         [dependencies]\n\
         xxhash-rust = {{ version = \"0.8\", features = [\"xxh3\"] }}\n"
    )
}

/// Emit the generated loader crate's `rustfmt.toml`. This file directs rustfmt
/// to not re-format the generated lib.rs code.
fn emit_rustfmt_toml() -> String {
    "# @generated by gloam.\n\
         format_generated_files = false\n"
        .to_string()
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

/// Emit the two `__missing` variants behind the `no-error` cargo feature: the
/// default panics by name (looked up in the existing name blob — no new data),
/// the `no-error` build collapses the dispatch match back to an unchecked call.
fn emit_missing_helpers(s: &mut String, api_label: &str) {
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
         unsafe fn __missing(_idx: usize) -> ! {\n",
    );
    let _ = write!(
        s,
        "    debug_assert!(false, \"unloaded {api_label} function called in a no-error build\");\n\
         \x20   unsafe {{ core::hint::unreachable_unchecked() }}\n\
         }}\n\n"
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

    Ok(())
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
/// `safety_doc` is the `# Safety` line; `map` prints each `TypeRef` in the
/// backend's spec-appropriate spelling.
fn emit_method(cmd: &Command, safety_doc: &str, map: &dyn Fn(&TypeRef) -> String) -> String {
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
            (name, map(&p.ty))
        })
        .collect();

    let ret = if cmd.return_ty.is_void() {
        None
    } else {
        Some(map(&cmd.return_ty))
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
         \x20   /// {safety_doc}\n\
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
            (name, rust_type(&p.ty))
        })
        .collect();

    let ret = if cmd.return_ty.is_void() {
        None
    } else {
        Some(rust_type(&cmd.return_ty))
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

/// Print a `TypeRef` (`Param.ty` / `Command.return_ty`) as a Rust type,
/// relying on the emitted base-type aliases and newtypes for the base name.
/// Pointer mutability is per-level: each Rust pointer is `*const` exactly
/// when the thing it points at is const-qualified in the C declarator, so
/// `const GLchar *const*` prints `*const *const GLchar` while
/// `const GLcharARB **` prints `*mut *const GLcharARB`.
fn rust_type(ty: &TypeRef) -> String {
    // Spec alias names pass through; raw C spellings map through the ABI
    // table.  Mapping matters for correctness, not just style: a bare C
    // `char` passed through verbatim would name RUST's 4-byte char and
    // silently break the ABI (EGL_MESA_image_dma_buf_export uses `int *`
    // and eglQueryString returns `const char *`).
    let base = match ty.base.as_str() {
        "GLvoid" => "c_void",
        other => abi_scalar(other).unwrap_or(other),
    };
    let mut t = base.to_string();
    // Inner array dimensions (all but the outermost) survive parameter decay
    // as Rust array layers.  No GL command uses them today; the printer stays
    // total so a spec that grows one can't silently mistranslate.
    for dim in ty.array.iter().skip(1).rev() {
        t = format!("[{t}; {dim}]");
    }
    for i in 0..ty.pointers.len() {
        let pointee_const = if i == 0 {
            ty.base_const
        } else {
            ty.pointers[i - 1]
        };
        t = format!("{}{t}", if pointee_const { "*const " } else { "*mut " });
    }
    // A C array parameter decays to a pointer to its element type.
    if !ty.array.is_empty() {
        let elem_const = if ty.pointers.is_empty() {
            ty.base_const
        } else {
            *ty.pointers.last().unwrap()
        };
        t = format!("{}{t}", if elem_const { "*const " } else { "*mut " });
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

#[cfg(test)]
mod tests {
    use super::*;

    fn rt(decl: &str) -> String {
        rust_type(&TypeRef::parse(decl).unwrap())
    }

    #[test]
    fn rust_type_scalars_and_void() {
        assert_eq!(rt("GLenum"), "GLenum");
        assert_eq!(rt("void"), "c_void");
        assert_eq!(rt("GLvoid"), "c_void");
    }

    #[test]
    fn rust_type_single_pointers() {
        assert_eq!(rt("const GLubyte *"), "*const GLubyte");
        assert_eq!(rt("GLuint *"), "*mut GLuint");
        assert_eq!(rt("void *"), "*mut c_void");
        assert_eq!(rt("const void *"), "*const c_void");
    }

    #[test]
    fn rust_type_pointer_constness_is_per_level() {
        // const T *const*: both levels const.
        assert_eq!(rt("const GLchar *const*"), "*const *const GLchar");
        // const T **: pointer-to-const under a MUTABLE outer pointer — the
        // case the old single-const translator emitted as *const *const.
        assert_eq!(rt("const GLcharARB **"), "*mut *const GLcharARB");
        assert_eq!(rt("void **"), "*mut *mut c_void");
    }

    #[test]
    fn rust_type_array_param_decays() {
        assert_eq!(rt("const GLfloat coords[4]"), "*const GLfloat");
        assert_eq!(rt("GLuint baseAndCount[2]"), "*mut GLuint");
    }
}
