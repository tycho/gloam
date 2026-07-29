# The generated C loader — consumer guide

> **Status:** implemented. This document describes the C output — what gloam
> generates, how to build it into a project, and the runtime API it exposes.
> The Rust output is covered by [rust-loader.md](rust-loader.md); the
> `.gloam/manifest.json` written alongside every tree is covered by
> [manifest.md](manifest.md).

The C backend emits one header/source pair per requested spec, plus the
auxiliary Khronos headers those files include:

```
<out-path>/
  .gloam/manifest.json    # provenance + the exact command line (see manifest.md)
  include/
    gloam/gl.h            # one public header per spec stem
    gloam/vk.h
    ...
    KHR/khrplatform.h     # auxiliary headers, only those the output needs
    vk_platform.h
    xxhash.h
  src/
    gl.c                  # one implementation file per spec stem
    vk.c
    ...
```

The stem is the spec name (`gl`, `gles2`, `vk`, `egl`, `glx`, `wgl`); a
merged GL + GLES2 request (`--merge`) produces a single `gl` pair serving
both APIs. Everything in the tree is self-contained — no system Khronos
headers are required (Vulkan can opt back into them with
`--external-headers`).

To integrate: add `include/` to the include path, compile the `src/*.c`
files with the rest of the project, and include `<gloam/gl.h>` (or the
stem you generated). On POSIX platforms `--loader` output needs `-ldl`;
nothing else links against anything.

---

## 1. Contexts and dispatch

Each spec gets a context struct (`GloamGLContext`, `GloamVulkanContext`,
...) holding every function pointer, the feature/extension presence flags,
and the detected version. A global instance of each is defined
(`gloam_gl_context`, `gloam_vk_context`, ...), and the API names dispatch
through it:

```c
glClear(GL_COLOR_BUFFER_BIT);       /* macro → gloam_gl_context.Clear(...) */
vkCmdDraw(cmd, 3, 1, 0, 0);         /* force-inlined fn → gloam_vk_context */
```

GL, GLES, EGL, GLX, and WGL dispatch uses `#define` macros. Vulkan
dispatch uses `static` force-inlined functions instead, so the names
survive coexistence with the upstream Vulkan headers and macro-heavy
libraries such as the Vulkan Memory Allocator. Either way a call compiles
to one load from the global context and an indirect call — there is no
per-call presence check, so calling a function the context did not load
jumps through a null pointer. Check the presence flags first (§3).

Every loader entry point comes in two forms: the plain form operating on
the global context, and a `*Context` form taking an explicit
`Gloam*Context *` for applications juggling several contexts at once
(`gloamLoadGLContext(&ctx, ...)`, `gloamVulkanLoadInstanceContext(&ctx,
...)`, ...). Dispatch macros always route through the *global* context;
with an explicit context you call the members directly
(`ctx.Clear(...)`).

Loading writes to the context are plain unsynchronized stores. Do the
load phases before fanning out to threads (or provide your own
happens-before edge); afterwards all dispatch is read-only and freely
concurrent.

## 2. Loading — GL, GLES, EGL, GLX, WGL

Each windowing/GL spec loads from a `GetProcAddress`-style callback you
provide, detecting the version first and extensions second:

```c
typedef void (*GloamAPIProc)(void);
typedef GloamAPIProc (*GloamLoadFunc)(const char *name);

int gloamLoadGL   (GloamLoadFunc getProcAddr);
int gloamLoadGLES2(GloamLoadFunc getProcAddr);                          /* merged gl output */
int gloamLoadEGL  (EGLDisplay display, GloamLoadFunc getProcAddr);
int gloamLoadWGL  (HDC hdc, GloamLoadFunc getProcAddr);
int gloamLoadGLX  (Display *display, int screen, GloamLoadFunc getProcAddr);
```

The return value is the detected version packed as `major << 8 | minor`
(`0x0303` for GL 3.3), or `0` when no usable API was found. The extra
parameters are what each API's extension query needs: EGL queries client
extensions when `display` is `EGL_NO_DISPLAY` and display extensions
otherwise (call it again after `eglInitialize` to pick those up), WGL
needs a device context for `wglGetExtensionsStringARB`, and GLX needs the
display and screen for `glXQueryExtensionsString`.

Load calls are additive and may be repeated — e.g. re-run `gloamLoadEGL`
with a real display after initializing it, or reload after switching
contexts. A matching context must be current on the calling thread for
GL/GLES/WGL, since version and extension detection go through the driver.

Extension detection never string-compares at load time: the loader hashes
each advertised extension name with XXH3 and binary-searches a pre-baked
sorted hash table. This is the main reason context initialization is fast;
it is invisible in the API.

## 3. Version and extension presence

The context records what was detected, exposed through generated macros
that read the global context (explicit-context users read the members
directly):

```c
if (GLOAM_GL_VERSION_3_3)       { /* GL 3.3 core available */ }
if (GLOAM_GL_ES_VERSION_3_0)    { /* OpenGL ES 3.0 available */ }
if (GLOAM_GL_KHR_debug)         { /* extension detected */ }
```

For GL, EGL, GLX, and WGL, presence means usable — no enable step exists
in those APIs. For Vulkan, presence flags are only set for extensions you
actually *enabled* (§4), matching Vulkan's validity rules.

## 4. Loading — Vulkan

Vulkan loads in phases mirroring the object lifecycle, and most
applications should use the **enabled-list** flow: tell gloam exactly
what you enabled, and it loads exactly those pointers — no enumeration
calls, no heap allocation, no ambiguous "supported but not enabled"
pointers.

