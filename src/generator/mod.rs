//! Code generation.  Each sub-module turns a resolved `FeatureSet` into output
//! files: `c` renders minijinja templates into a C loader; `rust` emits a Rust
//! loader directly.

pub mod c;
pub mod rust;

use indexmap::IndexMap;

use crate::provenance::manifest::{OutputEntry, ProvenancePin};

/// What one backend's `generate()` call produced: the provenance pins for every
/// source it used and an output-BOM entry for every file it wrote.  The run
/// loop merges these across feature sets into `.gloam/manifest.json`.
pub struct GeneratedTree {
    pub pins: IndexMap<String, ProvenancePin>,
    pub files: Vec<OutputEntry>,
}
