//! gloam dev task runner.
//!
//! `cargo xtask bundle` refreshes the embedded bundle: it fetches every
//! registry file at upstream HEAD via gloam's own acquisition path and writes
//! both the file bytes under `bundled/` and the provenance manifest
//! `bundled/provenance.json`.  Sharing gloam's acquisition code guarantees the
//! bundled and `--fetch` provenance are produced identically.
//!
//! `cargo xtask regen <tree-root> [--fresh]` regenerates every gloam output
//! tree found under `<tree-root>` (e.g. a gloam-pregen checkout) by re-running
//! the command line recorded in each tree's manifest with the current working
//! copy of gloam.  By default each run is pinned to the tree's recorded
//! provenance (`--lock`), so `git diff` in the tree shows only the effect of
//! gloam code changes; `--fresh` runs the recorded commands verbatim instead,
//! advancing to upstream HEAD (the normal tree-update workflow).

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use indexmap::IndexMap;

use gloam::provenance::acquire::Github;
use gloam::provenance::manifest::{
    BundledProvenance, Manifest, ProvenancePin, SCHEMA_VERSION, preserve_unchanged_repos,
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
    let root = root.context("usage: cargo xtask regen <tree-root> [--fresh]")?;
    if !root.is_dir() {
        bail!("tree root {} is not a directory", root.display());
    }
    // Keep the path as given (no canonicalize: Windows turns those into
    // \\?\ UNC paths, which is noise in every printed command).  Child
    // processes run with this as their cwd, so relative roots work too.

    // Every gloam manifest records the exact command line that produced its
    // tree, so the tree set is self-describing: generation trees carry
    // `.gloam/manifest.json`, lock snapshots are bare `manifest.json` files.
    let mut manifests: Vec<PathBuf> = Vec::new();
    find_manifests(&root, &mut manifests);
    manifests.sort();
    if manifests.is_empty() {
        bail!("no gloam manifests found under {}", root.display());
    }

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

    let mut ran = 0usize;
    for manifest_path in &manifests {
        let Some(recorded) = recorded_command(manifest_path) else {
            continue; // not a gloam manifest (or unreadable) — skip
        };

        let rel = manifest_path.strip_prefix(&root).unwrap_or(manifest_path);

        // Drop the recorded argv[0] ("gloam"); in locked mode, pin the run to
        // the tree's own provenance.  --lock must precede the subcommand, so
        // it goes first.  The path is root-relative because the child runs
        // with the tree root as its cwd (recorded --out-path values are too).
        let mut argv: Vec<String> = recorded.split_whitespace().skip(1).map(String::from).collect();
        if !fresh {
            argv.splice(0..0, ["--lock".to_string(), rel.display().to_string()]);
        }
        eprintln!("· {} $ gloam {}", rel.display(), argv.join(" "));

        // Recorded --out-path values are relative to the tree root.
        let status = std::process::Command::new(&bin)
            .args(&argv)
            .current_dir(&root)
            .status()
            .with_context(|| format!("running gloam for {}", rel.display()))?;
        if !status.success() {
            bail!("gloam failed for {}", rel.display());
        }
        ran += 1;
    }

    if ran == 0 {
        bail!(
            "found {} manifest.json file(s) under {}, but none recorded a gloam command line",
            manifests.len(),
            root.display()
        );
    }

    eprintln!("regenerated {ran} tree(s) under {}", root.display());
    eprintln!("review with:");
    eprintln!(
        "  git -C {} diff -I'^ \\* Generated by gloam ' -I'^    \"(version|describe|commit)\": '",
        root.display()
    );
    Ok(())
}

/// Recursively collect `manifest.json` files under `dir`, skipping `.git`.
fn find_manifests(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|n| n == ".git") {
                continue;
            }
            find_manifests(&path, out);
        } else if path.file_name().is_some_and(|n| n == "manifest.json") {
            out.push(path);
        }
    }
}

/// The gloam command line recorded in a manifest, if the file parses as a
/// gloam manifest at the current schema and recorded one.
fn recorded_command(path: &Path) -> Option<String> {
    let text = std::fs::read_to_string(path).ok()?;
    let m = Manifest::from_json(&text).ok()?;
    if m.schema_version != SCHEMA_VERSION {
        eprintln!(
            "· {}: skipping (schema_version {} != {})",
            path.display(),
            m.schema_version,
            SCHEMA_VERSION
        );
        return None;
    }
    let cmd = m.gloam.command_line;
    (cmd.split_whitespace().next() == Some("gloam")).then_some(cmd)
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
