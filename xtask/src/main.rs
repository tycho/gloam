//! gloam dev task runner.
//!
//! `cargo xtask bundle` refreshes the embedded bundle: it fetches every
//! registry file at upstream HEAD via gloam's own acquisition path and writes
//! both the file bytes under `bundled/` and the provenance manifest
//! `bundled/provenance.json`.  Sharing gloam's acquisition code guarantees the
//! bundled and `--fetch` provenance are produced identically.
//!
//! `cargo xtask regen [tree-root] [--fresh]` builds the working-copy gloam
//! and runs `gloam regen` over `tree-root` (default: the current directory),
//! regenerating every gloam output tree found beneath it (e.g. `examples/`
//! here, or a gloam-pregen checkout).  Discovery, path handling, and the
//! locked-vs-fresh semantics all live in `gloam regen` itself; this task
//! just supplies a freshly built binary.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use indexmap::IndexMap;

use gloam::provenance::acquire::Github;
use gloam::provenance::manifest::{
    BundledProvenance, ProvenancePin, SCHEMA_VERSION, preserve_unchanged_repos,
};
use gloam::provenance::{CLUSTERS, bundled_rel_path};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("bundle") | None => bundle(),
        Some("regen") => regen(&args[1..]),
        Some(other) => bail!("unknown xtask command '{other}' (try: bundle, regen)"),
    }
}

// ---------------------------------------------------------------------------
// regen
// ---------------------------------------------------------------------------

fn regen(args: &[String]) -> Result<()> {
    let mut root: Option<PathBuf> = None;
    let mut fresh = false;
    for a in args {
        match a.as_str() {
            "--fresh" => fresh = true,
            other if root.is_none() => root = Some(PathBuf::from(other)),
            other => bail!("unexpected regen argument '{other}'"),
        }
    }
    let root = root.unwrap_or_else(|| PathBuf::from("."));

    // Build and locate the working-copy gloam binary.
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = std::process::Command::new(&cargo)
        .args(["build", "-p", "gloam"])
        .current_dir(repo_root())
        .status()
        .context("running cargo build")?;
    if !status.success() {
        bail!("cargo build -p gloam failed");
    }
    let bin = repo_root()
        .join("target")
        .join("debug")
        .join(format!("gloam{}", std::env::consts::EXE_SUFFIX));

    // `gloam regen` does the rest: recursive discovery, path handling, and
    // locked-vs-fresh replay.  It runs with our cwd, so a relative root
    // behaves the same as it would for find/grep.
    let mut cmd = std::process::Command::new(&bin);
    cmd.arg("regen");
    if fresh {
        cmd.arg("--fresh");
    }
    cmd.arg(&root);
    let status = cmd
        .status()
        .with_context(|| format!("running {} regen", bin.display()))?;
    if !status.success() {
        bail!("gloam regen failed");
    }
    Ok(())
}

/// Best-effort read of the checked-in provenance manifest.  Missing,
/// unreadable, or schema-mismatched files are ignored — the bundle is simply
/// recorded fresh at the newly resolved commits.
fn read_previous(path: &Path) -> Option<BundledProvenance> {
    let text = std::fs::read_to_string(path).ok()?;
    let m = BundledProvenance::from_json(&text).ok()?;
    (m.schema_version == SCHEMA_VERSION).then_some(m)
}

/// Repository root (xtask lives directly under it).
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask is under the workspace root")
        .to_path_buf()
}

fn bundle() -> Result<()> {
    let bundled_dir = repo_root().join("bundled");
    let gh = Github::new()?;
    let mut pins: Vec<(String, ProvenancePin)> = Vec::new();

    for cluster in CLUSTERS {
        let keys: Vec<&str> = cluster.files.iter().map(|f| f.key).collect();
        eprintln!("· {} ({} files)", cluster.repo, keys.len());
        let fetched = gh
            .resolve_cluster_head(cluster, &keys)
            .with_context(|| format!("resolving {}", cluster.repo))?;
        eprintln!(
            "    commit {}",
            &fetched.repo.commit[..12.min(fetched.repo.commit.len())]
        );

        for (file, content) in &fetched.files {
            let dest = bundled_dir.join(bundled_rel_path(&file.key));
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("creating {}", parent.display()))?;
            }
            std::fs::write(&dest, content)
                .with_context(|| format!("writing {}", dest.display()))?;

            pins.push((
                file.key.clone(),
                ProvenancePin {
                    repo: fetched.repo.repo.clone(),
                    repo_url: fetched.repo.repo_url.clone(),
                    path_in_repo: file.path_in_repo.clone(),
                    commit: fetched.repo.commit.clone(),
                    blob: file.blob.clone(),
                },
            ));
        }
    }

    // Deterministic key order.
    pins.sort_by(|a, b| a.0.cmp(&b.0));
    let mut provenance = IndexMap::new();
    for (key, pin) in pins {
        provenance.insert(key, pin);
    }

    let dest = bundled_dir.join("provenance.json");

    // Carry forward commit/describe from the checked-in manifest for every
    // repo whose pinned content is unchanged, mirroring `gloam lock`.  An
    // upstream commit that touches nothing we bundle then leaves
    // provenance.json — and `--version` and every bundled-mode preamble
    // derived from it — byte-identical across re-bundles.
    if let Some(prev) = read_previous(&dest) {
        for repo in preserve_unchanged_repos(&mut provenance, &prev.provenance) {
            eprintln!("    {repo}: pinned content unchanged, keeping previous commit");
        }
    }

    let manifest = BundledProvenance {
        schema_version: SCHEMA_VERSION,
        provenance,
    };
    // Trailing newline so the file ends cleanly for diffs/editors.
    std::fs::write(&dest, manifest.to_json_pretty() + "\n")
        .with_context(|| format!("writing {}", dest.display()))?;
    eprintln!("wrote {}", dest.display());

    Ok(())
}
