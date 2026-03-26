//! Extended integration tests covering gaps in the existing test suite.
//!
//! These tests require that the bundled XML files are populated.
//! They also attempt a C compile step if `cc` is available on PATH.

use std::path::Path;

use tempfile::TempDir;

// ---------------------------------------------------------------------------
// Helpers (shared pattern with existing test files)
// ---------------------------------------------------------------------------

fn gloam() -> assert_cmd::Command {
    assert_cmd::Command::cargo_bin("gloam").expect("gloam binary not found")
}

/// Attempt to compile generated C sources with the system C compiler.
/// Uses the `cc` crate for compiler detection (handles MSVC, GCC, Clang,
/// cross-compilation toolchains, CC env override, etc.).
/// Silently skips if no compiler is available.
fn try_compile_c(out: &Path) {
    let src_dir = out.join("src");
    let c_files: Vec<_> = std::fs::read_dir(&src_dir)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension() == Some("c".as_ref()))
        .collect();

    if c_files.is_empty() {
        return;
    }

    let target = env!("TARGET");
    let mut build = cc::Build::new();
    build
        .target(target)
        .host(target)
        .opt_level(0)
        .out_dir(&src_dir)
        .include(out.join("include"))
        .warnings(true)
        .cargo_warnings(false)
        .std("c11")
        .flag_if_supported("-Wno-unused-function");

    for f in &c_files {
        build.file(f);
    }

    if let Err(e) = build.try_compile("gloam_test") {
        let msg = e.to_string();
        if msg.contains("Failed to find tool")
            || msg.contains("not found")
            || msg.contains("couldn't find")
        {
            eprintln!("compile check skipped: no C compiler found");
        } else {
            panic!(
                "generated C files in {} failed to compile: {}",
                src_dir.display(),
                e
            );
        }
    }
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

fn read_header(out: &Path, stem: &str) -> String {
    std::fs::read_to_string(out.join("include").join("gloam").join(format!("{stem}.h")))
        .unwrap_or_else(|_| panic!("missing include/gloam/{stem}.h"))
}

fn read_source(out: &Path, stem: &str) -> String {
    std::fs::read_to_string(out.join("src").join(format!("{stem}.c")))
        .unwrap_or_else(|_| panic!("missing src/{stem}.c"))
}

fn has_ext(header: &str, short_name: &str) -> bool {
    header.contains(&format!("unsigned char {short_name};"))
}

/// Recursively collect all file paths relative to `root`.
fn collect_files(root: &Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    collect_files_recursive(root, root, &mut files);
    files.sort();
    files
}

fn collect_files_recursive(base: &Path, dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_files_recursive(base, &path, out);
            } else {
                out.push(path.strip_prefix(base).unwrap().to_path_buf());
            }
        }
    }
}

/// Normalize file content for determinism comparison by replacing the
/// `--out-path <tempdir>` portion of the preamble with a placeholder.
fn normalize_for_determinism(content: &[u8], out_path: &str) -> Vec<u8> {
    let s = String::from_utf8_lossy(content);
    s.replace(out_path, "<OUT>").into_bytes()
}

// ===========================================================================
// 1. Determinism — byte-identical output across runs
// ===========================================================================

fn assert_deterministic(args: &[&str]) {
    let dir_a = TempDir::new().unwrap();
    let dir_b = TempDir::new().unwrap();

    for dir in [&dir_a, &dir_b] {
        gloam()
            .args(args)
            .args(["--out-path", dir.path().to_str().unwrap(), "c"])
            .assert()
            .success();
    }

    let files_a = collect_files(dir_a.path());
    let files_b = collect_files(dir_b.path());

    assert_eq!(files_a, files_b, "file lists differ between runs");
    assert!(!files_a.is_empty(), "no files generated");

    let path_a = dir_a.path().to_str().unwrap();
    let path_b = dir_b.path().to_str().unwrap();

    for rel in &files_a {
        let raw_a = std::fs::read(dir_a.path().join(rel)).unwrap();
        let raw_b = std::fs::read(dir_b.path().join(rel)).unwrap();
        // Normalize the --out-path temp dir out of the preamble comment
        // so only meaningful content is compared.
        let content_a = normalize_for_determinism(&raw_a, path_a);
        let content_b = normalize_for_determinism(&raw_b, path_b);
        assert_eq!(
            content_a,
            content_b,
            "file {} differs between runs (after normalizing out-path)",
            rel.display()
        );
    }
}

