//! Feature and extension selection.
//!
//! Phase 1 of the resolution pipeline: determine which features (versions)
//! and extensions are "in" for the current build, based on CLI arguments,
//! dependency analysis, promotion checks, and baseline exclusion.

use std::collections::{HashMap, HashSet};

use crate::cli::{ApiRequest, ExtensionFilter, canonical_api_name};
use crate::ir::RawSpec;

use super::spec_info::{ResolveConfig, api_order, build_api_set};
use super::types::SelectionReason;

// ---------------------------------------------------------------------------
// Internal types
// ---------------------------------------------------------------------------

pub(super) struct SelectedFeature<'a> {
    pub api: String,
    pub raw: &'a crate::ir::RawFeature,
}

pub(super) struct SelectedExt<'a> {
    pub raw: &'a crate::ir::RawExtension,
    pub reason: SelectionReason,
}

/// Result of select_extensions: the selected extensions plus exclusion info.
pub(super) struct ExtensionSelection<'a> {
    pub selected: Vec<SelectedExt<'a>>,
    /// Extensions excluded via `-` prefix in --extensions.
    pub excluded_explicit: Vec<String>,
    /// Extensions excluded because fully promoted into --baseline versions.
    pub excluded_baseline: Vec<String>,
}

// ---------------------------------------------------------------------------
// Feature selection
// ---------------------------------------------------------------------------

pub(super) fn select_features<'a>(
    raw: &'a RawSpec,
    requests: &[ApiRequest],
) -> Vec<SelectedFeature<'a>> {
    let mut selected = Vec::new();
    for req in requests {
        let max_ver = req.version.clone();
        for feat in &raw.features {
            if canonical_api_name(&feat.api) != canonical_api_name(&req.name) {
                continue;
            }
            if let Some(ref mv) = max_ver
                && feat.version > *mv
            {
                continue;
            }
            selected.push(SelectedFeature {
                api: req.name.clone(),
                raw: feat,
            });
        }
    }
    // Sort: GL versions first, then GLES, matching the spec's ordering rule.
    selected.sort_by(|a, b| {
        api_order(&a.api)
            .cmp(&api_order(&b.api))
            .then_with(|| a.raw.version.cmp(&b.raw.version))
    });
    selected
}

// ---------------------------------------------------------------------------
// Extension selection
// ---------------------------------------------------------------------------

