//! Vulkan loader emission for the Rust backend.
//!
//! Mirrors the C backend's phased Vulkan loading contract exactly:
//!
//! 1. `initialize(gipa)` — store `vkGetInstanceProcAddr`, load Global-scope
//!    PFNs (`vkCreateInstance`, `vkEnumerateInstance*`).
//! 2. `load_instance(instance, api_version, enabled_extensions)` — set
//!    feature flags from the application's requested version, extension
//!    flags from its enabled list, then load Global+Instance-scope PFNs.
//! 3. `load_physical_device_extensions(extensions)` — optionally pre-load
//!    the Instance-scope query commands of device extensions before a
//!    `VkDevice` exists.
//! 4. `load_device(device, physical_device, enabled_extensions)` — resolve
//!    `vkGetDeviceProcAddr`, cap features at the device's `apiVersion`,
//!    apply device extensions, and reload all scopes so Device-scope
//!    commands get the direct-dispatch fast path.
//!
//! Scope-aware loading consults a `COMMAND_SCOPES` table (the same
//! inference the C backend bakes into `kCommandScopes`): Global resolves
//! via `gipa(None, name)`, Instance via `gipa(instance, name)`, Device via
//! `gdpa(device, name)`; a loaded slot is only overwritten by a non-null
//! result, so repeated phases are additive.

use std::fmt::Write as _;

use anyhow::{Result, bail};

use super::common::{
    cmd_index, emit_command_tables, emit_extension_tables, emit_global_flag_query, emit_global_fn,
    emit_method, emit_missing_helpers,
};
use super::vk_types;
use crate::parse::ctype::TypeRef;
use crate::resolve::FeatureSet;

/// Scope byte for the `COMMAND_SCOPES` table.
fn scope_code(scope: &str) -> Result<u8> {
    Ok(match scope {
        "GloamCommandScopeGlobal" => 0,
        "GloamCommandScopeInstance" => 1,
        "GloamCommandScopeDevice" => 2,
        // vkGetInstanceProcAddr itself: never loadable through a proc-addr
        // callback.
        "GloamCommandScopeUnknown" => 3,
        other => bail!("unknown Vulkan command scope '{other}'"),
    })
}

