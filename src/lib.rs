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

use anyhow::Result;
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

    // Capture the effective command line for the preamble comment in
    // generated files.  Replace argv[0] with just "gloam" to avoid
    // embedding full filesystem paths.
    let command_line: String = {
        let mut args: Vec<String> = std::env::args().collect();
        if !args.is_empty() {
            args[0] = "gloam".to_string();
        }
        args.join(" ")
    };

    if !cli.quiet {
        eprintln!("gloam: resolving feature sets...");
    }

    let feature_sets = resolve::build_feature_sets(&cli)?;

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
                let tree =
                    generator::c::generate(fs, c_args, out, cli.use_fetch(), &command_line)?;
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
