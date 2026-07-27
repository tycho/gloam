//! The type/ABI machinery shared by every spec's Rust emission: the layer-1
//! ABI scalar map, the `TypeRef` → Rust type printers, and the derived
//! base-type emitter (`emit_base_types`) that turns the registry's structured
//! type payloads into aliases, callbacks, opaque handles, and platform types.

use std::fmt::Write as _;

use anyhow::{Result, bail};

use crate::parse::ctype::TypeRef;
use crate::resolve::FeatureSet;

/// Layer-1 ABI map: C scalar spellings — and the `khronos_*` aliases, whose
/// definitions live in khrplatform.h, outside any registry — to Rust.  This
/// is the one deliberately hand-maintained table: it encodes platform-ABI
/// facts no XML records.  Everything above it (the GL type aliases,
/// callbacks, opaque handles) is derived from the registry's structured
/// payloads, so an unknown base name here is a hard generation error rather
/// than a silently wrong hand-copied alias.
pub(super) fn abi_scalar(base: &str) -> Option<&'static str> {
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
        // `unsigned long` is genuinely platform-dependent (32-bit on LLP64
        // Windows, 64-bit on LP64 unix); c_ulong is Rust's spelling of
        // exactly that.  WGL's NV video counters and GLX's X11 types use it.
        "unsigned long" => "core::ffi::c_ulong",
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
        // `struct`-keyword base: when the tag names a type this module
        // defines (WGL's `typedef struct _GPU_DEVICE GPU_DEVICE`, whose
        // `_GPU_DEVICE` is a real emitted struct), use it directly;
        // otherwise it's a pointer to an incomplete struct (e.g.
        // `struct __GLsync *`) — synthesize an opaque type so the pointer
        // stays distinct.
        if !defined.contains(ty.base.as_str()) {
            opaque.insert(ty.base.clone());
        }
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
pub(super) fn emit_base_types(
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
                // windows.h-style handles: `DECLARE_HANDLE(NAME);` declares
                // `struct NAME__ *NAME`.  Mirror it exactly — a synthesized
                // opaque tag plus the pointer alias — so each handle stays a
                // distinct pointer type, as in C.
                if let Some(name) = t
                    .raw_c
                    .trim()
                    .strip_prefix("DECLARE_HANDLE(")
                    .and_then(|r| r.strip_suffix(");"))
                    .map(str::trim)
                    .filter(|n| n.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'))
                {
                    opaque.insert(format!("{name}__"));
                    let _ = writeln!(s, "pub type {name} = *mut {name}__;");
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
            // Inline record definitions (EGLClientPixmapHI, WGL's GPU_DEVICE,
            // GLX's event records): #[repr(C)] structs — or unions (GLX's
            // GLXEvent).  Union-ness comes from the raw text: these are
            // text-form registry entries with no category attribute, and the
            // payload was built from the same fragment, so the two views
            // cannot drift.
            TypePayload::Members(members) => {
                let kw = if t.raw_c.trim_start().starts_with("typedef union") {
                    "union"
                } else {
                    "struct"
                };
                let _ = writeln!(
                    s,
                    "#[repr(C)]\n#[derive(Copy, Clone)]\npub {kw} {} {{",
                    t.name
                );
                for m in members {
                    if m.ty.bitfield.is_some() {
                        bail!(
                            "{kw} '{}' member '{}': bitfields are not supported \
                             in GL-family records",
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

/// Print a `TypeRef` (`Param.ty` / `Command.return_ty`) as a Rust type,
/// relying on the emitted base-type aliases and newtypes for the base name.
/// Pointer mutability is per-level: each Rust pointer is `*const` exactly
/// when the thing it points at is const-qualified in the C declarator, so
/// `const GLchar *const*` prints `*const *const GLchar` while
/// `const GLcharARB **` prints `*mut *const GLcharARB`.
pub(super) fn rust_type(ty: &TypeRef) -> String {
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
pub(super) fn sanitize_ident(name: &str) -> String {
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
        // const T **: pointer-to-const under a MUTABLE outer pointer —
        // outer levels must not inherit the pointee's constness.
        assert_eq!(rt("const GLcharARB **"), "*mut *const GLcharARB");
        assert_eq!(rt("void **"), "*mut *mut c_void");
    }

    #[test]
    fn rust_type_array_param_decays() {
        assert_eq!(rt("const GLfloat coords[4]"), "*const GLfloat");
        assert_eq!(rt("GLuint baseAndCount[2]"), "*mut GLuint");
    }
}
