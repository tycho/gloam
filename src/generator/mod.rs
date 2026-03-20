//! Code generation. Sub-modules render minijinja templates against a
//! resolved `FeatureSet` to produce C and Rust loader files.

pub mod c;
pub mod rust;
