# vk-info-rs

Headless smoke test for the gloam-generated **Rust** Vulkan loader
(`gloam/`, crate `gloam_vk`): bootstraps `vkGetInstanceProcAddr` from the
platform Vulkan library, then walks the phased loading contract —
`initialize` → `load_instance` → `load_device` — and prints what the loader
detected.  `vkGetDeviceQueue` succeeding proves Device-scope commands
resolve through `vkGetDeviceProcAddr`.

```sh
cargo run
```

Exit codes: `0` pass, `1` Vulkan present but something failed, `77` no
usable Vulkan runtime (skip).

The checked-in `gloam/` crate was generated with:

```sh
gloam --api vk=1.3 --extensions VK_KHR_swapchain,VK_KHR_get_physical_device_properties2,VK_EXT_debug_utils,VK_KHR_timeline_semaphore,VK_KHR_synchronization2,VK_KHR_portability_enumeration --out-path examples/rust/vk-info/gloam rust --alias --mx-global
```

and can be refreshed in place with `cargo xtask regen examples`.
