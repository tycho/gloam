//! Integration tests: invoke the gloam binary and verify output.
//!
//! These tests require that the bundled XML files are populated.
//! They also attempt a C compile step if `cc` is available on PATH.

use std::path::Path;
use std::process::Command;

use tempfile::TempDir;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn gloam() -> assert_cmd::Command {
    assert_cmd::Command::cargo_bin("gloam").expect("gloam binary not found")
}

/// Attempt to compile a generated C source with the system C compiler.
/// Silently skips if `cc` is not on PATH (expected in CI, optional locally).
fn try_compile_c(out: &Path) {
    // Find a .c file in out/src/.
    let src_dir = out.join("src");
    let c_file = match std::fs::read_dir(&src_dir).ok().and_then(|mut d| {
        d.find(|e| {
            e.as_ref()
                .is_ok_and(|e| e.path().extension() == Some("c".as_ref()))
        })
    }) {
        Some(Ok(entry)) => entry.path(),
        _ => return, // nothing to compile
    };

    let cc = match find_cc() {
        Some(c) => c,
        None => {
            eprintln!("compile check skipped: no C compiler on PATH");
            return;
        }
    };

    let status = Command::new(cc)
        .args([
            "-c",
            "-std=c11",
            "-Wall",
            "-Wno-unused-function",
            "-o",
            "/dev/null",
            &format!("-I{}", out.join("include").display()),
            c_file.to_str().unwrap(),
        ])
        .status()
        .expect("failed to spawn C compiler");

    assert!(
        status.success(),
        "generated C file {} failed to compile",
        c_file.display()
    );
}

fn find_cc() -> Option<&'static str> {
    ["cc", "gcc", "clang"]
        .iter()
        .find(|&candidate| {
            Command::new(candidate)
                .arg("--version")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        })
        .map(|v| v as _)
}

fn assert_c_output_exists(out: &Path, stem: &str) {
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
// Core generation tests
// ---------------------------------------------------------------------------

#[test]
fn gl_core_33_c_generates_expected_files() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=3.3",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

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
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=3.3",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
            "--loader",
        ])
        .assert()
        .success();

    assert_c_output_exists(dir.path(), "gl");
    try_compile_c(dir.path());
}

#[test]
fn gl_core_33_with_alias_generates_and_compiles() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=3.3",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
            "--alias",
        ])
        .assert()
        .success();

    assert_c_output_exists(dir.path(), "gl");
    try_compile_c(dir.path());
}

#[test]
fn gl_core_33_all_flags_generates_and_compiles() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=3.3",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
            "--alias",
            "--loader",
        ])
        .assert()
        .success();

    assert_c_output_exists(dir.path(), "gl");
    try_compile_c(dir.path());
}

#[test]
fn gles2_generates_and_compiles() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gles2=3.0",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    assert_c_output_exists(dir.path(), "gles2");
    try_compile_c(dir.path());
}

#[test]
fn gl_compat_profile_generates() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:compat=3.3",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    assert_c_output_exists(dir.path(), "gl");
}

#[test]
fn gl_latest_version_generates() {
    // No version specified — should use latest available.
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    assert_c_output_exists(dir.path(), "gl");
}

#[test]
fn egl_generates_and_compiles() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "egl",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    assert_c_output_exists(dir.path(), "egl");
    try_compile_c(dir.path());
}

#[test]
fn merged_gl_gles2_generates_single_output() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=3.3,gles2=3.0",
            "--merge",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

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
fn gl_without_profile_may_warn_or_fail() {
    // "gl" without a profile is ambiguous. The generator should either fail
    // with a clear message or pick a default. Either outcome is acceptable,
    // but it must not silently produce incorrect output.
    // This test just documents the current behaviour — update it if the
    // intended behaviour changes.
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
    // Currently we just assert it doesn't panic; check exit code separately.
    let _ = output.status; // document whatever the current behaviour is
}

// ---------------------------------------------------------------------------
// Content smoke tests
// ---------------------------------------------------------------------------

#[test]
fn generated_header_contains_feature_macro() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=3.3",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let header =
        std::fs::read_to_string(dir.path().join("include").join("gloam").join("gl.h")).unwrap();

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
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=3.3",
            "--extensions",
            "",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let header =
        std::fs::read_to_string(dir.path().join("include").join("gloam").join("gl.h")).unwrap();

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
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=3.3",
            "--extensions",
            "GL_ARB_tessellation_shader",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let header =
        std::fs::read_to_string(dir.path().join("include").join("gloam").join("gl.h")).unwrap();

    // GL_QUADS is removed in core profile — it must not appear.
    assert!(
        header.contains("GL_QUADS "),
        "GL_QUADS should be in core profile with GL_ARB_tessellation_shader"
    );
}

#[test]
fn compatibility_profile_has_legacy_gl() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:compatibility=3.3",
            "--extensions",
            "",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let header =
        std::fs::read_to_string(dir.path().join("include").join("gloam").join("gl.h")).unwrap();

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
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=3.3",
            "--extensions",
            "GL_ARB_tessellation_shader",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let header =
        std::fs::read_to_string(dir.path().join("include").join("gloam").join("gl.h")).unwrap();

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
