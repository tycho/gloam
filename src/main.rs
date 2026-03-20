mod bundled;
mod cli;
mod fetch;
mod generator;
mod ir;
mod parse;
mod resolve;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, Generator};

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {:#}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    if !cli.quiet {
        eprintln!("gloam: resolving feature sets...");
    }

    let feature_sets = resolve::build_feature_sets(&cli)?;

    let out = std::path::Path::new(&cli.out_path);
    std::fs::create_dir_all(out)?;

    match &cli.generator {
        Generator::C(c_args) => {
            if !cli.quiet {
                eprintln!("gloam: generating C loader...");
            }
            for fs in &feature_sets {
                generator::c::generate(fs, c_args, out, cli.fetch)?;
            }
        }
        Generator::Rust(r_args) => {
            if !cli.quiet {
                eprintln!("gloam: generating Rust loader...");
            }
            for fs in &feature_sets {
                generator::rust::generate(fs, r_args, out)?;
            }
        }
    }

    if !cli.quiet {
        eprintln!("gloam: done.");
    }

    Ok(())
}
