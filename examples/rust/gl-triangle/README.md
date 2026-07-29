# gl-triangle (Rust)

A spinning triangle rendered through a **gloam-generated Rust GL loader**, with
the window and GL context provided by the Rust-native
[`winit`](https://crates.io/crates/winit) + [`glutin`](https://crates.io/crates/glutin)
stack — no C libraries to locate or link.

`gloam/` is the generated loader crate (`gloam_gl`), consumed here as a path
dependency. `src/main.rs` opens a window, creates a GL context, loads it with
`gl::load_gl_global(...)` or `gl::load_gles2_global(...)` (the `--mx-global`
layer — one merged loader serves both APIs), and draws the triangle with
free-function dispatch: `gl::DrawArrays(GL_TRIANGLES, 0, 3)` after
`use gloam_gl as gl` — no context value threaded through the code.
(The owned-context form, `let gl = Gl::load_gl(...)?` with `gl.DrawArrays(...)`
methods, is always generated too.)

## Run

```sh
cargo run            # open a window with a spinning triangle
cargo run -- --ci    # render one frame headlessly, verify a pixel, exit (0 = pass)
```

With no mode flag, a desktop OpenGL 3.3 core context is tried first and
OpenGL ES 3.0 is the fallback (matching the C example). An explicit mode
flag disables the fallback — if that context can't be created, the run
fails:

```sh
cargo run -- --gl                   # force desktop OpenGL 3.3 core
cargo run -- --es                   # force OpenGL ES 3.0
cargo run -- --use-angle <backend>  # force OpenGL ES 3.0 through ANGLE's libEGL
                                    # backend: d3d11, metal, vulkan, opengles
```

`--use-angle` needs ANGLE's `libEGL` (and `libGLESv2`) on the dynamic
linker's search path — e.g. next to the executable. glutin exposes no way
to pass ANGLE platform attributes to `eglGetPlatformDisplay`, so the
backend is selected through ANGLE's `ANGLE_DEFAULT_PLATFORM` environment
variable, which ANGLE consults exactly when no explicit platform-type
attribute was passed. Two consequences, both diverging from the C example
(which passes real platform attributes through SDL's EGL attribute
callbacks): `--use-angle opengl` is unavailable (the variable has no
desktop-GL value), and on macOS `--use-angle` fails outright (glutin only
supports CGL there).

## The `unsafe` story

A thin GL loader cannot honestly make dispatch safe — whether a call is sound
depends on driver state (is a matching context current on this thread?) that
the borrow checker cannot see, which is why every Rust GL binding marks its
calls `unsafe`. The idiomatic pattern, used here, is to keep whole GL-touching
phases inside `unsafe fn`s (`setup`, `render`): one `unsafe` boundary per
phase, not one block per call.

Two loader details take most of the residual pain out of that boundary:

- **Checked dispatch by default.** Calling a function the context didn't
  provide panics with the function's name (instead of jumping through a null
  pointer). Once an app is proven, build the loader crate with its `no-error`
  cargo feature — the KHR_no_error idea at compile time — to remove the
  per-call check entirely.
- **Typed constants.** `GL_*_BIT` constants are `GLbitfield`, booleans are
  `GLboolean`, everything else is the `GLenum` newtype — so mismatched
  arguments fail at compile time, with no casts at correct call sites.

## Regenerate the loader

`gloam/` is checked in, but you can regenerate it from the repo root:

```sh
cargo run -- --api gl:core=3.3,gles2=3.0 --merge --out-path examples/rust/gl-triangle/gloam rust --alias --mx-global
```
