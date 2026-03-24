//! Requirement collection — Phase 1 mutable state.
//!
//! `RequirementCollector` owns the 15+ mutable local variables that were
//! previously spread across `resolve_feature_set`'s flat scope.  It is built,
//! mutated during Phase 1 (feature + extension requirement gathering), then
//! borrowed immutably during Phase 2 (materialization).
//!
//! The borrow checker enforces the phase boundary: once you stop calling
//! `&mut self` methods, the collector is frozen.

use std::collections::{HashMap, HashSet};

use indexmap::IndexMap;

use crate::cli::ApiRequest;
use crate::ir::RawSpec;
use crate::parse::types::ident_words;

use super::selection::{SelectedExt, SelectedFeature, api_profile_matches, profile_matches};

// ---------------------------------------------------------------------------
// RequirementCollector
// ---------------------------------------------------------------------------

pub(super) struct RequirementCollector {
    pub req_types: HashSet<String>,
    pub req_enums: HashSet<String>,
    req_commands: IndexMap<String, ()>,
    removed_commands: HashSet<String>,
    removed_enums: HashSet<String>,
    pub per_api_core_cmds: HashMap<String, HashSet<String>>,
    ext_commands: IndexMap<String, ()>,
}

impl RequirementCollector {
    pub fn new() -> Self {
        Self {
            req_types: HashSet::new(),
            req_enums: HashSet::new(),
            req_commands: IndexMap::new(),
            removed_commands: HashSet::new(),
            removed_enums: HashSet::new(),
            per_api_core_cmds: HashMap::new(),
            ext_commands: IndexMap::new(),
        }
    }

    /// Collect required names from the selected features (versions).
    ///
    /// Populates req_types, req_enums, req_commands, removed_commands,
    /// removed_enums, and per_api_core_cmds.  Applies command removes
    /// immediately at the end so the caller doesn't need a separate step.
    pub fn collect_from_features(
        &mut self,
        features: &[SelectedFeature<'_>],
        requests: &[ApiRequest],
    ) {
        for feat in features {
            let req_for_api = requests.iter().find(|r| r.name == feat.api);
            let profile = req_for_api.and_then(|r| r.profile.as_deref());
            let api_cmds = self.per_api_core_cmds.entry(feat.api.clone()).or_default();

            for require in &feat.raw.requires {
                if !api_profile_matches(
                    require.api.as_deref(),
                    require.profile.as_deref(),
                    &feat.api,
                    profile,
                ) {
                    continue;
                }
                self.req_types.extend(require.types.iter().cloned());
                self.req_enums.extend(require.enums.iter().cloned());
                for cmd in &require.commands {
                    self.req_commands.entry(cmd.clone()).or_insert(());
                    api_cmds.insert(cmd.clone());
                }
            }
            for remove in &feat.raw.removes {
                if !profile_matches(remove.profile.as_deref(), profile) {
                    continue;
                }
                self.removed_commands
                    .extend(remove.commands.iter().cloned());
                self.removed_enums.extend(remove.enums.iter().cloned());
                // Apply removes inline — features are processed in version order so
                // each version's removes are applied immediately after its requires.
                for cmd in &remove.commands {
                    api_cmds.remove(cmd.as_str());
                }
            }
        }
        // Apply command removes to the master req_commands set.
        for cmd in &self.removed_commands {
            self.req_commands.shift_remove(cmd.as_str());
        }
    }

    /// Collect additional required names from the selected extensions.
    ///
    /// Populates ext_commands and updates req_types, req_enums.
    /// Applies enum removes at the end.
    pub fn collect_from_extensions(
        &mut self,
        selected_exts: &[SelectedExt<'_>],
        api_names: &[String],
    ) {
        for ext in selected_exts {
            for require in &ext.raw.requires {
                for api in api_names {
                    if !api_profile_matches(require.api.as_deref(), None, api, None) {
                        continue;
                    }
                    self.req_types.extend(require.types.iter().cloned());
                    self.req_enums.extend(require.enums.iter().cloned());
                    for e in &require.enums {
                        self.removed_enums.remove(e.as_str());
                    }
                    for cmd in &require.commands {
                        // Core commands already in req_commands stay there.
                        if !self.req_commands.contains_key(cmd.as_str()) {
                            self.ext_commands.entry(cmd.clone()).or_insert(());
                        }
                    }
                }
            }
        }

        // Apply enum removes.
        for e in &self.removed_enums {
            self.req_enums.remove(e.as_str());
        }
    }

    /// Iteratively expand req_types to a fixed point for Vulkan.
    ///
    /// Any type referenced in the raw_c of a selected type must itself be
    /// selected.  This catches member pointer types used inside required
    /// structs but never appearing in any <require><type> block.
    ///
    /// Also seeds req_types from parameter/return types of all selected
    /// commands, which catches types only referenced as command parameters.
    pub fn expand_vulkan_types(&mut self, raw: &RawSpec, all_cmd_names: &[&str]) {
        let type_names: HashSet<&str> = raw.types.iter().map(|t| t.name.as_str()).collect();

        // Seed from command parameter types.
        for &cmd_name in all_cmd_names {
            if let Some(raw_cmd) = raw.commands.get(cmd_name) {
                for param in &raw_cmd.params {
                    if !param.type_name.is_empty() {
                        self.req_types.insert(param.type_name.clone());
                    }
                }
            }
        }

        loop {
            let mut added = false;
            for t in &raw.types {
                if t.raw_c.is_empty() {
                    continue;
                }
                let auto = matches!(
                    t.category.as_str(),
                    "define" | "basetype" | "bitmask" | "funcpointer" | "enum" | "handle"
                );
                if !auto && !self.req_types.contains(&t.name) {
                    continue;
                }
                for word in ident_words(&t.raw_c) {
                    if type_names.contains(word) && self.req_types.insert(word.to_string()) {
                        added = true;
                    }
                }
            }
            if !added {
                break;
            }
        }
    }

    /// Core command names in insertion order.
    pub fn core_command_names(&self) -> Vec<String> {
        self.req_commands.keys().cloned().collect()
    }

    /// Extension command names in insertion order.
    pub fn ext_command_names(&self) -> Vec<String> {
        self.ext_commands.keys().cloned().collect()
    }
}
