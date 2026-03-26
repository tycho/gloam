//! XML and auxiliary header loading. Default mode uses compile-time-embedded
//! bundled copies; `--fetch` downloads from remote Khronos URLs.

// In case we built without features=fetch
#![allow(dead_code, unused)]

use anyhow::{Context, Result};

use crate::bundled;

// ---------------------------------------------------------------------------
// URL bases
// ---------------------------------------------------------------------------

const BASE_GL: &str = "https://raw.githubusercontent.com/KhronosGroup/OpenGL-Registry/main/xml/";
const BASE_EGL: &str = "https://raw.githubusercontent.com/KhronosGroup/EGL-Registry/main/api/";
const BASE_VK: &str = "https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/main/xml/";
const BASE_VK_HEADERS: &str =
    "https://raw.githubusercontent.com/KhronosGroup/Vulkan-Headers/main/include/";
const BASE_ANGLE: &str = "https://raw.githubusercontent.com/google/angle/main/scripts/";
const GLSL_EXTS_URL: &str =
    "https://raw.githubusercontent.com/tycho/gloam/refs/heads/master/bundled/xml/glsl_exts.xml";
const XXHASH_HEAD_URL: &str =
    "https://raw.githubusercontent.com/Cyan4973/xxHash/refs/heads/dev/xxhash.h";

// ---------------------------------------------------------------------------
// SpecSources
// ---------------------------------------------------------------------------

/// Raw XML text for one spec family: the primary doc plus any supplementals.
/// The parser iterates all of them in order, treating supplementals as if they
/// had been merged into the primary before parsing.
pub struct SpecSources {
    pub primary: String,
    pub supplementals: Vec<String>,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

pub fn load_spec(spec_name: &str, use_fetch: bool) -> Result<SpecSources> {
    #[cfg(feature = "fetch")]
    if use_fetch {
        return fetch_spec(spec_name);
    }
    bundled_spec(spec_name)
}

pub fn load_auxiliary_header(path: &str, use_fetch: bool) -> Result<String> {
    #[cfg(feature = "fetch")]
    if use_fetch {
        if let Some(url) = auxiliary_url(path) {
            return fetch_text(&url)
                .with_context(|| format!("fetching auxiliary header '{}'", path));
        }
        eprintln!(
            "no remote auxiliary URL for '{}'; using bundled version",
            path
        );
    }

    bundled_auxiliary(path).map(str::to_string)
}

// ---------------------------------------------------------------------------
// Bundled mode
// ---------------------------------------------------------------------------

fn bundled_spec(spec_name: &str) -> Result<SpecSources> {
    let primary = match spec_name {
        "gl" => bundled::gl_xml()?,
        "egl" => bundled::egl_xml()?,
        "glx" => bundled::glx_xml()?,
        "wgl" => bundled::wgl_xml()?,
        "vk" => bundled::vk_xml()?,
        other => anyhow::bail!("unknown spec name '{}'", other),
    }
    .to_string();

    let supplementals = match spec_name {
        "gl" => vec![
            bundled::glsl_exts_xml()?.to_string(),
            bundled::gl_angle_ext_xml()?.to_string(),
        ],
        "egl" => vec![bundled::egl_angle_ext_xml()?.to_string()],
        _ => vec![],
    };

    Ok(SpecSources {
        primary,
        supplementals,
    })
}

#[allow(dead_code)]
fn bundled_auxiliary(path: &str) -> Result<&'static str> {
    use bundled::*;
    Ok(match path {
        "xxhash.h" => XXHASH_H,
        "KHR/khrplatform.h" => KHR_PLATFORM_H,
        "EGL/eglplatform.h" => EGL_PLATFORM_H,
        "vk_platform.h" => VK_PLATFORM_H,
        "vk_video/vulkan_video_codecs_common.h" => VK_VIDEO_CODECS_COMMON_H,
        "vk_video/vulkan_video_codec_h264std.h" => VK_VIDEO_H264STD_H,
        "vk_video/vulkan_video_codec_h264std_decode.h" => VK_VIDEO_H264STD_DECODE_H,
        "vk_video/vulkan_video_codec_h264std_encode.h" => VK_VIDEO_H264STD_ENCODE_H,
        "vk_video/vulkan_video_codec_h265std.h" => VK_VIDEO_H265STD_H,
        "vk_video/vulkan_video_codec_h265std_decode.h" => VK_VIDEO_H265STD_DECODE_H,
        "vk_video/vulkan_video_codec_h265std_encode.h" => VK_VIDEO_H265STD_ENCODE_H,
        "vk_video/vulkan_video_codec_av1std.h" => VK_VIDEO_AV1STD_H,
        "vk_video/vulkan_video_codec_av1std_decode.h" => VK_VIDEO_AV1STD_DECODE_H,
        "vk_video/vulkan_video_codec_av1std_encode.h" => VK_VIDEO_AV1STD_ENCODE_H,
        "vk_video/vulkan_video_codec_vp9std.h" => VK_VIDEO_VP9STD_H,
        "vk_video/vulkan_video_codec_vp9std_decode.h" => VK_VIDEO_VP9STD_DECODE_H,
        other => anyhow::bail!("unknown auxiliary header '{}'", other),
    })
}

