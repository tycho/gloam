//! C loader generator — renders minijinja templates against a `FeatureSet`.
//!
//! All generation logic lives in the `.j2` template files under
//! `src/gen/c/templates/`.  This module only handles environment setup,
//! filter registration, and file I/O.

use std::path::Path;

use anyhow::{Context, Result};
use minijinja::{Environment, Value, context};

use crate::cli::CArgs;
use crate::fetch;
use crate::resolve::FeatureSet;

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

pub fn generate(fs: &FeatureSet, args: &CArgs, out: &Path, use_fetch: bool) -> Result<()> {
    let stem = output_stem(fs);
    let env = build_env()?;

    // Output tree:
    //   {out}/include/gloam/{stem}.h
    //   {out}/include/KHR/khrplatform.h   (and other aux headers)
    //   {out}/src/{stem}.c
    let include_dir = out.join("include");
    let gloam_dir = include_dir.join("gloam");
    let src_dir = out.join("src");
    std::fs::create_dir_all(&gloam_dir)?;
    std::fs::create_dir_all(&src_dir)?;

    let ctx = context! {
        fs     => fs,
        stem   => &stem,
        guard  => format!("{}_H", stem.to_uppercase()),
        alias  => args.alias,
        loader => args.loader,
    };

    std::fs::write(
        gloam_dir.join(format!("{stem}.h")),
        env.get_template("header.h.j2")?.render(&ctx)?,
    )?;
    std::fs::write(
        src_dir.join(format!("{stem}.c")),
        env.get_template("source.c.j2")?.render(&ctx)?,
    )?;

    // Copy auxiliary headers (khrplatform.h, vk_platform.h, etc.),
    // then transitively follow any quoted #include directives found inside
    // them.  This catches implicit dependencies like vulkan_video_codecs_common.h
    // which are #include'd by other vk_video headers but never declared in the
    // XML spec.
    //
    // xxhash.h is always needed by the generated .c (extension hash search).
    // It is not spec-derived so it is not in fs.required_headers.
    let mut queue: Vec<String> = std::iter::once("xxhash.h".to_string())
        .chain(fs.required_headers.iter().cloned())
        .collect();
    let mut visited: std::collections::HashSet<String> = std::collections::HashSet::new();

    while let Some(hdr_path) = queue.pop() {
        if !visited.insert(hdr_path.clone()) {
            continue; // already copied
        }

        let dest = include_dir.join(&hdr_path);
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = fetch::load_auxiliary_header(&hdr_path, use_fetch)
            .with_context(|| format!("loading auxiliary header '{}'", hdr_path))?;
        std::fs::write(&dest, &content)?;

        // Scan for `#include "relative/path.h"` lines and enqueue them,
        // resolved relative to the directory of the current header.
        let hdr_dir = std::path::Path::new(&hdr_path)
            .parent()
            .and_then(|p| p.to_str())
            .unwrap_or("");

        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.starts_with("#include") {
                continue;
            }
            // Match the quoted form only — angle-bracket system headers are
            // not bundled and don't need copying.
            if let Some(rest) = trimmed.strip_prefix("#include") {
                let rest = rest.trim();
                if rest.starts_with('"')
                    && let Some(end) = rest[1..].find('"')
                {
                    let included = &rest[1..1 + end];
                    // Resolve relative to the including header's directory.
                    let resolved = if hdr_dir.is_empty() {
                        included.to_string()
                    } else {
                        format!("{}/{}", hdr_dir, included)
                    };
                    if !visited.contains(&resolved) {
                        queue.push(resolved);
                    }
                }
            }
        }
    }

    Ok(())
}

fn output_stem(fs: &FeatureSet) -> String {
    if fs.is_merged {
        fs.spec_name.clone()
    } else {
        fs.apis
            .first()
            .cloned()
            .unwrap_or_else(|| fs.spec_name.clone())
    }
}

// ---------------------------------------------------------------------------
// Environment
// ---------------------------------------------------------------------------

