//! Parsing of `<commands>` sections into `RawCommand` records, including
//! alias-chain prototype fixup (spec gotcha #1) and Vulkan scope inference
//! (spec gotcha #12).

use anyhow::Result;
use indexmap::IndexMap;

use super::SpecDocs;
use crate::ir::{CommandScope, RawCommand, RawParam};

// Vulkan dispatchable handle types for scope inference.
const INSTANCE_HANDLES: &[&str] = &["VkInstance", "VkPhysicalDevice"];
const DEVICE_HANDLES: &[&str] = &["VkDevice", "VkQueue", "VkCommandBuffer"];

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

pub fn parse_commands(
    docs: &SpecDocs<'_, '_>,
    _spec_name: &str,
) -> Result<IndexMap<String, RawCommand>> {
    // First pass: parse all command elements, keeping unresolved aliases.
    // We store (name → RawCommand) but alias commands may still be missing
    // proto/params.
    let mut commands: IndexMap<String, RawCommand> = IndexMap::new();

    for node in docs.section_children("commands") {
        if node.tag_name().name() != "command" {
            continue;
        }
        parse_command_node(node, &mut commands)?;
    }

    // Second pass: fix up alias prototype chains (spec gotcha #1).
    // Walk chains until we find a command with a populated return_type, then
    // deep-copy that prototype/params onto the alias command.
    alias_fixup(&mut commands);

    Ok(commands)
}

// ---------------------------------------------------------------------------
// Parse a single <command> element
// ---------------------------------------------------------------------------

