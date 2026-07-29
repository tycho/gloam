//! gl-triangle in Rust: a spinning triangle rendered through a gloam-generated
//! GL loader, with the window + GL context provided by the Rust-native
//! winit + glutin stack.  Uses the --mx-global layer: after one
//! `gl::load_gl_global(...)` (or `gl::load_gles2_global(...)`), every GL call
//! is a free function through the process-global context —
//! `gl::DrawArrays(...)`, no context threading.
//!
//! Three context flavors are supported, mirroring the C example:
//!
//!   OpenGL 3.3 core       ->  gl::load_gl_global(...)
//!   OpenGL ES 3.0         ->  gl::load_gles2_global(...)
//!   OpenGL ES 3.0 ANGLE   ->  gl::load_gles2_global(...) through libEGL
//!
//! With no mode flag, OpenGL 3.3 core is tried first and OpenGL ES 3.0 is
//! the fallback. With an explicit flag there is no fallback — failure to
//! create that context is an error:
//!
//!   --gl                   force OpenGL 3.3 core
//!   --es                   force OpenGL ES 3.0
//!   --use-angle <backend>  force OpenGL ES 3.0 through ANGLE's libEGL;
//!                          backend is one of d3d11, metal, vulkan, opengles.
//!
//! glutin exposes no way to pass ANGLE platform attributes to
//! eglGetPlatformDisplay, so backend selection rides on ANGLE's
//! ANGLE_DEFAULT_PLATFORM environment variable instead (which ANGLE consults
//! exactly when no explicit platform-type attribute was passed). That
//! variable has no value for ANGLE's desktop-GL backend, so unlike the C
//! example, `--use-angle opengl` is not available here.
//!
//! Run with `--ci` to render one frame into a hidden window, verify the center
//! pixel, and exit (how automated environments exercise it).
//!
//! Exit codes: 0 = pass, 1 = failure, 77 = skipped (no usable GL driver).

use gloam_gl as gl;
use gloam_gl::*;
use glutin::config::ConfigTemplateBuilder;
use glutin::context::{
    ContextApi, ContextAttributesBuilder, GlProfile, NotCurrentGlContext, PossiblyCurrentContext,
    Version,
};
use glutin::display::{GetGlDisplay, GlDisplay};
use glutin::surface::{GlSurface, Surface, WindowSurface};
use glutin_winit::{ApiPreference, DisplayBuilder, GlWindow};
use raw_window_handle::HasWindowHandle;
use std::ffi::{c_char, c_void, CStr};
use std::num::NonZeroU32;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

const EXIT_SKIP: i32 = 77;

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Auto,
    Gl,
    Es,
    Angle,
}

struct App {
    ci: bool,
    mode: Mode,
    state: Option<State>,
}

struct State {
    window: Window,
    surface: Surface<WindowSurface>,
    context: PossiblyCurrentContext,
    program: GLuint,
    vao: GLuint,
    angle_loc: GLint,
    angle: f32,
}

