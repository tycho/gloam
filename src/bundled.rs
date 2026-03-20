//! Access to bundled (compile-time-embedded) XML specs and auxiliary headers.
//!
//! Each constant is the full text of the file.  An empty string means the
//! bundled copy has not yet been populated — run `scripts/fetch_bundled.sh`
//! before building to populate them.
//!
//! At runtime, `get_*` helpers return an error with an actionable message
//! rather than silently operating on empty content.

use anyhow::{Result, bail};

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
pub const VK_PLATFORM_H: &str = include_str!("../bundled/headers/vk_platform.h");

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
// Checked accessors
// ---------------------------------------------------------------------------

fn require(name: &str, content: &'static str) -> Result<&'static str> {
    if content.is_empty() {
        bail!(
            "bundled file '{}' is empty — run `scripts/fetch_bundled.sh` to populate bundled copies",
            name
        );
    }
    Ok(content)
}

pub fn gl_xml() -> Result<&'static str> {
    require("gl.xml", GL_XML)
}
pub fn egl_xml() -> Result<&'static str> {
    require("egl.xml", EGL_XML)
}
pub fn glx_xml() -> Result<&'static str> {
    require("glx.xml", GLX_XML)
}
pub fn wgl_xml() -> Result<&'static str> {
    require("wgl.xml", WGL_XML)
}
pub fn vk_xml() -> Result<&'static str> {
    require("vk.xml", VK_XML)
}

pub fn gl_angle_ext_xml() -> Result<&'static str> {
    require("gl_angle_ext.xml", GL_ANGLE_EXT_XML)
}
pub fn egl_angle_ext_xml() -> Result<&'static str> {
    require("egl_angle_ext.xml", EGL_ANGLE_EXT_XML)
}
pub fn glsl_exts_xml() -> Result<&'static str> {
    require("glsl_exts.xml", GLSL_EXTS_XML)
}