fn build_env() -> Result<Environment<'static>> {
    let mut env = Environment::new();

    env.add_template("utils.j2", include_str!("templates/utils.j2"))?;
    env.add_template("impl_util.j2", include_str!("templates/impl_util.j2"))?;
    env.add_template("hash_search.j2", include_str!("templates/hash_search.j2"))?;
    env.add_template("library.j2", include_str!("templates/library.j2"))?;
    env.add_template("loader.j2", include_str!("templates/loader.j2"))?;
    env.add_template("header.h.j2", include_str!("templates/header.h.j2"))?;
    env.add_template("source.c.j2", include_str!("templates/source.c.j2"))?;

    env.add_filter("rjust", filter_rjust);
    env.add_filter("ljust", filter_ljust);
    env.add_filter("hex4", filter_hex4);
    env.add_filter("api_display", filter_api_display);
    env.add_filter("spec_display", filter_spec_display);
    env.add_filter("c_ident", filter_c_ident);
    env.add_filter("vk_max_enum_name", filter_enum_max_name);

    Ok(env)
}

// ---------------------------------------------------------------------------
// Custom filters
// ---------------------------------------------------------------------------

/// Right-justify a value to `width` characters, padding with spaces on the left.
/// Usage in templates: `{{ value | rjust(4) }}`
fn filter_rjust(value: Value, width: usize) -> String {
    let s = value.to_string();
    format!("{s:>width$}")
}

/// Left-justify a value to `width` characters, padding with spaces on the right.
/// Usage in templates: `{{ value | ljust(4) }}`
fn filter_ljust(value: Value, width: usize) -> String {
    let s = value.to_string();
    format!("{s:<width$}")
}

/// Format a u16 packed version as a 4-digit lowercase hex literal: `0x0303`.
/// Used for packed version constants in `find_core_*` comparisons.
fn filter_hex4(value: Value) -> String {
    let n = value.as_i64().unwrap_or(0) as u64;
    format!("0x{n:04x}")
}

/// Ensure a string is a valid C identifier by prefixing with `_` if it starts
/// with a digit.  Used for struct member names: `3DFX_multisample` → `_3DFX_multisample`.
/// The macro names (e.g. `GL_3DFX_multisample`) don't need this because they
/// don't start with a digit themselves.
fn filter_c_ident(value: Value) -> String {
    let s = value.as_str().unwrap_or("");
    if s.starts_with(|c: char| c.is_ascii_digit()) {
        format!("_{s}")
    } else {
        s.to_string()
    }
}

/// Used to build public function names like `gloamLoadGLES2Context`.
fn filter_spec_display(value: Value) -> String {
    match value.as_str().unwrap_or("") {
        "gles1" | "gles2" | "gl" | "glcore" => "GL",
        "egl" => "EGL",
        "glx" => "GLX",
        "wgl" => "WGL",
        "vk" | "vulkan" => "Vulkan",
        other => return other.to_string(),
    }
    .to_string()
}

/// Used to build public function names like `gloamLoadGLES2Context`.
fn filter_api_display(value: Value) -> String {
    match value.as_str().unwrap_or("") {
        "gl" | "glcore" => "GL",
        "gles1" => "GLES1",
        "gles2" => "GLES2",
        "egl" => "EGL",
        "glx" => "GLX",
        "wgl" => "WGL",
        "vk" | "vulkan" => "Vulkan",
        other => return other.to_string(),
    }
    .to_string()
}

