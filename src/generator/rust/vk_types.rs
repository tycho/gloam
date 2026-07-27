//! Vulkan type emission for the Rust backend (RT-4 of the type-IR plan).
//!
//! Prints the resolved Vulkan type surface — API constants, typed enum
//! groups, handles, typedefs, function pointers, structs and unions — as
//! Rust, from the structured `TypePayload`s and `EnumGroup`s the resolver
//! carries.  Layout fidelity comes from `#[repr(C)]`/`#[repr(transparent)]`
//! over ABI-exact field types; the only hand-maintained inputs are the
//! scalar ABI map (shared with the GL backend) and the platform-handle
//! table below, both of which encode facts no registry records.
//!
//! Not yet wired into `generate` — the Vulkan loader phase (RT-6) turns it
//! on.  Until then the unit tests below drive it over the bundled vk.xml.
//!
//! Deliberate scope limits:
//! - Enum groups follow the C header's semantics: non-bitmask groups are
//!   `#[repr(transparent)]` newtypes (an unknown driver value is data, not
//!   UB — never a Rust `enum`); bitmask `*FlagBits` groups are aliases of
//!   their `*Flags` carrier with free constants, so flags combine with `|`
//!   and assign to struct fields exactly as in C.
//! - Bitfield members merge into a carrier field with accessor methods
//!   (only the acceleration-structure instance structs use them).
//! - Video-codec structs reference `StdVideo*` types that live in the
//!   vk_video headers, outside vk.xml; selecting them is a hard error until
//!   those headers get structured parsing.

// The printer is exercised by its tests until the VK loader (RT-6) calls
// `emit_vk_types` from `generate`; lib builds see it as dead until then.
#![allow(dead_code)]

use std::collections::HashSet;
use std::fmt::Write as _;

use anyhow::{Result, bail};
use indexmap::IndexSet;

use super::abi_scalar;
use crate::ir::{RawMember, TypeCategory, TypePayload, TypedefArm};
use crate::parse::ctype::TypeRef;
use crate::resolve::{EnumGroup, FeatureSet, TypeDef};

// ---------------------------------------------------------------------------
// Platform handle table
// ---------------------------------------------------------------------------

