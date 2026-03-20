# gloam

A loader generator for Vulkan, OpenGL, OpenGL ES, EGL, GLX, and WGL.
Reads Khronos XML spec files and generates C or Rust dispatch code.

gloam is a clean-room rewrite of [GLAD](https://github.com/Dav1dde/glad) in Rust,
with first-class support for ANGLE extension supplementals, Vulkan command-scope
inference, and a hash-based extension detection strategy that avoids string
comparisons at load time.

---

## Quick start

```sh
# 1. Populate bundled XML specs and headers (one-time setup, or to refresh).
./scripts/fetch_bundled.sh

# 2. Build.
cargo build --release

# 3. Generate a C loader for OpenGL 3.3 core + all extensions.
./target/release/gloam --api gl:core=3.3 c --loader

# 4. Generate a Rust loader for Vulkan 1.3.
./target/release/gloam --api vk=1.3 rust

# 5. Generate a combined GL 3.3 + GLES 2.0 + Vulkan + EGL loader.
./target/release/gloam --api gl:core,gles2,vulkan,egl --merge c --alias --loader
```

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
  --merge               Merge multiple APIs of the same spec into a single
                        output file. Required when combining gl and gles2.
  --out-path <DIR>      Output directory [default: .]
  --quiet               Suppress progress messages.
  --fetch               Fetch XML specs from upstream Khronos URLs instead
                        of the bundled copies. Any fetch failure is fatal.

Commands:
  c     Generate a C loader.
        --alias   Enable bijective function-pointer alias resolution at
                  load time. The alias table is always built; this flag
                  controls whether the runtime resolver loop is emitted.
        --loader  Include a built-in dlopen/LoadLibrary convenience layer.
  rust  Generate a Rust loader.
        --alias   Enable bijective function-pointer alias resolution.
```

---

## Generated C output

For `--api gl:core=3.3 c --loader`, gloam emits into the output directory:

```
include/
  gloam/
    gl.h          # public header — include this in your project
  KHR/
    khrplatform.h # auxiliary headers copied from bundled/
src/
  gl.c            # loader implementation
```

**`include/gloam/gl.h`**
- `GloamAPIProc` opaque function pointer type and `GloamLoadFunc` callback typedef
- `APIENTRY` / `APIENTRYP` / `GLOAM_API_PTR` / `GLAPIENTRY` calling convention macros
- `typedef` declarations in topological dependency order
- `#define` constants for all required enums
- PFN typedef for every selected command (e.g. `PFNGLACTIVETEXTUREPROC`)
- `GloamGLContext` struct with:
  - `featArray[]` — one `unsigned char` flag per version feature
  - `extArray[]` — one `unsigned char` flag per extension
  - `pfnArray[]` / named union members — one function pointer slot per command
- Feature presence macros (`GLOAM_GL_VERSION_3_3`, …)
- Extension presence macros (`GLOAM_GL_ARB_SYNC`, …)
- `#define glFoo (gloam_gl_context.Foo)` dispatch macros (or prototypes under `__INTELLISENSE__`)
- `gloamLoadGLContext(ctx, getProcAddr)` / `gloamLoadGL(getProcAddr)` declarations
- With `--loader`: `gloamLoaderLoadGLContext` / `gloamLoaderUnloadGLContext` /
  `gloamLoaderResetGLContext` (and non-Context variants) declarations

**`src/gl.c`**
- Static function-name string table (`kFnNames[]`)
- Feature PFN range table (contiguous-run compressed)
- Per-API extension hash table (pre-baked XXH3-64, sorted for binary search)
- Extension PFN range table and index subset per API
- `gloamLoadGLContext()`:
  1. Bootstraps `glGetString` then calls `find_core` to parse `GL_VERSION` and
     set `featArray` bits
  2. Bulk-loads function pointers for each enabled feature via the range table
  3. Hashes driver-reported extension names, Shellsorts them, binary-searches
     against the pre-baked table, sets `extArray` bits for matches
  4. Loads function pointers for each detected extension via the range table
  5. With `--alias`: resolves bijective function-pointer alias pairs
- With `--loader`: `gloamLoaderLoad/Unload/ResetGLContext()` — opens the platform
  GL library, wires up `wglGetProcAddress` / `glXGetProcAddressARB` with correct
  calling conventions, delegates to `gloamLoadGLContext`, stores the library
  handle in `context->glad_loader_handle`

### Extension detection strategy

At load time, the generated code:

1. Calls `glGetIntegerv(GL_NUM_EXTENSIONS, &n)` to get the count.
2. Calls `glGetStringi(GL_EXTENSIONS, i)` for each `i`, hashes each name
   with XXH3-64 (the same algorithm used at generator time), and stores
   the hashes in a heap-allocated `uint64_t[]`.
3. Shellsorts the array in-place (Ciura gap sequence — no extra memory,
   ~160 bytes of code).
4. Binary-searches the sorted driver hashes against the pre-baked known
   extension hash table embedded in the generated source.

This gives O(n log n) total work to detect all extensions, with O(log n)
per lookup, and zero string comparisons at runtime.

---

## Vulkan loading

Vulkan function loading requires knowing which `vkGet*ProcAddr` entry point
to use for each command.  gloam infers this from the first parameter type:

| First parameter         | Scope    | Loaded via                          |
|-------------------------|----------|-------------------------------------|
| (none / non-handle)     | Global   | `vkGetInstanceProcAddr(NULL, name)` |
| `VkInstance` / `VkPhysicalDevice` | Instance | `vkGetInstanceProcAddr(instance, name)` |
| `VkDevice` / `VkQueue` / `VkCommandBuffer` | Device | `vkGetDeviceProcAddr(device, name)` |
| `vkGetInstanceProcAddr` itself | Unknown | `dlsym` / `GetProcAddress`    |

### Multi-call loading model

`gloamLoadVulkanContext` (and `gloamLoaderLoadVulkanContext`) may be called
multiple times on the same context as the application progresses through
Vulkan initialisation.  Each call is **additive** — it fills in whatever
the current set of live handles allows, without wiping previous work:

```c
// Pass 1: all handles NULL — loads Global-scope functions,
//         detects available instance extensions.
gloamLoaderLoadVulkanContext(ctx, NULL, NULL, NULL);

// ... create VkInstance based on detected extensions ...

// Pass 2: instance live — loads Instance-scope functions,
//         detects available device extensions.
gloamLoaderLoadVulkanContext(ctx, instance, NULL, NULL);

// ... create VkPhysicalDevice / VkDevice ...

// Pass 3: device live — loads Device-scope functions.
gloamLoaderLoadVulkanContext(ctx, instance, physical_device, device);
```

### Caller-managed library handle

The library handle is stored in `context->glad_loader_handle` so you can
supply your own:

```c
// Pre-populate to skip dlopen:
ctx->glad_loader_handle = my_vk_handle;
gloamLoaderLoadVulkanContext(ctx, NULL, NULL, NULL);

// During cleanup, zero the field first to prevent gloam from closing
// a handle you still own:
ctx->glad_loader_handle = NULL;
gloamLoaderUnloadVulkanContext(ctx);
```

`glad_loader_handle` is present on **all** context types (GL, EGL, VK, …)
so this pattern is consistent across APIs.

---

## Bundled files

`bundled/` contains compile-time snapshots of upstream XML specs and
auxiliary headers.  They are embedded into the binary via `include_str!`
so the binary is fully self-contained.

To refresh them:

```sh
./scripts/fetch_bundled.sh          # fetch everything
./scripts/fetch_bundled.sh --xml    # XML specs only
./scripts/fetch_bundled.sh --hdrs   # headers only
```

The `--fetch` CLI flag bypasses the bundled copies and fetches directly
from upstream at generation time.  Any failure is fatal.

---

## Alias resolution (`--alias`)

Alias pairs (e.g. `glActiveTexture` / `glActiveTextureARB`) are always
discovered and stored in the feature set.  When `--alias` is passed, the
generated loader also emits a runtime resolver: if the canonical slot is
null but an alias was loaded by the driver (or vice versa), the loaded
pointer is propagated to both slots.  This is useful for extension
functions that were later promoted to core under a new name.

---

## Building from source

```sh
cargo build            # debug
cargo build --release  # release
cargo test             # (tests not yet written)
```

Requires Rust 1.75 or later.
