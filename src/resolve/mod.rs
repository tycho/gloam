//! Feature-set resolution.
//!
//! Takes a `RawSpec` plus the user's API requests and produces a `FeatureSet`
//! with all arrays indexed, range tables built, and alias pairs computed.
//! This is the bridge between the parser and the code generators.
//!
//! The resolution pipeline has two phases, one function each:
//!   1. **Selection** ([`phase1_select`]) — determine which features and
//!      extensions are "in" and gather the requirement sets they imply
//!   2. **Materialization** ([`phase2_materialize`]) — build the indexed
//!      arrays from the frozen selection
//!
//! (Protection grouping for header emission is not a resolve concern — it
//! lives in the C generator's `RenderModel`.)

mod commands;
mod enums;
#[cfg(test)]
mod fixtures;
mod pfn;
mod protect;
mod requirements;
mod selection;
mod spec_info;
mod typedefs;

// Public types — re-exported so external callers use `crate::resolve::FeatureSet` etc.
pub mod types;
pub use types::{
    Extension, Feature, FeatureSet, FlatEnum, Param, PfnRange, Protect, SelectionReason,
    SerVersion, TypeDef,
};

use std::collections::HashMap;

use anyhow::Result;
use indexmap::IndexMap;

use crate::cli::{ApiRequest, Cli};
use crate::fetch;
use crate::identity::Spec;
use crate::ir::RawSpec;
use crate::parse;

use commands::{build_alias_pairs, materialize_commands};
use enums::{build_enum_groups, build_flat_enums};
use pfn::{build_ext_pfn_ranges, build_feature_pfn_ranges};
use requirements::RequirementCollector;
use selection::{
    ExtensionSelection, SelectedExt, SelectedFeature, select_extensions, select_features,
};
use spec_info::{
    ResolveConfig, SpecInfo, api_names as request_api_names, ext_short_name, version_short_name,
};
use typedefs::{build_type_list, collect_required_headers};

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

pub fn build_feature_sets(
    cli: &Cli,
    store: &crate::provenance::load::SourceStore,
    diag: crate::diag::Diag,
) -> Result<Vec<FeatureSet>> {
    let requests = cli.api_requests()?;
    let ext_filter = cli.extension_filter()?;
    let baseline = cli.baseline_requests()?;
    let promoted = cli.promoted;
    let predecessors = cli.predecessors;

    let alias = match &cli.generator {
        crate::cli::Generator::C(c) => c.alias,
        crate::cli::Generator::Lock(_) => false, // never reached: lock skips resolution
    };

    // Batch the requests: a merged build resolves one feature set per spec
    // family (all of that spec's requests together); a non-merged build
    // resolves one feature set per request.  Batch order follows request
    // order either way (IndexMap keys by first occurrence).
    let batches: Vec<Vec<ApiRequest>> = if cli.merge {
        let mut by_spec: IndexMap<Spec, Vec<ApiRequest>> = IndexMap::new();
        for req in requests {
            by_spec.entry(req.spec()).or_default().push(req);
        }
        by_spec.into_values().collect()
    } else {
        requests.into_iter().map(|req| vec![req]).collect()
    };

    let config = ResolveConfig {
        ext_filter: &ext_filter,
        baseline: &baseline,
        is_merged: cli.merge,
        want_aliases: alias,
        want_promoted: promoted,
        want_predecessors: predecessors,
    };

    let mut feature_sets = Vec::new();
    for batch in &batches {
        let spec = batch[0].spec();
        let apis: Vec<&str> = batch.iter().map(|r| r.api.as_str()).collect();
        let sources = fetch::load_spec(spec.as_str(), &apis, store)?;
        let raw = parse::parse(&sources, spec, diag)?;
        let fs = resolve_feature_set(&raw, batch, &config, &sources.source_keys)?;
        feature_sets.push(fs);
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
    xml_source_keys: &[String],
) -> Result<FeatureSet> {
    let selection = phase1_select(raw, requests, config);
    phase2_materialize(raw, requests, config, xml_source_keys, selection)
}

/// Phase 1 output: which features and extensions are "in", and the frozen
/// requirement sets they imply.
struct Selection<'a> {
    selected_features: Vec<SelectedFeature<'a>>,
    selected_exts: Vec<SelectedExt<'a>>,
    excluded_explicit: Vec<String>,
    excluded_baseline: Vec<String>,
    reqs: RequirementCollector,
}