/// Rust declarations for the platform types Vulkan's WSI surface references.
/// In C these come from `#include <windows.h>` etc.; the generated crate has
/// no such headers, so the ABI-relevant shape of each is declared here.
/// Pointer-shaped handles are opaque-struct pointers or `*mut c_void`
/// aliases; X11's XID family is `unsigned long` (`c_ulong`).  Only names the
/// selected feature set actually references get emitted.
fn platform_type_decl(name: &str) -> Option<&'static str> {
    Some(match name {
        // windows.h
        "HANDLE" => "pub type HANDLE = *mut c_void;",
        "HINSTANCE" => "pub type HINSTANCE = *mut c_void;",
        "HWND" => "pub type HWND = *mut c_void;",
        "HMONITOR" => "pub type HMONITOR = *mut c_void;",
        "DWORD" => "pub type DWORD = u32;",
        "LPCWSTR" => "pub type LPCWSTR = *const u16;",
        "SECURITY_ATTRIBUTES" => {
            "#[repr(C)]\npub struct SECURITY_ATTRIBUTES {\n    \
             pub nLength: DWORD,\n    \
             pub lpSecurityDescriptor: *mut c_void,\n    \
             pub bInheritHandle: i32,\n}"
        }
        // Xlib / XRandR (XID family: unsigned long)
        "Display" => "pub enum Display {}",
        "Window" => "pub type Window = core::ffi::c_ulong;",
        "VisualID" => "pub type VisualID = core::ffi::c_ulong;",
        "RROutput" => "pub type RROutput = core::ffi::c_ulong;",
        // xcb
        "xcb_connection_t" => "pub enum xcb_connection_t {}",
        "xcb_window_t" => "pub type xcb_window_t = u32;",
        "xcb_visualid_t" => "pub type xcb_visualid_t = u32;",
        // Wayland
        "wl_display" => "pub enum wl_display {}",
        "wl_surface" => "pub enum wl_surface {}",
        // Android
        "ANativeWindow" => "pub enum ANativeWindow {}",
        "AHardwareBuffer" => "pub enum AHardwareBuffer {}",
        // Apple (Metal / QuartzCore).  The `id<Protocol>` Obj-C types are
        // pointer-shaped.
        "CAMetalLayer" => "pub enum CAMetalLayer {}",
        "MTLDevice_id" => "pub type MTLDevice_id = *mut c_void;",
        "MTLCommandQueue_id" => "pub type MTLCommandQueue_id = *mut c_void;",
        "MTLBuffer_id" => "pub type MTLBuffer_id = *mut c_void;",
        "MTLTexture_id" => "pub type MTLTexture_id = *mut c_void;",
        "MTLSharedEvent_id" => "pub type MTLSharedEvent_id = *mut c_void;",
        "IOSurfaceRef" => "pub type IOSurfaceRef = *mut c_void;",
        // Fuchsia
        "zx_handle_t" => "pub type zx_handle_t = u32;",
        // DirectFB
        "IDirectFB" => "pub enum IDirectFB {}",
        "IDirectFBSurface" => "pub enum IDirectFBSurface {}",
        // Google Games Platform
        "GgpStreamDescriptor" => "pub type GgpStreamDescriptor = u32;",
        "GgpFrameToken" => "pub type GgpFrameToken = u64;",
        // QNX Screen
        "_screen_context" => "pub enum _screen_context {}",
        "_screen_window" => "pub enum _screen_window {}",
        "_screen_buffer" => "pub enum _screen_buffer {}",
        // OpenHarmony
        "NativeWindow" => "pub enum NativeWindow {}",
        "OHNativeWindow" => "pub type OHNativeWindow = NativeWindow;",
        "OH_NativeBuffer" => "pub enum OH_NativeBuffer {}",
        "OHBufferHandle" => "pub enum OHBufferHandle {}",
        // CoreFoundation / IOKit
        "__IOSurface" => "pub enum __IOSurface {}",
        // NvSciSync / NvSciBuf
        "NvSciSyncAttrList" => "pub type NvSciSyncAttrList = *mut c_void;",
        "NvSciSyncObj" => "pub type NvSciSyncObj = *mut c_void;",
        "NvSciSyncFence" => {
            "#[repr(C)]\npub struct NvSciSyncFence {\n    pub payload: [u64; 6],\n}"
        }
        "NvSciBufAttrList" => "pub type NvSciBufAttrList = *mut c_void;",
        "NvSciBufObj" => "pub type NvSciBufObj = *mut c_void;",
        _ => return None,
    })
}

// ---------------------------------------------------------------------------
// Base-type mapping
// ---------------------------------------------------------------------------

/// Map a declarator base for Vulkan printing: ABI scalars through the shared
/// map, anything else verbatim (validated against `defined` by the caller
/// where a wrong name would otherwise slip through).
pub(super) fn vk_base(base: &str) -> String {
    match abi_scalar(base) {
        Some(s) => s.to_string(),
        None => base.to_string(),
    }
}

/// Print a `TypeRef` as a Rust type in a struct-member/param position:
/// pointer levels wrap with per-level constness, array dimensions stay
/// arrays (`[T; N]`, outermost last to wrap first).
fn vk_type(ty: &TypeRef) -> String {
    let mut t = vk_base(&ty.base);
    for i in 0..ty.pointers.len() {
        let pointee_const = if i == 0 {
            ty.base_const
        } else {
            ty.pointers[i - 1]
        };
        t = format!("{}{t}", if pointee_const { "*const " } else { "*mut " });
    }
    for dim in ty.array.iter().rev() {
        t = format!("[{t}; {} as usize]", dim_expr(dim));
    }
    t
}

/// An array dimension as written: a number stays itself, an identifier is a
/// generated constant.
fn dim_expr(dim: &str) -> String {
    dim.to_string()
}

// ---------------------------------------------------------------------------
// Constants (flat enums: VK_UUID_SIZE, extension NAME/SPEC_VERSION, ...)
// ---------------------------------------------------------------------------