/// glutin's `Display` only has an `Egl` variant on platforms with an EGL
/// backend; on macOS it is always CGL, so ANGLE is unreachable there.
#[cfg(not(target_os = "macos"))]
fn display_is_egl(display: &glutin::display::Display) -> bool {
    matches!(display, glutin::display::Display::Egl(_))
}
#[cfg(target_os = "macos")]
fn display_is_egl(_display: &glutin::display::Display) -> bool {
    false
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }

        let attrs = Window::default_attributes()
            .with_title("gloam gl-triangle (Rust)")
            .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
            .with_visible(!self.ci);
        let template = ConfigTemplateBuilder::new().with_alpha_size(8);
        let mut display_builder = DisplayBuilder::new().with_window_attributes(Some(attrs));
        if self.mode == Mode::Angle {
            // Ask for an EGL display (EGL-then-native); with ANGLE's libEGL
            // on the library search path, EGL *is* ANGLE.
            display_builder = display_builder.with_preference(ApiPreference::PreferEgl);
        }
        let (window, gl_config) = display_builder
            .build(event_loop, template, |mut configs| configs.next().unwrap())
            .expect("build display + window");
        let window = window.expect("window");
        let raw = window.window_handle().unwrap().as_raw();
        let gl_display = gl_config.display();

        if self.mode == Mode::Angle && !display_is_egl(&gl_display) {
            eprintln!(
                "gl-triangle: --use-angle needs an EGL display, but glutin selected a \
                 non-EGL API (is ANGLE's libEGL on the library search path? on macOS \
                 glutin only supports CGL, so ANGLE is unavailable)"
            );
            std::process::exit(1);
        }

        // (glutin defaults 3.3 contexts to the core profile on every backend,
        // but say so explicitly rather than lean on the default.)
        let core_33 = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
            .with_profile(GlProfile::Core)
            .build(Some(raw));
        let es_30 = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(Some(Version::new(3, 0))))
            .build(Some(raw));

        let attempts: &[(bool, &glutin::context::ContextAttributes)] = match self.mode {
            Mode::Auto => &[(false, &core_33), (true, &es_30)],
            Mode::Gl => &[(false, &core_33)],
            Mode::Es | Mode::Angle => &[(true, &es_30)],
        };

        let mut created = None;
        for &(es, ctx_attrs) in attempts {
            match unsafe { gl_display.create_context(&gl_config, ctx_attrs) } {
                Ok(context) => {
                    created = Some((es, context));
                    break;
                }
                Err(err) => eprintln!(
                    "gl-triangle: {} context creation failed: {err}",
                    if es {
                        "OpenGL ES 3.0"
                    } else {
                        "OpenGL 3.3 core"
                    }
                ),
            }
        }
        let Some((es, not_current)) = created else {
            if self.mode == Mode::Auto {
                eprintln!("gl-triangle: no GL 3.3 core or ES 3.0 context available, skipping");
                std::process::exit(EXIT_SKIP);
            }
            std::process::exit(1);
        };

        let surf_attrs = window.build_surface_attributes(<_>::default()).unwrap();
        let surface = unsafe {
            gl_display
                .create_window_surface(&gl_config, &surf_attrs)
                .expect("create surface")
        };
        let context = not_current.make_current(&surface).expect("make current");

        // ===== the gloam-generated Rust loader, into the global context =====
        let loaded = if es {
            unsafe { gl::load_gles2_global(|name| gl_display.get_proc_address(name)) }
        } else {
            unsafe { gl::load_gl_global(|name| gl_display.get_proc_address(name)) }
        };
        loaded.expect("gloam load failed");

        let v = gl::version();
        println!(
            "gloam {} -> {} {}.{}",
            if es { "load_gles2" } else { "load_gl" },
            if es { "OpenGL ES" } else { "GL" },
            v >> 8,
            v & 0xff
        );
        let renderer;
        unsafe {
            let get = |e| {
                CStr::from_ptr(gl::GetString(e) as *const c_char)
                    .to_string_lossy()
                    .into_owned()
            };
            renderer = get(GL_RENDERER);
            println!("  GL_VERSION:  {}", get(GL_VERSION));
            println!("  GL_RENDERER: {renderer}");
            println!("  KHR_debug ext: {}", gl::KHR_debug());
            println!(
                "  EXT_texture_filter_anisotropic ext: {}",
                gl::EXT_texture_filter_anisotropic()
            );
        }
        if self.mode == Mode::Angle && !renderer.starts_with("ANGLE") {
            eprintln!(
                "gl-triangle: --use-angle requested, but the EGL driver is not ANGLE \
                 (GL_RENDERER = {renderer})"
            );
            std::process::exit(1);
        }

        let (program, vao, angle_loc) = unsafe { setup(es) };

        // Headless CI: a hidden window never emits RedrawRequested on Windows,
        // so render one frame, verify the center pixel, and exit right here.
        if self.ci {
            let size = window.inner_size();
            let (w, h) = (size.width as i32, size.height as i32);
            let mut px = [0u8; 4];
            unsafe {
                render(program, vao, angle_loc, w, h, 0.0);
                gl::Finish();
                gl::ReadPixels(
                    w / 2,
                    h / 2,
                    1,
                    1,
                    GL_RGBA,
                    GL_UNSIGNED_BYTE,
                    px.as_mut_ptr() as *mut c_void,
                );
            }
            println!("center pixel: {} {} {}", px[0], px[1], px[2]);
            assert!(
                px[0] as u32 + px[1] as u32 + px[2] as u32 > 60,
                "center pixel is background — triangle did not render"
            );
            println!("\ngl-triangle (Rust, winit + glutin): PASS");
            event_loop.exit();
            return;
        }

        let state = State {
            window,
            surface,
            context,
            program,
            vao,
            angle_loc,
            angle: 0.0,
        };
        state.window.request_redraw();
        self.state = Some(state);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(state) = self.state.as_mut() else {
            return;
        };
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let (Some(w), Some(h)) =
                    (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                {
                    state.surface.resize(&state.context, w, h);
                }
            }
            WindowEvent::RedrawRequested => {
                let size = state.window.inner_size();
                let (w, h) = (size.width as i32, size.height as i32);
                state.angle += 0.02;
                unsafe {
                    render(state.program, state.vao, state.angle_loc, w, h, state.angle);
                }
                state.surface.swap_buffers(&state.context).unwrap();
                state.window.request_redraw();
            }
            _ => {}
        }
    }
}

