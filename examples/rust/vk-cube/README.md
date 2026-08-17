# vk-cube (Rust)

A spinning cube rendered through a **gloam-generated Rust Vulkan loader**
(`gloam/`, crate `gloam_vk`) — the Vulkan analogue of gl-triangle, with the
pieces a real application needs: a swapchain (recreated on resize), a depth
buffer, explicit image-layout barriers around Vulkan 1.3 dynamic rendering,
per-frame synchronization (two frames in flight), and push-constant
transforms.

Where the phased loading contract in vk-info runs on the `--mx-global`
layer, this example uses the owned-context form: `Vk::new()` →
`initialize(gipa)` → `load_instance(...)` → `load_device(...)`, with every
command dispatched as a method on the `Vk` value. The window comes from the
Rust-native [`winit`](https://crates.io/crates/winit), and the surface is
created through the loader's *own* platform commands
(`vkCreateWin32SurfaceKHR` / Xlib / Wayland, selected from the
`raw-window-handle` at runtime) — no windowing-library Vulkan glue involved.
The counterpart C example ([../../c/vk-cube](../../c/vk-cube/)) goes the
other way and lets SDL3 create the surface.

The instance opts into portability enumeration when the loader offers it,
and the device enables `VK_KHR_portability_subset` when advertised
(spec-required), so MoltenVK works out of the box.

## Run

```sh
cargo run            # open a window with a spinning cube
cargo run -- --ci    # render one frame hidden, verify a pixel, exit (0 = pass)
```

Exit codes: `0` pass, `1` Vulkan present but something failed, `77` no
usable Vulkan runtime or device (skip).

## Shaders

`shaders/` holds the GLSL sources and the SPIR-V binaries compiled from
them (embedded into the binary with `include_bytes!`). To rebuild after
editing the GLSL:

```sh
glslc -O shaders/cube.vert -o shaders/cube.vert.spv
glslc -O shaders/cube.frag -o shaders/cube.frag.spv
```

## Regenerate the loader

The checked-in `gloam/` crate was generated with:

```sh
gloam --api vk=1.3 --extensions VK_KHR_surface,VK_KHR_swapchain,VK_KHR_win32_surface,VK_KHR_xlib_surface,VK_KHR_wayland_surface,VK_KHR_get_physical_device_properties2,VK_KHR_portability_enumeration,VK_KHR_portability_subset --out-path examples/rust/vk-cube/gloam rust --alias
```

and can be refreshed in place with `cargo xtask regen examples`.
