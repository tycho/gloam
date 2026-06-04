//! Serializable schemas for the embedded bundle provenance
//! (`bundled/provenance.json`) and the per-loader output manifest
//! (`.gloam/manifest.json`), plus the git-blob-SHA-1 helper used for the
//! output BOM.
//!
//! See `docs/manifest.md` (consumer) and `docs/provenance-internals.md`
//! (producer) for the authoritative format description.
//!
//! All maps are `IndexMap` populated in sorted order so serialized output is
//! deterministic; JSON is always written pretty-printed for legible diffs.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

/// Manifest/provenance schema version (bump on incompatible layout changes).
pub const SCHEMA_VERSION: u32 = 1;

/// Immutable provenance for one upstream file — a "pin".
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenancePin {
    pub repo: String,
    pub repo_url: String,
    pub path_in_repo: String,
    /// Full upstream commit SHA-1.
    pub commit: String,
    /// `git describe`-style version.
    pub describe: String,
    /// Git blob SHA-1 of the file content.
    pub blob: String,
}

/// `bundled/provenance.json` — the checked-in pin set for the embedded bundle.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BundledProvenance {
    pub schema_version: u32,
    /// Keyed by registry file key (sorted).
    pub provenance: IndexMap<String, ProvenancePin>,
}

/// gloam self-metadata for the output manifest.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GloamMeta {
    pub version: String,
    pub describe: String,
    pub commit: String,
    pub command_line: String,
}

/// One generated output file in the BOM.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputEntry {
    /// Path relative to the output root.
    pub path: String,
    /// Git blob SHA-1 of the generated file (`git hash-object`).
    pub blob: String,
    /// True when the file is an upstream file copied verbatim.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub verbatim: bool,
    /// Provenance keys that influenced this file.
    pub derived_from: Vec<String>,
}

/// `.gloam/manifest.json` — the per-output-tree manifest.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub schema_version: u32,
    pub gloam: GloamMeta,
    /// The pin set (sorted by key).  On `--lock` this is carried verbatim from
    /// the input manifest, so it may contain entries no output references.
    pub provenance: IndexMap<String, ProvenancePin>,
    /// Output BOM, sorted by path.
    pub output: Vec<OutputEntry>,
}

impl BundledProvenance {
    pub fn to_json_pretty(&self) -> String {
        // Infallible for these plain structs; pretty-printed for legible diffs.
        serde_json::to_string_pretty(self).expect("serialize BundledProvenance")
    }

    pub fn from_json(s: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}

impl Manifest {
    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).expect("serialize Manifest")
    }

    pub fn from_json(s: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}

/// Compute the git blob SHA-1 of `content` — identical to `git hash-object`,
/// so manifest hashes equal the file's blob hash in any git repo.
pub fn git_blob_sha1(content: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(format!("blob {}\0", content.len()).as_bytes());
    hasher.update(content);
    let digest = hasher.finalize();
    let mut hex = String::with_capacity(40);
    for byte in digest {
        hex.push_str(&format!("{byte:02x}"));
    }
    hex
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn git_blob_sha1_matches_git_hash_object() {
        // `printf 'hello\n' | git hash-object --stdin`
        assert_eq!(
            git_blob_sha1(b"hello\n"),
            "ce013625030ba8dba906f756967f9e9ca394464a"
        );
        // `printf '' | git hash-object --stdin` (the empty blob)
        assert_eq!(
            git_blob_sha1(b""),
            "e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"
        );
    }

    #[test]
    fn manifest_roundtrips_through_json() {
        let mut provenance = IndexMap::new();
        provenance.insert(
            "gl.xml".to_string(),
            ProvenancePin {
                repo: "KhronosGroup/OpenGL-Registry".to_string(),
                repo_url: "https://github.com/KhronosGroup/OpenGL-Registry".to_string(),
                path_in_repo: "xml/gl.xml".to_string(),
                commit: "a1b2c3d4".to_string(),
                describe: "a1b2c3d".to_string(),
                blob: "0fa1e2d3".to_string(),
            },
        );
        let m = Manifest {
            schema_version: SCHEMA_VERSION,
            gloam: GloamMeta {
                version: "0.4.9".to_string(),
                describe: "v0.4.9-3-g8498f7e".to_string(),
                commit: "8498f7ec".to_string(),
                command_line: "gloam --api gl:core=3.3 c --loader".to_string(),
            },
            provenance,
            output: vec![OutputEntry {
                path: "include/gloam/gl.h".to_string(),
                blob: "deadbeef".to_string(),
                verbatim: false,
                derived_from: vec!["gl.xml".to_string()],
            }],
        };
        let json = m.to_json_pretty();
        let back = Manifest::from_json(&json).unwrap();
        assert_eq!(back.schema_version, SCHEMA_VERSION);
        assert_eq!(back.gloam, m.gloam);
        assert_eq!(back.provenance, m.provenance);
        assert_eq!(back.output, m.output);
    }

    #[test]
    fn output_entry_omits_false_verbatim() {
        let e = OutputEntry {
            path: "src/gl.c".to_string(),
            blob: "abc".to_string(),
            verbatim: false,
            derived_from: vec![],
        };
        let json = serde_json::to_string(&e).unwrap();
        assert!(!json.contains("verbatim"), "false verbatim should be omitted");

        let e2 = OutputEntry { verbatim: true, ..e };
        let json2 = serde_json::to_string(&e2).unwrap();
        assert!(json2.contains("\"verbatim\":true"));
    }
}