pub(super) fn select_extensions<'a>(
    raw: &'a RawSpec,
    requests: &[ApiRequest],
    config: &ResolveConfig<'_>,
    spec_name: &str,
    per_api_core_cmds: &HashMap<String, HashSet<String>>,
) -> ExtensionSelection<'a> {
    let filter = config.ext_filter;
    let baseline = config.baseline;
    let want_promoted = config.want_promoted;
    let want_predecessors = config.want_predecessors;

    let api_set = build_api_set(requests);

    // WGL mandatory extensions (spec gotcha #9).
    let wgl_mandatory: HashSet<&str> = if spec_name == "wgl" {
        ["WGL_ARB_extensions_string", "WGL_EXT_extensions_string"]
            .iter()
            .copied()
            .collect()
    } else {
        HashSet::new()
    };

    let mut selected: Vec<SelectedExt<'a>> = raw
        .extensions
        .iter()
        .filter_map(|e| {
            let supported = e.supported.iter().any(|s| api_set.contains(s.as_str()));
            if !supported {
                return None;
            }
            if wgl_mandatory.contains(e.name.as_str()) {
                return Some(SelectedExt {
                    raw: e,
                    reason: SelectionReason::Mandatory,
                });
            }
            match &filter.include {
                None => Some(SelectedExt {
                    raw: e,
                    reason: SelectionReason::AllExtensions,
                }),
                Some(list) => {
                    if list.contains(&e.name) {
                        Some(SelectedExt {
                            raw: e,
                            reason: SelectionReason::Explicit,
                        })
                    } else {
                        None
                    }
                }
            }
        })
        .collect();

    // Build an extension name → index lookup for dependency resolution.
    let ext_by_name: HashMap<&str, usize> = raw
        .extensions
        .iter()
        .enumerate()
        .map(|(i, e)| (e.name.as_str(), i))
        .collect();

    // Dependency-following pass: walk the `depends` field of each selected
    // extension and pull in any prerequisite extensions not already selected.
    // Fixed-point loop because dependencies can be transitive — pulling in
    // extension A may require extension B which requires extension C.
    // Runs before --promoted and --predecessors so that dependency-pulled
    // extensions' commands are visible to those passes.
    loop {
        let already: HashSet<&str> = selected.iter().map(|e| e.raw.name.as_str()).collect();
        let prev_len = selected.len();

        // Collect unique dependency names from all currently selected extensions.
        let needed: HashSet<&str> = selected
            .iter()
            .flat_map(|e| e.raw.depends.iter().map(String::as_str))
            .filter(|dep| {
                !already.contains(dep)
                    && ext_by_name.contains_key(dep)
                    // Only pull in extensions that support a requested API.
                    && raw.extensions[ext_by_name[dep]]
                        .supported
                        .iter()
                        .any(|s| api_set.contains(s.as_str()))
            })
            .collect();

        for dep_name in needed {
            if let Some(&idx) = ext_by_name.get(dep_name) {
                selected.push(SelectedExt {
                    raw: &raw.extensions[idx],
                    reason: SelectionReason::Dependency,
                });
            }
        }

        if selected.len() == prev_len {
            break;
        }
    }

    // Build the bidirectional alias maps once — they're used by both the
    // --promoted and --predecessors passes.
    let cmd_to_alias: HashMap<&str, &str> = if want_promoted || want_predecessors {
        let mut m = HashMap::new();
        for (name, cmd) in &raw.commands {
            if let Some(ref alias) = cmd.alias {
                m.insert(name.as_str(), alias.as_str());
                m.insert(alias.as_str(), name.as_str());
            }
        }
        m
    } else {
        HashMap::new()
    };
    let enum_to_alias: HashMap<&str, &str> = if want_predecessors {
        let mut m = HashMap::new();
        for (name, e) in &raw.flat_enums {
            if let Some(ref alias) = e.alias {
                m.insert(name.as_str(), alias.as_str());
                m.insert(alias.as_str(), name.as_str());
            }
        }
        m
    } else {
        HashMap::new()
    };

    if want_promoted {
        promoted_pass(
            raw,
            &api_set,
            per_api_core_cmds,
            &cmd_to_alias,
            &mut selected,
        );
    }

    if want_predecessors {
        predecessors_pass(raw, &api_set, &cmd_to_alias, &enum_to_alias, &mut selected);
    }

    // ------------------------------------------------------------------
    // Final exclusion pass
    // ------------------------------------------------------------------
    let (excluded_explicit, excluded_baseline) =
        apply_exclusions(raw, requests, filter, baseline, &mut selected);

    ExtensionSelection {
        selected,
        excluded_explicit,
        excluded_baseline,
    }
}

// ---------------------------------------------------------------------------
// --promoted pass
// ---------------------------------------------------------------------------

fn promoted_pass<'a>(
    raw: &'a RawSpec,
    api_set: &HashSet<&str>,
    per_api_core_cmds: &HashMap<String, HashSet<String>>,
    cmd_to_alias: &HashMap<&str, &str>,
    selected: &mut Vec<SelectedExt<'a>>,
) {
    // Snapshot names already selected so we don't duplicate them.
    let already: HashSet<&str> = selected.iter().map(|e| e.raw.name.as_str()).collect();

    for ext in &raw.extensions {
        if already.contains(ext.name.as_str()) {
            continue;
        }

        let is_promoted = ext
            .supported
            .iter()
            .filter(|s| api_set.contains(s.as_str()))
            .any(|api| {
                let Some(core_cmds) = per_api_core_cmds.get(canonical_api_name(api.as_str()))
                else {
                    return false;
                };
                ext.requires
                    .iter()
                    .filter(|req| api_profile_matches(req.api.as_deref(), None, api, None))
                    .any(|req| {
                        req.commands.iter().any(|c| {
                            core_cmds.contains(c.as_str())
                                || cmd_to_alias
                                    .get(c.as_str())
                                    .is_some_and(|a| core_cmds.contains(*a))
                        })
                    })
            });

        if is_promoted {
            selected.push(SelectedExt {
                raw: ext,
                reason: SelectionReason::Promoted,
            });
        }
    }
}