/// Phase 1: selection + requirement gathering.
///
/// All `RequirementCollector` mutation happens here — including the Vulkan
/// type-closure expansion, which only widens `req_types` and needs nothing
/// from materialization.
fn phase1_select<'a>(
    raw: &'a RawSpec,
    requests: &[ApiRequest],
    config: &ResolveConfig<'_>,
) -> Selection<'a> {
    let api_names = request_api_names(requests);
    let selected_features = select_features(raw, requests);

    let mut reqs = RequirementCollector::new();
    reqs.collect_from_features(&selected_features, requests);

    let ExtensionSelection {
        selected: selected_exts,
        excluded_explicit,
        excluded_baseline,
    } = select_extensions(raw, requests, config, raw.spec, &reqs.per_api_core_cmds);

    reqs.collect_from_extensions(&selected_exts, &api_names);

    if raw.spec.is_vulkan() {
        reqs.expand_vulkan_types(raw);
    }

    Selection {
        selected_features,
        selected_exts,
        excluded_explicit,
        excluded_baseline,
        reqs,
    }
}

/// Phase 2: materialize the indexed arrays from the frozen selection.
fn phase2_materialize(
    raw: &RawSpec,
    requests: &[ApiRequest],
    config: &ResolveConfig<'_>,
    xml_source_keys: &[String],
    selection: Selection<'_>,
) -> Result<FeatureSet> {
    let spec_kind = raw.spec;
    let spec = SpecInfo::new(spec_kind);
    let api_names = request_api_names(requests);
    let Selection {
        selected_features,
        selected_exts,
        excluded_explicit,
        excluded_baseline,
        reqs,
    } = selection;

    // -- Commands -------------------------------------------------------
    let commands = materialize_commands(
        raw,
        spec_kind,
        &reqs,
        &selected_features,
        &selected_exts,
        requests,
    )?;

    // -- Features -------------------------------------------------------
    let features: Vec<Feature> = selected_features
        .iter()
        .enumerate()
        .map(|(i, sf)| {
            let ver = &sf.raw.version;
            let short = version_short_name(&sf.raw.name, sf.api);
            Feature {
                index: i as u16,
                full_name: sf.raw.name.clone(),
                short_name: short,
                version: SerVersion {
                    major: ver.major,
                    minor: ver.minor,
                },
                packed: ver.packed(),
                api: sf.api.as_str().to_string(),
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

    // -- PFN ranges ------------------------------------------------------
    let feature_pfn_ranges = build_feature_pfn_ranges(&selected_features, &features, &commands);
    let mut ext_pfn_ranges: IndexMap<String, Vec<PfnRange>> = IndexMap::new();
    let mut ext_subset_indices: IndexMap<String, Vec<u16>> = IndexMap::new();

    for api in &api_names {
        let (ranges, indices) =
            build_ext_pfn_ranges(api, &selected_exts, &ext_index_map, &commands);
        ext_pfn_ranges.insert(api.clone(), ranges);
        ext_subset_indices.insert(api.clone(), indices);
    }

    // -- Types ----------------------------------------------------------
    let types = build_type_list(raw, &reqs.req_types, spec_kind, &selected_exts);

    // -- Enums ----------------------------------------------------------
    let flat_enums = build_flat_enums(raw, &reqs.req_enums, spec.is_vulkan);
    let enum_groups = build_enum_groups(raw);

    // -- Alias pairs ----------------------------------------------------
    let alias_pairs = if config.want_aliases {
        build_alias_pairs(raw, &commands)
    } else {
        Vec::new()
    };

    let required_headers = collect_required_headers(raw, &reqs.req_types, spec_kind);

    // Contributing source provenance: the merged XML sources, plus the
    // auxiliary headers emitted into the output tree and xxhash.h (always used
    // by the generated .c for hash-based extension detection).  Sorted/deduped.
    let source_keys = {
        let mut keys: Vec<String> = xml_source_keys.to_vec();
        keys.extend(required_headers.iter().cloned());
        keys.push("xxhash.h".to_string());
        keys.sort();
        keys.dedup();
        keys
    };

    Ok(FeatureSet {
        spec: spec_kind,
        spec_name: spec_kind.as_str().to_string(),
        display_name: spec.display_name.to_string(),
        apis: api_names,
        is_merged: config.is_merged,
        is_vulkan: spec.is_vulkan,
        is_gl_family: spec.is_gl_family,
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
        source_keys,
        excluded_explicit,
        excluded_baseline,
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
        protect: Protect(e.raw.protect.clone()),
        reason: e.reason,
    }
}
