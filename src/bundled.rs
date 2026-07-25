//! Access to bundled (compile-time-embedded) XML specs and auxiliary headers,
//! plus their provenance manifest (`bundled/provenance.json`).
//!
//! The files live under `bundled/` as readable text (populated by
//! `cargo xtask bundle`), but are embedded in the binary as raw-DEFLATE blobs:
//! `build.rs` compresses each into OUT_DIR, and each item below inflates its
//! blob once, on first use, into a [`LazyLock<String>`].  This keeps ~6 MiB of
//! XML out of the binary as ~0.65 MiB of blobs, inflated only for the files a
//! given invocation actually touches.
//!
//! Each item yields the full text of the file.  An empty string means the
//! bundled copy has not yet been populated — run `cargo xtask bundle` (or
//! `scripts/fetch_bundled.sh`) to populate the files and their provenance.

use std::io::Read;
use std::sync::LazyLock;

use anyhow::{Context, Result};
use flate2::read::DeflateDecoder;

use crate::provenance::manifest::BundledProvenance;

/// Inflate a raw-DEFLATE blob produced by `build.rs` back into its original
/// text.  The source files are valid UTF-8 (they are read as text by the
/// bundler), so a decode failure means a corrupt build artifact, not bad input.
fn inflate(compressed: &[u8]) -> String {
    let mut out = String::new();
    DeflateDecoder::new(compressed)
        .read_to_string(&mut out)
        .expect("bundled blob failed to inflate — corrupt build artifact");
    out
}

/// Declare a bundled file: a lazily-inflated `static` holding the full text of
/// `bundled/<rel>`, embedded by `build.rs` as `OUT_DIR/bundled/<rel>.deflate`.
macro_rules! bundled_text {
    ($name:ident, $rel:literal) => {
        static $name: LazyLock<String> = LazyLock::new(|| {
            inflate(include_bytes!(concat!(
                env!("OUT_DIR"),
                "/bundled/",
                $rel,
                ".deflate"
            )))
        });
    };
}

// ---------------------------------------------------------------------------
// Primary XML specs
// ---------------------------------------------------------------------------

bundled_text!(GL_XML, "xml/gl.xml");
bundled_text!(EGL_XML, "xml/egl.xml");
bundled_text!(GLX_XML, "xml/glx.xml");
bundled_text!(WGL_XML, "xml/wgl.xml");
bundled_text!(VK_XML, "xml/vk.xml");

// ---------------------------------------------------------------------------
// Supplemental XMLs
// ---------------------------------------------------------------------------

bundled_text!(GL_ANGLE_EXT_XML, "xml/gl_angle_ext.xml");
bundled_text!(EGL_ANGLE_EXT_XML, "xml/egl_angle_ext.xml");
bundled_text!(GLSL_EXTS_XML, "xml/glsl_exts.xml");

// ---------------------------------------------------------------------------
// Auxiliary headers (passed through verbatim to the output tree)
// ---------------------------------------------------------------------------

bundled_text!(XXHASH_H, "headers/xxhash.h");
bundled_text!(KHR_PLATFORM_H, "headers/KHR/khrplatform.h");
bundled_text!(EGL_PLATFORM_H, "headers/EGL/eglplatform.h");
bundled_text!(VK_PLATFORM_H, "headers/vulkan/vk_platform.h");

bundled_text!(
    VK_VIDEO_CODECS_COMMON_H,
    "headers/vk_video/vulkan_video_codecs_common.h"
);
bundled_text!(
    VK_VIDEO_H264STD_H,
    "headers/vk_video/vulkan_video_codec_h264std.h"
);
bundled_text!(
    VK_VIDEO_H264STD_DECODE_H,
    "headers/vk_video/vulkan_video_codec_h264std_decode.h"
);
bundled_text!(
    VK_VIDEO_H264STD_ENCODE_H,
    "headers/vk_video/vulkan_video_codec_h264std_encode.h"
);
bundled_text!(
    VK_VIDEO_H265STD_H,
    "headers/vk_video/vulkan_video_codec_h265std.h"
);
bundled_text!(
    VK_VIDEO_H265STD_DECODE_H,
    "headers/vk_video/vulkan_video_codec_h265std_decode.h"
);
bundled_text!(
    VK_VIDEO_H265STD_ENCODE_H,
    "headers/vk_video/vulkan_video_codec_h265std_encode.h"
);
bundled_text!(
    VK_VIDEO_AV1STD_H,
    "headers/vk_video/vulkan_video_codec_av1std.h"
);
bundled_text!(
    VK_VIDEO_AV1STD_DECODE_H,
    "headers/vk_video/vulkan_video_codec_av1std_decode.h"
);
bundled_text!(
    VK_VIDEO_AV1STD_ENCODE_H,
    "headers/vk_video/vulkan_video_codec_av1std_encode.h"
);
bundled_text!(
    VK_VIDEO_VP9STD_H,
    "headers/vk_video/vulkan_video_codec_vp9std.h"
);
bundled_text!(
    VK_VIDEO_VP9STD_DECODE_H,
    "headers/vk_video/vulkan_video_codec_vp9std_decode.h"
);

