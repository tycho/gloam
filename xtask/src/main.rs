//! gloam dev task runner.
//!
//! `cargo xtask bundle` refreshes the embedded bundle: it fetches every
//! registry file at upstream HEAD via gloam's own acquisition path and writes
//! both the file bytes under `bundled/` and the provenance manifest
//! `bundled/provenance.json`.  Sharing gloam's acquisition code guarantees the
//! bundled and `--fetch` provenance are produced identically.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use indexmap::IndexMap;

use gloam::provenance::acquire::Github;
use gloam::provenance::manifest::{BundledProvenance, ProvenancePin, SCHEMA_VERSION};
use gloam::provenance::{CLUSTERS, bundled_rel_path};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("bundle") | None => bundle(),
        Some(other) => bail!("unknown xtask command '{other}' (try: bundle)"),
    }
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
            "    {} @ {}",
            fetched.repo.describe,
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
                    describe: fetched.repo.describe.clone(),
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

    let manifest = BundledProvenance {
        schema_version: SCHEMA_VERSION,
        provenance,
    };
    let dest = bundled_dir.join("provenance.json");
    // Trailing newline so the file ends cleanly for diffs/editors.
    std::fs::write(&dest, manifest.to_json_pretty() + "\n")
        .with_context(|| format!("writing {}", dest.display()))?;
    eprintln!("wrote {}", dest.display());

    Ok(())
}
