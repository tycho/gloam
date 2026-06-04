//! gloam — loader generator for Vulkan, OpenGL, OpenGL ES, EGL, GLX, and WGL.
//!
//! The crate is split into a library (this file) and a thin binary
//! (`src/main.rs`).  The library form lets dev tooling — notably the
//! `cargo xtask` bundler — reuse gloam's acquisition and provenance code
//! instead of duplicating it.

mod build_info;
mod bundled;
mod cli;
mod fetch;
mod generator;
mod ir;
mod parse;
mod preamble;
pub mod provenance;
mod resolve;

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

    // Capture the effective command line for the preamble comment in generated
    // files.  Replace argv[0] with just "gloam", and drop `--lock <manifest>`:
    // locking only pins provenance, so a locked reproduction with otherwise
    // identical args produces byte-identical output to the original.
    let command_line: String = {
        let raw: Vec<String> = std::env::args().collect();
        let mut args: Vec<String> = Vec::with_capacity(raw.len());
        let mut i = 0;
        while i < raw.len() {
            let a = &raw[i];
            if a == "--lock" {
                i += 2; // skip the flag and its value
                continue;
            }
            if a.starts_with("--lock=") {
                i += 1;
                continue;
            }
            args.push(if i == 0 { "gloam".to_string() } else { a.clone() });
            i += 1;
        }
        args.join(" ")
    };

    // A --lock manifest pins upstream sources to recorded provenance.  Only its
    // `provenance` section is used; everything else is regenerated.
    let lock_pins: Option<IndexMap<String, ProvenancePin>> = match &cli.lock {
        Some(path) => {
            let text = std::fs::read_to_string(path)
                .with_context(|| format!("reading --lock manifest {}", path.display()))?;
            Some(Manifest::from_json(&text)?.provenance)
        }
        None => None,
    };
    let ctx = provenance::load::LoadCtx {
        use_fetch: cli.use_fetch(),
        lock: lock_pins.as_ref(),
    };

    if !cli.quiet {
        eprintln!("gloam: resolving feature sets...");
    }

    let feature_sets = resolve::build_feature_sets(&cli, &ctx)?;

    let out = std::path::Path::new(&cli.out_path);
    std::fs::create_dir_all(out)?;

    // Aggregate provenance pins and the output BOM across all feature sets
    // written into this tree, for `.gloam/manifest.json`.
    let mut pins: IndexMap<String, ProvenancePin> = IndexMap::new();
    let mut files: IndexMap<String, OutputEntry> = IndexMap::new();

    match &cli.generator {
        Generator::C(c_args) => {
            if !cli.quiet {
                eprintln!("gloam: generating C loader...");
            }
            for fs in &feature_sets {
                let tree = generator::c::generate(fs, c_args, out, &ctx, &command_line)?;
                pins.extend(tree.pins);
                for f in tree.files {
                    files.entry(f.path.clone()).or_insert(f);
                }
            }
        }
    }

    write_manifest(out, &command_line, pins, files)?;

    if !cli.quiet {
        eprintln!("gloam: done.");
    }

    Ok(())
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
        gloam: GloamMeta {
            version: build_info::PKG_VERSION.to_string(),
            describe: build_info::VERSION.to_string(),
            commit: build_info::GIT_SHA.unwrap_or("").to_string(),
            command_line: command_line.to_string(),
        },
        provenance: pins,
        output: files.into_values().collect(),
    };

    let dir = out.join(".gloam");
    std::fs::create_dir_all(&dir)?;
    std::fs::write(dir.join("manifest.json"), manifest.to_json_pretty() + "\n")?;
    Ok(())
}
