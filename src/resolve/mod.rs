//! Feature-set resolution.
//!
//! Takes a `RawSpec` plus the user's API requests and produces a `FeatureSet`
//! with all arrays indexed, range tables built, and alias pairs computed.
//! This is the bridge between the parser and the code generators.
//!
//! The resolution pipeline has three phases:
//!   1. **Selection** — determine which features and extensions are "in"
//!   2. **Materialization** — build indexed arrays from selected items
//!   3. **Grouping** — coalesce items by protection for header emission

mod commands;
mod enums;
mod pfn;
mod protect;
mod requirements;
mod selection;
mod spec_info;
mod typedefs;

// Public types — re-exported so external callers use `crate::resolve::FeatureSet` etc.
pub mod types;
pub use types::{
    CmdPfnEntry, Command, ExtGuardEntry, Extension, Feature, FeatureSet, PfnRange, ScopeBoundaries, SelectionReason, SerVersion,
};

use std::collections::HashMap;

use anyhow::Result;
use indexmap::IndexMap;

use crate::cli::{ApiRequest, Cli};
use crate::fetch;
use crate::ir::RawSpec;
use crate::parse;
use crate::parse::commands::infer_vulkan_scope;

use commands::{build_alias_pairs, build_command, build_command_protect_map, optimize_command_order};
use enums::{build_enum_groups, build_flat_enums};
use pfn::{build_ext_pfn_ranges, build_feature_pfn_ranges};
use protect::{group_by_protection, group_by_protection_pairs};
use requirements::RequirementCollector;
use selection::{ExtensionSelection, SelectedExt, select_extensions, select_features};
use spec_info::{ResolveConfig, SpecInfo, ext_short_name, version_short_name, xml_api_names};
use typedefs::{build_type_list, collect_required_headers};

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

pub fn build_feature_sets(cli: &Cli) -> Result<Vec<FeatureSet>> {
    let requests = cli.api_requests()?;
    let ext_filter = cli.extension_filter()?;
    let baseline = cli.baseline_requests()?;
    let promoted = cli.promoted;
    let predecessors = cli.predecessors;

    let alias = match &cli.generator {
        crate::cli::Generator::C(c) => c.alias,
    };
    let unchecked = match &cli.generator {
        crate::cli::Generator::C(c) => c.unchecked,
    };

    let mut feature_sets = Vec::new();

    if cli.merge {
        let mut by_spec: IndexMap<String, Vec<ApiRequest>> = IndexMap::new();
        for req in requests {
            by_spec
                .entry(req.spec_name().to_string())
                .or_default()
                .push(req);
        }
        for (spec_name, reqs) in &by_spec {
            let sources = fetch::load_spec(spec_name, cli.use_fetch())?;
            let raw = parse::parse(&sources, spec_name)?;
            let config = ResolveConfig {
                ext_filter: &ext_filter,
                baseline: &baseline,
                is_merged: true,
                want_aliases: alias,
                want_promoted: promoted,
                want_predecessors: predecessors,
                unchecked,
            };
            let fs = resolve_feature_set(&raw, reqs, &config)?;
            feature_sets.push(fs);
        }
    } else {
        for req in &requests {
            let spec_name = req.spec_name();
            let sources = fetch::load_spec(spec_name, cli.use_fetch())?;
            let raw = parse::parse(&sources, spec_name)?;
            let config = ResolveConfig {
                ext_filter: &ext_filter,
                baseline: &baseline,
                is_merged: false,
                want_aliases: alias,
                want_promoted: promoted,
                want_predecessors: predecessors,
                unchecked,
            };
            let fs = resolve_feature_set(&raw, std::slice::from_ref(req), &config)?;
            feature_sets.push(fs);
        }
    }

    Ok(feature_sets)
}

// ---------------------------------------------------------------------------
// Core resolution pipeline
// ---------------------------------------------------------------------------

