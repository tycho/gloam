//! Command building, PFN ordering optimization, and alias pairs.
//!
//! Transforms raw command data from the spec into indexed `Command` entries,
//! with optimized ordering to minimize PFN range table fragmentation.

use std::collections::HashMap;

use crate::cli::ApiRequest;
use crate::ir::{RawCommand, RawSpec};

use super::selection::{SelectedExt, SelectedFeature, api_profile_matches};
use super::types::{AliasPair, Command, Param};

// ---------------------------------------------------------------------------
// Build a single Command entry
// ---------------------------------------------------------------------------

pub(super) fn build_command(
    index: u16,
    raw: &RawCommand,
    scope: &str,
    protect: Option<String>,
    pfn_prefix: &str,
    name_prefix: &str,
) -> Command {
    let short_name = raw
        .name
        .strip_prefix(name_prefix)
        .unwrap_or(&raw.name)
        .to_string();

    let pfn_type = if pfn_prefix == "PFN_" {
        // Vulkan: PFN_vkFoo
        format!("PFN_{}", raw.name)
    } else {
        // GL family: PFNGLFOOPROC — strip the lowercase api prefix (e.g. "gl")
        // before uppercasing so we don't get PFNGLGLFOOPROC.
        let stem = raw.name.strip_prefix(name_prefix).unwrap_or(&raw.name);
        format!("{}{}PROC", pfn_prefix, stem.to_uppercase())
    };

    let params: Vec<Param> = raw
        .params
        .iter()
        .map(|p| Param {
            type_raw: p.type_raw.clone(),
            name: p.name.clone(),
        })
        .collect();

    let params_str = if params.is_empty() {
        "void".to_string()
    } else {
        params
            .iter()
            .map(|p| {
                if p.name.is_empty() {
                    p.type_raw.clone()
                } else if p.type_raw.trim_end().ends_with(']') {
                    // Array param: type_raw already contains the name and
                    // array suffix, e.g. "float blendConstants[4]".
                    // Emit verbatim — don't append the name again.
                    p.type_raw.trim().to_string()
                } else {
                    format!("{} {}", p.type_raw, p.name)
                }
            })
            .collect::<Vec<_>>()
            .join(", ")
    };

    Command {
        index,
        name: raw.name.clone(),
        short_name,
        pfn_type,
        return_type: raw.return_type.clone(),
        params_str,
        params,
        scope: scope.to_string(),
        protect,
        name_offset: 0, // computed after command list is finalized
    }
}

// ---------------------------------------------------------------------------
// Command → platform protect mapping
// ---------------------------------------------------------------------------

/// Build a map from command name → platform protection macro, derived from
/// extensions.  A single pass over all extensions replaces the previous
/// per-command linear scan (O(cmds × exts × requires) → O(exts × requires)).
pub(super) fn build_command_protect_map<'a>(exts: &[SelectedExt<'a>]) -> HashMap<&'a str, String> {
    let mut map = HashMap::new();
    for ext in exts {
        if let Some(protect) = ext.raw.protect.first() {
            for require in &ext.raw.requires {
                for cmd in &require.commands {
                    map.entry(cmd.as_str()).or_insert_with(|| protect.clone());
                }
            }
        }
    }
    map
}

// ---------------------------------------------------------------------------
// PFN ordering optimization
// ---------------------------------------------------------------------------