/// Translate one Vulkan `#define`-style constant literal into a Rust
/// `(type, expression)`.  The literal grammar is tiny and closed; anything
/// new is a hard error so it gets a deliberate mapping.
fn vk_const(literal: &str) -> Result<(String, String)> {
    let t = literal.trim();
    // String constants (extension names).
    if let Some(body) = t.strip_prefix('"').and_then(|r| r.strip_suffix('"')) {
        return Ok(("&core::ffi::CStr".to_string(), format!("c\"{body}\"")));
    }
    // Parenthesized complement forms: (~0U), (~1U), (~0ULL), (~0U-1), ...
    if let Some(inner) = t.strip_prefix("(~").and_then(|r| r.strip_suffix(')')) {
        let digits: String = inner.chars().take_while(char::is_ascii_digit).collect();
        let rest = &inner[digits.len()..];
        if !digits.is_empty() {
            return Ok(match rest {
                "U" => ("u32".to_string(), format!("!{digits}u32")),
                "ULL" => ("u64".to_string(), format!("!{digits}u64")),
                "U-1" => ("u32".to_string(), format!("!{digits}u32 - 1")),
                "U-2" => ("u32".to_string(), format!("!{digits}u32 - 2")),
                other => bail!("unrecognized Vulkan constant form '(~{digits}{other})'"),
            });
        }
    }
    // Float constants: 1000.0F.
    if let Some(num) = t.strip_suffix(['F', 'f'])
        && num.contains('.')
    {
        return Ok(("f32".to_string(), num.to_string()));
    }
    // Plain (possibly suffixed) integers: 16, 256U, 4294967295.
    let digits = t.trim_end_matches(['u', 'U', 'l', 'L']);
    if !digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit()) {
        return Ok(("u32".to_string(), digits.to_string()));
    }
    bail!("unrecognized Vulkan constant literal '{t}'")
}

/// Emit the flat API constants.  Constants used as array dimensions are
/// emitted as `u32` like the rest — array types cast them (`N as usize`),
/// keeping one definition per constant no matter how it's used.
fn emit_flat_consts(fs: &FeatureSet, s: &mut String) -> Result<()> {
    s.push_str("// ── API constants ───────────────────────────────────────────\n");
    let mut seen: HashSet<&str> = HashSet::new();
    for e in &fs.flat_enums {
        if !seen.insert(e.name.as_str()) {
            continue;
        }
        // Aliases of other constants (literal resolves to a non-literal
        // name) reference the canonical constant.
        if seen.contains(e.literal_value.as_str()) {
            let _ = writeln!(s, "pub const {}: u32 = {};", e.name, e.literal_value);
            continue;
        }
        let (ty, expr) = vk_const(&e.literal_value)?;
        let _ = writeln!(s, "pub const {}: {ty} = {expr};", e.name);
    }
    s.push('\n');
    Ok(())
}

// ---------------------------------------------------------------------------
// Enum groups
// ---------------------------------------------------------------------------

/// Emit the typed enum groups.  See the module doc for the newtype /
/// Flags-alias split.
fn emit_enum_groups(fs: &FeatureSet, s: &mut String) -> Result<()> {
    s.push_str("// ── Enum groups ─────────────────────────────────────────────\n");
    for g in &fs.enum_groups {
        if g.is_bitmask {
            emit_bitmask_group(g, s)?;
        } else {
            emit_enum_group(g, s)?;
        }
        s.push('\n');
    }
    Ok(())
}

/// A non-bitmask group (VkResult, VkFormat, ...): a `#[repr(transparent)]`
/// newtype over the C enum's ABI type plus free constants, so unknown driver
/// values are representable data.
fn emit_enum_group(g: &EnumGroup, s: &mut String) -> Result<()> {
    if g.bitwidth != 32 {
        bail!(
            "enum group '{}' is {}-bit but not a bitmask — no emission rule",
            g.name,
            g.bitwidth
        );
    }
    let _ = writeln!(
        s,
        "#[repr(transparent)]\n\
         #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]\n\
         pub struct {}(pub i32);",
        g.name
    );
    for v in &g.values {
        let _ = writeln!(
            s,
            "pub const {}: {} = {}({});",
            v.name, g.name, g.name, v.literal_value
        );
    }
    Ok(())
}