fn resolve_feature_set(
    raw: &RawSpec,
    requests: &[ApiRequest],
    config: &ResolveConfig<'_>,
) -> Result<FeatureSet> {
    let spec_name = &raw.spec_name;
    let spec = SpecInfo::new(spec_name);
    let api_names = xml_api_names(requests);

    // ==================================================================
    // Phase 1: Selection + requirement gathering
    // ==================================================================
    let selected_features = select_features(raw, requests);

    let mut reqs = RequirementCollector::new();
    reqs.collect_from_features(&selected_features, requests);

    let ext_sel = select_extensions(raw, requests, config, spec_name, &reqs.per_api_core_cmds);
    let ExtensionSelection {
        selected: selected_exts,
        excluded_explicit,
        excluded_baseline,
    } = ext_sel;

    reqs.collect_from_extensions(&selected_exts, &api_names);

    // ==================================================================
    // Phase 2: Materialize indexed arrays
    // ==================================================================

    // -- Commands -------------------------------------------------------
    let cmd_protect_map = build_command_protect_map(&selected_exts);

    // Extract command name lists from the collector.  These owned Vecs must
    // live in the function scope so that `all_cmd_names: Vec<&str>` can
    // borrow from them regardless of which branch is taken.
    let core_names = reqs.core_command_names();
    let ext_names = reqs.ext_command_names();

    // Storage for the optimized name list in normal mode; unused in
    // unchecked mode but must live as long as all_cmd_names.
    let optimized_names: Vec<String>;

    let all_cmd_names: Vec<&str> = if spec.is_vulkan && config.unchecked {
        // Combine core + ext, sort by (guarded, scope, protect, alpha).
        let mut names: Vec<&str> = core_names
            .iter()
            .map(String::as_str)
            .chain(ext_names.iter().map(String::as_str))
            .collect();
        names.sort_by(|a, b| {
            let scope_key = |name: &&str| -> u8 {
                raw.commands
                    .get(*name)
                    .map(|c| infer_vulkan_scope(c) as u8)
                    .unwrap_or(4)
            };
            let guarded_key = |name: &&str| -> u8 { cmd_protect_map.contains_key(*name) as u8 };
            let protect_key = |name: &&str| -> &str {
                cmd_protect_map.get(*name).map(|s| s.as_str()).unwrap_or("")
            };
            guarded_key(a)
                .cmp(&guarded_key(b))
                .then_with(|| scope_key(a).cmp(&scope_key(b)))
                .then_with(|| protect_key(a).cmp(protect_key(b)))
                .then_with(|| a.cmp(b))
        });
        names
    } else {
        let (sorted_core, sorted_ext) = optimize_command_order(
            &core_names,
            &ext_names,
            &selected_features,
            &selected_exts,
            requests,
        );
        optimized_names = sorted_core.into_iter().chain(sorted_ext).collect();
        optimized_names.iter().map(String::as_str).collect()
    };

    // Vulkan type expansion (needs all_cmd_names).
    if spec.is_vulkan {
        reqs.expand_vulkan_types(raw, &all_cmd_names);
    }

    let mut commands: Vec<Command> = Vec::with_capacity(all_cmd_names.len());
    for (idx, &cmd_name) in all_cmd_names.iter().enumerate() {
        let raw_cmd = match raw.commands.get(cmd_name) {
            Some(c) => c,
            None => {
                eprintln!("warning: command '{}' required but not in spec", cmd_name);
                continue;
            }
        };
        let scope = if spec.is_vulkan {
            infer_vulkan_scope(raw_cmd).c_name().to_string()
        } else {
            String::new()
        };
        let protect = cmd_protect_map.get(cmd_name).cloned();
        commands.push(build_command(
            idx as u16,
            raw_cmd,
            &scope,
            protect,
            spec.pfn_prefix,
            spec.name_prefix,
        ));
    }

    // -- Features -------------------------------------------------------
    let features: Vec<Feature> = selected_features
        .iter()
        .enumerate()
        .map(|(i, sf)| {
            let ver = &sf.raw.version;
            let short = version_short_name(&sf.raw.name, &sf.api);
            Feature {
                index: i as u16,
                full_name: sf.raw.name.clone(),
                short_name: short,
                version: SerVersion {
                    major: ver.major,
                    minor: ver.minor,
                },
                packed: ver.packed(),
                api: sf.api.clone(),
            }
        })
        .collect();

    // -- Extensions (sorted alphabetically) ----------------------------
    let mut sorted_exts: Vec<_> = selected_exts.iter().collect();
    sorted_exts.sort_by_key(|e| e.raw.name.as_str());

    let extensions: Vec<Extension> = sorted_exts
        .iter()
        .enumerate()
        .map(|(i, e)| build_extension(i as u16, e))
        .collect();

    let ext_index_map: HashMap<&str, u16> = extensions
        .iter()
        .map(|e| (e.name.as_str(), e.index))
        .collect();

    // -- PFN ranges / scope boundaries ---------------------------------
    let (feature_pfn_ranges, ext_pfn_ranges, ext_subset_indices, scope_boundaries) =
        if spec.is_vulkan && config.unchecked {
            let sb = compute_scope_boundaries(raw, &commands);
            let empty_ranges: IndexMap<String, Vec<PfnRange>> = IndexMap::new();
            let empty_indices: IndexMap<String, Vec<u16>> = IndexMap::new();
            (Vec::new(), empty_ranges, empty_indices, Some(sb))
        } else {
            let feat_ranges = build_feature_pfn_ranges(&selected_features, &features, &commands);
            let mut ext_ranges: IndexMap<String, Vec<PfnRange>> = IndexMap::new();
            let mut ext_indices: IndexMap<String, Vec<u16>> = IndexMap::new();

            for api in &api_names {
                let (ranges, indices) =
                    build_ext_pfn_ranges(api, &selected_exts, &ext_index_map, &commands);
                ext_ranges.insert(api.clone(), ranges);
                ext_indices.insert(api.clone(), indices);
            }

            (feat_ranges, ext_ranges, ext_indices, None)
        };

    // -- Types ----------------------------------------------------------
    let types = build_type_list(raw, &reqs.req_types, spec_name, spec.is_vulkan, &selected_exts);

    // -- Enums ----------------------------------------------------------
    let flat_enums = build_flat_enums(raw, &reqs.req_enums, spec.is_vulkan);
    let enum_groups = build_enum_groups(raw);

    // -- Alias pairs ----------------------------------------------------
    let alias_pairs = if config.want_aliases {
        build_alias_pairs(raw, &commands)
    } else {
        Vec::new()
    };

    let required_headers = collect_required_headers(raw, &reqs.req_types, spec_name);

    // ==================================================================
    // Phase 3: Protection grouping
    // ==================================================================
    let include_type_groups = group_by_protection(
        types
            .iter()
            .filter(|t| t.category == "include" && !t.raw_c.is_empty())
            .cloned(),
        |t| t.protect.clone(),
    );

    let type_groups = group_by_protection(
        types
            .iter()
            .filter(|t| t.category != "include" && !t.raw_c.is_empty())
            .cloned(),
        |t| t.protect.clone(),
    );

    let ext_guard_groups = group_by_protection_pairs(extensions.iter().map(|e| {
        (
            e.protect.clone(),
            ExtGuardEntry {
                name: e.name.clone(),
                short_name: e.short_name.clone(),
            },
        )
    }));

    let cmd_pfn_groups = group_by_protection_pairs(commands.iter().map(|c| {
        let protect = c
            .protect
            .as_ref()
            .map(|p| vec![p.clone()])
            .unwrap_or_default();
        (
            protect,
            CmdPfnEntry {
                index: c.index,
                name: c.name.clone(),
                short_name: c.short_name.clone(),
                pfn_type: c.pfn_type.clone(),
                return_type: c.return_type.clone(),
                params_str: c.params_str.clone(),
            },
        )
    }));

    let flat_enum_groups = group_by_protection(flat_enums.iter().cloned(), |e| e.protect.clone());

    Ok(FeatureSet {
        spec_name: spec_name.clone(),
        display_name: spec.display_name.to_string(),
        apis: api_names,
        is_merged: config.is_merged,
        is_vulkan: spec.is_vulkan,
        is_gl_family: spec.is_gl_family,
        context_name: spec.context_name,
        features,
        extensions,
        commands,
        types,
        flat_enums,
        enum_groups,
        feature_pfn_ranges,
        ext_pfn_ranges,
        ext_subset_indices,
        alias_pairs,
        required_headers,
        excluded_explicit,
        excluded_baseline,
        include_type_groups,
        type_groups,
        ext_guard_groups,
        cmd_pfn_groups,
        flat_enum_groups,
        scope_boundaries,
    })
}