// ---------------------------------------------------------------------------
// Bundle provenance
// ---------------------------------------------------------------------------

bundled_text!(PROVENANCE_JSON, "provenance.json");

/// Parse the embedded `bundled/provenance.json`, rejecting a schema version
/// this gloam build does not understand (an internal invariant: the bundler
/// and the binary are built from the same tree, so a mismatch means the
/// checked-in manifest was written by an incompatible tool).
pub fn bundled_provenance() -> Result<BundledProvenance> {
    let p = BundledProvenance::from_json(PROVENANCE_JSON.as_str())
        .context("parsing bundled/provenance.json")?;
    if p.schema_version != crate::provenance::manifest::SCHEMA_VERSION {
        anyhow::bail!(
            "bundled/provenance.json has schema_version {}, but this gloam build \
             understands {} — re-run `cargo xtask bundle`",
            p.schema_version,
            crate::provenance::manifest::SCHEMA_VERSION
        );
    }
    Ok(p)
}

/// Map a provenance registry key to its embedded file content, or `None` when
/// the key is unknown or its bundled copy is empty (not yet populated).
pub fn content_by_key(key: &str) -> Option<&'static str> {
    raw_content_by_key(key).filter(|c| !c.is_empty())
}

/// Like [`content_by_key`] but returns the constant even when empty; `None`
/// only for keys not in the registry.  Used to verify registry coverage.
fn raw_content_by_key(key: &str) -> Option<&'static str> {
    let content = match key {
        "gl.xml" => GL_XML.as_str(),
        "egl.xml" => EGL_XML.as_str(),
        "glx.xml" => GLX_XML.as_str(),
        "wgl.xml" => WGL_XML.as_str(),
        "vk.xml" => VK_XML.as_str(),
        "gl_angle_ext.xml" => GL_ANGLE_EXT_XML.as_str(),
        "egl_angle_ext.xml" => EGL_ANGLE_EXT_XML.as_str(),
        "glsl_exts.xml" => GLSL_EXTS_XML.as_str(),
        "xxhash.h" => XXHASH_H.as_str(),
        "KHR/khrplatform.h" => KHR_PLATFORM_H.as_str(),
        "EGL/eglplatform.h" => EGL_PLATFORM_H.as_str(),
        "vulkan/vk_platform.h" => VK_PLATFORM_H.as_str(),
        "vk_video/vulkan_video_codecs_common.h" => VK_VIDEO_CODECS_COMMON_H.as_str(),
        "vk_video/vulkan_video_codec_h264std.h" => VK_VIDEO_H264STD_H.as_str(),
        "vk_video/vulkan_video_codec_h264std_decode.h" => VK_VIDEO_H264STD_DECODE_H.as_str(),
        "vk_video/vulkan_video_codec_h264std_encode.h" => VK_VIDEO_H264STD_ENCODE_H.as_str(),
        "vk_video/vulkan_video_codec_h265std.h" => VK_VIDEO_H265STD_H.as_str(),
        "vk_video/vulkan_video_codec_h265std_decode.h" => VK_VIDEO_H265STD_DECODE_H.as_str(),
        "vk_video/vulkan_video_codec_h265std_encode.h" => VK_VIDEO_H265STD_ENCODE_H.as_str(),
        "vk_video/vulkan_video_codec_av1std.h" => VK_VIDEO_AV1STD_H.as_str(),
        "vk_video/vulkan_video_codec_av1std_decode.h" => VK_VIDEO_AV1STD_DECODE_H.as_str(),
        "vk_video/vulkan_video_codec_av1std_encode.h" => VK_VIDEO_AV1STD_ENCODE_H.as_str(),
        "vk_video/vulkan_video_codec_vp9std.h" => VK_VIDEO_VP9STD_H.as_str(),
        "vk_video/vulkan_video_codec_vp9std_decode.h" => VK_VIDEO_VP9STD_DECODE_H.as_str(),
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
        assert_eq!(
            p.schema_version,
            crate::provenance::manifest::SCHEMA_VERSION
        );
        // Empty until `cargo xtask bundle` populates it.
    }
}
