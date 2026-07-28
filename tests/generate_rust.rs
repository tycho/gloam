//! Integration tests for the Rust backend: invoke the gloam binary, verify
//! the emitted crate's structure, and `cargo check` it (both feature
//! configurations) when cargo is available.

mod common;
use common::{generate_rust, gloam, read_rust_lib, read_rust_manifest, try_cargo_check_rust};

// ---------------------------------------------------------------------------
// Structure
// ---------------------------------------------------------------------------

#[test]
fn gl_core_33_rust_generates_crate() {
    let dir = generate_rust(&["--api", "gl:core=3.3"], &[]);
    let lib = read_rust_lib(dir.path());
    let manifest = read_rust_manifest(dir.path());

    assert!(manifest.contains("name = \"gloam_gl\""));
    assert!(manifest.contains("no-error = []"));
    assert!(lib.contains("#![no_std]"));
    assert!(lib.contains("#![deny(unsafe_op_in_unsafe_fn)]"));
    assert!(lib.contains("pub struct Gl {"));
    assert!(lib.contains("pub unsafe fn load_gl("));
    assert!(lib.contains("pub enum LoadError {"));
    // No --alias, no --mx-global: neither layer should be emitted.
    assert!(!lib.contains("ALIAS_PAIRS"));
    assert!(!lib.contains("pub unsafe fn init_global"));
}

#[test]
fn merged_rust_emits_both_apis_alias_and_global_layers() {
    let dir = generate_rust(
        &["--api", "gl:core=3.3,gles2=3.0", "--merge"],
        &["--alias", "--mx-global"],
    );
    let lib = read_rust_lib(dir.path());

    assert!(lib.contains("pub unsafe fn load_gl("));
    assert!(lib.contains("pub unsafe fn load_gles2("));
    assert!(lib.contains("static EXT_RANGES_gl:"));
    assert!(lib.contains("static EXT_RANGES_gles2:"));
    assert!(lib.contains("static ALIAS_PAIRS:"));
    assert!(lib.contains("pub unsafe fn init_global"));
    assert!(lib.contains("pub unsafe fn load_gl_global"));
    assert!(lib.contains("pub unsafe fn load_gles2_global"));
}

#[test]
fn rust_constants_are_newtyped() {
    let dir = generate_rust(&["--api", "gl:core=3.3", "--extensions", ""], &[]);
    let lib = read_rust_lib(dir.path());

    // Bitmask-block constants take the GLbitfield newtype; special numbers
    // take their name-based types; everything else is GLenum.
    assert!(lib.contains("pub const GL_COLOR_BUFFER_BIT: GLbitfield = GLbitfield(0x00004000);"));
    assert!(lib.contains("pub const GL_TRIANGLES: GLenum = GLenum(0x0004);"));
    assert!(lib.contains("pub const GL_TRUE: GLboolean = 1;"));
    assert!(lib.contains("pub const GL_FALSE: GLboolean = 0;"));
    assert!(lib.contains("pub const GL_TIMEOUT_IGNORED: GLuint64 = 0xFFFFFFFFFFFFFFFF;"));
    assert!(lib.contains("pub const GL_INVALID_INDEX: GLuint = 0xFFFFFFFF;"));
}

#[test]
fn rust_backend_rejects_unsupported_specs() {
    // WGL/GLX have no Rust backend yet (GL/GLES, Vulkan, and EGL do).
    let dir = tempfile::TempDir::new().unwrap();
    gloam()
        .args(["--api", "wgl"])
        .args(["--out-path", dir.path().to_str().unwrap(), "rust"])
        .assert()
        .failure();
}

#[test]
fn rust_backend_rejects_multi_spec_requests() {
    // Multiple resolved loaders would clobber one crate's files.
    let dir = tempfile::TempDir::new().unwrap();
    gloam()
        .args(["--api", "gl:core=3.3,egl", "--merge"])
        .args(["--out-path", dir.path().to_str().unwrap(), "rust"])
        .assert()
        .failure();
}

// ---------------------------------------------------------------------------
// Compile smoke tests
// ---------------------------------------------------------------------------

#[test]
fn rust_crate_passes_cargo_check() {
    let dir = generate_rust(
        &["--api", "gl:core=3.3,gles2=3.0", "--merge"],
        &["--alias", "--mx-global"],
    );
    try_cargo_check_rust(dir.path(), &[]);
    try_cargo_check_rust(dir.path(), &["no-error"]);
}
