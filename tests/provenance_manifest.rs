//! Integration tests for the implicit provenance baseline: regenerating into
//! an existing tree must preserve commit/describe for repos whose pinned
//! content is unchanged, and advance whole repos whose content changed.

mod common;
use common::generate as generate_fresh;
use common::gloam;

use std::path::Path;

use tempfile::TempDir;

/// Generate a small GL loader into `dir` (regenerating in place on repeat calls).
fn generate_gl(dir: &Path) {
    gloam()
        .args([
            "--api",
            "gl:core=3.3",
            "--out-path",
            dir.to_str().unwrap(),
            "c",
        ])
        .assert()
        .success();
}

/// Rewrite every provenance pin's commit/describe in the tree manifest to a
/// sentinel, optionally corrupting one pin's blob.  This simulates "the
/// previous run recorded older commits" without needing network.
fn tamper_manifest(dir: &Path, corrupt_blob_key: Option<&str>) {
    let path = dir.join(".gloam").join("manifest.json");
    let mut m: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    for (key, pin) in m["provenance"].as_object_mut().unwrap() {
        pin["commit"] = serde_json::json!("f".repeat(40));
        pin["describe"] = serde_json::json!("sentinelver");
        if corrupt_blob_key == Some(key.as_str()) {
            pin["blob"] = serde_json::json!("0".repeat(40));
        }
    }
    std::fs::write(&path, serde_json::to_string_pretty(&m).unwrap()).unwrap();
}

fn manifest_pins(dir: &Path) -> serde_json::Value {
    let path = dir.join(".gloam").join("manifest.json");
    serde_json::from_str::<serde_json::Value>(&std::fs::read_to_string(&path).unwrap()).unwrap()
        ["provenance"]
        .clone()
}

#[test]
fn regeneration_preserves_unchanged_provenance_commits() {
    let dir = TempDir::new().unwrap();
    generate_gl(dir.path());
    tamper_manifest(dir.path(), None);

    // Regenerate: every blob still matches the (bundled) sources, so the
    // sentinel commit/describe must be carried forward everywhere...
    generate_gl(dir.path());
    let pins = manifest_pins(dir.path());
    for (key, pin) in pins.as_object().unwrap() {
        assert_eq!(
            pin["commit"].as_str().unwrap(),
            "f".repeat(40),
            "pin '{key}' should keep the previous commit"
        );
    }

    // ...including into the generated header's sources block.
    let header =
        std::fs::read_to_string(dir.path().join("include").join("gloam").join("gl.h")).unwrap();
    assert!(
        header.contains("(sentinelver)"),
        "preamble should use the preserved describe"
    );

    // A further regeneration with nothing changed is byte-identical.
    let manifest_before = std::fs::read(dir.path().join(".gloam").join("manifest.json")).unwrap();
    let header_before =
        std::fs::read(dir.path().join("include").join("gloam").join("gl.h")).unwrap();
    generate_gl(dir.path());
    assert_eq!(
        std::fs::read(dir.path().join(".gloam").join("manifest.json")).unwrap(),
        manifest_before,
        "repeat regeneration must leave the manifest byte-identical"
    );
    assert_eq!(
        std::fs::read(dir.path().join("include").join("gloam").join("gl.h")).unwrap(),
        header_before,
        "repeat regeneration must leave the header byte-identical"
    );
}

#[test]
fn regeneration_advances_repo_whose_blob_changed() {
    let dir = TempDir::new().unwrap();
    generate_gl(dir.path());
    // Sentinel commits everywhere, but gl.xml's recorded blob no longer
    // matches the source content — OpenGL-Registry must advance as a whole
    // while untouched repos keep the sentinel.
    tamper_manifest(dir.path(), Some("gl.xml"));

    generate_gl(dir.path());
    let pins = manifest_pins(dir.path());
    for (key, pin) in pins.as_object().unwrap() {
        let commit = pin["commit"].as_str().unwrap();
        if pin["repo"].as_str().unwrap() == "KhronosGroup/OpenGL-Registry" {
            assert_ne!(
                commit,
                "f".repeat(40),
                "pin '{key}' belongs to a changed repo and must advance"
            );
        } else {
            assert_eq!(
                commit,
                "f".repeat(40),
                "pin '{key}' is unchanged and must keep the previous commit"
            );
        }
    }
}

#[test]
fn fresh_generation_records_manifest() {
    // A fresh tree gets a .gloam/manifest.json with a provenance section
    // covering at least the primary spec and xxhash.h.
    let dir = generate_fresh(&["--api", "gl:core=3.3"], &[]);
    let pins = manifest_pins(dir.path());
    let obj = pins.as_object().unwrap();
    assert!(obj.contains_key("gl.xml"), "manifest should pin gl.xml");
    assert!(obj.contains_key("xxhash.h"), "manifest should pin xxhash.h");
}