unsafe fn compile(kind: GLenum, src: &CStr) -> GLuint {
    let sh = gl::CreateShader(kind);
    let srcs = [src.as_ptr()];
    gl::ShaderSource(sh, 1, srcs.as_ptr(), core::ptr::null());
    gl::CompileShader(sh);
    let mut ok = 0;
    gl::GetShaderiv(sh, GL_COMPILE_STATUS, &mut ok);
    if ok == 0 {
        let mut log = [0 as GLchar; 1024];
        gl::GetShaderInfoLog(sh, 1024, core::ptr::null_mut(), log.as_mut_ptr());
        panic!(
            "shader compile failed: {}",
            CStr::from_ptr(log.as_ptr()).to_string_lossy()
        );
    }
    sh
}

unsafe fn setup(es: bool) -> (GLuint, GLuint, GLint) {
    const VS_GL: &CStr = c"#version 330 core\nin vec2 a_pos;\nin vec3 a_color;\nuniform float u_angle;\nout vec3 v_color;\nvoid main(){float c=cos(u_angle),s=sin(u_angle);gl_Position=vec4(mat2(c,s,-s,c)*a_pos,0.0,1.0);v_color=a_color;}";
    const FS_GL: &CStr =
        c"#version 330 core\nin vec3 v_color;\nout vec4 o;\nvoid main(){o=vec4(v_color,1.0);}";
    const VS_ES: &CStr = c"#version 300 es\nin vec2 a_pos;\nin vec3 a_color;\nuniform float u_angle;\nout vec3 v_color;\nvoid main(){float c=cos(u_angle),s=sin(u_angle);gl_Position=vec4(mat2(c,s,-s,c)*a_pos,0.0,1.0);v_color=a_color;}";
    const FS_ES: &CStr =
        c"#version 300 es\nprecision mediump float;\nin vec3 v_color;\nout vec4 o;\nvoid main(){o=vec4(v_color,1.0);}";
    let vs = compile(GL_VERTEX_SHADER, if es { VS_ES } else { VS_GL });
    let fs = compile(GL_FRAGMENT_SHADER, if es { FS_ES } else { FS_GL });
    let program = gl::CreateProgram();
    gl::AttachShader(program, vs);
    gl::AttachShader(program, fs);
    gl::BindAttribLocation(program, 0, c"a_pos".as_ptr());
    gl::BindAttribLocation(program, 1, c"a_color".as_ptr());
    gl::LinkProgram(program);
    let mut ok = 0;
    gl::GetProgramiv(program, GL_LINK_STATUS, &mut ok);
    assert!(ok != 0, "program link failed");
    let angle_loc = gl::GetUniformLocation(program, c"u_angle".as_ptr());

    #[rustfmt::skip]
    let verts: [f32; 15] = [
         0.0,  0.6, 1.0, 0.2, 0.2,
        -0.6, -0.5, 0.2, 1.0, 0.2,
         0.6, -0.5, 0.2, 0.2, 1.0,
    ];
    let mut vao = 0;
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);
    let mut vbo = 0;
    gl::GenBuffers(1, &mut vbo);
    gl::BindBuffer(GL_ARRAY_BUFFER, vbo);
    gl::BufferData(
        GL_ARRAY_BUFFER,
        core::mem::size_of_val(&verts) as GLsizeiptr,
        verts.as_ptr() as *const c_void,
        GL_STATIC_DRAW,
    );
    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE, 20, std::ptr::null::<c_void>());
    gl::EnableVertexAttribArray(1);
    gl::VertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE, 20, 8usize as *const c_void);
    (program, vao, angle_loc)
}

