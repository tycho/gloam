//! Integration tests for `gloam regen` — in-place regeneration of existing
//! output trees by replaying each tree's recorded command line.
//!
//! The headline property: regeneration works from *any* working directory,
//! because the effective output path is derived from the manifest's own
//! location, not from the recorded `--out-path` (which was relative to
//! whatever cwd the original invocation happened to use).  Everything here
//! is offline: the bundled blobs match every recorded pin by construction.

mod common;
use common::{collect_files, gloam};

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use tempfile::TempDir;

/// Full byte snapshot of a tree, keyed by root-relative path.
fn snapshot(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    collect_files(root)
        .into_iter()
        .map(|rel| {
            let bytes = std::fs::read(root.join(&rel)).unwrap();
            (rel, bytes)
        })
        .collect()
}

/// Corrupt one generated (non-manifest) file so a byte-identical result
/// proves regen actually rewrote the tree.  The manifest must stay intact —
/// it is regen's input.
fn corrupt_one_output(root: &Path) -> PathBuf {
    let victim = collect_files(root)
        .into_iter()
        .find(|rel| !rel.starts_with(".gloam"))
        .expect("tree has generated files");
    std::fs::write(root.join(&victim), b"corrupted\n").unwrap();
    victim
}

#[test]
fn regen_restores_tree_from_a_different_cwd() {
    // Generate with a cwd-relative --out-path, then regen from an unrelated
    // cwd by naming the tree — the case the recorded path can't handle.
    let dir = TempDir::new().unwrap();
    let elsewhere = TempDir::new().unwrap();
    gloam()
        .current_dir(dir.path())
        .args([
            "--quiet",
            "--api",
            "gl:core=3.3",
            "--out-path",
            "out",
            "c",
            "--alias",
        ])
        .assert()
        .success();

    let tree = dir.path().join("out");
    let before = snapshot(&tree);
    assert!(!before.is_empty());
    corrupt_one_output(&tree);

    gloam()
        .current_dir(elsewhere.path())
        .args(["--quiet", "regen", tree.to_str().unwrap()])
        .assert()
        .success();

    assert_eq!(
        snapshot(&tree),
        before,
        "locked regen from another cwd must restore the tree byte-identically"
    );
}

#[test]
fn regen_preserves_recorded_command_line_verbatim() {
    // The recorded command line — including its original, cwd-relative
    // --out-path — is a historical record; regen must re-record it verbatim
    // even though placement is derived from the manifest location.
    let dir = TempDir::new().unwrap();
    let elsewhere = TempDir::new().unwrap();
    gloam()
        .current_dir(dir.path())
        .args(["--quiet", "--api", "egl", "--out-path", "out", "c"])
        .assert()
        .success();

    let manifest = dir.path().join("out").join(".gloam").join("manifest.json");
    let recorded = |path: &Path| -> String {
        let m: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
        m["gloam"]["command_line"].as_str().unwrap().to_string()
    };
    let original = recorded(&manifest);
    assert!(
        original.contains("--out-path out"),
        "precondition: the original invocation's out-path is recorded: {original}"
    );

    gloam()
        .current_dir(elsewhere.path())
        .args(["--quiet", "regen", dir.path().to_str().unwrap()])
        .assert()
        .success();
    assert_eq!(
        recorded(&manifest),
        original,
        "regen must not rewrite the recorded command line"
    );
}