// ---------------------------------------------------------------------------
// --predecessors pass
// ---------------------------------------------------------------------------

fn predecessors_pass<'a>(
    raw: &'a RawSpec,
    api_set: &HashSet<&str>,
    cmd_to_alias: &HashMap<&str, &str>,
    enum_to_alias: &HashMap<&str, &str>,
    selected: &mut Vec<SelectedExt<'a>>,
) {
    let mut selected_ext_cmds: HashSet<&str> = selected
        .iter()
        .flat_map(|e| {
            e.raw
                .requires
                .iter()
                .flat_map(|req| req.commands.iter().map(String::as_str))
        })
        .collect();

    let mut selected_ext_enums: HashSet<&str> = selected
        .iter()
        .flat_map(|e| {
            e.raw
                .requires
                .iter()
                .flat_map(|req| req.enums.iter().map(String::as_str))
        })
        .collect();

    loop {
        let already: HashSet<&str> = selected.iter().map(|e| e.raw.name.as_str()).collect();
        let prev_len = selected.len();

        for ext in &raw.extensions {
            if already.contains(ext.name.as_str()) {
                continue;
            }
            let supported = ext.supported.iter().any(|s| api_set.contains(s.as_str()));
            if !supported {
                continue;
            }
            let is_predecessor = ext.requires.iter().any(|req| {
                req.commands.iter().any(|c| {
                    selected_ext_cmds.contains(c.as_str())
                        || cmd_to_alias
                            .get(c.as_str())
                            .is_some_and(|a| selected_ext_cmds.contains(*a))
                }) || req.enums.iter().any(|e| {
                    selected_ext_enums.contains(e.as_str())
                        || enum_to_alias
                            .get(e.as_str())
                            .is_some_and(|a| selected_ext_enums.contains(*a))
                })
            });
            if is_predecessor {
                selected.push(SelectedExt {
                    raw: ext,
                    reason: SelectionReason::Predecessor,
                });
            }
        }

        if selected.len() == prev_len {
            break;
        }

        // Incrementally add commands/enums from only the newly selected extensions.
        for ext in &selected[prev_len..] {
            for req in &ext.raw.requires {
                selected_ext_cmds.extend(req.commands.iter().map(String::as_str));
                selected_ext_enums.extend(req.enums.iter().map(String::as_str));
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Exclusion pass (explicit + baseline)
// ---------------------------------------------------------------------------

fn apply_exclusions(
    raw: &RawSpec,
    requests: &[ApiRequest],
    filter: &ExtensionFilter,
    baseline: &[ApiRequest],
    selected: &mut Vec<SelectedExt<'_>>,
) -> (Vec<String>, Vec<String>) {
    let explicit_excludes: HashSet<&str> = filter.exclude.iter().map(String::as_str).collect();
    let mut baseline_excludes: HashSet<String> = HashSet::new();

    // --baseline: compute extensions fully promoted into baseline versions.
    if !baseline.is_empty() {
        baseline_excludes = compute_baseline_excludes(raw, requests, baseline);
    }

    // Build unified exclude set and apply.
    let explicit_keeps: HashSet<&str> = {
        let mut keeps: HashSet<&str> = filter.keep.iter().map(String::as_str).collect();
        if let Some(ref list) = filter.include {
            keeps.extend(list.iter().map(String::as_str));
        }
        keeps
    };

    if !explicit_excludes.is_empty() || !baseline_excludes.is_empty() {
        selected.retain(|e| {
            let name = e.raw.name.as_str();
            if explicit_excludes.contains(name) {
                return false;
            }
            if baseline_excludes.contains(&e.raw.name) && !explicit_keeps.contains(name) {
                return false;
            }
            true
        });
    }

    // Remove kept extensions from baseline_excludes for accurate reporting.
    if !explicit_keeps.is_empty() {
        baseline_excludes.retain(|name| !explicit_keeps.contains(name.as_str()));
    }

    let excluded_explicit: Vec<String> = filter.exclude.iter().cloned().collect();
    let mut excluded_baseline: Vec<String> = baseline_excludes.into_iter().collect();
    excluded_baseline.sort();

    (excluded_explicit, excluded_baseline)
}

/// Compute the set of extension names that are fully promoted into the
/// baseline versions and should be excluded.
fn compute_baseline_excludes(
    raw: &RawSpec,
    requests: &[ApiRequest],
    baseline: &[ApiRequest],
) -> HashSet<String> {
    let baseline_features = select_features(raw, baseline);
    let mut baseline_core_cmds: HashMap<String, HashSet<String>> = HashMap::new();

    for feat in &baseline_features {
        let req_for_api = baseline
            .iter()
            .find(|r| canonical_api_name(&r.name) == canonical_api_name(&feat.api));
        let profile = req_for_api.and_then(|r| r.profile.as_deref());
        let api_cmds = baseline_core_cmds.entry(feat.api.clone()).or_default();

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
                api_cmds.insert(cmd.clone());
            }
        }
        for remove in &feat.raw.removes {
            if !profile_matches(remove.profile.as_deref(), profile) {
                continue;
            }
            for cmd in &remove.commands {
                api_cmds.remove(cmd.as_str());
            }
        }
    }

    // Build bidirectional alias map for baseline promotion checks.
    let baseline_cmd_aliases: HashMap<&str, &str> = {
        let mut m = HashMap::new();
        for (name, cmd) in &raw.commands {
            if let Some(ref alias) = cmd.alias {
                m.insert(name.as_str(), alias.as_str());
                m.insert(alias.as_str(), name.as_str());
            }
        }
        m
    };

    let request_api_set: HashSet<&str> = requests.iter().map(|r| r.name.as_str()).collect();
    let mut excludes: HashSet<String> = HashSet::new();

    for ext in &raw.extensions {
        let ext_build_apis: Vec<&str> = ext
            .supported
            .iter()
            .map(|s| canonical_api_name(s.as_str()))
            .filter(|s| request_api_set.contains(s))
            .collect();

        if ext_build_apis.is_empty() {
            continue;
        }

        let dominated = ext_build_apis.iter().all(|api| {
            let Some(core_cmds) = baseline_core_cmds.get(*api) else {
                return false;
            };
            let ext_cmds: Vec<&str> = ext
                .requires
                .iter()
                .filter(|req| api_profile_matches(req.api.as_deref(), None, api, None))
                .flat_map(|req| req.commands.iter().map(String::as_str))
                .collect();
            !ext_cmds.is_empty()
                && ext_cmds.iter().all(|c| {
                    core_cmds.contains(*c)
                        || baseline_cmd_aliases
                            .get(c)
                            .is_some_and(|a| core_cmds.contains(*a))
                })
        });

        if dominated {
            excludes.insert(ext.name.clone());
        }
    }

    excludes
}

// ---------------------------------------------------------------------------
// Profile / API matching helpers
// ---------------------------------------------------------------------------

pub(super) fn api_profile_matches(
    elem_api: Option<&str>,
    elem_profile: Option<&str>,
    target_api: &str,
    target_prof: Option<&str>,
) -> bool {
    if let Some(a) = elem_api
        && !a
            .split(',')
            .any(|x| canonical_api_name(x.trim()) == canonical_api_name(target_api))
    {
        return false;
    }
    profile_matches(elem_profile, target_prof)
}

pub(super) fn profile_matches(elem_profile: Option<&str>, target_profile: Option<&str>) -> bool {
    match (elem_profile, target_profile) {
        (None, _) => true,
        (Some(_), None) => true,
        (Some(ep), Some(tp)) => ep == tp,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ---- profile_matches ----

    #[test]
    fn profile_matches_both_none() {
        assert!(profile_matches(None, None));
    }

    #[test]
    fn profile_matches_element_none_always_matches() {
        assert!(profile_matches(None, Some("core")));
        assert!(profile_matches(None, Some("compat")));
    }

    #[test]
    fn profile_matches_target_none_always_matches() {
        assert!(profile_matches(Some("core"), None));
    }

    #[test]
    fn profile_matches_same() {
        assert!(profile_matches(Some("core"), Some("core")));
    }

    #[test]
    fn profile_matches_different() {
        assert!(!profile_matches(Some("core"), Some("compat")));
    }
}
