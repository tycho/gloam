//! Integration tests: invoke the gloam binary and verify output.
//!
//! These tests require that the bundled XML files are populated.
//! They also attempt a C compile step if `cc` is available on PATH.

mod common;
use common::{assert_c_output_exists, generate, gloam, read_header, try_compile_c};

use tempfile::TempDir;

// ---------------------------------------------------------------------------
// Core generation tests
// ---------------------------------------------------------------------------

#[test]
fn gl_core_33_c_generates_expected_files() {
    let dir = generate(&["--api", "gl:core=3.3"], &[]);
    assert_c_output_exists(dir.path(), "gl");
    assert!(
        dir.path()
            .join("include")
            .join("KHR")
            .join("khrplatform.h")
            .exists()
    );
    try_compile_c(dir.path());
}

#[test]
fn gl_core_33_with_loader_generates_and_compiles() {
    let dir = generate(&["--api", "gl:core=3.3"], &["--loader"]);
    assert_c_output_exists(dir.path(), "gl");
    try_compile_c(dir.path());
}

#[test]
fn gl_core_33_with_alias_generates_and_compiles() {
    let dir = generate(&["--api", "gl:core=3.3"], &["--alias"]);
    assert_c_output_exists(dir.path(), "gl");
    try_compile_c(dir.path());
}

#[test]
fn gl_core_33_all_flags_generates_and_compiles() {
    let dir = generate(&["--api", "gl:core=3.3"], &["--alias", "--loader"]);
    assert_c_output_exists(dir.path(), "gl");
    try_compile_c(dir.path());
}

#[test]
fn gles2_generates_and_compiles() {
    let dir = generate(&["--api", "gles2=3.0"], &[]);
    assert_c_output_exists(dir.path(), "gles2");
    try_compile_c(dir.path());
}

#[test]
fn gl_compatibility_profile_generates() {
    let dir = generate(&["--api", "gl:compatibility=3.3"], &[]);
    assert_c_output_exists(dir.path(), "gl");
}

#[test]
fn gl_latest_version_generates() {
    // No version specified — should use latest available.
    let dir = generate(&["--api", "gl:core"], &[]);
    assert_c_output_exists(dir.path(), "gl");
}

#[test]
fn egl_generates_and_compiles() {
    let dir = generate(&["--api", "egl"], &[]);
    assert_c_output_exists(dir.path(), "egl");
    try_compile_c(dir.path());
}

#[test]
fn merged_gl_gles2_generates_single_output() {
    let dir = generate(&["--api", "gl:core=3.3,gles2=3.0", "--merge"], &[]);
    // Merged output: both APIs in a single "gl" stem file.
    assert_c_output_exists(dir.path(), "gl");
    try_compile_c(dir.path());
}

// ---------------------------------------------------------------------------
// Error / invalid input tests
// ---------------------------------------------------------------------------

#[test]
fn missing_api_arg_fails() {
    gloam().args(["c"]).assert().failure();
}

#[test]
fn invalid_version_format_fails() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=3",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .failure();
}

#[test]
fn gl_without_profile_is_rejected() {
    // "gl" without a profile is ambiguous — XML requires/removes are
    // profile-conditional, so no profile silently produced a core/compat
    // hybrid (compat enums present, compat commands removed).  It is now a
    // parse-time error with a clear message.
    let dir = TempDir::new().unwrap();
    let output = gloam()
        .args([
            "--api",
            "gl=3.3",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success(), "gl without profile must fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("requires a profile"),
        "error should explain the fix, got: {stderr}"
    );
}

// ---------------------------------------------------------------------------
// Content smoke tests
// ---------------------------------------------------------------------------

#[test]
fn generated_header_contains_feature_macro() {
    let dir = generate(&["--api", "gl:core=3.3"], &[]);
    let header = read_header(dir.path(), "gl");

    assert!(
        header.contains("GLOAM_GL_VERSION_3_3"),
        "missing version macro"
    );
    assert!(header.contains("GloamGLContext"), "missing context struct");
    assert!(
        header.contains("gloamLoadGL"),
        "missing load function declaration"
    );
}

#[test]
fn generated_header_does_not_contain_removed_compat_enums_in_core() {
    // In core profile, deprecated constants like GL_QUADS should not appear.
    // This is a regression guard for feature-set resolution.
    let dir = generate(&["--api", "gl:core=3.3", "--extensions", ""], &[]);
    let header = read_header(dir.path(), "gl");

    // GL_QUADS is removed in core profile — it must not appear.
    assert!(
        !header.contains("GL_QUADS "),
        "GL_QUADS should be absent in core profile"
    );
}

#[test]
fn removed_enums_readded_by_extensions() {
    // In core profile, deprecated constants like GL_QUADS get removed, *except* when they're
    // required by extensions.
    let dir = generate(
        &[
            "--api",
            "gl:core=3.3",
            "--extensions",
            "GL_ARB_tessellation_shader",
        ],
        &[],
    );
    let header = read_header(dir.path(), "gl");

    assert!(
        header.contains("GL_QUADS "),
        "GL_QUADS should be in core profile with GL_ARB_tessellation_shader"
    );
}

#[test]
fn compatibility_profile_has_legacy_gl() {
    let dir = generate(&["--api", "gl:compatibility=3.3", "--extensions", ""], &[]);
    let header = read_header(dir.path(), "gl");

    // Ensure that the compatibility profile has the legacy OpenGL functionality
    assert!(header.contains(" GL_QUADS "), "GL_QUADS should be defined");
    assert!(
        header.contains(" glVertex3f "),
        "glVertex3f should be defined"
    );
    assert!(
        header.contains(" glNormal3f "),
        "glNormal3f should be defined"
    );
}

#[test]
fn generated_has_support_macros() {
    let dir = generate(
        &[
            "--api",
            "gl:core=3.3",
            "--extensions",
            "GL_ARB_tessellation_shader",
        ],
        &[],
    );
    let header = read_header(dir.path(), "gl");

    // We targeted OpenGL 3.3, we should have the macro defined to 1 indicating
    // support for that and prior versions
    assert!(
        header.contains(" GL_VERSION_1_0 1"),
        "GL_VERSION_1_0 should be defined"
    );
    assert!(
        header.contains(" GL_VERSION_3_3 1"),
        "GL_VERSION_3_3 should be defined"
    );

    // We should also have a macro for GL_ARB_tesselation_shader
    assert!(
        header.contains(" GL_ARB_tessellation_shader 1"),
        "GL_ARB_tessellation_shader should be defined"
    );
}
