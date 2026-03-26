# gloam &emsp; [![Build Status]][actions] [![Latest Version]][crates.io]

[Build Status]: https://img.shields.io/github/actions/workflow/status/tycho/gloam/ci.yml?branch=master
[actions]: https://github.com/tycho/gloam/actions?query=branch%3Amaster
[Latest Version]: https://img.shields.io/crates/v/gloam.svg
[crates.io]: https://crates.io/crates/gloam

A loader generator for Vulkan, OpenGL, OpenGL ES, EGL, GLX, and WGL.
Reads Khronos XML spec files and generates C dispatch code. The gloam
binary is fully self-contained — XML specs and auxiliary headers are
embedded at compile time.

---

## Why gloam?

**Fast initialization.** Extension detection uses pre-baked [xxHash] hashes
with binary search — zero string comparisons at load time. gloam's enabled-list
API for Vulkan loads only the function pointers you actually enabled, skipping
the expensive `vkEnumerate*ExtensionProperties` calls entirely.

**Small output.** Generated object files and linked binary sizes are a fraction
of the size of comparable loaders.

**Correct by default.** Bijective alias resolution (`--alias`) propagates
function pointers between core and extension spellings automatically.
`--promoted` and `--predecessors` ensure that both core and extension versions
of functions and enums are available in the generated loader. Merged builds
(`--merge`) scope extensions per-API to prevent cross-contamination.

**Deterministic.** Output is byte-identical across runs — safe to check
into version control and audit diffs after regenerating.

### Benchmarks

Vulkan teardown-and-reinitialize cycle on Linux `x86_64` (see
[vk-api-loader-shootout] for full results across platforms):

| Loader | Avg time | Object size |
|---|---:|---:|
| GLAD (upstream) | 16437 us | 316,288 bytes |
| Volk | 875.75 us | 312,912 bytes |
| gloam (discover) | 9895.55 us | 48,104 bytes |
| **gloam (enabled-list)** | **148.55 us** | **48,104 bytes** |

[vk-api-loader-shootout]: https://github.com/tycho/vk-api-loader-shootout

---

## Installation

```sh
cargo install gloam
```

Or build from source (requires Rust 1.88+):

```sh
cargo build --release
```

---

## Quick start

Generate loaders for every supported API at once:

```sh
gloam --api gl:core,gles2,vulkan,egl,glx,wgl --out-path generated --merge c --alias --loader
```

This produces a merged GL/GLES2 loader, plus separate Vulkan, EGL, GLX,
and WGL loaders — all with full extension coverage, alias resolution,
and a built-in library-opening convenience layer.

Output directory:

```
generated/
  include/
    gloam/
      gl.h            # merged GL + GLES2 public header
      vk.h            # Vulkan public header
      egl.h           # EGL public header
      glx.h           # GLX public header
      wgl.h           # WGL public header
    KHR/
      khrplatform.h   # Khronos platform types
    EGL/
      eglplatform.h   # EGL platform types
    vk_platform.h     # Vulkan platform types
    xxhash.h          # xxHash header (used by loaders)
  src/
    gl.c              # merged GL + GLES2 loader implementation
    vk.c              # Vulkan loader implementation
    egl.c             # EGL loader implementation
    glx.c             # GLX loader implementation
    wgl.c             # WGL loader implementation
```

Without `--merge`, each API gets its own stem: `gl` for desktop GL
only, `gles2` for GLES2 only, `vulkan` for Vulkan, etc.

---

## Command reference