/// Reorder command names to minimize PFN range table fragmentation.
///
/// The pfnArray index of each command determines how many `PfnRange` entries
/// are needed in the range tables.  When commands required by the same
/// feature or extension are scattered across the array, each disjoint group
/// becomes a separate range.  By placing commands with identical consumer
/// sets adjacent, most consumers collapse to a single contiguous range.
///
/// **Algorithm**: assign each command a "consumer signature" — the sorted
/// set of feature/extension indices that include it.  Sort commands by
/// signature (lexicographic on the index lists).  This groups commands with
/// identical consumers together, and orders the groups so that consumers
/// with overlapping command sets are near each other.
///
/// **Effect**: in a merged GL 4.6 + GLES 3.2 build, GLES 2.0 drops from
/// ~34 ranges to ~1–3 because its cherry-picked subset of GL 1.0 commands
/// are now contiguous rather than interleaved with GL-only commands.
pub(super) fn optimize_command_order(
    core_cmds: &[String],
    ext_cmds: &[String],
    selected_features: &[SelectedFeature<'_>],
    selected_exts: &[SelectedExt<'_>],
    requests: &[ApiRequest],
) -> (Vec<String>, Vec<String>) {
    let num_features = selected_features.len();

    // Build command → sorted consumer-index set.
    // Consumers 0..num_features are features, num_features.. are extensions.
    let mut consumers: HashMap<&str, Vec<u32>> = HashMap::new();

    // Feature consumers — respect API/profile filtering.
    for (fi, feat) in selected_features.iter().enumerate() {
        let profile = requests
            .iter()
            .find(|r| r.name == feat.api)
            .and_then(|r| r.profile.as_deref());

        for require in &feat.raw.requires {
            if !api_profile_matches(
                require.api.as_deref(),
                require.profile.as_deref(),
                &feat.api,
                profile,
            ) {
                continue;
            }
            for cmd in &require.commands {
                consumers.entry(cmd.as_str()).or_default().push(fi as u32);
            }
        }
    }

    // Extension consumers.
    for (ei, ext) in selected_exts.iter().enumerate() {
        for require in &ext.raw.requires {
            for cmd in &require.commands {
                consumers
                    .entry(cmd.as_str())
                    .or_default()
                    .push((num_features + ei) as u32);
            }
        }
    }

    // Deduplicate and sort each consumer list so the signature is canonical.
    for list in consumers.values_mut() {
        list.sort_unstable();
        list.dedup();
    }

    // Sort each command list by consumer signature (lexicographic).
    let sort_by_consumers = |names: &[String]| -> Vec<String> {
        let mut sorted = names.to_vec();
        sorted.sort_by(|a, b| {
            let ca = consumers.get(a.as_str()).map(Vec::as_slice).unwrap_or(&[]);
            let cb = consumers.get(b.as_str()).map(Vec::as_slice).unwrap_or(&[]);
            ca.cmp(cb).then_with(|| a.cmp(b)) // tie-break alphabetically for stability
        });
        sorted
    };

    (sort_by_consumers(core_cmds), sort_by_consumers(ext_cmds))
}

// ---------------------------------------------------------------------------
// Alias pairs
// ---------------------------------------------------------------------------

pub(super) fn build_alias_pairs(raw: &RawSpec, commands: &[Command]) -> Vec<AliasPair> {
    // Build name -> index map for quick lookup.
    let idx: HashMap<&str, u16> = commands
        .iter()
        .map(|c| (c.name.as_str(), c.index))
        .collect();

    // Group by canonical (shortest name).
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();
    for (name, cmd) in &raw.commands {
        if let Some(ref alias) = cmd.alias {
            // Both must be in the selected command set.
            if !idx.contains_key(name.as_str()) || !idx.contains_key(alias.as_str()) {
                continue;
            }
            // Canonical = shortest name; if equal, alphabetical.
            let (canonical, secondary) = if alias.len() < name.len()
                || (alias.len() == name.len() && alias.as_str() < name.as_str())
            {
                (alias.clone(), name.clone())
            } else {
                (name.clone(), alias.clone())
            };
            groups.entry(canonical).or_default().push(secondary);
        }
    }

    let mut pairs: Vec<AliasPair> = Vec::new();
    for (canonical, secondaries) in groups {
        let Some(&ci) = idx.get(canonical.as_str()) else {
            continue;
        };
        for secondary in secondaries {
            let Some(&si) = idx.get(secondary.as_str()) else {
                continue;
            };
            pairs.push(AliasPair {
                canonical: ci,
                secondary: si,
            });
        }
    }

    // Sort by canonical index (the load loop depends on consecutive ordering).
    pairs.sort_by_key(|p| (p.canonical, p.secondary));
    pairs
}
