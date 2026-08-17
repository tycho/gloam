# The generated Rust loader — consumer guide

> **Status:** implemented. This document describes the Rust output — the
> generated crate, its runtime API, and how every piece corresponds to the
> C loader (which [c-loader.md](c-loader.md) covers). The two backends share
> gloam's resolver, so a C and a Rust loader generated from the same command
> line detect versions and extensions identically.

The `rust` subcommand emits a self-contained crate instead of C sources:

```
<out-path>/
  .gloam/manifest.json    # provenance + the exact command line (see manifest.md)
  Cargo.toml
  rustfmt.toml
  src/
    lib.rs                # crate root: #![no_std], one pub mod per spec
    gl.rs                 # one module per spec stem
    vk.rs
    ...
```

The crate is named `gloam_` plus the joined stems (`gloam_gl`,
`gloam_vk`, `gloam_gl_vk_egl_glx_wgl`, ...), is `#![no_std]`, requires
Rust 1.81, and has a single dependency: `xxhash-rust`, for the same XXH3
extension-name hashing the C loader bakes in. Consume it as a path
dependency or workspace member; like the C output, the intended workflow
is to generate once and check the crate into your repository (the
examples under [examples/rust/](../examples/rust/) all do this).

---

## 1. Contexts and dispatch

Each spec module defines an owned context type (`Gl`, `Vk`, `Egl`,
`Wgl`, `Glx`) holding the function-pointer table, presence flags, and
detected version. Loading returns the context by value; every command is
an `#[inline]` method on it:

```rust
use gloam_gl::*;

let gl = unsafe { Gl::load_gl(|name| display.get_proc_address(name))? };
unsafe { gl.Clear(GL_COLOR_BUFFER_BIT) };
if gl.VERSION_3_3() { /* GL 3.3 core available */ }
if gl.KHR_debug()   { /* extension detected */ }
```

Method names are the C names without the API prefix (`glClear` →
`gl.Clear`, `vkCmdDraw` → `vk.CmdDraw`), so translation between the
backends is mechanical. The context is a flat struct — one pointer array
plus flag arrays, no heap, no indirection — and deliberately not
`Clone`/`Copy`, so a context is passed by reference rather than
duplicated wholesale.

By default every dispatch method checks that its pointer was loaded and
**panics with the command's name** if not — the misuse that in C jumps
through a null pointer. Building the crate with the `no-error` feature
removes the check (the KHR_no_error idea applied at compile time),
restoring C behavior and cost exactly.

Constants are typed: `GLenum` and `GLbitfield` are `#[repr(transparent)]`
newtypes (`GLbitfield` implements the bit-ops), `GLboolean` is `u8`, and
command signatures use the same types — passing `GL_TRIANGLES` where a
bitfield belongs is a compile error rather than a silent `0x0004`.

## 2. Loading — GL, GLES, EGL, GLX, WGL

The loading entry points take a `GetProcAddress`-style closure
(`impl FnMut(&CStr) -> *const c_void`) and mirror the C signatures
otherwise:

```rust
Gl::load_gl(loader)                    -> Result<Gl, LoadError>
Gl::load_gles2(loader)                 -> Result<Gl, LoadError>   // merged gl output
Egl::load_egl(display, loader)         -> Result<Egl, LoadError>
Wgl::load_wgl(hdc, loader)             -> Result<Wgl, LoadError>
Glx::load_glx(display, screen, loader) -> Result<Glx, LoadError>
```

Where C returns `0` for failure and a packed version otherwise, Rust
splits the two: the `Result` carries a `LoadError` naming what went wrong
(no `glGetString`, unparseable version string, ...), and `.version()` on
the loaded context returns the same `major << 8 | minor` packing the C
loader uses. The per-API parameters mean the same things as in C
(EGL client vs display extensions by whether `display` is
`EGL_NO_DISPLAY`, and so on — see [c-loader.md §2](c-loader.md)).

The closure is type-erased internally, so the loader body is compiled
once per crate, not once per closure type.

## 3. Loading — Vulkan

The Vulkan module follows the C loader's phased enabled-list contract,
with two deliberate differences: the crate never opens the Vulkan
library (bring `vkGetInstanceProcAddr` yourself — `libloading` in the
examples, or link against the loader), and extension lists are `&[&CStr]`
instead of count/pointer pairs.

```rust
let mut vk = vk::Vk::new();
unsafe { vk.initialize(gipa) };                                   // phase 0

// vkCreateInstance as usual ...
unsafe { vk.load_instance(instance, api_version, &enabled_inst) };// phase 1

// vkCreateDevice as usual ...
unsafe { vk.load_device(device, physical_device, &enabled_dev) }; // phase 2
```

