//! Rich `--version` output: the gloam version followed by the embedded bundle's
//! provenance, formatted like the generated-header sources block (minus the
//! comment asterisks).  Shares the repo grouping with the preamble so the two
//! never drift.

use std::sync::OnceLock;

use crate::build_info;
use crate::bundled;
use crate::provenance;

/// Long version text for clap's `long_version` (shown by `--version`).  Computed
/// once and returned as `&'static str`.  clap prefixes it with the binary name,
/// so this starts with the version string, not "gloam".
pub fn long_version() -> &'static str {
    static TEXT: OnceLock<String> = OnceLock::new();
    TEXT.get_or_init(build_long_version)
}

fn build_long_version() -> String {
    let mut out = String::from(build_info::VERSION);
    match bundled::bundled_provenance() {
        Ok(bundle) if !bundle.provenance.is_empty() => {
            out.push_str("\n\nEmbedded upstream sources:");
            for group in provenance::group_pins_by_repo(&bundle.provenance) {
                out.push_str(&format!(
                    "\n  {} ({})",
                    group.repo,
                    &group.commit[..7.min(group.commit.len())]
                ));
                for (path, blob) in &group.files {
                    out.push_str(&format!(
                        "\n    {} (blob {})",
                        path,
                        &blob[..7.min(blob.len())]
                    ));
                }
            }
        }
        _ => out.push_str("\n\n(no embedded provenance — run `cargo xtask bundle`)"),
    }
    out
}
