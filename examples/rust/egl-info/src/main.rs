//! egl-info-rs — exercise the gloam-generated Rust EGL loader end to end.
//!
//! Mirrors Darwinia's ANGLE flow: phase-0 `load_egl(EGL_NO_DISPLAY)` detects
//! the *client* extensions; when `EGL_ANGLE_platform_angle` is present, each
//! ANGLE render backend (D3D11, Vulkan, desktop GL, GLES) is selected via
//! `eglGetPlatformDisplay(EGL_PLATFORM_ANGLE_ANGLE, ..., [EGL_PLATFORM_ANGLE_TYPE_ANGLE,
//! <type>, EGL_NONE])`, initialized, and reported.  On a plain EGL
//! implementation (e.g. Mesa) the default display is used instead.
//!
//! libEGL is found via the `EGL_LIBRARY` environment variable (point it at
//! an ANGLE build's libEGL.dll — libGLESv2.dll must sit next to it) or the
//! platform default names.
//!
//! Exit codes: 0 = pass (at least one display initialized), 1 = EGL present
//! but nothing worked, 77 = no EGL library found (skip).

use std::ffi::CStr;
use std::process::ExitCode;
use std::ptr;

use gloam_egl as egl;

const LIB_NAMES: &[&str] = &[
    #[cfg(target_os = "windows")]
    "libEGL.dll",
    #[cfg(not(target_os = "windows"))]
    "libEGL.so.1",
    #[cfg(not(target_os = "windows"))]
    "libEGL.so",
];

fn open_egl() -> Option<libloading::Library> {
    if let Ok(path) = std::env::var("EGL_LIBRARY") {
        return unsafe { libloading::Library::new(path).ok() };
    }
    LIB_NAMES
        .iter()
        .find_map(|name| unsafe { libloading::Library::new(name).ok() })
}

/// GetProcAddress-style loader over the library: dlsym first, then
/// eglGetProcAddress for extension entry points (mirroring the C built-in
/// loader's fallback order).
struct Loader {
    lib: libloading::Library,
    get_proc: Option<unsafe extern "system" fn(*const std::ffi::c_char) -> *const std::ffi::c_void>,
}

impl Loader {
    fn load(&self, name: &CStr) -> *const std::ffi::c_void {
        let sym = unsafe {
            self.lib
                .get::<*const std::ffi::c_void>(name.to_bytes_with_nul())
                .map(|s| *s)
                .unwrap_or(ptr::null())
        };
        if !sym.is_null() {
            return sym;
        }
        match self.get_proc {
            Some(gpa) => unsafe { gpa(name.as_ptr()) },
            None => ptr::null(),
        }
    }
}

fn main() -> ExitCode {
    let Some(lib) = open_egl() else {
        eprintln!("egl-info-rs: no EGL library found (set EGL_LIBRARY or install one); skip");
        return ExitCode::from(77);
    };
    let get_proc = unsafe {
        lib.get::<unsafe extern "system" fn(*const std::ffi::c_char) -> *const std::ffi::c_void>(
            b"eglGetProcAddress\0",
        )
        .map(|s| *s)
        .ok()
    };
    let loader = Box::leak(Box::new(Loader { lib, get_proc }));

    // Phase 0: client version + client extensions at EGL_NO_DISPLAY.
    let client = match unsafe { egl::Egl::load_egl(egl::EGL_NO_DISPLAY, |n| loader.load(n)) } {
        Ok(c) => c,
        Err(e) => {
            eprintln!("egl-info-rs: phase-0 load failed: {e}");
            return ExitCode::from(1);
        }
    };
    println!(
        "client EGL {}.{}",
        client.version() >> 8,
        client.version() & 0xff
    );
    println!("EGL_ANGLE_platform_angle: {}", client.ANGLE_platform_angle());

    let mut passed = 0u32;
    if client.ANGLE_platform_angle() {
        // ANGLE backend-selection flow (mirrors Darwinia).
        let backends: &[(&str, egl::EGLenum, bool)] = &[
            ("D3D11", egl::EGL_PLATFORM_ANGLE_TYPE_D3D11_ANGLE, client.ANGLE_platform_angle_d3d()),
            ("Vulkan", egl::EGL_PLATFORM_ANGLE_TYPE_VULKAN_ANGLE, client.ANGLE_platform_angle_vulkan()),
            ("OpenGL", egl::EGL_PLATFORM_ANGLE_TYPE_OPENGL_ANGLE, client.ANGLE_platform_angle_opengl()),
            ("OpenGLES", egl::EGL_PLATFORM_ANGLE_TYPE_OPENGLES_ANGLE, client.ANGLE_platform_angle_opengl()),
        ];
        println!("ANGLE backends:");
        for &(label, ty, advertised) in backends {
            if !advertised {
                println!("  {label:<12} not advertised");
                continue;
            }
            let attribs: [egl::EGLAttrib; 5] = [
                egl::EGL_PLATFORM_ANGLE_TYPE_ANGLE as egl::EGLAttrib,
                ty as egl::EGLAttrib,
                egl::EGL_NONE as egl::EGLAttrib,
                egl::EGL_NONE as egl::EGLAttrib,
                egl::EGL_NONE as egl::EGLAttrib,
            ];
            let display = unsafe {
                client.GetPlatformDisplay(
                    egl::EGL_PLATFORM_ANGLE_ANGLE,
                    egl::EGL_DEFAULT_DISPLAY as *mut std::ffi::c_void,
                    attribs.as_ptr(),
                )
            };
            if display == egl::EGL_NO_DISPLAY {
                println!("  {label:<12} no display");
                continue;
            }
            if report_display_with(label, &client, display, loader) {
                passed += 1;
            }
        }
    } else {
        // Plain EGL: default display.
        let display =
            unsafe { client.GetDisplay(egl::EGL_DEFAULT_DISPLAY as egl::EGLNativeDisplayType) };
        if display != egl::EGL_NO_DISPLAY && report_display_with("default", &client, display, loader)
        {
            passed += 1;
        }
    }

    if passed > 0 {
        println!("egl-info-rs: PASS ({passed} display(s))");
        ExitCode::SUCCESS
    } else {
        eprintln!("egl-info-rs: no display initialized");
        ExitCode::from(1)
    }
}

fn report_display_with(
    label: &str,
    ctx: &egl::Egl,
    display: egl::EGLDisplay,
    loader: &Loader,
) -> bool {
    let (mut major, mut minor): (egl::EGLint, egl::EGLint) = (0, 0);
    if unsafe { ctx.Initialize(display, &mut major, &mut minor) }
        != egl::EGL_TRUE as egl::EGLBoolean
    {
        println!("  {label:<12} initialize failed");
        return false;
    }
    let ok = match unsafe { egl::Egl::load_egl(display, |n| loader.load(n)) } {
        Ok(dctx) => {
            let vendor = unsafe { dctx.QueryString(display, egl::EGL_VENDOR as egl::EGLint) };
            let vendor = if vendor.is_null() {
                "?".into()
            } else {
                unsafe { CStr::from_ptr(vendor) }
                    .to_string_lossy()
                    .into_owned()
            };
            println!(
                "  {label:<12} EGL {}.{}  vendor: {vendor}",
                dctx.version() >> 8,
                dctx.version() & 0xff
            );
            true
        }
        Err(e) => {
            println!("  {label:<12} load_egl failed: {e}");
            false
        }
    };
    unsafe { ctx.Terminate(display) };
    ok
}
