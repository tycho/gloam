# egl-info-rs

Headless smoke test for the gloam-generated **Rust** EGL loader (`gloam/`,
crate `gloam_egl`), mirroring a real ANGLE application's startup: phase-0
`load_egl(EGL_NO_DISPLAY)` detects the *client* extensions, and when
`EGL_ANGLE_platform_angle` is present each ANGLE render backend (D3D11,
Vulkan, desktop OpenGL, OpenGL ES) is selected with
`eglGetPlatformDisplay(EGL_PLATFORM_ANGLE_ANGLE, ...)`, initialized, and
reported.  On a plain EGL implementation (e.g. Mesa) the default display is
used instead.

```sh
# ANGLE (libGLESv2 must sit next to libEGL; do not commit the DLLs):
EGL_LIBRARY=/path/to/angle/libEGL.dll cargo run

# System EGL (Mesa etc.):
cargo run
```

Exit codes: `0` pass (at least one display initialized), `1` EGL present but
nothing worked, `77` no EGL library found (skip).

The checked-in `gloam/` crate was generated with:

```sh
gloam --api egl --out-path examples/rust/egl-info/gloam rust --alias
```

and can be refreshed in place with `cargo xtask regen examples`.