```
gloam [OPTIONS] <COMMAND>

Options:
  --api <SPEC>          API specifiers (required). Comma-separated list of
                        name[:profile][=major.minor] tokens. Profile is
                        required for GL (core|compat). Version is optional
                        (latest if omitted). Examples:
                          gl:core=3.3
                          gles2=3.0
                          gl:core=3.3,gles2=3.0
                          vk=1.3
                          egl=1.5
  --extensions <FILTER> Restrict extensions. Either a path to a file (one
                        name per line) or a comma-separated inline list.
                        Omit to include all extensions supported by the
                        requested API.
  --baseline <SPEC>     Baseline API versions (same format as --api).
                        Extensions fully promoted into these versions or
                        earlier are excluded — they are guaranteed present
                        in a context of at least the baseline version.
                        Example: --baseline gl:core=3.3,gles2=3.0
  --promoted            Include extensions whose commands were promoted into
                        the requested core version. For example, with
                        gl:core=3.3, this adds every extension that was
                        promoted to core in GL 3.3 or earlier, ensuring
                        that both the core and extension-era function names
                        and enums are present in the output. Handles both
                        same-name promotion (e.g. ARB_copy_buffer) and
                        renamed promotion (e.g. ARB_multitexture ->
                        glActiveTexture). Scoped per-API in merged builds.
  --predecessors        Include predecessor extensions of the already-selected
                        set. If an extension's commands or enums are aliases
                        of those in a selected extension, the predecessor is
                        added too. Follows chains to a fixed point, so
                        indirect predecessors are included. Runs after
                        --promoted, so promoted extensions also seed the
                        predecessor search.
  --merge               Merge multiple APIs of the same spec into a single
                        output file. Required when combining gl and gles2.
  --out-path <DIR>      Output directory [default: .]
  --quiet               Suppress progress messages.
  --fetch               Fetch XML specs from upstream Khronos URLs instead
                        of the bundled copies. Any fetch failure is fatal.

Commands:
  c     Generate a C loader.
        --alias   Enable bijective function-pointer alias resolution at
                  load time. If the canonical slot is null but an alias
                  was loaded by the driver (or vice versa), the pointer
                  is propagated to both slots.
        --loader  Include a built-in dlopen/LoadLibrary convenience layer.
```

### Extension selection flags

The extension-related flags are orthogonal and compose freely:

| Flag | What it does |
|------|--------------|
| `--alias` | *Runtime*: propagates loaded function pointers to alias slots at load time |
| `--promoted` | *Selection*: adds extensions promoted into the requested core version |
| `--predecessors` | *Selection*: adds predecessor extensions of the already-selected set |
| `--baseline` | *Selection*: excludes extensions fully promoted into the baseline version |

---

## Generated output

### Context struct and dispatch

Each API gets a context struct (`GloamGLContext`, `GloamVulkanContext`,
etc.) with named members for every loaded function pointer. A global
context variable is declared for each API (`gloam_gl_context`,
`gloam_vk_context`, etc.), and dispatch macros route calls through it:

```c
// This expands to a function pointer in the global context:
glClear(GL_COLOR_BUFFER_BIT);
```

The emitted loader headers have full function prototypes specifically for
IntelliSense, so you get clear hints when developing in Visual Studio.

### Feature and extension presence

The context struct also has named members for checking feature version
support and extension presence at runtime:

```c
if (GLOAM_GL_VERSION_3_3)        { /* GL 3.3 core available */ }
if (GLOAM_GL_ES_VERSION_3_0)     { /* OpenGL ES 3.0 available */ }
if (GLOAM_GL_ARB_draw_indirect)  { /* ARB_draw_indirect available */ }
```

These macros expand to the corresponding named member on the global
context (e.g. `gloam_gl_context.ARB_draw_indirect`).

---

## Vulkan loading

gloam generates two Vulkan loading modes. Most Vulkan applications
should use the enabled-list API.

### Enabled-list mode (recommended)

The caller creates their Vulkan instance and device normally, then
passes the same API version and extension lists to gloam. gloam loads
only those function pointers, sets the corresponding presence flags,
and runs alias resolution. No enumeration calls, no heap allocation,
no ambiguous pointers.

```c
// Phase 0: open the Vulkan library, load global-scope bootstrap functions.
// Pass NULL to let gloam find and open libvulkan; or pass your own handle.
gloamVulkanInitialize(NULL);

// Create your VkInstance as usual...
VkApplicationInfo appInfo = { ... };
VkInstanceCreateInfo instanceCreateInfo = { ... };
vkCreateInstance(&instanceCreateInfo, NULL, &instance);

// Phase 1: load instance-scope functions.
// These are the same values you passed to VkApplicationInfo and
// VkInstanceCreateInfo.
gloamVulkanLoadInstance(instance,
    appInfo.apiVersion,
    instanceCreateInfo.enabledExtensionCount,
    instanceCreateInfo.ppEnabledExtensionNames);

// Create your VkDevice as usual...
VkDeviceCreateInfo deviceCreateInfo = { ... };
vkCreateDevice(physicalDevice, &deviceCreateInfo, NULL, &device);

// Phase 2: load device-scope functions.
// These are the same values you passed to VkDeviceCreateInfo.
gloamVulkanLoadDevice(device, physicalDevice,
    deviceCreateInfo.enabledExtensionCount,
    deviceCreateInfo.ppEnabledExtensionNames);

// ... use Vulkan normally ...

// Cleanup: close library handle and zero context.
gloamVulkanFinalize();
```

