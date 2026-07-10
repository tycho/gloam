//! Integration tests for WGL, GLX, and other API-specific edge cases.
//!
//! These tests require that the bundled XML files are populated.

mod common;
use common::{assert_c_output_exists, generate, has_ext, read_header};

// ---------------------------------------------------------------------------
// WGL
// ---------------------------------------------------------------------------

#[test]
fn wgl_generates_expected_files() {
    let dir = generate(&["--api", "wgl"], &[]);
    assert_c_output_exists(dir.path(), "wgl");
}

#[test]
fn wgl_mandatory_extensions_always_present() {
    // WGL_ARB_extensions_string and WGL_EXT_extensions_string are mandatory
    // (spec gotcha #9): they must appear even with an empty extension filter.
    let dir = generate(&["--api", "wgl", "--extensions", ""], &[]);
    let header = read_header(dir.path(), "wgl");

    assert!(
        has_ext(&header, "ARB_extensions_string"),
        "WGL_ARB_extensions_string should always be present (mandatory)"
    );
    assert!(
        has_ext(&header, "EXT_extensions_string"),
        "WGL_EXT_extensions_string should always be present (mandatory)"
    );
}

#[test]
fn wgl_with_explicit_extension_includes_mandatory_too() {
    // Even when requesting a specific extension, the mandatory pair must
    // still appear.
    let dir = generate(
        &["--api", "wgl", "--extensions", "WGL_ARB_pixel_format"],
        &[],
    );
    let header = read_header(dir.path(), "wgl");

    assert!(
        has_ext(&header, "ARB_pixel_format"),
        "WGL_ARB_pixel_format should be present (explicitly requested)"
    );
    assert!(
        has_ext(&header, "ARB_extensions_string"),
        "WGL_ARB_extensions_string should always be present (mandatory)"
    );
    assert!(
        has_ext(&header, "EXT_extensions_string"),
        "WGL_EXT_extensions_string should always be present (mandatory)"
    );
}

// ---------------------------------------------------------------------------
// GLX
// ---------------------------------------------------------------------------

#[test]
fn glx_generates_expected_files() {
    let dir = generate(&["--api", "glx"], &[]);
    assert_c_output_exists(dir.path(), "glx");
}

#[test]
fn glx_with_empty_extensions_generates() {
    let dir = generate(&["--api", "glx", "--extensions", ""], &[]);
    assert_c_output_exists(dir.path(), "glx");

    let header = read_header(dir.path(), "glx");
    assert!(
        header.contains("GloamGLXContext"),
        "missing GLX context struct"
    );
}
