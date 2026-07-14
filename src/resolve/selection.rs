//! Feature and extension selection.
//!
//! Phase 1 of the resolution pipeline: determine which features (versions)
//! and extensions are "in" for the current build, based on CLI arguments,
//! dependency analysis, promotion checks, and baseline exclusion.

use std::collections::{HashMap, HashSet};

use crate::cli::{ApiRequest, ExtensionFilter};
use crate::identity::{Api, Spec, canonical_api_name};
use crate::ir::RawSpec;

use super::requirements::per_api_core_commands;
use super::spec_info::{ResolveConfig, build_api_set};
use super::types::SelectionReason;

// ---------------------------------------------------------------------------
// Internal types
// ---------------------------------------------------------------------------

pub(super) struct SelectedFeature<'a> {
    pub api: Api,
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
            if canonical_api_name(&feat.api) != req.api.as_str() {
                continue;
            }
            if let Some(ref mv) = max_ver
                && feat.version > *mv
            {
                continue;
            }
            selected.push(SelectedFeature {
                api: req.api,
                raw: feat,
            });
        }
    }
    // Sort: GL versions first, then GLES, matching the spec's ordering rule.
    selected.sort_by(|a, b| {
        a.api
            .sort_order()
            .cmp(&b.api.sort_order())
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
    spec: Spec,
    per_api_core_cmds: &HashMap<String, HashSet<String>>,
) -> ExtensionSelection<'a> {
    let filter = config.ext_filter;
    let baseline = config.baseline;
    let want_promoted = config.want_promoted;
    let want_predecessors = config.want_predecessors;

    let api_set = build_api_set(requests);

    // WGL mandatory extensions (spec gotcha #9).
    let wgl_mandatory: HashSet<&str> = if spec == Spec::Wgl {
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
        cmd_alias_map(raw)
    } else {
        HashMap::new()
    };
    let enum_to_alias: HashMap<&str, &str> = if want_predecessors {
        bidirectional_alias_map(
            raw.flat_enums
                .iter()
                .map(|(name, e)| (name.as_str(), e.alias.as_deref())),
        )
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
// Alias lookup helpers
// ---------------------------------------------------------------------------

/// Bidirectional name↔alias lookup: both directions of every alias
/// relationship map to the other name.
fn bidirectional_alias_map<'a>(
    entries: impl Iterator<Item = (&'a str, Option<&'a str>)>,
) -> HashMap<&'a str, &'a str> {
    let mut m = HashMap::new();
    for (name, alias) in entries {
        if let Some(alias) = alias {
            m.insert(name, alias);
            m.insert(alias, name);
        }
    }
    m
}

/// Bidirectional command↔alias lookup for the whole spec.
fn cmd_alias_map(raw: &RawSpec) -> HashMap<&str, &str> {
    bidirectional_alias_map(
        raw.commands
            .iter()
            .map(|(name, cmd)| (name.as_str(), cmd.alias.as_deref())),
    )
}

/// True if `cmd` — or its cross-version alias — is in the core command set.
///
/// This is the shared kernel of the --promoted and --baseline checks; the
/// passes differ only in the quantifier applied over an extension's
/// commands: promotion needs ANY command in core, baseline domination needs
/// ALL of them.
fn cmd_in_core(core_cmds: &HashSet<String>, aliases: &HashMap<&str, &str>, cmd: &str) -> bool {
    core_cmds.contains(cmd) || aliases.get(cmd).is_some_and(|a| core_cmds.contains(*a))
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
                        req.commands
                            .iter()
                            .any(|c| cmd_in_core(core_cmds, cmd_to_alias, c))
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
    let baseline_core_cmds = per_api_core_commands(&baseline_features, baseline);
    let baseline_cmd_aliases = cmd_alias_map(raw);

    let request_api_set: HashSet<&str> = requests.iter().map(|r| r.api.as_str()).collect();
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
                && ext_cmds
                    .iter()
                    .all(|c| cmd_in_core(core_cmds, &baseline_cmd_aliases, c))
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
    use super::super::fixtures::*;
    use super::*;

    fn names<'a>(selected: &'a [SelectedExt<'_>]) -> Vec<&'a str> {
        selected.iter().map(|e| e.raw.name.as_str()).collect()
    }

    // ---- promoted_pass ----

    #[test]
    fn promoted_selects_same_name_and_alias_promotions_only() {
        let mut raw = raw_spec(Spec::Gl);
        add_command(&mut raw, "glCoreCmd", None);
        add_command(&mut raw, "glCmdARB", Some("glCoreCmd"));
        add_command(&mut raw, "glOther", None);
        raw.extensions.push(extension(
            "GL_ARB_same_name",
            &["gl"],
            &[],
            vec![require_cmds(&["glCoreCmd"])],
        ));
        raw.extensions.push(extension(
            "GL_ARB_renamed",
            &["gl"],
            &[],
            vec![require_cmds(&["glCmdARB"])],
        ));
        raw.extensions.push(extension(
            "GL_ARB_unrelated",
            &["gl"],
            &[],
            vec![require_cmds(&["glOther"])],
        ));

        let api_set: HashSet<&str> = ["gl"].into_iter().collect();
        let core: HashMap<String, HashSet<String>> =
            HashMap::from([("gl".to_string(), HashSet::from(["glCoreCmd".to_string()]))]);
        let aliases = cmd_alias_map(&raw);

        let mut selected = Vec::new();
        promoted_pass(&raw, &api_set, &core, &aliases, &mut selected);

        // Same-name promotion and alias-mediated promotion are in; an
        // extension whose commands never reached core is not.
        assert_eq!(names(&selected), ["GL_ARB_same_name", "GL_ARB_renamed"]);
        assert!(
            selected
                .iter()
                .all(|e| matches!(e.reason, SelectionReason::Promoted))
        );
    }

    // ---- predecessors_pass ----

    #[test]
    fn predecessors_follow_shared_commands_transitively() {
        // EXT_new (selected) shares a command with EXT_old, which shares a
        // different command with EXT_older — the fixed-point loop must pull
        // in both.
        let mut raw = raw_spec(Spec::Gl);
        raw.extensions.push(extension(
            "GL_EXT_new",
            &["gl"],
            &[],
            vec![require_cmds(&["glShared"])],
        ));
        raw.extensions.push(extension(
            "GL_EXT_old",
            &["gl"],
            &[],
            vec![require_cmds(&["glShared", "glOlder"])],
        ));
        raw.extensions.push(extension(
            "GL_EXT_older",
            &["gl"],
            &[],
            vec![require_cmds(&["glOlder"])],
        ));

        let api_set: HashSet<&str> = ["gl"].into_iter().collect();
        let mut selected = vec![selected_ext(&raw.extensions[0])];
        predecessors_pass(
            &raw,
            &api_set,
            &HashMap::new(),
            &HashMap::new(),
            &mut selected,
        );

        assert_eq!(
            names(&selected),
            ["GL_EXT_new", "GL_EXT_old", "GL_EXT_older"]
        );
    }

    #[test]
    fn predecessors_match_on_shared_enums_too() {
        let mut raw = raw_spec(Spec::Gl);
        raw.extensions.push(extension(
            "GL_EXT_selected",
            &["gl"],
            &[],
            vec![require_enums(&["GL_SOME_TOKEN"])],
        ));
        raw.extensions.push(extension(
            "GL_EXT_enum_only",
            &["gl"],
            &[],
            vec![require_enums(&["GL_SOME_TOKEN"])],
        ));

        let api_set: HashSet<&str> = ["gl"].into_iter().collect();
        let mut selected = vec![selected_ext(&raw.extensions[0])];
        predecessors_pass(
            &raw,
            &api_set,
            &HashMap::new(),
            &HashMap::new(),
            &mut selected,
        );

        assert_eq!(names(&selected), ["GL_EXT_selected", "GL_EXT_enum_only"]);
    }

    // ---- compute_baseline_excludes ----

    #[test]
    fn baseline_excludes_require_all_commands_dominated() {
        let mut raw = raw_spec(Spec::Gl);
        add_command(&mut raw, "a", None);
        add_command(&mut raw, "b", None);
        add_command(&mut raw, "z", None);
        add_command(&mut raw, "aARB", Some("a"));
        raw.features.push(feature(
            "gl",
            "GL_VERSION_3_0",
            (3, 0),
            vec![require_cmds(&["a", "b"])],
        ));
        raw.extensions.push(extension(
            "GL_EXT_dominated",
            &["gl"],
            &[],
            vec![require_cmds(&["a"])],
        ));
        raw.extensions.push(extension(
            "GL_EXT_alias_dominated",
            &["gl"],
            &[],
            vec![require_cmds(&["aARB"])],
        ));
        raw.extensions.push(extension(
            "GL_EXT_partial",
            &["gl"],
            &[],
            vec![require_cmds(&["a", "z"])],
        ));
        raw.extensions
            .push(extension("GL_EXT_no_cmds", &["gl"], &[], vec![]));
        raw.extensions.push(extension(
            "GL_EXT_multi_api",
            &["gl", "gles2"],
            &[],
            vec![require_cmds(&["a"])],
        ));

        let requests = vec![
            api_request(Api::Gl, Some((4, 6)), Some("core")),
            api_request(Api::Gles2, Some((3, 2)), None),
        ];
        let baseline = vec![api_request(Api::Gl, Some((3, 0)), Some("core"))];

        let excludes = compute_baseline_excludes(&raw, &requests, &baseline);

        // ALL of an extension's commands must be in baseline core (directly
        // or via alias) — and every requested API it supports must dominate.
        // GL_EXT_partial fails on `z`; GL_EXT_no_cmds has nothing to
        // dominate; GL_EXT_multi_api fails because gles2 has no baseline.
        assert!(excludes.contains("GL_EXT_dominated"));
        assert!(excludes.contains("GL_EXT_alias_dominated"));
        assert!(!excludes.contains("GL_EXT_partial"));
        assert!(!excludes.contains("GL_EXT_no_cmds"));
        assert!(!excludes.contains("GL_EXT_multi_api"));
    }

    // ---- apply_exclusions ----

    fn exclusion_fixture() -> RawSpec {
        let mut raw = raw_spec(Spec::Gl);
        add_command(&mut raw, "a", None);
        raw.features.push(feature(
            "gl",
            "GL_VERSION_3_0",
            (3, 0),
            vec![require_cmds(&["a"])],
        ));
        for name in ["GL_X_alpha", "GL_X_beta", "GL_X_gamma"] {
            raw.extensions
                .push(extension(name, &["gl"], &[], vec![require_cmds(&["a"])]));
        }
        raw
    }

    #[test]
    fn exclusion_precedence_explicit_beats_baseline_keep_beats_baseline() {
        let raw = exclusion_fixture();
        let mut selected: Vec<SelectedExt<'_>> = raw.extensions.iter().map(selected_ext).collect();

        // All three extensions are baseline-dominated; alpha is explicitly
        // excluded, beta is kept.
        let filter = ExtensionFilter {
            include: None,
            exclude: HashSet::from(["GL_X_alpha".to_string()]),
            keep: HashSet::from(["GL_X_beta".to_string()]),
        };
        let requests = vec![api_request(Api::Gl, Some((4, 6)), Some("core"))];
        let baseline = vec![api_request(Api::Gl, Some((3, 0)), Some("core"))];

        let (excluded_explicit, excluded_baseline) =
            apply_exclusions(&raw, &requests, &filter, &baseline, &mut selected);

        // Only the kept extension survives.
        assert_eq!(names(&selected), ["GL_X_beta"]);
        assert_eq!(excluded_explicit, vec!["GL_X_alpha".to_string()]);
        // The baseline report drops kept names but keeps explicit ones
        // (alpha was both explicitly and baseline excluded).
        assert_eq!(
            excluded_baseline,
            vec!["GL_X_alpha".to_string(), "GL_X_gamma".to_string()]
        );
    }

    #[test]
    fn exclusion_explicit_exclude_beats_keep() {
        let raw = exclusion_fixture();
        let mut selected: Vec<SelectedExt<'_>> = raw.extensions.iter().map(selected_ext).collect();

        let filter = ExtensionFilter {
            include: None,
            exclude: HashSet::from(["GL_X_alpha".to_string()]),
            keep: HashSet::from(["GL_X_alpha".to_string()]),
        };
        let requests = vec![api_request(Api::Gl, Some((4, 6)), Some("core"))];

        // No baseline — only the explicit exclude applies, and keeping the
        // same name does not rescue it.
        let (excluded_explicit, _) = apply_exclusions(&raw, &requests, &filter, &[], &mut selected);

        assert_eq!(names(&selected), ["GL_X_beta", "GL_X_gamma"]);
        assert_eq!(excluded_explicit, vec!["GL_X_alpha".to_string()]);
    }

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
