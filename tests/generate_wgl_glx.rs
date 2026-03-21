//! Integration tests for WGL, GLX, and other API-specific edge cases.
//!
//! These tests require that the bundled XML files are populated.

use tempfile::TempDir;

fn gloam() -> assert_cmd::Command {
    assert_cmd::Command::cargo_bin("gloam").expect("gloam binary not found")
}

fn read_header(out: &std::path::Path, stem: &str) -> String {
    std::fs::read_to_string(out.join("include").join("gloam").join(format!("{stem}.h")))
        .unwrap_or_else(|_| panic!("missing include/gloam/{stem}.h"))
}

fn has_ext(header: &str, short_name: &str) -> bool {
    header.contains(&format!("unsigned char {short_name};"))
}

fn assert_c_output_exists(out: &std::path::Path, stem: &str) {
    assert!(
        out.join("include")
            .join("gloam")
            .join(format!("{stem}.h"))
            .exists(),
        "missing include/gloam/{stem}.h"
    );
    assert!(
        out.join("src").join(format!("{stem}.c")).exists(),
        "missing src/{stem}.c"
    );
}

// ---------------------------------------------------------------------------
// WGL
// ---------------------------------------------------------------------------

#[test]
fn wgl_generates_expected_files() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "wgl",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    assert_c_output_exists(dir.path(), "wgl");
}

#[test]
fn wgl_mandatory_extensions_always_present() {
    // WGL_ARB_extensions_string and WGL_EXT_extensions_string are mandatory
    // (spec gotcha #9): they must appear even with an empty extension filter.
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "wgl",
            "--extensions",
            "",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

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
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "wgl",
            "--extensions",
            "WGL_ARB_pixel_format",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

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
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "glx",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    assert_c_output_exists(dir.path(), "glx");
}

#[test]
fn glx_with_empty_extensions_generates() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "glx",
            "--extensions",
            "",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    assert_c_output_exists(dir.path(), "glx");

    let header = read_header(dir.path(), "glx");
    assert!(
        header.contains("GloamGLXContext"),
        "missing GLX context struct"
    );
}
