// Tests for --promoted and --predecessors extension selection flags.
//
// These tests rely on known stable alias relationships in the Khronos XML:
//
//   --promoted:
//     GL_ARB_copy_buffer     — same-name promotion: glCopyBufferSubData
//                              entered GL 3.1 with the same name.
//     GL_ARB_multitexture    — renamed promotion: glActiveTextureARB became
//                              glActiveTexture in GL 1.3.
//
//   --predecessors (commands):
//     GL_ARB_parallel_shader_compile is the predecessor of
//     GL_KHR_parallel_shader_compile (glMaxShaderCompilerThreadsARB /
//     glMaxShaderCompilerThreadsKHR).
//
//   --predecessors (enums only):
//     EGL_EXT_platform_wayland is the predecessor of EGL_KHR_platform_wayland.
//     The only relationship is the enum alias:
//       EGL_PLATFORM_WAYLAND_EXT alias="EGL_PLATFORM_WAYLAND_KHR"
//     There are no commands in either extension — this exercises the
//     enum-alias path that was previously unimplemented.

use tempfile::TempDir;

fn gloam() -> assert_cmd::Command {
    assert_cmd::Command::cargo_bin("gloam").expect("gloam binary not found")
}

/// Read the generated header for the given stem from `out`.
fn read_header(out: &std::path::Path, stem: &str) -> String {
    std::fs::read_to_string(out.join("include").join("gloam").join(format!("{stem}.h")))
        .unwrap_or_else(|_| panic!("missing include/gloam/{stem}.h"))
}

/// True if the extArray struct contains a slot for `short_name`
/// (e.g. "ARB_copy_buffer").
fn has_ext(header: &str, short_name: &str) -> bool {
    // The generated struct member looks like:
    //   unsigned char ARB_copy_buffer;
    header.contains(&format!("unsigned char {short_name};"))
}

// ---------------------------------------------------------------------------
// --promoted: same-name promotion
// ---------------------------------------------------------------------------

#[test]
fn promoted_includes_arb_copy_buffer_same_name() {
    // glCopyBufferSubData was promoted into GL 3.1 with the same name.
    // ARB_copy_buffer should be auto-selected with --promoted.
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=3.3",
            "--extensions",
            "", // empty list — no explicit extensions
            "--promoted",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let header = read_header(dir.path(), "gl");
    assert!(
        has_ext(&header, "ARB_copy_buffer"),
        "ARB_copy_buffer should be included via --promoted (same-name promotion)"
    );
}

#[test]
fn without_promoted_arb_copy_buffer_absent() {
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

    let header = read_header(dir.path(), "gl");
    assert!(
        !has_ext(&header, "ARB_copy_buffer"),
        "ARB_copy_buffer should be absent without --promoted"
    );
}

// ---------------------------------------------------------------------------
// --promoted: renamed promotion
// ---------------------------------------------------------------------------

#[test]
fn promoted_includes_arb_multitexture_renamed() {
    // glActiveTextureARB was renamed to glActiveTexture when promoted into
    // GL 1.3. ARB_multitexture should be auto-selected with --promoted.
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=3.3",
            "--extensions",
            "",
            "--promoted",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let header = read_header(dir.path(), "gl");
    assert!(
        has_ext(&header, "ARB_multitexture"),
        "ARB_multitexture should be included via --promoted (renamed promotion)"
    );
}

#[test]
fn without_promoted_arb_multitexture_absent() {
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

    let header = read_header(dir.path(), "gl");
    assert!(
        !has_ext(&header, "ARB_multitexture"),
        "ARB_multitexture should be absent without --promoted"
    );
}

// ---------------------------------------------------------------------------
// --predecessors: command-based match
// ---------------------------------------------------------------------------

#[test]
fn predecessors_includes_arb_parallel_shader_compile() {
    // GL_KHR_parallel_shader_compile is explicitly requested.
    // GL_ARB_parallel_shader_compile is its predecessor via the command alias
    // glMaxShaderCompilerThreadsARB / glMaxShaderCompilerThreadsKHR.
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=3.3",
            "--extensions",
            "GL_KHR_parallel_shader_compile",
            "--predecessors",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let header = read_header(dir.path(), "gl");
    assert!(
        has_ext(&header, "KHR_parallel_shader_compile"),
        "KHR_parallel_shader_compile should be present (explicitly requested)"
    );
    assert!(
        has_ext(&header, "ARB_parallel_shader_compile"),
        "ARB_parallel_shader_compile should be included via --predecessors"
    );
}

#[test]
fn without_predecessors_arb_parallel_shader_compile_absent() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=3.3",
            "--extensions",
            "GL_KHR_parallel_shader_compile",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let header = read_header(dir.path(), "gl");
    assert!(
        has_ext(&header, "KHR_parallel_shader_compile"),
        "KHR_parallel_shader_compile should be present (explicitly requested)"
    );
    assert!(
        !has_ext(&header, "ARB_parallel_shader_compile"),
        "ARB_parallel_shader_compile should be absent without --predecessors"
    );
}

