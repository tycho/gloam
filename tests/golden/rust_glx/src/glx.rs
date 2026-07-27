#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, clippy::all)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::ffi::{CStr, c_char, c_void};

// ── GLX base types ──────────────────────────────────────────
pub type GLintptr = isize;
pub type GLsizeiptr = isize;
pub type GLXFBConfig = *mut __GLXFBConfigRec;
pub type GLXContext = *mut __GLXcontextRec;
pub type __GLXextFuncPtr = Option<unsafe extern "system" fn()>;
pub type GLXVideoDeviceNV = u32;
pub type GLXFBConfigSGIX = *mut __GLXFBConfigRec;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GLXHyperpipeNetworkSGIX {
    pub pipeName: [c_char; 80],
    pub networkId: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GLXHyperpipeConfigSGIX {
    pub pipeName: [c_char; 80],
    pub channel: i32,
    pub participationType: u32,
    pub timeSlice: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GLXPipeRect {
    pub pipeName: [c_char; 80],
    pub srcXOrigin: i32,
    pub srcYOrigin: i32,
    pub srcWidth: i32,
    pub srcHeight: i32,
    pub destXOrigin: i32,
    pub destYOrigin: i32,
    pub destWidth: i32,
    pub destHeight: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GLXPipeRectLimits {
    pub pipeName: [c_char; 80],
    pub XOrigin: i32,
    pub YOrigin: i32,
    pub maxHeight: i32,
    pub maxWidth: i32,
}
pub type GLXFBConfigID = XID;
pub type GLXContextID = XID;
pub type GLXPixmap = XID;
pub type GLXDrawable = XID;
pub type GLXWindow = XID;
pub type GLXPbuffer = XID;
pub type GLXVideoCaptureDeviceNV = XID;
pub type GLXVideoSourceSGIX = XID;
pub type GLXFBConfigIDSGIX = XID;
pub type GLXPbufferSGIX = XID;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GLXPbufferClobberEvent {
    pub event_type: i32,
    pub draw_type: i32,
    pub serial: core::ffi::c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub drawable: GLXDrawable,
    pub buffer_mask: u32,
    pub aux_buffer: u32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub count: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GLXStereoNotifyEventEXT {
    pub type_: i32,
    pub serial: core::ffi::c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub extension: i32,
    pub evtype: i32,
    pub window: GLXDrawable,
    pub stereo_tree: Bool,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GLXBufferClobberEventSGIX {
    pub type_: i32,
    pub serial: core::ffi::c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub drawable: GLXDrawable,
    pub event_type: i32,
    pub draw_type: i32,
    pub mask: u32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub count: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GLXBufferSwapComplete {
    pub type_: i32,
    pub serial: core::ffi::c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub drawable: GLXDrawable,
    pub event_type: i32,
    pub ust: i64,
    pub msc: i64,
    pub sbc: i64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union GLXEvent {
    pub glxpbufferclobber: GLXPbufferClobberEvent,
    pub glxbufferswapcomplete: GLXBufferSwapComplete,
    pub pad: [isize; 24],
}

// Opaque C struct types (incomplete in the spec).  Zero-sized so
// pointers to them stay distinct types, exactly as in C.
#[repr(C)]
pub struct __GLXFBConfigRec {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct __GLXcontextRec {
    _opaque: [u8; 0],
}

// Platform types (in C these come from platform headers via
// khrplatform/eglplatform); declared here with their per-target
// ABI shapes.
pub type XID = core::ffi::c_ulong;
pub type Bool = i32;
#[repr(C)]
pub struct Display {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct XVisualInfo {
    _opaque: [u8; 0],
}
pub type Pixmap = core::ffi::c_ulong;
pub type Font = core::ffi::c_ulong;
pub type Window = core::ffi::c_ulong;
pub type GLubyte = u8;
pub type GLint = i32;
pub type GLbitfield = u32;
pub type GLenum = u32;
pub type Colormap = core::ffi::c_ulong;
pub type GLboolean = u8;
pub type GLuint = u32;
pub type GLsizei = i32;
pub type GLfloat = f32;
pub type Status = i32;

// ── Constants ───────────────────────────────────────────────
pub const GLX_EXTENSION_NAME: &CStr = c"GLX";
pub const GLX_VENDOR: i32 = 0x1;
pub const GLX_VERSION: i32 = 0x2;
pub const GLX_EXTENSIONS: i32 = 0x3;
pub const GLX_3DFX_WINDOW_MODE_MESA: i32 = 0x1;
pub const GLX_3DFX_FULLSCREEN_MODE_MESA: i32 = 0x2;
pub const GLX_PbufferClobber: i32 = 0;
pub const GLX_BufferSwapComplete: i32 = 1;
pub const __GLX_NUMBER_EVENTS: i32 = 17;
pub const GLX_BAD_SCREEN: i32 = 1;
pub const GLX_BAD_ATTRIBUTE: i32 = 2;
pub const GLX_NO_EXTENSION: i32 = 3;
pub const GLX_BAD_VISUAL: i32 = 4;
pub const GLX_BAD_CONTEXT: i32 = 5;
pub const GLX_BAD_VALUE: i32 = 6;
pub const GLX_BAD_ENUM: i32 = 7;
pub const GLX_BAD_HYPERPIPE_CONFIG_SGIX: i32 = 91;
pub const GLX_BAD_HYPERPIPE_SGIX: i32 = 92;
pub const GLX_STEREO_NOTIFY_EXT: i32 = 0x00000000;
pub const GLX_WINDOW_BIT: u32 = 0x00000001;
pub const GLX_WINDOW_BIT_SGIX: u32 = 0x00000001;
pub const GLX_PIXMAP_BIT: u32 = 0x00000002;
pub const GLX_PIXMAP_BIT_SGIX: u32 = 0x00000002;
pub const GLX_PBUFFER_BIT: u32 = 0x00000004;
pub const GLX_PBUFFER_BIT_SGIX: u32 = 0x00000004;
pub const GLX_RGBA_BIT: u32 = 0x00000001;
pub const GLX_RGBA_BIT_SGIX: u32 = 0x00000001;
pub const GLX_COLOR_INDEX_BIT: u32 = 0x00000002;
pub const GLX_COLOR_INDEX_BIT_SGIX: u32 = 0x00000002;
pub const GLX_RGBA_FLOAT_BIT_ARB: u32 = 0x00000004;
pub const GLX_RGBA_UNSIGNED_FLOAT_BIT_EXT: u32 = 0x00000008;
pub const GLX_SYNC_FRAME_SGIX: u32 = 0x00000000;
pub const GLX_SYNC_SWAP_SGIX: u32 = 0x00000001;
pub const GLX_STEREO_NOTIFY_MASK_EXT: u32 = 0x00000001;
pub const GLX_BUFFER_SWAP_COMPLETE_INTEL_MASK: u32 = 0x04000000;
pub const GLX_PBUFFER_CLOBBER_MASK: u32 = 0x08000000;
pub const GLX_BUFFER_CLOBBER_MASK_SGIX: u32 = 0x08000000;
pub const GLX_FRONT_LEFT_BUFFER_BIT: u32 = 0x00000001;
pub const GLX_FRONT_LEFT_BUFFER_BIT_SGIX: u32 = 0x00000001;
pub const GLX_FRONT_RIGHT_BUFFER_BIT: u32 = 0x00000002;
pub const GLX_FRONT_RIGHT_BUFFER_BIT_SGIX: u32 = 0x00000002;
pub const GLX_BACK_LEFT_BUFFER_BIT: u32 = 0x00000004;
pub const GLX_BACK_LEFT_BUFFER_BIT_SGIX: u32 = 0x00000004;
pub const GLX_BACK_RIGHT_BUFFER_BIT: u32 = 0x00000008;
pub const GLX_BACK_RIGHT_BUFFER_BIT_SGIX: u32 = 0x00000008;
pub const GLX_AUX_BUFFERS_BIT: u32 = 0x00000010;
pub const GLX_AUX_BUFFERS_BIT_SGIX: u32 = 0x00000010;
pub const GLX_DEPTH_BUFFER_BIT: u32 = 0x00000020;
pub const GLX_DEPTH_BUFFER_BIT_SGIX: u32 = 0x00000020;
pub const GLX_STENCIL_BUFFER_BIT: u32 = 0x00000040;
pub const GLX_STENCIL_BUFFER_BIT_SGIX: u32 = 0x00000040;
pub const GLX_ACCUM_BUFFER_BIT: u32 = 0x00000080;
pub const GLX_ACCUM_BUFFER_BIT_SGIX: u32 = 0x00000080;
pub const GLX_SAMPLE_BUFFERS_BIT_SGIX: u32 = 0x00000100;
pub const GLX_HYPERPIPE_DISPLAY_PIPE_SGIX: u32 = 0x00000001;
pub const GLX_HYPERPIPE_RENDER_PIPE_SGIX: u32 = 0x00000002;
pub const GLX_PIPE_RECT_SGIX: u32 = 0x00000001;
pub const GLX_PIPE_RECT_LIMITS_SGIX: u32 = 0x00000002;
pub const GLX_HYPERPIPE_STEREO_SGIX: u32 = 0x00000003;
pub const GLX_HYPERPIPE_PIXEL_AVERAGE_SGIX: u32 = 0x00000004;
pub const GLX_TEXTURE_1D_BIT_EXT: u32 = 0x00000001;
pub const GLX_TEXTURE_2D_BIT_EXT: u32 = 0x00000002;
pub const GLX_TEXTURE_RECTANGLE_BIT_EXT: u32 = 0x00000004;
pub const GLX_CONTEXT_DEBUG_BIT_ARB: u32 = 0x00000001;
pub const GLX_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: u32 = 0x00000002;
pub const GLX_CONTEXT_ROBUST_ACCESS_BIT_ARB: u32 = 0x00000004;
pub const GLX_CONTEXT_RESET_ISOLATION_BIT_ARB: u32 = 0x00000008;
pub const GLX_CONTEXT_CORE_PROFILE_BIT_ARB: u32 = 0x00000001;
pub const GLX_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: u32 = 0x00000002;
pub const GLX_CONTEXT_ES_PROFILE_BIT_EXT: u32 = 0x00000004;
pub const GLX_CONTEXT_ES2_PROFILE_BIT_EXT: u32 = 0x00000004;
pub const GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX: i32 = 80;
pub const GLX_CONTEXT_RELEASE_BEHAVIOR_NONE_ARB: i32 = 0;
pub const GLX_DONT_CARE: u32 = 0xFFFFFFFF;
pub const GLX_USE_GL: i32 = 1;
pub const GLX_BUFFER_SIZE: i32 = 2;
pub const GLX_LEVEL: i32 = 3;
pub const GLX_RGBA: i32 = 4;
pub const GLX_DOUBLEBUFFER: i32 = 5;
pub const GLX_STEREO: i32 = 6;
pub const GLX_AUX_BUFFERS: i32 = 7;
pub const GLX_RED_SIZE: i32 = 8;
pub const GLX_GREEN_SIZE: i32 = 9;
pub const GLX_BLUE_SIZE: i32 = 10;
pub const GLX_ALPHA_SIZE: i32 = 11;
pub const GLX_DEPTH_SIZE: i32 = 12;
pub const GLX_STENCIL_SIZE: i32 = 13;
pub const GLX_ACCUM_RED_SIZE: i32 = 14;
pub const GLX_ACCUM_GREEN_SIZE: i32 = 15;
pub const GLX_ACCUM_BLUE_SIZE: i32 = 16;
pub const GLX_ACCUM_ALPHA_SIZE: i32 = 17;
pub const GLX_CONFIG_CAVEAT: i32 = 0x20;
pub const GLX_VISUAL_CAVEAT_EXT: i32 = 0x20;
pub const GLX_X_VISUAL_TYPE: i32 = 0x22;
pub const GLX_X_VISUAL_TYPE_EXT: i32 = 0x22;
pub const GLX_TRANSPARENT_TYPE: i32 = 0x23;
pub const GLX_TRANSPARENT_TYPE_EXT: i32 = 0x23;
pub const GLX_TRANSPARENT_INDEX_VALUE: i32 = 0x24;
pub const GLX_TRANSPARENT_INDEX_VALUE_EXT: i32 = 0x24;
pub const GLX_TRANSPARENT_RED_VALUE: i32 = 0x25;
pub const GLX_TRANSPARENT_RED_VALUE_EXT: i32 = 0x25;
pub const GLX_TRANSPARENT_GREEN_VALUE: i32 = 0x26;
pub const GLX_TRANSPARENT_GREEN_VALUE_EXT: i32 = 0x26;
pub const GLX_TRANSPARENT_BLUE_VALUE: i32 = 0x27;
pub const GLX_TRANSPARENT_BLUE_VALUE_EXT: i32 = 0x27;
pub const GLX_TRANSPARENT_ALPHA_VALUE: i32 = 0x28;
pub const GLX_TRANSPARENT_ALPHA_VALUE_EXT: i32 = 0x28;
pub const GLX_GPU_VENDOR_AMD: i32 = 0x1F00;
pub const GLX_GPU_RENDERER_STRING_AMD: i32 = 0x1F01;
pub const GLX_GPU_OPENGL_VERSION_STRING_AMD: i32 = 0x1F02;
pub const GLX_CONTEXT_MAJOR_VERSION_ARB: i32 = 0x2091;
pub const GLX_CONTEXT_MINOR_VERSION_ARB: i32 = 0x2092;
pub const GLX_CONTEXT_FLAGS_ARB: i32 = 0x2094;
pub const GLX_CONTEXT_ALLOW_BUFFER_BYTE_ORDER_MISMATCH_ARB: i32 = 0x2095;
pub const GLX_CONTEXT_RELEASE_BEHAVIOR_ARB: i32 = 0x2097;
pub const GLX_CONTEXT_RELEASE_BEHAVIOR_FLUSH_ARB: i32 = 0x2098;
pub const GLX_CONTEXT_MULTIGPU_ATTRIB_NV: i32 = 0x20AA;
pub const GLX_CONTEXT_MULTIGPU_ATTRIB_SINGLE_NV: i32 = 0x20AB;
pub const GLX_CONTEXT_MULTIGPU_ATTRIB_AFR_NV: i32 = 0x20AC;
pub const GLX_CONTEXT_MULTIGPU_ATTRIB_MULTICAST_NV: i32 = 0x20AD;
pub const GLX_CONTEXT_MULTIGPU_ATTRIB_MULTI_DISPLAY_MULTICAST_NV: i32 = 0x20AE;
pub const GLX_FLOAT_COMPONENTS_NV: i32 = 0x20B0;
pub const GLX_RGBA_UNSIGNED_FLOAT_TYPE_EXT: i32 = 0x20B1;
pub const GLX_FRAMEBUFFER_SRGB_CAPABLE_ARB: i32 = 0x20B2;
pub const GLX_FRAMEBUFFER_SRGB_CAPABLE_EXT: i32 = 0x20B2;
pub const GLX_COLOR_SAMPLES_NV: i32 = 0x20B3;
pub const GLX_RGBA_FLOAT_TYPE_ARB: i32 = 0x20B9;
pub const GLX_VIDEO_OUT_COLOR_NV: i32 = 0x20C3;
pub const GLX_VIDEO_OUT_ALPHA_NV: i32 = 0x20C4;
pub const GLX_VIDEO_OUT_DEPTH_NV: i32 = 0x20C5;
pub const GLX_VIDEO_OUT_COLOR_AND_ALPHA_NV: i32 = 0x20C6;
pub const GLX_VIDEO_OUT_COLOR_AND_DEPTH_NV: i32 = 0x20C7;
pub const GLX_VIDEO_OUT_FRAME_NV: i32 = 0x20C8;
pub const GLX_VIDEO_OUT_FIELD_1_NV: i32 = 0x20C9;
pub const GLX_VIDEO_OUT_FIELD_2_NV: i32 = 0x20CA;
pub const GLX_VIDEO_OUT_STACKED_FIELDS_1_2_NV: i32 = 0x20CB;
pub const GLX_VIDEO_OUT_STACKED_FIELDS_2_1_NV: i32 = 0x20CC;
pub const GLX_DEVICE_ID_NV: i32 = 0x20CD;
pub const GLX_UNIQUE_ID_NV: i32 = 0x20CE;
pub const GLX_NUM_VIDEO_CAPTURE_SLOTS_NV: i32 = 0x20CF;
pub const GLX_BIND_TO_TEXTURE_RGB_EXT: i32 = 0x20D0;
pub const GLX_BIND_TO_TEXTURE_RGBA_EXT: i32 = 0x20D1;
pub const GLX_BIND_TO_MIPMAP_TEXTURE_EXT: i32 = 0x20D2;
pub const GLX_BIND_TO_TEXTURE_TARGETS_EXT: i32 = 0x20D3;
pub const GLX_Y_INVERTED_EXT: i32 = 0x20D4;
pub const GLX_TEXTURE_FORMAT_EXT: i32 = 0x20D5;
pub const GLX_TEXTURE_TARGET_EXT: i32 = 0x20D6;
pub const GLX_MIPMAP_TEXTURE_EXT: i32 = 0x20D7;
pub const GLX_TEXTURE_FORMAT_NONE_EXT: i32 = 0x20D8;
pub const GLX_TEXTURE_FORMAT_RGB_EXT: i32 = 0x20D9;
pub const GLX_TEXTURE_FORMAT_RGBA_EXT: i32 = 0x20DA;
pub const GLX_TEXTURE_1D_EXT: i32 = 0x20DB;
pub const GLX_TEXTURE_2D_EXT: i32 = 0x20DC;
pub const GLX_TEXTURE_RECTANGLE_EXT: i32 = 0x20DD;
pub const GLX_FRONT_LEFT_EXT: i32 = 0x20DE;
pub const GLX_FRONT_RIGHT_EXT: i32 = 0x20DF;
pub const GLX_BACK_LEFT_EXT: i32 = 0x20E0;
pub const GLX_BACK_RIGHT_EXT: i32 = 0x20E1;
pub const GLX_FRONT_EXT: i32 = 0x20DE;
pub const GLX_BACK_EXT: i32 = 0x20E0;
pub const GLX_AUX0_EXT: i32 = 0x20E2;
pub const GLX_AUX1_EXT: i32 = 0x20E3;
pub const GLX_AUX2_EXT: i32 = 0x20E4;
pub const GLX_AUX3_EXT: i32 = 0x20E5;
pub const GLX_AUX4_EXT: i32 = 0x20E6;
pub const GLX_AUX5_EXT: i32 = 0x20E7;
pub const GLX_AUX6_EXT: i32 = 0x20E8;
pub const GLX_AUX7_EXT: i32 = 0x20E9;
pub const GLX_AUX8_EXT: i32 = 0x20EA;
pub const GLX_AUX9_EXT: i32 = 0x20EB;
pub const GLX_NUM_VIDEO_SLOTS_NV: i32 = 0x20F0;
pub const GLX_SWAP_INTERVAL_EXT: i32 = 0x20F1;
pub const GLX_MAX_SWAP_INTERVAL_EXT: i32 = 0x20F2;
pub const GLX_LATE_SWAPS_TEAR_EXT: i32 = 0x20F3;
pub const GLX_BACK_BUFFER_AGE_EXT: i32 = 0x20F4;
pub const GLX_STEREO_TREE_EXT: i32 = 0x20F5;
pub const GLX_VENDOR_NAMES_EXT: i32 = 0x20F6;
pub const GLX_GENERATE_RESET_ON_VIDEO_MEMORY_PURGE_NV: i32 = 0x20F7;
pub const GLX_GPU_FASTEST_TARGET_GPUS_AMD: i32 = 0x21A2;
pub const GLX_GPU_RAM_AMD: i32 = 0x21A3;
pub const GLX_GPU_CLOCK_AMD: i32 = 0x21A4;
pub const GLX_GPU_NUM_PIPES_AMD: i32 = 0x21A5;
pub const GLX_GPU_NUM_SIMD_AMD: i32 = 0x21A6;
pub const GLX_GPU_NUM_RB_AMD: i32 = 0x21A7;
pub const GLX_GPU_NUM_SPI_AMD: i32 = 0x21A8;
pub const GLX_CONTEXT_PRIORITY_LEVEL_EXT: i32 = 0x3100;
pub const GLX_CONTEXT_PRIORITY_HIGH_EXT: i32 = 0x3101;
pub const GLX_CONTEXT_PRIORITY_MEDIUM_EXT: i32 = 0x3102;
pub const GLX_CONTEXT_PRIORITY_LOW_EXT: i32 = 0x3103;
pub const GLX_CONTEXT_OPENGL_NO_ERROR_ARB: i32 = 0x31B3;
pub const GLX_NONE: i32 = 0x8000;
pub const GLX_SLOW_CONFIG: i32 = 0x8001;
pub const GLX_TRUE_COLOR: i32 = 0x8002;
pub const GLX_DIRECT_COLOR: i32 = 0x8003;
pub const GLX_PSEUDO_COLOR: i32 = 0x8004;
pub const GLX_STATIC_COLOR: i32 = 0x8005;
pub const GLX_GRAY_SCALE: i32 = 0x8006;
pub const GLX_STATIC_GRAY: i32 = 0x8007;
pub const GLX_TRANSPARENT_RGB: i32 = 0x8008;
pub const GLX_TRANSPARENT_INDEX: i32 = 0x8009;
pub const GLX_VISUAL_ID: i32 = 0x800B;
pub const GLX_SCREEN: i32 = 0x800C;
pub const GLX_NON_CONFORMANT_CONFIG: i32 = 0x800D;
pub const GLX_DRAWABLE_TYPE: i32 = 0x8010;
pub const GLX_RENDER_TYPE: i32 = 0x8011;
pub const GLX_X_RENDERABLE: i32 = 0x8012;
pub const GLX_FBCONFIG_ID: i32 = 0x8013;
pub const GLX_RGBA_TYPE: i32 = 0x8014;
pub const GLX_COLOR_INDEX_TYPE: i32 = 0x8015;
pub const GLX_MAX_PBUFFER_WIDTH: i32 = 0x8016;
pub const GLX_MAX_PBUFFER_HEIGHT: i32 = 0x8017;
pub const GLX_MAX_PBUFFER_PIXELS: i32 = 0x8018;
pub const GLX_PRESERVED_CONTENTS: i32 = 0x801B;
pub const GLX_LARGEST_PBUFFER: i32 = 0x801C;
pub const GLX_WIDTH: i32 = 0x801D;
pub const GLX_HEIGHT: i32 = 0x801E;
pub const GLX_EVENT_MASK: i32 = 0x801F;
pub const GLX_DAMAGED: i32 = 0x8020;
pub const GLX_SAVED: i32 = 0x8021;
pub const GLX_WINDOW: i32 = 0x8022;
pub const GLX_PBUFFER: i32 = 0x8023;
pub const GLX_NONE_EXT: i32 = 0x8000;
pub const GLX_SLOW_VISUAL_EXT: i32 = 0x8001;
pub const GLX_TRUE_COLOR_EXT: i32 = 0x8002;
pub const GLX_DIRECT_COLOR_EXT: i32 = 0x8003;
pub const GLX_PSEUDO_COLOR_EXT: i32 = 0x8004;
pub const GLX_STATIC_COLOR_EXT: i32 = 0x8005;
pub const GLX_GRAY_SCALE_EXT: i32 = 0x8006;
pub const GLX_STATIC_GRAY_EXT: i32 = 0x8007;
pub const GLX_TRANSPARENT_RGB_EXT: i32 = 0x8008;
pub const GLX_TRANSPARENT_INDEX_EXT: i32 = 0x8009;
pub const GLX_SHARE_CONTEXT_EXT: i32 = 0x800A;
pub const GLX_VISUAL_ID_EXT: i32 = 0x800B;
pub const GLX_SCREEN_EXT: i32 = 0x800C;
pub const GLX_NON_CONFORMANT_VISUAL_EXT: i32 = 0x800D;
pub const GLX_DRAWABLE_TYPE_SGIX: i32 = 0x8010;
pub const GLX_RENDER_TYPE_SGIX: i32 = 0x8011;
pub const GLX_X_RENDERABLE_SGIX: i32 = 0x8012;
pub const GLX_FBCONFIG_ID_SGIX: i32 = 0x8013;
pub const GLX_RGBA_TYPE_SGIX: i32 = 0x8014;
pub const GLX_COLOR_INDEX_TYPE_SGIX: i32 = 0x8015;
pub const GLX_MAX_PBUFFER_WIDTH_SGIX: i32 = 0x8016;
pub const GLX_MAX_PBUFFER_HEIGHT_SGIX: i32 = 0x8017;
pub const GLX_MAX_PBUFFER_PIXELS_SGIX: i32 = 0x8018;
pub const GLX_OPTIMAL_PBUFFER_WIDTH_SGIX: i32 = 0x8019;
pub const GLX_OPTIMAL_PBUFFER_HEIGHT_SGIX: i32 = 0x801A;
pub const GLX_PRESERVED_CONTENTS_SGIX: i32 = 0x801B;
pub const GLX_LARGEST_PBUFFER_SGIX: i32 = 0x801C;
pub const GLX_WIDTH_SGIX: i32 = 0x801D;
pub const GLX_HEIGHT_SGIX: i32 = 0x801E;
pub const GLX_EVENT_MASK_SGIX: i32 = 0x801F;
pub const GLX_DAMAGED_SGIX: i32 = 0x8020;
pub const GLX_SAVED_SGIX: i32 = 0x8021;
pub const GLX_WINDOW_SGIX: i32 = 0x8022;
pub const GLX_PBUFFER_SGIX: i32 = 0x8023;
pub const GLX_BLENDED_RGBA_SGIS: i32 = 0x8025;
pub const GLX_MULTISAMPLE_SUB_RECT_WIDTH_SGIS: i32 = 0x8026;
pub const GLX_MULTISAMPLE_SUB_RECT_HEIGHT_SGIS: i32 = 0x8027;
pub const GLX_VISUAL_SELECT_GROUP_SGIX: i32 = 0x8028;
pub const GLX_HYPERPIPE_ID_SGIX: i32 = 0x8030;
pub const GLX_PBUFFER_HEIGHT: i32 = 0x8040;
pub const GLX_PBUFFER_WIDTH: i32 = 0x8041;
pub const GLX_SAMPLE_BUFFERS_3DFX: i32 = 0x8050;
pub const GLX_SAMPLES_3DFX: i32 = 0x8051;
pub const GLX_SWAP_METHOD_OML: i32 = 0x8060;
pub const GLX_SWAP_EXCHANGE_OML: i32 = 0x8061;
pub const GLX_SWAP_COPY_OML: i32 = 0x8062;
pub const GLX_SWAP_UNDEFINED_OML: i32 = 0x8063;
pub const GLX_EXCHANGE_COMPLETE_INTEL: i32 = 0x8180;
pub const GLX_COPY_COMPLETE_INTEL: i32 = 0x8181;
pub const GLX_FLIP_COMPLETE_INTEL: i32 = 0x8182;
pub const GLX_RENDERER_VENDOR_ID_MESA: i32 = 0x8183;
pub const GLX_RENDERER_DEVICE_ID_MESA: i32 = 0x8184;
pub const GLX_RENDERER_VERSION_MESA: i32 = 0x8185;
pub const GLX_RENDERER_ACCELERATED_MESA: i32 = 0x8186;
pub const GLX_RENDERER_VIDEO_MEMORY_MESA: i32 = 0x8187;
pub const GLX_RENDERER_UNIFIED_MEMORY_ARCHITECTURE_MESA: i32 = 0x8188;
pub const GLX_RENDERER_PREFERRED_PROFILE_MESA: i32 = 0x8189;
pub const GLX_RENDERER_OPENGL_CORE_PROFILE_VERSION_MESA: i32 = 0x818A;
pub const GLX_RENDERER_OPENGL_COMPATIBILITY_PROFILE_VERSION_MESA: i32 = 0x818B;
pub const GLX_RENDERER_OPENGL_ES_PROFILE_VERSION_MESA: i32 = 0x818C;
pub const GLX_RENDERER_OPENGL_ES2_PROFILE_VERSION_MESA: i32 = 0x818D;
pub const GLX_LOSE_CONTEXT_ON_RESET_ARB: i32 = 0x8252;
pub const GLX_CONTEXT_RESET_NOTIFICATION_STRATEGY_ARB: i32 = 0x8256;
pub const GLX_NO_RESET_NOTIFICATION_ARB: i32 = 0x8261;
pub const GLX_CONTEXT_PROFILE_MASK_ARB: i32 = 0x9126;
pub const GLX_SAMPLE_BUFFERS: i32 = 100000;
pub const GLX_SAMPLE_BUFFERS_ARB: i32 = 100000;
pub const GLX_SAMPLE_BUFFERS_SGIS: i32 = 100000;
pub const GLX_SAMPLES: i32 = 100001;
pub const GLX_SAMPLES_ARB: i32 = 100001;
pub const GLX_SAMPLES_SGIS: i32 = 100001;
pub const GLX_COVERAGE_SAMPLES_NV: i32 = 100001;

// ── Command table ───────────────────────────────────────────
pub const COMMAND_COUNT: usize = 131;
pub const FEATURE_COUNT: usize = 5;

#[rustfmt::skip]
static FN_NAME_DATA: &[u8] = b"\
    glXChooseVisual\0\
    glXCopyContext\0\
    glXCreateContext\0\
    glXCreateGLXPixmap\0\
    glXDestroyContext\0\
    glXDestroyGLXPixmap\0\
    glXGetConfig\0\
    glXGetCurrentContext\0\
    glXGetCurrentDrawable\0\
    glXIsDirect\0\
    glXMakeCurrent\0\
    glXQueryExtension\0\
    glXQueryVersion\0\
    glXSwapBuffers\0\
    glXUseXFont\0\
    glXWaitGL\0\
    glXWaitX\0\
    glXGetClientString\0\
    glXQueryExtensionsString\0\
    glXQueryServerString\0\
    glXGetCurrentDisplay\0\
    glXChooseFBConfig\0\
    glXCreateNewContext\0\
    glXCreatePbuffer\0\
    glXCreatePixmap\0\
    glXCreateWindow\0\
    glXDestroyPbuffer\0\
    glXDestroyPixmap\0\
    glXDestroyWindow\0\
    glXGetCurrentReadDrawable\0\
    glXGetFBConfigAttrib\0\
    glXGetFBConfigs\0\
    glXGetSelectedEvent\0\
    glXGetVisualFromFBConfig\0\
    glXMakeContextCurrent\0\
    glXQueryContext\0\
    glXQueryDrawable\0\
    glXSelectEvent\0\
    glXGetProcAddress\0\
    glXBlitContextFramebufferAMD\0\
    glXCreateAssociatedContextAMD\0\
    glXCreateAssociatedContextAttribsAMD\0\
    glXDeleteAssociatedContextAMD\0\
    glXGetContextGPUIDAMD\0\
    glXGetCurrentAssociatedContextAMD\0\
    glXGetGPUIDsAMD\0\
    glXGetGPUInfoAMD\0\
    glXMakeAssociatedContextCurrentAMD\0\
    glXCreateContextAttribsARB\0\
    glXGetProcAddressARB\0\
    glXFreeContextEXT\0\
    glXGetContextIDEXT\0\
    glXGetCurrentDisplayEXT\0\
    glXImportContextEXT\0\
    glXQueryContextInfoEXT\0\
    glXSwapIntervalEXT\0\
    glXBindTexImageEXT\0\
    glXReleaseTexImageEXT\0\
    glXGetAGPOffsetMESA\0\
    glXCopySubBufferMESA\0\
    glXCreateGLXPixmapMESA\0\
    glXQueryCurrentRendererIntegerMESA\0\
    glXQueryCurrentRendererStringMESA\0\
    glXQueryRendererIntegerMESA\0\
    glXQueryRendererStringMESA\0\
    glXReleaseBuffersMESA\0\
    glXSet3DfxModeMESA\0\
    glXGetSwapIntervalMESA\0\
    glXSwapIntervalMESA\0\
    glXCopyBufferSubDataNV\0\
    glXNamedCopyBufferSubDataNV\0\
    glXCopyImageSubDataNV\0\
    glXDelayBeforeSwapNV\0\
    glXBindVideoDeviceNV\0\
    glXEnumerateVideoDevicesNV\0\
    glXBindSwapBarrierNV\0\
    glXJoinSwapGroupNV\0\
    glXQueryFrameCountNV\0\
    glXQueryMaxSwapGroupsNV\0\
    glXQuerySwapGroupNV\0\
    glXResetFrameCountNV\0\
    glXBindVideoCaptureDeviceNV\0\
    glXEnumerateVideoCaptureDevicesNV\0\
    glXLockVideoCaptureDeviceNV\0\
    glXQueryVideoCaptureDeviceNV\0\
    glXReleaseVideoCaptureDeviceNV\0\
    glXBindVideoImageNV\0\
    glXGetVideoDeviceNV\0\
    glXGetVideoInfoNV\0\
    glXReleaseVideoDeviceNV\0\
    glXReleaseVideoImageNV\0\
    glXSendPbufferToVideoNV\0\
    glXGetMscRateOML\0\
    glXGetSyncValuesOML\0\
    glXSwapBuffersMscOML\0\
    glXWaitForMscOML\0\
    glXWaitForSbcOML\0\
    glXCushionSGI\0\
    glXGetCurrentReadDrawableSGI\0\
    glXMakeCurrentReadSGI\0\
    glXSwapIntervalSGI\0\
    glXGetVideoSyncSGI\0\
    glXWaitVideoSyncSGI\0\
    glXChooseFBConfigSGIX\0\
    glXCreateContextWithConfigSGIX\0\
    glXCreateGLXPixmapWithConfigSGIX\0\
    glXGetFBConfigAttribSGIX\0\
    glXGetFBConfigFromVisualSGIX\0\
    glXGetVisualFromFBConfigSGIX\0\
    glXBindHyperpipeSGIX\0\
    glXDestroyHyperpipeConfigSGIX\0\
    glXHyperpipeAttribSGIX\0\
    glXHyperpipeConfigSGIX\0\
    glXQueryHyperpipeAttribSGIX\0\
    glXQueryHyperpipeBestAttribSGIX\0\
    glXQueryHyperpipeConfigSGIX\0\
    glXQueryHyperpipeNetworkSGIX\0\
    glXCreateGLXPbufferSGIX\0\
    glXDestroyGLXPbufferSGIX\0\
    glXGetSelectedEventSGIX\0\
    glXQueryGLXPbufferSGIX\0\
    glXSelectEventSGIX\0\
    glXBindSwapBarrierSGIX\0\
    glXQueryMaxSwapBarriersSGIX\0\
    glXJoinSwapGroupSGIX\0\
    glXBindChannelToWindowSGIX\0\
    glXChannelRectSGIX\0\
    glXChannelRectSyncSGIX\0\
    glXQueryChannelDeltasSGIX\0\
    glXQueryChannelRectSGIX\0\
    glXGetTransparentIndexSUN\0\
";

// Byte offset of each command name in FN_NAME_DATA, indexed in
// lockstep with the pfn table (slot [i] == command i).
#[rustfmt::skip]
static FN_NAME_OFFSETS: [u16; COMMAND_COUNT] = [
          0, // [0] glXChooseVisual
         16, // [1] glXCopyContext
         31, // [2] glXCreateContext
         48, // [3] glXCreateGLXPixmap
         67, // [4] glXDestroyContext
         85, // [5] glXDestroyGLXPixmap
        105, // [6] glXGetConfig
        118, // [7] glXGetCurrentContext
        139, // [8] glXGetCurrentDrawable
        161, // [9] glXIsDirect
        173, // [10] glXMakeCurrent
        188, // [11] glXQueryExtension
        206, // [12] glXQueryVersion
        222, // [13] glXSwapBuffers
        237, // [14] glXUseXFont
        249, // [15] glXWaitGL
        259, // [16] glXWaitX
        268, // [17] glXGetClientString
        287, // [18] glXQueryExtensionsString
        312, // [19] glXQueryServerString
        333, // [20] glXGetCurrentDisplay
        354, // [21] glXChooseFBConfig
        372, // [22] glXCreateNewContext
        392, // [23] glXCreatePbuffer
        409, // [24] glXCreatePixmap
        425, // [25] glXCreateWindow
        441, // [26] glXDestroyPbuffer
        459, // [27] glXDestroyPixmap
        476, // [28] glXDestroyWindow
        493, // [29] glXGetCurrentReadDrawable
        519, // [30] glXGetFBConfigAttrib
        540, // [31] glXGetFBConfigs
        556, // [32] glXGetSelectedEvent
        576, // [33] glXGetVisualFromFBConfig
        601, // [34] glXMakeContextCurrent
        623, // [35] glXQueryContext
        639, // [36] glXQueryDrawable
        656, // [37] glXSelectEvent
        671, // [38] glXGetProcAddress
        689, // [39] glXBlitContextFramebufferAMD
        718, // [40] glXCreateAssociatedContextAMD
        748, // [41] glXCreateAssociatedContextAttribsAMD
        785, // [42] glXDeleteAssociatedContextAMD
        815, // [43] glXGetContextGPUIDAMD
        837, // [44] glXGetCurrentAssociatedContextAMD
        871, // [45] glXGetGPUIDsAMD
        887, // [46] glXGetGPUInfoAMD
        904, // [47] glXMakeAssociatedContextCurrentAMD
        939, // [48] glXCreateContextAttribsARB
        966, // [49] glXGetProcAddressARB
        987, // [50] glXFreeContextEXT
       1005, // [51] glXGetContextIDEXT
       1024, // [52] glXGetCurrentDisplayEXT
       1048, // [53] glXImportContextEXT
       1068, // [54] glXQueryContextInfoEXT
       1091, // [55] glXSwapIntervalEXT
       1110, // [56] glXBindTexImageEXT
       1129, // [57] glXReleaseTexImageEXT
       1151, // [58] glXGetAGPOffsetMESA
       1171, // [59] glXCopySubBufferMESA
       1192, // [60] glXCreateGLXPixmapMESA
       1215, // [61] glXQueryCurrentRendererIntegerMESA
       1250, // [62] glXQueryCurrentRendererStringMESA
       1284, // [63] glXQueryRendererIntegerMESA
       1312, // [64] glXQueryRendererStringMESA
       1339, // [65] glXReleaseBuffersMESA
       1361, // [66] glXSet3DfxModeMESA
       1380, // [67] glXGetSwapIntervalMESA
       1403, // [68] glXSwapIntervalMESA
       1423, // [69] glXCopyBufferSubDataNV
       1446, // [70] glXNamedCopyBufferSubDataNV
       1474, // [71] glXCopyImageSubDataNV
       1496, // [72] glXDelayBeforeSwapNV
       1517, // [73] glXBindVideoDeviceNV
       1538, // [74] glXEnumerateVideoDevicesNV
       1565, // [75] glXBindSwapBarrierNV
       1586, // [76] glXJoinSwapGroupNV
       1605, // [77] glXQueryFrameCountNV
       1626, // [78] glXQueryMaxSwapGroupsNV
       1650, // [79] glXQuerySwapGroupNV
       1670, // [80] glXResetFrameCountNV
       1691, // [81] glXBindVideoCaptureDeviceNV
       1719, // [82] glXEnumerateVideoCaptureDevicesNV
       1753, // [83] glXLockVideoCaptureDeviceNV
       1781, // [84] glXQueryVideoCaptureDeviceNV
       1810, // [85] glXReleaseVideoCaptureDeviceNV
       1841, // [86] glXBindVideoImageNV
       1861, // [87] glXGetVideoDeviceNV
       1881, // [88] glXGetVideoInfoNV
       1899, // [89] glXReleaseVideoDeviceNV
       1923, // [90] glXReleaseVideoImageNV
       1946, // [91] glXSendPbufferToVideoNV
       1970, // [92] glXGetMscRateOML
       1987, // [93] glXGetSyncValuesOML
       2007, // [94] glXSwapBuffersMscOML
       2028, // [95] glXWaitForMscOML
       2045, // [96] glXWaitForSbcOML
       2062, // [97] glXCushionSGI
       2076, // [98] glXGetCurrentReadDrawableSGI
       2105, // [99] glXMakeCurrentReadSGI
       2127, // [100] glXSwapIntervalSGI
       2146, // [101] glXGetVideoSyncSGI
       2165, // [102] glXWaitVideoSyncSGI
       2185, // [103] glXChooseFBConfigSGIX
       2207, // [104] glXCreateContextWithConfigSGIX
       2238, // [105] glXCreateGLXPixmapWithConfigSGIX
       2271, // [106] glXGetFBConfigAttribSGIX
       2296, // [107] glXGetFBConfigFromVisualSGIX
       2325, // [108] glXGetVisualFromFBConfigSGIX
       2354, // [109] glXBindHyperpipeSGIX
       2375, // [110] glXDestroyHyperpipeConfigSGIX
       2405, // [111] glXHyperpipeAttribSGIX
       2428, // [112] glXHyperpipeConfigSGIX
       2451, // [113] glXQueryHyperpipeAttribSGIX
       2479, // [114] glXQueryHyperpipeBestAttribSGIX
       2511, // [115] glXQueryHyperpipeConfigSGIX
       2539, // [116] glXQueryHyperpipeNetworkSGIX
       2568, // [117] glXCreateGLXPbufferSGIX
       2592, // [118] glXDestroyGLXPbufferSGIX
       2617, // [119] glXGetSelectedEventSGIX
       2641, // [120] glXQueryGLXPbufferSGIX
       2664, // [121] glXSelectEventSGIX
       2683, // [122] glXBindSwapBarrierSGIX
       2706, // [123] glXQueryMaxSwapBarriersSGIX
       2734, // [124] glXJoinSwapGroupSGIX
       2755, // [125] glXBindChannelToWindowSGIX
       2782, // [126] glXChannelRectSGIX
       2801, // [127] glXChannelRectSyncSGIX
       2824, // [128] glXQueryChannelDeltasSGIX
       2850, // [129] glXQueryChannelRectSGIX
       2874, // [130] glXGetTransparentIndexSUN
];

#[rustfmt::skip]
static FEATURE_RANGES: [(u16, u16, u16); 5] = [
    (   0,    0,   17), // GLX_VERSION_1_0
    (   1,   17,    3), // GLX_VERSION_1_1
    (   2,   20,    1), // GLX_VERSION_1_2
    (   3,   21,   17), // GLX_VERSION_1_3
    (   4,   38,    1), // GLX_VERSION_1_4
];

#[rustfmt::skip]
static EXT_RANGES_glx: [(u16, u16, u16); 32] = [
    (   1,   39,    9), // GLX_AMD_gpu_association
    (   3,   48,    1), // GLX_ARB_create_context
    (   9,   49,    1), // GLX_ARB_get_proc_address
    (  21,   50,    5), // GLX_EXT_import_context
    (  25,   55,    1), // GLX_EXT_swap_control
    (  27,   56,    2), // GLX_EXT_texture_from_pixmap
    (  31,   58,    1), // GLX_MESA_agp_offset
    (  32,   59,    1), // GLX_MESA_copy_sub_buffer
    (  33,   60,    1), // GLX_MESA_pixmap_colormap
    (  34,   61,    4), // GLX_MESA_query_renderer
    (  35,   65,    1), // GLX_MESA_release_buffers
    (  36,   66,    1), // GLX_MESA_set_3dfx_mode
    (  37,   67,    2), // GLX_MESA_swap_control
    (  38,   69,    2), // GLX_NV_copy_buffer
    (  39,   71,    1), // GLX_NV_copy_image
    (  40,   72,    1), // GLX_NV_delay_before_swap
    (  44,   73,    2), // GLX_NV_present_video
    (  46,   75,    6), // GLX_NV_swap_group
    (  47,   81,    5), // GLX_NV_video_capture
    (  48,   86,    6), // GLX_NV_video_out
    (  50,   92,    5), // GLX_OML_sync_control
    (  61,   97,    1), // GLX_SGI_cushion
    (  62,   98,    2), // GLX_SGI_make_current_read
    (  63,  100,    1), // GLX_SGI_swap_control
    (  64,  101,    2), // GLX_SGI_video_sync
    (  54,  103,    6), // GLX_SGIX_fbconfig
    (  55,  109,    8), // GLX_SGIX_hyperpipe
    (  56,  117,    5), // GLX_SGIX_pbuffer
    (  57,  122,    2), // GLX_SGIX_swap_barrier
    (  58,  124,    1), // GLX_SGIX_swap_group
    (  59,  125,    5), // GLX_SGIX_video_resize
    (  65,  130,    1), // GLX_SUN_get_transparent_index
];

// ── Extensions ──────────────────────────────────────────────
pub const EXT_COUNT: usize = 66;

// XXH3-64 of each extension name, sorted for binary search.
#[rustfmt::skip]
static EXT_HASH_KEYS: [u64; EXT_COUNT] = [
    0x03bcfc5a3cae3c31, // GLX_EXT_import_context
    0x047a6e2eedd3d3d3, // GLX_ARB_create_context_robustness
    0x048a82b333a65f74, // GLX_ARB_context_flush_control
    0x072d0fa7672bf59a, // GLX_AMD_gpu_association
    0x07a1d27a401cc274, // GLX_SGIX_visual_select_group
    0x081dfaeea13e2411, // GLX_ARB_robustness_share_group_isolation
    0x09ddc4c71d7734d0, // GLX_SGIX_pbuffer
    0x0d617d9b2139534b, // GLX_SGI_video_sync
    0x0e4e87fbd85b8f81, // GLX_MESA_release_buffers
    0x12a9351ba20ce5a3, // GLX_EXT_no_config_context
    0x139db0a508493724, // GLX_OML_swap_method
    0x1b481fa7315ee5b6, // GLX_EXT_context_priority
    0x29561836651d917a, // GLX_MESA_set_3dfx_mode
    0x29596e504e9a6a25, // GLX_MESA_query_renderer
    0x2e804f63a6dcd082, // GLX_ARB_vertex_buffer_object
    0x2f414e4fcd131e32, // GLX_NV_swap_group
    0x341bcc9d120c6f49, // GLX_SGIX_hyperpipe
    0x346e291f40286e34, // GLX_EXT_fbconfig_packed_float
    0x3f4d9cd2b079e931, // GLX_ARB_create_context
    0x4cab82df7b1591ee, // GLX_SGIX_swap_group
    0x4e3ccfe50be2411d, // GLX_NV_float_buffer
    0x5c16b82b4300c08c, // GLX_SGIS_shared_multisample
    0x6073c41473e50bfa, // GLX_NV_multisample_coverage
    0x6c1fc772574daedc, // GLX_EXT_visual_rating
    0x6dadb65d0ec947c2, // GLX_SGIX_fbconfig
    0x6f6d381e8ae478f2, // GLX_EXT_libglvnd
    0x6fad53b639e2a390, // GLX_ARB_get_proc_address
    0x7cef1b9da97fd70e, // GLX_EXT_stereo_tree
    0x7dd3a391032250e3, // GLX_ARB_multisample
    0x81782a45ccd71ff8, // GLX_EXT_swap_control_tear
    0x83387f3dfb09e41f, // GLX_EXT_get_drawable_type
    0x8690d94d5a4ce24d, // GLX_ARB_create_context_profile
    0x871aa145ea386a73, // GLX_NV_present_video
    0x900c6b39bcf9e3ae, // GLX_SGIS_multisample
    0x90a93448eb7baa50, // GLX_INTEL_swap_event
    0x910ff30a1deb6b79, // GLX_MESA_copy_sub_buffer
    0x939dc7e8d56cb021, // GLX_NV_copy_image
    0x963ec6ee3eb6b7fc, // GLX_NV_video_out
    0xa07f3edb8ce15ce0, // GLX_ARB_fbconfig_float
    0xa1de47dd92ad3d02, // GLX_SUN_get_transparent_index
    0xad9c9c9f23007dd1, // GLX_SGI_swap_control
    0xadbdc7e24c604283, // GLX_SGIX_video_resize
    0xaec606cc661980b4, // GLX_EXT_create_context_es2_profile
    0xaf8c148fdb3c3939, // GLX_SGIX_swap_barrier
    0xb01c250c6b9a122a, // GLX_SGI_make_current_read
    0xb279c1493993d32d, // GLX_EXT_swap_control
    0xb54066e1e7b44f60, // GLX_SGI_cushion
    0xb747ecf03b25c5b0, // GLX_EXT_visual_info
    0xbd7c0e0c70fd916b, // GLX_ARB_create_context_no_error
    0xc090be22f50edf0f, // GLX_3DFX_multisample
    0xc52be3ef322db374, // GLX_NV_multigpu_context
    0xc6df36e9a84a750e, // GLX_NV_copy_buffer
    0xcd4daa50b9e2718a, // GLX_EXT_texture_from_pixmap
    0xce0282d625639b9b, // GLX_MESA_agp_offset
    0xd153ec8aa16793b3, // GLX_EXT_framebuffer_sRGB
    0xd32cf4de2880d29f, // GLX_MESA_swap_control
    0xd5e2cc7383f82f45, // GLX_ARB_framebuffer_sRGB
    0xdab23c1e71961e55, // GLX_MESA_pixmap_colormap
    0xdae9b56ed6046e29, // GLX_NV_delay_before_swap
    0xdb0def6df3a16d6c, // GLX_NV_robustness_video_memory_purge
    0xdba9a6195cec812f, // GLX_ARB_robustness_application_isolation
    0xe386bd135a6e5f2c, // GLX_SGIS_blended_overlay
    0xe7306595f98ee4e1, // GLX_EXT_buffer_age
    0xefa7c60274d87055, // GLX_NV_video_capture
    0xf1d970eb90dd86a0, // GLX_EXT_create_context_es_profile
    0xffaa3afdd9aa1090, // GLX_OML_sync_control
];
// extArray index for the correspondingly-ranked EXT_HASH_KEYS entry.
#[rustfmt::skip]
static EXT_HASH_IDX: [u16; EXT_COUNT] = [
    21, 6, 2, 1, 60, 12, 56, 64, 35, 23, 49, 15, 36, 34, 13, 46, 55, 18, 3, 58,
    41, 53, 43, 29, 54, 22, 9, 24, 10, 26, 20, 5, 44, 52, 30, 32, 39, 48, 7, 65,
    63, 59, 16, 57, 62, 25, 61, 28, 4, 0, 42, 38, 27, 31, 19, 37, 8, 33, 40, 45,
    11, 51, 14, 47, 17, 50,
];

// ── Unloaded-call handling ──────────────────────────────────
/// Reached when a dispatch wrapper finds a null PFN: the function is not
/// loaded (feature/extension absent, or the context was never loaded).
/// Panics with the function's name from the name blob.
#[cfg(not(feature = "no-error"))]
#[cold]
#[inline(never)]
unsafe fn __missing(idx: usize) -> ! {
    let off = FN_NAME_OFFSETS[idx] as usize;
    let name = CStr::from_bytes_until_nul(&FN_NAME_DATA[off..]).unwrap_or(c"?");
    panic!(
        "{} is not loaded (unsupported by this context, or called before load)",
        name.to_str().unwrap_or("?")
    )
}

/// `no-error` build (the KHR_no_error analogue): promise the compiler the
/// null case is impossible, so the dispatch match compiles to an
/// unchecked call.  Calling an unloaded function is undefined behavior,
/// exactly as in C.
#[cfg(feature = "no-error")]
#[inline(always)]
unsafe fn __missing(_idx: usize) -> ! {
    debug_assert!(false, "unloaded GLX function called in a no-error build");
    unsafe { core::hint::unreachable_unchecked() }
}

// ── Context ─────────────────────────────────────────────────
/// Why [`Glx::load_glx`] failed.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LoadError {
    /// `display` was null.  (The C loader opens the default display
    /// here, but that links libX11; this crate takes no link
    /// dependencies, so the caller supplies the connection.)
    NoDisplay,
    /// `loader` returned null for `glXQueryVersion` — not a GLX
    /// proc-address source.
    MissingQueryVersion,
    /// `glXQueryVersion` reported no usable version.
    QueryVersionFailed,
    /// `glXQueryExtensionsString` was unavailable or returned null.
    MissingExtensionsString,
}

impl core::fmt::Display for LoadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            LoadError::NoDisplay => "display is null",
            LoadError::MissingQueryVersion => "glXQueryVersion is not available",
            LoadError::QueryVersionFailed => "glXQueryVersion reported no version",
            LoadError::MissingExtensionsString => "GLX extensions string missing",
        })
    }
}

impl core::error::Error for LoadError {}

/// Loaded GLX entry points plus detected feature/extension presence.
pub struct Glx {
    pfns: [*const c_void; COMMAND_COUNT],
    feat: [bool; FEATURE_COUNT],
    ext: [bool; EXT_COUNT],
    version: u32,
}

impl Glx {
    /// Load GLX against `loader` (a glXGetProcAddress-style callback)
    /// and detect the version, then extensions, for `display` +
    /// `screen`.
    ///
    /// # Safety
    /// `display` must be a valid X connection, `screen` a valid screen
    /// number on it, and `loader` must yield pointers callable as the
    /// named GLX functions.
    #[inline]
    pub unsafe fn load_glx(
        display: *mut Display,
        screen: i32,
        mut loader: impl FnMut(&CStr) -> *const c_void,
    ) -> Result<Self, LoadError> {
        // Immediately erase to `&mut dyn` — the real loader is compiled
        // once, not once per closure type.
        unsafe { Self::load_glx_dyn(display, screen, &mut loader) }
    }

    unsafe fn load_glx_dyn(
        display: *mut Display,
        screen: i32,
        loader: &mut dyn FnMut(&CStr) -> *const c_void,
    ) -> Result<Self, LoadError> {
        let mut glx = Self {
            pfns: [core::ptr::null(); COMMAND_COUNT],
            feat: [false; FEATURE_COUNT],
            ext: [false; EXT_COUNT],
            version: 0,
        };
        if display.is_null() {
            return Err(LoadError::NoDisplay);
        }
        glx.pfns[12] = loader(c"glXQueryVersion");
        if glx.pfns[12].is_null() {
            return Err(LoadError::MissingQueryVersion);
        }
        let (mut major, mut minor): (i32, i32) = (0, 0);
        unsafe { glx.QueryVersion(display, &mut major, &mut minor) };
        glx.version = ((major as u32) << 8) | (minor as u32 & 0xff);
        if glx.version == 0 {
            return Err(LoadError::QueryVersionFailed);
        }
        // Feature presence from the queried version.
        glx.feat[0] = glx.version >= 0x0100;
        glx.feat[1] = glx.version >= 0x0101;
        glx.feat[2] = glx.version >= 0x0102;
        glx.feat[3] = glx.version >= 0x0103;
        glx.feat[4] = glx.version >= 0x0104;
        // Load every PFN upfront, then additionally mark features whose
        // every PFN resolved (set-only, mirroring the C loader).
        unsafe { glx.load_range(loader, 0, COMMAND_COUNT as u16) };
        for &(fi, start, count) in FEATURE_RANGES.iter() {
            let mut ok = true;
            for i in start..start + count {
                ok &= !glx.pfns[i as usize].is_null();
            }
            if ok {
                glx.feat[fi as usize] = true;
            }
        }
        unsafe { glx.detect_extensions(display, screen)? };
        for &(ei, start, count) in EXT_RANGES_glx.iter() {
            if glx.ext[ei as usize] {
                unsafe { glx.load_range(loader, start, count) };
            }
        }
        glx.resolve_aliases();
        Ok(glx)
    }

    #[inline]
    unsafe fn load_range(
        &mut self,
        loader: &mut dyn FnMut(&CStr) -> *const c_void,
        start: u16,
        count: u16,
    ) {
        for i in start..start + count {
            let idx = i as usize;
            let off = FN_NAME_OFFSETS[idx] as usize;
            let name =
                unsafe { CStr::from_bytes_until_nul(&FN_NAME_DATA[off..]).unwrap_unchecked() };
            self.pfns[idx] = loader(name);
        }
    }

    /// The space-separated extensions string, hashed word-by-word
    /// against the pre-baked table.
    unsafe fn detect_extensions(&mut self, display: *mut Display, screen: i32) -> Result<(), LoadError> {
        if self.pfns[18].is_null() {
            return Err(LoadError::MissingExtensionsString);
        }
        let ext_str = unsafe { self.QueryExtensionsString(display, screen) };
        if ext_str.is_null() {
            return Err(LoadError::MissingExtensionsString);
        }
        unsafe { self.hash_ext_words(ext_str) };
        Ok(())
    }

    /// Tokenize a NUL-terminated, space-separated extension list and
    /// set the flag for every known name (XXH3 + binary search — the
    /// same pre-baked hashes the C loader uses).
    unsafe fn hash_ext_words(&mut self, p: *const c_char) {
        let bytes = unsafe { CStr::from_ptr(p) }.to_bytes();
        for word in bytes.split(|&b| b == b' ') {
            if word.is_empty() {
                continue;
            }
            let h = xxhash_rust::xxh3::xxh3_64(word);
            if let Ok(pos) = EXT_HASH_KEYS.binary_search(&h) {
                self.ext[EXT_HASH_IDX[pos] as usize] = true;
            }
        }
    }

    fn resolve_aliases(&mut self) {}

    /// Detected GLX version, packed as `major << 8 | minor`.
    #[inline]
    pub fn version(&self) -> u32 {
        self.version
    }

    // Dispatch wrappers.  The pointer local is named `__pfn` because
    // parameter names could otherwise collide with it.

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn ChooseVisual(&self, dpy: *mut Display, screen: i32, attribList: *mut i32) -> *mut XVisualInfo {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32, *mut i32) -> *mut XVisualInfo> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(0)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(0) },
        };
        unsafe { __pfn(dpy, screen, attribList) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn CopyContext(&self, dpy: *mut Display, src: GLXContext, dst: GLXContext, mask: core::ffi::c_ulong) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXContext, GLXContext, core::ffi::c_ulong)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(1)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(1) },
        };
        unsafe { __pfn(dpy, src, dst, mask) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn CreateContext(&self, dpy: *mut Display, vis: *mut XVisualInfo, shareList: GLXContext, direct: Bool) -> GLXContext {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, *mut XVisualInfo, GLXContext, Bool) -> GLXContext> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(2)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(2) },
        };
        unsafe { __pfn(dpy, vis, shareList, direct) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn CreateGLXPixmap(&self, dpy: *mut Display, visual: *mut XVisualInfo, pixmap: Pixmap) -> GLXPixmap {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, *mut XVisualInfo, Pixmap) -> GLXPixmap> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(3)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(3) },
        };
        unsafe { __pfn(dpy, visual, pixmap) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn DestroyContext(&self, dpy: *mut Display, ctx: GLXContext) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXContext)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(4)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(4) },
        };
        unsafe { __pfn(dpy, ctx) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn DestroyGLXPixmap(&self, dpy: *mut Display, pixmap: GLXPixmap) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXPixmap)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(5)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(5) },
        };
        unsafe { __pfn(dpy, pixmap) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetConfig(&self, dpy: *mut Display, visual: *mut XVisualInfo, attrib: i32, value: *mut i32) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, *mut XVisualInfo, i32, *mut i32) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(6)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(6) },
        };
        unsafe { __pfn(dpy, visual, attrib, value) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetCurrentContext(&self) -> GLXContext {
        let __pfn: Option<unsafe extern "system" fn() -> GLXContext> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(7)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(7) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetCurrentDrawable(&self) -> GLXDrawable {
        let __pfn: Option<unsafe extern "system" fn() -> GLXDrawable> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(8)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(8) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn IsDirect(&self, dpy: *mut Display, ctx: GLXContext) -> Bool {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXContext) -> Bool> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(9)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(9) },
        };
        unsafe { __pfn(dpy, ctx) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn MakeCurrent(&self, dpy: *mut Display, drawable: GLXDrawable, ctx: GLXContext) -> Bool {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXDrawable, GLXContext) -> Bool> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(10)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(10) },
        };
        unsafe { __pfn(dpy, drawable, ctx) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn QueryExtension(&self, dpy: *mut Display, errorb: *mut i32, event: *mut i32) -> Bool {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, *mut i32, *mut i32) -> Bool> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(11)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(11) },
        };
        unsafe { __pfn(dpy, errorb, event) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn QueryVersion(&self, dpy: *mut Display, maj: *mut i32, min: *mut i32) -> Bool {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, *mut i32, *mut i32) -> Bool> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(12)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(12) },
        };
        unsafe { __pfn(dpy, maj, min) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn SwapBuffers(&self, dpy: *mut Display, drawable: GLXDrawable) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXDrawable)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(13)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(13) },
        };
        unsafe { __pfn(dpy, drawable) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn UseXFont(&self, font: Font, first: i32, count: i32, list: i32) {
        let __pfn: Option<unsafe extern "system" fn(Font, i32, i32, i32)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(14)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(14) },
        };
        unsafe { __pfn(font, first, count, list) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn WaitGL(&self) {
        let __pfn: Option<unsafe extern "system" fn()> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(15)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(15) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn WaitX(&self) {
        let __pfn: Option<unsafe extern "system" fn()> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(16)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(16) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetClientString(&self, dpy: *mut Display, name: i32) -> *const c_char {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32) -> *const c_char> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(17)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(17) },
        };
        unsafe { __pfn(dpy, name) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn QueryExtensionsString(&self, dpy: *mut Display, screen: i32) -> *const c_char {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32) -> *const c_char> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(18)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(18) },
        };
        unsafe { __pfn(dpy, screen) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn QueryServerString(&self, dpy: *mut Display, screen: i32, name: i32) -> *const c_char {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32, i32) -> *const c_char> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(19)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(19) },
        };
        unsafe { __pfn(dpy, screen, name) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetCurrentDisplay(&self) -> *mut Display {
        let __pfn: Option<unsafe extern "system" fn() -> *mut Display> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(20)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(20) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn ChooseFBConfig(&self, dpy: *mut Display, screen: i32, attrib_list: *const i32, nelements: *mut i32) -> *mut GLXFBConfig {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32, *const i32, *mut i32) -> *mut GLXFBConfig> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(21)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(21) },
        };
        unsafe { __pfn(dpy, screen, attrib_list, nelements) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn CreateNewContext(&self, dpy: *mut Display, config: GLXFBConfig, render_type: i32, share_list: GLXContext, direct: Bool) -> GLXContext {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXFBConfig, i32, GLXContext, Bool) -> GLXContext> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(22)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(22) },
        };
        unsafe { __pfn(dpy, config, render_type, share_list, direct) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn CreatePbuffer(&self, dpy: *mut Display, config: GLXFBConfig, attrib_list: *const i32) -> GLXPbuffer {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXFBConfig, *const i32) -> GLXPbuffer> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(23)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(23) },
        };
        unsafe { __pfn(dpy, config, attrib_list) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn CreatePixmap(&self, dpy: *mut Display, config: GLXFBConfig, pixmap: Pixmap, attrib_list: *const i32) -> GLXPixmap {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXFBConfig, Pixmap, *const i32) -> GLXPixmap> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(24)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(24) },
        };
        unsafe { __pfn(dpy, config, pixmap, attrib_list) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn CreateWindow(&self, dpy: *mut Display, config: GLXFBConfig, win: Window, attrib_list: *const i32) -> GLXWindow {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXFBConfig, Window, *const i32) -> GLXWindow> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(25)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(25) },
        };
        unsafe { __pfn(dpy, config, win, attrib_list) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn DestroyPbuffer(&self, dpy: *mut Display, pbuf: GLXPbuffer) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXPbuffer)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(26)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(26) },
        };
        unsafe { __pfn(dpy, pbuf) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn DestroyPixmap(&self, dpy: *mut Display, pixmap: GLXPixmap) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXPixmap)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(27)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(27) },
        };
        unsafe { __pfn(dpy, pixmap) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn DestroyWindow(&self, dpy: *mut Display, win: GLXWindow) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXWindow)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(28)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(28) },
        };
        unsafe { __pfn(dpy, win) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetCurrentReadDrawable(&self) -> GLXDrawable {
        let __pfn: Option<unsafe extern "system" fn() -> GLXDrawable> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(29)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(29) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetFBConfigAttrib(&self, dpy: *mut Display, config: GLXFBConfig, attribute: i32, value: *mut i32) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXFBConfig, i32, *mut i32) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(30)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(30) },
        };
        unsafe { __pfn(dpy, config, attribute, value) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetFBConfigs(&self, dpy: *mut Display, screen: i32, nelements: *mut i32) -> *mut GLXFBConfig {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32, *mut i32) -> *mut GLXFBConfig> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(31)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(31) },
        };
        unsafe { __pfn(dpy, screen, nelements) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetSelectedEvent(&self, dpy: *mut Display, draw: GLXDrawable, event_mask: *mut core::ffi::c_ulong) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXDrawable, *mut core::ffi::c_ulong)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(32)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(32) },
        };
        unsafe { __pfn(dpy, draw, event_mask) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetVisualFromFBConfig(&self, dpy: *mut Display, config: GLXFBConfig) -> *mut XVisualInfo {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXFBConfig) -> *mut XVisualInfo> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(33)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(33) },
        };
        unsafe { __pfn(dpy, config) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn MakeContextCurrent(&self, dpy: *mut Display, draw: GLXDrawable, read: GLXDrawable, ctx: GLXContext) -> Bool {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXDrawable, GLXDrawable, GLXContext) -> Bool> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(34)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(34) },
        };
        unsafe { __pfn(dpy, draw, read, ctx) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn QueryContext(&self, dpy: *mut Display, ctx: GLXContext, attribute: i32, value: *mut i32) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXContext, i32, *mut i32) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(35)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(35) },
        };
        unsafe { __pfn(dpy, ctx, attribute, value) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn QueryDrawable(&self, dpy: *mut Display, draw: GLXDrawable, attribute: i32, value: *mut u32) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXDrawable, i32, *mut u32)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(36)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(36) },
        };
        unsafe { __pfn(dpy, draw, attribute, value) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn SelectEvent(&self, dpy: *mut Display, draw: GLXDrawable, event_mask: core::ffi::c_ulong) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXDrawable, core::ffi::c_ulong)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(37)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(37) },
        };
        unsafe { __pfn(dpy, draw, event_mask) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetProcAddress(&self, procName: *const GLubyte) -> __GLXextFuncPtr {
        let __pfn: Option<unsafe extern "system" fn(*const GLubyte) -> __GLXextFuncPtr> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(38)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(38) },
        };
        unsafe { __pfn(procName) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn BlitContextFramebufferAMD(&self, dstCtx: GLXContext, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLXContext, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLbitfield, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(39)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(39) },
        };
        unsafe { __pfn(dstCtx, srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn CreateAssociatedContextAMD(&self, id: u32, share_list: GLXContext) -> GLXContext {
        let __pfn: Option<unsafe extern "system" fn(u32, GLXContext) -> GLXContext> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(40)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(40) },
        };
        unsafe { __pfn(id, share_list) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn CreateAssociatedContextAttribsAMD(&self, id: u32, share_context: GLXContext, attribList: *const i32) -> GLXContext {
        let __pfn: Option<unsafe extern "system" fn(u32, GLXContext, *const i32) -> GLXContext> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(41)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(41) },
        };
        unsafe { __pfn(id, share_context, attribList) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn DeleteAssociatedContextAMD(&self, ctx: GLXContext) -> Bool {
        let __pfn: Option<unsafe extern "system" fn(GLXContext) -> Bool> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(42)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(42) },
        };
        unsafe { __pfn(ctx) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetContextGPUIDAMD(&self, ctx: GLXContext) -> u32 {
        let __pfn: Option<unsafe extern "system" fn(GLXContext) -> u32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(43)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(43) },
        };
        unsafe { __pfn(ctx) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetCurrentAssociatedContextAMD(&self) -> GLXContext {
        let __pfn: Option<unsafe extern "system" fn() -> GLXContext> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(44)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(44) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetGPUIDsAMD(&self, maxCount: u32, ids: *mut u32) -> u32 {
        let __pfn: Option<unsafe extern "system" fn(u32, *mut u32) -> u32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(45)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(45) },
        };
        unsafe { __pfn(maxCount, ids) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetGPUInfoAMD(&self, id: u32, property: i32, dataType: GLenum, size: u32, data: *mut c_void) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(u32, i32, GLenum, u32, *mut c_void) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(46)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(46) },
        };
        unsafe { __pfn(id, property, dataType, size, data) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn MakeAssociatedContextCurrentAMD(&self, ctx: GLXContext) -> Bool {
        let __pfn: Option<unsafe extern "system" fn(GLXContext) -> Bool> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(47)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(47) },
        };
        unsafe { __pfn(ctx) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn CreateContextAttribsARB(&self, dpy: *mut Display, config: GLXFBConfig, share_context: GLXContext, direct: Bool, attrib_list: *const i32) -> GLXContext {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXFBConfig, GLXContext, Bool, *const i32) -> GLXContext> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(48)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(48) },
        };
        unsafe { __pfn(dpy, config, share_context, direct, attrib_list) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetProcAddressARB(&self, procName: *const GLubyte) -> __GLXextFuncPtr {
        let __pfn: Option<unsafe extern "system" fn(*const GLubyte) -> __GLXextFuncPtr> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(49)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(49) },
        };
        unsafe { __pfn(procName) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn FreeContextEXT(&self, dpy: *mut Display, context: GLXContext) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXContext)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(50)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(50) },
        };
        unsafe { __pfn(dpy, context) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetContextIDEXT(&self, context: GLXContext) -> GLXContextID {
        let __pfn: Option<unsafe extern "system" fn(GLXContext) -> GLXContextID> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(51)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(51) },
        };
        unsafe { __pfn(context) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetCurrentDisplayEXT(&self) -> *mut Display {
        let __pfn: Option<unsafe extern "system" fn() -> *mut Display> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(52)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(52) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn ImportContextEXT(&self, dpy: *mut Display, contextID: GLXContextID) -> GLXContext {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXContextID) -> GLXContext> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(53)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(53) },
        };
        unsafe { __pfn(dpy, contextID) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn QueryContextInfoEXT(&self, dpy: *mut Display, context: GLXContext, attribute: i32, value: *mut i32) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXContext, i32, *mut i32) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(54)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(54) },
        };
        unsafe { __pfn(dpy, context, attribute, value) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn SwapIntervalEXT(&self, dpy: *mut Display, drawable: GLXDrawable, interval: i32) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXDrawable, i32)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(55)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(55) },
        };
        unsafe { __pfn(dpy, drawable, interval) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn BindTexImageEXT(&self, dpy: *mut Display, drawable: GLXDrawable, buffer: i32, attrib_list: *const i32) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXDrawable, i32, *const i32)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(56)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(56) },
        };
        unsafe { __pfn(dpy, drawable, buffer, attrib_list) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn ReleaseTexImageEXT(&self, dpy: *mut Display, drawable: GLXDrawable, buffer: i32) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXDrawable, i32)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(57)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(57) },
        };
        unsafe { __pfn(dpy, drawable, buffer) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetAGPOffsetMESA(&self, pointer: *const c_void) -> u32 {
        let __pfn: Option<unsafe extern "system" fn(*const c_void) -> u32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(58)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(58) },
        };
        unsafe { __pfn(pointer) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn CopySubBufferMESA(&self, dpy: *mut Display, drawable: GLXDrawable, x: i32, y: i32, width: i32, height: i32) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXDrawable, i32, i32, i32, i32)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(59)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(59) },
        };
        unsafe { __pfn(dpy, drawable, x, y, width, height) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn CreateGLXPixmapMESA(&self, dpy: *mut Display, visual: *mut XVisualInfo, pixmap: Pixmap, cmap: Colormap) -> GLXPixmap {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, *mut XVisualInfo, Pixmap, Colormap) -> GLXPixmap> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(60)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(60) },
        };
        unsafe { __pfn(dpy, visual, pixmap, cmap) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn QueryCurrentRendererIntegerMESA(&self, attribute: i32, value: *mut u32) -> Bool {
        let __pfn: Option<unsafe extern "system" fn(i32, *mut u32) -> Bool> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(61)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(61) },
        };
        unsafe { __pfn(attribute, value) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn QueryCurrentRendererStringMESA(&self, attribute: i32) -> *const c_char {
        let __pfn: Option<unsafe extern "system" fn(i32) -> *const c_char> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(62)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(62) },
        };
        unsafe { __pfn(attribute) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn QueryRendererIntegerMESA(&self, dpy: *mut Display, screen: i32, renderer: i32, attribute: i32, value: *mut u32) -> Bool {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32, i32, i32, *mut u32) -> Bool> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(63)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(63) },
        };
        unsafe { __pfn(dpy, screen, renderer, attribute, value) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn QueryRendererStringMESA(&self, dpy: *mut Display, screen: i32, renderer: i32, attribute: i32) -> *const c_char {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32, i32, i32) -> *const c_char> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(64)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(64) },
        };
        unsafe { __pfn(dpy, screen, renderer, attribute) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn ReleaseBuffersMESA(&self, dpy: *mut Display, drawable: GLXDrawable) -> Bool {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXDrawable) -> Bool> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(65)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(65) },
        };
        unsafe { __pfn(dpy, drawable) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn Set3DfxModeMESA(&self, mode: GLint) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLint) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(66)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(66) },
        };
        unsafe { __pfn(mode) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetSwapIntervalMESA(&self) -> i32 {
        let __pfn: Option<unsafe extern "system" fn() -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(67)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(67) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn SwapIntervalMESA(&self, interval: u32) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(u32) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(68)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(68) },
        };
        unsafe { __pfn(interval) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn CopyBufferSubDataNV(&self, dpy: *mut Display, readCtx: GLXContext, writeCtx: GLXContext, readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXContext, GLXContext, GLenum, GLenum, GLintptr, GLintptr, GLsizeiptr)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(69)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(69) },
        };
        unsafe { __pfn(dpy, readCtx, writeCtx, readTarget, writeTarget, readOffset, writeOffset, size) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn NamedCopyBufferSubDataNV(&self, dpy: *mut Display, readCtx: GLXContext, writeCtx: GLXContext, readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXContext, GLXContext, GLuint, GLuint, GLintptr, GLintptr, GLsizeiptr)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(70)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(70) },
        };
        unsafe { __pfn(dpy, readCtx, writeCtx, readBuffer, writeBuffer, readOffset, writeOffset, size) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn CopyImageSubDataNV(&self, dpy: *mut Display, srcCtx: GLXContext, srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstCtx: GLXContext, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, width: GLsizei, height: GLsizei, depth: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXContext, GLuint, GLenum, GLint, GLint, GLint, GLint, GLXContext, GLuint, GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(71)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(71) },
        };
        unsafe { __pfn(dpy, srcCtx, srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dstCtx, dstName, dstTarget, dstLevel, dstX, dstY, dstZ, width, height, depth) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn DelayBeforeSwapNV(&self, dpy: *mut Display, drawable: GLXDrawable, seconds: GLfloat) -> Bool {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXDrawable, GLfloat) -> Bool> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(72)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(72) },
        };
        unsafe { __pfn(dpy, drawable, seconds) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn BindVideoDeviceNV(&self, dpy: *mut Display, video_slot: u32, video_device: u32, attrib_list: *const i32) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, u32, u32, *const i32) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(73)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(73) },
        };
        unsafe { __pfn(dpy, video_slot, video_device, attrib_list) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn EnumerateVideoDevicesNV(&self, dpy: *mut Display, screen: i32, nelements: *mut i32) -> *mut u32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32, *mut i32) -> *mut u32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(74)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(74) },
        };
        unsafe { __pfn(dpy, screen, nelements) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn BindSwapBarrierNV(&self, dpy: *mut Display, group: GLuint, barrier: GLuint) -> Bool {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLuint, GLuint) -> Bool> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(75)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(75) },
        };
        unsafe { __pfn(dpy, group, barrier) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn JoinSwapGroupNV(&self, dpy: *mut Display, drawable: GLXDrawable, group: GLuint) -> Bool {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXDrawable, GLuint) -> Bool> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(76)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(76) },
        };
        unsafe { __pfn(dpy, drawable, group) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn QueryFrameCountNV(&self, dpy: *mut Display, screen: i32, count: *mut GLuint) -> Bool {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32, *mut GLuint) -> Bool> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(77)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(77) },
        };
        unsafe { __pfn(dpy, screen, count) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn QueryMaxSwapGroupsNV(&self, dpy: *mut Display, screen: i32, maxGroups: *mut GLuint, maxBarriers: *mut GLuint) -> Bool {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32, *mut GLuint, *mut GLuint) -> Bool> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(78)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(78) },
        };
        unsafe { __pfn(dpy, screen, maxGroups, maxBarriers) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn QuerySwapGroupNV(&self, dpy: *mut Display, drawable: GLXDrawable, group: *mut GLuint, barrier: *mut GLuint) -> Bool {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXDrawable, *mut GLuint, *mut GLuint) -> Bool> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(79)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(79) },
        };
        unsafe { __pfn(dpy, drawable, group, barrier) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn ResetFrameCountNV(&self, dpy: *mut Display, screen: i32) -> Bool {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32) -> Bool> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(80)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(80) },
        };
        unsafe { __pfn(dpy, screen) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn BindVideoCaptureDeviceNV(&self, dpy: *mut Display, video_capture_slot: u32, device: GLXVideoCaptureDeviceNV) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, u32, GLXVideoCaptureDeviceNV) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(81)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(81) },
        };
        unsafe { __pfn(dpy, video_capture_slot, device) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn EnumerateVideoCaptureDevicesNV(&self, dpy: *mut Display, screen: i32, nelements: *mut i32) -> *mut GLXVideoCaptureDeviceNV {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32, *mut i32) -> *mut GLXVideoCaptureDeviceNV> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(82)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(82) },
        };
        unsafe { __pfn(dpy, screen, nelements) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn LockVideoCaptureDeviceNV(&self, dpy: *mut Display, device: GLXVideoCaptureDeviceNV) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXVideoCaptureDeviceNV)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(83)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(83) },
        };
        unsafe { __pfn(dpy, device) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn QueryVideoCaptureDeviceNV(&self, dpy: *mut Display, device: GLXVideoCaptureDeviceNV, attribute: i32, value: *mut i32) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXVideoCaptureDeviceNV, i32, *mut i32) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(84)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(84) },
        };
        unsafe { __pfn(dpy, device, attribute, value) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn ReleaseVideoCaptureDeviceNV(&self, dpy: *mut Display, device: GLXVideoCaptureDeviceNV) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXVideoCaptureDeviceNV)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(85)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(85) },
        };
        unsafe { __pfn(dpy, device) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn BindVideoImageNV(&self, dpy: *mut Display, VideoDevice: GLXVideoDeviceNV, pbuf: GLXPbuffer, iVideoBuffer: i32) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXVideoDeviceNV, GLXPbuffer, i32) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(86)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(86) },
        };
        unsafe { __pfn(dpy, VideoDevice, pbuf, iVideoBuffer) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetVideoDeviceNV(&self, dpy: *mut Display, screen: i32, numVideoDevices: i32, pVideoDevice: *mut GLXVideoDeviceNV) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32, i32, *mut GLXVideoDeviceNV) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(87)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(87) },
        };
        unsafe { __pfn(dpy, screen, numVideoDevices, pVideoDevice) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetVideoInfoNV(&self, dpy: *mut Display, screen: i32, VideoDevice: GLXVideoDeviceNV, pulCounterOutputPbuffer: *mut core::ffi::c_ulong, pulCounterOutputVideo: *mut core::ffi::c_ulong) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32, GLXVideoDeviceNV, *mut core::ffi::c_ulong, *mut core::ffi::c_ulong) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(88)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(88) },
        };
        unsafe { __pfn(dpy, screen, VideoDevice, pulCounterOutputPbuffer, pulCounterOutputVideo) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn ReleaseVideoDeviceNV(&self, dpy: *mut Display, screen: i32, VideoDevice: GLXVideoDeviceNV) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32, GLXVideoDeviceNV) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(89)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(89) },
        };
        unsafe { __pfn(dpy, screen, VideoDevice) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn ReleaseVideoImageNV(&self, dpy: *mut Display, pbuf: GLXPbuffer) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXPbuffer) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(90)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(90) },
        };
        unsafe { __pfn(dpy, pbuf) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn SendPbufferToVideoNV(&self, dpy: *mut Display, pbuf: GLXPbuffer, iBufferType: i32, pulCounterPbuffer: *mut core::ffi::c_ulong, bBlock: GLboolean) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXPbuffer, i32, *mut core::ffi::c_ulong, GLboolean) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(91)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(91) },
        };
        unsafe { __pfn(dpy, pbuf, iBufferType, pulCounterPbuffer, bBlock) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetMscRateOML(&self, dpy: *mut Display, drawable: GLXDrawable, numerator: *mut i32, denominator: *mut i32) -> Bool {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXDrawable, *mut i32, *mut i32) -> Bool> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(92)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(92) },
        };
        unsafe { __pfn(dpy, drawable, numerator, denominator) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetSyncValuesOML(&self, dpy: *mut Display, drawable: GLXDrawable, ust: *mut i64, msc: *mut i64, sbc: *mut i64) -> Bool {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXDrawable, *mut i64, *mut i64, *mut i64) -> Bool> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(93)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(93) },
        };
        unsafe { __pfn(dpy, drawable, ust, msc, sbc) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn SwapBuffersMscOML(&self, dpy: *mut Display, drawable: GLXDrawable, target_msc: i64, divisor: i64, remainder: i64) -> i64 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXDrawable, i64, i64, i64) -> i64> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(94)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(94) },
        };
        unsafe { __pfn(dpy, drawable, target_msc, divisor, remainder) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn WaitForMscOML(&self, dpy: *mut Display, drawable: GLXDrawable, target_msc: i64, divisor: i64, remainder: i64, ust: *mut i64, msc: *mut i64, sbc: *mut i64) -> Bool {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXDrawable, i64, i64, i64, *mut i64, *mut i64, *mut i64) -> Bool> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(95)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(95) },
        };
        unsafe { __pfn(dpy, drawable, target_msc, divisor, remainder, ust, msc, sbc) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn WaitForSbcOML(&self, dpy: *mut Display, drawable: GLXDrawable, target_sbc: i64, ust: *mut i64, msc: *mut i64, sbc: *mut i64) -> Bool {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXDrawable, i64, *mut i64, *mut i64, *mut i64) -> Bool> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(96)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(96) },
        };
        unsafe { __pfn(dpy, drawable, target_sbc, ust, msc, sbc) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn CushionSGI(&self, dpy: *mut Display, window: Window, cushion: f32) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, Window, f32)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(97)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(97) },
        };
        unsafe { __pfn(dpy, window, cushion) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetCurrentReadDrawableSGI(&self) -> GLXDrawable {
        let __pfn: Option<unsafe extern "system" fn() -> GLXDrawable> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(98)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(98) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn MakeCurrentReadSGI(&self, dpy: *mut Display, draw: GLXDrawable, read: GLXDrawable, ctx: GLXContext) -> Bool {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXDrawable, GLXDrawable, GLXContext) -> Bool> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(99)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(99) },
        };
        unsafe { __pfn(dpy, draw, read, ctx) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn SwapIntervalSGI(&self, interval: i32) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(i32) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(100)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(100) },
        };
        unsafe { __pfn(interval) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetVideoSyncSGI(&self, count: *mut u32) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut u32) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(101)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(101) },
        };
        unsafe { __pfn(count) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn WaitVideoSyncSGI(&self, divisor: i32, remainder: i32, count: *mut u32) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(i32, i32, *mut u32) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(102)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(102) },
        };
        unsafe { __pfn(divisor, remainder, count) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn ChooseFBConfigSGIX(&self, dpy: *mut Display, screen: i32, attrib_list: *mut i32, nelements: *mut i32) -> *mut GLXFBConfigSGIX {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32, *mut i32, *mut i32) -> *mut GLXFBConfigSGIX> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(103)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(103) },
        };
        unsafe { __pfn(dpy, screen, attrib_list, nelements) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn CreateContextWithConfigSGIX(&self, dpy: *mut Display, config: GLXFBConfigSGIX, render_type: i32, share_list: GLXContext, direct: Bool) -> GLXContext {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXFBConfigSGIX, i32, GLXContext, Bool) -> GLXContext> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(104)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(104) },
        };
        unsafe { __pfn(dpy, config, render_type, share_list, direct) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn CreateGLXPixmapWithConfigSGIX(&self, dpy: *mut Display, config: GLXFBConfigSGIX, pixmap: Pixmap) -> GLXPixmap {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXFBConfigSGIX, Pixmap) -> GLXPixmap> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(105)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(105) },
        };
        unsafe { __pfn(dpy, config, pixmap) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetFBConfigAttribSGIX(&self, dpy: *mut Display, config: GLXFBConfigSGIX, attribute: i32, value: *mut i32) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXFBConfigSGIX, i32, *mut i32) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(106)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(106) },
        };
        unsafe { __pfn(dpy, config, attribute, value) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetFBConfigFromVisualSGIX(&self, dpy: *mut Display, vis: *mut XVisualInfo) -> GLXFBConfigSGIX {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, *mut XVisualInfo) -> GLXFBConfigSGIX> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(107)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(107) },
        };
        unsafe { __pfn(dpy, vis) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetVisualFromFBConfigSGIX(&self, dpy: *mut Display, config: GLXFBConfigSGIX) -> *mut XVisualInfo {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXFBConfigSGIX) -> *mut XVisualInfo> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(108)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(108) },
        };
        unsafe { __pfn(dpy, config) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn BindHyperpipeSGIX(&self, dpy: *mut Display, hpId: i32) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(109)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(109) },
        };
        unsafe { __pfn(dpy, hpId) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn DestroyHyperpipeConfigSGIX(&self, dpy: *mut Display, hpId: i32) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(110)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(110) },
        };
        unsafe { __pfn(dpy, hpId) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn HyperpipeAttribSGIX(&self, dpy: *mut Display, timeSlice: i32, attrib: i32, size: i32, attribList: *mut c_void) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32, i32, i32, *mut c_void) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(111)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(111) },
        };
        unsafe { __pfn(dpy, timeSlice, attrib, size, attribList) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn HyperpipeConfigSGIX(&self, dpy: *mut Display, networkId: i32, npipes: i32, cfg: *mut GLXHyperpipeConfigSGIX, hpId: *mut i32) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32, i32, *mut GLXHyperpipeConfigSGIX, *mut i32) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(112)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(112) },
        };
        unsafe { __pfn(dpy, networkId, npipes, cfg, hpId) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn QueryHyperpipeAttribSGIX(&self, dpy: *mut Display, timeSlice: i32, attrib: i32, size: i32, returnAttribList: *mut c_void) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32, i32, i32, *mut c_void) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(113)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(113) },
        };
        unsafe { __pfn(dpy, timeSlice, attrib, size, returnAttribList) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn QueryHyperpipeBestAttribSGIX(&self, dpy: *mut Display, timeSlice: i32, attrib: i32, size: i32, attribList: *mut c_void, returnAttribList: *mut c_void) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32, i32, i32, *mut c_void, *mut c_void) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(114)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(114) },
        };
        unsafe { __pfn(dpy, timeSlice, attrib, size, attribList, returnAttribList) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn QueryHyperpipeConfigSGIX(&self, dpy: *mut Display, hpId: i32, npipes: *mut i32) -> *mut GLXHyperpipeConfigSGIX {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32, *mut i32) -> *mut GLXHyperpipeConfigSGIX> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(115)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(115) },
        };
        unsafe { __pfn(dpy, hpId, npipes) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn QueryHyperpipeNetworkSGIX(&self, dpy: *mut Display, npipes: *mut i32) -> *mut GLXHyperpipeNetworkSGIX {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, *mut i32) -> *mut GLXHyperpipeNetworkSGIX> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(116)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(116) },
        };
        unsafe { __pfn(dpy, npipes) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn CreateGLXPbufferSGIX(&self, dpy: *mut Display, config: GLXFBConfigSGIX, width: u32, height: u32, attrib_list: *mut i32) -> GLXPbufferSGIX {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXFBConfigSGIX, u32, u32, *mut i32) -> GLXPbufferSGIX> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(117)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(117) },
        };
        unsafe { __pfn(dpy, config, width, height, attrib_list) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn DestroyGLXPbufferSGIX(&self, dpy: *mut Display, pbuf: GLXPbufferSGIX) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXPbufferSGIX)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(118)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(118) },
        };
        unsafe { __pfn(dpy, pbuf) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetSelectedEventSGIX(&self, dpy: *mut Display, drawable: GLXDrawable, mask: *mut core::ffi::c_ulong) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXDrawable, *mut core::ffi::c_ulong)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(119)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(119) },
        };
        unsafe { __pfn(dpy, drawable, mask) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn QueryGLXPbufferSGIX(&self, dpy: *mut Display, pbuf: GLXPbufferSGIX, attribute: i32, value: *mut u32) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXPbufferSGIX, i32, *mut u32)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(120)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(120) },
        };
        unsafe { __pfn(dpy, pbuf, attribute, value) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn SelectEventSGIX(&self, dpy: *mut Display, drawable: GLXDrawable, mask: core::ffi::c_ulong) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXDrawable, core::ffi::c_ulong)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(121)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(121) },
        };
        unsafe { __pfn(dpy, drawable, mask) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn BindSwapBarrierSGIX(&self, dpy: *mut Display, drawable: GLXDrawable, barrier: i32) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXDrawable, i32)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(122)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(122) },
        };
        unsafe { __pfn(dpy, drawable, barrier) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn QueryMaxSwapBarriersSGIX(&self, dpy: *mut Display, screen: i32, max: *mut i32) -> Bool {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32, *mut i32) -> Bool> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(123)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(123) },
        };
        unsafe { __pfn(dpy, screen, max) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn JoinSwapGroupSGIX(&self, dpy: *mut Display, drawable: GLXDrawable, member: GLXDrawable) {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, GLXDrawable, GLXDrawable)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(124)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(124) },
        };
        unsafe { __pfn(dpy, drawable, member) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn BindChannelToWindowSGIX(&self, display: *mut Display, screen: i32, channel: i32, window: Window) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32, i32, Window) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(125)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(125) },
        };
        unsafe { __pfn(display, screen, channel, window) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn ChannelRectSGIX(&self, display: *mut Display, screen: i32, channel: i32, x: i32, y: i32, w: i32, h: i32) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32, i32, i32, i32, i32, i32) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(126)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(126) },
        };
        unsafe { __pfn(display, screen, channel, x, y, w, h) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn ChannelRectSyncSGIX(&self, display: *mut Display, screen: i32, channel: i32, synctype: GLenum) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32, i32, GLenum) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(127)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(127) },
        };
        unsafe { __pfn(display, screen, channel, synctype) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn QueryChannelDeltasSGIX(&self, display: *mut Display, screen: i32, channel: i32, x: *mut i32, y: *mut i32, w: *mut i32, h: *mut i32) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32, i32, *mut i32, *mut i32, *mut i32, *mut i32) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(128)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(128) },
        };
        unsafe { __pfn(display, screen, channel, x, y, w, h) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn QueryChannelRectSGIX(&self, display: *mut Display, screen: i32, channel: i32, dx: *mut i32, dy: *mut i32, dw: *mut i32, dh: *mut i32) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, i32, i32, *mut i32, *mut i32, *mut i32, *mut i32) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(129)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(129) },
        };
        unsafe { __pfn(display, screen, channel, dx, dy, dw, dh) }
    }

    /// # Safety
    /// GLX must be loaded for a current display; see [`Glx::load_glx`].
    #[inline]
    pub unsafe fn GetTransparentIndexSUN(&self, dpy: *mut Display, overlay: Window, underlay: Window, pTransparentIndex: *mut core::ffi::c_ulong) -> Status {
        let __pfn: Option<unsafe extern "system" fn(*mut Display, Window, Window, *mut core::ffi::c_ulong) -> Status> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(130)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(130) },
        };
        unsafe { __pfn(dpy, overlay, underlay, pTransparentIndex) }
    }

    /// Whether the driver advertises `GLX_3DFX_multisample`.
    #[inline]
    pub fn _3DFX_multisample(&self) -> bool {
        self.ext[0]
    }

    /// Whether the driver advertises `GLX_AMD_gpu_association`.
    #[inline]
    pub fn AMD_gpu_association(&self) -> bool {
        self.ext[1]
    }

    /// Whether the driver advertises `GLX_ARB_context_flush_control`.
    #[inline]
    pub fn ARB_context_flush_control(&self) -> bool {
        self.ext[2]
    }

    /// Whether the driver advertises `GLX_ARB_create_context`.
    #[inline]
    pub fn ARB_create_context(&self) -> bool {
        self.ext[3]
    }

    /// Whether the driver advertises `GLX_ARB_create_context_no_error`.
    #[inline]
    pub fn ARB_create_context_no_error(&self) -> bool {
        self.ext[4]
    }

    /// Whether the driver advertises `GLX_ARB_create_context_profile`.
    #[inline]
    pub fn ARB_create_context_profile(&self) -> bool {
        self.ext[5]
    }

    /// Whether the driver advertises `GLX_ARB_create_context_robustness`.
    #[inline]
    pub fn ARB_create_context_robustness(&self) -> bool {
        self.ext[6]
    }

    /// Whether the driver advertises `GLX_ARB_fbconfig_float`.
    #[inline]
    pub fn ARB_fbconfig_float(&self) -> bool {
        self.ext[7]
    }

    /// Whether the driver advertises `GLX_ARB_framebuffer_sRGB`.
    #[inline]
    pub fn ARB_framebuffer_sRGB(&self) -> bool {
        self.ext[8]
    }

    /// Whether the driver advertises `GLX_ARB_get_proc_address`.
    #[inline]
    pub fn ARB_get_proc_address(&self) -> bool {
        self.ext[9]
    }

    /// Whether the driver advertises `GLX_ARB_multisample`.
    #[inline]
    pub fn ARB_multisample(&self) -> bool {
        self.ext[10]
    }

    /// Whether the driver advertises `GLX_ARB_robustness_application_isolation`.
    #[inline]
    pub fn ARB_robustness_application_isolation(&self) -> bool {
        self.ext[11]
    }

    /// Whether the driver advertises `GLX_ARB_robustness_share_group_isolation`.
    #[inline]
    pub fn ARB_robustness_share_group_isolation(&self) -> bool {
        self.ext[12]
    }

    /// Whether the driver advertises `GLX_ARB_vertex_buffer_object`.
    #[inline]
    pub fn ARB_vertex_buffer_object(&self) -> bool {
        self.ext[13]
    }

    /// Whether the driver advertises `GLX_EXT_buffer_age`.
    #[inline]
    pub fn EXT_buffer_age(&self) -> bool {
        self.ext[14]
    }

    /// Whether the driver advertises `GLX_EXT_context_priority`.
    #[inline]
    pub fn EXT_context_priority(&self) -> bool {
        self.ext[15]
    }

    /// Whether the driver advertises `GLX_EXT_create_context_es2_profile`.
    #[inline]
    pub fn EXT_create_context_es2_profile(&self) -> bool {
        self.ext[16]
    }

    /// Whether the driver advertises `GLX_EXT_create_context_es_profile`.
    #[inline]
    pub fn EXT_create_context_es_profile(&self) -> bool {
        self.ext[17]
    }

    /// Whether the driver advertises `GLX_EXT_fbconfig_packed_float`.
    #[inline]
    pub fn EXT_fbconfig_packed_float(&self) -> bool {
        self.ext[18]
    }

    /// Whether the driver advertises `GLX_EXT_framebuffer_sRGB`.
    #[inline]
    pub fn EXT_framebuffer_sRGB(&self) -> bool {
        self.ext[19]
    }

    /// Whether the driver advertises `GLX_EXT_get_drawable_type`.
    #[inline]
    pub fn EXT_get_drawable_type(&self) -> bool {
        self.ext[20]
    }

    /// Whether the driver advertises `GLX_EXT_import_context`.
    #[inline]
    pub fn EXT_import_context(&self) -> bool {
        self.ext[21]
    }

    /// Whether the driver advertises `GLX_EXT_libglvnd`.
    #[inline]
    pub fn EXT_libglvnd(&self) -> bool {
        self.ext[22]
    }

    /// Whether the driver advertises `GLX_EXT_no_config_context`.
    #[inline]
    pub fn EXT_no_config_context(&self) -> bool {
        self.ext[23]
    }

    /// Whether the driver advertises `GLX_EXT_stereo_tree`.
    #[inline]
    pub fn EXT_stereo_tree(&self) -> bool {
        self.ext[24]
    }

    /// Whether the driver advertises `GLX_EXT_swap_control`.
    #[inline]
    pub fn EXT_swap_control(&self) -> bool {
        self.ext[25]
    }

    /// Whether the driver advertises `GLX_EXT_swap_control_tear`.
    #[inline]
    pub fn EXT_swap_control_tear(&self) -> bool {
        self.ext[26]
    }

    /// Whether the driver advertises `GLX_EXT_texture_from_pixmap`.
    #[inline]
    pub fn EXT_texture_from_pixmap(&self) -> bool {
        self.ext[27]
    }

    /// Whether the driver advertises `GLX_EXT_visual_info`.
    #[inline]
    pub fn EXT_visual_info(&self) -> bool {
        self.ext[28]
    }

    /// Whether the driver advertises `GLX_EXT_visual_rating`.
    #[inline]
    pub fn EXT_visual_rating(&self) -> bool {
        self.ext[29]
    }

    /// Whether the driver advertises `GLX_INTEL_swap_event`.
    #[inline]
    pub fn INTEL_swap_event(&self) -> bool {
        self.ext[30]
    }

    /// Whether the driver advertises `GLX_MESA_agp_offset`.
    #[inline]
    pub fn MESA_agp_offset(&self) -> bool {
        self.ext[31]
    }

    /// Whether the driver advertises `GLX_MESA_copy_sub_buffer`.
    #[inline]
    pub fn MESA_copy_sub_buffer(&self) -> bool {
        self.ext[32]
    }

    /// Whether the driver advertises `GLX_MESA_pixmap_colormap`.
    #[inline]
    pub fn MESA_pixmap_colormap(&self) -> bool {
        self.ext[33]
    }

    /// Whether the driver advertises `GLX_MESA_query_renderer`.
    #[inline]
    pub fn MESA_query_renderer(&self) -> bool {
        self.ext[34]
    }

    /// Whether the driver advertises `GLX_MESA_release_buffers`.
    #[inline]
    pub fn MESA_release_buffers(&self) -> bool {
        self.ext[35]
    }

    /// Whether the driver advertises `GLX_MESA_set_3dfx_mode`.
    #[inline]
    pub fn MESA_set_3dfx_mode(&self) -> bool {
        self.ext[36]
    }

    /// Whether the driver advertises `GLX_MESA_swap_control`.
    #[inline]
    pub fn MESA_swap_control(&self) -> bool {
        self.ext[37]
    }

    /// Whether the driver advertises `GLX_NV_copy_buffer`.
    #[inline]
    pub fn NV_copy_buffer(&self) -> bool {
        self.ext[38]
    }

    /// Whether the driver advertises `GLX_NV_copy_image`.
    #[inline]
    pub fn NV_copy_image(&self) -> bool {
        self.ext[39]
    }

    /// Whether the driver advertises `GLX_NV_delay_before_swap`.
    #[inline]
    pub fn NV_delay_before_swap(&self) -> bool {
        self.ext[40]
    }

    /// Whether the driver advertises `GLX_NV_float_buffer`.
    #[inline]
    pub fn NV_float_buffer(&self) -> bool {
        self.ext[41]
    }

    /// Whether the driver advertises `GLX_NV_multigpu_context`.
    #[inline]
    pub fn NV_multigpu_context(&self) -> bool {
        self.ext[42]
    }

    /// Whether the driver advertises `GLX_NV_multisample_coverage`.
    #[inline]
    pub fn NV_multisample_coverage(&self) -> bool {
        self.ext[43]
    }

    /// Whether the driver advertises `GLX_NV_present_video`.
    #[inline]
    pub fn NV_present_video(&self) -> bool {
        self.ext[44]
    }

    /// Whether the driver advertises `GLX_NV_robustness_video_memory_purge`.
    #[inline]
    pub fn NV_robustness_video_memory_purge(&self) -> bool {
        self.ext[45]
    }

    /// Whether the driver advertises `GLX_NV_swap_group`.
    #[inline]
    pub fn NV_swap_group(&self) -> bool {
        self.ext[46]
    }

    /// Whether the driver advertises `GLX_NV_video_capture`.
    #[inline]
    pub fn NV_video_capture(&self) -> bool {
        self.ext[47]
    }

    /// Whether the driver advertises `GLX_NV_video_out`.
    #[inline]
    pub fn NV_video_out(&self) -> bool {
        self.ext[48]
    }

    /// Whether the driver advertises `GLX_OML_swap_method`.
    #[inline]
    pub fn OML_swap_method(&self) -> bool {
        self.ext[49]
    }

    /// Whether the driver advertises `GLX_OML_sync_control`.
    #[inline]
    pub fn OML_sync_control(&self) -> bool {
        self.ext[50]
    }

    /// Whether the driver advertises `GLX_SGIS_blended_overlay`.
    #[inline]
    pub fn SGIS_blended_overlay(&self) -> bool {
        self.ext[51]
    }

    /// Whether the driver advertises `GLX_SGIS_multisample`.
    #[inline]
    pub fn SGIS_multisample(&self) -> bool {
        self.ext[52]
    }

    /// Whether the driver advertises `GLX_SGIS_shared_multisample`.
    #[inline]
    pub fn SGIS_shared_multisample(&self) -> bool {
        self.ext[53]
    }

    /// Whether the driver advertises `GLX_SGIX_fbconfig`.
    #[inline]
    pub fn SGIX_fbconfig(&self) -> bool {
        self.ext[54]
    }

    /// Whether the driver advertises `GLX_SGIX_hyperpipe`.
    #[inline]
    pub fn SGIX_hyperpipe(&self) -> bool {
        self.ext[55]
    }

    /// Whether the driver advertises `GLX_SGIX_pbuffer`.
    #[inline]
    pub fn SGIX_pbuffer(&self) -> bool {
        self.ext[56]
    }

    /// Whether the driver advertises `GLX_SGIX_swap_barrier`.
    #[inline]
    pub fn SGIX_swap_barrier(&self) -> bool {
        self.ext[57]
    }

    /// Whether the driver advertises `GLX_SGIX_swap_group`.
    #[inline]
    pub fn SGIX_swap_group(&self) -> bool {
        self.ext[58]
    }

    /// Whether the driver advertises `GLX_SGIX_video_resize`.
    #[inline]
    pub fn SGIX_video_resize(&self) -> bool {
        self.ext[59]
    }

    /// Whether the driver advertises `GLX_SGIX_visual_select_group`.
    #[inline]
    pub fn SGIX_visual_select_group(&self) -> bool {
        self.ext[60]
    }

    /// Whether the driver advertises `GLX_SGI_cushion`.
    #[inline]
    pub fn SGI_cushion(&self) -> bool {
        self.ext[61]
    }

    /// Whether the driver advertises `GLX_SGI_make_current_read`.
    #[inline]
    pub fn SGI_make_current_read(&self) -> bool {
        self.ext[62]
    }

    /// Whether the driver advertises `GLX_SGI_swap_control`.
    #[inline]
    pub fn SGI_swap_control(&self) -> bool {
        self.ext[63]
    }

    /// Whether the driver advertises `GLX_SGI_video_sync`.
    #[inline]
    pub fn SGI_video_sync(&self) -> bool {
        self.ext[64]
    }

    /// Whether the driver advertises `GLX_SUN_get_transparent_index`.
    #[inline]
    pub fn SUN_get_transparent_index(&self) -> bool {
        self.ext[65]
    }

    /// Whether the driver supports `GLX_VERSION_1_0`.
    #[inline]
    pub fn VERSION_1_0(&self) -> bool {
        self.feat[0]
    }

    /// Whether the driver supports `GLX_VERSION_1_1`.
    #[inline]
    pub fn VERSION_1_1(&self) -> bool {
        self.feat[1]
    }

    /// Whether the driver supports `GLX_VERSION_1_2`.
    #[inline]
    pub fn VERSION_1_2(&self) -> bool {
        self.feat[2]
    }

    /// Whether the driver supports `GLX_VERSION_1_3`.
    #[inline]
    pub fn VERSION_1_3(&self) -> bool {
        self.feat[3]
    }

    /// Whether the driver supports `GLX_VERSION_1_4`.
    #[inline]
    pub fn VERSION_1_4(&self) -> bool {
        self.feat[4]
    }
}
