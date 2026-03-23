mod build_info;
mod bundled;
mod cli;
mod fetch;
mod generator;
mod ir;
mod parse;
mod preamble;
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

    match &cli.generator {
        Generator::C(c_args) => {
            if !cli.quiet {
                eprintln!("gloam: generating C loader...");
            }
            for fs in &feature_sets {
                generator::c::generate(fs, c_args, out, cli.use_fetch(), &command_line)?;
            }
        }
    }

    if !cli.quiet {
        eprintln!("gloam: done.");
    }

    Ok(())
}