`load_instance`/`load_device` return `bool` like their C counterparts,
version capping against the driver works identically, and
`vk.load_physical_device_extensions(&[...])` is phase 1.5 (instance-scope
commands from device extensions, pre-device-creation; presence flags are
still only set by `load_device`). Presence queries are methods
(`vk.KHR_swapchain()`), and `vk.version()` is the applied packed version.
Extension flags are scope-exact, as in C: each load phase assigns exactly
its own scope's flags and leaves the other scope untouched.

Presence *probing* is decoupled from the context: `VkExtensions` is a
standalone snapshot with one query method per extension, filled by
`vk.query_instance_extensions()` / `vk.query_device_extensions(pd)` (one
enumeration answers every known extension — probe each candidate device
once during selection) or `VkExtensions::from_properties(&props)` (pure,
from your own enumeration). `vk.load_device_from_query(device, pd,
&snapshot)` / `vk.load_instance_from_query(...)` are phase variants that
copy a snapshot's flags instead of hashing an enabled-name list, so the
winner's selection probe is reused without re-enumerating.

`vk.discover(instance, physical_device, device)` is the C `--loader`
discovery mode: it enumerates extension properties itself, with the same
caveat that detection means *supported*, not *enabled*. Passing a
different `physical_device` than the previous call invalidates and
re-queries all device-derived state. `discover` and the `query_*`
probes are the APIs that allocate, so they sit behind the default-on
`alloc` cargo feature; the phased loaders never allocate and work with
`default-features = false`.

There is no `Finalize` — dropping the `Vk` (or overwriting it with
`Vk::new()`) is the teardown, and the library handle is yours.

Handles are `#[repr(transparent)]` newtypes (`VkInstance(*mut c_void)`
dispatchable, `VkSwapchainKHR(u64)` non-dispatchable), structs and unions
are `#[repr(C)]` with the C member names, and the few bitfield-carrying
structs expose getter/setter methods over packed carrier fields. Layout
equivalence with the C headers is enforced mechanically by the repo's
layout-oracle test, which compiles thousands of `_Static_assert`s from
the crate's self-reported layouts.

Platform-windowing surface types and commands sit behind cargo features
named exactly like the C headers' defines (`VK_USE_PLATFORM_WIN32_KHR`,
`VK_USE_PLATFORM_WAYLAND_KHR`, ..., plus `VK_ENABLE_BETA_EXTENSIONS`).
The platform types are plain ABI declarations, so enabling a feature for
another OS still compiles — a portable binary can enable all its targets
unconditionally, as [examples/rust/vk-cube](../examples/rust/vk-cube/)
does.

## 4. The global context (`--mx-global`)

With `--mx-global`, each module also emits a process-global context with
free-function dispatch — the analogue of the C loader's global-context
macros:

```rust
use gloam_gl as gl;

unsafe { gl::load_gl_global(|name| display.get_proc_address(name))? };
unsafe { gl::DrawArrays(GL_TRIANGLES, 0, 3) };   // no context threading
if gl::KHR_debug() { /* ... */ }
```

Vulkan gets `initialize_global` / `load_instance_global` /
`load_device_global` / `discover_global` (plus `query_*_extensions_global`
and `load_*_from_query_global` probe mirrors), and every module gets
`global()`, which returns `&'static` access to the underlying context
for passing into context-taking code.

The safety contract matches the C global context: the `*_global` load
phases are **exclusive writers** — nothing may use the global while one
runs, and a reference obtained from `global()` must not be held across a
later phase call (its `'static` lifetime is a loan against the global
staying untouched). Loads are unsynchronized stores, so publish them to
other threads through an ordinary happens-before edge. Re-initializing
after a teardown is fine under the same rule. After loading, all
dispatch and presence reads are read-only and freely concurrent.

## 5. Cargo features

| Feature | Default | Effect |
|---|---|---|
| `alloc` | on | Enables `discover()` (Vulkan), the only allocating API. Off = fully allocator-free. |
| `no-error` | off | Removes the per-call is-loaded check from dispatch; calling an unloaded command is UB (C behavior) instead of a panic. |
| `VK_USE_PLATFORM_*`, `VK_ENABLE_BETA_EXTENSIONS` | off | Expose the platform-guarded Vulkan types and commands, mirroring the C `#define`s. |

## 6. On `unsafe`

A thin loader cannot honestly make dispatch safe: whether a call is
sound depends on driver state (is a matching context current? were these
handles created by this instance?) that the type system cannot see,
which is why every Rust GL/Vulkan binding marks dispatch `unsafe`. The
generated API keeps the boundary honest rather than pretending —
loading and dispatch are `unsafe fn`s with documented contracts, and the
idiomatic consumer pattern is one `unsafe` boundary per GL/Vulkan-touching
phase (see the examples), not one block per call. The checked dispatch
default (§1) removes the most common footgun inside that boundary.