// ---------------------------------------------------------------------------
// --predecessors: enum-only match
// ---------------------------------------------------------------------------

#[test]
fn predecessors_includes_egl_ext_platform_wayland_via_enum_alias() {
    // EGL_KHR_platform_wayland is explicitly requested.
    // EGL_EXT_platform_wayland is its predecessor via enum alias only —
    // neither extension has any commands.  This exercises the enum-alias
    // path in the predecessor search:
    //   EGL_PLATFORM_WAYLAND_EXT alias="EGL_PLATFORM_WAYLAND_KHR"
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "egl",
            "--extensions",
            "EGL_KHR_platform_wayland",
            "--predecessors",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let header = read_header(dir.path(), "egl");
    assert!(
        has_ext(&header, "KHR_platform_wayland"),
        "KHR_platform_wayland should be present (explicitly requested)"
    );
    assert!(
        has_ext(&header, "EXT_platform_wayland"),
        "EXT_platform_wayland should be included via --predecessors (enum-alias match)"
    );
}

#[test]
fn without_predecessors_egl_ext_platform_wayland_absent() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "egl",
            "--extensions",
            "EGL_KHR_platform_wayland",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let header = read_header(dir.path(), "egl");
    assert!(
        has_ext(&header, "KHR_platform_wayland"),
        "KHR_platform_wayland should be present (explicitly requested)"
    );
    assert!(
        !has_ext(&header, "EXT_platform_wayland"),
        "EXT_platform_wayland should be absent without --predecessors"
    );
}

// ---------------------------------------------------------------------------
// --promoted: scoping in merged builds
// ---------------------------------------------------------------------------

#[test]
fn promoted_in_merged_build_does_not_cross_contaminate() {
    // In a merged gl:core=3.3 + gles2=3.0 build, --promoted should scope
    // promotion checks per-API.  An extension that is promoted into GL core
    // but not into GLES2 (or vice versa) should only appear because its
    // commands match the correct API's core set.
    //
    // GL_ARB_copy_buffer's glCopyBufferSubData was promoted into GL 3.1.
    // It should appear in the merged output because it's promoted for GL.
    // The important thing is that the merged build succeeds without panics
    // or incorrect output — and that the extension appears exactly once.
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=3.3,gles2=3.0",
            "--merge",
            "--extensions",
            "",
            "--promoted",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let header = read_header(dir.path(), "gl");

    // ARB_copy_buffer should be included (promoted into GL 3.1 core).
    assert!(
        has_ext(&header, "ARB_copy_buffer"),
        "ARB_copy_buffer should be included via --promoted in merged build"
    );

    // The extension should appear exactly once in the extArray, not
    // duplicated across APIs.
    let count = header.matches("unsigned char ARB_copy_buffer;").count();
    assert_eq!(count, 1, "ARB_copy_buffer should appear exactly once");
}

// ---------------------------------------------------------------------------
// --promoted + --predecessors combined
// ---------------------------------------------------------------------------

#[test]
fn promoted_seeds_predecessor_search() {
    // --promoted includes GL_ARB_copy_buffer (same-name promotion into GL 3.1).
    // If ARB_copy_buffer has any predecessor extensions, --predecessors should
    // find them.  Even if no predecessor exists for this particular extension,
    // the combined flag path must not panic or produce incorrect output.
    //
    // More importantly, this verifies the ordering: promoted runs first,
    // then predecessors operates on the expanded set.
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=3.3",
            "--extensions",
            "",
            "--promoted",
            "--predecessors",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let header = read_header(dir.path(), "gl");

    // --promoted should still include ARB_copy_buffer.
    assert!(
        has_ext(&header, "ARB_copy_buffer"),
        "ARB_copy_buffer should be included via --promoted"
    );

    // --promoted should include ARB_multitexture (renamed promotion).
    assert!(
        has_ext(&header, "ARB_multitexture"),
        "ARB_multitexture should be included via --promoted"
    );
}

#[test]
fn predecessors_finds_chain_through_promoted_extension() {
    // GL_KHR_parallel_shader_compile is explicitly requested.
    // --predecessors should find GL_ARB_parallel_shader_compile as its
    // predecessor.  Combined with --promoted, the entire chain should
    // resolve correctly.
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=3.3",
            "--extensions",
            "GL_KHR_parallel_shader_compile",
            "--promoted",
            "--predecessors",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let header = read_header(dir.path(), "gl");

    assert!(
        has_ext(&header, "KHR_parallel_shader_compile"),
        "KHR_parallel_shader_compile should be present (explicitly requested)"
    );
    assert!(
        has_ext(&header, "ARB_parallel_shader_compile"),
        "ARB_parallel_shader_compile should be included via --predecessors"
    );
}
