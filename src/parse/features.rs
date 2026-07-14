//! Parsing of `<feature>` and `<extension>` elements.

use anyhow::{Context, Result, anyhow};
use indexmap::IndexMap;

use super::SpecDocs;
use crate::diag::Diag;
use crate::identity::Spec;
use crate::ir::{RawExtension, RawFeature, Remove, Require};

// GLX extensions with unresolvable type dependencies (spec gotcha #8).
const BROKEN_GLX_EXTENSIONS: &[&str] = &["GLX_SGIX_video_source", "GLX_SGIX_dmbuffer"];

// WGL extensions that must always be present (spec gotcha #9).
const WGL_MANDATORY_EXTENSIONS: &[&str] =
    &["WGL_ARB_extensions_string", "WGL_EXT_extensions_string"];

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

pub fn parse_features_extensions(
    docs: &SpecDocs<'_, '_>,
    spec: Spec,
    platforms: &IndexMap<String, String>,
    diag: Diag,
) -> Result<(Vec<RawFeature>, Vec<RawExtension>)> {
    let features = parse_features(docs)?;
    let extensions = parse_extensions(docs, spec, platforms, diag)?;
    Ok((features, extensions))
}

// ---------------------------------------------------------------------------
// Features
// ---------------------------------------------------------------------------

/// Extract the identifying attributes of a `<feature>` element, erroring on
/// structural damage.  Features are the API-level skeleton of the spec:
/// silently dropping one (as a `continue` here used to) yields a loader that
/// quietly lacks an entire API version — the worst possible failure mode —
/// so a malformed feature is a hard error, not a skip.
fn feature_identity(node: roxmltree::Node<'_, '_>) -> Result<(String, String, crate::ir::Version)> {
    let name = node
        .attribute("name")
        .map(str::to_string)
        .unwrap_or_else(|| "<unnamed>".to_string());
    let api_raw = node
        .attribute("api")
        .ok_or_else(|| anyhow!("<feature> '{name}' has no api attribute"))?
        .to_string();
    let version_str = node
        .attribute("number")
        .ok_or_else(|| anyhow!("<feature> '{name}' has no number attribute"))?;
    let version = super::parse_version(version_str)
        .with_context(|| format!("<feature> '{name}' has a malformed number attribute"))?;
    if name == "<unnamed>" {
        anyhow::bail!("<feature> (api={api_raw}, number={version_str}) has no name attribute");
    }
    Ok((name, api_raw, version))
}

fn parse_features(docs: &SpecDocs<'_, '_>) -> Result<Vec<RawFeature>> {
    let mut features: Vec<RawFeature> = Vec::new();

    // --- Pass 1: collect all public (non-internal) features ---
    for node in docs.all_features() {
        let api_type = node.attribute("apitype");
        let is_internal = api_type == Some("internal");
        if is_internal {
            continue;
        }

        let (name, api_raw, version) = feature_identity(node)?;

        let requires = node
            .children()
            .filter(|n| n.is_element() && n.tag_name().name() == "require")
            .map(parse_require)
            .collect::<Vec<_>>();
        let removes = node
            .children()
            .filter(|n| n.is_element() && n.tag_name().name() == "remove")
            .map(parse_remove)
            .collect::<Vec<_>>();

        // A feature may list multiple APIs (comma-separated).
        for api in api_raw.split(',') {
            let api = api.trim().to_string();
            features.push(RawFeature {
                name: name.clone(),
                api: api.clone(),
                version: version.clone(),
                requires: requires.clone(),
                removes: removes.clone(),
            });
        }
    }

    // --- Pass 2: merge internal feature require blocks into matching public
    // features (same api + version).  This mirrors what GLAD's parse.py and
    // Khronos's reg.py do — internal features (apitype="internal") partition
    // the API for VulkanBase bookkeeping but their requirements belong to the
    // public API.  We must do this as a second pass because internal features
    // can appear before their corresponding public feature in the XML, so a
    // single-pass approach silently drops the merge when the public feature
    // hasn't been pushed yet.
    for node in docs.all_features() {
        let api_type = node.attribute("apitype");
        let is_internal = api_type == Some("internal");
        if !is_internal {
            continue;
        }

        let (_name, api_raw, version) = feature_identity(node)?;

        let extra_requires: Vec<_> = node
            .children()
            .filter(|n| n.is_element() && n.tag_name().name() == "require")
            .map(parse_require)
            .collect();

        for api in api_raw.split(',') {
            let api = api.trim();
            if let Some(public) = features
                .iter_mut()
                .find(|f| f.api == api && f.version == version)
            {
                public.requires.extend(extra_requires.clone());
            }
            // No match means this api/version isn't selected — that's fine.
        }
    }

    // Sort: by api lexicographically, then by ascending version.
    features.sort_by(|a, b| a.api.cmp(&b.api).then_with(|| a.version.cmp(&b.version)));

    Ok(features)
}

// ---------------------------------------------------------------------------
// Extensions
// ---------------------------------------------------------------------------

