# gloam examples

Small programs showing generated loaders doing real work. Each example's
`gloam/` directory is checked-in generator output — the normal way to consume
gloam is to generate once and commit the result to your project.

| Example | Language | What it shows |
| --- | --- | --- |
| [c/gl-triangle](c/gl-triangle/) | C | The merged GL + GLES2 loader (the production pattern): an SDL3 window, desktop core-profile context with OpenGL ES fallback, one context struct serving both, `GL_KHR_debug` wired to a debug callback, and a spinning triangle. |
| [c/vk-info](c/vk-info/) | C | The phased Vulkan flow (`Initialize → LoadInstance → LoadDevice`) with the built-in `--loader` opening the platform Vulkan library — headless device info plus a cross-check of gloam's extension flags against what was actually enabled. |
| [rust/gl-triangle](rust/gl-triangle/) | Rust | The generated loader as a crate (`gloam_gl`), consumed via the Rust-native `winit` + `glutin` stack: a desktop GL 3.3 context, `--mx-global` free-function dispatch, and a spinning triangle. |
| [rust/vk-info](rust/vk-info/) | Rust | The phased Vulkan flow on the `gloam_vk` crate's `--mx-global` layer: `vk::initialize → load_instance → load_device`, headless device info, and per-extension presence flags. |
| [rust/egl-info](rust/egl-info/) | Rust | The `gloam_egl` crate probing EGL client extensions, then initializing each ANGLE render backend via `EGL_ANGLE_platform_angle` (or the default display on plain EGL). |

## Building & running

### C examples (CMake)

```sh
cmake -B build -S examples/c
cmake --build build
./build/gl-triangle/gl-triangle          # interactive spinning triangle
./build/gl-triangle/gl-triangle --ci     # one hidden frame + pixel check
./build/gl-triangle/gl-triangle --es     # force the OpenGL ES fallback path
./build/vk-info/vk-info                  # headless device + extension table
```

SDL3 is used from the system when available and otherwise fetched and built
from source. `vk-info` has no dependencies at all. Each program exits `0` on
success, `1` on failure, and `77` when the machine has no usable driver (the
automake "skip" convention), so they are safe to run in CI.

### Rust examples (Cargo)

```sh
cd examples/rust/gl-triangle
cargo run            # interactive spinning triangle
cargo run -- --ci    # one headless frame + pixel check (exit 0 = pass)

cd ../vk-info
cargo run            # headless device info + extension flags

cd ../egl-info
cargo run            # EGL / ANGLE backend probe (see its README)
```

Each Rust example is a standalone Cargo workspace, so example-only
dependencies (`winit`, `glutin`, ...) stay out of the main gloam build.

## Regenerating the loaders

The exact command line for each loader is recorded in its
`gloam/.gloam/manifest.json`, with `--out-path` relative to the language
directory. They were generated with:

```sh
# from examples/c
gloam --api gl:core=3.3,gles2=3.0 --merge \
      --extensions GL_KHR_debug,GL_EXT_texture_filter_anisotropic \
      --out-path gl-triangle/gloam c --alias
gloam --api vk=1.3 \
      --extensions VK_KHR_swapchain,VK_KHR_get_physical_device_properties2,VK_EXT_debug_utils,VK_KHR_timeline_semaphore,VK_KHR_synchronization2,VK_KHR_portability_enumeration \
      --out-path vk-info/gloam c --loader

# from the repo root
gloam --api gl:core=3.3,gles2=3.0 --merge \
      --out-path examples/rust/gl-triangle/gloam rust --alias --mx-global
gloam --api vk=1.3 \
      --extensions VK_KHR_swapchain,VK_KHR_get_physical_device_properties2,VK_EXT_debug_utils,VK_KHR_timeline_semaphore,VK_KHR_synchronization2,VK_KHR_portability_enumeration \
      --out-path examples/rust/vk-info/gloam rust --alias --mx-global
gloam --api egl --out-path examples/rust/egl-info/gloam rust --alias
```

`cargo xtask regen examples` re-runs the recorded commands with the
working-copy gloam, regenerating every tree in place (the output path is
derived from each `.gloam/manifest.json` location, so the recorded
`--out-path` values above don't need to match your cwd).  Add `--fresh` to
advance the trees to the current bundle instead of pinning to their
recorded provenance — CI does this automatically when the weekly bundle
update changes example output.
