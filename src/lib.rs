//! gloam — loader generator for Vulkan, OpenGL, OpenGL ES, EGL, GLX, and WGL.
//!
//! The crate is split into a library (this file) and a thin binary
//! (`src/main.rs`).  The library form lets dev tooling — notably the
//! `cargo xtask` bundler — reuse gloam's acquisition and provenance code
//! instead of duplicating it.

mod build_info;
mod bundled;
mod cli;
mod diag;
mod fetch;
mod generator;
mod identity;
mod ir;
mod parse;
mod preamble;
pub mod provenance;
mod resolve;
mod version;

use anyhow::{Context, Result};
use clap::Parser;
use indexmap::IndexMap;

use cli::{Cli, Generator};
use provenance::manifest::{GloamMeta, Manifest, OutputEntry, ProvenancePin, SCHEMA_VERSION};

/// Binary entry point.  Parses the CLI and runs generation, printing errors
/// and setting the process exit code.
pub fn main() {
    if let Err(e) = run() {
        eprintln!("error: {:#}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    let argv: Vec<String> = std::env::args().collect();
    let command_line = reconstruct_command_line(&argv);

    // A --lock manifest pins upstream sources to recorded provenance.  Only its
    // `provenance` section is used; everything else is regenerated.  Unlike the
    // best-effort implicit baseline (`read_snapshot`), --lock is a contract, so
    // an unknown schema version is refused rather than reinterpreted.
    let lock_pins: Option<IndexMap<String, ProvenancePin>> = match &cli.lock {
        Some(path) => {
            let text = std::fs::read_to_string(path)
                .with_context(|| format!("reading --lock manifest {}", path.display()))?;
            let m = Manifest::from_json(&text)?;
            if m.schema_version != SCHEMA_VERSION {
                anyhow::bail!(
                    "--lock manifest {} has schema_version {}, but this gloam \
                     understands {}",
                    path.display(),
                    m.schema_version,
                    SCHEMA_VERSION
                );
            }
            Some(m.provenance)
        }
        None => None,
    };
    // One run-scoped source store: the fetch engine is constructed at most
    // once, and every source is resolved/read/verified at most once.
    let mut store = provenance::load::SourceStore::new(cli.use_fetch(), lock_pins);

    let diag = diag::Diag::new(cli.quiet);

    // `gloam lock`: write a provenance-only snapshot, no loader generation.
    if let Generator::Lock(lock_args) = &cli.generator {
        return write_lock_snapshot(&store, &command_line, lock_args, diag);
    }

    diag.info("resolving feature sets...");

    let feature_sets = resolve::build_feature_sets(&cli, &store, diag)?;

    let out = std::path::Path::new(&cli.out_path);
    std::fs::create_dir_all(out)?;

    // Without an explicit --lock, generation is an implicit lock-then-generate:
    // snapshot every source this tree needs (XML source keys plus the
    // auxiliary-header closure), carry forward commit/describe from the tree's
    // existing .gloam/manifest.json for repos whose pinned content is
    // unchanged, and generate from that settled pin set.  Settling the pins
    // once, before any output is written, keeps every preamble and the
    // manifest agreeing on one commit per repo; upstream commits that don't
    // touch any contributing file leave the whole tree byte-identical.
    let implicit_pins: Option<IndexMap<String, ProvenancePin>> = match &cli.generator {
        Generator::C(c_args) if cli.lock.is_none() => Some(implicit_lock_pins(
            &feature_sets,
            c_args.external_headers,
            out,
            &store,
            diag,
        )?),
        _ => None,
    };
    // Settle the pins for generation: the store re-reads pins from the new
    // lock set but keeps its content memo (blobs are content-addressed).
    if let Some(pins) = implicit_pins {
        store.set_lock(Some(pins));
    }

    // Aggregate provenance pins and the output BOM across all feature sets
    // written into this tree, for `.gloam/manifest.json`.
    let mut pins: IndexMap<String, ProvenancePin> = IndexMap::new();
    let mut files: IndexMap<String, OutputEntry> = IndexMap::new();

    match &cli.generator {
        Generator::C(c_args) => {
            diag.info("generating C loader...");
            for fs in &feature_sets {
                let tree = generator::c::generate(fs, c_args, out, &store, &command_line)?;
                pins.extend(tree.pins);
                for f in tree.files {
                    files.entry(f.path.clone()).or_insert(f);
                }
            }
        }
        Generator::Lock(_) => unreachable!("handled above"),
    }

    write_manifest(out, &command_line, pins, files)?;

    diag.info("done.");

    Ok(())
}

/// Reconstruct the effective command line for the preamble comment in
/// generated files.  argv[0] becomes just "gloam", and `--lock <manifest>` /
/// `--lock=<manifest>` are dropped: locking only pins provenance, so a locked
/// reproduction with otherwise identical args produces byte-identical output
/// to the original.
fn reconstruct_command_line(argv: &[String]) -> String {
    let mut args: Vec<String> = Vec::with_capacity(argv.len());
    let mut i = 0;
    while i < argv.len() {
        let a = &argv[i];
        if a == "--lock" {
            i += 2; // skip the flag and its value
            continue;
        }
        if a.starts_with("--lock=") {
            i += 1;
            continue;
        }
        args.push(if i == 0 {
            "gloam".to_string()
        } else {
            a.clone()
        });
        i += 1;
    }
    args.join(" ")
}

/// gloam self-metadata for a manifest.
fn gloam_meta(command_line: &str) -> GloamMeta {
    GloamMeta {
        version: build_info::PKG_VERSION.to_string(),
        describe: build_info::VERSION.to_string(),
        commit: build_info::GIT_SHA.unwrap_or("").to_string(),
        command_line: command_line.to_string(),
    }
}

/// `gloam lock`: resolve provenance for every supported source and write a
/// provenance-only snapshot manifest (no output BOM).
fn write_lock_snapshot(
    store: &provenance::load::SourceStore,
    command_line: &str,
    args: &cli::LockArgs,
    diag: diag::Diag,
) -> Result<()> {
    diag.info("snapshotting provenance...");
    let keys = provenance::all_keys();
    let mut pins: IndexMap<String, ProvenancePin> = store
        .resolve(&keys)?
        .into_iter()
        .map(|(key, src)| (key, src.pin))
        .collect();
    pins.sort_keys();

    let path = std::path::Path::new(&args.out);

    // Carry forward commit/describe from an existing snapshot at --out for
    // every repo whose pinned content is unchanged.  An upstream commit that
    // doesn't touch any pinned file then leaves the manifest — and everything
    // regenerated from it — byte-identical.  Deleting the file forces a full
    // re-snapshot.
    if let Some(prev) = read_snapshot(path) {
        for repo in provenance::manifest::preserve_unchanged_repos(&mut pins, &prev.provenance) {
            diag.info(format!(
                "{repo}: pinned content unchanged, keeping previous commit"
            ));
        }
    }

    let manifest = Manifest {
        schema_version: SCHEMA_VERSION,
        gloam: gloam_meta(command_line),
        provenance: pins,
        output: Vec::new(),
    };

    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, manifest.to_json_pretty() + "\n")?;
    diag.info(format!("wrote {}", path.display()));
    Ok(())
}

/// Build the implicit lock pin set for a generation run without `--lock`:
/// resolve every source the feature sets need (XML source keys plus the
/// auxiliary-header closure), then carry forward commit/describe from the
/// output tree's existing `.gloam/manifest.json` for every repo whose pinned
/// content is unchanged.  Unlike an explicit `--lock`, the previous manifest
/// is a best-effort baseline, not a contract: keys it lacks resolve fresh
/// (advancing their whole repo) instead of being refused.
fn implicit_lock_pins(
    feature_sets: &[resolve::FeatureSet],
    external_headers: bool,
    out: &std::path::Path,
    store: &provenance::load::SourceStore,
    diag: diag::Diag,
) -> Result<IndexMap<String, ProvenancePin>> {
    let mut keys: Vec<String> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for fs in feature_sets {
        let aux = generator::c::aux_headers(fs, store, external_headers)?;
        for key in fs
            .source_keys
            .iter()
            .cloned()
            .chain(aux.into_iter().map(|(key, _)| key))
        {
            if seen.insert(key.clone()) {
                keys.push(key);
            }
        }
    }

    let key_refs: Vec<&str> = keys.iter().map(String::as_str).collect();
    let mut pins: IndexMap<String, ProvenancePin> = store
        .resolve(&key_refs)?
        .into_iter()
        .map(|(key, src)| (key, src.pin))
        .collect();

    if let Some(prev) = read_snapshot(&out.join(".gloam").join("manifest.json")) {
        for repo in provenance::manifest::preserve_unchanged_repos(&mut pins, &prev.provenance) {
            diag.info(format!(
                "{repo}: pinned content unchanged, keeping previous commit"
            ));
        }
    }
    Ok(pins)
}

/// Best-effort read of an existing snapshot manifest.  Missing, unreadable, or
/// schema-mismatched files are ignored — the snapshot is simply taken fresh.
fn read_snapshot(path: &std::path::Path) -> Option<Manifest> {
    let text = std::fs::read_to_string(path).ok()?;
    let m = Manifest::from_json(&text).ok()?;
    (m.schema_version == SCHEMA_VERSION).then_some(m)
}

/// Write `.gloam/manifest.json` — the deterministic, pretty-printed bill of
/// materials for the output tree.  No timestamps: identical inputs + gloam
/// version produce a byte-identical manifest.
fn write_manifest(
    out: &std::path::Path,
    command_line: &str,
    mut pins: IndexMap<String, ProvenancePin>,
    mut files: IndexMap<String, OutputEntry>,
) -> Result<()> {
    pins.sort_keys();
    files.sort_keys();

    let manifest = Manifest {
        schema_version: SCHEMA_VERSION,
        gloam: gloam_meta(command_line),
        provenance: pins,
        output: files.into_values().collect(),
    };

    let dir = out.join(".gloam");
    std::fs::create_dir_all(&dir)?;
    std::fs::write(dir.join("manifest.json"), manifest.to_json_pretty() + "\n")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::reconstruct_command_line;

    fn argv(args: &[&str]) -> Vec<String> {
        args.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn command_line_replaces_argv0() {
        let line = reconstruct_command_line(&argv(&[
            r"C:\somewhere\gloam.exe",
            "--api",
            "gl:core=3.3",
            "c",
        ]));
        assert_eq!(line, "gloam --api gl:core=3.3 c");
    }

    #[test]
    fn command_line_drops_lock_flag_and_value() {
        let line = reconstruct_command_line(&argv(&[
            "gloam",
            "--api",
            "vk=1.0",
            "--lock",
            "manifest.json",
            "c",
        ]));
        assert_eq!(line, "gloam --api vk=1.0 c");
    }

    #[test]
    fn command_line_drops_lock_equals_form() {
        let line = reconstruct_command_line(&argv(&[
            "gloam",
            "--lock=manifest.json",
            "--api",
            "vk=1.0",
            "c",
        ]));
        assert_eq!(line, "gloam --api vk=1.0 c");
    }

    #[test]
    fn command_line_lock_at_end_without_value() {
        // Malformed but must not panic: `--lock` as the final token skips
        // past the end cleanly.
        let line = reconstruct_command_line(&argv(&["gloam", "c", "--lock"]));
        assert_eq!(line, "gloam c");
    }
}
