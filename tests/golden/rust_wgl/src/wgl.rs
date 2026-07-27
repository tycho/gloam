#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, clippy::all)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::ffi::{CStr, c_char, c_void};

// ── WGL base types ──────────────────────────────────────────
pub type HPBUFFERARB = *mut HPBUFFERARB__;
pub type HPBUFFEREXT = *mut HPBUFFEREXT__;
pub type HVIDEOOUTPUTDEVICENV = *mut HVIDEOOUTPUTDEVICENV__;
pub type HPVIDEODEV = *mut HPVIDEODEV__;
pub type HPGPUNV = *mut HPGPUNV__;
pub type HGPUNV = *mut HGPUNV__;
pub type HVIDEOINPUTDEVICENV = *mut HVIDEOINPUTDEVICENV__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _GPU_DEVICE {
    pub cb: DWORD,
    pub DeviceName: [CHAR; 32],
    pub DeviceString: [CHAR; 128],
    pub Flags: DWORD,
    pub rcVirtualScreen: RECT,
}
pub type GPU_DEVICE = _GPU_DEVICE;
pub type PGPU_DEVICE = *mut _GPU_DEVICE;

// Opaque C struct types (incomplete in the spec).  Zero-sized so
// pointers to them stay distinct types, exactly as in C.
#[repr(C)]
pub struct HPBUFFERARB__ {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct HPBUFFEREXT__ {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct HVIDEOOUTPUTDEVICENV__ {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct HPVIDEODEV__ {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct HPGPUNV__ {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct HGPUNV__ {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct HVIDEOINPUTDEVICENV__ {
    _opaque: [u8; 0],
}

// Platform types (in C these come from platform headers via
// khrplatform/eglplatform); declared here with their per-target
// ABI shapes.
pub type DWORD = u32;
pub type CHAR = c_char;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
pub type HDC = *mut c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PIXELFORMATDESCRIPTOR {
    pub nSize: u16,
    pub nVersion: u16,
    pub dwFlags: u32,
    pub iPixelType: u8,
    pub cColorBits: u8,
    pub cRedBits: u8,
    pub cRedShift: u8,
    pub cGreenBits: u8,
    pub cGreenShift: u8,
    pub cBlueBits: u8,
    pub cBlueShift: u8,
    pub cAlphaBits: u8,
    pub cAlphaShift: u8,
    pub cAccumBits: u8,
    pub cAccumRedBits: u8,
    pub cAccumGreenBits: u8,
    pub cAccumBlueBits: u8,
    pub cAccumAlphaBits: u8,
    pub cDepthBits: u8,
    pub cStencilBits: u8,
    pub cAuxBuffers: u8,
    pub iLayerType: u8,
    pub bReserved: u8,
    pub dwLayerMask: u32,
    pub dwVisibleMask: u32,
    pub dwDamageMask: u32,
}
pub type UINT = u32;
pub type HENHMETAFILE = *mut c_void;
pub type BOOL = i32;
pub type HGLRC = *mut c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LAYERPLANEDESCRIPTOR {
    pub nSize: u16,
    pub nVersion: u16,
    pub dwFlags: u32,
    pub iPixelType: u8,
    pub cColorBits: u8,
    pub cRedBits: u8,
    pub cRedShift: u8,
    pub cGreenBits: u8,
    pub cGreenShift: u8,
    pub cBlueBits: u8,
    pub cBlueShift: u8,
    pub cAlphaBits: u8,
    pub cAlphaShift: u8,
    pub cAccumBits: u8,
    pub cAccumRedBits: u8,
    pub cAccumGreenBits: u8,
    pub cAccumBlueBits: u8,
    pub cAccumAlphaBits: u8,
    pub cDepthBits: u8,
    pub cStencilBits: u8,
    pub cAuxBuffers: u8,
    pub iLayerPlane: u8,
    pub bReserved: u8,
    pub crTransparent: COLORREF,
}
pub type COLORREF = u32;
pub type LPCSTR = *const c_char;
pub type PROC = Option<unsafe extern "system" fn()>;
pub type FLOAT = f32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct POINTFLOAT {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GLYPHMETRICSFLOAT {
    pub gmfBlackBoxX: f32,
    pub gmfBlackBoxY: f32,
    pub gmfptGlyphOrigin: POINTFLOAT,
    pub gmfCellIncX: f32,
    pub gmfCellIncY: f32,
}
pub type LPGLYPHMETRICSFLOAT = *mut GLYPHMETRICSFLOAT;
pub type GLint = i32;
pub type GLbitfield = u32;
pub type GLenum = u32;
pub type INT = i32;
pub type HANDLE = *mut c_void;
pub type GLushort = u16;
pub type GLboolean = u8;
pub type GLuint = u32;
pub type USHORT = u16;
pub type LPVOID = *mut c_void;
pub type GLsizei = i32;
pub type GLfloat = f32;
pub type INT32 = i32;
pub type INT64 = i64;

// ── Constants ───────────────────────────────────────────────
pub const WGL_SWAP_MAIN_PLANE: u32 = 0x00000001;
pub const WGL_SWAP_OVERLAY1: u32 = 0x00000002;
pub const WGL_SWAP_OVERLAY2: u32 = 0x00000004;
pub const WGL_SWAP_OVERLAY3: u32 = 0x00000008;
pub const WGL_SWAP_OVERLAY4: u32 = 0x00000010;
pub const WGL_SWAP_OVERLAY5: u32 = 0x00000020;
pub const WGL_SWAP_OVERLAY6: u32 = 0x00000040;
pub const WGL_SWAP_OVERLAY7: u32 = 0x00000080;
pub const WGL_SWAP_OVERLAY8: u32 = 0x00000100;
pub const WGL_SWAP_OVERLAY9: u32 = 0x00000200;
pub const WGL_SWAP_OVERLAY10: u32 = 0x00000400;
pub const WGL_SWAP_OVERLAY11: u32 = 0x00000800;
pub const WGL_SWAP_OVERLAY12: u32 = 0x00001000;
pub const WGL_SWAP_OVERLAY13: u32 = 0x00002000;
pub const WGL_SWAP_OVERLAY14: u32 = 0x00004000;
pub const WGL_SWAP_OVERLAY15: u32 = 0x00008000;
pub const WGL_SWAP_UNDERLAY1: u32 = 0x00010000;
pub const WGL_SWAP_UNDERLAY2: u32 = 0x00020000;
pub const WGL_SWAP_UNDERLAY3: u32 = 0x00040000;
pub const WGL_SWAP_UNDERLAY4: u32 = 0x00080000;
pub const WGL_SWAP_UNDERLAY5: u32 = 0x00100000;
pub const WGL_SWAP_UNDERLAY6: u32 = 0x00200000;
pub const WGL_SWAP_UNDERLAY7: u32 = 0x00400000;
pub const WGL_SWAP_UNDERLAY8: u32 = 0x00800000;
pub const WGL_SWAP_UNDERLAY9: u32 = 0x01000000;
pub const WGL_SWAP_UNDERLAY10: u32 = 0x02000000;
pub const WGL_SWAP_UNDERLAY11: u32 = 0x04000000;
pub const WGL_SWAP_UNDERLAY12: u32 = 0x08000000;
pub const WGL_SWAP_UNDERLAY13: u32 = 0x10000000;
pub const WGL_SWAP_UNDERLAY14: u32 = 0x20000000;
pub const WGL_SWAP_UNDERLAY15: u32 = 0x40000000;
pub const WGL_FRONT_COLOR_BUFFER_BIT_ARB: u32 = 0x00000001;
pub const WGL_BACK_COLOR_BUFFER_BIT_ARB: u32 = 0x00000002;
pub const WGL_DEPTH_BUFFER_BIT_ARB: u32 = 0x00000004;
pub const WGL_STENCIL_BUFFER_BIT_ARB: u32 = 0x00000008;
pub const WGL_CONTEXT_DEBUG_BIT_ARB: u32 = 0x00000001;
pub const WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: u32 = 0x00000002;
pub const WGL_CONTEXT_ROBUST_ACCESS_BIT_ARB: u32 = 0x00000004;
pub const WGL_CONTEXT_RESET_ISOLATION_BIT_ARB: u32 = 0x00000008;
pub const WGL_CONTEXT_CORE_PROFILE_BIT_ARB: u32 = 0x00000001;
pub const WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: u32 = 0x00000002;
pub const WGL_CONTEXT_ES_PROFILE_BIT_EXT: u32 = 0x00000004;
pub const WGL_CONTEXT_ES2_PROFILE_BIT_EXT: u32 = 0x00000004;
pub const WGL_IMAGE_BUFFER_MIN_ACCESS_I3D: u32 = 0x00000001;
pub const WGL_IMAGE_BUFFER_LOCK_I3D: u32 = 0x00000002;
pub const WGL_ACCESS_READ_ONLY_NV: u32 = 0x00000000;
pub const WGL_ACCESS_READ_WRITE_NV: u32 = 0x00000001;
pub const WGL_ACCESS_WRITE_DISCARD_NV: u32 = 0x00000002;
pub const WGL_CONTEXT_RELEASE_BEHAVIOR_NONE_ARB: i32 = 0;
pub const WGL_FONT_LINES: i32 = 0;
pub const WGL_FONT_POLYGONS: i32 = 1;
pub const WGL_GPU_VENDOR_AMD: i32 = 0x1F00;
pub const WGL_GPU_RENDERER_STRING_AMD: i32 = 0x1F01;
pub const WGL_GPU_OPENGL_VERSION_STRING_AMD: i32 = 0x1F02;
pub const WGL_NUMBER_PIXEL_FORMATS_ARB: i32 = 0x2000;
pub const WGL_NUMBER_PIXEL_FORMATS_EXT: i32 = 0x2000;
pub const WGL_DRAW_TO_WINDOW_ARB: i32 = 0x2001;
pub const WGL_DRAW_TO_WINDOW_EXT: i32 = 0x2001;
pub const WGL_DRAW_TO_BITMAP_ARB: i32 = 0x2002;
pub const WGL_DRAW_TO_BITMAP_EXT: i32 = 0x2002;
pub const WGL_ACCELERATION_ARB: i32 = 0x2003;
pub const WGL_ACCELERATION_EXT: i32 = 0x2003;
pub const WGL_NEED_PALETTE_ARB: i32 = 0x2004;
pub const WGL_NEED_PALETTE_EXT: i32 = 0x2004;
pub const WGL_NEED_SYSTEM_PALETTE_ARB: i32 = 0x2005;
pub const WGL_NEED_SYSTEM_PALETTE_EXT: i32 = 0x2005;
pub const WGL_SWAP_LAYER_BUFFERS_ARB: i32 = 0x2006;
pub const WGL_SWAP_LAYER_BUFFERS_EXT: i32 = 0x2006;
pub const WGL_SWAP_METHOD_ARB: i32 = 0x2007;
pub const WGL_SWAP_METHOD_EXT: i32 = 0x2007;
pub const WGL_NUMBER_OVERLAYS_ARB: i32 = 0x2008;
pub const WGL_NUMBER_OVERLAYS_EXT: i32 = 0x2008;
pub const WGL_NUMBER_UNDERLAYS_ARB: i32 = 0x2009;
pub const WGL_NUMBER_UNDERLAYS_EXT: i32 = 0x2009;
pub const WGL_TRANSPARENT_ARB: i32 = 0x200A;
pub const WGL_TRANSPARENT_EXT: i32 = 0x200A;
pub const WGL_TRANSPARENT_VALUE_EXT: i32 = 0x200B;
pub const WGL_SHARE_DEPTH_ARB: i32 = 0x200C;
pub const WGL_SHARE_DEPTH_EXT: i32 = 0x200C;
pub const WGL_SHARE_STENCIL_ARB: i32 = 0x200D;
pub const WGL_SHARE_STENCIL_EXT: i32 = 0x200D;
pub const WGL_SHARE_ACCUM_ARB: i32 = 0x200E;
pub const WGL_SHARE_ACCUM_EXT: i32 = 0x200E;
pub const WGL_SUPPORT_GDI_ARB: i32 = 0x200F;
pub const WGL_SUPPORT_GDI_EXT: i32 = 0x200F;
pub const WGL_SUPPORT_OPENGL_ARB: i32 = 0x2010;
pub const WGL_SUPPORT_OPENGL_EXT: i32 = 0x2010;
pub const WGL_DOUBLE_BUFFER_ARB: i32 = 0x2011;
pub const WGL_DOUBLE_BUFFER_EXT: i32 = 0x2011;
pub const WGL_STEREO_ARB: i32 = 0x2012;
pub const WGL_STEREO_EXT: i32 = 0x2012;
pub const WGL_PIXEL_TYPE_ARB: i32 = 0x2013;
pub const WGL_PIXEL_TYPE_EXT: i32 = 0x2013;
pub const WGL_COLOR_BITS_ARB: i32 = 0x2014;
pub const WGL_COLOR_BITS_EXT: i32 = 0x2014;
pub const WGL_RED_BITS_ARB: i32 = 0x2015;
pub const WGL_RED_BITS_EXT: i32 = 0x2015;
pub const WGL_RED_SHIFT_ARB: i32 = 0x2016;
pub const WGL_RED_SHIFT_EXT: i32 = 0x2016;
pub const WGL_GREEN_BITS_ARB: i32 = 0x2017;
pub const WGL_GREEN_BITS_EXT: i32 = 0x2017;
pub const WGL_GREEN_SHIFT_ARB: i32 = 0x2018;
pub const WGL_GREEN_SHIFT_EXT: i32 = 0x2018;
pub const WGL_BLUE_BITS_ARB: i32 = 0x2019;
pub const WGL_BLUE_BITS_EXT: i32 = 0x2019;
pub const WGL_BLUE_SHIFT_ARB: i32 = 0x201A;
pub const WGL_BLUE_SHIFT_EXT: i32 = 0x201A;
pub const WGL_ALPHA_BITS_ARB: i32 = 0x201B;
pub const WGL_ALPHA_BITS_EXT: i32 = 0x201B;
pub const WGL_ALPHA_SHIFT_ARB: i32 = 0x201C;
pub const WGL_ALPHA_SHIFT_EXT: i32 = 0x201C;
pub const WGL_ACCUM_BITS_ARB: i32 = 0x201D;
pub const WGL_ACCUM_BITS_EXT: i32 = 0x201D;
pub const WGL_ACCUM_RED_BITS_ARB: i32 = 0x201E;
pub const WGL_ACCUM_RED_BITS_EXT: i32 = 0x201E;
pub const WGL_ACCUM_GREEN_BITS_ARB: i32 = 0x201F;
pub const WGL_ACCUM_GREEN_BITS_EXT: i32 = 0x201F;
pub const WGL_ACCUM_BLUE_BITS_ARB: i32 = 0x2020;
pub const WGL_ACCUM_BLUE_BITS_EXT: i32 = 0x2020;
pub const WGL_ACCUM_ALPHA_BITS_ARB: i32 = 0x2021;
pub const WGL_ACCUM_ALPHA_BITS_EXT: i32 = 0x2021;
pub const WGL_DEPTH_BITS_ARB: i32 = 0x2022;
pub const WGL_DEPTH_BITS_EXT: i32 = 0x2022;
pub const WGL_STENCIL_BITS_ARB: i32 = 0x2023;
pub const WGL_STENCIL_BITS_EXT: i32 = 0x2023;
pub const WGL_AUX_BUFFERS_ARB: i32 = 0x2024;
pub const WGL_AUX_BUFFERS_EXT: i32 = 0x2024;
pub const WGL_NO_ACCELERATION_ARB: i32 = 0x2025;
pub const WGL_NO_ACCELERATION_EXT: i32 = 0x2025;
pub const WGL_GENERIC_ACCELERATION_ARB: i32 = 0x2026;
pub const WGL_GENERIC_ACCELERATION_EXT: i32 = 0x2026;
pub const WGL_FULL_ACCELERATION_ARB: i32 = 0x2027;
pub const WGL_FULL_ACCELERATION_EXT: i32 = 0x2027;
pub const WGL_SWAP_EXCHANGE_ARB: i32 = 0x2028;
pub const WGL_SWAP_EXCHANGE_EXT: i32 = 0x2028;
pub const WGL_SWAP_COPY_ARB: i32 = 0x2029;
pub const WGL_SWAP_COPY_EXT: i32 = 0x2029;
pub const WGL_SWAP_UNDEFINED_ARB: i32 = 0x202A;
pub const WGL_SWAP_UNDEFINED_EXT: i32 = 0x202A;
pub const WGL_TYPE_RGBA_ARB: i32 = 0x202B;
pub const WGL_TYPE_RGBA_EXT: i32 = 0x202B;
pub const WGL_TYPE_COLORINDEX_ARB: i32 = 0x202C;
pub const WGL_TYPE_COLORINDEX_EXT: i32 = 0x202C;
pub const WGL_DRAW_TO_PBUFFER_ARB: i32 = 0x202D;
pub const WGL_DRAW_TO_PBUFFER_EXT: i32 = 0x202D;
pub const WGL_MAX_PBUFFER_PIXELS_ARB: i32 = 0x202E;
pub const WGL_MAX_PBUFFER_PIXELS_EXT: i32 = 0x202E;
pub const WGL_MAX_PBUFFER_WIDTH_ARB: i32 = 0x202F;
pub const WGL_MAX_PBUFFER_WIDTH_EXT: i32 = 0x202F;
pub const WGL_MAX_PBUFFER_HEIGHT_ARB: i32 = 0x2030;
pub const WGL_MAX_PBUFFER_HEIGHT_EXT: i32 = 0x2030;
pub const WGL_OPTIMAL_PBUFFER_WIDTH_EXT: i32 = 0x2031;
pub const WGL_OPTIMAL_PBUFFER_HEIGHT_EXT: i32 = 0x2032;
pub const WGL_PBUFFER_LARGEST_ARB: i32 = 0x2033;
pub const WGL_PBUFFER_LARGEST_EXT: i32 = 0x2033;
pub const WGL_PBUFFER_WIDTH_ARB: i32 = 0x2034;
pub const WGL_PBUFFER_WIDTH_EXT: i32 = 0x2034;
pub const WGL_PBUFFER_HEIGHT_ARB: i32 = 0x2035;
pub const WGL_PBUFFER_HEIGHT_EXT: i32 = 0x2035;
pub const WGL_PBUFFER_LOST_ARB: i32 = 0x2036;
pub const WGL_TRANSPARENT_RED_VALUE_ARB: i32 = 0x2037;
pub const WGL_TRANSPARENT_GREEN_VALUE_ARB: i32 = 0x2038;
pub const WGL_TRANSPARENT_BLUE_VALUE_ARB: i32 = 0x2039;
pub const WGL_TRANSPARENT_ALPHA_VALUE_ARB: i32 = 0x203A;
pub const WGL_TRANSPARENT_INDEX_VALUE_ARB: i32 = 0x203B;
pub const WGL_DEPTH_FLOAT_EXT: i32 = 0x2040;
pub const WGL_SAMPLE_BUFFERS_ARB: i32 = 0x2041;
pub const WGL_SAMPLE_BUFFERS_EXT: i32 = 0x2041;
pub const WGL_COVERAGE_SAMPLES_NV: i32 = 0x2042;
pub const WGL_SAMPLES_ARB: i32 = 0x2042;
pub const WGL_SAMPLES_EXT: i32 = 0x2042;
pub const ERROR_INVALID_PIXEL_TYPE_ARB: i32 = 0x2043;
pub const ERROR_INVALID_PIXEL_TYPE_EXT: i32 = 0x2043;
pub const WGL_GENLOCK_SOURCE_MULTIVIEW_I3D: i32 = 0x2044;
pub const WGL_GENLOCK_SOURCE_EXTERNAL_SYNC_I3D: i32 = 0x2045;
pub const WGL_GENLOCK_SOURCE_EXTERNAL_FIELD_I3D: i32 = 0x2046;
pub const WGL_GENLOCK_SOURCE_EXTERNAL_TTL_I3D: i32 = 0x2047;
pub const WGL_GENLOCK_SOURCE_DIGITAL_SYNC_I3D: i32 = 0x2048;
pub const WGL_GENLOCK_SOURCE_DIGITAL_FIELD_I3D: i32 = 0x2049;
pub const WGL_GENLOCK_SOURCE_EDGE_FALLING_I3D: i32 = 0x204A;
pub const WGL_GENLOCK_SOURCE_EDGE_RISING_I3D: i32 = 0x204B;
pub const WGL_GENLOCK_SOURCE_EDGE_BOTH_I3D: i32 = 0x204C;
pub const WGL_GAMMA_TABLE_SIZE_I3D: i32 = 0x204E;
pub const WGL_GAMMA_EXCLUDE_DESKTOP_I3D: i32 = 0x204F;
pub const WGL_DIGITAL_VIDEO_CURSOR_ALPHA_FRAMEBUFFER_I3D: i32 = 0x2050;
pub const WGL_DIGITAL_VIDEO_CURSOR_ALPHA_VALUE_I3D: i32 = 0x2051;
pub const WGL_DIGITAL_VIDEO_CURSOR_INCLUDED_I3D: i32 = 0x2052;
pub const WGL_DIGITAL_VIDEO_GAMMA_CORRECTED_I3D: i32 = 0x2053;
pub const ERROR_INCOMPATIBLE_DEVICE_CONTEXTS_ARB: i32 = 0x2054;
pub const WGL_STEREO_EMITTER_ENABLE_3DL: i32 = 0x2055;
pub const WGL_STEREO_EMITTER_DISABLE_3DL: i32 = 0x2056;
pub const WGL_STEREO_POLARITY_NORMAL_3DL: i32 = 0x2057;
pub const WGL_STEREO_POLARITY_INVERT_3DL: i32 = 0x2058;
pub const WGL_SAMPLE_BUFFERS_3DFX: i32 = 0x2060;
pub const WGL_SAMPLES_3DFX: i32 = 0x2061;
pub const WGL_BIND_TO_TEXTURE_RGB_ARB: i32 = 0x2070;
pub const WGL_BIND_TO_TEXTURE_RGBA_ARB: i32 = 0x2071;
pub const WGL_TEXTURE_FORMAT_ARB: i32 = 0x2072;
pub const WGL_TEXTURE_TARGET_ARB: i32 = 0x2073;
pub const WGL_MIPMAP_TEXTURE_ARB: i32 = 0x2074;
pub const WGL_TEXTURE_RGB_ARB: i32 = 0x2075;
pub const WGL_TEXTURE_RGBA_ARB: i32 = 0x2076;
pub const WGL_NO_TEXTURE_ARB: i32 = 0x2077;
pub const WGL_TEXTURE_CUBE_MAP_ARB: i32 = 0x2078;
pub const WGL_TEXTURE_1D_ARB: i32 = 0x2079;
pub const WGL_TEXTURE_2D_ARB: i32 = 0x207A;
pub const WGL_MIPMAP_LEVEL_ARB: i32 = 0x207B;
pub const WGL_CUBE_MAP_FACE_ARB: i32 = 0x207C;
pub const WGL_TEXTURE_CUBE_MAP_POSITIVE_X_ARB: i32 = 0x207D;
pub const WGL_TEXTURE_CUBE_MAP_NEGATIVE_X_ARB: i32 = 0x207E;
pub const WGL_TEXTURE_CUBE_MAP_POSITIVE_Y_ARB: i32 = 0x207F;
pub const WGL_TEXTURE_CUBE_MAP_NEGATIVE_Y_ARB: i32 = 0x2080;
pub const WGL_TEXTURE_CUBE_MAP_POSITIVE_Z_ARB: i32 = 0x2081;
pub const WGL_TEXTURE_CUBE_MAP_NEGATIVE_Z_ARB: i32 = 0x2082;
pub const WGL_FRONT_LEFT_ARB: i32 = 0x2083;
pub const WGL_FRONT_RIGHT_ARB: i32 = 0x2084;
pub const WGL_BACK_LEFT_ARB: i32 = 0x2085;
pub const WGL_BACK_RIGHT_ARB: i32 = 0x2086;
pub const WGL_AUX0_ARB: i32 = 0x2087;
pub const WGL_AUX1_ARB: i32 = 0x2088;
pub const WGL_AUX2_ARB: i32 = 0x2089;
pub const WGL_AUX3_ARB: i32 = 0x208A;
pub const WGL_AUX4_ARB: i32 = 0x208B;
pub const WGL_AUX5_ARB: i32 = 0x208C;
pub const WGL_AUX6_ARB: i32 = 0x208D;
pub const WGL_AUX7_ARB: i32 = 0x208E;
pub const WGL_AUX8_ARB: i32 = 0x208F;
pub const WGL_AUX9_ARB: i32 = 0x2090;
pub const WGL_CONTEXT_MAJOR_VERSION_ARB: i32 = 0x2091;
pub const WGL_CONTEXT_MINOR_VERSION_ARB: i32 = 0x2092;
pub const WGL_CONTEXT_LAYER_PLANE_ARB: i32 = 0x2093;
pub const WGL_CONTEXT_FLAGS_ARB: i32 = 0x2094;
pub const ERROR_INVALID_VERSION_ARB: i32 = 0x2095;
pub const ERROR_INVALID_PROFILE_ARB: i32 = 0x2096;
pub const WGL_CONTEXT_RELEASE_BEHAVIOR_ARB: i32 = 0x2097;
pub const WGL_CONTEXT_RELEASE_BEHAVIOR_FLUSH_ARB: i32 = 0x2098;
pub const WGL_BIND_TO_TEXTURE_RECTANGLE_RGB_NV: i32 = 0x20A0;
pub const WGL_BIND_TO_TEXTURE_RECTANGLE_RGBA_NV: i32 = 0x20A1;
pub const WGL_TEXTURE_RECTANGLE_NV: i32 = 0x20A2;
pub const WGL_BIND_TO_TEXTURE_DEPTH_NV: i32 = 0x20A3;
pub const WGL_BIND_TO_TEXTURE_RECTANGLE_DEPTH_NV: i32 = 0x20A4;
pub const WGL_DEPTH_TEXTURE_FORMAT_NV: i32 = 0x20A5;
pub const WGL_TEXTURE_DEPTH_COMPONENT_NV: i32 = 0x20A6;
pub const WGL_DEPTH_COMPONENT_NV: i32 = 0x20A7;
pub const WGL_TYPE_RGBA_UNSIGNED_FLOAT_EXT: i32 = 0x20A8;
pub const WGL_FRAMEBUFFER_SRGB_CAPABLE_ARB: i32 = 0x20A9;
pub const WGL_FRAMEBUFFER_SRGB_CAPABLE_EXT: i32 = 0x20A9;
pub const WGL_CONTEXT_MULTIGPU_ATTRIB_NV: i32 = 0x20AA;
pub const WGL_CONTEXT_MULTIGPU_ATTRIB_SINGLE_NV: i32 = 0x20AB;
pub const WGL_CONTEXT_MULTIGPU_ATTRIB_AFR_NV: i32 = 0x20AC;
pub const WGL_CONTEXT_MULTIGPU_ATTRIB_MULTICAST_NV: i32 = 0x20AD;
pub const WGL_CONTEXT_MULTIGPU_ATTRIB_MULTI_DISPLAY_MULTICAST_NV: i32 = 0x20AE;
pub const WGL_FLOAT_COMPONENTS_NV: i32 = 0x20B0;
pub const WGL_BIND_TO_TEXTURE_RECTANGLE_FLOAT_R_NV: i32 = 0x20B1;
pub const WGL_BIND_TO_TEXTURE_RECTANGLE_FLOAT_RG_NV: i32 = 0x20B2;
pub const WGL_BIND_TO_TEXTURE_RECTANGLE_FLOAT_RGB_NV: i32 = 0x20B3;
pub const WGL_BIND_TO_TEXTURE_RECTANGLE_FLOAT_RGBA_NV: i32 = 0x20B4;
pub const WGL_TEXTURE_FLOAT_R_NV: i32 = 0x20B5;
pub const WGL_TEXTURE_FLOAT_RG_NV: i32 = 0x20B6;
pub const WGL_TEXTURE_FLOAT_RGB_NV: i32 = 0x20B7;
pub const WGL_TEXTURE_FLOAT_RGBA_NV: i32 = 0x20B8;
pub const WGL_COLOR_SAMPLES_NV: i32 = 0x20B9;
pub const WGL_BIND_TO_VIDEO_RGB_NV: i32 = 0x20C0;
pub const WGL_BIND_TO_VIDEO_RGBA_NV: i32 = 0x20C1;
pub const WGL_BIND_TO_VIDEO_RGB_AND_DEPTH_NV: i32 = 0x20C2;
pub const WGL_VIDEO_OUT_COLOR_NV: i32 = 0x20C3;
pub const WGL_VIDEO_OUT_ALPHA_NV: i32 = 0x20C4;
pub const WGL_VIDEO_OUT_DEPTH_NV: i32 = 0x20C5;
pub const WGL_VIDEO_OUT_COLOR_AND_ALPHA_NV: i32 = 0x20C6;
pub const WGL_VIDEO_OUT_COLOR_AND_DEPTH_NV: i32 = 0x20C7;
pub const WGL_VIDEO_OUT_FRAME: i32 = 0x20C8;
pub const WGL_VIDEO_OUT_FIELD_1: i32 = 0x20C9;
pub const WGL_VIDEO_OUT_FIELD_2: i32 = 0x20CA;
pub const WGL_VIDEO_OUT_STACKED_FIELDS_1_2: i32 = 0x20CB;
pub const WGL_VIDEO_OUT_STACKED_FIELDS_2_1: i32 = 0x20CC;
pub const WGL_UNIQUE_ID_NV: i32 = 0x20CE;
pub const WGL_NUM_VIDEO_CAPTURE_SLOTS_NV: i32 = 0x20CF;
pub const ERROR_INCOMPATIBLE_AFFINITY_MASKS_NV: i32 = 0x20D0;
pub const ERROR_MISSING_AFFINITY_MASK_NV: i32 = 0x20D1;
pub const WGL_NUM_VIDEO_SLOTS_NV: i32 = 0x20F0;
pub const WGL_TYPE_RGBA_FLOAT_ARB: i32 = 0x21A0;
pub const WGL_TYPE_RGBA_FLOAT_ATI: i32 = 0x21A0;
pub const WGL_GPU_FASTEST_TARGET_GPUS_AMD: i32 = 0x21A2;
pub const WGL_GPU_RAM_AMD: i32 = 0x21A3;
pub const WGL_GPU_CLOCK_AMD: i32 = 0x21A4;
pub const WGL_GPU_NUM_PIPES_AMD: i32 = 0x21A5;
pub const WGL_TEXTURE_RECTANGLE_ATI: i32 = 0x21A5;
pub const WGL_GPU_NUM_SIMD_AMD: i32 = 0x21A6;
pub const WGL_GPU_NUM_RB_AMD: i32 = 0x21A7;
pub const WGL_GPU_NUM_SPI_AMD: i32 = 0x21A8;
pub const WGL_COLORSPACE_EXT: i32 = 0x309D;
pub const WGL_COLORSPACE_SRGB_EXT: i32 = 0x3089;
pub const WGL_COLORSPACE_LINEAR_EXT: i32 = 0x308A;
pub const WGL_CONTEXT_OPENGL_NO_ERROR_ARB: i32 = 0x31B3;
pub const WGL_LOSE_CONTEXT_ON_RESET_ARB: i32 = 0x8252;
pub const WGL_CONTEXT_RESET_NOTIFICATION_STRATEGY_ARB: i32 = 0x8256;
pub const WGL_NO_RESET_NOTIFICATION_ARB: i32 = 0x8261;
pub const WGL_CONTEXT_PROFILE_MASK_ARB: i32 = 0x9126;

// ── Command table ───────────────────────────────────────────
pub const COMMAND_COUNT: usize = 145;
pub const FEATURE_COUNT: usize = 1;

#[rustfmt::skip]
static FN_NAME_DATA: &[u8] = b"\
    ChoosePixelFormat\0\
    DescribePixelFormat\0\
    GetEnhMetaFilePixelFormat\0\
    GetPixelFormat\0\
    SetPixelFormat\0\
    SwapBuffers\0\
    wglCopyContext\0\
    wglCreateContext\0\
    wglCreateLayerContext\0\
    wglDeleteContext\0\
    wglDescribeLayerPlane\0\
    wglGetCurrentContext\0\
    wglGetCurrentDC\0\
    wglGetLayerPaletteEntries\0\
    wglGetProcAddress\0\
    wglMakeCurrent\0\
    wglRealizeLayerPalette\0\
    wglSetLayerPaletteEntries\0\
    wglShareLists\0\
    wglSwapLayerBuffers\0\
    wglUseFontBitmaps\0\
    wglUseFontBitmapsA\0\
    wglUseFontBitmapsW\0\
    wglUseFontOutlines\0\
    wglUseFontOutlinesA\0\
    wglUseFontOutlinesW\0\
    wglSetStereoEmitterState3DL\0\
    wglBlitContextFramebufferAMD\0\
    wglCreateAssociatedContextAMD\0\
    wglCreateAssociatedContextAttribsAMD\0\
    wglDeleteAssociatedContextAMD\0\
    wglGetContextGPUIDAMD\0\
    wglGetCurrentAssociatedContextAMD\0\
    wglGetGPUIDsAMD\0\
    wglGetGPUInfoAMD\0\
    wglMakeAssociatedContextCurrentAMD\0\
    wglCreateBufferRegionARB\0\
    wglDeleteBufferRegionARB\0\
    wglRestoreBufferRegionARB\0\
    wglSaveBufferRegionARB\0\
    wglCreateContextAttribsARB\0\
    wglGetExtensionsStringARB\0\
    wglGetCurrentReadDCARB\0\
    wglMakeContextCurrentARB\0\
    wglCreatePbufferARB\0\
    wglDestroyPbufferARB\0\
    wglGetPbufferDCARB\0\
    wglQueryPbufferARB\0\
    wglReleasePbufferDCARB\0\
    wglChoosePixelFormatARB\0\
    wglGetPixelFormatAttribfvARB\0\
    wglGetPixelFormatAttribivARB\0\
    wglBindTexImageARB\0\
    wglReleaseTexImageARB\0\
    wglSetPbufferAttribARB\0\
    wglBindDisplayColorTableEXT\0\
    wglCreateDisplayColorTableEXT\0\
    wglDestroyDisplayColorTableEXT\0\
    wglLoadDisplayColorTableEXT\0\
    wglGetExtensionsStringEXT\0\
    wglGetCurrentReadDCEXT\0\
    wglMakeContextCurrentEXT\0\
    wglCreatePbufferEXT\0\
    wglDestroyPbufferEXT\0\
    wglGetPbufferDCEXT\0\
    wglQueryPbufferEXT\0\
    wglReleasePbufferDCEXT\0\
    wglChoosePixelFormatEXT\0\
    wglGetPixelFormatAttribfvEXT\0\
    wglGetPixelFormatAttribivEXT\0\
    wglGetSwapIntervalEXT\0\
    wglSwapIntervalEXT\0\
    wglGetDigitalVideoParametersI3D\0\
    wglSetDigitalVideoParametersI3D\0\
    wglGetGammaTableI3D\0\
    wglGetGammaTableParametersI3D\0\
    wglSetGammaTableI3D\0\
    wglSetGammaTableParametersI3D\0\
    wglDisableGenlockI3D\0\
    wglEnableGenlockI3D\0\
    wglGenlockSampleRateI3D\0\
    wglGenlockSourceDelayI3D\0\
    wglGenlockSourceEdgeI3D\0\
    wglGenlockSourceI3D\0\
    wglGetGenlockSampleRateI3D\0\
    wglGetGenlockSourceDelayI3D\0\
    wglGetGenlockSourceEdgeI3D\0\
    wglGetGenlockSourceI3D\0\
    wglIsEnabledGenlockI3D\0\
    wglQueryGenlockMaxSourceDelayI3D\0\
    wglAssociateImageBufferEventsI3D\0\
    wglCreateImageBufferI3D\0\
    wglDestroyImageBufferI3D\0\
    wglReleaseImageBufferEventsI3D\0\
    wglDisableFrameLockI3D\0\
    wglEnableFrameLockI3D\0\
    wglIsEnabledFrameLockI3D\0\
    wglQueryFrameLockMasterI3D\0\
    wglBeginFrameTrackingI3D\0\
    wglEndFrameTrackingI3D\0\
    wglGetFrameUsageI3D\0\
    wglQueryFrameTrackingI3D\0\
    wglCopyImageSubDataNV\0\
    wglDelayBeforeSwapNV\0\
    wglDXCloseDeviceNV\0\
    wglDXLockObjectsNV\0\
    wglDXObjectAccessNV\0\
    wglDXOpenDeviceNV\0\
    wglDXRegisterObjectNV\0\
    wglDXSetResourceShareHandleNV\0\
    wglDXUnlockObjectsNV\0\
    wglDXUnregisterObjectNV\0\
    wglCreateAffinityDCNV\0\
    wglDeleteDCNV\0\
    wglEnumGpuDevicesNV\0\
    wglEnumGpusFromAffinityDCNV\0\
    wglEnumGpusNV\0\
    wglBindVideoDeviceNV\0\
    wglEnumerateVideoDevicesNV\0\
    wglQueryCurrentContextNV\0\
    wglBindSwapBarrierNV\0\
    wglJoinSwapGroupNV\0\
    wglQueryFrameCountNV\0\
    wglQueryMaxSwapGroupsNV\0\
    wglQuerySwapGroupNV\0\
    wglResetFrameCountNV\0\
    wglBindVideoCaptureDeviceNV\0\
    wglEnumerateVideoCaptureDevicesNV\0\
    wglLockVideoCaptureDeviceNV\0\
    wglQueryVideoCaptureDeviceNV\0\
    wglReleaseVideoCaptureDeviceNV\0\
    wglBindVideoImageNV\0\
    wglGetVideoDeviceNV\0\
    wglGetVideoInfoNV\0\
    wglReleaseVideoDeviceNV\0\
    wglReleaseVideoImageNV\0\
    wglSendPbufferToVideoNV\0\
    wglAllocateMemoryNV\0\
    wglFreeMemoryNV\0\
    wglGetMscRateOML\0\
    wglGetSyncValuesOML\0\
    wglSwapBuffersMscOML\0\
    wglSwapLayerBuffersMscOML\0\
    wglWaitForMscOML\0\
    wglWaitForSbcOML\0\
";

// Byte offset of each command name in FN_NAME_DATA, indexed in
// lockstep with the pfn table (slot [i] == command i).
#[rustfmt::skip]
static FN_NAME_OFFSETS: [u16; COMMAND_COUNT] = [
          0, // [0] ChoosePixelFormat
         18, // [1] DescribePixelFormat
         38, // [2] GetEnhMetaFilePixelFormat
         64, // [3] GetPixelFormat
         79, // [4] SetPixelFormat
         94, // [5] SwapBuffers
        106, // [6] wglCopyContext
        121, // [7] wglCreateContext
        138, // [8] wglCreateLayerContext
        160, // [9] wglDeleteContext
        177, // [10] wglDescribeLayerPlane
        199, // [11] wglGetCurrentContext
        220, // [12] wglGetCurrentDC
        236, // [13] wglGetLayerPaletteEntries
        262, // [14] wglGetProcAddress
        280, // [15] wglMakeCurrent
        295, // [16] wglRealizeLayerPalette
        318, // [17] wglSetLayerPaletteEntries
        344, // [18] wglShareLists
        358, // [19] wglSwapLayerBuffers
        378, // [20] wglUseFontBitmaps
        396, // [21] wglUseFontBitmapsA
        415, // [22] wglUseFontBitmapsW
        434, // [23] wglUseFontOutlines
        453, // [24] wglUseFontOutlinesA
        473, // [25] wglUseFontOutlinesW
        493, // [26] wglSetStereoEmitterState3DL
        521, // [27] wglBlitContextFramebufferAMD
        550, // [28] wglCreateAssociatedContextAMD
        580, // [29] wglCreateAssociatedContextAttribsAMD
        617, // [30] wglDeleteAssociatedContextAMD
        647, // [31] wglGetContextGPUIDAMD
        669, // [32] wglGetCurrentAssociatedContextAMD
        703, // [33] wglGetGPUIDsAMD
        719, // [34] wglGetGPUInfoAMD
        736, // [35] wglMakeAssociatedContextCurrentAMD
        771, // [36] wglCreateBufferRegionARB
        796, // [37] wglDeleteBufferRegionARB
        821, // [38] wglRestoreBufferRegionARB
        847, // [39] wglSaveBufferRegionARB
        870, // [40] wglCreateContextAttribsARB
        897, // [41] wglGetExtensionsStringARB
        923, // [42] wglGetCurrentReadDCARB
        946, // [43] wglMakeContextCurrentARB
        971, // [44] wglCreatePbufferARB
        991, // [45] wglDestroyPbufferARB
       1012, // [46] wglGetPbufferDCARB
       1031, // [47] wglQueryPbufferARB
       1050, // [48] wglReleasePbufferDCARB
       1073, // [49] wglChoosePixelFormatARB
       1097, // [50] wglGetPixelFormatAttribfvARB
       1126, // [51] wglGetPixelFormatAttribivARB
       1155, // [52] wglBindTexImageARB
       1174, // [53] wglReleaseTexImageARB
       1196, // [54] wglSetPbufferAttribARB
       1219, // [55] wglBindDisplayColorTableEXT
       1247, // [56] wglCreateDisplayColorTableEXT
       1277, // [57] wglDestroyDisplayColorTableEXT
       1308, // [58] wglLoadDisplayColorTableEXT
       1336, // [59] wglGetExtensionsStringEXT
       1362, // [60] wglGetCurrentReadDCEXT
       1385, // [61] wglMakeContextCurrentEXT
       1410, // [62] wglCreatePbufferEXT
       1430, // [63] wglDestroyPbufferEXT
       1451, // [64] wglGetPbufferDCEXT
       1470, // [65] wglQueryPbufferEXT
       1489, // [66] wglReleasePbufferDCEXT
       1512, // [67] wglChoosePixelFormatEXT
       1536, // [68] wglGetPixelFormatAttribfvEXT
       1565, // [69] wglGetPixelFormatAttribivEXT
       1594, // [70] wglGetSwapIntervalEXT
       1616, // [71] wglSwapIntervalEXT
       1635, // [72] wglGetDigitalVideoParametersI3D
       1667, // [73] wglSetDigitalVideoParametersI3D
       1699, // [74] wglGetGammaTableI3D
       1719, // [75] wglGetGammaTableParametersI3D
       1749, // [76] wglSetGammaTableI3D
       1769, // [77] wglSetGammaTableParametersI3D
       1799, // [78] wglDisableGenlockI3D
       1820, // [79] wglEnableGenlockI3D
       1840, // [80] wglGenlockSampleRateI3D
       1864, // [81] wglGenlockSourceDelayI3D
       1889, // [82] wglGenlockSourceEdgeI3D
       1913, // [83] wglGenlockSourceI3D
       1933, // [84] wglGetGenlockSampleRateI3D
       1960, // [85] wglGetGenlockSourceDelayI3D
       1988, // [86] wglGetGenlockSourceEdgeI3D
       2015, // [87] wglGetGenlockSourceI3D
       2038, // [88] wglIsEnabledGenlockI3D
       2061, // [89] wglQueryGenlockMaxSourceDelayI3D
       2094, // [90] wglAssociateImageBufferEventsI3D
       2127, // [91] wglCreateImageBufferI3D
       2151, // [92] wglDestroyImageBufferI3D
       2176, // [93] wglReleaseImageBufferEventsI3D
       2207, // [94] wglDisableFrameLockI3D
       2230, // [95] wglEnableFrameLockI3D
       2252, // [96] wglIsEnabledFrameLockI3D
       2277, // [97] wglQueryFrameLockMasterI3D
       2304, // [98] wglBeginFrameTrackingI3D
       2329, // [99] wglEndFrameTrackingI3D
       2352, // [100] wglGetFrameUsageI3D
       2372, // [101] wglQueryFrameTrackingI3D
       2397, // [102] wglCopyImageSubDataNV
       2419, // [103] wglDelayBeforeSwapNV
       2440, // [104] wglDXCloseDeviceNV
       2459, // [105] wglDXLockObjectsNV
       2478, // [106] wglDXObjectAccessNV
       2498, // [107] wglDXOpenDeviceNV
       2516, // [108] wglDXRegisterObjectNV
       2538, // [109] wglDXSetResourceShareHandleNV
       2568, // [110] wglDXUnlockObjectsNV
       2589, // [111] wglDXUnregisterObjectNV
       2613, // [112] wglCreateAffinityDCNV
       2635, // [113] wglDeleteDCNV
       2649, // [114] wglEnumGpuDevicesNV
       2669, // [115] wglEnumGpusFromAffinityDCNV
       2697, // [116] wglEnumGpusNV
       2711, // [117] wglBindVideoDeviceNV
       2732, // [118] wglEnumerateVideoDevicesNV
       2759, // [119] wglQueryCurrentContextNV
       2784, // [120] wglBindSwapBarrierNV
       2805, // [121] wglJoinSwapGroupNV
       2824, // [122] wglQueryFrameCountNV
       2845, // [123] wglQueryMaxSwapGroupsNV
       2869, // [124] wglQuerySwapGroupNV
       2889, // [125] wglResetFrameCountNV
       2910, // [126] wglBindVideoCaptureDeviceNV
       2938, // [127] wglEnumerateVideoCaptureDevicesNV
       2972, // [128] wglLockVideoCaptureDeviceNV
       3000, // [129] wglQueryVideoCaptureDeviceNV
       3029, // [130] wglReleaseVideoCaptureDeviceNV
       3060, // [131] wglBindVideoImageNV
       3080, // [132] wglGetVideoDeviceNV
       3100, // [133] wglGetVideoInfoNV
       3118, // [134] wglReleaseVideoDeviceNV
       3142, // [135] wglReleaseVideoImageNV
       3165, // [136] wglSendPbufferToVideoNV
       3189, // [137] wglAllocateMemoryNV
       3209, // [138] wglFreeMemoryNV
       3225, // [139] wglGetMscRateOML
       3242, // [140] wglGetSyncValuesOML
       3262, // [141] wglSwapBuffersMscOML
       3283, // [142] wglSwapLayerBuffersMscOML
       3309, // [143] wglWaitForMscOML
       3326, // [144] wglWaitForSbcOML
];

#[rustfmt::skip]
static FEATURE_RANGES: [(u16, u16, u16); 1] = [
    (   0,    0,   26), // WGL_VERSION_1_0
];

#[rustfmt::skip]
static EXT_RANGES_wgl: [(u16, u16, u16); 31] = [
    (   1,   26,    1), // WGL_3DL_stereo_control
    (   2,   27,    9), // WGL_AMD_gpu_association
    (   3,   36,    4), // WGL_ARB_buffer_region
    (   5,   40,    1), // WGL_ARB_create_context
    (   9,   41,    1), // WGL_ARB_extensions_string
    (  11,   42,    2), // WGL_ARB_make_current_read
    (  13,   44,    5), // WGL_ARB_pbuffer
    (  14,   49,    3), // WGL_ARB_pixel_format
    (  16,   52,    3), // WGL_ARB_render_texture
    (  25,   55,    4), // WGL_EXT_display_color_table
    (  26,   59,    1), // WGL_EXT_extensions_string
    (  28,   60,    2), // WGL_EXT_make_current_read
    (  30,   62,    5), // WGL_EXT_pbuffer
    (  31,   67,    3), // WGL_EXT_pixel_format
    (  33,   70,    2), // WGL_EXT_swap_control
    (  35,   72,    2), // WGL_I3D_digital_video_control
    (  36,   74,    4), // WGL_I3D_gamma
    (  37,   78,   12), // WGL_I3D_genlock
    (  38,   90,    4), // WGL_I3D_image_buffer
    (  39,   94,    4), // WGL_I3D_swap_frame_lock
    (  40,   98,    4), // WGL_I3D_swap_frame_usage
    (  43,  102,    1), // WGL_NV_copy_image
    (  44,  103,    1), // WGL_NV_delay_before_swap
    (  41,  104,    8), // WGL_NV_DX_interop
    (  46,  112,    5), // WGL_NV_gpu_affinity
    (  49,  117,    3), // WGL_NV_present_video
    (  52,  120,    6), // WGL_NV_swap_group
    (  54,  126,    5), // WGL_NV_video_capture
    (  55,  131,    6), // WGL_NV_video_output
    (  53,  137,    2), // WGL_NV_vertex_array_range
    (  56,  139,    6), // WGL_OML_sync_control
];

// ── Extensions ──────────────────────────────────────────────
pub const EXT_COUNT: usize = 57;

// XXH3-64 of each extension name, sorted for binary search.
#[rustfmt::skip]
static EXT_HASH_KEYS: [u64; EXT_COUNT] = [
    0x045e190c435b4d35, // WGL_NV_multigpu_context
    0x0bec45d23000040c, // WGL_3DFX_multisample
    0x0bed92c76c78efff, // WGL_ARB_pixel_format_float
    0x1187bf238685f6ba, // WGL_NV_DX_interop2
    0x13f677cdbcea2701, // WGL_I3D_genlock
    0x1f5a579fdea0f148, // WGL_EXT_framebuffer_sRGB
    0x212a6bdfca219506, // WGL_EXT_swap_control
    0x23735e3ef94191df, // WGL_ARB_buffer_region
    0x24a0369001a0d30d, // WGL_ARB_create_context_robustness
    0x275c113e57078fcc, // WGL_NV_render_texture_rectangle
    0x293670a02363360b, // WGL_NV_vertex_array_range
    0x2ac871c1f2dbac4d, // WGL_NV_render_depth_texture
    0x2b2322e59fb5083b, // WGL_ARB_framebuffer_sRGB
    0x2d0e6cfb46a0106f, // WGL_EXT_display_color_table
    0x3404dab980e61ca2, // WGL_EXT_depth_float
    0x3444be63e457de0a, // WGL_AMD_gpu_association
    0x344eee0b6820af34, // WGL_EXT_colorspace
    0x3c2462ed17d12185, // WGL_OML_sync_control
    0x3e1dd8f8bde73422, // WGL_NV_delay_before_swap
    0x40521e21b96727d5, // WGL_ATI_pixel_format_float
    0x4d3ff082cec434da, // WGL_ARB_create_context_no_error
    0x50b4447ba45d250e, // WGL_I3D_gamma
    0x51b3a5602fa896a1, // WGL_EXT_pbuffer
    0x5f2a9df50cf1003a, // WGL_NV_present_video
    0x61cb3bc9968979c9, // WGL_EXT_pixel_format_packed_float
    0x675d378710334a65, // WGL_EXT_extensions_string
    0x6879670f69d523e8, // WGL_ARB_create_context_profile
    0x6a042befb2e60d17, // WGL_ATI_render_texture_rectangle
    0x6c50f503bc89115e, // WGL_ARB_robustness_application_isolation
    0x6d4cef39fc681fb3, // WGL_EXT_make_current_read
    0x7030d8df95640f74, // WGL_I3D_image_buffer
    0x712af2217200a415, // WGL_EXT_multisample
    0x7d7c383b450b63af, // WGL_NV_multisample_coverage
    0x7e5f58022569e61e, // WGL_EXT_create_context_es_profile
    0x83ba23356bc5c9c7, // WGL_NV_gpu_affinity
    0x8cf54648141ca138, // WGL_NV_video_capture
    0x93b339a9e2cbe314, // WGL_ARB_create_context
    0x93c433aa5564a48d, // WGL_I3D_digital_video_control
    0x944ee36787077ccf, // WGL_ARB_robustness_share_group_isolation
    0x97c79eb8ab4c254a, // WGL_EXT_swap_control_tear
    0xa544e2233909cd6c, // WGL_3DL_stereo_control
    0xa54b9b658ac06b9a, // WGL_NV_DX_interop
    0xad82eec327c2f1d8, // WGL_ARB_multisample
    0xafab55f5e37bcb25, // WGL_NV_copy_image
    0xcba9d91bf8152127, // WGL_ARB_context_flush_control
    0xd6afdfe5c6fa3614, // WGL_ARB_extensions_string
    0xd91b7247d42d3ba5, // WGL_NV_swap_group
    0xdad29f54a41cb160, // WGL_NV_float_buffer
    0xdea75ca5e2edbb10, // WGL_EXT_create_context_es2_profile
    0xe483d21a182c9656, // WGL_I3D_swap_frame_usage
    0xe73be941e5f7916b, // WGL_ARB_pixel_format
    0xeab153b5cac91b24, // WGL_NV_video_output
    0xf0753dc5c832f146, // WGL_ARB_render_texture
    0xf09d08e1c43f564e, // WGL_EXT_pixel_format
    0xf6b9458b542c30dc, // WGL_ARB_pbuffer
    0xf89e0e880fa2991d, // WGL_I3D_swap_frame_lock
    0xffb3e932bb0ccdd5, // WGL_ARB_make_current_read
];
// extArray index for the correspondingly-ranked EXT_HASH_KEYS entry.
#[rustfmt::skip]
static EXT_HASH_IDX: [u16; EXT_COUNT] = [
    47, 0, 15, 42, 37, 27, 33, 3, 8, 51, 53, 50, 10, 25, 24, 2, 21, 56, 44, 19,
    6, 36, 30, 49, 32, 26, 7, 20, 17, 28, 38, 29, 48, 23, 46, 54, 5, 35, 18, 34,
    1, 41, 12, 43, 4, 9, 52, 45, 22, 40, 14, 55, 16, 31, 13, 39, 11,
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
    debug_assert!(false, "unloaded WGL function called in a no-error build");
    unsafe { core::hint::unreachable_unchecked() }
}

// ── Context ─────────────────────────────────────────────────
/// Why [`Wgl::load_wgl`] failed.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LoadError {
    /// Neither `wglGetExtensionsStringARB` nor
    /// `wglGetExtensionsStringEXT` produced an extension string —
    /// `loader` is not a WGL proc-address source, or no context is
    /// current on the HDC.
    MissingExtensionsString,
}

impl core::fmt::Display for LoadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            LoadError::MissingExtensionsString => "WGL extensions string missing",
        })
    }
}

impl core::error::Error for LoadError {}

/// Loaded WGL entry points plus detected feature/extension presence.
pub struct Wgl {
    pfns: [*const c_void; COMMAND_COUNT],
    feat: [bool; FEATURE_COUNT],
    ext: [bool; EXT_COUNT],
    version: u32,
}

impl Wgl {
    /// Load WGL against `loader` (a wglGetProcAddress-style callback)
    /// and detect extensions for `hdc`.
    ///
    /// WGL has no version query, so the version is a faux 1.0 and
    /// every PFN is loaded upfront — availability, not version, gates
    /// WGL entry points (as in the C loader).
    ///
    /// # Safety
    /// A WGL context must be current on the calling thread, `hdc`
    /// must be a valid device context, and `loader` must yield
    /// pointers callable as the named WGL functions.
    #[inline]
    pub unsafe fn load_wgl(
        hdc: HDC,
        mut loader: impl FnMut(&CStr) -> *const c_void,
    ) -> Result<Self, LoadError> {
        // Immediately erase to `&mut dyn` — the real loader is compiled
        // once, not once per closure type.
        unsafe { Self::load_wgl_dyn(hdc, &mut loader) }
    }