/// A bitmask group (`Vk*FlagBits`): an alias of the scalar its `Vk*Flags`
/// carrier wraps, with free constants of that type — combinable with `|`
/// and directly assignable to `Flags`-typed struct fields, as in C.  The
/// alias targets the scalar rather than the `Flags` typedef because the
/// typedef can sit behind a platform `cfg` (Wayland, Fuchsia, Metal) while
/// the group itself is unguarded.
fn emit_bitmask_group(g: &EnumGroup, s: &mut String) -> Result<()> {
    let scalar = match g.bitwidth {
        32 => "u32",
        64 => "u64",
        w => bail!("bitmask group '{}' has unsupported bitwidth {w}", g.name),
    };
    let _ = writeln!(s, "pub type {} = {scalar};", g.name);
    for v in &g.values {
        let lit = v
            .literal_value
            .trim_end_matches(['u', 'U', 'l', 'L'])
            .to_string();
        let _ = writeln!(s, "pub const {}: {} = {lit};", v.name, g.name);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Types (handles, typedefs, funcpointers, structs, unions)
// ---------------------------------------------------------------------------

/// Emit every selected `TypeDef` in topological order, collecting referenced
/// platform types for the trailing platform section.
pub(super) fn emit_vk_types(fs: &FeatureSet, s: &mut String) -> Result<()> {
    // Names defined somewhere in this module: types, enum groups, and the
    // platform table.  Used to validate typedef targets and struct fields.
    let mut defined: HashSet<&str> = fs.types.iter().map(|t| t.name.as_str()).collect();
    for g in &fs.enum_groups {
        defined.insert(g.name.as_str());
    }

    emit_flat_consts(fs, s)?;
    emit_enum_groups(fs, s)?;

    s.push_str("// ── Types ───────────────────────────────────────────────────\n");
    let mut platform: IndexSet<&str> = IndexSet::new();
    let mut seen: HashSet<&str> = HashSet::new();
    for t in &fs.types {
        if t.category == TypeCategory::Include || !seen.insert(t.name.as_str()) {
            continue;
        }
        let mut body = String::new();
        emit_type(t, &mut body, &defined, &mut platform)?;
        if body.is_empty() {
            continue;
        }
        push_guarded(s, &t.protect.0, &body);
    }

    // Commands reference platform types too (vkGetPhysicalDeviceXlib-
    // PresentationSupportKHR takes a VisualID); sweep their signatures so
    // the platform section covers the dispatch methods as well.
    for cmd in &fs.commands {
        for ty in cmd.params.iter().map(|p| &p.ty).chain([&cmd.return_ty]) {
            if platform_type_decl(&ty.base).is_some() {
                record_platform(&ty.base, &mut platform);
            }
        }
    }

    if !platform.is_empty() {
        s.push_str(
            "\n// ── Platform types ──────────────────────────────────────────\n\
             // ABI-shape declarations for the platform handles Vulkan's WSI\n\
             // surface references (in C these come from platform headers).\n",
        );
        for name in &platform {
            let _ = writeln!(s, "{}", platform_type_decl(name).unwrap());
        }
    }
    Ok(())
}

/// Wrap `body` in `#[cfg(feature = ...)]` attributes when the type carries
/// platform protection macros.  Each protected item is guarded individually;
/// Rust has no `#ifdef` block to coalesce into.
fn push_guarded(s: &mut String, protect: &[String], body: &str) {
    for line_group in body
        .split_inclusive('\n')
        .collect::<Vec<_>>()
        .split(|l| l.trim().is_empty())
    {
        if line_group.is_empty() {
            continue;
        }
        for p in protect {
            let _ = writeln!(s, "#[cfg(feature = \"{p}\")]");
        }
        for line in line_group {
            s.push_str(line);
        }
    }
}

fn emit_type(
    t: &TypeDef,
    s: &mut String,
    defined: &HashSet<&str>,
    platform: &mut IndexSet<&'static str>,
) -> Result<()> {
    // Platform types (forward declarations, #ifdef __OBJC__ bodies) are
    // declared by the platform table, whatever payload shape they parsed
    // into; record the reference and emit nothing here.
    if platform_type_decl(&t.name).is_some() {
        record_platform(&t.name, platform);
        return Ok(());
    }
    // Define-category machinery (VK_DEFINE_HANDLE, the VK_API_VERSION_*
    // macros, VK_NULL_HANDLE) is C preprocessor surface: handles are
    // emitted directly from their payloads, and the version helpers are
    // hand-written const fns in the crate prelude (RT-6).
    if t.category == TypeCategory::Define {
        return Ok(());
    }
    match &t.payload {
        TypePayload::Handle { dispatchable } => {
            // Dispatchable handles are pointers to opaque driver objects;
            // non-dispatchable are 64-bit values on every platform (the
            // C headers' VK_USE_64_BIT_PTR_DEFINES pointer option is an
            // opt-in we deliberately don't take: u64 is the one shape
            // that's ABI-correct everywhere).
            let inner = if *dispatchable { "*mut c_void" } else { "u64" };
            let _ = writeln!(
                s,
                "#[repr(transparent)]\n\
                 #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]\n\
                 pub struct {}(pub {inner});",
                t.name
            );
        }
        TypePayload::Typedef(arms) => emit_vk_typedef(t, arms, s, defined, platform)?,
        TypePayload::Funcpointer(sig) => {
            let params = sig
                .params
                .iter()
                .map(|(_, ty)| vk_type(ty))
                .collect::<Vec<_>>()
                .join(", ");
            let ret = if sig.ret.is_void() {
                String::new()
            } else {
                format!(" -> {}", vk_type(&sig.ret))
            };
            let _ = writeln!(
                s,
                "pub type {} = Option<unsafe extern \"system\" fn({params}){ret}>;",
                t.name
            );
        }
        TypePayload::Members(members) => emit_vk_struct(t, members, s, defined, platform)?,
        TypePayload::Opaque => {
            // `#define X Y` aliases (bitmask/handle/enum aliases whose
            // targets are typedef'd integers in C).  In Rust a type alias
            // covers every case.
            if let Some(rest) = t.raw_c.strip_prefix("#define ") {
                let mut it = rest.split_whitespace();
                if let (Some(name), Some(target), None) = (it.next(), it.next(), it.next())
                    && defined.contains(target)
                {
                    let _ = writeln!(s, "pub type {name} = {target};");
                    return Ok(());
                }
            }
            bail!(
                "Vulkan type '{}' has no structured form for Rust emission \
                 (raw C: '{}')",
                t.name,
                t.raw_c
            )
        }
    }
    Ok(())
}

fn emit_vk_typedef(
    t: &TypeDef,
    arms: &[TypedefArm],
    s: &mut String,
    defined: &HashSet<&str>,
    platform: &mut IndexSet<&'static str>,
) -> Result<()> {
    let [arm] = arms else {
        bail!(
            "Vulkan typedef '{}' has {} preprocessor arms; expected 1",
            t.name,
            arms.len()
        );
    };
    let target = check_base(&arm.ty, defined, platform)
        .map(|()| vk_type(&arm.ty))
        .map_err(|e| e.context(format!("typedef '{}'", t.name)))?;
    let _ = writeln!(s, "pub type {} = {target};", t.name);
    Ok(())
}

/// Validate a declarator base: an ABI scalar, a name defined in this module,
/// or a known platform type (recorded for the platform section).  Anything
/// else — e.g. the vk_video `StdVideo*` types, which live outside vk.xml —
/// is a hard error naming the base.
fn check_base(
    ty: &TypeRef,
    defined: &HashSet<&str>,
    platform: &mut IndexSet<&'static str>,
) -> Result<()> {
    if abi_scalar(&ty.base).is_some() {
        return Ok(());
    }
    if platform_type_decl(&ty.base).is_some() {
        record_platform(&ty.base, platform);
        return Ok(());
    }
    if defined.contains(ty.base.as_str()) {
        return Ok(());
    }
    if ty.base.starts_with("StdVideo") {
        bail!(
            "base type '{}' lives in the vk_video headers, which the Rust \
             backend cannot emit yet — exclude the video-codec extensions \
             via --extensions to generate this loader",
            ty.base
        );
    }
    bail!(
        "base type '{}' is not an ABI scalar, a defined Vulkan type, or a \
         known platform type",
        ty.base
    )
}

/// Intern a platform type name (the table's keys are 'static) and pull in
/// the platform types its own declaration references (OHNativeWindow →
/// NativeWindow).
fn record_platform(name: &str, platform: &mut IndexSet<&'static str>) {
    let key = PLATFORM_NAMES
        .iter()
        .find(|n| **n == name)
        .expect("platform_type_decl and PLATFORM_NAMES agree");
    if platform.insert(key) && *key == "OHNativeWindow" {
        record_platform("NativeWindow", platform);
    }
}

/// Names accepted by `platform_type_decl`, for interning.
const PLATFORM_NAMES: &[&str] = &[
    "HANDLE",
    "HINSTANCE",
    "HWND",
    "HMONITOR",
    "DWORD",
    "LPCWSTR",
    "SECURITY_ATTRIBUTES",
    "Display",
    "Window",
    "VisualID",
    "RROutput",
    "xcb_connection_t",
    "xcb_window_t",
    "xcb_visualid_t",
    "wl_display",
    "wl_surface",
    "ANativeWindow",
    "AHardwareBuffer",
    "CAMetalLayer",
    "MTLDevice_id",
    "MTLCommandQueue_id",
    "MTLBuffer_id",
    "MTLTexture_id",
    "MTLSharedEvent_id",
    "IOSurfaceRef",
    "zx_handle_t",
    "IDirectFB",
    "IDirectFBSurface",
    "GgpStreamDescriptor",
    "GgpFrameToken",
    "_screen_context",
    "_screen_window",
    "_screen_buffer",
    "NativeWindow",
    "OHNativeWindow",
    "OH_NativeBuffer",
    "OHBufferHandle",
    "__IOSurface",
    "NvSciSyncAttrList",
    "NvSciSyncObj",
    "NvSciSyncFence",
    "NvSciBufAttrList",
    "NvSciBufObj",
];

/// Emit a struct or union from its member records, merging bitfield runs
/// into carrier fields with accessor methods.
fn emit_vk_struct(
    t: &TypeDef,
    members: &[RawMember],
    s: &mut String,
    defined: &HashSet<&str>,
    platform: &mut IndexSet<&'static str>,
) -> Result<()> {
    let is_union = t.category == TypeCategory::Union;
    let kw = if is_union { "union" } else { "struct" };

    // Group members: a run of consecutive bitfield members becomes one
    // carrier field.
    enum Field<'a> {
        Plain(&'a RawMember),
        Packed(Vec<&'a RawMember>),
    }
    let mut fields: Vec<Field<'_>> = Vec::new();
    for m in members {
        check_base(&m.ty, defined, platform)
            .map_err(|e| e.context(format!("{} member '{}'", t.name, m.name)))?;
        if let Some(width) = m.ty.bitfield {
            if let Some(Field::Packed(run)) = fields.last_mut() {
                let used: u32 = run.iter().filter_map(|r| r.ty.bitfield).sum();
                let carrier_bits = 32; // all Vulkan bitfield runs are 32-bit
                if used + width <= carrier_bits {
                    run.push(m);
                    continue;
                }
            }
            fields.push(Field::Packed(vec![m]));
        } else {
            fields.push(Field::Plain(m));
        }
    }

    let _ = writeln!(
        s,
        "#[repr(C)]\n#[derive(Copy, Clone)]\npub {kw} {} {{",
        t.name
    );
    let mut accessors = String::new();
    for f in &fields {
        match f {
            Field::Plain(m) => {
                let _ = writeln!(
                    s,
                    "    pub {}: {},",
                    super::sanitize_ident(&m.name),
                    vk_type(&m.ty)
                );
            }
            Field::Packed(run) => {
                let field_name = run
                    .iter()
                    .map(|m| m.name.as_str())
                    .collect::<Vec<_>>()
                    .join("_and_");
                let _ = writeln!(s, "    pub {field_name}: u32,");
                let mut shift = 0u32;
                for m in run.iter() {
                    let width = m.ty.bitfield.unwrap();
                    let mask = if width == 32 {
                        u32::MAX
                    } else {
                        (1u32 << width) - 1
                    };
                    let _ = write!(
                        accessors,
                        "    /// Bits {shift}..{} of `{field_name}` (C bitfield `{}:{width}`).\n\
                         \x20   #[inline]\n\
                         \x20   pub fn {}(&self) -> u32 {{\n\
                         \x20       (self.{field_name} >> {shift}) & 0x{mask:x}\n\
                         \x20   }}\n\
                         \x20   #[inline]\n\
                         \x20   pub fn set_{}(&mut self, v: u32) {{\n\
                         \x20       self.{field_name} = (self.{field_name} & !(0x{mask:x} << {shift})) \
                         | ((v & 0x{mask:x}) << {shift});\n\
                         \x20   }}\n",
                        shift + width,
                        m.name,
                        m.name,
                        m.name,
                    );
                    shift += width;
                }
            }
        }
    }
    s.push_str("}\n");
    if !accessors.is_empty() {
        let _ = writeln!(s, "impl {} {{\n{accessors}}}", t.name);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolve::Protect;

    fn typedef(name: &str, category: TypeCategory, raw_c: &str, payload: TypePayload) -> TypeDef {
        TypeDef {
            name: name.to_string(),
            raw_c: raw_c.to_string(),
            category,
            payload,
            protect: Protect::default(),
        }
    }

    fn parse_vk_types() -> Vec<crate::ir::RawType> {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("bundled/xml/vk.xml");
        let text = std::fs::read_to_string(path).unwrap();
        let doc = roxmltree::Document::parse(&text).unwrap();
        let docs = crate::parse::SpecDocs {
            primary: &doc,
            supplementals: &[],
        };
        crate::parse::types::parse_types(&docs, crate::diag::Diag::new(true))
    }

    fn emit_one(raw: &crate::ir::RawType, defined: &HashSet<&str>) -> String {
        let t = typedef(&raw.name, raw.category, &raw.raw_c, raw.payload.clone());
        let mut s = String::new();
        let mut platform = IndexSet::new();
        emit_type(&t, &mut s, defined, &mut platform).unwrap();
        s
    }

    #[test]
    fn vk_const_literal_forms() {
        assert_eq!(vk_const("256U").unwrap(), ("u32".into(), "256".into()));
        assert_eq!(vk_const("(~0U)").unwrap(), ("u32".into(), "!0u32".into()));
        assert_eq!(vk_const("(~0ULL)").unwrap(), ("u64".into(), "!0u64".into()));
        assert_eq!(
            vk_const("(~0U-2)").unwrap(),
            ("u32".into(), "!0u32 - 2".into())
        );
        assert_eq!(
            vk_const("1000.0F").unwrap(),
            ("f32".into(), "1000.0".into())
        );
        assert_eq!(
            vk_const("\"VK_KHR_swapchain\"").unwrap(),
            ("&core::ffi::CStr".into(), "c\"VK_KHR_swapchain\"".into())
        );
        assert!(vk_const("offsetof(x)").is_err());
    }

    #[test]
    fn handles_structs_and_typedefs_from_real_spec() {
        let types = parse_vk_types();
        let defined: HashSet<&str> = types.iter().map(|t| t.name.as_str()).collect();
        let by_name = |n: &str| types.iter().find(|t| t.name == n).unwrap();

        // Handles.
        let inst = emit_one(by_name("VkInstance"), &defined);
        assert!(inst.contains("pub struct VkInstance(pub *mut c_void);"));
        let buf = emit_one(by_name("VkBuffer"), &defined);
        assert!(buf.contains("pub struct VkBuffer(pub u64);"));

        // Bitmask-category typedef: VkFlags -> u32.
        let flags = emit_one(by_name("VkFlags"), &defined);
        assert_eq!(flags, "pub type VkFlags = u32;\n");
        let flags64 = emit_one(by_name("VkFlags64"), &defined);
        assert_eq!(flags64, "pub type VkFlags64 = u64;\n");

        // A plain struct with array + pointer members.
        let props = emit_one(by_name("VkPhysicalDeviceProperties"), &defined);
        assert!(props.contains("#[repr(C)]"));
        assert!(
            props.contains("pub deviceName: [c_char; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],")
        );
        assert!(props.contains("pub limits: VkPhysicalDeviceLimits,"));

        // sType/pNext shapes.
        let ici = emit_one(by_name("VkInstanceCreateInfo"), &defined);
        assert!(ici.contains("pub sType: VkStructureType,"));
        assert!(ici.contains("pub pNext: *const c_void,"));
        assert!(ici.contains("pub ppEnabledExtensionNames: *const *const c_char,"));

        // A union.
        let ccv = emit_one(by_name("VkClearColorValue"), &defined);
        assert!(ccv.contains("pub union VkClearColorValue {"));
        assert!(ccv.contains("pub float32: [f32; 4 as usize],"));

        // Bitfield merging: the acceleration-structure instance struct.
        let asi = emit_one(by_name("VkAccelerationStructureInstanceKHR"), &defined);
        assert!(asi.contains("pub instanceCustomIndex_and_mask: u32,"));
        assert!(
            asi.contains("pub instanceShaderBindingTableRecordOffset_and_flags: u32,"),
            "second bitfield run must pack separately:\n{asi}"
        );
        assert!(asi.contains("pub fn instanceCustomIndex(&self) -> u32 {"));
        assert!(asi.contains("(self.instanceCustomIndex_and_mask >> 0) & 0xffffff"));
        assert!(asi.contains("pub fn mask(&self) -> u32 {"));
        assert!(asi.contains("(self.instanceCustomIndex_and_mask >> 24) & 0xff"));
        assert!(asi.contains("pub fn set_mask(&mut self, v: u32) {"));

        // Old-format funcpointer.
        let pfn = emit_one(by_name("PFN_vkAllocationFunction"), &defined);
        assert!(
            pfn.contains("pub type PFN_vkAllocationFunction = Option<unsafe extern \"system\" fn(")
        );
        assert!(pfn.contains(") -> *mut c_void>;"));

        // Platform-referencing struct records its platform types.
        let win32 = by_name("VkImportMemoryWin32HandleInfoKHR");
        let t = typedef(
            &win32.name,
            win32.category,
            &win32.raw_c,
            win32.payload.clone(),
        );
        let mut s = String::new();
        let mut platform = IndexSet::new();
        emit_type(&t, &mut s, &defined, &mut platform).unwrap();
        assert!(platform.contains("HANDLE"));
        assert!(s.contains("pub handle: HANDLE,"));
    }

    /// Every selected-shape Vulkan type in the bundled spec must emit (or
    /// name a StdVideo dependency, which is a documented hard error).  This
    /// is the coverage guarantee for the printer itself.
    #[test]
    fn corpus_every_vk_type_emits_or_names_stdvideo() {
        let types = parse_vk_types();
        let mut defined: HashSet<&str> = types.iter().map(|t| t.name.as_str()).collect();
        // Enum-group names double as types (VkStructureType etc.); the
        // parse-level view here has them as enum-category RawTypes already
        // included above.  Add the video interface names C gets from
        // vk_video headers so only genuinely unknown bases fail.
        let mut failures: Vec<String> = Vec::new();
        defined.insert("VkBool32");

        let mut emitted = 0usize;
        for raw in &types {
            if raw.category == TypeCategory::Include || raw.raw_c.is_empty() {
                continue;
            }
            let t = typedef(&raw.name, raw.category, &raw.raw_c, raw.payload.clone());
            let mut s = String::new();
            let mut platform = IndexSet::new();
            match emit_type(&t, &mut s, &defined, &mut platform) {
                Ok(()) => emitted += 1,
                Err(e) => {
                    let msg = format!("{e:#}");
                    if !msg.contains("StdVideo") {
                        failures.push(format!("{}: {msg}", raw.name));
                    }
                }
            }
        }
        assert!(emitted > 900, "suspiciously few types emitted ({emitted})");
        assert!(
            failures.is_empty(),
            "{} types failed to emit:\n{}",
            failures.len(),
            failures.join("\n")
        );
    }
}