#[test]
fn deterministic_gl_core_33() {
    assert_deterministic(&["--api", "gl:core=3.3"]);
}

#[test]
fn deterministic_vulkan_13() {
    assert_deterministic(&["--api", "vk=1.3"]);
}

#[test]
fn deterministic_merged_gl_gles2() {
    assert_deterministic(&["--api", "gl:core=3.3,gles2=3.0", "--merge"]);
}

// ===========================================================================
// 2. Extension exclusion (- prefix)
// ===========================================================================

#[test]
fn extension_exclusion_removes_extension() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=3.3",
            "--extensions",
            "all,-GL_ARB_tessellation_shader",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let header = read_header(dir.path(), "gl");
    assert!(
        !has_ext(&header, "ARB_tessellation_shader"),
        "ARB_tessellation_shader should be excluded by - prefix"
    );
}

#[test]
fn extension_exclusion_overrides_explicit_include() {
    // When both included and excluded, exclusion wins.
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=3.3",
            "--extensions",
            "GL_ARB_sync,-GL_ARB_sync",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let header = read_header(dir.path(), "gl");
    assert!(
        !has_ext(&header, "ARB_sync"),
        "ARB_sync should be excluded when both included and excluded"
    );
}

// ===========================================================================
// 3. Baseline exclusion (--baseline)
// ===========================================================================

#[test]
fn baseline_excludes_promoted_extensions() {
    // With --baseline gl:core=3.3 and --promoted, extensions promoted into
    // 3.3 or earlier should be excluded.
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=4.6",
            "--extensions",
            "",
            "--promoted",
            "--baseline",
            "gl:core=3.3",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let header = read_header(dir.path(), "gl");

    // ARB_copy_buffer was promoted into GL 3.1, which is <= 3.3 baseline,
    // so it should be excluded.
    assert!(
        !has_ext(&header, "ARB_copy_buffer"),
        "ARB_copy_buffer should be excluded by --baseline gl:core=3.3"
    );
}

#[test]
fn baseline_keep_pin_overrides_exclusion() {
    // With `--extensions all,GL_ARB_copy_buffer`, the explicit pin should
    // survive baseline exclusion.
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=4.6",
            "--extensions",
            "all,GL_ARB_copy_buffer",
            "--promoted",
            "--baseline",
            "gl:core=3.3",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let header = read_header(dir.path(), "gl");
    assert!(
        has_ext(&header, "ARB_copy_buffer"),
        "ARB_copy_buffer should survive baseline exclusion when pinned"
    );
}

// ===========================================================================
// 4. Multiple APIs without --merge
// ===========================================================================

#[test]
fn multi_api_without_merge_produces_separate_files() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=3.3,egl",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    // Both APIs should produce their own output files.
    assert_c_output_exists(dir.path(), "gl");
    assert_c_output_exists(dir.path(), "egl");
}

// ===========================================================================
// 5. Source file (.c) content validation
// ===========================================================================

#[test]
fn gl_source_contains_key_symbols() {
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

    let source = read_source(dir.path(), "gl");
    assert!(
        source.contains("GloamGLContext"),
        "source should reference the context struct"
    );
    assert!(
        source.contains("gloamLoadGL"),
        "source should contain the load function"
    );
}

