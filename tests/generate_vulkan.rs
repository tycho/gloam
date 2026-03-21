//! Integration tests for Vulkan loader generation.
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

fn try_compile_c(out: &Path) {
    let src_dir = out.join("src");
    let c_file = match std::fs::read_dir(&src_dir).ok().and_then(|mut d| {
        d.find(|e| {
            e.as_ref()
                .is_ok_and(|e| e.path().extension() == Some("c".as_ref()))
        })
    }) {
        Some(Ok(entry)) => entry.path(),
        _ => return,
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

    assert_c_output_exists(dir.path(), "vk");
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

    assert_c_output_exists(dir.path(), "vk");
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

    assert_c_output_exists(dir.path(), "vk");
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

    assert_c_output_exists(dir.path(), "vk");
    try_compile_c(dir.path());
}

#[test]
fn vulkan_latest_version_generates() {
    // No version — should use latest available.
    let dir = TempDir::new().unwrap();
    gloam()
        .args([
            "--api",
            "vk",
            "--out-path",
            dir.path().to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();

    assert_c_output_exists(dir.path(), "vk");
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

    assert_c_output_exists(dir.path(), "vk");
    try_compile_c(dir.path());
}

// ---------------------------------------------------------------------------
// Content smoke tests
// ---------------------------------------------------------------------------

#[test]
fn vulkan_header_has_context_and_scope_enum() {
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

    let header = read_header(dir.path(), "vk");

    assert!(
        header.contains("GloamVulkanContext"),
        "missing context struct"
    );
    assert!(
        header.contains("GloamCommandScopeDevice"),
        "missing device scope enum"
    );
    assert!(
        header.contains("GloamCommandScopeInstance"),
        "missing instance scope enum"
    );
    assert!(
        header.contains("GloamCommandScopeGlobal"),
        "missing global scope enum"
    );
}

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

    let header = read_header(dir.path(), "vk");

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

    let header = read_header(dir.path(), "vk");

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

    let header = read_header(dir.path(), "vk");

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

    let header = read_header(dir.path(), "vk");
    assert!(
        header.contains("KHR_swapchain"),
        "VK_KHR_swapchain should be present when explicitly requested"
    );
    try_compile_c(dir.path());
}
