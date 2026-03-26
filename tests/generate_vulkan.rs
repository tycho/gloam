//! Integration tests for Vulkan loader generation.
//!
//! These tests require that the bundled XML files are populated.
//! They also attempt a C compile step if `cc` is available on PATH.
//!
//! Vulkan WSI headers (X11/Xlib.h, windows.h, etc.) are behind `#ifdef`
//! guards that are not defined during the compile check, so they pose no
//! issue even without a Vulkan SDK installed.

use std::path::Path;

use tempfile::TempDir;

// ---------------------------------------------------------------------------
// Helpers
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

// ---------------------------------------------------------------------------
// Core Vulkan generation
// ---------------------------------------------------------------------------

#[test]
fn vulkan_13_generates_expected_files() {
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

    assert_c_output_exists(dir.path(), "vulkan");
    try_compile_c(dir.path());
}

#[test]
fn vulkan_13_with_loader_generates_and_compiles() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "vk=1.3",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
            "--loader",
        ])
        .assert()
        .success();

    assert_c_output_exists(dir.path(), "vulkan");
    try_compile_c(dir.path());
}

#[test]
fn vulkan_13_with_alias_generates_and_compiles() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "vk=1.3",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
            "--alias",
        ])
        .assert()
        .success();

    assert_c_output_exists(dir.path(), "vulkan");
    try_compile_c(dir.path());
}

#[test]
fn vulkan_13_all_flags_generates_and_compiles() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "vk=1.3",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
            "--alias",
            "--loader",
        ])
        .assert()
        .success();

    assert_c_output_exists(dir.path(), "vulkan");
    try_compile_c(dir.path());
}

#[test]
fn vulkan_latest_version_generates() {
    // No version — should use latest available.
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "vulkan",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    assert_c_output_exists(dir.path(), "vulkan");
    try_compile_c(dir.path());
}

#[test]
fn vulkan_long_name_normalizes_to_vk_stem() {
    // "--api vulkan=1.3" must produce the same output as "--api vk=1.3":
    // files named vk.h / vk.c, not vulkan.h / vulkan.c.
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "vulkan=1.3",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    assert_c_output_exists(dir.path(), "vulkan");
    try_compile_c(dir.path());
}

// ---------------------------------------------------------------------------
// Content smoke tests
// ---------------------------------------------------------------------------

#[test]
fn vulkan_header_has_core_commands() {
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

    let header = read_header(dir.path(), "vulkan");

    // Fundamental Vulkan entry points that must always be present.
    assert!(
        header.contains("PFN_vkCreateInstance"),
        "missing PFN_vkCreateInstance"
    );
    assert!(
        header.contains("PFN_vkCreateDevice"),
        "missing PFN_vkCreateDevice"
    );
    assert!(
        header.contains("PFN_vkGetDeviceProcAddr"),
        "missing PFN_vkGetDeviceProcAddr"
    );
    assert!(
        header.contains("PFN_vkGetInstanceProcAddr"),
        "missing PFN_vkGetInstanceProcAddr"
    );
}

#[test]
fn vulkan_header_has_version_macros() {
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

    let header = read_header(dir.path(), "vulkan");

    assert!(
        header.contains("VK_VERSION_1_0 1"),
        "missing VK_VERSION_1_0"
    );
    assert!(
        header.contains("VK_VERSION_1_1 1"),
        "missing VK_VERSION_1_1"
    );
    assert!(
        header.contains("VK_VERSION_1_2 1"),
        "missing VK_VERSION_1_2"
    );
    assert!(
        header.contains("VK_VERSION_1_3 1"),
        "missing VK_VERSION_1_3"
    );
}

#[test]
fn vulkan_10_does_not_have_13_features() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "vk=1.0",
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
        "missing VK_VERSION_1_0"
    );
    // 1.1+ features should not appear when targeting 1.0.
    assert!(
        !header.contains("VK_VERSION_1_1 1"),
        "VK_VERSION_1_1 should be absent when targeting 1.0"
    );
}

// ---------------------------------------------------------------------------
// Inline dispatch and VK_NO_PROTOTYPES
// ---------------------------------------------------------------------------