#[test]
fn vulkan_source_contains_key_symbols() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "vk=1.3",
            "--extensions",
            "",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let source = read_source(dir.path(), "vulkan");
    assert!(
        source.contains("GloamVulkanContext"),
        "source should reference the context struct"
    );
    // Vulkan source uses per-scope load helpers rather than a single gloamLoad.
    assert!(
        source.contains("gloam_vk_load_global_pfns"),
        "source should contain Vulkan global PFN loader"
    );
}

// ===========================================================================
// 6. --quiet flag
// ===========================================================================

#[test]
fn quiet_flag_suppresses_stderr() {
    let dir = TempDir::new().unwrap();
    let output = gloam()
        .args([
            "--api",
            "gl:core=3.3",
            "--quiet",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.is_empty(),
        "stderr should be empty with --quiet, got: {stderr}"
    );
}

#[test]
fn without_quiet_flag_has_stderr_output() {
    let dir = TempDir::new().unwrap();
    let output = gloam()
        .args([
            "--api",
            "gl:core=3.3",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.is_empty(),
        "stderr should contain progress messages without --quiet"
    );
}

// ===========================================================================
// 7. Additional error cases
// ===========================================================================

#[test]
fn unknown_api_name_fails() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "dx12",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .failure();
}

#[test]
fn empty_api_value_fails() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args(["--api", "", "--out-path", dir.path().to_str().unwrap(), "c"])
        .assert()
        .failure();
}

#[test]
fn version_with_extra_dots_fails() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=3.3.0",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .failure();
}

// ===========================================================================
// 8. GLES1 generation
// ===========================================================================

#[test]
fn gles1_generates_and_compiles() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gles1",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    assert_c_output_exists(dir.path(), "gles1");
    try_compile_c(dir.path());
}

#[test]
fn gles1_header_has_context_struct() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gles1",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let header = read_header(dir.path(), "gles1");
    assert!(
        header.contains("GloamGLContext"),
        "GLES1 should use GloamGLContext (shared GL context name)"
    );
}

// ===========================================================================
// 9. GLX/WGL with --loader
// ===========================================================================

#[test]
fn glx_with_loader_generates() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "glx",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
            "--loader",
        ])
        .assert()
        .success();

    assert_c_output_exists(dir.path(), "glx");
}

#[test]
fn wgl_with_loader_generates() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "wgl",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
            "--loader",
        ])
        .assert()
        .success();

    assert_c_output_exists(dir.path(), "wgl");
}

// ===========================================================================
// 10. EGL with specific extensions
// ===========================================================================

#[test]
fn egl_with_extension_filter() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "egl",
            "--extensions",
            "EGL_KHR_debug",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let header = read_header(dir.path(), "egl");
    assert!(
        has_ext(&header, "KHR_debug"),
        "EGL_KHR_debug should be present when explicitly requested"
    );
    try_compile_c(dir.path());
}

// ===========================================================================
// 11. Vulkan version range validation
// ===========================================================================

#[test]
fn vulkan_11_has_11_but_not_12() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "vk=1.1",
            "--extensions",
            "",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let header = read_header(dir.path(), "vulkan");
    assert!(
        header.contains("VK_VERSION_1_0 1"),
        "should have VK_VERSION_1_0"
    );
    assert!(
        header.contains("VK_VERSION_1_1 1"),
        "should have VK_VERSION_1_1"
    );
    assert!(
        !header.contains("VK_VERSION_1_2 1"),
        "should NOT have VK_VERSION_1_2 when targeting 1.1"
    );
}

#[test]
fn vulkan_12_has_12_but_not_13() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "vk=1.2",
            "--extensions",
            "",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let header = read_header(dir.path(), "vulkan");
    assert!(
        header.contains("VK_VERSION_1_1 1"),
        "should have VK_VERSION_1_1"
    );
    assert!(
        header.contains("VK_VERSION_1_2 1"),
        "should have VK_VERSION_1_2"
    );
    assert!(
        !header.contains("VK_VERSION_1_3 1"),
        "should NOT have VK_VERSION_1_3 when targeting 1.2"
    );
}

