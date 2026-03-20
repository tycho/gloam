//! Parsing of `<enums>` blocks into `RawEnumGroup` (Vulkan typed enums) and
//! the flat `IndexMap<String, RawEnum>` (GL-style `#define` constants).

use anyhow::Result;
use indexmap::IndexMap;

use super::{SpecDocs, compute_ext_enum_value};
use crate::ir::{RawEnum, RawEnumGroup};

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

pub fn parse_enums(
    docs: &SpecDocs<'_, '_>,
    spec_name: &str,
) -> Result<(Vec<RawEnumGroup>, IndexMap<String, RawEnum>)> {
    let is_vulkan = spec_name == "vk";

    let mut enum_groups: Vec<RawEnumGroup> = Vec::new();
    let mut flat_enums: IndexMap<String, RawEnum> = IndexMap::new();

    for block in docs.all_enums_blocks() {
        let block_type = block.attribute("type");

        // Spec gotcha #11: For Vulkan, `<enums type="enum"|"bitmask">` elements
        // are already declared as typed enum types — do not re-process as flat
        // constants.
        if is_vulkan && matches!(block_type, Some("enum") | Some("bitmask")) {
            let group = parse_enum_group(block, docs)?;
            // Do NOT prune empty groups here — extensions may add values to
            // them later in collect_vulkan_extending_enums.  We prune after
            // that pass completes.
            enum_groups.push(group);
            continue;
        }

        // GL-style flat constant block.
        let _namespace = block.attribute("namespace").unwrap_or("");
        let _parent_group = block.attribute("group").unwrap_or("");
        let _comment = block.attribute("comment").unwrap_or("");

        for child in block.children().filter(|n| n.is_element()) {
            let tag = child.tag_name().name();
            if tag == "unused" || tag == "comment" {
                continue;
            }
            if tag != "enum" {
                continue;
            }

            let enum_val = parse_flat_enum(child, None, None)?;
            flat_enums.entry(enum_val.name.clone()).or_insert(enum_val);
        }
    }

    // For Vulkan, also collect enum extensions from <extension> / <feature> require blocks.
    if is_vulkan {
        collect_vulkan_extending_enums(docs, &mut enum_groups, &mut flat_enums)?;
        // Spec gotcha #3: prune Vulkan enum groups that are still empty after
        // all extension values have been collected.  These are forward-declared
        // types with no values in any supported extension.
        enum_groups.retain(|g| !g.values.is_empty());
    }

    Ok((enum_groups, flat_enums))
}

// ---------------------------------------------------------------------------
// Vulkan typed enum group
// ---------------------------------------------------------------------------

fn parse_enum_group(
    block: roxmltree::Node<'_, '_>,
    _docs: &SpecDocs<'_, '_>,
) -> Result<RawEnumGroup> {
    let name = block
        .attribute("name")
        .ok_or_else(|| anyhow::anyhow!("<enums> block has no name attribute"))?
        .to_string();

    let bitwidth = block
        .attribute("bitwidth")
        .and_then(|s| s.parse::<u32>().ok());

    let mut values: IndexMap<String, RawEnum> = IndexMap::new();

    for child in block.children().filter(|n| n.is_element()) {
        if child.tag_name().name() != "enum" {
            continue;
        }
        let e = parse_flat_enum(child, None, Some(&name))?;
        values.entry(e.name.clone()).or_insert(e);
    }

    Ok(RawEnumGroup {
        name,
        bitwidth,
        values: values.into_values().collect(),
    })
}

// ---------------------------------------------------------------------------
// Collect Vulkan enum extensions from <extension> / <feature> blocks
// ---------------------------------------------------------------------------

