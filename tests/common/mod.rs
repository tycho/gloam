//! Shared harness for gloam integration tests.
//!
//! Each integration-test crate pulls this in with `mod common;`.  Every crate
//! compiles its own copy and uses a subset of the helpers, so unused-item
//! warnings are suppressed for the module as a whole.
#![allow(dead_code)]

use std::path::{Path, PathBuf};

use tempfile::TempDir;

/// The gloam binary under test.
pub fn gloam() -> assert_cmd::Command {
    assert_cmd::Command::cargo_bin("gloam").expect("gloam binary not found")
}

/// Run gloam into a fresh temp dir and assert success.
///
/// `global_args` go before the `c` subcommand (e.g. `--api`, `--extensions`,
/// `--merge`); `c_flags` go after it (e.g. `--alias`, `--loader`).  Returns
/// the temp dir holding the generated tree; it is deleted on drop.
pub fn generate(global_args: &[&str], c_flags: &[&str]) -> TempDir {
    let dir = TempDir::new().unwrap();
    gloam()
        .args(global_args)
        .args(["--out-path", dir.path().to_str().unwrap(), "c"])
        .args(c_flags)
        .assert()
        .success();
    dir
}

/// Assert that the standard C output pair exists for `stem`.
pub fn assert_c_output_exists(out: &Path, stem: &str) {
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

/// Read the generated header for `stem`.
pub fn read_header(out: &Path, stem: &str) -> String {
    std::fs::read_to_string(out.join("include").join("gloam").join(format!("{stem}.h")))
        .unwrap_or_else(|_| panic!("missing include/gloam/{stem}.h"))
}

/// Read the generated source for `stem`.
pub fn read_source(out: &Path, stem: &str) -> String {
    std::fs::read_to_string(out.join("src").join(format!("{stem}.c")))
        .unwrap_or_else(|_| panic!("missing src/{stem}.c"))
}

/// True if the extArray struct contains a slot for `short_name`
/// (e.g. "ARB_copy_buffer").  The generated member looks like:
///   unsigned char ARB_copy_buffer;
pub fn has_ext(header: &str, short_name: &str) -> bool {
    header.contains(&format!("unsigned char {short_name};"))
}

/// Attempt to compile generated C sources with the system C compiler.
/// Uses the `cc` crate for compiler detection (handles MSVC, GCC, Clang,
/// cross-compilation toolchains, CC env override, etc.).
/// Silently skips if no compiler is available.
pub fn try_compile_c(out: &Path) {
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

    // The cc crate expects TARGET/HOST env vars (normally set by Cargo during
    // build.rs).  In test context they're absent, so provide them.
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
        // Distinguish "no compiler" from "compilation failed".
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

/// Recursively collect all file paths under `root`, relative to it, sorted.
pub fn collect_files(root: &Path) -> Vec<PathBuf> {
    fn walk(base: &Path, dir: &Path, out: &mut Vec<PathBuf>) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk(base, &path, out);
                } else {
                    out.push(path.strip_prefix(base).unwrap().to_path_buf());
                }
            }
        }
    }
    let mut files = Vec::new();
    walk(root, root, &mut files);
    files.sort();
    files
}