```c
gloamVulkanInitialize(NULL);                    /* phase 0: open libvulkan, global PFNs */

vkCreateInstance(&instanceCreateInfo, NULL, &instance);
gloamVulkanLoadInstance(instance,               /* phase 1: instance-scope PFNs */
    appInfo.apiVersion,
    instanceCreateInfo.enabledExtensionCount,
    instanceCreateInfo.ppEnabledExtensionNames);

vkCreateDevice(physicalDevice, &deviceCreateInfo, NULL, &device);
gloamVulkanLoadDevice(device, physicalDevice,   /* phase 2: device-scope PFNs */
    deviceCreateInfo.enabledExtensionCount,
    deviceCreateInfo.ppEnabledExtensionNames);

/* ... */
gloamVulkanFinalize();                          /* close library, zero context */
```

The version and extension lists are the same values you passed to
`VkApplicationInfo` / `Vk*CreateInfo` — no separate bookkeeping. Each
phase returns nonzero on success. Details:

- `gloamVulkanInitialize(NULL)` finds and opens the platform Vulkan
  library (`vulkan-1.dll`, `libvulkan.so.1`, `libvulkan.dylib`); pass
  your own library handle instead to control the open. If you already
  have a `vkGetInstanceProcAddr`, skip the library entirely with
  `gloamVulkanInitializeCustom(getInstanceProcAddr)`.
- `gloamVulkanGetInstanceVersion()` wraps `vkEnumerateInstanceVersion`
  after phase 0 (reporting 1.0 on loaders that predate it), for choosing
  `apiVersion` before instance creation.
- The applied version is capped by what the driver reports, so requesting
  1.3 against a 1.2 device loads the 1.2 command set.
- Commands load through the right entry point automatically: gloam infers
  global/instance/device scope from each command's first parameter type
  and uses `vkGetInstanceProcAddr` or `vkGetDeviceProcAddr` accordingly,
  so device-scope calls skip the loader trampoline.

**Phase 1.5 (rare):** some device extensions carry instance-scope query
commands that must be callable *before* device creation —
`vkGetPhysicalDeviceFragmentShadingRatesKHR` is the canonical example.
`gloamVulkanLoadPhysicalDeviceExtension(name)` (singular) and
`gloamVulkanLoadPhysicalDeviceExtensions(count, names)` (plural) pre-load
those commands between phases 1 and 2. They do **not** set presence
flags; only `gloamVulkanLoadDevice` does, once the extension is actually
enabled.

**Discovery mode** (generated with `--loader`) enumerates instead of
being told:

```c
gloamLoaderLoadVulkan(NULL, VK_NULL_HANDLE, VK_NULL_HANDLE);
gloamLoaderLoadVulkan(instance, VK_NULL_HANDLE, VK_NULL_HANDLE);
gloamLoaderLoadVulkan(instance, physical_device, device);
gloamLoaderUnloadVulkan();
```

Each call loads what the passed handles allow, calling
`vkEnumerate*ExtensionProperties` to detect extensions. Two caveats make
enabled-list the recommended flow: detection means *supported*, not
*enabled* (using a detected-but-unenabled extension violates Vulkan
validity), and the enumeration calls are expensive — the Khronos loader
rescans ICDs and layers on each one.

## 5. The built-in loader layer (`--loader`)

With `--loader`, each spec also gets a convenience layer that opens the
platform library and resolves proc addresses itself, so an application
with no windowing-toolkit `GetProcAddress` can still load:

```c
int  gloamLoaderLoadGL(void);      /* dlopen/LoadLibrary + gloamLoadGL   */
void gloamLoaderUnloadGL(void);    /* close library, zero context        */
void gloamLoaderResetGL(void);     /* zero context, keep library open    */
```

The same triple exists for every stem (`...LoadGLES2`, `...LoadEGL`,
`...LoadWGL`, `...LoadGLX`, `...LoadVulkan`), each with a `*Context`
variant. Load calls are additive and repeatable, like the underlying
`gloamLoad*` functions; `Reset` clears detection state without the
library-close/reopen cost, for context switches.

## 6. Alias resolution (`--alias`)

Many commands exist under both a core and an extension spelling
(`glDebugMessageCallback` / `glDebugMessageCallbackKHR`). With `--alias`,
after loading, any pointer present under one spelling is propagated to
the other — bijectively, in both directions — so application code can use
whichever name it prefers regardless of which one the driver exports.
Without `--alias` each slot holds exactly what the driver returned for
that exact name.

## 7. Merged loaders (`--merge`)

`--merge` with `gl` and `gles2` produces one loader — one context, one
header, one set of dispatch macros — serving whichever API the current
context speaks. Call `gloamLoadGL` after creating a desktop context or
`gloamLoadGLES2` after an ES context; shared commands occupy the same
slots either way, and extension detection is scoped per-API, so a
GL-only extension is never reported present under GLES. This is the
production pattern for applications that run on desktop GL and on GLES
(natively or via ANGLE) from one binary — see
[examples/c/gl-triangle](../examples/c/gl-triangle/).

## 8. Regenerating

Every tree records the command line that produced it in
`.gloam/manifest.json`, so `gloam regen <tree>` reproduces it and
`gloam regen --fresh <tree>` advances it to current spec content. See
[manifest.md](manifest.md) for the format and
[README](../README.md#provenance-and-reproducible-builds) for the
pinning workflow.
