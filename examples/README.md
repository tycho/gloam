# gloam examples

Two small C programs showing generated loaders doing real work. Each
example's `gloam/` directory is checked-in generator output — the normal way
to consume gloam is to generate once and commit the result to your project.

| Example | What it shows |
| --- | --- |
| [gl-triangle](gl-triangle/) | The merged GL + GLES2 loader (the production pattern): an SDL3 window, desktop core-profile context with OpenGL ES fallback, one context struct serving both, `GL_KHR_debug` wired to a debug callback, and a spinning triangle. |
| [vk-info](vk-info/) | The phased Vulkan flow (`Initialize → LoadInstance → LoadDevice`) with the built-in `--loader` opening the platform Vulkan library — headless device info plus a cross-check of gloam's extension flags against what was actually enabled. |

## Building

```sh
cmake -B build -S examples
cmake --build build
```

SDL3 is used from the system when available and otherwise fetched and built
from source (first configure takes a few minutes in that case). `vk-info`
has no dependencies at all — the generated loader opens `vulkan-1.dll` /
`libvulkan.so.1` itself.

## Running

Both programs exit `0` on success, `1` on failure, and `77` when the machine
has no usable driver (the automake "skip" convention), so they are safe to
run in CI:

```sh
./build/gl-triangle/gl-triangle          # interactive spinning triangle
./build/gl-triangle/gl-triangle --ci     # one hidden frame + pixel check
./build/gl-triangle/gl-triangle --es     # force the OpenGL ES fallback path
./build/vk-info/vk-info                  # headless device + extension table
```

## Regenerating the loaders

The exact command line for each loader is recorded in its
`gloam/.gloam/manifest.json`, with paths relative to this directory. They
were generated from `examples/` with:

```sh
gloam --api gl:core=3.3,gles2=3.0 --merge \
      --extensions GL_KHR_debug,GL_EXT_texture_filter_anisotropic \
      --out-path gl-triangle/gloam c --alias

gloam --api vk=1.3 \
      --extensions VK_KHR_swapchain,VK_KHR_get_physical_device_properties2,VK_EXT_debug_utils,VK_KHR_timeline_semaphore,VK_KHR_synchronization2 \
      --out-path vk-info/gloam c --loader
```

`cargo xtask regen examples` re-runs both recorded commands with the
working-copy gloam.