    unsafe fn load_wgl_dyn(
        hdc: HDC,
        loader: &mut dyn FnMut(&CStr) -> *const c_void,
    ) -> Result<Self, LoadError> {
        let mut wgl = Self {
            pfns: [core::ptr::null(); COMMAND_COUNT],
            feat: [false; FEATURE_COUNT],
            ext: [false; EXT_COUNT],
            version: 0x0100, // faux WGL 1.0
        };
        // Feature presence from the faux version.
        wgl.feat[0] = wgl.version >= 0x0100;
        // Load every PFN upfront, then additionally mark features whose
        // every PFN resolved (set-only, mirroring the C loader).
        unsafe { wgl.load_range(loader, 0, COMMAND_COUNT as u16) };
        for &(fi, start, count) in FEATURE_RANGES.iter() {
            let mut ok = true;
            for i in start..start + count {
                ok &= !wgl.pfns[i as usize].is_null();
            }
            if ok {
                wgl.feat[fi as usize] = true;
            }
        }
        unsafe { wgl.detect_extensions(hdc)? };
        for &(ei, start, count) in EXT_RANGES_wgl.iter() {
            if wgl.ext[ei as usize] {
                unsafe { wgl.load_range(loader, start, count) };
            }
        }
        wgl.resolve_aliases();
        Ok(wgl)
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
    unsafe fn detect_extensions(&mut self, hdc: HDC) -> Result<(), LoadError> {
        let mut ext_str: *const c_char = core::ptr::null();
        if !self.pfns[41].is_null() {
            ext_str = unsafe { self.GetExtensionsStringARB(hdc) };
        }
        if ext_str.is_null() && !self.pfns[59].is_null() {
            ext_str = unsafe { self.GetExtensionsStringEXT() };
        }
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

    /// The faux WGL version (always `1 << 8 | 0` — WGL has no version
    /// query; capability lives in the extension flags).
    #[inline]
    pub fn version(&self) -> u32 {
        self.version
    }

    // Dispatch wrappers.  The pointer local is named `__pfn` because
    // parameter names could otherwise collide with it.

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn ChoosePixelFormat(&self, hDc: HDC, pPfd: *const PIXELFORMATDESCRIPTOR) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(HDC, *const PIXELFORMATDESCRIPTOR) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(0)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(0) },
        };
        unsafe { __pfn(hDc, pPfd) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn DescribePixelFormat(&self, hdc: HDC, ipfd: i32, cjpfd: UINT, ppfd: *mut PIXELFORMATDESCRIPTOR) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(HDC, i32, UINT, *mut PIXELFORMATDESCRIPTOR) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(1)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(1) },
        };
        unsafe { __pfn(hdc, ipfd, cjpfd, ppfd) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetEnhMetaFilePixelFormat(&self, hemf: HENHMETAFILE, cbBuffer: UINT, ppfd: *mut PIXELFORMATDESCRIPTOR) -> UINT {
        let __pfn: Option<unsafe extern "system" fn(HENHMETAFILE, UINT, *mut PIXELFORMATDESCRIPTOR) -> UINT> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(2)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(2) },
        };
        unsafe { __pfn(hemf, cbBuffer, ppfd) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetPixelFormat(&self, hdc: HDC) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(HDC) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(3)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(3) },
        };
        unsafe { __pfn(hdc) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn SetPixelFormat(&self, hdc: HDC, ipfd: i32, ppfd: *const PIXELFORMATDESCRIPTOR) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, i32, *const PIXELFORMATDESCRIPTOR) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(4)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(4) },
        };
        unsafe { __pfn(hdc, ipfd, ppfd) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn SwapBuffers(&self, hdc: HDC) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(5)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(5) },
        };
        unsafe { __pfn(hdc) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn CopyContext(&self, hglrcSrc: HGLRC, hglrcDst: HGLRC, mask: UINT) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HGLRC, HGLRC, UINT) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(6)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(6) },
        };
        unsafe { __pfn(hglrcSrc, hglrcDst, mask) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn CreateContext(&self, hDc: HDC) -> HGLRC {
        let __pfn: Option<unsafe extern "system" fn(HDC) -> HGLRC> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(7)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(7) },
        };
        unsafe { __pfn(hDc) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn CreateLayerContext(&self, hDc: HDC, level: i32) -> HGLRC {
        let __pfn: Option<unsafe extern "system" fn(HDC, i32) -> HGLRC> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(8)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(8) },
        };
        unsafe { __pfn(hDc, level) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn DeleteContext(&self, oldContext: HGLRC) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HGLRC) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(9)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(9) },
        };
        unsafe { __pfn(oldContext) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn DescribeLayerPlane(&self, hDc: HDC, pixelFormat: i32, layerPlane: i32, nBytes: UINT, plpd: *mut LAYERPLANEDESCRIPTOR) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, i32, i32, UINT, *mut LAYERPLANEDESCRIPTOR) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(10)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(10) },
        };
        unsafe { __pfn(hDc, pixelFormat, layerPlane, nBytes, plpd) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetCurrentContext(&self) -> HGLRC {
        let __pfn: Option<unsafe extern "system" fn() -> HGLRC> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(11)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(11) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetCurrentDC(&self) -> HDC {
        let __pfn: Option<unsafe extern "system" fn() -> HDC> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(12)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(12) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetLayerPaletteEntries(&self, hdc: HDC, iLayerPlane: i32, iStart: i32, cEntries: i32, pcr: *mut COLORREF) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(HDC, i32, i32, i32, *mut COLORREF) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(13)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(13) },
        };
        unsafe { __pfn(hdc, iLayerPlane, iStart, cEntries, pcr) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetProcAddress(&self, lpszProc: LPCSTR) -> PROC {
        let __pfn: Option<unsafe extern "system" fn(LPCSTR) -> PROC> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(14)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(14) },
        };
        unsafe { __pfn(lpszProc) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn MakeCurrent(&self, hDc: HDC, newContext: HGLRC) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, HGLRC) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(15)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(15) },
        };
        unsafe { __pfn(hDc, newContext) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn RealizeLayerPalette(&self, hdc: HDC, iLayerPlane: i32, bRealize: BOOL) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, i32, BOOL) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(16)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(16) },
        };
        unsafe { __pfn(hdc, iLayerPlane, bRealize) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn SetLayerPaletteEntries(&self, hdc: HDC, iLayerPlane: i32, iStart: i32, cEntries: i32, pcr: *const COLORREF) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(HDC, i32, i32, i32, *const COLORREF) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(17)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(17) },
        };
        unsafe { __pfn(hdc, iLayerPlane, iStart, cEntries, pcr) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn ShareLists(&self, hrcSrvShare: HGLRC, hrcSrvSource: HGLRC) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HGLRC, HGLRC) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(18)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(18) },
        };
        unsafe { __pfn(hrcSrvShare, hrcSrvSource) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn SwapLayerBuffers(&self, hdc: HDC, fuFlags: UINT) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, UINT) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(19)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(19) },
        };
        unsafe { __pfn(hdc, fuFlags) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn UseFontBitmaps(&self, hDC: HDC, first: DWORD, count: DWORD, listBase: DWORD) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, DWORD, DWORD, DWORD) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(20)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(20) },
        };
        unsafe { __pfn(hDC, first, count, listBase) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn UseFontBitmapsA(&self, hDC: HDC, first: DWORD, count: DWORD, listBase: DWORD) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, DWORD, DWORD, DWORD) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(21)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(21) },
        };
        unsafe { __pfn(hDC, first, count, listBase) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn UseFontBitmapsW(&self, hDC: HDC, first: DWORD, count: DWORD, listBase: DWORD) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, DWORD, DWORD, DWORD) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(22)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(22) },
        };
        unsafe { __pfn(hDC, first, count, listBase) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn UseFontOutlines(&self, hDC: HDC, first: DWORD, count: DWORD, listBase: DWORD, deviation: FLOAT, extrusion: FLOAT, format: i32, lpgmf: LPGLYPHMETRICSFLOAT) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, DWORD, DWORD, DWORD, FLOAT, FLOAT, i32, LPGLYPHMETRICSFLOAT) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(23)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(23) },
        };
        unsafe { __pfn(hDC, first, count, listBase, deviation, extrusion, format, lpgmf) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn UseFontOutlinesA(&self, hDC: HDC, first: DWORD, count: DWORD, listBase: DWORD, deviation: FLOAT, extrusion: FLOAT, format: i32, lpgmf: LPGLYPHMETRICSFLOAT) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, DWORD, DWORD, DWORD, FLOAT, FLOAT, i32, LPGLYPHMETRICSFLOAT) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(24)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(24) },
        };
        unsafe { __pfn(hDC, first, count, listBase, deviation, extrusion, format, lpgmf) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn UseFontOutlinesW(&self, hDC: HDC, first: DWORD, count: DWORD, listBase: DWORD, deviation: FLOAT, extrusion: FLOAT, format: i32, lpgmf: LPGLYPHMETRICSFLOAT) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, DWORD, DWORD, DWORD, FLOAT, FLOAT, i32, LPGLYPHMETRICSFLOAT) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(25)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(25) },
        };
        unsafe { __pfn(hDC, first, count, listBase, deviation, extrusion, format, lpgmf) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn SetStereoEmitterState3DL(&self, hDC: HDC, uState: UINT) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, UINT) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(26)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(26) },
        };
        unsafe { __pfn(hDC, uState) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn BlitContextFramebufferAMD(&self, dstCtx: HGLRC, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(HGLRC, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLbitfield, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(27)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(27) },
        };
        unsafe { __pfn(dstCtx, srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn CreateAssociatedContextAMD(&self, id: UINT) -> HGLRC {
        let __pfn: Option<unsafe extern "system" fn(UINT) -> HGLRC> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(28)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(28) },
        };
        unsafe { __pfn(id) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn CreateAssociatedContextAttribsAMD(&self, id: UINT, hShareContext: HGLRC, attribList: *const i32) -> HGLRC {
        let __pfn: Option<unsafe extern "system" fn(UINT, HGLRC, *const i32) -> HGLRC> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(29)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(29) },
        };
        unsafe { __pfn(id, hShareContext, attribList) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn DeleteAssociatedContextAMD(&self, hglrc: HGLRC) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HGLRC) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(30)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(30) },
        };
        unsafe { __pfn(hglrc) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetContextGPUIDAMD(&self, hglrc: HGLRC) -> UINT {
        let __pfn: Option<unsafe extern "system" fn(HGLRC) -> UINT> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(31)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(31) },
        };
        unsafe { __pfn(hglrc) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetCurrentAssociatedContextAMD(&self) -> HGLRC {
        let __pfn: Option<unsafe extern "system" fn() -> HGLRC> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(32)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(32) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetGPUIDsAMD(&self, maxCount: UINT, ids: *mut UINT) -> UINT {
        let __pfn: Option<unsafe extern "system" fn(UINT, *mut UINT) -> UINT> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(33)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(33) },
        };
        unsafe { __pfn(maxCount, ids) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetGPUInfoAMD(&self, id: UINT, property: INT, dataType: GLenum, size: UINT, data: *mut c_void) -> INT {
        let __pfn: Option<unsafe extern "system" fn(UINT, INT, GLenum, UINT, *mut c_void) -> INT> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(34)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(34) },
        };
        unsafe { __pfn(id, property, dataType, size, data) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn MakeAssociatedContextCurrentAMD(&self, hglrc: HGLRC) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HGLRC) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(35)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(35) },
        };
        unsafe { __pfn(hglrc) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn CreateBufferRegionARB(&self, hDC: HDC, iLayerPlane: i32, uType: UINT) -> HANDLE {
        let __pfn: Option<unsafe extern "system" fn(HDC, i32, UINT) -> HANDLE> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(36)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(36) },
        };
        unsafe { __pfn(hDC, iLayerPlane, uType) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn DeleteBufferRegionARB(&self, hRegion: HANDLE) {
        let __pfn: Option<unsafe extern "system" fn(HANDLE)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(37)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(37) },
        };
        unsafe { __pfn(hRegion) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn RestoreBufferRegionARB(&self, hRegion: HANDLE, x: i32, y: i32, width: i32, height: i32, xSrc: i32, ySrc: i32) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HANDLE, i32, i32, i32, i32, i32, i32) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(38)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(38) },
        };
        unsafe { __pfn(hRegion, x, y, width, height, xSrc, ySrc) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn SaveBufferRegionARB(&self, hRegion: HANDLE, x: i32, y: i32, width: i32, height: i32) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HANDLE, i32, i32, i32, i32) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(39)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(39) },
        };
        unsafe { __pfn(hRegion, x, y, width, height) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn CreateContextAttribsARB(&self, hDC: HDC, hShareContext: HGLRC, attribList: *const i32) -> HGLRC {
        let __pfn: Option<unsafe extern "system" fn(HDC, HGLRC, *const i32) -> HGLRC> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(40)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(40) },
        };
        unsafe { __pfn(hDC, hShareContext, attribList) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetExtensionsStringARB(&self, hdc: HDC) -> *const c_char {
        let __pfn: Option<unsafe extern "system" fn(HDC) -> *const c_char> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(41)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(41) },
        };
        unsafe { __pfn(hdc) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetCurrentReadDCARB(&self) -> HDC {
        let __pfn: Option<unsafe extern "system" fn() -> HDC> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(42)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(42) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn MakeContextCurrentARB(&self, hDrawDC: HDC, hReadDC: HDC, hglrc: HGLRC) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, HDC, HGLRC) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(43)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(43) },
        };
        unsafe { __pfn(hDrawDC, hReadDC, hglrc) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn CreatePbufferARB(&self, hDC: HDC, iPixelFormat: i32, iWidth: i32, iHeight: i32, piAttribList: *const i32) -> HPBUFFERARB {
        let __pfn: Option<unsafe extern "system" fn(HDC, i32, i32, i32, *const i32) -> HPBUFFERARB> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(44)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(44) },
        };
        unsafe { __pfn(hDC, iPixelFormat, iWidth, iHeight, piAttribList) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn DestroyPbufferARB(&self, hPbuffer: HPBUFFERARB) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HPBUFFERARB) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(45)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(45) },
        };
        unsafe { __pfn(hPbuffer) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetPbufferDCARB(&self, hPbuffer: HPBUFFERARB) -> HDC {
        let __pfn: Option<unsafe extern "system" fn(HPBUFFERARB) -> HDC> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(46)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(46) },
        };
        unsafe { __pfn(hPbuffer) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn QueryPbufferARB(&self, hPbuffer: HPBUFFERARB, iAttribute: i32, piValue: *mut i32) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HPBUFFERARB, i32, *mut i32) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(47)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(47) },
        };
        unsafe { __pfn(hPbuffer, iAttribute, piValue) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn ReleasePbufferDCARB(&self, hPbuffer: HPBUFFERARB, hDC: HDC) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(HPBUFFERARB, HDC) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(48)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(48) },
        };
        unsafe { __pfn(hPbuffer, hDC) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn ChoosePixelFormatARB(&self, hdc: HDC, piAttribIList: *const i32, pfAttribFList: *const FLOAT, nMaxFormats: UINT, piFormats: *mut i32, nNumFormats: *mut UINT) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, *const i32, *const FLOAT, UINT, *mut i32, *mut UINT) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(49)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(49) },
        };
        unsafe { __pfn(hdc, piAttribIList, pfAttribFList, nMaxFormats, piFormats, nNumFormats) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetPixelFormatAttribfvARB(&self, hdc: HDC, iPixelFormat: i32, iLayerPlane: i32, nAttributes: UINT, piAttributes: *const i32, pfValues: *mut FLOAT) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, i32, i32, UINT, *const i32, *mut FLOAT) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(50)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(50) },
        };
        unsafe { __pfn(hdc, iPixelFormat, iLayerPlane, nAttributes, piAttributes, pfValues) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetPixelFormatAttribivARB(&self, hdc: HDC, iPixelFormat: i32, iLayerPlane: i32, nAttributes: UINT, piAttributes: *const i32, piValues: *mut i32) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, i32, i32, UINT, *const i32, *mut i32) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(51)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(51) },
        };
        unsafe { __pfn(hdc, iPixelFormat, iLayerPlane, nAttributes, piAttributes, piValues) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn BindTexImageARB(&self, hPbuffer: HPBUFFERARB, iBuffer: i32) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HPBUFFERARB, i32) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(52)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(52) },
        };
        unsafe { __pfn(hPbuffer, iBuffer) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn ReleaseTexImageARB(&self, hPbuffer: HPBUFFERARB, iBuffer: i32) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HPBUFFERARB, i32) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(53)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(53) },
        };
        unsafe { __pfn(hPbuffer, iBuffer) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn SetPbufferAttribARB(&self, hPbuffer: HPBUFFERARB, piAttribList: *const i32) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HPBUFFERARB, *const i32) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(54)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(54) },
        };
        unsafe { __pfn(hPbuffer, piAttribList) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn BindDisplayColorTableEXT(&self, id: GLushort) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLushort) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(55)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(55) },
        };
        unsafe { __pfn(id) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn CreateDisplayColorTableEXT(&self, id: GLushort) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLushort) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(56)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(56) },
        };
        unsafe { __pfn(id) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn DestroyDisplayColorTableEXT(&self, id: GLushort) {
        let __pfn: Option<unsafe extern "system" fn(GLushort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(57)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(57) },
        };
        unsafe { __pfn(id) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn LoadDisplayColorTableEXT(&self, table: *const GLushort, length: GLuint) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(*const GLushort, GLuint) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(58)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(58) },
        };
        unsafe { __pfn(table, length) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetExtensionsStringEXT(&self) -> *const c_char {
        let __pfn: Option<unsafe extern "system" fn() -> *const c_char> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(59)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(59) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetCurrentReadDCEXT(&self) -> HDC {
        let __pfn: Option<unsafe extern "system" fn() -> HDC> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(60)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(60) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn MakeContextCurrentEXT(&self, hDrawDC: HDC, hReadDC: HDC, hglrc: HGLRC) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, HDC, HGLRC) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(61)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(61) },
        };
        unsafe { __pfn(hDrawDC, hReadDC, hglrc) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn CreatePbufferEXT(&self, hDC: HDC, iPixelFormat: i32, iWidth: i32, iHeight: i32, piAttribList: *const i32) -> HPBUFFEREXT {
        let __pfn: Option<unsafe extern "system" fn(HDC, i32, i32, i32, *const i32) -> HPBUFFEREXT> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(62)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(62) },
        };
        unsafe { __pfn(hDC, iPixelFormat, iWidth, iHeight, piAttribList) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn DestroyPbufferEXT(&self, hPbuffer: HPBUFFEREXT) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HPBUFFEREXT) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(63)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(63) },
        };
        unsafe { __pfn(hPbuffer) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetPbufferDCEXT(&self, hPbuffer: HPBUFFEREXT) -> HDC {
        let __pfn: Option<unsafe extern "system" fn(HPBUFFEREXT) -> HDC> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(64)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(64) },
        };
        unsafe { __pfn(hPbuffer) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn QueryPbufferEXT(&self, hPbuffer: HPBUFFEREXT, iAttribute: i32, piValue: *mut i32) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HPBUFFEREXT, i32, *mut i32) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(65)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(65) },
        };
        unsafe { __pfn(hPbuffer, iAttribute, piValue) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn ReleasePbufferDCEXT(&self, hPbuffer: HPBUFFEREXT, hDC: HDC) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(HPBUFFEREXT, HDC) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(66)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(66) },
        };
        unsafe { __pfn(hPbuffer, hDC) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn ChoosePixelFormatEXT(&self, hdc: HDC, piAttribIList: *const i32, pfAttribFList: *const FLOAT, nMaxFormats: UINT, piFormats: *mut i32, nNumFormats: *mut UINT) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, *const i32, *const FLOAT, UINT, *mut i32, *mut UINT) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(67)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(67) },
        };
        unsafe { __pfn(hdc, piAttribIList, pfAttribFList, nMaxFormats, piFormats, nNumFormats) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetPixelFormatAttribfvEXT(&self, hdc: HDC, iPixelFormat: i32, iLayerPlane: i32, nAttributes: UINT, piAttributes: *mut i32, pfValues: *mut FLOAT) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, i32, i32, UINT, *mut i32, *mut FLOAT) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(68)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(68) },
        };
        unsafe { __pfn(hdc, iPixelFormat, iLayerPlane, nAttributes, piAttributes, pfValues) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetPixelFormatAttribivEXT(&self, hdc: HDC, iPixelFormat: i32, iLayerPlane: i32, nAttributes: UINT, piAttributes: *mut i32, piValues: *mut i32) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, i32, i32, UINT, *mut i32, *mut i32) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(69)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(69) },
        };
        unsafe { __pfn(hdc, iPixelFormat, iLayerPlane, nAttributes, piAttributes, piValues) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetSwapIntervalEXT(&self) -> i32 {
        let __pfn: Option<unsafe extern "system" fn() -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(70)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(70) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn SwapIntervalEXT(&self, interval: i32) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(i32) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(71)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(71) },
        };
        unsafe { __pfn(interval) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetDigitalVideoParametersI3D(&self, hDC: HDC, iAttribute: i32, piValue: *mut i32) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, i32, *mut i32) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(72)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(72) },
        };
        unsafe { __pfn(hDC, iAttribute, piValue) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn SetDigitalVideoParametersI3D(&self, hDC: HDC, iAttribute: i32, piValue: *const i32) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, i32, *const i32) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(73)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(73) },
        };
        unsafe { __pfn(hDC, iAttribute, piValue) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetGammaTableI3D(&self, hDC: HDC, iEntries: i32, puRed: *mut USHORT, puGreen: *mut USHORT, puBlue: *mut USHORT) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, i32, *mut USHORT, *mut USHORT, *mut USHORT) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(74)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(74) },
        };
        unsafe { __pfn(hDC, iEntries, puRed, puGreen, puBlue) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetGammaTableParametersI3D(&self, hDC: HDC, iAttribute: i32, piValue: *mut i32) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, i32, *mut i32) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(75)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(75) },
        };
        unsafe { __pfn(hDC, iAttribute, piValue) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn SetGammaTableI3D(&self, hDC: HDC, iEntries: i32, puRed: *const USHORT, puGreen: *const USHORT, puBlue: *const USHORT) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, i32, *const USHORT, *const USHORT, *const USHORT) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(76)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(76) },
        };
        unsafe { __pfn(hDC, iEntries, puRed, puGreen, puBlue) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn SetGammaTableParametersI3D(&self, hDC: HDC, iAttribute: i32, piValue: *const i32) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, i32, *const i32) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(77)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(77) },
        };
        unsafe { __pfn(hDC, iAttribute, piValue) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn DisableGenlockI3D(&self, hDC: HDC) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(78)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(78) },
        };
        unsafe { __pfn(hDC) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn EnableGenlockI3D(&self, hDC: HDC) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(79)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(79) },
        };
        unsafe { __pfn(hDC) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GenlockSampleRateI3D(&self, hDC: HDC, uRate: UINT) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, UINT) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(80)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(80) },
        };
        unsafe { __pfn(hDC, uRate) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GenlockSourceDelayI3D(&self, hDC: HDC, uDelay: UINT) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, UINT) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(81)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(81) },
        };
        unsafe { __pfn(hDC, uDelay) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GenlockSourceEdgeI3D(&self, hDC: HDC, uEdge: UINT) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, UINT) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(82)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(82) },
        };
        unsafe { __pfn(hDC, uEdge) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GenlockSourceI3D(&self, hDC: HDC, uSource: UINT) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, UINT) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(83)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(83) },
        };
        unsafe { __pfn(hDC, uSource) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetGenlockSampleRateI3D(&self, hDC: HDC, uRate: *mut UINT) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, *mut UINT) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(84)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(84) },
        };
        unsafe { __pfn(hDC, uRate) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetGenlockSourceDelayI3D(&self, hDC: HDC, uDelay: *mut UINT) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, *mut UINT) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(85)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(85) },
        };
        unsafe { __pfn(hDC, uDelay) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetGenlockSourceEdgeI3D(&self, hDC: HDC, uEdge: *mut UINT) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, *mut UINT) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(86)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(86) },
        };
        unsafe { __pfn(hDC, uEdge) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetGenlockSourceI3D(&self, hDC: HDC, uSource: *mut UINT) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, *mut UINT) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(87)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(87) },
        };
        unsafe { __pfn(hDC, uSource) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn IsEnabledGenlockI3D(&self, hDC: HDC, pFlag: *mut BOOL) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, *mut BOOL) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(88)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(88) },
        };
        unsafe { __pfn(hDC, pFlag) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn QueryGenlockMaxSourceDelayI3D(&self, hDC: HDC, uMaxLineDelay: *mut UINT, uMaxPixelDelay: *mut UINT) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, *mut UINT, *mut UINT) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(89)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(89) },
        };
        unsafe { __pfn(hDC, uMaxLineDelay, uMaxPixelDelay) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn AssociateImageBufferEventsI3D(&self, hDC: HDC, pEvent: *const HANDLE, pAddress: *const LPVOID, pSize: *const DWORD, count: UINT) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, *const HANDLE, *const LPVOID, *const DWORD, UINT) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(90)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(90) },
        };
        unsafe { __pfn(hDC, pEvent, pAddress, pSize, count) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn CreateImageBufferI3D(&self, hDC: HDC, dwSize: DWORD, uFlags: UINT) -> LPVOID {
        let __pfn: Option<unsafe extern "system" fn(HDC, DWORD, UINT) -> LPVOID> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(91)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(91) },
        };
        unsafe { __pfn(hDC, dwSize, uFlags) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn DestroyImageBufferI3D(&self, hDC: HDC, pAddress: LPVOID) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, LPVOID) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(92)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(92) },
        };
        unsafe { __pfn(hDC, pAddress) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn ReleaseImageBufferEventsI3D(&self, hDC: HDC, pAddress: *const LPVOID, count: UINT) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, *const LPVOID, UINT) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(93)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(93) },
        };
        unsafe { __pfn(hDC, pAddress, count) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn DisableFrameLockI3D(&self) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn() -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(94)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(94) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn EnableFrameLockI3D(&self) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn() -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(95)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(95) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn IsEnabledFrameLockI3D(&self, pFlag: *mut BOOL) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(*mut BOOL) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(96)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(96) },
        };
        unsafe { __pfn(pFlag) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn QueryFrameLockMasterI3D(&self, pFlag: *mut BOOL) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(*mut BOOL) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(97)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(97) },
        };
        unsafe { __pfn(pFlag) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn BeginFrameTrackingI3D(&self) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn() -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(98)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(98) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn EndFrameTrackingI3D(&self) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn() -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(99)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(99) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetFrameUsageI3D(&self, pUsage: *mut f32) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(*mut f32) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(100)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(100) },
        };
        unsafe { __pfn(pUsage) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn QueryFrameTrackingI3D(&self, pFrameCount: *mut DWORD, pMissedFrames: *mut DWORD, pLastMissedUsage: *mut f32) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(*mut DWORD, *mut DWORD, *mut f32) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(101)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(101) },
        };
        unsafe { __pfn(pFrameCount, pMissedFrames, pLastMissedUsage) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn CopyImageSubDataNV(&self, hSrcRC: HGLRC, srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, hDstRC: HGLRC, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, width: GLsizei, height: GLsizei, depth: GLsizei) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HGLRC, GLuint, GLenum, GLint, GLint, GLint, GLint, HGLRC, GLuint, GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(102)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(102) },
        };
        unsafe { __pfn(hSrcRC, srcName, srcTarget, srcLevel, srcX, srcY, srcZ, hDstRC, dstName, dstTarget, dstLevel, dstX, dstY, dstZ, width, height, depth) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn DelayBeforeSwapNV(&self, hDC: HDC, seconds: GLfloat) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, GLfloat) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(103)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(103) },
        };
        unsafe { __pfn(hDC, seconds) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn DXCloseDeviceNV(&self, hDevice: HANDLE) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HANDLE) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(104)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(104) },
        };
        unsafe { __pfn(hDevice) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn DXLockObjectsNV(&self, hDevice: HANDLE, count: GLint, hObjects: *mut HANDLE) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HANDLE, GLint, *mut HANDLE) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(105)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(105) },
        };
        unsafe { __pfn(hDevice, count, hObjects) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn DXObjectAccessNV(&self, hObject: HANDLE, access: GLenum) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HANDLE, GLenum) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(106)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(106) },
        };
        unsafe { __pfn(hObject, access) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn DXOpenDeviceNV(&self, dxDevice: *mut c_void) -> HANDLE {
        let __pfn: Option<unsafe extern "system" fn(*mut c_void) -> HANDLE> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(107)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(107) },
        };
        unsafe { __pfn(dxDevice) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn DXRegisterObjectNV(&self, hDevice: HANDLE, dxObject: *mut c_void, name: GLuint, type_: GLenum, access: GLenum) -> HANDLE {
        let __pfn: Option<unsafe extern "system" fn(HANDLE, *mut c_void, GLuint, GLenum, GLenum) -> HANDLE> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(108)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(108) },
        };
        unsafe { __pfn(hDevice, dxObject, name, type_, access) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn DXSetResourceShareHandleNV(&self, dxObject: *mut c_void, shareHandle: HANDLE) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(*mut c_void, HANDLE) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(109)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(109) },
        };
        unsafe { __pfn(dxObject, shareHandle) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn DXUnlockObjectsNV(&self, hDevice: HANDLE, count: GLint, hObjects: *mut HANDLE) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HANDLE, GLint, *mut HANDLE) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(110)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(110) },
        };
        unsafe { __pfn(hDevice, count, hObjects) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn DXUnregisterObjectNV(&self, hDevice: HANDLE, hObject: HANDLE) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HANDLE, HANDLE) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(111)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(111) },
        };
        unsafe { __pfn(hDevice, hObject) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn CreateAffinityDCNV(&self, phGpuList: *const HGPUNV) -> HDC {
        let __pfn: Option<unsafe extern "system" fn(*const HGPUNV) -> HDC> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(112)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(112) },
        };
        unsafe { __pfn(phGpuList) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn DeleteDCNV(&self, hdc: HDC) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(113)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(113) },
        };
        unsafe { __pfn(hdc) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn EnumGpuDevicesNV(&self, hGpu: HGPUNV, iDeviceIndex: UINT, lpGpuDevice: PGPU_DEVICE) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HGPUNV, UINT, PGPU_DEVICE) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(114)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(114) },
        };
        unsafe { __pfn(hGpu, iDeviceIndex, lpGpuDevice) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn EnumGpusFromAffinityDCNV(&self, hAffinityDC: HDC, iGpuIndex: UINT, hGpu: *mut HGPUNV) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, UINT, *mut HGPUNV) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(115)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(115) },
        };
        unsafe { __pfn(hAffinityDC, iGpuIndex, hGpu) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn EnumGpusNV(&self, iGpuIndex: UINT, phGpu: *mut HGPUNV) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(UINT, *mut HGPUNV) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(116)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(116) },
        };
        unsafe { __pfn(iGpuIndex, phGpu) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn BindVideoDeviceNV(&self, hDc: HDC, uVideoSlot: u32, hVideoDevice: HVIDEOOUTPUTDEVICENV, piAttribList: *const i32) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, u32, HVIDEOOUTPUTDEVICENV, *const i32) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(117)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(117) },
        };
        unsafe { __pfn(hDc, uVideoSlot, hVideoDevice, piAttribList) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn EnumerateVideoDevicesNV(&self, hDc: HDC, phDeviceList: *mut HVIDEOOUTPUTDEVICENV) -> i32 {
        let __pfn: Option<unsafe extern "system" fn(HDC, *mut HVIDEOOUTPUTDEVICENV) -> i32> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(118)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(118) },
        };
        unsafe { __pfn(hDc, phDeviceList) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn QueryCurrentContextNV(&self, iAttribute: i32, piValue: *mut i32) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(i32, *mut i32) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(119)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(119) },
        };
        unsafe { __pfn(iAttribute, piValue) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn BindSwapBarrierNV(&self, group: GLuint, barrier: GLuint) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(120)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(120) },
        };
        unsafe { __pfn(group, barrier) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn JoinSwapGroupNV(&self, hDC: HDC, group: GLuint) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, GLuint) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(121)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(121) },
        };
        unsafe { __pfn(hDC, group) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn QueryFrameCountNV(&self, hDC: HDC, count: *mut GLuint) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, *mut GLuint) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(122)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(122) },
        };
        unsafe { __pfn(hDC, count) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn QueryMaxSwapGroupsNV(&self, hDC: HDC, maxGroups: *mut GLuint, maxBarriers: *mut GLuint) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, *mut GLuint, *mut GLuint) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(123)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(123) },
        };
        unsafe { __pfn(hDC, maxGroups, maxBarriers) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn QuerySwapGroupNV(&self, hDC: HDC, group: *mut GLuint, barrier: *mut GLuint) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, *mut GLuint, *mut GLuint) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(124)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(124) },
        };
        unsafe { __pfn(hDC, group, barrier) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn ResetFrameCountNV(&self, hDC: HDC) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(125)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(125) },
        };
        unsafe { __pfn(hDC) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn BindVideoCaptureDeviceNV(&self, uVideoSlot: UINT, hDevice: HVIDEOINPUTDEVICENV) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(UINT, HVIDEOINPUTDEVICENV) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(126)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(126) },
        };
        unsafe { __pfn(uVideoSlot, hDevice) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn EnumerateVideoCaptureDevicesNV(&self, hDc: HDC, phDeviceList: *mut HVIDEOINPUTDEVICENV) -> UINT {
        let __pfn: Option<unsafe extern "system" fn(HDC, *mut HVIDEOINPUTDEVICENV) -> UINT> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(127)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(127) },
        };
        unsafe { __pfn(hDc, phDeviceList) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn LockVideoCaptureDeviceNV(&self, hDc: HDC, hDevice: HVIDEOINPUTDEVICENV) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, HVIDEOINPUTDEVICENV) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(128)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(128) },
        };
        unsafe { __pfn(hDc, hDevice) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn QueryVideoCaptureDeviceNV(&self, hDc: HDC, hDevice: HVIDEOINPUTDEVICENV, iAttribute: i32, piValue: *mut i32) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, HVIDEOINPUTDEVICENV, i32, *mut i32) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(129)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(129) },
        };
        unsafe { __pfn(hDc, hDevice, iAttribute, piValue) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn ReleaseVideoCaptureDeviceNV(&self, hDc: HDC, hDevice: HVIDEOINPUTDEVICENV) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, HVIDEOINPUTDEVICENV) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(130)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(130) },
        };
        unsafe { __pfn(hDc, hDevice) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn BindVideoImageNV(&self, hVideoDevice: HPVIDEODEV, hPbuffer: HPBUFFERARB, iVideoBuffer: i32) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HPVIDEODEV, HPBUFFERARB, i32) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(131)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(131) },
        };
        unsafe { __pfn(hVideoDevice, hPbuffer, iVideoBuffer) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetVideoDeviceNV(&self, hDC: HDC, numDevices: i32, hVideoDevice: *mut HPVIDEODEV) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, i32, *mut HPVIDEODEV) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(132)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(132) },
        };
        unsafe { __pfn(hDC, numDevices, hVideoDevice) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetVideoInfoNV(&self, hpVideoDevice: HPVIDEODEV, pulCounterOutputPbuffer: *mut core::ffi::c_ulong, pulCounterOutputVideo: *mut core::ffi::c_ulong) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HPVIDEODEV, *mut core::ffi::c_ulong, *mut core::ffi::c_ulong) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(133)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(133) },
        };
        unsafe { __pfn(hpVideoDevice, pulCounterOutputPbuffer, pulCounterOutputVideo) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn ReleaseVideoDeviceNV(&self, hVideoDevice: HPVIDEODEV) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HPVIDEODEV) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(134)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(134) },
        };
        unsafe { __pfn(hVideoDevice) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn ReleaseVideoImageNV(&self, hPbuffer: HPBUFFERARB, iVideoBuffer: i32) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HPBUFFERARB, i32) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(135)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(135) },
        };
        unsafe { __pfn(hPbuffer, iVideoBuffer) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn SendPbufferToVideoNV(&self, hPbuffer: HPBUFFERARB, iBufferType: i32, pulCounterPbuffer: *mut core::ffi::c_ulong, bBlock: BOOL) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HPBUFFERARB, i32, *mut core::ffi::c_ulong, BOOL) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(136)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(136) },
        };
        unsafe { __pfn(hPbuffer, iBufferType, pulCounterPbuffer, bBlock) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn AllocateMemoryNV(&self, size: GLsizei, readfreq: GLfloat, writefreq: GLfloat, priority: GLfloat) -> *mut c_void {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, GLfloat, GLfloat, GLfloat) -> *mut c_void> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(137)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(137) },
        };
        unsafe { __pfn(size, readfreq, writefreq, priority) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn FreeMemoryNV(&self, pointer: *mut c_void) {
        let __pfn: Option<unsafe extern "system" fn(*mut c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(138)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(138) },
        };
        unsafe { __pfn(pointer) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetMscRateOML(&self, hdc: HDC, numerator: *mut INT32, denominator: *mut INT32) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, *mut INT32, *mut INT32) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(139)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(139) },
        };
        unsafe { __pfn(hdc, numerator, denominator) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn GetSyncValuesOML(&self, hdc: HDC, ust: *mut INT64, msc: *mut INT64, sbc: *mut INT64) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, *mut INT64, *mut INT64, *mut INT64) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(140)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(140) },
        };
        unsafe { __pfn(hdc, ust, msc, sbc) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn SwapBuffersMscOML(&self, hdc: HDC, target_msc: INT64, divisor: INT64, remainder: INT64) -> INT64 {
        let __pfn: Option<unsafe extern "system" fn(HDC, INT64, INT64, INT64) -> INT64> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(141)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(141) },
        };
        unsafe { __pfn(hdc, target_msc, divisor, remainder) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn SwapLayerBuffersMscOML(&self, hdc: HDC, fuPlanes: INT, target_msc: INT64, divisor: INT64, remainder: INT64) -> INT64 {
        let __pfn: Option<unsafe extern "system" fn(HDC, INT, INT64, INT64, INT64) -> INT64> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(142)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(142) },
        };
        unsafe { __pfn(hdc, fuPlanes, target_msc, divisor, remainder) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn WaitForMscOML(&self, hdc: HDC, target_msc: INT64, divisor: INT64, remainder: INT64, ust: *mut INT64, msc: *mut INT64, sbc: *mut INT64) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, INT64, INT64, INT64, *mut INT64, *mut INT64, *mut INT64) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(143)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(143) },
        };
        unsafe { __pfn(hdc, target_msc, divisor, remainder, ust, msc, sbc) }
    }

    /// # Safety
    /// A WGL context must be loaded and current; see [`Wgl::load_wgl`].
    #[inline]
    pub unsafe fn WaitForSbcOML(&self, hdc: HDC, target_sbc: INT64, ust: *mut INT64, msc: *mut INT64, sbc: *mut INT64) -> BOOL {
        let __pfn: Option<unsafe extern "system" fn(HDC, INT64, *mut INT64, *mut INT64, *mut INT64) -> BOOL> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(144)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(144) },
        };
        unsafe { __pfn(hdc, target_sbc, ust, msc, sbc) }
    }

    /// Whether the driver advertises `WGL_3DFX_multisample`.
    #[inline]
    pub fn _3DFX_multisample(&self) -> bool {
        self.ext[0]
    }

    /// Whether the driver advertises `WGL_3DL_stereo_control`.
    #[inline]
    pub fn _3DL_stereo_control(&self) -> bool {
        self.ext[1]
    }

    /// Whether the driver advertises `WGL_AMD_gpu_association`.
    #[inline]
    pub fn AMD_gpu_association(&self) -> bool {
        self.ext[2]
    }

    /// Whether the driver advertises `WGL_ARB_buffer_region`.
    #[inline]
    pub fn ARB_buffer_region(&self) -> bool {
        self.ext[3]
    }

    /// Whether the driver advertises `WGL_ARB_context_flush_control`.
    #[inline]
    pub fn ARB_context_flush_control(&self) -> bool {
        self.ext[4]
    }

    /// Whether the driver advertises `WGL_ARB_create_context`.
    #[inline]
    pub fn ARB_create_context(&self) -> bool {
        self.ext[5]
    }

    /// Whether the driver advertises `WGL_ARB_create_context_no_error`.
    #[inline]
    pub fn ARB_create_context_no_error(&self) -> bool {
        self.ext[6]
    }

    /// Whether the driver advertises `WGL_ARB_create_context_profile`.
    #[inline]
    pub fn ARB_create_context_profile(&self) -> bool {
        self.ext[7]
    }

    /// Whether the driver advertises `WGL_ARB_create_context_robustness`.
    #[inline]
    pub fn ARB_create_context_robustness(&self) -> bool {
        self.ext[8]
    }

    /// Whether the driver advertises `WGL_ARB_extensions_string`.
    #[inline]
    pub fn ARB_extensions_string(&self) -> bool {
        self.ext[9]
    }

    /// Whether the driver advertises `WGL_ARB_framebuffer_sRGB`.
    #[inline]
    pub fn ARB_framebuffer_sRGB(&self) -> bool {
        self.ext[10]
    }

    /// Whether the driver advertises `WGL_ARB_make_current_read`.
    #[inline]
    pub fn ARB_make_current_read(&self) -> bool {
        self.ext[11]
    }

    /// Whether the driver advertises `WGL_ARB_multisample`.
    #[inline]
    pub fn ARB_multisample(&self) -> bool {
        self.ext[12]
    }

    /// Whether the driver advertises `WGL_ARB_pbuffer`.
    #[inline]
    pub fn ARB_pbuffer(&self) -> bool {
        self.ext[13]
    }

    /// Whether the driver advertises `WGL_ARB_pixel_format`.
    #[inline]
    pub fn ARB_pixel_format(&self) -> bool {
        self.ext[14]
    }

    /// Whether the driver advertises `WGL_ARB_pixel_format_float`.
    #[inline]
    pub fn ARB_pixel_format_float(&self) -> bool {
        self.ext[15]
    }

    /// Whether the driver advertises `WGL_ARB_render_texture`.
    #[inline]
    pub fn ARB_render_texture(&self) -> bool {
        self.ext[16]
    }

    /// Whether the driver advertises `WGL_ARB_robustness_application_isolation`.
    #[inline]
    pub fn ARB_robustness_application_isolation(&self) -> bool {
        self.ext[17]
    }

    /// Whether the driver advertises `WGL_ARB_robustness_share_group_isolation`.
    #[inline]
    pub fn ARB_robustness_share_group_isolation(&self) -> bool {
        self.ext[18]
    }

    /// Whether the driver advertises `WGL_ATI_pixel_format_float`.
    #[inline]
    pub fn ATI_pixel_format_float(&self) -> bool {
        self.ext[19]
    }

    /// Whether the driver advertises `WGL_ATI_render_texture_rectangle`.
    #[inline]
    pub fn ATI_render_texture_rectangle(&self) -> bool {
        self.ext[20]
    }

    /// Whether the driver advertises `WGL_EXT_colorspace`.
    #[inline]
    pub fn EXT_colorspace(&self) -> bool {
        self.ext[21]
    }

    /// Whether the driver advertises `WGL_EXT_create_context_es2_profile`.
    #[inline]
    pub fn EXT_create_context_es2_profile(&self) -> bool {
        self.ext[22]
    }

    /// Whether the driver advertises `WGL_EXT_create_context_es_profile`.
    #[inline]
    pub fn EXT_create_context_es_profile(&self) -> bool {
        self.ext[23]
    }

    /// Whether the driver advertises `WGL_EXT_depth_float`.
    #[inline]
    pub fn EXT_depth_float(&self) -> bool {
        self.ext[24]
    }

    /// Whether the driver advertises `WGL_EXT_display_color_table`.
    #[inline]
    pub fn EXT_display_color_table(&self) -> bool {
        self.ext[25]
    }

    /// Whether the driver advertises `WGL_EXT_extensions_string`.
    #[inline]
    pub fn EXT_extensions_string(&self) -> bool {
        self.ext[26]
    }

    /// Whether the driver advertises `WGL_EXT_framebuffer_sRGB`.
    #[inline]
    pub fn EXT_framebuffer_sRGB(&self) -> bool {
        self.ext[27]
    }

    /// Whether the driver advertises `WGL_EXT_make_current_read`.
    #[inline]
    pub fn EXT_make_current_read(&self) -> bool {
        self.ext[28]
    }

    /// Whether the driver advertises `WGL_EXT_multisample`.
    #[inline]
    pub fn EXT_multisample(&self) -> bool {
        self.ext[29]
    }

    /// Whether the driver advertises `WGL_EXT_pbuffer`.
    #[inline]
    pub fn EXT_pbuffer(&self) -> bool {
        self.ext[30]
    }

    /// Whether the driver advertises `WGL_EXT_pixel_format`.
    #[inline]
    pub fn EXT_pixel_format(&self) -> bool {
        self.ext[31]
    }

    /// Whether the driver advertises `WGL_EXT_pixel_format_packed_float`.
    #[inline]
    pub fn EXT_pixel_format_packed_float(&self) -> bool {
        self.ext[32]
    }

    /// Whether the driver advertises `WGL_EXT_swap_control`.
    #[inline]
    pub fn EXT_swap_control(&self) -> bool {
        self.ext[33]
    }

    /// Whether the driver advertises `WGL_EXT_swap_control_tear`.
    #[inline]
    pub fn EXT_swap_control_tear(&self) -> bool {
        self.ext[34]
    }

    /// Whether the driver advertises `WGL_I3D_digital_video_control`.
    #[inline]
    pub fn I3D_digital_video_control(&self) -> bool {
        self.ext[35]
    }

    /// Whether the driver advertises `WGL_I3D_gamma`.
    #[inline]
    pub fn I3D_gamma(&self) -> bool {
        self.ext[36]
    }

    /// Whether the driver advertises `WGL_I3D_genlock`.
    #[inline]
    pub fn I3D_genlock(&self) -> bool {
        self.ext[37]
    }

    /// Whether the driver advertises `WGL_I3D_image_buffer`.
    #[inline]
    pub fn I3D_image_buffer(&self) -> bool {
        self.ext[38]
    }

    /// Whether the driver advertises `WGL_I3D_swap_frame_lock`.
    #[inline]
    pub fn I3D_swap_frame_lock(&self) -> bool {
        self.ext[39]
    }

    /// Whether the driver advertises `WGL_I3D_swap_frame_usage`.
    #[inline]
    pub fn I3D_swap_frame_usage(&self) -> bool {
        self.ext[40]
    }

    /// Whether the driver advertises `WGL_NV_DX_interop`.
    #[inline]
    pub fn NV_DX_interop(&self) -> bool {
        self.ext[41]
    }

    /// Whether the driver advertises `WGL_NV_DX_interop2`.
    #[inline]
    pub fn NV_DX_interop2(&self) -> bool {
        self.ext[42]
    }

    /// Whether the driver advertises `WGL_NV_copy_image`.
    #[inline]
    pub fn NV_copy_image(&self) -> bool {
        self.ext[43]
    }

    /// Whether the driver advertises `WGL_NV_delay_before_swap`.
    #[inline]
    pub fn NV_delay_before_swap(&self) -> bool {
        self.ext[44]
    }

    /// Whether the driver advertises `WGL_NV_float_buffer`.
    #[inline]
    pub fn NV_float_buffer(&self) -> bool {
        self.ext[45]
    }

    /// Whether the driver advertises `WGL_NV_gpu_affinity`.
    #[inline]
    pub fn NV_gpu_affinity(&self) -> bool {
        self.ext[46]
    }

    /// Whether the driver advertises `WGL_NV_multigpu_context`.
    #[inline]
    pub fn NV_multigpu_context(&self) -> bool {
        self.ext[47]
    }

    /// Whether the driver advertises `WGL_NV_multisample_coverage`.
    #[inline]
    pub fn NV_multisample_coverage(&self) -> bool {
        self.ext[48]
    }

    /// Whether the driver advertises `WGL_NV_present_video`.
    #[inline]
    pub fn NV_present_video(&self) -> bool {
        self.ext[49]
    }

    /// Whether the driver advertises `WGL_NV_render_depth_texture`.
    #[inline]
    pub fn NV_render_depth_texture(&self) -> bool {
        self.ext[50]
    }

    /// Whether the driver advertises `WGL_NV_render_texture_rectangle`.
    #[inline]
    pub fn NV_render_texture_rectangle(&self) -> bool {
        self.ext[51]
    }

    /// Whether the driver advertises `WGL_NV_swap_group`.
    #[inline]
    pub fn NV_swap_group(&self) -> bool {
        self.ext[52]
    }

    /// Whether the driver advertises `WGL_NV_vertex_array_range`.
    #[inline]
    pub fn NV_vertex_array_range(&self) -> bool {
        self.ext[53]
    }

    /// Whether the driver advertises `WGL_NV_video_capture`.
    #[inline]
    pub fn NV_video_capture(&self) -> bool {
        self.ext[54]
    }

    /// Whether the driver advertises `WGL_NV_video_output`.
    #[inline]
    pub fn NV_video_output(&self) -> bool {
        self.ext[55]
    }

    /// Whether the driver advertises `WGL_OML_sync_control`.
    #[inline]
    pub fn OML_sync_control(&self) -> bool {
        self.ext[56]
    }

    /// Whether the driver supports `WGL_VERSION_1_0`.
    #[inline]
    pub fn VERSION_1_0(&self) -> bool {
        self.feat[0]
    }
}
