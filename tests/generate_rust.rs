//! Integration tests for the Rust backend: invoke the gloam binary, verify
//! the emitted crate's structure, and `cargo check` it (both feature
//! configurations) when cargo is available.

mod common;
use common::{generate_rust, read_rust_lib, read_rust_manifest, try_cargo_check_rust};

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
fn rust_multi_spec_emits_one_crate_with_per_spec_modules() {
    let dir = generate_rust(
        &["--api", "gl:core=3.3,egl", "--merge", "--extensions", ""],
        &[],
    );
    let manifest = read_rust_manifest(dir.path());
    assert!(manifest.contains("name = \"gloam_gl_egl\""));

    let lib = std::fs::read_to_string(dir.path().join("src/lib.rs")).unwrap();
    assert!(lib.contains("pub mod gl;"));
    assert!(lib.contains("pub mod egl;"));
    // Loader types collide across modules (each has its own LoadError), so
    // multi-spec crates do not re-export at the root.
    assert!(!lib.contains("pub use self::"));

    assert!(dir.path().join("src/gl.rs").exists());
    assert!(dir.path().join("src/egl.rs").exists());
}

#[test]
fn rust_single_spec_reexports_module_at_root() {
    let dir = generate_rust(&["--api", "gl:core=3.3", "--extensions", ""], &[]);
    let lib = std::fs::read_to_string(dir.path().join("src/lib.rs")).unwrap();
    assert!(lib.contains("pub mod gl;"));
    assert!(lib.contains("pub use self::gl::*;"));
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

// ---------------------------------------------------------------------------
// Vulkan probe API and scope-exact detection
// ---------------------------------------------------------------------------

#[test]
fn rust_vulkan_emits_probe_api_and_scope_table() {
    let dir = generate_rust(
        &["--api", "vk=1.0", "--extensions", "VK_KHR_swapchain"],
        &["--mx-global"],
    );
    let lib = read_rust_lib(dir.path());

    // Scope table partitions detection; flags are assigned, never OR-merged.
    assert!(lib.contains("static EXT_SCOPES: [u8; EXT_COUNT]"));
    assert!(lib.contains("EXT_SCOPE_INSTANCE"));
    assert!(lib.contains("EXT_SCOPE_DEVICE"));

    // Probe snapshot type and its constructors/queries.
    assert!(lib.contains("pub struct VkExtensions"));
    assert!(lib.contains("pub fn from_properties(properties: &[VkExtensionProperties])"));
    assert!(lib.contains("pub unsafe fn query_instance_extensions(&self)"));
    assert!(lib.contains("pub unsafe fn query_device_extensions("));
    assert!(lib.contains("pub unsafe fn load_instance_from_query("));
    assert!(lib.contains("pub unsafe fn load_device_from_query("));

    // --mx-global mirrors.
    assert!(lib.contains("pub unsafe fn query_device_extensions_global("));
    assert!(lib.contains("pub unsafe fn load_device_from_query_global("));

    // Physical-device change invalidates cached device state in discover.
    assert!(lib.contains("self.found_device_exts = false;"));

    try_cargo_check_rust(dir.path(), &[]);
    try_cargo_check_rust(dir.path(), &["alloc"]);
}