## 7. C ↔ Rust equivalence

Loading and lifecycle:

| C | Rust |
|---|---|
| `gloamLoadGL(getProcAddr)` | `Gl::load_gl(loader)?` |
| `gloamLoadGLES2(getProcAddr)` | `Gl::load_gles2(loader)?` |
| `gloamLoadEGL(display, getProcAddr)` | `Egl::load_egl(display, loader)?` |
| `gloamLoadWGL(hdc, getProcAddr)` | `Wgl::load_wgl(hdc, loader)?` |
| `gloamLoadGLX(dpy, screen, getProcAddr)` | `Glx::load_glx(display, screen, loader)?` |
| return `0` / packed version | `Err(LoadError)` / `Ok(ctx)` + `ctx.version()` |
| `gloamLoadGLContext(&ctx, ...)` (explicit context) | the owned value *is* the context |
| `gloamVulkanInitialize(NULL)` (opens libvulkan) | not generated — bring `vkGetInstanceProcAddr` (e.g. `libloading`) |
| `gloamVulkanInitializeCustom(gipa)` | `Vk::new()` + `vk.initialize(gipa)` |
| `gloamVulkanGetInstanceVersion()` | queried internally; `vk.version()` after `load_instance` |
| `gloamVulkanLoadInstance(inst, ver, n, names)` | `vk.load_instance(instance, api_version, &[&CStr])` |
| `gloamVulkanLoadPhysicalDeviceExtension[s](...)` | `vk.load_physical_device_extensions(&[&CStr])` |
| `gloamVulkanLoadDevice(dev, pd, n, names)` | `vk.load_device(device, physical_device, &[&CStr])` |
| `gloamVulkanQueryInstanceExtensions(&out)` | `vk.query_instance_extensions() -> Option<VkExtensions>` |
| `gloamVulkanQueryDeviceExtensions(pd, &out)` | `vk.query_device_extensions(pd) -> Option<VkExtensions>` |
| `gloamVulkanHashExtensionProperties(n, props, &out)` | `VkExtensions::from_properties(&props)` |
| `gloamVulkanLoadInstanceFromQuery(inst, ver, &exts)` | `vk.load_instance_from_query(instance, api_version, &exts)` |
| `gloamVulkanLoadDeviceFromQuery(dev, pd, &exts)` | `vk.load_device_from_query(device, physical_device, &exts)` |
| `gloamLoaderLoadVulkan(inst, pd, dev)` (discovery) | `vk.discover(instance, physical_device, device)` |
| `gloamVulkanFinalize()` | drop the `Vk` |
| `gloamLoaderLoad*` (dlopen layer) | not generated — open the library yourself |

Dispatch and presence:

| C | Rust |
|---|---|
| `glClear(...)` (macro over global context) | `gl.Clear(...)` method; `gl::Clear(...)` with `--mx-global` |
| `vkCmdDraw(...)` (force-inlined fn) | `vk.CmdDraw(...)` / `vk::CmdDraw(...)` |
| `gloam_gl_context` (global variable) | `gl::global()` (`--mx-global`) |
| `GLOAM_GL_VERSION_3_3` | `gl.VERSION_3_3()` / `gl::VERSION_3_3()` |
| `GLOAM_GL_ES_VERSION_3_0` | `gl.ES_VERSION_3_0()` |
| `GLOAM_GL_KHR_debug` | `gl.KHR_debug()` |
| call through unloaded pointer = crash/UB | panic naming the command (`no-error` restores C behavior) |

Extension methods whose names start with a digit get a leading
underscore (`GLOAM_GL_3DFX_multisample` → `gl._3DFX_multisample()`);
everything else carries the C spelling unchanged.

Types:

| C | Rust |
|---|---|
| `GLenum` (`unsigned int`) | `GLenum(pub u32)` newtype |
| `GLbitfield` | `GLbitfield(pub u32)` newtype with bit-ops |
| `GLboolean` | `u8` |
| `VkInstance` (dispatchable handle) | `VkInstance(pub *mut c_void)` newtype |
| `VkSwapchainKHR` (non-dispatchable) | `VkSwapchainKHR(pub u64)` newtype |
| Vulkan structs/unions | `#[repr(C)]`, same member names; bitfields as carrier + accessors |
| `VK_USE_PLATFORM_*` defines | cargo features of the same name |

`--alias` and `--merge` behave identically in both backends: alias
resolution propagates pointers bijectively between core and extension
spellings after loading, and a merged GL + GLES2 crate is one `Gl` type
with `load_gl`/`load_gles2` constructors and per-API extension scoping.

## 8. Regenerating

Identical to the C output: the crate's `.gloam/manifest.json` records
the generating command line, `gloam regen <tree>` reproduces it, and
`gloam regen --fresh <tree>` advances it. See [manifest.md](manifest.md).