/// Print a `TypeRef` in a command-signature position: Vulkan base mapping,
/// C parameter-array decay (`float blendConstants[4]` → `*const f32`).
fn vk_param_type(ty: &TypeRef) -> String {
    let mut t = vk_types::vk_base(&ty.base);
    for i in 0..ty.pointers.len() {
        let pointee_const = if i == 0 {
            ty.base_const
        } else {
            ty.pointers[i - 1]
        };
        t = format!("{}{t}", if pointee_const { "*const " } else { "*mut " });
    }
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

/// Build the whole Vulkan module body (everything after the preamble), plus
/// the layout-probe facts collected while emitting the types.
pub(super) fn emit_vk_module(
    fs: &FeatureSet,
    preamble: &str,
    mx_global: bool,
) -> Result<(String, Vec<vk_types::ProbeEntry>)> {
    let mut s = String::with_capacity(1024 * 1024);
    let mut probe: Vec<vk_types::ProbeEntry> = Vec::new();

    s.push_str(preamble);
    s.push('\n');
    s.push_str(
        "#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, clippy::all)]\n\
         #![deny(unsafe_op_in_unsafe_fn)]\n\n\
         use core::ffi::{CStr, c_char, c_void};\n\n",
    );

    vk_types::emit_vk_types(fs, &mut s, &mut probe)?;
    s.push('\n');
    emit_prelude(fs, &mut s);
    emit_command_tables(fs, &mut s);
    emit_scope_table(fs, &mut s)?;
    emit_extension_tables(fs, &mut s)?;
    emit_missing_helpers(&mut s, "Vulkan");
    emit_context(fs, &mut s);
    if mx_global {
        emit_global(fs, &mut s);
    }

    Ok((s, probe))
}

/// Emit the discovery loader (`Vk::discover` + `discover_extensions`) — the
/// analogue of the C backend's `gloamVulkanDiscoverContext`: gloam
/// enumerates what's *available* instead of the application reporting what
/// it *enabled*.  Behind the generated crate's `alloc` cargo feature
/// (enumeration needs buffers); versions and per-scope enumerations are
/// cached across calls, mirroring the C context's `vk_*` cache fields.
fn emit_discover(fs: &FeatureSet, s: &mut String) {
    let idx_eiv = cmd_index(fs, "vkEnumerateInstanceVersion");
    let idx_gpdp = cmd_index(fs, "vkGetPhysicalDeviceProperties");
    let idx_eiep = cmd_index(fs, "vkEnumerateInstanceExtensionProperties");
    let idx_edep = cmd_index(fs, "vkEnumerateDeviceExtensionProperties");

    s.push_str(
        "\n    /// Discovery loading — the C loader's `gloamVulkanDiscoverContext`:\n\
         \x20   /// gloam enumerates what is *available* instead of the application\n\
         \x20   /// reporting what it *enabled*.  The version comes from\n\
         \x20   /// `vkEnumerateInstanceVersion` (capped by the physical device's\n\
         \x20   /// `apiVersion` once one is given), and extension flags are set\n\
         \x20   /// from the instance/device extension enumerations — so the flags\n\
         \x20   /// mean \"available\", unlike the phased path's \"enabled\".\n\
         \x20   /// Additive multi-call: pass handles as they come to exist;\n\
         \x20   /// versions and enumerations are cached across calls.  Returns the\n\
         \x20   /// packed `major << 8 | minor` version, or 0 when\n\
         \x20   /// [`Vk::initialize`] has not run or the instance extension scope\n\
         \x20   /// cannot be enumerated.\n\
         \x20   ///\n\
         \x20   /// # Safety\n\
         \x20   /// [`Vk::initialize`]'s contract; every non-null handle must be a\n\
         \x20   /// live object of this implementation.\n\
         \x20   #[cfg(feature = \"alloc\")]\n\
         \x20   pub unsafe fn discover(\n\
         \x20       &mut self,\n\
         \x20       instance: VkInstance,\n\
         \x20       physical_device: VkPhysicalDevice,\n\
         \x20       device: VkDevice,\n\
         \x20   ) -> u32 {\n\
         \x20       // The top 3 bits of apiVersion encode the variant — mask them\n\
         \x20       // off so Vulkan SC compares like plain Vulkan.\n\
         \x20       const VARIANT_MASK: u32 = 0xe000_0000;\n\
         \x20       let Some(gipa) = self.gipa else { return 0 };\n\
         \x20       // Resolve vkGetDeviceProcAddr through the instance when available.\n\
         \x20       if !instance.0.is_null() && self.gdpa.is_none() {\n\
         \x20           let pfn = unsafe { gipa(instance, c\"vkGetDeviceProcAddr\".as_ptr()) };\n\
         \x20           self.gdpa = unsafe {\n\
         \x20               core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetDeviceProcAddr>>(pfn)\n\
         \x20           };\n\
         \x20       }\n",
    );
    if let Some(eiv) = idx_eiv {
        let _ = write!(
            s,
            "        // Instance version: queried once (a 1.0 loader lacks the\n\
             \x20       // entry point — assume 1.0).\n\
             \x20       if self.instance_version == 0 && !self.pfns[{eiv}].is_null() {{\n\
             \x20           let mut v: u32 = 0;\n\
             \x20           if unsafe {{ self.EnumerateInstanceVersion(&mut v) }} == VK_SUCCESS {{\n\
             \x20               self.instance_version = v & !VARIANT_MASK;\n\
             \x20           }}\n\
             \x20       }}\n"
        );
    }
    if let Some(gpdp) = idx_gpdp {
        let _ = write!(
            s,
            "        // Device version: the authoritative cap once a physical\n\
             \x20       // device exists; queried and cached once.\n\
             \x20       if self.device_version == 0\n\
             \x20           && !physical_device.0.is_null()\n\
             \x20           && !self.pfns[{gpdp}].is_null()\n\
             \x20       {{\n\
             \x20           let mut props: VkPhysicalDeviceProperties = unsafe {{ core::mem::zeroed() }};\n\
             \x20           unsafe {{ self.GetPhysicalDeviceProperties(physical_device, &mut props) }};\n\
             \x20           self.device_version = props.apiVersion & !VARIANT_MASK;\n\
             \x20       }}\n"
        );
    }
    s.push_str(
        "        let packed = if self.device_version != 0 {\n\
         \x20           self.device_version\n\
         \x20       } else if self.instance_version != 0 {\n\
         \x20           self.instance_version\n\
         \x20       } else {\n\
         \x20           VK_API_VERSION_1_0\n\
         \x20       };\n\
         \x20       self.apply_version(packed);\n\
         \x20       if !unsafe { self.discover_extensions(physical_device) } {\n\
         \x20           return 0;\n\
         \x20       }\n\
         \x20       for &(fi, start, count) in FEATURE_RANGES.iter() {\n\
         \x20           if self.feat[fi as usize] {\n\
         \x20               unsafe { self.load_range(instance, device, start, count) };\n\
         \x20           }\n\
         \x20       }\n\
         \x20       for &(ei, start, count) in EXT_RANGES_vk.iter() {\n\
         \x20           if self.ext[ei as usize] {\n\
         \x20               unsafe { self.load_range(instance, device, start, count) };\n\
         \x20           }\n\
         \x20       }\n\
         \x20       self.resolve_aliases();\n\
         \x20       self.version\n\
         \x20   }\n\n\
         \x20   /// Enumerate available instance extensions (once) and — once a\n\
         \x20   /// physical device exists — its device extensions (once), setting\n\
         \x20   /// each recognized name's flag.  Flags accumulate, mirroring the C\n\
         \x20   /// loader's `|=`; a missing instance enumeration entry point while\n\
         \x20   /// that scope is unqueried fails discovery, as in C.\n\
         \x20   #[cfg(feature = \"alloc\")]\n\
         \x20   unsafe fn discover_extensions(&mut self, physical_device: VkPhysicalDevice) -> bool {\n",
    );
    if let Some(eiep) = idx_eiep {
        let _ = write!(
            s,
            "        if !self.found_instance_exts {{\n\
             \x20           if self.pfns[{eiep}].is_null() {{\n\
             \x20               return false;\n\
             \x20           }}\n\
             \x20           let mut n: u32 = 0;\n\
             \x20           unsafe {{\n\
             \x20               self.EnumerateInstanceExtensionProperties(\n\
             \x20                   core::ptr::null(),\n\
             \x20                   &mut n,\n\
             \x20                   core::ptr::null_mut(),\n\
             \x20               )\n\
             \x20           }};\n\
             \x20           let mut props: alloc::vec::Vec<VkExtensionProperties> =\n\
             \x20               alloc::vec::Vec::new();\n\
             \x20           props.resize(n as usize, unsafe {{ core::mem::zeroed() }});\n\
             \x20           unsafe {{\n\
             \x20               self.EnumerateInstanceExtensionProperties(\n\
             \x20                   core::ptr::null(),\n\
             \x20                   &mut n,\n\
             \x20                   props.as_mut_ptr(),\n\
             \x20               )\n\
             \x20           }};\n\
             \x20           for p in &props[..n as usize] {{\n\
             \x20               let name = unsafe {{ CStr::from_ptr(p.extensionName.as_ptr()) }};\n\
             \x20               let h = xxhash_rust::xxh3::xxh3_64(name.to_bytes());\n\
             \x20               if let Ok(pos) = EXT_HASH_KEYS.binary_search(&h) {{\n\
             \x20                   self.ext[EXT_HASH_IDX[pos] as usize] = true;\n\
             \x20               }}\n\
             \x20           }}\n\
             \x20           self.found_instance_exts = true;\n\
             \x20       }}\n"
        );
    } else {
        s.push_str(
            "        if !self.found_instance_exts {\n\
             \x20           return false; // vkEnumerateInstanceExtensionProperties not in this build\n\
             \x20       }\n",
        );
    }
    if let Some(edep) = idx_edep {
        let _ = write!(
            s,
            "        if !physical_device.0.is_null()\n\
             \x20           && !self.found_device_exts\n\
             \x20           && !self.pfns[{edep}].is_null()\n\
             \x20       {{\n\
             \x20           let mut n: u32 = 0;\n\
             \x20           unsafe {{\n\
             \x20               self.EnumerateDeviceExtensionProperties(\n\
             \x20                   physical_device,\n\
             \x20                   core::ptr::null(),\n\
             \x20                   &mut n,\n\
             \x20                   core::ptr::null_mut(),\n\
             \x20               )\n\
             \x20           }};\n\
             \x20           let mut props: alloc::vec::Vec<VkExtensionProperties> =\n\
             \x20               alloc::vec::Vec::new();\n\
             \x20           props.resize(n as usize, unsafe {{ core::mem::zeroed() }});\n\
             \x20           unsafe {{\n\
             \x20               self.EnumerateDeviceExtensionProperties(\n\
             \x20                   physical_device,\n\
             \x20                   core::ptr::null(),\n\
             \x20                   &mut n,\n\
             \x20                   props.as_mut_ptr(),\n\
             \x20               )\n\
             \x20           }};\n\
             \x20           for p in &props[..n as usize] {{\n\
             \x20               let name = unsafe {{ CStr::from_ptr(p.extensionName.as_ptr()) }};\n\
             \x20               let h = xxhash_rust::xxh3::xxh3_64(name.to_bytes());\n\
             \x20               if let Ok(pos) = EXT_HASH_KEYS.binary_search(&h) {{\n\
             \x20                   self.ext[EXT_HASH_IDX[pos] as usize] = true;\n\
             \x20               }}\n\
             \x20           }}\n\
             \x20           self.found_device_exts = true;\n\
             \x20       }}\n"
        );
    }
    s.push_str("        true\n    }\n");
}

/// Emit the `--mx-global` layer: a process-global context in an `UnsafeCell`
/// plus one free function per command / presence query delegating to it —
/// the analogue of the C loader's `gloam_vk_context` global + force-inlined
/// wrappers.  Unlike the GL global's single install, the Vulkan global is
/// written by each *load phase* (`initialize_global`, `load_instance_global`,
/// `load_device_global`); every write carries the same exclusivity contract,
/// which Vulkan setup code upholds naturally (instance/device creation is a
/// guarded, effectively single-threaded stage of every engine).
fn emit_global(fs: &FeatureSet, s: &mut String) {
    s.push_str(
        "\n// ── Global context (--mx-global) ───────────────────────────\n\
         struct GlobalCell(core::cell::UnsafeCell<Vk>);\n\
         // SAFETY: every write goes through the *_global phase functions, whose\n\
         // contracts make each write exclusive (nothing else touches the global\n\
         // while a phase runs, and no reference from `global()` outlives into\n\
         // one); all other access is read-only.\n\
         unsafe impl Sync for GlobalCell {}\n\n\
         static GLOBAL: GlobalCell = GlobalCell(core::cell::UnsafeCell::new(Vk::new()));\n\n\
         /// The process-global context (an empty context before\n\
         /// [`initialize_global`]: flags read false, dispatch is a contract\n\
         /// violation).  Do not hold the returned reference across a later\n\
         /// `*_global` phase call — its `'static` lifetime is a loan against\n\
         /// the global staying untouched, exactly like keeping a pointer to the\n\
         /// C loader's global context across a load phase.\n\
         #[inline]\n\
         pub fn global() -> &'static Vk {\n\
         \x20   // SAFETY: no `&mut` can exist outside the `*_global` phase\n\
         \x20   // functions, whose contracts forbid running while this reference\n\
         \x20   // (or any use of it) lives.\n\
         \x20   unsafe { &*GLOBAL.0.get() }\n\
         }\n\n\
         /// Phase 0 against the global context; see [`Vk::initialize`].\n\
         ///\n\
         /// # Safety\n\
         /// [`Vk::initialize`]'s contract, plus exclusivity: nothing may use\n\
         /// the global while any `*_global` phase function runs — no free-\n\
         /// function calls, no [`global`] calls, and no held `&Vk` from\n\
         /// [`global`].  Each phase's write is unsynchronized, so publish it to\n\
         /// other threads through an ordinary happens-before edge, exactly as\n\
         /// with the C loader's global context.\n\
         pub unsafe fn initialize_global(gipa: PFN_vkGetInstanceProcAddr) {\n\
         \x20   unsafe { (*GLOBAL.0.get()).initialize(gipa) };\n\
         }\n\n\
         /// Phase 1 against the global context; see [`Vk::load_instance`].\n\
         ///\n\
         /// # Safety\n\
         /// [`Vk::load_instance`]'s contract plus [`initialize_global`]'s\n\
         /// exclusivity rule.\n\
         pub unsafe fn load_instance_global(\n\
         \x20   instance: VkInstance,\n\
         \x20   api_version: u32,\n\
         \x20   enabled_extensions: &[&CStr],\n\
         ) -> bool {\n\
         \x20   unsafe { (*GLOBAL.0.get()).load_instance(instance, api_version, enabled_extensions) }\n\
         }\n\n\
         /// Phase 1.5 against the global context; see\n\
         /// [`Vk::load_physical_device_extensions`].\n\
         ///\n\
         /// # Safety\n\
         /// [`Vk::load_physical_device_extensions`]'s contract plus\n\
         /// [`initialize_global`]'s exclusivity rule.\n\
         pub unsafe fn load_physical_device_extensions_global(extensions: &[&CStr]) {\n\
         \x20   unsafe { (*GLOBAL.0.get()).load_physical_device_extensions(extensions) };\n\
         }\n\n\
         /// Phase 2 against the global context; see [`Vk::load_device`].\n\
         ///\n\
         /// # Safety\n\
         /// [`Vk::load_device`]'s contract plus [`initialize_global`]'s\n\
         /// exclusivity rule.\n\
         pub unsafe fn load_device_global(\n\
         \x20   device: VkDevice,\n\
         \x20   physical_device: VkPhysicalDevice,\n\
         \x20   enabled_extensions: &[&CStr],\n\
         ) -> bool {\n\
         \x20   unsafe { (*GLOBAL.0.get()).load_device(device, physical_device, enabled_extensions) }\n\
         }\n\n\
         /// Discovery loading against the global context; see [`Vk::discover`].\n\
         ///\n\
         /// # Safety\n\
         /// [`Vk::discover`]'s contract plus [`initialize_global`]'s\n\
         /// exclusivity rule.\n\
         #[cfg(feature = \"alloc\")]\n\
         pub unsafe fn discover_global(\n\
         \x20   instance: VkInstance,\n\
         \x20   physical_device: VkPhysicalDevice,\n\
         \x20   device: VkDevice,\n\
         ) -> u32 {\n\
         \x20   unsafe { (*GLOBAL.0.get()).discover(instance, physical_device, device) }\n\
         }\n\n\
         /// Applied API version of the global context (`major << 8 | minor`; 0\n\
         /// before [`load_instance_global`]).\n\
         #[inline]\n\
         pub fn version() -> u32 {\n\
         \x20   global().version()\n\
         }\n",
    );

    // Free-function mirrors: commands (with the methods' cfg guards), then
    // extension / feature queries, with the same collision-guard order.
    let mut used: std::collections::HashSet<String> = std::collections::HashSet::new();
    for cmd in &fs.commands {
        if used.insert(cmd.short_name.clone()) {
            s.push('\n');
            for p in &cmd.protect.0 {
                let _ = writeln!(s, "#[cfg(feature = \"{p}\")]");
            }
            s.push_str(&emit_global_fn(cmd, "Vk", &vk_param_type));
            s.push('\n');
        }
    }
    for e in &fs.extensions {
        let name = super::common::method_name(&e.short_name);
        if used.insert(name.clone()) {
            emit_global_flag_query(s, &name, &e.name, "has enabled");
        }
    }
    for feat in &fs.features {
        let name = super::common::method_name(&feat.short_name);
        if used.insert(name.clone()) {
            emit_global_flag_query(s, &name, &feat.full_name, "supports");
        }
    }
}

/// Version helpers and the proc-addr function-pointer types the loader
/// stores.  These are spec-fixed ABI shapes, not registry-derived: the
/// `PFN_vkGet*ProcAddr` typedefs in C come from the per-command PFN
/// section, which the Rust backend replaces with dispatch methods.
fn emit_prelude(fs: &FeatureSet, s: &mut String) {
    s.push_str(
        "// ── Loader prelude ──────────────────────────────────────────\n\
         pub type PFN_vkGetInstanceProcAddr =\n\
         \x20   unsafe extern \"system\" fn(VkInstance, *const c_char) -> PFN_vkVoidFunction;\n\
         pub type PFN_vkGetDeviceProcAddr =\n\
         \x20   unsafe extern \"system\" fn(VkDevice, *const c_char) -> PFN_vkVoidFunction;\n\n\
         /// `VK_MAKE_API_VERSION` — pack a Vulkan version number.\n\
         pub const fn VK_MAKE_API_VERSION(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {\n\
         \x20   (variant << 29) | (major << 22) | (minor << 12) | patch\n\
         }\n\
         pub const fn VK_API_VERSION_VARIANT(version: u32) -> u32 {\n\
         \x20   version >> 29\n\
         }\n\
         pub const fn VK_API_VERSION_MAJOR(version: u32) -> u32 {\n\
         \x20   (version >> 22) & 0x7f\n\
         }\n\
         pub const fn VK_API_VERSION_MINOR(version: u32) -> u32 {\n\
         \x20   (version >> 12) & 0x3ff\n\
         }\n\
         pub const fn VK_API_VERSION_PATCH(version: u32) -> u32 {\n\
         \x20   version & 0xfff\n\
         }\n",
    );
    for feat in &fs.features {
        let major = feat.packed >> 8;
        let minor = feat.packed & 0xff;
        let _ = writeln!(
            s,
            "pub const VK_API_VERSION_{major}_{minor}: u32 = VK_MAKE_API_VERSION(0, {major}, {minor}, 0);"
        );
    }
    s.push('\n');
}

/// The per-command scope table driving scope-aware loading.
fn emit_scope_table(fs: &FeatureSet, s: &mut String) -> Result<()> {
    s.push_str(
        "// Dispatch scope of each command (0 = Global, 1 = Instance,\n\
         // 2 = Device, 3 = unloadable), in pfn-slot order.\n",
    );
    s.push_str("#[rustfmt::skip]\n");
    let _ = writeln!(s, "static COMMAND_SCOPES: [u8; COMMAND_COUNT] = [");
    for chunk in fs.commands.chunks(20) {
        let line: Vec<String> = chunk
            .iter()
            .map(|c| scope_code(&c.scope).map(|v| v.to_string()))
            .collect::<Result<_>>()?;
        let _ = writeln!(s, "    {},", line.join(", "));
    }
    s.push_str("];\n\n");
    Ok(())
}

/// Emit the `Vk` context struct and its full impl: phased loading, scope
/// dispatch, alias resolution, dispatch methods, presence queries.
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
         /// Loaded Vulkan entry points plus feature/extension flags.  Unlike the\n\
         /// GL context this is mutable and loaded in phases as the application\n\
         /// progresses through instance and device creation; see [`Vk::initialize`].\n\
         pub struct Vk {\n\
         \x20   pfns: [*const c_void; COMMAND_COUNT],\n\
         \x20   feat: [bool; FEATURE_COUNT],\n\
         \x20   ext: [bool; EXT_COUNT],\n\
         \x20   gipa: Option<PFN_vkGetInstanceProcAddr>,\n\
         \x20   gdpa: Option<PFN_vkGetDeviceProcAddr>,\n\
         \x20   instance: VkInstance,\n\
         \x20   version: u32,\n\
         \x20   // Discovery caches ([`Vk::discover`]): enumerated versions\n\
         \x20   // (variant-masked) and whether each extension scope has been\n\
         \x20   // enumerated, mirroring the C context's vk_* cache fields.\n\
         \x20   instance_version: u32,\n\
         \x20   device_version: u32,\n\
         \x20   found_instance_exts: bool,\n\
         \x20   found_device_exts: bool,\n\
         }\n\n\
         impl Vk {\n\
         \x20   /// An empty context: no PFNs, every flag false.  Load it with\n\
         \x20   /// [`Vk::initialize`] then [`Vk::load_instance`] / [`Vk::load_device`].\n\
         \x20   pub const fn new() -> Vk {\n\
         \x20       Vk {\n\
         \x20           pfns: [core::ptr::null(); COMMAND_COUNT],\n\
         \x20           feat: [false; FEATURE_COUNT],\n\
         \x20           ext: [false; EXT_COUNT],\n\
         \x20           gipa: None,\n\
         \x20           gdpa: None,\n\
         \x20           instance: VkInstance(core::ptr::null_mut()),\n\
         \x20           version: 0,\n\
         \x20           instance_version: 0,\n\
         \x20           device_version: 0,\n\
         \x20           found_instance_exts: false,\n\
         \x20           found_device_exts: false,\n\
         \x20       }\n\
         \x20   }\n\n\
         \x20   /// Phase 0: store `vkGetInstanceProcAddr` and load Global-scope\n\
         \x20   /// commands (`vkCreateInstance`, `vkEnumerateInstance*`), which are\n\
         \x20   /// callable before any instance exists.\n\
         \x20   ///\n\
         \x20   /// # Safety\n\
         \x20   /// `gipa` must be a valid `vkGetInstanceProcAddr` for a Vulkan\n\
         \x20   /// implementation that outlives this context.\n\
         \x20   pub unsafe fn initialize(&mut self, gipa: PFN_vkGetInstanceProcAddr) {\n\
         \x20       *self = Vk::new();\n\
         \x20       self.gipa = Some(gipa);\n\
         \x20       unsafe { self.load_range(VkInstance(core::ptr::null_mut()), VkDevice(core::ptr::null_mut()), 0, COMMAND_COUNT as u16) };\n\
         \x20   }\n\n\
         \x20   /// Scope-aware range loader.  Only Global commands resolve while\n\
         \x20   /// `instance` is null; only Device commands consult `device`.  A\n\
         \x20   /// non-null result overwrites; a null result never clobbers a\n\
         \x20   /// previously loaded slot, so phases are additive.\n\
         \x20   unsafe fn load_range(&mut self, instance: VkInstance, device: VkDevice, start: u16, count: u16) {\n\
         \x20       let Some(gipa) = self.gipa else { return };\n\
         \x20       for i in start..start + count {\n\
         \x20           let idx = i as usize;\n\
         \x20           let off = FN_NAME_OFFSETS[idx] as usize;\n\
         \x20           let name =\n\
         \x20               unsafe { CStr::from_bytes_until_nul(&FN_NAME_DATA[off..]).unwrap_unchecked() };\n\
         \x20           let pfn: PFN_vkVoidFunction = match COMMAND_SCOPES[idx] {\n\
         \x20               0 => unsafe { gipa(VkInstance(core::ptr::null_mut()), name.as_ptr()) },\n\
         \x20               1 if !instance.0.is_null() => unsafe { gipa(instance, name.as_ptr()) },\n\
         \x20               2 if !device.0.is_null() => match self.gdpa {\n\
         \x20                   Some(gdpa) => unsafe { gdpa(device, name.as_ptr()) },\n\
         \x20                   None => None,\n\
         \x20               },\n\
         \x20               _ => None,\n\
         \x20           };\n\
         \x20           if let Some(f) = pfn {\n\
         \x20               self.pfns[idx] = f as *const c_void;\n\
         \x20           }\n\
         \x20       }\n\
         \x20   }\n\n\
         \x20   /// Set feature flags from a packed Vulkan API version\n\
         \x20   /// (`VK_MAKE_API_VERSION` / a device's `apiVersion`).\n\
         \x20   fn apply_version(&mut self, api_version: u32) {\n\
         \x20       let v = (VK_API_VERSION_MAJOR(api_version) << 8) | VK_API_VERSION_MINOR(api_version);\n\
         \x20       self.version = v;\n",
    );
    for feat in &fs.features {
        let _ = writeln!(
            s,
            "        self.feat[{idx}] = v >= 0x{packed:04x};",
            idx = feat.index,
            packed = feat.packed
        );
    }
    s.push_str(
        "    }\n\n\
         \x20   /// Set extension flags for every recognized name in `names`\n\
         \x20   /// (the application's enabled-extension lists).  Flags accumulate\n\
         \x20   /// across calls, mirroring the C loader's `|=`.\n\
         \x20   fn apply_extensions(&mut self, names: &[&CStr]) {\n\
         \x20       for name in names {\n\
         \x20           let h = xxhash_rust::xxh3::xxh3_64(name.to_bytes());\n\
         \x20           if let Ok(pos) = EXT_HASH_KEYS.binary_search(&h) {\n\
         \x20               self.ext[EXT_HASH_IDX[pos] as usize] = true;\n\
         \x20           }\n\
         \x20       }\n\
         \x20   }\n\n\
         \x20   /// Phase 1: the application created `instance` with `api_version`\n\
         \x20   /// and `enabled_extensions` — set flags accordingly and load\n\
         \x20   /// Global+Instance-scope commands for everything enabled.\n\
         \x20   /// Device-scope commands wait for [`Vk::load_device`].\n\
         \x20   ///\n\
         \x20   /// # Safety\n\
         \x20   /// `instance` must be a live `VkInstance` created from the same\n\
         \x20   /// implementation passed to [`Vk::initialize`].\n\
         \x20   pub unsafe fn load_instance(\n\
         \x20       &mut self,\n\
         \x20       instance: VkInstance,\n\
         \x20       api_version: u32,\n\
         \x20       enabled_extensions: &[&CStr],\n\
         \x20   ) -> bool {\n\
         \x20       if self.gipa.is_none() || instance.0.is_null() {\n\
         \x20           return false;\n\
         \x20       }\n\
         \x20       self.apply_version(api_version);\n\
         \x20       self.apply_extensions(enabled_extensions);\n\
         \x20       let null_dev = VkDevice(core::ptr::null_mut());\n\
         \x20       for &(fi, start, count) in FEATURE_RANGES.iter() {\n\
         \x20           if self.feat[fi as usize] {\n\
         \x20               unsafe { self.load_range(instance, null_dev, start, count) };\n\
         \x20           }\n\
         \x20       }\n\
         \x20       for &(ei, start, count) in EXT_RANGES_vk.iter() {\n\
         \x20           if self.ext[ei as usize] {\n\
         \x20               unsafe { self.load_range(instance, null_dev, start, count) };\n\
         \x20           }\n\
         \x20       }\n\
         \x20       self.resolve_aliases();\n\
         \x20       self.instance = instance;\n\
         \x20       true\n\
         \x20   }\n\n\
         \x20   /// Phase 1.5 (optional): pre-load the Global+Instance-scope query\n\
         \x20   /// commands of device extensions before a `VkDevice` exists (e.g.\n\
         \x20   /// `vkGetPhysicalDeviceFragmentShadingRatesKHR`).  Does not set\n\
         \x20   /// extension flags — presence still reflects what the application\n\
         \x20   /// has actually enabled.\n\
         \x20   ///\n\
         \x20   /// # Safety\n\
         \x20   /// [`Vk::load_instance`] must have succeeded.\n\
         \x20   pub unsafe fn load_physical_device_extensions(&mut self, extensions: &[&CStr]) {\n\
         \x20       if self.gipa.is_none() || self.instance.0.is_null() {\n\
         \x20           return;\n\
         \x20       }\n\
         \x20       let instance = self.instance;\n\
         \x20       let null_dev = VkDevice(core::ptr::null_mut());\n\
         \x20       for name in extensions {\n\
         \x20           let h = xxhash_rust::xxh3::xxh3_64(name.to_bytes());\n\
         \x20           let Ok(pos) = EXT_HASH_KEYS.binary_search(&h) else {\n\
         \x20               continue;\n\
         \x20           };\n\
         \x20           let ext_idx = EXT_HASH_IDX[pos];\n\
         \x20           for &(ei, start, count) in EXT_RANGES_vk.iter() {\n\
         \x20               if ei == ext_idx {\n\
         \x20                   unsafe { self.load_range(instance, null_dev, start, count) };\n\
         \x20               }\n\
         \x20           }\n\
         \x20       }\n\
         \x20       self.resolve_aliases();\n\
         \x20   }\n\n\
         \x20   /// Phase 2: the application created `device` on `physical_device`\n\
         \x20   /// with `enabled_extensions` — cap features at the device's\n\
         \x20   /// `apiVersion`, apply the extensions, and reload all scopes so\n\
         \x20   /// Device-scope commands dispatch through `vkGetDeviceProcAddr`\n\
         \x20   /// (bypassing the loader trampoline).\n\
         \x20   ///\n\
         \x20   /// # Safety\n\
         \x20   /// [`Vk::load_instance`] must have succeeded; `device` and\n\
         \x20   /// `physical_device` must be live objects of that instance.\n\
         \x20   pub unsafe fn load_device(\n\
         \x20       &mut self,\n\
         \x20       device: VkDevice,\n\
         \x20       physical_device: VkPhysicalDevice,\n\
         \x20       enabled_extensions: &[&CStr],\n\
         \x20   ) -> bool {\n\
         \x20       let instance = self.instance;\n\
         \x20       if self.gdpa.is_none() && !instance.0.is_null() {\n\
         \x20           if let Some(gipa) = self.gipa {\n\
         \x20               let pfn = unsafe { gipa(instance, c\"vkGetDeviceProcAddr\".as_ptr()) };\n\
         \x20               self.gdpa = unsafe { core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetDeviceProcAddr>>(pfn) };\n\
         \x20           }\n\
         \x20       }\n\
         \x20       if self.gdpa.is_none() || device.0.is_null() {\n\
         \x20           return false;\n\
         \x20       }\n\
         \x20       let mut props: VkPhysicalDeviceProperties = unsafe { core::mem::zeroed() };\n\
         \x20       unsafe { self.GetPhysicalDeviceProperties(physical_device, &mut props) };\n\
         \x20       // Mask the variant bits so Vulkan SC compares like plain Vulkan.\n\
         \x20       self.apply_version(props.apiVersion & !(0x7u32 << 29));\n\
         \x20       self.apply_extensions(enabled_extensions);\n\
         \x20       for &(fi, start, count) in FEATURE_RANGES.iter() {\n\
         \x20           if self.feat[fi as usize] {\n\
         \x20               unsafe { self.load_range(instance, device, start, count) };\n\
         \x20           }\n\
         \x20       }\n\
         \x20       for &(ei, start, count) in EXT_RANGES_vk.iter() {\n\
         \x20           if self.ext[ei as usize] {\n\
         \x20               unsafe { self.load_range(instance, device, start, count) };\n\
         \x20           }\n\
         \x20       }\n\
         \x20       self.resolve_aliases();\n\
         \x20       true\n\
         \x20   }\n",
    );

    emit_discover(fs, s);

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
        "\n    /// Applied API version, packed as `major << 8 | minor` (0 before\n\
         \x20   /// [`Vk::load_instance`]).\n\
         \x20   #[inline]\n\
         \x20   pub fn version(&self) -> u32 {\n\
         \x20       self.version\n\
         \x20   }\n",
    );

    // Dispatch methods, then presence queries, sharing one collision set.
    s.push_str(
        "\n    // Dispatch wrappers.  The pointer local is named `__pfn` because\n\
         \x20   // parameter names could otherwise collide with it.\n",
    );
    let safety =
        "The command's scope must be loaded (see [`Vk::initialize`]) and its handles valid.";
    let mut used: std::collections::HashSet<String> = std::collections::HashSet::new();
    for cmd in &fs.commands {
        if used.insert(cmd.short_name.clone()) {
            s.push('\n');
            // Platform-protected commands reference cfg-guarded types; guard
            // the method identically (the pfn slot itself stays unguarded so
            // table indices are stable).
            for p in &cmd.protect.0 {
                let _ = writeln!(s, "    #[cfg(feature = \"{p}\")]");
            }
            s.push_str(&emit_method(cmd, safety, &vk_param_type));
            s.push('\n');
        }
    }
    for e in &fs.extensions {
        let name = super::common::method_name(&e.short_name);
        if used.insert(name.clone()) {
            super::common::emit_flag_query(s, &name, &e.name, "ext", e.index, "has enabled");
        }
    }
    for feat in &fs.features {
        let name = super::common::method_name(&feat.short_name);
        if used.insert(name.clone()) {
            super::common::emit_flag_query(
                s,
                &name,
                &feat.full_name,
                "feat",
                feat.index,
                "supports",
            );
        }
    }

    s.push_str("}\n\n");
    s.push_str(
        "impl Default for Vk {\n\
         \x20   fn default() -> Self {\n\
         \x20       Vk::new()\n\
         \x20   }\n\
         }\n",
    );
}