fn collect_vulkan_extending_enums(
    docs: &SpecDocs<'_, '_>,
    groups: &mut Vec<RawEnumGroup>,
    flat_enums: &mut IndexMap<String, RawEnum>,
) -> Result<()> {
    // Build a quick lookup: group name -> index in `groups`.
    let mut group_index: IndexMap<String, usize> = IndexMap::new();
    for (i, g) in groups.iter().enumerate() {
        group_index.insert(g.name.clone(), i);
    }

    // Walk every <require> block inside every <extension> and <feature>.
    let process_require = |require: roxmltree::Node<'_, '_>,
                           extnumber: Option<u32>,
                           groups: &mut Vec<RawEnumGroup>,
                           flat_enums: &mut IndexMap<String, RawEnum>|
     -> Result<()> {
        for child in require.children().filter(|n| n.is_element()) {
            if child.tag_name().name() != "enum" {
                continue;
            }

            let _api_attr = child.attribute("api"); // reserved for per-api filtering in resolver

            if let Some(extends) = child.attribute("extends") {
                // This value extends an existing typed enum group.
                let e = parse_flat_enum(child, extnumber, Some(extends))?;

                if let Some(&gi) = group_index.get(extends) {
                    // Check for duplicate with conflicting value (spec gotcha #13).
                    if let Some(existing) = groups[gi].values.iter().find(|v| v.name == e.name) {
                        if existing.value != e.value {
                            anyhow::bail!(
                                "extension enum '{}' required multiple times with different values",
                                e.name
                            );
                        }
                        // Same value — fine, skip.
                    } else {
                        groups[gi].values.push(e);
                    }
                }
                // If the group still isn't known, it was never declared in any
                // <enums> block — skip rather than bail, as some extensions
                // reference groups only present in later spec versions.
            } else {
                // No extends= — this is an inline constant definition such as
                //   VK_KHR_PORTABILITY_ENUMERATION_SPEC_VERSION = 1
                //   VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME = "VK_KHR_..."
                // These are not part of any typed enum group; they belong in
                // flat_enums alongside the API constants.
                // Only capture entries that actually carry a value — pure alias
                // entries (alias= only, no value/bitpos/offset) are resolved
                // through the existing alias mechanism.
                let has_value = child.attribute("value").is_some()
                    || child.attribute("bitpos").is_some()
                    || child.attribute("offset").is_some();
                if !has_value {
                    continue;
                }
                let e = parse_flat_enum(child, extnumber, None)?;
                flat_enums.entry(e.name.clone()).or_insert(e);
            }
        }
        Ok(())
    };

    // Process <feature> blocks.
    for feat in docs.all_features() {
        let extnumber = feat.attribute("number").and_then(|s| s.parse::<u32>().ok());
        for require in feat
            .children()
            .filter(|n| n.is_element() && n.tag_name().name() == "require")
        {
            process_require(require, extnumber, groups, flat_enums)?;
        }
    }

    // Process <extension> blocks.
    for ext in docs.all_extensions() {
        let extnumber = ext.attribute("number").and_then(|s| s.parse::<u32>().ok());
        for require in ext
            .children()
            .filter(|n| n.is_element() && n.tag_name().name() == "require")
        {
            process_require(require, extnumber, groups, flat_enums)?;
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Parse a single <enum> element
// ---------------------------------------------------------------------------

fn parse_flat_enum(
    node: roxmltree::Node<'_, '_>,
    parent_extnumber: Option<u32>,
    parent_type: Option<&str>,
) -> Result<RawEnum> {
    let name = node
        .attribute("name")
        .ok_or_else(|| anyhow::anyhow!("<enum> element has no name"))?
        .to_string();

    let api = node.attribute("api").map(str::to_string);
    let type_suffix = node.attribute("type").map(str::to_string);
    let alias = node.attribute("alias").map(str::to_string);
    let comment = node.attribute("comment").unwrap_or("").to_string();

    // A per-element extnumber overrides the parent's.
    let extnumber = node
        .attribute("extnumber")
        .and_then(|s| s.parse::<u32>().ok())
        .or(parent_extnumber);

    // Determine the numeric value.
    let value = if let Some(v) = node.attribute("value") {
        // Direct value — takes priority over everything.
        Some(v.to_string())
    } else if let Some(bitpos) = node.attribute("bitpos") {
        // Bitmask value via bit position.
        let pos: u32 = bitpos.parse()?;
        Some(format!("0x{:016X}", 1u64 << pos))
    } else if let Some(offset) = node.attribute("offset") {
        // Extension offset formula (spec gotcha #2 in enums section).
        let ext = extnumber.ok_or_else(|| {
            anyhow::anyhow!("enum '{}' has offset but no extnumber is available", name)
        })?;
        let off: u32 = offset.parse()?;
        let dir = node.attribute("dir");
        let computed = compute_ext_enum_value(ext, off, dir);
        Some(computed.to_string())
    } else {
        // Pure alias — no independent value.
        None
    };

    Ok(RawEnum {
        name,
        value,
        api,
        type_suffix,
        alias,
        comment,
        parent_type: parent_type.map(str::to_string),
    })
}