fn parse_command_node(
    node: roxmltree::Node<'_, '_>,
    commands: &mut IndexMap<String, RawCommand>,
) -> Result<()> {
    // Command-level alias: either an `alias=` attribute (Vulkan form) or a
    // child `<alias name="..."/>` element (GL form).  Check both.
    let cmd_alias = node.attribute("alias").map(str::to_string).or_else(|| {
        node.children()
            .find(|n| n.is_element() && n.tag_name().name() == "alias")
            .and_then(|n| n.attribute("name"))
            .map(str::to_string)
    });
    let cmd_api = node.attribute("api").map(str::to_string);

    if let Some(proto) = node
        .children()
        .find(|n| n.is_element() && n.tag_name().name() == "proto")
    {
        // Full command with prototype.
        let (name, return_type) = parse_proto(proto);
        let params = node
            .children()
            .filter(|n| n.is_element() && n.tag_name().name() == "param")
            .filter(|n| {
                // Skip params restricted to a non-Vulkan API variant (e.g. api="vulkansc").
                n.attribute("api").is_none_or(|a| {
                    a.split(',')
                        .any(|s| s.trim() == "vulkan" || s.trim() == "vulkanbase")
                })
            })
            .map(parse_param)
            .collect::<Vec<_>>();

        let cmd = RawCommand {
            name: name.clone(),
            api: cmd_api,
            return_type,
            params,
            alias: cmd_alias,
        };
        commands.entry(name).or_insert(cmd);
    } else {
        // Alias-only command: get name from `name=` attribute.
        let name = match node.attribute("name") {
            Some(n) => n.to_string(),
            None => return Ok(()), // malformed, skip
        };
        let cmd = RawCommand {
            name: name.clone(),
            api: cmd_api,
            return_type: String::new(), // to be filled by alias fixup
            params: Vec::new(),
            alias: cmd_alias,
        };
        commands.entry(name).or_insert(cmd);
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Parse <proto> element
// ---------------------------------------------------------------------------

fn parse_proto(proto: roxmltree::Node<'_, '_>) -> (String, String) {
    let mut return_type = String::new();
    let mut name = String::new();

    for child in proto.children() {
        if child.is_text() {
            return_type.push_str(child.text().unwrap_or(""));
        } else if child.is_element() {
            match child.tag_name().name() {
                "name" => {
                    name = child.text().unwrap_or("").to_string();
                    // Do NOT include <name> in return_type.
                }
                _ => {
                    // <ptype> or other sub-elements contribute to the return type.
                    return_type.push_str(child.text().unwrap_or(""));
                }
            }
        }
    }

    (name, return_type.trim().to_string())
}

// ---------------------------------------------------------------------------
// Parse <param> element
// ---------------------------------------------------------------------------

fn parse_param(param: roxmltree::Node<'_, '_>) -> RawParam {
    let api = param.attribute("api").map(str::to_string);

    // Extract the param name from the <n> child.
    let param_name = param
        .children()
        .find(|n| n.is_element() && n.tag_name().name() == "name")
        .and_then(|n| n.text())
        .unwrap_or("")
        .trim_start_matches('*')
        .to_string();

    // Extract type_name from <ptype> or <type>, falling back to raw text.
    let mut type_name = String::new();
    for child in param.children() {
        if child.is_element() && matches!(child.tag_name().name(), "ptype" | "type") {
            let t = child.text().unwrap_or("");
            type_name = t.replace("struct ", "").trim().to_string();
            break;
        }
    }

    // Build the full raw text of the param by concatenating all text content
    // in document order.  extract_raw_c already does this correctly.
    let full_raw = super::extract_raw_c(param);
    let full_trimmed = full_raw.trim();

    // Determine type_raw:
    //   - If the name appears in the text followed by non-whitespace (e.g.
    //     "[4]"), it is an array param.  type_raw = the full declaration
    //     (including the name) so resolve.rs can emit it verbatim.
    //   - Otherwise type_raw = text before the name (normal case).
    let type_raw = if !param_name.is_empty() {
        if let Some(name_pos) = full_trimmed.rfind(param_name.as_str()) {
            let after = full_trimmed[name_pos + param_name.len()..].trim();
            if after.is_empty() {
                // Normal: strip the name from type_raw.
                full_trimmed[..name_pos].trim().to_string()
            } else {
                // Array suffix present: keep full declaration.
                full_trimmed.to_string()
            }
        } else {
            full_trimmed.to_string()
        }
    } else {
        full_trimmed.to_string()
    };

    if type_name.is_empty() {
        type_name = extract_base_type(&type_raw);
    }

    RawParam {
        name: param_name,
        type_raw,
        type_name,
        api,
    }
}

/// Extract the base type name from a raw C type string.
/// e.g. "const GLubyte *" → "GLubyte", "VkInstance" → "VkInstance"
fn extract_base_type(raw: &str) -> String {
    raw.replace("const", "")
        .replace("unsigned", "")
        .replace("struct", "")
        .replace('*', "")
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_string()
}

// ---------------------------------------------------------------------------
// Alias fixup (spec gotcha #1)
// ---------------------------------------------------------------------------

fn alias_fixup(commands: &mut IndexMap<String, RawCommand>) {
    // Collect the names of commands that need fixup.
    let aliases_needing_fixup: Vec<String> = commands
        .values()
        .filter(|c| c.return_type.is_empty() && c.alias.is_some())
        .map(|c| c.name.clone())
        .collect();

    for name in aliases_needing_fixup {
        let resolved = walk_alias_chain(commands, &name);
        if let Some((ret, params)) = resolved {
            if let Some(cmd) = commands.get_mut(&name) {
                cmd.return_type = ret;
                cmd.params = params;
            }
        } else {
            eprintln!(
                "warning: could not resolve alias chain for command '{}'",
                name
            );
        }
    }
}

/// Walk the alias chain starting from `name` until a command with a non-empty
/// return_type is found.  Returns the cloned (return_type, params) or None.
fn walk_alias_chain(
    commands: &IndexMap<String, RawCommand>,
    name: &str,
) -> Option<(String, Vec<RawParam>)> {
    let mut current = name;
    let mut visited = std::collections::HashSet::new();

    loop {
        if visited.contains(current) {
            return None; // cycle
        }
        visited.insert(current);

        let cmd = commands.get(current)?;
        if !cmd.return_type.is_empty() {
            return Some((cmd.return_type.clone(), cmd.params.clone()));
        }
        current = cmd.alias.as_deref()?;
    }
}

pub fn infer_vulkan_scope(cmd: &RawCommand) -> CommandScope {
    infer_scope(cmd)
}

fn infer_scope(cmd: &RawCommand) -> CommandScope {
    // Special case: vkGetInstanceProcAddr cannot be loaded through any
    // proc-addr API — it must use dlsym.
    if cmd.name == "vkGetInstanceProcAddr" {
        return CommandScope::Unknown;
    }

    let first_type = cmd
        .params
        .first()
        .map(|p| p.type_name.as_str())
        .unwrap_or("");

    if DEVICE_HANDLES.contains(&first_type) {
        CommandScope::Device
    } else if INSTANCE_HANDLES.contains(&first_type) {
        CommandScope::Instance
    } else {
        CommandScope::Global
    }
}