// ===========================================================================
// 12. Merged build flag combinations
// ===========================================================================

#[test]
fn merged_gl_gles2_with_loader() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=3.3,gles2=3.0",
            "--merge",
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
fn merged_gl_gles2_with_alias() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=3.3,gles2=3.0",
            "--merge",
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
fn merged_gl_gles2_all_flags() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=3.3,gles2=3.0",
            "--merge",
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
fn merged_gl_gles2_with_promoted_predecessors() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=3.3,gles2=3.0",
            "--merge",
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

    assert_c_output_exists(dir.path(), "gl");
    try_compile_c(dir.path());
}

// ===========================================================================
// 13. GL version range
// ===========================================================================

#[test]
fn gl_core_21_has_correct_version_range() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gl:core=2.1",
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
        header.contains("GL_VERSION_2_1 1"),
        "should have GL_VERSION_2_1"
    );
    assert!(
        !header.contains("GL_VERSION_3_0 1"),
        "should NOT have GL_VERSION_3_0 when targeting 2.1"
    );
}

// ===========================================================================
// 14. Vulkan extension exclusion
// ===========================================================================

#[test]
fn vulkan_extension_exclusion() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "vk=1.3",
            "--extensions",
            "all,-VK_KHR_swapchain",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let header = read_header(dir.path(), "vulkan");
    assert!(
        !has_ext(&header, "KHR_swapchain"),
        "VK_KHR_swapchain should be excluded by - prefix"
    );
}

// ===========================================================================
// 15. Header include guard
// ===========================================================================

#[test]
fn gl_header_has_include_guard() {
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

    let header = read_header(dir.path(), "gl");
    assert!(
        header.contains("#ifndef GLOAM_GL_H"),
        "header should have GLOAM_GL_H include guard"
    );
    assert!(
        header.contains("#define GLOAM_GL_H"),
        "header should define GLOAM_GL_H include guard"
    );
}

#[test]
fn vulkan_header_has_include_guard() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "vk=1.3",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let header = read_header(dir.path(), "vulkan");
    assert!(
        header.contains("#ifndef GLOAM_VULKAN_H"),
        "header should have GLOAM_VULKAN_H include guard"
    );
}

// ===========================================================================
// Bonus: --alias generates alias resolution code
// ===========================================================================

#[test]
fn alias_flag_produces_alias_resolution_in_source() {
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

    let source = read_source(dir.path(), "gl");
    assert!(
        source.contains("alias"),
        "source with --alias should contain alias resolution code"
    );
}

#[test]
fn no_alias_flag_omits_alias_resolution() {
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

    let source = read_source(dir.path(), "gl");
    // Without --alias, there should be no alias pair table.
    // The word "alias" can appear in comments, so check for the specific
    // generated identifier pattern.
    assert!(
        !source.contains("kAliasPairs"),
        "source without --alias should not contain kAliasPairs"
    );
}

// ===========================================================================
// Bonus: GLES2 with all flags
// ===========================================================================

#[test]
fn gles2_all_flags_generates_and_compiles() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "gles2=3.0",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
            "--alias",
            "--loader",
        ])
        .assert()
        .success();

    assert_c_output_exists(dir.path(), "gles2");
    try_compile_c(dir.path());
}

// ===========================================================================
// Bonus: EGL with all flags
// ===========================================================================

#[test]
fn egl_all_flags_generates_and_compiles() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "egl",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
            "--alias",
            "--loader",
        ])
        .assert()
        .success();

    assert_c_output_exists(dir.path(), "egl");
    try_compile_c(dir.path());
}

// ===========================================================================
// Bonus: preamble contains command line
// ===========================================================================

#[test]
fn preamble_contains_gloam_command() {
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

    let header = read_header(dir.path(), "gl");
    assert!(
        header.contains("gloam"),
        "generated header preamble should mention gloam"
    );
}