// ---------------------------------------------------------------------------
// Fetch mode
// ---------------------------------------------------------------------------

#[cfg(feature = "fetch")]
fn fetch_spec(spec_name: &str) -> Result<SpecSources> {
    let primary_url = match spec_name {
        "gl" => format!("{}gl.xml", BASE_GL),
        "egl" => format!("{}egl.xml", BASE_EGL),
        "glx" => format!("{}glx.xml", BASE_GL),
        "wgl" => format!("{}wgl.xml", BASE_GL),
        "vk" => format!("{}vk.xml", BASE_VK),
        other => anyhow::bail!("unknown spec name '{}'", other),
    };
    let primary = fetch_text(&primary_url)
        .with_context(|| format!("fetching primary XML for '{}'", spec_name))?;

    let supp_urls: Vec<String> = match spec_name {
        "gl" => vec![
            GLSL_EXTS_URL.to_string(),
            format!("{}gl_angle_ext.xml", BASE_ANGLE),
        ],
        "egl" => vec![format!("{}egl_angle_ext.xml", BASE_ANGLE)],
        _ => vec![],
    };

    let supplementals = supp_urls
        .iter()
        .map(|url| fetch_text(url).with_context(|| format!("fetching supplemental '{}'", url)))
        .collect::<Result<Vec<_>>>()?;

    Ok(SpecSources {
        primary,
        supplementals,
    })
}

fn auxiliary_url(path: &str) -> Option<String> {
    if path.starts_with("vk_video/") {
        Some(format!("{}{}", BASE_VK_HEADERS, path))
    } else if path == "vk_platform.h" {
        Some(format!("{}vulkan/{}", BASE_VK_HEADERS, path))
    } else if path.starts_with("KHR/") || path.starts_with("EGL/") {
        Some(format!("{}{}", BASE_EGL, path))
    } else if path == "xxhash.h" {
        Some(XXHASH_HEAD_URL.to_string())
    } else {
        None
    }
}

#[cfg(feature = "fetch")]
fn fetch_text(url: &str) -> Result<String> {
    let resp = reqwest::blocking::get(url)
        .with_context(|| format!("GET {}", url))?
        .error_for_status()
        .with_context(|| format!("HTTP error from {}", url))?;
    Ok(resp.text()?)
}

#[cfg(all(test, feature = "fetch"))]
mod tests {
    use super::*;

    /// Collect every remote URL that `--fetch` mode may request, then HEAD each
    /// one to verify it still resolves.  This catches stale base-paths and
    /// renamed upstream files before they break real generation runs.
    #[test]
    fn remote_urls_are_reachable() {
        // -- spec XMLs (fetch_spec) ------------------------------------------
        let spec_urls = vec![
            format!("{}gl.xml", BASE_GL),
            format!("{}glx.xml", BASE_GL),
            format!("{}wgl.xml", BASE_GL),
            format!("{}egl.xml", BASE_EGL),
            format!("{}vk.xml", BASE_VK),
        ];

        // -- supplemental XMLs -----------------------------------------------
        let supplemental_urls = vec![
            GLSL_EXTS_URL.to_string(),
            format!("{}gl_angle_ext.xml", BASE_ANGLE),
            format!("{}egl_angle_ext.xml", BASE_ANGLE),
        ];

        // -- auxiliary headers (auxiliary_url) --------------------------------
        // One representative URL per branch in auxiliary_url(), plus every
        // bundled vk_video header since those are dictated by the Vulkan spec
        // and new ones appear (or move) over time.
        let auxiliary_urls: Vec<String> = vec![
            "vk_platform.h",
            "KHR/khrplatform.h",
            "EGL/eglplatform.h",
            "xxhash.h",
            "vk_video/vulkan_video_codecs_common.h",
            "vk_video/vulkan_video_codec_h264std.h",
            "vk_video/vulkan_video_codec_h264std_decode.h",
            "vk_video/vulkan_video_codec_h264std_encode.h",
            "vk_video/vulkan_video_codec_h265std.h",
            "vk_video/vulkan_video_codec_h265std_decode.h",
            "vk_video/vulkan_video_codec_h265std_encode.h",
            "vk_video/vulkan_video_codec_av1std.h",
            "vk_video/vulkan_video_codec_av1std_decode.h",
            "vk_video/vulkan_video_codec_av1std_encode.h",
            "vk_video/vulkan_video_codec_vp9std.h",
            "vk_video/vulkan_video_codec_vp9std_decode.h",
        ]
        .into_iter()
        .map(|p| auxiliary_url(p).expect(&format!("no URL mapping for '{}'", p)))
        .collect();

        let all_urls = spec_urls
            .into_iter()
            .chain(supplemental_urls)
            .chain(auxiliary_urls);

        let client = reqwest::blocking::Client::new();
        let mut failures = Vec::new();

        for url in all_urls {
            let result = client
                .head(&url)
                .send()
                .and_then(|r| r.error_for_status());

            if let Err(e) = result {
                failures.push(format!("  {} — {}", url, e));
            }
        }

        assert!(
            failures.is_empty(),
            "the following remote URLs are unreachable:\n{}",
            failures.join("\n")
        );
    }
}