unsafe fn render(program: GLuint, vao: GLuint, angle_loc: GLint, w: i32, h: i32, angle: f32) {
    gl::Viewport(0, 0, w, h);
    gl::ClearColor(0.05, 0.05, 0.08, 1.0);
    gl::Clear(GL_COLOR_BUFFER_BIT);
    gl::UseProgram(program);
    gl::Uniform1f(angle_loc, angle);
    gl::BindVertexArray(vao);
    gl::DrawArrays(GL_TRIANGLES, 0, 3);
}

fn usage() -> ! {
    eprintln!(
        "usage: gl-triangle-rs [--ci] [--gl | --es | --use-angle <backend>]\n\
         \x20 --ci                   render one frame headlessly, verify a pixel, exit\n\
         \x20 --gl                   force desktop OpenGL 3.3 core\n\
         \x20 --es                   force OpenGL ES 3.0\n\
         \x20 --use-angle <backend>  force OpenGL ES 3.0 via ANGLE's libEGL;\n\
         \x20                        backend: d3d11, metal, vulkan, opengles"
    );
    std::process::exit(1);
}

fn main() {
    let mut ci = false;
    let mut mode = Mode::Auto;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--ci" => ci = true,
            "--gl" => mode = Mode::Gl,
            "--es" => mode = Mode::Es,
            "--use-angle" => {
                let Some(backend) = args.next() else { usage() };
                // ANGLE consults ANGLE_DEFAULT_PLATFORM when the display is
                // created without explicit platform-type attributes — which
                // is exactly how glutin creates EGL displays.
                let env_value = match backend.as_str() {
                    "d3d11" => "d3d11",
                    "metal" => "metal",
                    "vulkan" => "vulkan",
                    "opengles" => "gl",
                    "opengl" => {
                        eprintln!(
                            "gl-triangle: ANGLE_DEFAULT_PLATFORM has no value for ANGLE's \
                             desktop-GL backend, and glutin passes no ANGLE platform \
                             attributes; use the C example for --use-angle opengl"
                        );
                        std::process::exit(1);
                    }
                    _ => {
                        eprintln!("gl-triangle: unknown ANGLE backend \"{backend}\"");
                        usage()
                    }
                };
                std::env::set_var("ANGLE_DEFAULT_PLATFORM", env_value);
                mode = Mode::Angle;
            }
            _ => usage(),
        }
    }

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App {
        ci,
        mode,
        state: None,
    };
    event_loop.run_app(&mut app).unwrap();
}
