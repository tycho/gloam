//! Fixture builders for resolve unit tests.
//!
//! Terse constructors for the IR and CLI types that selection, command
//! ordering, and protection inference consume — so tests read as scenarios
//! instead of struct literals.  Only compiled for tests.

use indexmap::IndexMap;

use crate::cli::ApiRequest;
use crate::identity::{Api, Spec};
use crate::ir::{
    RawCommand, RawExtension, RawFeature, RawSpec, RawType, Require, TypeCategory, Version,
};

use super::selection::{SelectedExt, SelectedFeature};
use super::types::SelectionReason;

pub(super) fn strs(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

/// An empty spec of the given family; push features/extensions/types onto it.
pub(super) fn raw_spec(spec: Spec) -> RawSpec {
    RawSpec {
        spec,
        platforms: IndexMap::new(),
        types: vec![],
        enum_groups: vec![],
        flat_enums: IndexMap::new(),
        commands: IndexMap::new(),
        features: vec![],
        extensions: vec![],
    }
}

pub(super) fn api_request(
    api: Api,
    version: Option<(u32, u32)>,
    profile: Option<&str>,
) -> ApiRequest {
    ApiRequest {
        api,
        profile: profile.map(str::to_string),
        version: version.map(|(major, minor)| Version::new(major, minor)),
    }
}

pub(super) fn require_cmds(cmds: &[&str]) -> Require {
    Require {
        commands: strs(cmds),
        ..Default::default()
    }
}

pub(super) fn require_enums(enums: &[&str]) -> Require {
    Require {
        enums: strs(enums),
        ..Default::default()
    }
}

pub(super) fn require_types(types: &[&str]) -> Require {
    Require {
        types: strs(types),
        ..Default::default()
    }
}

pub(super) fn feature(
    api: &str,
    name: &str,
    (major, minor): (u32, u32),
    requires: Vec<Require>,
) -> RawFeature {
    RawFeature {
        name: name.to_string(),
        api: api.to_string(),
        version: Version::new(major, minor),
        requires,
        removes: vec![],
    }
}

pub(super) fn extension(
    name: &str,
    supported: &[&str],
    protect: &[&str],
    requires: Vec<Require>,
) -> RawExtension {
    RawExtension {
        name: name.to_string(),
        supported: strs(supported),
        requires,
        protect: strs(protect),
        number: None,
        depends: vec![],
    }
}

pub(super) fn add_command(raw: &mut RawSpec, name: &str, alias: Option<&str>) {
    raw.commands.insert(
        name.to_string(),
        RawCommand {
            name: name.to_string(),
            api: None,
            return_type: "void".to_string(),
            params: vec![],
            alias: alias.map(str::to_string),
        },
    );
}

pub(super) fn raw_type(
    name: &str,
    category: TypeCategory,
    requires: Option<&str>,
    raw_c: &str,
    protect: Option<&str>,
) -> RawType {
    RawType {
        name: name.to_string(),
        api: None,
        category,
        requires: requires.map(str::to_string),
        alias: None,
        bitwidth: None,
        raw_c: raw_c.to_string(),
        protect: protect.map(str::to_string),
    }
}

pub(super) fn selected_feature<'a>(api: Api, raw: &'a RawFeature) -> SelectedFeature<'a> {
    SelectedFeature { api, raw }
}

pub(super) fn selected_ext<'a>(raw: &'a RawExtension) -> SelectedExt<'a> {
    SelectedExt {
        raw,
        reason: SelectionReason::Explicit,
    }
}
