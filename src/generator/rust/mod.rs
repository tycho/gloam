//! Rust loader generator — renders a minijinja template against a `FeatureSet`.
//!
//! All generation logic lives in `src/gen/rust/templates/mod.rs.j2`.
//! This module only handles environment setup, filter registration, and file I/O.

use std::path::Path;

use anyhow::Result;
use minijinja::{Environment, Value, context};

use crate::cli::RustArgs;
use crate::resolve::FeatureSet;

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

pub fn generate(fs: &FeatureSet, _args: &RustArgs, out: &Path) -> Result<()> {
    let stem = output_stem(fs);
    let env = build_env()?;

    let ctx = context! {
        fs   => fs,
        stem => &stem,
    };

    std::fs::write(
        out.join(format!("{stem}.rs")),
        env.get_template("mod.rs.j2")?.render(&ctx)?,
    )?;

    Ok(())
}

fn output_stem(fs: &FeatureSet) -> String {
    if fs.is_merged {
        format!("gloam_{}", fs.apis.join("_"))
    } else {
        format!("gloam_{}", fs.apis[0])
    }
}

// ---------------------------------------------------------------------------
// Environment
// ---------------------------------------------------------------------------

fn build_env() -> Result<Environment<'static>> {
    let mut env = Environment::new();

    env.add_template("mod.rs.j2", include_str!("templates/mod.rs.j2"))?;

    env.add_filter("rjust", filter_rjust);
    env.add_filter("rust_value", filter_rust_value);
    env.add_filter("upper", filter_upper);

    Ok(env)
}

// ---------------------------------------------------------------------------
// Custom filters
// ---------------------------------------------------------------------------

/// Right-justify a value to `width` characters with space padding on the left.
fn filter_rjust(value: Value, width: usize) -> String {
    let s = value.to_string();
    format!("{s:>width$}")
}

/// Convert a C constant value string to a Rust literal.
/// Strips ULL/LL suffixes, preserves hex and negative values.
fn filter_rust_value(value: Value) -> String {
    let v = value.as_str().unwrap_or("0");
    // Strip integer suffix
    let v = v
        .trim_end_matches("ULL")
        .trim_end_matches("LL")
        .trim_end_matches('u')
        .trim_end_matches('U');
    v.to_string()
}

/// Uppercase a string value — used for extension short_name → constant name.
fn filter_upper(value: Value) -> String {
    value.as_str().unwrap_or("").to_uppercase()
}
