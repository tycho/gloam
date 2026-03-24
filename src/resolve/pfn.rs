//! PFN range table construction.
//!
//! Builds the feature and extension PFN range tables that the generated
//! loader uses to map feature/extension indices to contiguous command
//! ranges in the pfnArray.

use std::collections::HashMap;

use crate::cli::canonical_api_name;

use super::selection::{SelectedExt, SelectedFeature, api_profile_matches};
use super::types::{Command, Feature, PfnRange};

// ---------------------------------------------------------------------------
// Feature PFN ranges
// ---------------------------------------------------------------------------

pub(super) fn build_feature_pfn_ranges(
    features: &[SelectedFeature<'_>],
    feat_entries: &[Feature],
    commands: &[Command],
) -> Vec<PfnRange> {
    debug_assert_eq!(
        features.len(),
        feat_entries.len(),
        "SelectedFeature and Feature slices must be built in the same order"
    );

    // Build a map: command name → pfnArray index.
    let cmd_index: HashMap<&str, u16> = commands
        .iter()
        .map(|c| (c.name.as_str(), c.index))
        .collect();

    let mut ranges: Vec<PfnRange> = Vec::new();

    // features[] and feat_entries[] are built from the same source in the same
    // order, so we zip rather than doing O(n) string searches per feature.
    for (sf, feat) in features.iter().zip(feat_entries.iter()) {
        debug_assert_eq!(sf.raw.name, feat.full_name);

        let mut cmd_indices: Vec<u16> = Vec::new();
        for require in &sf.raw.requires {
            for cmd_name in &require.commands {
                if let Some(&idx) = cmd_index.get(cmd_name.as_str()) {
                    cmd_indices.push(idx);
                }
            }
        }
        cmd_indices.sort_unstable();
        cmd_indices.dedup();

        ranges.extend(indices_to_ranges(feat.index, &cmd_indices));
    }

    ranges
}

// ---------------------------------------------------------------------------
// Extension PFN ranges
// ---------------------------------------------------------------------------

pub(super) fn build_ext_pfn_ranges(
    api: &str,
    exts: &[SelectedExt<'_>],
    ext_index_map: &HashMap<&str, u16>,
    commands: &[Command],
) -> (Vec<PfnRange>, Vec<u16>) {
    let cmd_index: HashMap<&str, u16> = commands
        .iter()
        .map(|c| (c.name.as_str(), c.index))
        .collect();

    let mut ranges: Vec<PfnRange> = Vec::new();
    let mut subset_indices: Vec<u16> = Vec::new();

    // Collect extensions relevant to this API.
    let relevant_exts: Vec<(usize, &SelectedExt)> = exts
        .iter()
        .enumerate()
        .filter(|(_, e)| {
            e.raw
                .supported
                .iter()
                .any(|s| canonical_api_name(s) == canonical_api_name(api))
        })
        .collect();

    for (_orig_idx, ext) in &relevant_exts {
        let sorted_ext_idx = match ext_index_map.get(ext.raw.name.as_str()) {
            Some(&i) => i,
            None => continue,
        };

        subset_indices.push(sorted_ext_idx);

        // Commands belonging to this extension for this API.
        let mut cmd_indices: Vec<u16> = Vec::new();
        for require in &ext.raw.requires {
            if !api_profile_matches(require.api.as_deref(), None, api, None) {
                continue;
            }
            for cmd_name in &require.commands {
                if let Some(&pfn_idx) = cmd_index.get(cmd_name.as_str()) {
                    cmd_indices.push(pfn_idx);
                }
            }
        }
        cmd_indices.sort_unstable();
        cmd_indices.dedup();

        ranges.extend(indices_to_ranges(sorted_ext_idx, &cmd_indices));
    }

    subset_indices.sort_unstable();
    (ranges, subset_indices)
}

// ---------------------------------------------------------------------------
// indices_to_ranges
// ---------------------------------------------------------------------------

/// Convert a sorted list of pfnArray indices belonging to the same feature/ext
/// into one or more PfnRange entries (one per contiguous run).
pub(super) fn indices_to_ranges(ext_idx: u16, sorted: &[u16]) -> Vec<PfnRange> {
    if sorted.is_empty() {
        return Vec::new();
    }
    let mut ranges = Vec::new();
    let mut start = sorted[0];
    let mut count = 1u16;

    for &idx in &sorted[1..] {
        if idx == start + count {
            count += 1;
        } else {
            ranges.push(PfnRange {
                extension: ext_idx,
                start,
                count,
            });
            start = idx;
            count = 1;
        }
    }
    ranges.push(PfnRange {
        extension: ext_idx,
        start,
        count,
    });
    ranges
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indices_to_ranges_empty() {
        assert!(indices_to_ranges(0, &[]).is_empty());
    }

    #[test]
    fn indices_to_ranges_single_element() {
        let r = indices_to_ranges(5, &[42]);
        assert_eq!(
            r,
            vec![PfnRange {
                extension: 5,
                start: 42,
                count: 1
            }]
        );
    }

    #[test]
    fn indices_to_ranges_fully_contiguous() {
        let r = indices_to_ranges(0, &[10, 11, 12, 13, 14]);
        assert_eq!(
            r,
            vec![PfnRange {
                extension: 0,
                start: 10,
                count: 5
            }]
        );
    }

    #[test]
    fn indices_to_ranges_single_gap() {
        let r = indices_to_ranges(1, &[3, 4, 5, 10, 11]);
        assert_eq!(
            r,
            vec![
                PfnRange {
                    extension: 1,
                    start: 3,
                    count: 3
                },
                PfnRange {
                    extension: 1,
                    start: 10,
                    count: 2
                },
            ]
        );
    }

    #[test]
    fn indices_to_ranges_all_disjoint() {
        let r = indices_to_ranges(2, &[0, 5, 10]);
        assert_eq!(
            r,
            vec![
                PfnRange {
                    extension: 2,
                    start: 0,
                    count: 1
                },
                PfnRange {
                    extension: 2,
                    start: 5,
                    count: 1
                },
                PfnRange {
                    extension: 2,
                    start: 10,
                    count: 1
                },
            ]
        );
    }

    #[test]
    fn indices_to_ranges_multiple_gaps() {
        let r = indices_to_ranges(0, &[1, 2, 5, 6, 7, 20]);
        assert_eq!(
            r,
            vec![
                PfnRange {
                    extension: 0,
                    start: 1,
                    count: 2
                },
                PfnRange {
                    extension: 0,
                    start: 5,
                    count: 3
                },
                PfnRange {
                    extension: 0,
                    start: 20,
                    count: 1
                },
            ]
        );
    }
}
