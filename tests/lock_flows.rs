//! Integration tests for `gloam lock` and `--lock` reproduction — the
//! headline provenance features.  Everything here is offline: the bundled
//! blobs match `bundled/provenance.json` by construction, so locked
//! resolution succeeds without `--fetch`.

mod common;
use common::{collect_files, gloam};

use tempfile::TempDir;

// ---------------------------------------------------------------------------
// gloam lock (snapshot writing)
// ---------------------------------------------------------------------------

#[test]
fn lock_snapshot_is_idempotent() {
    // Two consecutive snapshots to the same path must be byte-identical:
    // the second run carries forward commit metadata for unchanged repos.
    let dir = TempDir::new().unwrap();
    let out = dir.path().join("manifest.json");

    for _ in 0..2 {
        gloam()
            .args(["--quiet", "lock", "--out", out.to_str().unwrap()])
            .assert()
            .success();
    }
    let first = std::fs::read(&out).unwrap();

    gloam()
        .args(["--quiet", "lock", "--out", out.to_str().unwrap()])
        .assert()
        .success();
    assert_eq!(
        std::fs::read(&out).unwrap(),
        first,
        "repeated lock snapshots must be byte-identical"
    );
}

#[test]
fn lock_snapshot_creates_parent_directories() {
    let dir = TempDir::new().unwrap();
    let out = dir.path().join("nested").join("dirs").join("manifest.json");
    gloam()
        .args(["--quiet", "lock", "--out", out.to_str().unwrap()])
        .assert()
        .success();
    assert!(out.exists());
}

#[test]
fn lock_snapshot_covers_every_registry_key() {
    let dir = TempDir::new().unwrap();
    let out = dir.path().join("manifest.json");
    gloam()
        .args(["--quiet", "lock", "--out", out.to_str().unwrap()])
        .assert()
        .success();

    let m: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&out).unwrap()).unwrap();
    let pins = m["provenance"].as_object().unwrap();
    // Spot-check the corners of the registry: every spec family's primary
    // XML, an auxiliary header, and the gloam-registry supplemental.
    for key in ["gl.xml", "egl.xml", "vk.xml", "xxhash.h", "glsl_exts.xml"] {
        assert!(pins.contains_key(key), "snapshot must pin '{key}'");
    }
    assert!(
        m["output"].as_array().unwrap().is_empty(),
        "a lock snapshot has no output BOM"
    );
}

// ---------------------------------------------------------------------------
// --lock reproduction
// ---------------------------------------------------------------------------

/// Generate with identical args and a *relative* out-path from two different
/// working directories (so the recorded command line — and therefore every
/// byte — can match), the second run locked to the first run's manifest.
#[test]
fn locked_regeneration_reproduces_byte_identical_tree() {
    let dir_a = TempDir::new().unwrap();
    let dir_b = TempDir::new().unwrap();
    let args = ["--api", "gl:core=3.3", "--out-path", "out", "c", "--alias"];

    gloam()
        .current_dir(dir_a.path())
        .args(args)
        .assert()
        .success();

    let manifest_a = dir_a.path().join("out").join(".gloam").join("manifest.json");
    gloam()
        .current_dir(dir_b.path())
        .args(["--lock", manifest_a.to_str().unwrap()])
        .args(args)
        .assert()
        .success();

    let out_a = dir_a.path().join("out");
    let out_b = dir_b.path().join("out");
    let files_a = collect_files(&out_a);
    assert_eq!(files_a, collect_files(&out_b), "file lists differ");
    assert!(!files_a.is_empty());
    for rel in &files_a {
        assert_eq!(
            std::fs::read(out_a.join(rel)).unwrap(),
            std::fs::read(out_b.join(rel)).unwrap(),
            "locked reproduction of {} must be byte-identical",
            rel.display()
        );
    }
}

#[test]
fn lock_missing_pin_is_refused_with_guidance() {
    // Generate, then strip one required pin from the manifest: regeneration
    // under --lock must refuse and point at the fix.
    let dir = TempDir::new().unwrap();
    let args = ["--api", "gl:core=3.3", "--out-path", "out", "c"];
    gloam()
        .current_dir(dir.path())
        .args(args)
        .assert()
        .success();

    let manifest = dir.path().join("out").join(".gloam").join("manifest.json");
    let mut m: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&manifest).unwrap()).unwrap();
    m["provenance"].as_object_mut().unwrap().remove("gl.xml");
    std::fs::write(&manifest, serde_json::to_string_pretty(&m).unwrap()).unwrap();

    let output = gloam()
        .current_dir(dir.path())
        .args(["--lock", manifest.to_str().unwrap()])
        .args(args)
        .output()
        .unwrap();
    assert!(!output.status.success(), "missing pin must be refused");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("no provenance for required file 'gl.xml'"),
        "{stderr}"
    );
    assert!(
        stderr.contains("regenerate without --lock"),
        "refusal should carry guidance: {stderr}"
    );
}

#[test]
fn lock_blob_mismatch_without_fetch_is_refused() {
    // A locked blob that doesn't match this build's bundle needs --fetch.
    let dir = TempDir::new().unwrap();
    let args = ["--api", "gl:core=3.3", "--out-path", "out", "c"];
    gloam()
        .current_dir(dir.path())
        .args(args)
        .assert()
        .success();

    let manifest = dir.path().join("out").join(".gloam").join("manifest.json");
    let mut m: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&manifest).unwrap()).unwrap();
    m["provenance"]["gl.xml"]["blob"] = serde_json::json!("0".repeat(40));
    std::fs::write(&manifest, serde_json::to_string_pretty(&m).unwrap()).unwrap();

    let output = gloam()
        .current_dir(dir.path())
        .args(["--lock", manifest.to_str().unwrap()])
        .args(args)
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("does not match the locked blob"),
        "{stderr}"
    );
}

#[test]
fn lock_rejects_older_schema_versions() {
    // --lock is a contract: a v1 manifest must be refused, not reinterpreted.
    let dir = TempDir::new().unwrap();
    let args = ["--api", "gl:core=3.3", "--out-path", "out", "c"];
    gloam()
        .current_dir(dir.path())
        .args(args)
        .assert()
        .success();

    let manifest = dir.path().join("out").join(".gloam").join("manifest.json");
    let mut m: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&manifest).unwrap()).unwrap();
    m["schema_version"] = serde_json::json!(1);
    std::fs::write(&manifest, serde_json::to_string_pretty(&m).unwrap()).unwrap();

    let output = gloam()
        .current_dir(dir.path())
        .args(["--lock", manifest.to_str().unwrap()])
        .args(args)
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("schema_version"), "{stderr}");
}

#[test]
fn lock_snapshot_usable_as_lock_input() {
    // A `gloam lock` snapshot pins every registry key, so generation locked
    // to it succeeds offline for any API.
    let dir = TempDir::new().unwrap();
    let snapshot = dir.path().join("manifest.json");
    gloam()
        .args(["--quiet", "lock", "--out", snapshot.to_str().unwrap()])
        .assert()
        .success();

    gloam()
        .current_dir(dir.path())
        .args(["--lock", snapshot.to_str().unwrap()])
        .args(["--api", "egl", "--out-path", "out", "c"])
        .assert()
        .success();
    assert!(dir.path().join("out").join("src").join("egl.c").exists());
}