// ---------------------------------------------------------------------------
// Build a single Extension entry
// ---------------------------------------------------------------------------

fn build_extension(index: u16, e: &SelectedExt<'_>) -> Extension {
    use xxhash_rust::xxh3::xxh3_64;

    let hash_val = xxh3_64(e.raw.name.as_bytes());
    let hash = format!("0x{:016x}", hash_val);
    let short = ext_short_name(&e.raw.name);

    Extension {
        index,
        name: e.raw.name.clone(),
        short_name: short,
        hash,
        protect: e.raw.protect.clone(),
        reason: e.reason,
    }
}

// ---------------------------------------------------------------------------
// Scope boundaries (--unchecked Vulkan mode)
// ---------------------------------------------------------------------------

fn compute_scope_boundaries(raw: &RawSpec, commands: &[Command]) -> ScopeBoundaries {
    let mut global_start: Option<u16> = None;
    let mut instance_start: Option<u16> = None;
    let mut device_start: Option<u16> = None;
    let mut guarded_start: Option<u16> = None;
    let mut guarded_global_start: Option<u16> = None;
    let mut guarded_instance_start: Option<u16> = None;
    let mut guarded_device_start: Option<u16> = None;

    for cmd in commands {
        let is_guarded = cmd.protect.is_some();
        let ordinal = raw
            .commands
            .get(cmd.name.as_str())
            .map(|c| infer_vulkan_scope(c) as u8)
            .unwrap_or(0);

        if !is_guarded {
            if global_start.is_none() && ordinal >= 1 {
                global_start = Some(cmd.index);
            }
            if instance_start.is_none() && ordinal >= 2 {
                instance_start = Some(cmd.index);
            }
            if device_start.is_none() && ordinal >= 3 {
                device_start = Some(cmd.index);
            }
        } else {
            if guarded_start.is_none() {
                guarded_start = Some(cmd.index);
            }
            if guarded_global_start.is_none() && ordinal >= 1 {
                guarded_global_start = Some(cmd.index);
            }
            if guarded_instance_start.is_none() && ordinal >= 2 {
                guarded_instance_start = Some(cmd.index);
            }
            if guarded_device_start.is_none() && ordinal >= 3 {
                guarded_device_start = Some(cmd.index);
            }
        }
    }

    let end = commands.len() as u16;
    let device_start = device_start.unwrap_or(guarded_start.unwrap_or(end));
    let instance_start = instance_start.unwrap_or(device_start);
    let global_start = global_start.unwrap_or(instance_start);
    let guarded = guarded_start.unwrap_or(end);
    let guarded_device = guarded_device_start.unwrap_or(end);
    let guarded_instance = guarded_instance_start.unwrap_or(guarded_device);
    let guarded_global = guarded_global_start.unwrap_or(guarded_instance);
    let guarded_unknown = guarded;

    ScopeBoundaries {
        unknown: 0,
        global: global_start,
        instance: instance_start,
        device: device_start,
        guarded,
        guarded_unknown,
        guarded_global,
        guarded_instance,
        guarded_device,
        end,
    }
}