#[test]
fn vulkan_header_uses_inline_dispatch() {
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

    let header = read_header(dir.path(), "vulkan");

    // Inline functions should be present, not macro dispatch.
    assert!(
        header.contains("GLOAM_FORCE_INLINE"),
        "missing GLOAM_FORCE_INLINE inline dispatch functions"
    );
    assert!(
        !header.contains("#define vkCreateInstance (gloam_vk_context."),
        "should not use #define macro dispatch for Vulkan"
    );

    // VK_NO_PROTOTYPES guard.
    assert!(
        header.contains("#ifndef VK_NO_PROTOTYPES"),
        "dispatch wrappers should be guarded by VK_NO_PROTOTYPES"
    );
}

// ---------------------------------------------------------------------------
// External headers mode
// ---------------------------------------------------------------------------

#[test]
fn vulkan_external_headers_does_not_bundle_vulkan_headers() {
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
            "--external-headers",
        ])
        .assert()
        .success();

    // xxhash.h should still be bundled (used by the generated .c).
    assert!(
        dir.path().join("include").join("xxhash.h").exists(),
        "xxhash.h should still be bundled in external-headers mode"
    );

    // vk_platform.h and vk_video/ should NOT be bundled.
    assert!(
        !dir.path().join("include").join("vk_platform.h").exists(),
        "vk_platform.h should not be bundled in external-headers mode"
    );
    assert!(
        !dir.path().join("include").join("vk_video").exists(),
        "vk_video/ should not be bundled in external-headers mode"
    );
}

#[test]
fn vulkan_external_headers_generates() {
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
            "--external-headers",
        ])
        .assert()
        .success();

    assert_c_output_exists(dir.path(), "vulkan");

    let header = read_header(dir.path(), "vulkan");

    // Should include upstream vulkan headers.
    assert!(
        header.contains("#include <vulkan/vulkan_core.h>"),
        "external-headers mode should include vulkan_core.h"
    );

    // Should NOT embed type definitions.
    assert!(
        !header.contains("typedef enum VkStructureType"),
        "external-headers mode should not embed Vulkan enum types"
    );

    // Should NOT emit PFN typedefs.
    assert!(
        !header.contains("typedef VkResult (VKAPI_PTR *PFN_vkCreateInstance)"),
        "external-headers mode should not embed PFN typedefs"
    );

    // Should NOT have conflict guards (no #error).
    assert!(
        !header.contains("#error"),
        "external-headers mode should not emit #error conflict guards"
    );

    // Context struct should still be present.
    assert!(
        header.contains("GloamVulkanContext"),
        "context struct should be present in external-headers mode"
    );

    // Inline dispatch and VK_NO_PROTOTYPES should still be present.
    assert!(
        header.contains("GLOAM_FORCE_INLINE"),
        "inline dispatch should be present in external-headers mode"
    );
    assert!(
        header.contains("#ifndef VK_NO_PROTOTYPES"),
        "VK_NO_PROTOTYPES guard should be present in external-headers mode"
    );
}

#[test]
fn vulkan_external_headers_with_loader_generates() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "vk=1.3",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
            "--external-headers",
            "--loader",
        ])
        .assert()
        .success();

    assert_c_output_exists(dir.path(), "vulkan");
}

#[test]
fn vulkan_external_headers_with_all_extensions_generates() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "vk=1.3",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
            "--external-headers",
        ])
        .assert()
        .success();

    assert_c_output_exists(dir.path(), "vulkan");

    let header = read_header(dir.path(), "vulkan");

    // Platform-specific headers should be conditionally included.
    assert!(
        header.contains("#ifdef VK_USE_PLATFORM_WIN32_KHR"),
        "external-headers should have Win32 platform guard"
    );
    assert!(
        header.contains("#include <vulkan/vulkan_win32.h>"),
        "external-headers should include vulkan_win32.h"
    );
}

// ---------------------------------------------------------------------------
// Vulkan with explicit extension selection
// ---------------------------------------------------------------------------

#[test]
fn vulkan_with_extension_filter_generates() {
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "vk=1.3",
            "--extensions",
            "VK_KHR_swapchain",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    let header = read_header(dir.path(), "vulkan");
    assert!(
        header.contains("KHR_swapchain"),
        "VK_KHR_swapchain should be present when explicitly requested"
    );
    try_compile_c(dir.path());
}