#[test]
fn regen_recurses_and_handles_c_and_rust_trees() {
    // One C tree and one Rust tree under a common root, both with recorded
    // out-paths that are meaningless outside their original cwds.  A single
    // recursive regen of the root must restore both.
    let root = TempDir::new().unwrap();
    gloam()
        .current_dir(root.path())
        .args([
            "--quiet",
            "--api",
            "gl:core=3.3",
            "--out-path",
            "a/gloam",
            "c",
        ])
        .assert()
        .success();
    gloam()
        .current_dir(root.path())
        .args([
            "--quiet",
            "--api",
            "gles2=3.0",
            "--out-path",
            "b/gloam",
            "rust",
        ])
        .assert()
        .success();

    let tree_a = root.path().join("a").join("gloam");
    let tree_b = root.path().join("b").join("gloam");
    let before_a = snapshot(&tree_a);
    let before_b = snapshot(&tree_b);
    corrupt_one_output(&tree_a);
    corrupt_one_output(&tree_b);

    let elsewhere = TempDir::new().unwrap();
    gloam()
        .current_dir(elsewhere.path())
        .args(["--quiet", "regen", root.path().to_str().unwrap()])
        .assert()
        .success();

    assert_eq!(snapshot(&tree_a), before_a, "C tree not restored");
    assert_eq!(snapshot(&tree_b), before_b, "Rust tree not restored");
}

#[test]
fn regen_fresh_matches_locked_output_offline() {
    // With bundled sources, --fresh re-resolves to the same pins the tree
    // recorded, so the result is still byte-identical.
    let dir = TempDir::new().unwrap();
    gloam()
        .current_dir(dir.path())
        .args(["--quiet", "--api", "egl", "--out-path", "out", "c"])
        .assert()
        .success();

    let tree = dir.path().join("out");
    let before = snapshot(&tree);
    corrupt_one_output(&tree);

    gloam()
        .args(["--quiet", "regen", "--fresh", tree.to_str().unwrap()])
        .assert()
        .success();
    assert_eq!(snapshot(&tree), before);
}

#[test]
fn regen_regenerates_lock_snapshot_in_place() {
    // A bare `gloam lock` snapshot is also self-describing; naming the file
    // regenerates it in place, byte-identically under the same gloam.
    let dir = TempDir::new().unwrap();
    let snap = dir.path().join("manifest.json");
    gloam()
        .args(["--quiet", "lock", "--out", snap.to_str().unwrap()])
        .assert()
        .success();
    let before = std::fs::read(&snap).unwrap();

    gloam()
        .args(["--quiet", "regen", snap.to_str().unwrap()])
        .assert()
        .success();
    assert_eq!(std::fs::read(&snap).unwrap(), before);
}

#[test]
fn regen_skips_foreign_manifest_json() {
    // Discovery must ignore unrelated files named manifest.json (npm, PWA,
    // etc.) while still regenerating real trees next to them.
    let root = TempDir::new().unwrap();
    std::fs::write(
        root.path().join("manifest.json"),
        r#"{ "name": "someone-elses-app", "start_url": "/" }"#,
    )
    .unwrap();
    gloam()
        .current_dir(root.path())
        .args(["--quiet", "--api", "egl", "--out-path", "out", "c"])
        .assert()
        .success();

    let tree = root.path().join("out");
    let before = snapshot(&tree);
    corrupt_one_output(&tree);

    gloam()
        .args(["--quiet", "regen", root.path().to_str().unwrap()])
        .assert()
        .success();
    assert_eq!(snapshot(&tree), before);
}

#[test]
fn regen_with_no_manifests_errors() {
    let empty = TempDir::new().unwrap();
    let output = gloam()
        .args(["regen", empty.path().to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("no gloam manifests found"), "{stderr}");
}

#[test]
fn regen_explicit_manifest_with_wrong_schema_errors() {
    // Schema mismatches are skipped (with a warning) during discovery, but
    // an explicitly named manifest is a contract — refuse loudly.
    let dir = TempDir::new().unwrap();
    gloam()
        .current_dir(dir.path())
        .args(["--quiet", "--api", "egl", "--out-path", "out", "c"])
        .assert()
        .success();
    let manifest = dir.path().join("out").join(".gloam").join("manifest.json");
    let mut m: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&manifest).unwrap()).unwrap();
    m["schema_version"] = serde_json::json!(1);
    std::fs::write(&manifest, serde_json::to_string_pretty(&m).unwrap()).unwrap();

    let output = gloam()
        .args(["regen", manifest.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("schema_version"), "{stderr}");
}