fn parse_extensions(
    docs: &SpecDocs<'_, '_>,
    spec: Spec,
    platforms: &IndexMap<String, String>,
    diag: Diag,
) -> Result<Vec<RawExtension>> {
    let mut extensions: Vec<RawExtension> = Vec::new();
    let mut seen_names = std::collections::HashSet::new();

    for node in docs.all_extensions() {
        if node.tag_name().name() != "extension" {
            continue;
        }

        let Some(name) = node.attribute("name").map(str::to_string) else {
            diag.warn("<extension> with no name attribute, skipping");
            continue;
        };

        // Spec gotcha #8: silently drop broken GLX extensions (they have
        // unresolvable type dependencies and are known-dead upstream).
        if spec == Spec::Glx && BROKEN_GLX_EXTENSIONS.contains(&name.as_str()) {
            continue;
        }

        // Dedup by name (supplemental XMLs may re-declare extensions).
        if !seen_names.insert(name.clone()) {
            continue;
        }

        // `supported` uses `|` in GL and `,` in Vulkan.
        let supported_raw = node.attribute("supported").unwrap_or("disabled");
        let supported: Vec<String> = supported_raw
            .replace('|', ",")
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty() && *s != "disabled")
            .map(str::to_string)
            .collect();

        if supported.is_empty() {
            continue;
        }

        // Resolve protection: explicit `protect=` wins; otherwise look up
        // `platform=` in the platforms registry.
        let mut protect: Vec<String> = node
            .attribute("protect")
            .unwrap_or("")
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect();

        if protect.is_empty()
            && let Some(platform) = node.attribute("platform")
            && let Some(p) = platforms.get(platform)
        {
            protect.push(p.clone());
        }

        // Extension-to-extension dependencies: GL uses `requires=` (comma-
        // separated), Vulkan uses `depends=` with `+` (AND), `,` (OR), and
        // parentheses.  We extract every extension-name-looking token from
        // whichever attribute is present — the resolver needs all prerequisites
        // regardless of AND/OR semantics.
        let depends = parse_extension_depends(node);

        let requires = node
            .children()
            .filter(|n| n.is_element() && n.tag_name().name() == "require")
            .map(parse_require)
            .collect::<Vec<_>>();

        extensions.push(RawExtension {
            name,
            supported,
            requires,
            protect,
            depends,
        });
    }

    // Spec gotcha #9: WGL mandatory extensions.
    if spec == Spec::Wgl {
        for &mandatory in WGL_MANDATORY_EXTENSIONS {
            if !extensions.iter().any(|e| e.name == mandatory) {
                diag.warn(format!(
                    "WGL mandatory extension '{mandatory}' not found in spec"
                ));
            }
        }
    }

    Ok(extensions)
}
// ---------------------------------------------------------------------------
// Parse extension dependency attributes
// ---------------------------------------------------------------------------

/// Extract extension dependency names from the `requires=` (GL) or `depends=`
/// (Vulkan) attribute on an `<extension>` element.
///
/// GL uses comma-separated names: `requires="GL_ARB_draw_indirect"`
/// Vulkan uses a boolean expression: `depends="VK_KHR_foo+VK_KHR_bar,VK_VERSION_1_1"`
/// with `+` (AND), `,` (OR), and parentheses.
///
/// We split on all delimiters and return every token that looks like an
/// extension name (contains `_` and doesn't start with a digit).  Version
/// requirements like `VK_VERSION_1_1` are included — the resolver filters
/// them against the actual extension list.
fn parse_extension_depends(node: roxmltree::Node<'_, '_>) -> Vec<String> {
    let attr = node
        .attribute("depends")
        .or_else(|| node.attribute("requires"));

    let Some(raw) = attr else {
        return Vec::new();
    };

    raw.split([',', '+', '(', ')'])
        .map(str::trim)
        .filter(|s| !s.is_empty() && s.contains('_'))
        .map(str::to_string)
        .collect()
}

// ---------------------------------------------------------------------------
// Parse <require> and <remove> blocks
// ---------------------------------------------------------------------------

fn parse_require(node: roxmltree::Node<'_, '_>) -> Require {
    let api = node.attribute("api").map(str::to_string);
    let profile = node.attribute("profile").map(str::to_string);

    let mut req = Require {
        api,
        profile,
        ..Default::default()
    };

    for child in node.children().filter(|n| n.is_element()) {
        let tag = child.tag_name().name();
        let name = child.attribute("name").unwrap_or("").to_string();
        if name.is_empty() {
            continue;
        }
        // Skip extending-enum entries here; they are handled separately in
        // enums.rs to avoid double-counting.
        if tag == "enum" && child.attribute("extends").is_some() {
            continue;
        }
        match tag {
            "type" => req.types.push(name),
            "enum" => req.enums.push(name),
            "command" => req.commands.push(name),
            _ => {}
        }
    }

    req
}

fn parse_remove(node: roxmltree::Node<'_, '_>) -> Remove {
    let profile = node.attribute("profile").map(str::to_string);
    let mut rem = Remove {
        profile,
        ..Default::default()
    };

    for child in node.children().filter(|n| n.is_element()) {
        let name = child.attribute("name").unwrap_or("").to_string();
        if name.is_empty() {
            continue;
        }
        match child.tag_name().name() {
            "command" => rem.commands.push(name),
            "enum" => rem.enums.push(name),
            _ => {}
        }
    }

    rem
}