/// Convert a CamelCase Vulkan type name to its SCREAMING_SNAKE_CASE MAX_ENUM
/// sentinel name.  e.g. `VkDriverId` → `VK_DRIVER_ID_MAX_ENUM`.
///
/// Rule: insert `_` before any uppercase letter that is either:
///   - preceded by a lowercase letter or digit  (e.g. Driver→_Driver)
///   - preceded by an uppercase letter AND followed by a lowercase letter
///     (handles acronyms: `VkEGL` → `VK_EGL`, not `VK_E_G_L`)
fn filter_enum_max_name(value: Value) -> String {
    let name = value.as_str().unwrap_or("");
    let chars: Vec<char> = name.chars().collect();
    let mut out = String::with_capacity(name.len() + 8);

    for (i, &c) in chars.iter().enumerate() {
        if c.is_ascii_uppercase() && i > 0 {
            let prev = chars[i - 1];
            let next = chars.get(i + 1).copied();
            let split = prev.is_ascii_lowercase()
                || prev.is_ascii_digit()
                || (prev.is_ascii_uppercase() && next.is_some_and(|n| n.is_ascii_lowercase()));
            if split {
                out.push('_');
            }
        }
        out.push(c.to_ascii_uppercase());
    }

    out.push_str("_MAX_ENUM");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- filter_enum_max_name ----

    fn max(s: &str) -> String {
        filter_enum_max_name(Value::from(s))
    }

    #[test]
    fn enum_max_simple_camel() {
        assert_eq!(max("VkDriverId"), "VK_DRIVER_ID_MAX_ENUM");
    }

    #[test]
    fn enum_max_acronym_not_split() {
        // "EGL" should stay together; "Image" triggers a split after.
        assert_eq!(
            max("VkEGLImageCreateFlagBitsKHR"),
            "VK_EGL_IMAGE_CREATE_FLAG_BITS_KHR_MAX_ENUM"
        );
    }

    #[test]
    fn enum_max_trailing_acronym_not_split() {
        // Trailing uppercase run (KHR, EXT) should not be internally split.
        assert_eq!(
            max("VkSamplerAddressMode"),
            "VK_SAMPLER_ADDRESS_MODE_MAX_ENUM"
        );
    }

    #[test]
    fn enum_max_single_word() {
        assert_eq!(max("VkFormat"), "VK_FORMAT_MAX_ENUM");
    }

    // ---- filter_c_ident ----

    fn c_ident(s: &str) -> String {
        filter_c_ident(Value::from(s))
    }

    #[test]
    fn c_ident_digit_prefix_gets_underscore() {
        assert_eq!(c_ident("3DFX_multisample"), "_3DFX_multisample");
    }

    #[test]
    fn c_ident_normal_name_unchanged() {
        assert_eq!(c_ident("ARB_sync"), "ARB_sync");
        assert_eq!(c_ident("ANGLE_framebuffer_blit"), "ANGLE_framebuffer_blit");
    }

    #[test]
    fn c_ident_empty_string_unchanged() {
        assert_eq!(c_ident(""), "");
    }

    // ---- filter_api_display / filter_spec_display ----

    fn api_disp(s: &str) -> String {
        filter_api_display(Value::from(s))
    }

    fn spec_disp(s: &str) -> String {
        filter_spec_display(Value::from(s))
    }

    #[test]
    fn api_display_gl_variants() {
        assert_eq!(api_disp("gl"), "GL");
        assert_eq!(api_disp("gles1"), "GLES1");
        assert_eq!(api_disp("gles2"), "GLES2");
        assert_eq!(api_disp("glcore"), "GL");
    }

    #[test]
    fn api_display_other() {
        assert_eq!(api_disp("egl"), "EGL");
        assert_eq!(api_disp("vk"), "Vulkan");
        assert_eq!(api_disp("vulkan"), "Vulkan");
    }

    #[test]
    fn spec_display_gl_family_all_map_to_gl() {
        for api in &["gl", "gles1", "gles2", "glcore"] {
            assert_eq!(spec_disp(api), "GL", "failed for '{api}'");
        }
    }

    // ---- filter_hex4 ----

    #[test]
    fn hex4_formats_correctly() {
        assert_eq!(filter_hex4(Value::from(0x0303_i64)), "0x0303");
        assert_eq!(filter_hex4(Value::from(0x0100_i64)), "0x0100");
        assert_eq!(filter_hex4(Value::from(0_i64)), "0x0000");
    }
}
