//! Access to bundled (compile-time-embedded) XML specs and auxiliary headers.
//!
//! Each constant is the full text of the file.  An empty string means the
//! bundled copy has not yet been populated — run `scripts/fetch_bundled.sh`
//! before building to populate them.
//!
//! At runtime, `get_*` helpers return an error with an actionable message
//! rather than silently operating on empty content.

use anyhow::{Context, Result};

use crate::provenance::manifest::BundledProvenance;

// ---------------------------------------------------------------------------
// Primary XML specs
// ---------------------------------------------------------------------------

pub const GL_XML: &str = include_str!("../bundled/xml/gl.xml");
pub const EGL_XML: &str = include_str!("../bundled/xml/egl.xml");
pub const GLX_XML: &str = include_str!("../bundled/xml/glx.xml");
pub const WGL_XML: &str = include_str!("../bundled/xml/wgl.xml");
pub const VK_XML: &str = include_str!("../bundled/xml/vk.xml");

// ---------------------------------------------------------------------------
// Supplemental XMLs
// ---------------------------------------------------------------------------

pub const GL_ANGLE_EXT_XML: &str = include_str!("../bundled/xml/gl_angle_ext.xml");
pub const EGL_ANGLE_EXT_XML: &str = include_str!("../bundled/xml/egl_angle_ext.xml");
pub const GLSL_EXTS_XML: &str = include_str!("../bundled/xml/glsl_exts.xml");

// ---------------------------------------------------------------------------
// Auxiliary headers (passed through verbatim to the output tree)
// ---------------------------------------------------------------------------

pub const XXHASH_H: &str = include_str!("../bundled/headers/xxhash.h");
pub const KHR_PLATFORM_H: &str = include_str!("../bundled/headers/KHR/khrplatform.h");
pub const EGL_PLATFORM_H: &str = include_str!("../bundled/headers/EGL/eglplatform.h");
pub const VK_PLATFORM_H: &str = include_str!("../bundled/headers/vulkan/vk_platform.h");

pub const VK_VIDEO_CODECS_COMMON_H: &str =
    include_str!("../bundled/headers/vk_video/vulkan_video_codecs_common.h");
pub const VK_VIDEO_H264STD_H: &str =
    include_str!("../bundled/headers/vk_video/vulkan_video_codec_h264std.h");
pub const VK_VIDEO_H264STD_DECODE_H: &str =
    include_str!("../bundled/headers/vk_video/vulkan_video_codec_h264std_decode.h");
pub const VK_VIDEO_H264STD_ENCODE_H: &str =
    include_str!("../bundled/headers/vk_video/vulkan_video_codec_h264std_encode.h");
pub const VK_VIDEO_H265STD_H: &str =
    include_str!("../bundled/headers/vk_video/vulkan_video_codec_h265std.h");
pub const VK_VIDEO_H265STD_DECODE_H: &str =
    include_str!("../bundled/headers/vk_video/vulkan_video_codec_h265std_decode.h");
pub const VK_VIDEO_H265STD_ENCODE_H: &str =
    include_str!("../bundled/headers/vk_video/vulkan_video_codec_h265std_encode.h");
pub const VK_VIDEO_AV1STD_H: &str =
    include_str!("../bundled/headers/vk_video/vulkan_video_codec_av1std.h");
pub const VK_VIDEO_AV1STD_DECODE_H: &str =
    include_str!("../bundled/headers/vk_video/vulkan_video_codec_av1std_decode.h");
pub const VK_VIDEO_AV1STD_ENCODE_H: &str =
    include_str!("../bundled/headers/vk_video/vulkan_video_codec_av1std_encode.h");
pub const VK_VIDEO_VP9STD_H: &str =
    include_str!("../bundled/headers/vk_video/vulkan_video_codec_vp9std.h");
pub const VK_VIDEO_VP9STD_DECODE_H: &str =
    include_str!("../bundled/headers/vk_video/vulkan_video_codec_vp9std_decode.h");

// ---------------------------------------------------------------------------
// Bundle provenance
// ---------------------------------------------------------------------------

/// The embedded provenance manifest describing where each bundled file came
/// from.  Populated by `cargo xtask bundle`; an empty pin set until then.
// Consumed by --version and bundled-mode provenance in later slices.
#[allow(dead_code)]
pub const PROVENANCE_JSON: &str = include_str!("../bundled/provenance.json");

/// Parse the embedded `bundled/provenance.json`.
#[allow(dead_code)]
pub fn bundled_provenance() -> Result<BundledProvenance> {
    BundledProvenance::from_json(PROVENANCE_JSON).context("parsing bundled/provenance.json")
}

/// Map a provenance registry key to its embedded file content, or `None` when
/// the key is unknown or its bundled copy is empty (not yet populated).
// Wired into cache seeding when fetch loading moves onto the engine.
#[allow(dead_code)]
pub fn content_by_key(key: &str) -> Option<&'static str> {
    raw_content_by_key(key).filter(|c| !c.is_empty())
}

/// Like [`content_by_key`] but returns the constant even when empty; `None`
/// only for keys not in the registry.  Used to verify registry coverage.
fn raw_content_by_key(key: &str) -> Option<&'static str> {
    let content = match key {
        "gl.xml" => GL_XML,
        "egl.xml" => EGL_XML,
        "glx.xml" => GLX_XML,
        "wgl.xml" => WGL_XML,
        "vk.xml" => VK_XML,
        "gl_angle_ext.xml" => GL_ANGLE_EXT_XML,
        "egl_angle_ext.xml" => EGL_ANGLE_EXT_XML,
        "glsl_exts.xml" => GLSL_EXTS_XML,
        "xxhash.h" => XXHASH_H,
        "KHR/khrplatform.h" => KHR_PLATFORM_H,
        "EGL/eglplatform.h" => EGL_PLATFORM_H,
        "vulkan/vk_platform.h" => VK_PLATFORM_H,
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
        _ => return None,
    };
    Some(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provenance::CLUSTERS;

    #[test]
    fn content_lookup_covers_every_registry_key() {
        for cluster in CLUSTERS {
            for file in cluster.files {
                assert!(
                    raw_content_by_key(file.key).is_some(),
                    "no embedded content mapping for registry key '{}'",
                    file.key
                );
            }
        }
    }

    #[test]
    fn placeholder_provenance_parses() {
        let p = bundled_provenance().expect("bundled/provenance.json parses");
        assert_eq!(p.schema_version, crate::provenance::manifest::SCHEMA_VERSION);
        // Empty until `cargo xtask bundle` populates it.
    }
}