Each function also has a `*Context` variant that takes an explicit
context pointer instead of using the global `gloam_vk_context`
(e.g. `gloamVulkanInitializeContext(ctx, NULL)`).

A `gloamVulkanInitializeCustom(getInstanceProcAddr)` variant is
available when you already have a `vkGetInstanceProcAddr` and want to
skip the library-opening step.

### Discovery mode

With `--loader`, gloam also generates a discovery API that calls
`vkEnumerate*ExtensionProperties` to auto-detect available extensions:

```c
gloamLoaderLoadVulkan(NULL, VK_NULL_HANDLE, VK_NULL_HANDLE);
// ... create instance ...
gloamLoaderLoadVulkan(instance, VK_NULL_HANDLE, VK_NULL_HANDLE);
// ... create device ...
gloamLoaderLoadVulkan(instance, physical_device, device);

// Cleanup.
gloamLoaderUnloadVulkan();
```

**Note:** Discovery mode detects which extensions the driver *supports*,
but Vulkan's validity rules require that you *enable* an extension at
instance or device creation before using its commands or structures.
Extension presence in the context does not mean the extension is enabled.
For this reason, the enabled-list API is the recommended approach for
Vulkan — it loads exactly what you enabled, with no ambiguity.

Discovery mode is also slower, because the Vulkan enumeration APIs are
expensive — the loader library scans ICDs and implicit layers on each
call. See [vk-api-loader-shootout] for detailed benchmarks.

For GL, EGL, GLX, and WGL, discovery is the only loading path, and
extension presence means you can use it immediately — no enable step
is required.

### Vulkan command scope inference

gloam infers the correct `vkGet*ProcAddr` entry point from the first
parameter type of each command:

| First parameter | Scope | Loaded via |
|---|---|---|
| (none / non-handle) | Global | `vkGetInstanceProcAddr(NULL, name)` |
| `VkInstance` / `VkPhysicalDevice` | Instance | `vkGetInstanceProcAddr(instance, name)` |
| `VkDevice` / `VkQueue` / `VkCommandBuffer` | Device | `vkGetDeviceProcAddr(device, name)` |

---

## Integration

Add the generated files to your C or C++ project:

1. Add `generated/include` to your include path.
2. Compile the `generated/src/*.c` files alongside your project.
3. Include the appropriate header:
   ```c
   #include <gloam/gl.h>   // GL (or merged GL + GLES2 with --merge)
   #include <gloam/vk.h>   // Vulkan (or vulkan.h without --merge)
   #include <gloam/egl.h>  // EGL
   ```

With `--loader`, the generated code handles library opening
(`dlopen`/`LoadLibrary`) for you. On POSIX platforms you may need to
link `-ldl`; on Windows no extra link flags are needed.

### CMake example

```cmake
target_include_directories(myapp PRIVATE generated/include)
target_sources(myapp PRIVATE
    generated/src/gl.c
    generated/src/vk.c
)

if(UNIX)
    target_link_libraries(myapp PRIVATE dl)
endif()
```

---

## Bundled specs

gloam embeds compile-time snapshots of upstream Khronos XML specs and
auxiliary headers. The binary needs no external files at runtime.

To refresh the bundled copies:

```sh
./scripts/fetch_bundled.sh          # fetch everything
./scripts/fetch_bundled.sh --xml    # XML specs only
./scripts/fetch_bundled.sh --hdrs   # headers only
```

The `--fetch` CLI flag bypasses the bundled copies and fetches specs
directly from upstream at generation time. This may provide fresher
specs, but is less tested and may encounter upstream spec regressions.

---

## Building from source

```sh
cargo build --release
cargo test
```

Requires Rust 1.88 or later. The `fetch` feature (enabled by default)
pulls in `reqwest` for `--fetch` mode; disable with
`--no-default-features` if not needed.

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for architecture details, spec
gotcha documentation, and development guidelines.

---

## License

gloam is licensed under either of [Apache License, Version 2.0](LICENSE-APACHE)
or [MIT license](LICENSE-MIT) at your option.

Generated output includes [xxHash] by Yann Collet, licensed under the
[BSD 2-Clause license](https://github.com/Cyan4973/xxHash/blob/dev/LICENSE).
The xxHash header is output unmodified and retains its original license
notice.

gloam outputs are derived from the Khronos Group XML API Registry
specifications, and are licensed by the Khronos group under the Apache 2.0
license.

gloam also uses the ANGLE-specific extension XML registry files for GLES2 and
EGL outputs, which are owned by Google and licensed under the BSD 3-clause
license.

[xxHash]: https://github.com/Cyan4973/xxHash
