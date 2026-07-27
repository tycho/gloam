#![no_std]
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, clippy::all)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::ffi::{CStr, c_char, c_void};

// ── GL base types ───────────────────────────────────────────
pub type GLboolean = u8;
pub type GLvoid = c_void;
pub type GLint = i32;
pub type GLuint = u32;
pub type GLsizei = i32;
pub type GLdouble = f64;
pub type GLclampd = f64;
pub type GLeglClientBufferEXT = *mut c_void;
pub type GLeglImageOES = *mut c_void;
pub type GLchar = c_char;
pub type GLcharARB = c_char;
#[cfg(target_vendor = "apple")]
pub type GLhandleARB = *mut c_void;
#[cfg(not(target_vendor = "apple"))]
pub type GLhandleARB = u32;
pub type GLsync = *mut __GLsync;
pub type GLhalfNV = u16;
pub type GLVULKANPROCNV = Option<unsafe extern "system" fn()>;
pub type GLbyte = i8;
pub type GLubyte = u8;
pub type GLshort = i16;
pub type GLushort = u16;
pub type GLclampx = i32;
pub type GLfloat = f32;
pub type GLclampf = f32;
pub type GLhalf = u16;
pub type GLhalfARB = u16;
pub type GLfixed = i32;
pub type GLintptr = isize;
pub type GLintptrARB = isize;
pub type GLsizeiptr = isize;
pub type GLsizeiptrARB = isize;
pub type GLint64 = i64;
pub type GLint64EXT = i64;
pub type GLuint64 = u64;
pub type GLuint64EXT = u64;
pub type GLDEBUGPROC = Option<unsafe extern "system" fn(GLenum, GLenum, GLuint, GLenum, GLsizei, *const GLchar, *const c_void)>;
pub type GLDEBUGPROCARB = Option<unsafe extern "system" fn(GLenum, GLenum, GLuint, GLenum, GLsizei, *const GLchar, *const c_void)>;
pub type GLDEBUGPROCKHR = Option<unsafe extern "system" fn(GLenum, GLenum, GLuint, GLenum, GLsizei, *const GLchar, *const c_void)>;
pub type GLDEBUGPROCAMD = Option<unsafe extern "system" fn(GLuint, GLenum, GLenum, GLsizei, *const GLchar, *mut c_void)>;
pub type GLvdpauSurfaceNV = GLintptr;

// Opaque C struct types (incomplete in the spec).  Zero-sized so
// pointers to them stay distinct types, exactly as in C.
#[repr(C)]
pub struct __GLsync {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct _cl_context {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct _cl_event {
    _opaque: [u8; 0],
}

// ── Enum newtypes ───────────────────────────────────────────
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct GLenum(pub u32);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct GLbitfield(pub u32);

impl core::ops::BitOr for GLbitfield {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        GLbitfield(self.0 | rhs.0)
    }
}

impl core::ops::BitAnd for GLbitfield {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        GLbitfield(self.0 & rhs.0)
    }
}

// ── Constants ───────────────────────────────────────────────
pub const GL_DEPTH_BUFFER_BIT: GLbitfield = GLbitfield(0x00000100);
pub const GL_STENCIL_BUFFER_BIT: GLbitfield = GLbitfield(0x00000400);
pub const GL_COLOR_BUFFER_BIT: GLbitfield = GLbitfield(0x00004000);
pub const GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: GLbitfield = GLbitfield(0x00000001);
pub const GL_CONTEXT_CORE_PROFILE_BIT: GLbitfield = GLbitfield(0x00000001);
pub const GL_CONTEXT_COMPATIBILITY_PROFILE_BIT: GLbitfield = GLbitfield(0x00000002);
pub const GL_MAP_READ_BIT: GLbitfield = GLbitfield(0x0001);
pub const GL_MAP_WRITE_BIT: GLbitfield = GLbitfield(0x0002);
pub const GL_MAP_INVALIDATE_RANGE_BIT: GLbitfield = GLbitfield(0x0004);
pub const GL_MAP_INVALIDATE_BUFFER_BIT: GLbitfield = GLbitfield(0x0008);
pub const GL_MAP_FLUSH_EXPLICIT_BIT: GLbitfield = GLbitfield(0x0010);
pub const GL_MAP_UNSYNCHRONIZED_BIT: GLbitfield = GLbitfield(0x0020);
pub const GL_SYNC_FLUSH_COMMANDS_BIT: GLbitfield = GLbitfield(0x00000001);
pub const GL_FALSE: GLboolean = 0;
pub const GL_NO_ERROR: GLenum = GLenum(0);
pub const GL_ZERO: GLenum = GLenum(0);
pub const GL_NONE: GLenum = GLenum(0);
pub const GL_TRUE: GLboolean = 1;
pub const GL_ONE: GLenum = GLenum(1);
pub const GL_INVALID_INDEX: GLuint = 0xFFFFFFFF;
pub const GL_TIMEOUT_IGNORED: GLuint64 = 0xFFFFFFFFFFFFFFFF;
pub const GL_POINTS: GLenum = GLenum(0x0000);
pub const GL_LINES: GLenum = GLenum(0x0001);
pub const GL_LINE_LOOP: GLenum = GLenum(0x0002);
pub const GL_LINE_STRIP: GLenum = GLenum(0x0003);
pub const GL_TRIANGLES: GLenum = GLenum(0x0004);
pub const GL_TRIANGLE_STRIP: GLenum = GLenum(0x0005);
pub const GL_TRIANGLE_FAN: GLenum = GLenum(0x0006);
pub const GL_LINES_ADJACENCY: GLenum = GLenum(0x000A);
pub const GL_LINE_STRIP_ADJACENCY: GLenum = GLenum(0x000B);
pub const GL_TRIANGLES_ADJACENCY: GLenum = GLenum(0x000C);
pub const GL_TRIANGLE_STRIP_ADJACENCY: GLenum = GLenum(0x000D);
pub const GL_NEVER: GLenum = GLenum(0x0200);
pub const GL_LESS: GLenum = GLenum(0x0201);
pub const GL_EQUAL: GLenum = GLenum(0x0202);
pub const GL_LEQUAL: GLenum = GLenum(0x0203);
pub const GL_GREATER: GLenum = GLenum(0x0204);
pub const GL_NOTEQUAL: GLenum = GLenum(0x0205);
pub const GL_GEQUAL: GLenum = GLenum(0x0206);
pub const GL_ALWAYS: GLenum = GLenum(0x0207);
pub const GL_SRC_COLOR: GLenum = GLenum(0x0300);
pub const GL_ONE_MINUS_SRC_COLOR: GLenum = GLenum(0x0301);
pub const GL_SRC_ALPHA: GLenum = GLenum(0x0302);
pub const GL_ONE_MINUS_SRC_ALPHA: GLenum = GLenum(0x0303);
pub const GL_DST_ALPHA: GLenum = GLenum(0x0304);
pub const GL_ONE_MINUS_DST_ALPHA: GLenum = GLenum(0x0305);
pub const GL_DST_COLOR: GLenum = GLenum(0x0306);
pub const GL_ONE_MINUS_DST_COLOR: GLenum = GLenum(0x0307);
pub const GL_SRC_ALPHA_SATURATE: GLenum = GLenum(0x0308);
pub const GL_FRONT_LEFT: GLenum = GLenum(0x0400);
pub const GL_FRONT_RIGHT: GLenum = GLenum(0x0401);
pub const GL_BACK_LEFT: GLenum = GLenum(0x0402);
pub const GL_BACK_RIGHT: GLenum = GLenum(0x0403);
pub const GL_FRONT: GLenum = GLenum(0x0404);
pub const GL_BACK: GLenum = GLenum(0x0405);
pub const GL_LEFT: GLenum = GLenum(0x0406);
pub const GL_RIGHT: GLenum = GLenum(0x0407);
pub const GL_FRONT_AND_BACK: GLenum = GLenum(0x0408);
pub const GL_INVALID_ENUM: GLenum = GLenum(0x0500);
pub const GL_INVALID_VALUE: GLenum = GLenum(0x0501);
pub const GL_INVALID_OPERATION: GLenum = GLenum(0x0502);
pub const GL_OUT_OF_MEMORY: GLenum = GLenum(0x0505);
pub const GL_INVALID_FRAMEBUFFER_OPERATION: GLenum = GLenum(0x0506);
pub const GL_CW: GLenum = GLenum(0x0900);
pub const GL_CCW: GLenum = GLenum(0x0901);
pub const GL_POINT_SIZE: GLenum = GLenum(0x0B11);
pub const GL_POINT_SIZE_RANGE: GLenum = GLenum(0x0B12);
pub const GL_SMOOTH_POINT_SIZE_RANGE: GLenum = GLenum(0x0B12);
pub const GL_POINT_SIZE_GRANULARITY: GLenum = GLenum(0x0B13);
pub const GL_SMOOTH_POINT_SIZE_GRANULARITY: GLenum = GLenum(0x0B13);
pub const GL_LINE_SMOOTH: GLenum = GLenum(0x0B20);
pub const GL_LINE_WIDTH: GLenum = GLenum(0x0B21);
pub const GL_LINE_WIDTH_RANGE: GLenum = GLenum(0x0B22);
pub const GL_SMOOTH_LINE_WIDTH_RANGE: GLenum = GLenum(0x0B22);
pub const GL_LINE_WIDTH_GRANULARITY: GLenum = GLenum(0x0B23);
pub const GL_SMOOTH_LINE_WIDTH_GRANULARITY: GLenum = GLenum(0x0B23);
pub const GL_POLYGON_MODE: GLenum = GLenum(0x0B40);
pub const GL_POLYGON_SMOOTH: GLenum = GLenum(0x0B41);
pub const GL_CULL_FACE: GLenum = GLenum(0x0B44);
pub const GL_CULL_FACE_MODE: GLenum = GLenum(0x0B45);
pub const GL_FRONT_FACE: GLenum = GLenum(0x0B46);
pub const GL_DEPTH_RANGE: GLenum = GLenum(0x0B70);
pub const GL_DEPTH_TEST: GLenum = GLenum(0x0B71);
pub const GL_DEPTH_WRITEMASK: GLenum = GLenum(0x0B72);
pub const GL_DEPTH_CLEAR_VALUE: GLenum = GLenum(0x0B73);
pub const GL_DEPTH_FUNC: GLenum = GLenum(0x0B74);
pub const GL_STENCIL_TEST: GLenum = GLenum(0x0B90);
pub const GL_STENCIL_CLEAR_VALUE: GLenum = GLenum(0x0B91);
pub const GL_STENCIL_FUNC: GLenum = GLenum(0x0B92);
pub const GL_STENCIL_VALUE_MASK: GLenum = GLenum(0x0B93);
pub const GL_STENCIL_FAIL: GLenum = GLenum(0x0B94);
pub const GL_STENCIL_PASS_DEPTH_FAIL: GLenum = GLenum(0x0B95);
pub const GL_STENCIL_PASS_DEPTH_PASS: GLenum = GLenum(0x0B96);
pub const GL_STENCIL_REF: GLenum = GLenum(0x0B97);
pub const GL_STENCIL_WRITEMASK: GLenum = GLenum(0x0B98);
pub const GL_VIEWPORT: GLenum = GLenum(0x0BA2);
pub const GL_DITHER: GLenum = GLenum(0x0BD0);
pub const GL_BLEND_DST: GLenum = GLenum(0x0BE0);
pub const GL_BLEND_SRC: GLenum = GLenum(0x0BE1);
pub const GL_BLEND: GLenum = GLenum(0x0BE2);
pub const GL_LOGIC_OP_MODE: GLenum = GLenum(0x0BF0);
pub const GL_COLOR_LOGIC_OP: GLenum = GLenum(0x0BF2);
pub const GL_DRAW_BUFFER: GLenum = GLenum(0x0C01);
pub const GL_READ_BUFFER: GLenum = GLenum(0x0C02);
pub const GL_SCISSOR_BOX: GLenum = GLenum(0x0C10);
pub const GL_SCISSOR_TEST: GLenum = GLenum(0x0C11);
pub const GL_COLOR_CLEAR_VALUE: GLenum = GLenum(0x0C22);
pub const GL_COLOR_WRITEMASK: GLenum = GLenum(0x0C23);
pub const GL_DOUBLEBUFFER: GLenum = GLenum(0x0C32);
pub const GL_STEREO: GLenum = GLenum(0x0C33);
pub const GL_LINE_SMOOTH_HINT: GLenum = GLenum(0x0C52);
pub const GL_POLYGON_SMOOTH_HINT: GLenum = GLenum(0x0C53);
pub const GL_UNPACK_SWAP_BYTES: GLenum = GLenum(0x0CF0);
pub const GL_UNPACK_LSB_FIRST: GLenum = GLenum(0x0CF1);
pub const GL_UNPACK_ROW_LENGTH: GLenum = GLenum(0x0CF2);
pub const GL_UNPACK_SKIP_ROWS: GLenum = GLenum(0x0CF3);
pub const GL_UNPACK_SKIP_PIXELS: GLenum = GLenum(0x0CF4);
pub const GL_UNPACK_ALIGNMENT: GLenum = GLenum(0x0CF5);
pub const GL_PACK_SWAP_BYTES: GLenum = GLenum(0x0D00);
pub const GL_PACK_LSB_FIRST: GLenum = GLenum(0x0D01);
pub const GL_PACK_ROW_LENGTH: GLenum = GLenum(0x0D02);
pub const GL_PACK_SKIP_ROWS: GLenum = GLenum(0x0D03);
pub const GL_PACK_SKIP_PIXELS: GLenum = GLenum(0x0D04);
pub const GL_PACK_ALIGNMENT: GLenum = GLenum(0x0D05);
pub const GL_MAX_CLIP_DISTANCES: GLenum = GLenum(0x0D32);
pub const GL_MAX_TEXTURE_SIZE: GLenum = GLenum(0x0D33);
pub const GL_MAX_VIEWPORT_DIMS: GLenum = GLenum(0x0D3A);
pub const GL_SUBPIXEL_BITS: GLenum = GLenum(0x0D50);
pub const GL_TEXTURE_1D: GLenum = GLenum(0x0DE0);
pub const GL_TEXTURE_2D: GLenum = GLenum(0x0DE1);
pub const GL_TEXTURE_WIDTH: GLenum = GLenum(0x1000);
pub const GL_TEXTURE_HEIGHT: GLenum = GLenum(0x1001);
pub const GL_TEXTURE_INTERNAL_FORMAT: GLenum = GLenum(0x1003);
pub const GL_TEXTURE_BORDER_COLOR: GLenum = GLenum(0x1004);
pub const GL_DONT_CARE: GLenum = GLenum(0x1100);
pub const GL_FASTEST: GLenum = GLenum(0x1101);
pub const GL_NICEST: GLenum = GLenum(0x1102);
pub const GL_BYTE: GLenum = GLenum(0x1400);
pub const GL_UNSIGNED_BYTE: GLenum = GLenum(0x1401);
pub const GL_SHORT: GLenum = GLenum(0x1402);
pub const GL_UNSIGNED_SHORT: GLenum = GLenum(0x1403);
pub const GL_INT: GLenum = GLenum(0x1404);
pub const GL_UNSIGNED_INT: GLenum = GLenum(0x1405);
pub const GL_FLOAT: GLenum = GLenum(0x1406);
pub const GL_DOUBLE: GLenum = GLenum(0x140A);
pub const GL_HALF_FLOAT: GLenum = GLenum(0x140B);
pub const GL_CLEAR: GLenum = GLenum(0x1500);
pub const GL_AND: GLenum = GLenum(0x1501);
pub const GL_AND_REVERSE: GLenum = GLenum(0x1502);
pub const GL_COPY: GLenum = GLenum(0x1503);
pub const GL_AND_INVERTED: GLenum = GLenum(0x1504);
pub const GL_NOOP: GLenum = GLenum(0x1505);
pub const GL_XOR: GLenum = GLenum(0x1506);
pub const GL_OR: GLenum = GLenum(0x1507);
pub const GL_NOR: GLenum = GLenum(0x1508);
pub const GL_EQUIV: GLenum = GLenum(0x1509);
pub const GL_INVERT: GLenum = GLenum(0x150A);
pub const GL_OR_REVERSE: GLenum = GLenum(0x150B);
pub const GL_COPY_INVERTED: GLenum = GLenum(0x150C);
pub const GL_OR_INVERTED: GLenum = GLenum(0x150D);
pub const GL_NAND: GLenum = GLenum(0x150E);
pub const GL_SET: GLenum = GLenum(0x150F);
pub const GL_TEXTURE: GLenum = GLenum(0x1702);
pub const GL_COLOR: GLenum = GLenum(0x1800);
pub const GL_DEPTH: GLenum = GLenum(0x1801);
pub const GL_STENCIL: GLenum = GLenum(0x1802);
pub const GL_STENCIL_INDEX: GLenum = GLenum(0x1901);
pub const GL_DEPTH_COMPONENT: GLenum = GLenum(0x1902);
pub const GL_RED: GLenum = GLenum(0x1903);
pub const GL_GREEN: GLenum = GLenum(0x1904);
pub const GL_BLUE: GLenum = GLenum(0x1905);
pub const GL_ALPHA: GLenum = GLenum(0x1906);
pub const GL_RGB: GLenum = GLenum(0x1907);
pub const GL_RGBA: GLenum = GLenum(0x1908);
pub const GL_POINT: GLenum = GLenum(0x1B00);
pub const GL_LINE: GLenum = GLenum(0x1B01);
pub const GL_FILL: GLenum = GLenum(0x1B02);
pub const GL_KEEP: GLenum = GLenum(0x1E00);
pub const GL_REPLACE: GLenum = GLenum(0x1E01);
pub const GL_INCR: GLenum = GLenum(0x1E02);
pub const GL_DECR: GLenum = GLenum(0x1E03);
pub const GL_VENDOR: GLenum = GLenum(0x1F00);
pub const GL_RENDERER: GLenum = GLenum(0x1F01);
pub const GL_VERSION: GLenum = GLenum(0x1F02);
pub const GL_EXTENSIONS: GLenum = GLenum(0x1F03);
pub const GL_NEAREST: GLenum = GLenum(0x2600);
pub const GL_LINEAR: GLenum = GLenum(0x2601);
pub const GL_NEAREST_MIPMAP_NEAREST: GLenum = GLenum(0x2700);
pub const GL_LINEAR_MIPMAP_NEAREST: GLenum = GLenum(0x2701);
pub const GL_NEAREST_MIPMAP_LINEAR: GLenum = GLenum(0x2702);
pub const GL_LINEAR_MIPMAP_LINEAR: GLenum = GLenum(0x2703);
pub const GL_TEXTURE_MAG_FILTER: GLenum = GLenum(0x2800);
pub const GL_TEXTURE_MIN_FILTER: GLenum = GLenum(0x2801);
pub const GL_TEXTURE_WRAP_S: GLenum = GLenum(0x2802);
pub const GL_TEXTURE_WRAP_T: GLenum = GLenum(0x2803);
pub const GL_REPEAT: GLenum = GLenum(0x2901);
pub const GL_POLYGON_OFFSET_UNITS: GLenum = GLenum(0x2A00);
pub const GL_POLYGON_OFFSET_POINT: GLenum = GLenum(0x2A01);
pub const GL_POLYGON_OFFSET_LINE: GLenum = GLenum(0x2A02);
pub const GL_R3_G3_B2: GLenum = GLenum(0x2A10);
pub const GL_CLIP_DISTANCE0: GLenum = GLenum(0x3000);
pub const GL_CLIP_DISTANCE1: GLenum = GLenum(0x3001);
pub const GL_CLIP_DISTANCE2: GLenum = GLenum(0x3002);
pub const GL_CLIP_DISTANCE3: GLenum = GLenum(0x3003);
pub const GL_CLIP_DISTANCE4: GLenum = GLenum(0x3004);
pub const GL_CLIP_DISTANCE5: GLenum = GLenum(0x3005);
pub const GL_CLIP_DISTANCE6: GLenum = GLenum(0x3006);
pub const GL_CLIP_DISTANCE7: GLenum = GLenum(0x3007);
pub const GL_CONSTANT_COLOR: GLenum = GLenum(0x8001);
pub const GL_ONE_MINUS_CONSTANT_COLOR: GLenum = GLenum(0x8002);
pub const GL_CONSTANT_ALPHA: GLenum = GLenum(0x8003);
pub const GL_ONE_MINUS_CONSTANT_ALPHA: GLenum = GLenum(0x8004);
pub const GL_BLEND_COLOR: GLenum = GLenum(0x8005);
pub const GL_FUNC_ADD: GLenum = GLenum(0x8006);
pub const GL_MIN: GLenum = GLenum(0x8007);
pub const GL_MAX: GLenum = GLenum(0x8008);
pub const GL_BLEND_EQUATION: GLenum = GLenum(0x8009);
pub const GL_BLEND_EQUATION_RGB: GLenum = GLenum(0x8009);
pub const GL_FUNC_SUBTRACT: GLenum = GLenum(0x800A);
pub const GL_FUNC_REVERSE_SUBTRACT: GLenum = GLenum(0x800B);
pub const GL_UNSIGNED_BYTE_3_3_2: GLenum = GLenum(0x8032);
pub const GL_UNSIGNED_SHORT_4_4_4_4: GLenum = GLenum(0x8033);
pub const GL_UNSIGNED_SHORT_5_5_5_1: GLenum = GLenum(0x8034);
pub const GL_UNSIGNED_INT_8_8_8_8: GLenum = GLenum(0x8035);
pub const GL_UNSIGNED_INT_10_10_10_2: GLenum = GLenum(0x8036);
pub const GL_POLYGON_OFFSET_FILL: GLenum = GLenum(0x8037);
pub const GL_POLYGON_OFFSET_FACTOR: GLenum = GLenum(0x8038);
pub const GL_RGB4: GLenum = GLenum(0x804F);
pub const GL_RGB5: GLenum = GLenum(0x8050);
pub const GL_RGB8: GLenum = GLenum(0x8051);
pub const GL_RGB10: GLenum = GLenum(0x8052);
pub const GL_RGB12: GLenum = GLenum(0x8053);
pub const GL_RGB16: GLenum = GLenum(0x8054);
pub const GL_RGBA2: GLenum = GLenum(0x8055);
pub const GL_RGBA4: GLenum = GLenum(0x8056);
pub const GL_RGB5_A1: GLenum = GLenum(0x8057);
pub const GL_RGBA8: GLenum = GLenum(0x8058);
pub const GL_RGB10_A2: GLenum = GLenum(0x8059);
pub const GL_RGBA12: GLenum = GLenum(0x805A);
pub const GL_RGBA16: GLenum = GLenum(0x805B);
pub const GL_TEXTURE_RED_SIZE: GLenum = GLenum(0x805C);
pub const GL_TEXTURE_GREEN_SIZE: GLenum = GLenum(0x805D);
pub const GL_TEXTURE_BLUE_SIZE: GLenum = GLenum(0x805E);
pub const GL_TEXTURE_ALPHA_SIZE: GLenum = GLenum(0x805F);
pub const GL_PROXY_TEXTURE_1D: GLenum = GLenum(0x8063);
pub const GL_PROXY_TEXTURE_2D: GLenum = GLenum(0x8064);
pub const GL_TEXTURE_BINDING_1D: GLenum = GLenum(0x8068);
pub const GL_TEXTURE_BINDING_2D: GLenum = GLenum(0x8069);
pub const GL_TEXTURE_BINDING_3D: GLenum = GLenum(0x806A);
pub const GL_PACK_SKIP_IMAGES: GLenum = GLenum(0x806B);
pub const GL_PACK_IMAGE_HEIGHT: GLenum = GLenum(0x806C);
pub const GL_UNPACK_SKIP_IMAGES: GLenum = GLenum(0x806D);
pub const GL_UNPACK_IMAGE_HEIGHT: GLenum = GLenum(0x806E);
pub const GL_TEXTURE_3D: GLenum = GLenum(0x806F);
pub const GL_PROXY_TEXTURE_3D: GLenum = GLenum(0x8070);
pub const GL_TEXTURE_DEPTH: GLenum = GLenum(0x8071);
pub const GL_TEXTURE_WRAP_R: GLenum = GLenum(0x8072);
pub const GL_MAX_3D_TEXTURE_SIZE: GLenum = GLenum(0x8073);
pub const GL_MULTISAMPLE: GLenum = GLenum(0x809D);
pub const GL_SAMPLE_ALPHA_TO_COVERAGE: GLenum = GLenum(0x809E);
pub const GL_SAMPLE_ALPHA_TO_ONE: GLenum = GLenum(0x809F);
pub const GL_SAMPLE_COVERAGE: GLenum = GLenum(0x80A0);
pub const GL_SAMPLE_BUFFERS: GLenum = GLenum(0x80A8);
pub const GL_SAMPLES: GLenum = GLenum(0x80A9);
pub const GL_SAMPLE_COVERAGE_VALUE: GLenum = GLenum(0x80AA);
pub const GL_SAMPLE_COVERAGE_INVERT: GLenum = GLenum(0x80AB);
pub const GL_BLEND_DST_RGB: GLenum = GLenum(0x80C8);
pub const GL_BLEND_SRC_RGB: GLenum = GLenum(0x80C9);
pub const GL_BLEND_DST_ALPHA: GLenum = GLenum(0x80CA);
pub const GL_BLEND_SRC_ALPHA: GLenum = GLenum(0x80CB);
pub const GL_BGR: GLenum = GLenum(0x80E0);
pub const GL_BGRA: GLenum = GLenum(0x80E1);
pub const GL_MAX_ELEMENTS_VERTICES: GLenum = GLenum(0x80E8);
pub const GL_MAX_ELEMENTS_INDICES: GLenum = GLenum(0x80E9);
pub const GL_POINT_FADE_THRESHOLD_SIZE: GLenum = GLenum(0x8128);
pub const GL_CLAMP_TO_BORDER: GLenum = GLenum(0x812D);
pub const GL_CLAMP_TO_EDGE: GLenum = GLenum(0x812F);
pub const GL_TEXTURE_MIN_LOD: GLenum = GLenum(0x813A);
pub const GL_TEXTURE_MAX_LOD: GLenum = GLenum(0x813B);
pub const GL_TEXTURE_BASE_LEVEL: GLenum = GLenum(0x813C);
pub const GL_TEXTURE_MAX_LEVEL: GLenum = GLenum(0x813D);
pub const GL_DEPTH_COMPONENT16: GLenum = GLenum(0x81A5);
pub const GL_DEPTH_COMPONENT24: GLenum = GLenum(0x81A6);
pub const GL_DEPTH_COMPONENT32: GLenum = GLenum(0x81A7);
pub const GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLenum = GLenum(0x8210);
pub const GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLenum = GLenum(0x8211);
pub const GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLenum = GLenum(0x8212);
pub const GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLenum = GLenum(0x8213);
pub const GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLenum = GLenum(0x8214);
pub const GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLenum = GLenum(0x8215);
pub const GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLenum = GLenum(0x8216);
pub const GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLenum = GLenum(0x8217);
pub const GL_FRAMEBUFFER_DEFAULT: GLenum = GLenum(0x8218);
pub const GL_FRAMEBUFFER_UNDEFINED: GLenum = GLenum(0x8219);
pub const GL_DEPTH_STENCIL_ATTACHMENT: GLenum = GLenum(0x821A);
pub const GL_MAJOR_VERSION: GLenum = GLenum(0x821B);
pub const GL_MINOR_VERSION: GLenum = GLenum(0x821C);
pub const GL_NUM_EXTENSIONS: GLenum = GLenum(0x821D);
pub const GL_CONTEXT_FLAGS: GLenum = GLenum(0x821E);
pub const GL_COMPRESSED_RED: GLenum = GLenum(0x8225);
pub const GL_COMPRESSED_RG: GLenum = GLenum(0x8226);
pub const GL_RG: GLenum = GLenum(0x8227);
pub const GL_RG_INTEGER: GLenum = GLenum(0x8228);
pub const GL_R8: GLenum = GLenum(0x8229);
pub const GL_R16: GLenum = GLenum(0x822A);
pub const GL_RG8: GLenum = GLenum(0x822B);
pub const GL_RG16: GLenum = GLenum(0x822C);
pub const GL_R16F: GLenum = GLenum(0x822D);
pub const GL_R32F: GLenum = GLenum(0x822E);
pub const GL_RG16F: GLenum = GLenum(0x822F);
pub const GL_RG32F: GLenum = GLenum(0x8230);
pub const GL_R8I: GLenum = GLenum(0x8231);
pub const GL_R8UI: GLenum = GLenum(0x8232);
pub const GL_R16I: GLenum = GLenum(0x8233);
pub const GL_R16UI: GLenum = GLenum(0x8234);
pub const GL_R32I: GLenum = GLenum(0x8235);
pub const GL_R32UI: GLenum = GLenum(0x8236);
pub const GL_RG8I: GLenum = GLenum(0x8237);
pub const GL_RG8UI: GLenum = GLenum(0x8238);
pub const GL_RG16I: GLenum = GLenum(0x8239);
pub const GL_RG16UI: GLenum = GLenum(0x823A);
pub const GL_RG32I: GLenum = GLenum(0x823B);
pub const GL_RG32UI: GLenum = GLenum(0x823C);
pub const GL_UNSIGNED_BYTE_2_3_3_REV: GLenum = GLenum(0x8362);
pub const GL_UNSIGNED_SHORT_5_6_5: GLenum = GLenum(0x8363);
pub const GL_UNSIGNED_SHORT_5_6_5_REV: GLenum = GLenum(0x8364);
pub const GL_UNSIGNED_SHORT_4_4_4_4_REV: GLenum = GLenum(0x8365);
pub const GL_UNSIGNED_SHORT_1_5_5_5_REV: GLenum = GLenum(0x8366);
pub const GL_UNSIGNED_INT_8_8_8_8_REV: GLenum = GLenum(0x8367);
pub const GL_UNSIGNED_INT_2_10_10_10_REV: GLenum = GLenum(0x8368);
pub const GL_MIRRORED_REPEAT: GLenum = GLenum(0x8370);
pub const GL_ALIASED_LINE_WIDTH_RANGE: GLenum = GLenum(0x846E);
pub const GL_TEXTURE0: GLenum = GLenum(0x84C0);
pub const GL_TEXTURE1: GLenum = GLenum(0x84C1);
pub const GL_TEXTURE2: GLenum = GLenum(0x84C2);
pub const GL_TEXTURE3: GLenum = GLenum(0x84C3);
pub const GL_TEXTURE4: GLenum = GLenum(0x84C4);
pub const GL_TEXTURE5: GLenum = GLenum(0x84C5);
pub const GL_TEXTURE6: GLenum = GLenum(0x84C6);
pub const GL_TEXTURE7: GLenum = GLenum(0x84C7);
pub const GL_TEXTURE8: GLenum = GLenum(0x84C8);
pub const GL_TEXTURE9: GLenum = GLenum(0x84C9);
pub const GL_TEXTURE10: GLenum = GLenum(0x84CA);
pub const GL_TEXTURE11: GLenum = GLenum(0x84CB);
pub const GL_TEXTURE12: GLenum = GLenum(0x84CC);
pub const GL_TEXTURE13: GLenum = GLenum(0x84CD);
pub const GL_TEXTURE14: GLenum = GLenum(0x84CE);
pub const GL_TEXTURE15: GLenum = GLenum(0x84CF);
pub const GL_TEXTURE16: GLenum = GLenum(0x84D0);
pub const GL_TEXTURE17: GLenum = GLenum(0x84D1);
pub const GL_TEXTURE18: GLenum = GLenum(0x84D2);
pub const GL_TEXTURE19: GLenum = GLenum(0x84D3);
pub const GL_TEXTURE20: GLenum = GLenum(0x84D4);
pub const GL_TEXTURE21: GLenum = GLenum(0x84D5);
pub const GL_TEXTURE22: GLenum = GLenum(0x84D6);
pub const GL_TEXTURE23: GLenum = GLenum(0x84D7);
pub const GL_TEXTURE24: GLenum = GLenum(0x84D8);
pub const GL_TEXTURE25: GLenum = GLenum(0x84D9);
pub const GL_TEXTURE26: GLenum = GLenum(0x84DA);
pub const GL_TEXTURE27: GLenum = GLenum(0x84DB);
pub const GL_TEXTURE28: GLenum = GLenum(0x84DC);
pub const GL_TEXTURE29: GLenum = GLenum(0x84DD);
pub const GL_TEXTURE30: GLenum = GLenum(0x84DE);
pub const GL_TEXTURE31: GLenum = GLenum(0x84DF);
pub const GL_ACTIVE_TEXTURE: GLenum = GLenum(0x84E0);
pub const GL_MAX_RENDERBUFFER_SIZE: GLenum = GLenum(0x84E8);
pub const GL_COMPRESSED_RGB: GLenum = GLenum(0x84ED);
pub const GL_COMPRESSED_RGBA: GLenum = GLenum(0x84EE);
pub const GL_TEXTURE_COMPRESSION_HINT: GLenum = GLenum(0x84EF);
pub const GL_TEXTURE_RECTANGLE: GLenum = GLenum(0x84F5);
pub const GL_TEXTURE_BINDING_RECTANGLE: GLenum = GLenum(0x84F6);
pub const GL_PROXY_TEXTURE_RECTANGLE: GLenum = GLenum(0x84F7);
pub const GL_MAX_RECTANGLE_TEXTURE_SIZE: GLenum = GLenum(0x84F8);
pub const GL_DEPTH_STENCIL: GLenum = GLenum(0x84F9);
pub const GL_UNSIGNED_INT_24_8: GLenum = GLenum(0x84FA);
pub const GL_MAX_TEXTURE_LOD_BIAS: GLenum = GLenum(0x84FD);
pub const GL_TEXTURE_LOD_BIAS: GLenum = GLenum(0x8501);
pub const GL_INCR_WRAP: GLenum = GLenum(0x8507);
pub const GL_DECR_WRAP: GLenum = GLenum(0x8508);
pub const GL_TEXTURE_CUBE_MAP: GLenum = GLenum(0x8513);
pub const GL_TEXTURE_BINDING_CUBE_MAP: GLenum = GLenum(0x8514);
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = GLenum(0x8515);
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = GLenum(0x8516);
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = GLenum(0x8517);
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = GLenum(0x8518);
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = GLenum(0x8519);
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = GLenum(0x851A);
pub const GL_PROXY_TEXTURE_CUBE_MAP: GLenum = GLenum(0x851B);
pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = GLenum(0x851C);
pub const GL_SRC1_ALPHA: GLenum = GLenum(0x8589);
pub const GL_VERTEX_ARRAY_BINDING: GLenum = GLenum(0x85B5);
pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = GLenum(0x8622);
pub const GL_VERTEX_ATTRIB_ARRAY_SIZE: GLenum = GLenum(0x8623);
pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = GLenum(0x8624);
pub const GL_VERTEX_ATTRIB_ARRAY_TYPE: GLenum = GLenum(0x8625);
pub const GL_CURRENT_VERTEX_ATTRIB: GLenum = GLenum(0x8626);
pub const GL_VERTEX_PROGRAM_POINT_SIZE: GLenum = GLenum(0x8642);
pub const GL_PROGRAM_POINT_SIZE: GLenum = GLenum(0x8642);
pub const GL_VERTEX_ATTRIB_ARRAY_POINTER: GLenum = GLenum(0x8645);
pub const GL_DEPTH_CLAMP: GLenum = GLenum(0x864F);
pub const GL_TEXTURE_COMPRESSED_IMAGE_SIZE: GLenum = GLenum(0x86A0);
pub const GL_TEXTURE_COMPRESSED: GLenum = GLenum(0x86A1);
pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS: GLenum = GLenum(0x86A2);
pub const GL_COMPRESSED_TEXTURE_FORMATS: GLenum = GLenum(0x86A3);
pub const GL_BUFFER_SIZE: GLenum = GLenum(0x8764);
pub const GL_BUFFER_USAGE: GLenum = GLenum(0x8765);
pub const GL_STENCIL_BACK_FUNC: GLenum = GLenum(0x8800);
pub const GL_STENCIL_BACK_FAIL: GLenum = GLenum(0x8801);
pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = GLenum(0x8802);
pub const GL_STENCIL_BACK_PASS_DEPTH_PASS: GLenum = GLenum(0x8803);
pub const GL_RGBA32F: GLenum = GLenum(0x8814);
pub const GL_RGB32F: GLenum = GLenum(0x8815);
pub const GL_RGBA16F: GLenum = GLenum(0x881A);
pub const GL_RGB16F: GLenum = GLenum(0x881B);
pub const GL_MAX_DRAW_BUFFERS: GLenum = GLenum(0x8824);
pub const GL_DRAW_BUFFER0: GLenum = GLenum(0x8825);
pub const GL_DRAW_BUFFER1: GLenum = GLenum(0x8826);
pub const GL_DRAW_BUFFER2: GLenum = GLenum(0x8827);
pub const GL_DRAW_BUFFER3: GLenum = GLenum(0x8828);
pub const GL_DRAW_BUFFER4: GLenum = GLenum(0x8829);
pub const GL_DRAW_BUFFER5: GLenum = GLenum(0x882A);
pub const GL_DRAW_BUFFER6: GLenum = GLenum(0x882B);
pub const GL_DRAW_BUFFER7: GLenum = GLenum(0x882C);
pub const GL_DRAW_BUFFER8: GLenum = GLenum(0x882D);
pub const GL_DRAW_BUFFER9: GLenum = GLenum(0x882E);
pub const GL_DRAW_BUFFER10: GLenum = GLenum(0x882F);
pub const GL_DRAW_BUFFER11: GLenum = GLenum(0x8830);
pub const GL_DRAW_BUFFER12: GLenum = GLenum(0x8831);
pub const GL_DRAW_BUFFER13: GLenum = GLenum(0x8832);
pub const GL_DRAW_BUFFER14: GLenum = GLenum(0x8833);
pub const GL_DRAW_BUFFER15: GLenum = GLenum(0x8834);
pub const GL_BLEND_EQUATION_ALPHA: GLenum = GLenum(0x883D);
pub const GL_TEXTURE_DEPTH_SIZE: GLenum = GLenum(0x884A);
pub const GL_TEXTURE_COMPARE_MODE: GLenum = GLenum(0x884C);
pub const GL_TEXTURE_COMPARE_FUNC: GLenum = GLenum(0x884D);
pub const GL_COMPARE_REF_TO_TEXTURE: GLenum = GLenum(0x884E);
pub const GL_TEXTURE_CUBE_MAP_SEAMLESS: GLenum = GLenum(0x884F);
pub const GL_QUERY_COUNTER_BITS: GLenum = GLenum(0x8864);
pub const GL_CURRENT_QUERY: GLenum = GLenum(0x8865);
pub const GL_QUERY_RESULT: GLenum = GLenum(0x8866);
pub const GL_QUERY_RESULT_AVAILABLE: GLenum = GLenum(0x8867);
pub const GL_MAX_VERTEX_ATTRIBS: GLenum = GLenum(0x8869);
pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = GLenum(0x886A);
pub const GL_MAX_TEXTURE_IMAGE_UNITS: GLenum = GLenum(0x8872);
pub const GL_ARRAY_BUFFER: GLenum = GLenum(0x8892);
pub const GL_ELEMENT_ARRAY_BUFFER: GLenum = GLenum(0x8893);
pub const GL_ARRAY_BUFFER_BINDING: GLenum = GLenum(0x8894);
pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: GLenum = GLenum(0x8895);
pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = GLenum(0x889F);
pub const GL_READ_ONLY: GLenum = GLenum(0x88B8);
pub const GL_WRITE_ONLY: GLenum = GLenum(0x88B9);
pub const GL_READ_WRITE: GLenum = GLenum(0x88BA);
pub const GL_BUFFER_ACCESS: GLenum = GLenum(0x88BB);
pub const GL_BUFFER_MAPPED: GLenum = GLenum(0x88BC);
pub const GL_BUFFER_MAP_POINTER: GLenum = GLenum(0x88BD);
pub const GL_TIME_ELAPSED: GLenum = GLenum(0x88BF);
pub const GL_STREAM_DRAW: GLenum = GLenum(0x88E0);
pub const GL_STREAM_READ: GLenum = GLenum(0x88E1);
pub const GL_STREAM_COPY: GLenum = GLenum(0x88E2);
pub const GL_STATIC_DRAW: GLenum = GLenum(0x88E4);
pub const GL_STATIC_READ: GLenum = GLenum(0x88E5);
pub const GL_STATIC_COPY: GLenum = GLenum(0x88E6);
pub const GL_DYNAMIC_DRAW: GLenum = GLenum(0x88E8);
pub const GL_DYNAMIC_READ: GLenum = GLenum(0x88E9);
pub const GL_DYNAMIC_COPY: GLenum = GLenum(0x88EA);
pub const GL_PIXEL_PACK_BUFFER: GLenum = GLenum(0x88EB);
pub const GL_PIXEL_UNPACK_BUFFER: GLenum = GLenum(0x88EC);
pub const GL_PIXEL_PACK_BUFFER_BINDING: GLenum = GLenum(0x88ED);
pub const GL_PIXEL_UNPACK_BUFFER_BINDING: GLenum = GLenum(0x88EF);
pub const GL_DEPTH24_STENCIL8: GLenum = GLenum(0x88F0);
pub const GL_TEXTURE_STENCIL_SIZE: GLenum = GLenum(0x88F1);
pub const GL_SRC1_COLOR: GLenum = GLenum(0x88F9);
pub const GL_ONE_MINUS_SRC1_COLOR: GLenum = GLenum(0x88FA);
pub const GL_ONE_MINUS_SRC1_ALPHA: GLenum = GLenum(0x88FB);
pub const GL_MAX_DUAL_SOURCE_DRAW_BUFFERS: GLenum = GLenum(0x88FC);
pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER: GLenum = GLenum(0x88FD);
pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR: GLenum = GLenum(0x88FE);
pub const GL_MAX_ARRAY_TEXTURE_LAYERS: GLenum = GLenum(0x88FF);
pub const GL_MIN_PROGRAM_TEXEL_OFFSET: GLenum = GLenum(0x8904);
pub const GL_MAX_PROGRAM_TEXEL_OFFSET: GLenum = GLenum(0x8905);
pub const GL_SAMPLES_PASSED: GLenum = GLenum(0x8914);
pub const GL_GEOMETRY_VERTICES_OUT: GLenum = GLenum(0x8916);
pub const GL_GEOMETRY_INPUT_TYPE: GLenum = GLenum(0x8917);
pub const GL_GEOMETRY_OUTPUT_TYPE: GLenum = GLenum(0x8918);
pub const GL_SAMPLER_BINDING: GLenum = GLenum(0x8919);
pub const GL_CLAMP_READ_COLOR: GLenum = GLenum(0x891C);
pub const GL_FIXED_ONLY: GLenum = GLenum(0x891D);
pub const GL_UNIFORM_BUFFER: GLenum = GLenum(0x8A11);
pub const GL_UNIFORM_BUFFER_BINDING: GLenum = GLenum(0x8A28);
pub const GL_UNIFORM_BUFFER_START: GLenum = GLenum(0x8A29);
pub const GL_UNIFORM_BUFFER_SIZE: GLenum = GLenum(0x8A2A);
pub const GL_MAX_VERTEX_UNIFORM_BLOCKS: GLenum = GLenum(0x8A2B);
pub const GL_MAX_GEOMETRY_UNIFORM_BLOCKS: GLenum = GLenum(0x8A2C);
pub const GL_MAX_FRAGMENT_UNIFORM_BLOCKS: GLenum = GLenum(0x8A2D);
pub const GL_MAX_COMBINED_UNIFORM_BLOCKS: GLenum = GLenum(0x8A2E);
pub const GL_MAX_UNIFORM_BUFFER_BINDINGS: GLenum = GLenum(0x8A2F);
pub const GL_MAX_UNIFORM_BLOCK_SIZE: GLenum = GLenum(0x8A30);
pub const GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLenum = GLenum(0x8A31);
pub const GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: GLenum = GLenum(0x8A32);
pub const GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLenum = GLenum(0x8A33);
pub const GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLenum = GLenum(0x8A34);
pub const GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: GLenum = GLenum(0x8A35);
pub const GL_ACTIVE_UNIFORM_BLOCKS: GLenum = GLenum(0x8A36);
pub const GL_UNIFORM_TYPE: GLenum = GLenum(0x8A37);
pub const GL_UNIFORM_SIZE: GLenum = GLenum(0x8A38);
pub const GL_UNIFORM_NAME_LENGTH: GLenum = GLenum(0x8A39);
pub const GL_UNIFORM_BLOCK_INDEX: GLenum = GLenum(0x8A3A);
pub const GL_UNIFORM_OFFSET: GLenum = GLenum(0x8A3B);
pub const GL_UNIFORM_ARRAY_STRIDE: GLenum = GLenum(0x8A3C);
pub const GL_UNIFORM_MATRIX_STRIDE: GLenum = GLenum(0x8A3D);
pub const GL_UNIFORM_IS_ROW_MAJOR: GLenum = GLenum(0x8A3E);
pub const GL_UNIFORM_BLOCK_BINDING: GLenum = GLenum(0x8A3F);
pub const GL_UNIFORM_BLOCK_DATA_SIZE: GLenum = GLenum(0x8A40);
pub const GL_UNIFORM_BLOCK_NAME_LENGTH: GLenum = GLenum(0x8A41);
pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLenum = GLenum(0x8A42);
pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLenum = GLenum(0x8A43);
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLenum = GLenum(0x8A44);
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: GLenum = GLenum(0x8A45);
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLenum = GLenum(0x8A46);
pub const GL_FRAGMENT_SHADER: GLenum = GLenum(0x8B30);
pub const GL_VERTEX_SHADER: GLenum = GLenum(0x8B31);
pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS: GLenum = GLenum(0x8B49);
pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS: GLenum = GLenum(0x8B4A);
pub const GL_MAX_VARYING_FLOATS: GLenum = GLenum(0x8B4B);
pub const GL_MAX_VARYING_COMPONENTS: GLenum = GLenum(0x8B4B);
pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = GLenum(0x8B4C);
pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = GLenum(0x8B4D);
pub const GL_SHADER_TYPE: GLenum = GLenum(0x8B4F);
pub const GL_FLOAT_VEC2: GLenum = GLenum(0x8B50);
pub const GL_FLOAT_VEC3: GLenum = GLenum(0x8B51);
pub const GL_FLOAT_VEC4: GLenum = GLenum(0x8B52);
pub const GL_INT_VEC2: GLenum = GLenum(0x8B53);
pub const GL_INT_VEC3: GLenum = GLenum(0x8B54);
pub const GL_INT_VEC4: GLenum = GLenum(0x8B55);
pub const GL_BOOL: GLenum = GLenum(0x8B56);
pub const GL_BOOL_VEC2: GLenum = GLenum(0x8B57);
pub const GL_BOOL_VEC3: GLenum = GLenum(0x8B58);
pub const GL_BOOL_VEC4: GLenum = GLenum(0x8B59);
pub const GL_FLOAT_MAT2: GLenum = GLenum(0x8B5A);
pub const GL_FLOAT_MAT3: GLenum = GLenum(0x8B5B);
pub const GL_FLOAT_MAT4: GLenum = GLenum(0x8B5C);
pub const GL_SAMPLER_1D: GLenum = GLenum(0x8B5D);
pub const GL_SAMPLER_2D: GLenum = GLenum(0x8B5E);
pub const GL_SAMPLER_3D: GLenum = GLenum(0x8B5F);
pub const GL_SAMPLER_CUBE: GLenum = GLenum(0x8B60);
pub const GL_SAMPLER_1D_SHADOW: GLenum = GLenum(0x8B61);
pub const GL_SAMPLER_2D_SHADOW: GLenum = GLenum(0x8B62);
pub const GL_SAMPLER_2D_RECT: GLenum = GLenum(0x8B63);
pub const GL_SAMPLER_2D_RECT_SHADOW: GLenum = GLenum(0x8B64);
pub const GL_FLOAT_MAT2x3: GLenum = GLenum(0x8B65);
pub const GL_FLOAT_MAT2x4: GLenum = GLenum(0x8B66);
pub const GL_FLOAT_MAT3x2: GLenum = GLenum(0x8B67);
pub const GL_FLOAT_MAT3x4: GLenum = GLenum(0x8B68);
pub const GL_FLOAT_MAT4x2: GLenum = GLenum(0x8B69);
pub const GL_FLOAT_MAT4x3: GLenum = GLenum(0x8B6A);
pub const GL_DELETE_STATUS: GLenum = GLenum(0x8B80);
pub const GL_COMPILE_STATUS: GLenum = GLenum(0x8B81);
pub const GL_LINK_STATUS: GLenum = GLenum(0x8B82);
pub const GL_VALIDATE_STATUS: GLenum = GLenum(0x8B83);
pub const GL_INFO_LOG_LENGTH: GLenum = GLenum(0x8B84);
pub const GL_ATTACHED_SHADERS: GLenum = GLenum(0x8B85);
pub const GL_ACTIVE_UNIFORMS: GLenum = GLenum(0x8B86);
pub const GL_ACTIVE_UNIFORM_MAX_LENGTH: GLenum = GLenum(0x8B87);
pub const GL_SHADER_SOURCE_LENGTH: GLenum = GLenum(0x8B88);
pub const GL_ACTIVE_ATTRIBUTES: GLenum = GLenum(0x8B89);
pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH: GLenum = GLenum(0x8B8A);
pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT: GLenum = GLenum(0x8B8B);
pub const GL_SHADING_LANGUAGE_VERSION: GLenum = GLenum(0x8B8C);
pub const GL_CURRENT_PROGRAM: GLenum = GLenum(0x8B8D);
pub const GL_TEXTURE_RED_TYPE: GLenum = GLenum(0x8C10);
pub const GL_TEXTURE_GREEN_TYPE: GLenum = GLenum(0x8C11);
pub const GL_TEXTURE_BLUE_TYPE: GLenum = GLenum(0x8C12);
pub const GL_TEXTURE_ALPHA_TYPE: GLenum = GLenum(0x8C13);
pub const GL_TEXTURE_DEPTH_TYPE: GLenum = GLenum(0x8C16);
pub const GL_UNSIGNED_NORMALIZED: GLenum = GLenum(0x8C17);
pub const GL_TEXTURE_1D_ARRAY: GLenum = GLenum(0x8C18);
pub const GL_PROXY_TEXTURE_1D_ARRAY: GLenum = GLenum(0x8C19);
pub const GL_TEXTURE_2D_ARRAY: GLenum = GLenum(0x8C1A);
pub const GL_PROXY_TEXTURE_2D_ARRAY: GLenum = GLenum(0x8C1B);
pub const GL_TEXTURE_BINDING_1D_ARRAY: GLenum = GLenum(0x8C1C);
pub const GL_TEXTURE_BINDING_2D_ARRAY: GLenum = GLenum(0x8C1D);
pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: GLenum = GLenum(0x8C29);
pub const GL_TEXTURE_BUFFER: GLenum = GLenum(0x8C2A);
pub const GL_MAX_TEXTURE_BUFFER_SIZE: GLenum = GLenum(0x8C2B);
pub const GL_TEXTURE_BINDING_BUFFER: GLenum = GLenum(0x8C2C);
pub const GL_TEXTURE_BUFFER_DATA_STORE_BINDING: GLenum = GLenum(0x8C2D);
pub const GL_ANY_SAMPLES_PASSED: GLenum = GLenum(0x8C2F);
pub const GL_R11F_G11F_B10F: GLenum = GLenum(0x8C3A);
pub const GL_UNSIGNED_INT_10F_11F_11F_REV: GLenum = GLenum(0x8C3B);
pub const GL_RGB9_E5: GLenum = GLenum(0x8C3D);
pub const GL_UNSIGNED_INT_5_9_9_9_REV: GLenum = GLenum(0x8C3E);
pub const GL_TEXTURE_SHARED_SIZE: GLenum = GLenum(0x8C3F);
pub const GL_SRGB: GLenum = GLenum(0x8C40);
pub const GL_SRGB8: GLenum = GLenum(0x8C41);
pub const GL_SRGB_ALPHA: GLenum = GLenum(0x8C42);
pub const GL_SRGB8_ALPHA8: GLenum = GLenum(0x8C43);
pub const GL_COMPRESSED_SRGB: GLenum = GLenum(0x8C48);
pub const GL_COMPRESSED_SRGB_ALPHA: GLenum = GLenum(0x8C49);
pub const GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: GLenum = GLenum(0x8C76);
pub const GL_TRANSFORM_FEEDBACK_BUFFER_MODE: GLenum = GLenum(0x8C7F);
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLenum = GLenum(0x8C80);
pub const GL_TRANSFORM_FEEDBACK_VARYINGS: GLenum = GLenum(0x8C83);
pub const GL_TRANSFORM_FEEDBACK_BUFFER_START: GLenum = GLenum(0x8C84);
pub const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE: GLenum = GLenum(0x8C85);
pub const GL_PRIMITIVES_GENERATED: GLenum = GLenum(0x8C87);
pub const GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLenum = GLenum(0x8C88);
pub const GL_RASTERIZER_DISCARD: GLenum = GLenum(0x8C89);
pub const GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLenum = GLenum(0x8C8A);
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLenum = GLenum(0x8C8B);
pub const GL_INTERLEAVED_ATTRIBS: GLenum = GLenum(0x8C8C);
pub const GL_SEPARATE_ATTRIBS: GLenum = GLenum(0x8C8D);
pub const GL_TRANSFORM_FEEDBACK_BUFFER: GLenum = GLenum(0x8C8E);
pub const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING: GLenum = GLenum(0x8C8F);
pub const GL_POINT_SPRITE_COORD_ORIGIN: GLenum = GLenum(0x8CA0);
pub const GL_LOWER_LEFT: GLenum = GLenum(0x8CA1);
pub const GL_UPPER_LEFT: GLenum = GLenum(0x8CA2);
pub const GL_STENCIL_BACK_REF: GLenum = GLenum(0x8CA3);
pub const GL_STENCIL_BACK_VALUE_MASK: GLenum = GLenum(0x8CA4);
pub const GL_STENCIL_BACK_WRITEMASK: GLenum = GLenum(0x8CA5);
pub const GL_DRAW_FRAMEBUFFER_BINDING: GLenum = GLenum(0x8CA6);
pub const GL_FRAMEBUFFER_BINDING: GLenum = GLenum(0x8CA6);
pub const GL_RENDERBUFFER_BINDING: GLenum = GLenum(0x8CA7);
pub const GL_READ_FRAMEBUFFER: GLenum = GLenum(0x8CA8);
pub const GL_DRAW_FRAMEBUFFER: GLenum = GLenum(0x8CA9);
pub const GL_READ_FRAMEBUFFER_BINDING: GLenum = GLenum(0x8CAA);
pub const GL_RENDERBUFFER_SAMPLES: GLenum = GLenum(0x8CAB);
pub const GL_DEPTH_COMPONENT32F: GLenum = GLenum(0x8CAC);
pub const GL_DEPTH32F_STENCIL8: GLenum = GLenum(0x8CAD);
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = GLenum(0x8CD0);
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = GLenum(0x8CD1);
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = GLenum(0x8CD2);
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = GLenum(0x8CD3);
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLenum = GLenum(0x8CD4);
pub const GL_FRAMEBUFFER_COMPLETE: GLenum = GLenum(0x8CD5);
pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = GLenum(0x8CD6);
pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = GLenum(0x8CD7);
pub const GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: GLenum = GLenum(0x8CDB);
pub const GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER: GLenum = GLenum(0x8CDC);
pub const GL_FRAMEBUFFER_UNSUPPORTED: GLenum = GLenum(0x8CDD);
pub const GL_MAX_COLOR_ATTACHMENTS: GLenum = GLenum(0x8CDF);
pub const GL_COLOR_ATTACHMENT0: GLenum = GLenum(0x8CE0);
pub const GL_COLOR_ATTACHMENT1: GLenum = GLenum(0x8CE1);
pub const GL_COLOR_ATTACHMENT2: GLenum = GLenum(0x8CE2);
pub const GL_COLOR_ATTACHMENT3: GLenum = GLenum(0x8CE3);
pub const GL_COLOR_ATTACHMENT4: GLenum = GLenum(0x8CE4);
pub const GL_COLOR_ATTACHMENT5: GLenum = GLenum(0x8CE5);
pub const GL_COLOR_ATTACHMENT6: GLenum = GLenum(0x8CE6);
pub const GL_COLOR_ATTACHMENT7: GLenum = GLenum(0x8CE7);
pub const GL_COLOR_ATTACHMENT8: GLenum = GLenum(0x8CE8);
pub const GL_COLOR_ATTACHMENT9: GLenum = GLenum(0x8CE9);
pub const GL_COLOR_ATTACHMENT10: GLenum = GLenum(0x8CEA);
pub const GL_COLOR_ATTACHMENT11: GLenum = GLenum(0x8CEB);
pub const GL_COLOR_ATTACHMENT12: GLenum = GLenum(0x8CEC);
pub const GL_COLOR_ATTACHMENT13: GLenum = GLenum(0x8CED);
pub const GL_COLOR_ATTACHMENT14: GLenum = GLenum(0x8CEE);
pub const GL_COLOR_ATTACHMENT15: GLenum = GLenum(0x8CEF);
pub const GL_COLOR_ATTACHMENT16: GLenum = GLenum(0x8CF0);
pub const GL_COLOR_ATTACHMENT17: GLenum = GLenum(0x8CF1);
pub const GL_COLOR_ATTACHMENT18: GLenum = GLenum(0x8CF2);
pub const GL_COLOR_ATTACHMENT19: GLenum = GLenum(0x8CF3);
pub const GL_COLOR_ATTACHMENT20: GLenum = GLenum(0x8CF4);
pub const GL_COLOR_ATTACHMENT21: GLenum = GLenum(0x8CF5);
pub const GL_COLOR_ATTACHMENT22: GLenum = GLenum(0x8CF6);
pub const GL_COLOR_ATTACHMENT23: GLenum = GLenum(0x8CF7);
pub const GL_COLOR_ATTACHMENT24: GLenum = GLenum(0x8CF8);
pub const GL_COLOR_ATTACHMENT25: GLenum = GLenum(0x8CF9);
pub const GL_COLOR_ATTACHMENT26: GLenum = GLenum(0x8CFA);
pub const GL_COLOR_ATTACHMENT27: GLenum = GLenum(0x8CFB);
pub const GL_COLOR_ATTACHMENT28: GLenum = GLenum(0x8CFC);
pub const GL_COLOR_ATTACHMENT29: GLenum = GLenum(0x8CFD);
pub const GL_COLOR_ATTACHMENT30: GLenum = GLenum(0x8CFE);
pub const GL_COLOR_ATTACHMENT31: GLenum = GLenum(0x8CFF);
pub const GL_DEPTH_ATTACHMENT: GLenum = GLenum(0x8D00);
pub const GL_STENCIL_ATTACHMENT: GLenum = GLenum(0x8D20);
pub const GL_FRAMEBUFFER: GLenum = GLenum(0x8D40);
pub const GL_RENDERBUFFER: GLenum = GLenum(0x8D41);
pub const GL_RENDERBUFFER_WIDTH: GLenum = GLenum(0x8D42);
pub const GL_RENDERBUFFER_HEIGHT: GLenum = GLenum(0x8D43);
pub const GL_RENDERBUFFER_INTERNAL_FORMAT: GLenum = GLenum(0x8D44);
pub const GL_STENCIL_INDEX1: GLenum = GLenum(0x8D46);
pub const GL_STENCIL_INDEX4: GLenum = GLenum(0x8D47);
pub const GL_STENCIL_INDEX8: GLenum = GLenum(0x8D48);
pub const GL_STENCIL_INDEX16: GLenum = GLenum(0x8D49);
pub const GL_RENDERBUFFER_RED_SIZE: GLenum = GLenum(0x8D50);
pub const GL_RENDERBUFFER_GREEN_SIZE: GLenum = GLenum(0x8D51);
pub const GL_RENDERBUFFER_BLUE_SIZE: GLenum = GLenum(0x8D52);
pub const GL_RENDERBUFFER_ALPHA_SIZE: GLenum = GLenum(0x8D53);
pub const GL_RENDERBUFFER_DEPTH_SIZE: GLenum = GLenum(0x8D54);
pub const GL_RENDERBUFFER_STENCIL_SIZE: GLenum = GLenum(0x8D55);
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLenum = GLenum(0x8D56);
pub const GL_MAX_SAMPLES: GLenum = GLenum(0x8D57);
pub const GL_RGBA32UI: GLenum = GLenum(0x8D70);
pub const GL_RGB32UI: GLenum = GLenum(0x8D71);
pub const GL_RGBA16UI: GLenum = GLenum(0x8D76);
pub const GL_RGB16UI: GLenum = GLenum(0x8D77);
pub const GL_RGBA8UI: GLenum = GLenum(0x8D7C);
pub const GL_RGB8UI: GLenum = GLenum(0x8D7D);
pub const GL_RGBA32I: GLenum = GLenum(0x8D82);
pub const GL_RGB32I: GLenum = GLenum(0x8D83);
pub const GL_RGBA16I: GLenum = GLenum(0x8D88);
pub const GL_RGB16I: GLenum = GLenum(0x8D89);
pub const GL_RGBA8I: GLenum = GLenum(0x8D8E);
pub const GL_RGB8I: GLenum = GLenum(0x8D8F);
pub const GL_RED_INTEGER: GLenum = GLenum(0x8D94);
pub const GL_GREEN_INTEGER: GLenum = GLenum(0x8D95);
pub const GL_BLUE_INTEGER: GLenum = GLenum(0x8D96);
pub const GL_RGB_INTEGER: GLenum = GLenum(0x8D98);
pub const GL_RGBA_INTEGER: GLenum = GLenum(0x8D99);
pub const GL_BGR_INTEGER: GLenum = GLenum(0x8D9A);
pub const GL_BGRA_INTEGER: GLenum = GLenum(0x8D9B);
pub const GL_INT_2_10_10_10_REV: GLenum = GLenum(0x8D9F);
pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED: GLenum = GLenum(0x8DA7);
pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: GLenum = GLenum(0x8DA8);
pub const GL_FLOAT_32_UNSIGNED_INT_24_8_REV: GLenum = GLenum(0x8DAD);
pub const GL_FRAMEBUFFER_SRGB: GLenum = GLenum(0x8DB9);
pub const GL_COMPRESSED_RED_RGTC1: GLenum = GLenum(0x8DBB);
pub const GL_COMPRESSED_SIGNED_RED_RGTC1: GLenum = GLenum(0x8DBC);
pub const GL_COMPRESSED_RG_RGTC2: GLenum = GLenum(0x8DBD);
pub const GL_COMPRESSED_SIGNED_RG_RGTC2: GLenum = GLenum(0x8DBE);
pub const GL_SAMPLER_1D_ARRAY: GLenum = GLenum(0x8DC0);
pub const GL_SAMPLER_2D_ARRAY: GLenum = GLenum(0x8DC1);
pub const GL_SAMPLER_BUFFER: GLenum = GLenum(0x8DC2);
pub const GL_SAMPLER_1D_ARRAY_SHADOW: GLenum = GLenum(0x8DC3);
pub const GL_SAMPLER_2D_ARRAY_SHADOW: GLenum = GLenum(0x8DC4);
pub const GL_SAMPLER_CUBE_SHADOW: GLenum = GLenum(0x8DC5);
pub const GL_UNSIGNED_INT_VEC2: GLenum = GLenum(0x8DC6);
pub const GL_UNSIGNED_INT_VEC3: GLenum = GLenum(0x8DC7);
pub const GL_UNSIGNED_INT_VEC4: GLenum = GLenum(0x8DC8);
pub const GL_INT_SAMPLER_1D: GLenum = GLenum(0x8DC9);
pub const GL_INT_SAMPLER_2D: GLenum = GLenum(0x8DCA);
pub const GL_INT_SAMPLER_3D: GLenum = GLenum(0x8DCB);
pub const GL_INT_SAMPLER_CUBE: GLenum = GLenum(0x8DCC);
pub const GL_INT_SAMPLER_2D_RECT: GLenum = GLenum(0x8DCD);
pub const GL_INT_SAMPLER_1D_ARRAY: GLenum = GLenum(0x8DCE);
pub const GL_INT_SAMPLER_2D_ARRAY: GLenum = GLenum(0x8DCF);
pub const GL_INT_SAMPLER_BUFFER: GLenum = GLenum(0x8DD0);
pub const GL_UNSIGNED_INT_SAMPLER_1D: GLenum = GLenum(0x8DD1);
pub const GL_UNSIGNED_INT_SAMPLER_2D: GLenum = GLenum(0x8DD2);
pub const GL_UNSIGNED_INT_SAMPLER_3D: GLenum = GLenum(0x8DD3);
pub const GL_UNSIGNED_INT_SAMPLER_CUBE: GLenum = GLenum(0x8DD4);
pub const GL_UNSIGNED_INT_SAMPLER_2D_RECT: GLenum = GLenum(0x8DD5);
pub const GL_UNSIGNED_INT_SAMPLER_1D_ARRAY: GLenum = GLenum(0x8DD6);
pub const GL_UNSIGNED_INT_SAMPLER_2D_ARRAY: GLenum = GLenum(0x8DD7);
pub const GL_UNSIGNED_INT_SAMPLER_BUFFER: GLenum = GLenum(0x8DD8);
pub const GL_GEOMETRY_SHADER: GLenum = GLenum(0x8DD9);
pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS: GLenum = GLenum(0x8DDF);
pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES: GLenum = GLenum(0x8DE0);
pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: GLenum = GLenum(0x8DE1);
pub const GL_QUERY_WAIT: GLenum = GLenum(0x8E13);
pub const GL_QUERY_NO_WAIT: GLenum = GLenum(0x8E14);
pub const GL_QUERY_BY_REGION_WAIT: GLenum = GLenum(0x8E15);
pub const GL_QUERY_BY_REGION_NO_WAIT: GLenum = GLenum(0x8E16);
pub const GL_TIMESTAMP: GLenum = GLenum(0x8E28);
pub const GL_TEXTURE_SWIZZLE_R: GLenum = GLenum(0x8E42);
pub const GL_TEXTURE_SWIZZLE_G: GLenum = GLenum(0x8E43);
pub const GL_TEXTURE_SWIZZLE_B: GLenum = GLenum(0x8E44);
pub const GL_TEXTURE_SWIZZLE_A: GLenum = GLenum(0x8E45);
pub const GL_TEXTURE_SWIZZLE_RGBA: GLenum = GLenum(0x8E46);
pub const GL_QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: GLenum = GLenum(0x8E4C);
pub const GL_FIRST_VERTEX_CONVENTION: GLenum = GLenum(0x8E4D);
pub const GL_LAST_VERTEX_CONVENTION: GLenum = GLenum(0x8E4E);
pub const GL_PROVOKING_VERTEX: GLenum = GLenum(0x8E4F);
pub const GL_SAMPLE_POSITION: GLenum = GLenum(0x8E50);
pub const GL_SAMPLE_MASK: GLenum = GLenum(0x8E51);
pub const GL_SAMPLE_MASK_VALUE: GLenum = GLenum(0x8E52);
pub const GL_MAX_SAMPLE_MASK_WORDS: GLenum = GLenum(0x8E59);
pub const GL_COPY_READ_BUFFER: GLenum = GLenum(0x8F36);
pub const GL_COPY_WRITE_BUFFER: GLenum = GLenum(0x8F37);
pub const GL_R8_SNORM: GLenum = GLenum(0x8F94);
pub const GL_RG8_SNORM: GLenum = GLenum(0x8F95);
pub const GL_RGB8_SNORM: GLenum = GLenum(0x8F96);
pub const GL_RGBA8_SNORM: GLenum = GLenum(0x8F97);
pub const GL_R16_SNORM: GLenum = GLenum(0x8F98);
pub const GL_RG16_SNORM: GLenum = GLenum(0x8F99);
pub const GL_RGB16_SNORM: GLenum = GLenum(0x8F9A);
pub const GL_RGBA16_SNORM: GLenum = GLenum(0x8F9B);
pub const GL_SIGNED_NORMALIZED: GLenum = GLenum(0x8F9C);
pub const GL_PRIMITIVE_RESTART: GLenum = GLenum(0x8F9D);
pub const GL_PRIMITIVE_RESTART_INDEX: GLenum = GLenum(0x8F9E);
pub const GL_RGB10_A2UI: GLenum = GLenum(0x906F);
pub const GL_TEXTURE_2D_MULTISAMPLE: GLenum = GLenum(0x9100);
pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE: GLenum = GLenum(0x9101);
pub const GL_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = GLenum(0x9102);
pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = GLenum(0x9103);
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE: GLenum = GLenum(0x9104);
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: GLenum = GLenum(0x9105);
pub const GL_TEXTURE_SAMPLES: GLenum = GLenum(0x9106);
pub const GL_TEXTURE_FIXED_SAMPLE_LOCATIONS: GLenum = GLenum(0x9107);
pub const GL_SAMPLER_2D_MULTISAMPLE: GLenum = GLenum(0x9108);
pub const GL_INT_SAMPLER_2D_MULTISAMPLE: GLenum = GLenum(0x9109);
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: GLenum = GLenum(0x910A);
pub const GL_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = GLenum(0x910B);
pub const GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = GLenum(0x910C);
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = GLenum(0x910D);
pub const GL_MAX_COLOR_TEXTURE_SAMPLES: GLenum = GLenum(0x910E);
pub const GL_MAX_DEPTH_TEXTURE_SAMPLES: GLenum = GLenum(0x910F);
pub const GL_MAX_INTEGER_SAMPLES: GLenum = GLenum(0x9110);
pub const GL_MAX_SERVER_WAIT_TIMEOUT: GLenum = GLenum(0x9111);
pub const GL_OBJECT_TYPE: GLenum = GLenum(0x9112);
pub const GL_SYNC_CONDITION: GLenum = GLenum(0x9113);
pub const GL_SYNC_STATUS: GLenum = GLenum(0x9114);
pub const GL_SYNC_FLAGS: GLenum = GLenum(0x9115);
pub const GL_SYNC_FENCE: GLenum = GLenum(0x9116);
pub const GL_SYNC_GPU_COMMANDS_COMPLETE: GLenum = GLenum(0x9117);
pub const GL_UNSIGNALED: GLenum = GLenum(0x9118);
pub const GL_SIGNALED: GLenum = GLenum(0x9119);
pub const GL_ALREADY_SIGNALED: GLenum = GLenum(0x911A);
pub const GL_TIMEOUT_EXPIRED: GLenum = GLenum(0x911B);
pub const GL_CONDITION_SATISFIED: GLenum = GLenum(0x911C);
pub const GL_WAIT_FAILED: GLenum = GLenum(0x911D);
pub const GL_BUFFER_ACCESS_FLAGS: GLenum = GLenum(0x911F);
pub const GL_BUFFER_MAP_LENGTH: GLenum = GLenum(0x9120);
pub const GL_BUFFER_MAP_OFFSET: GLenum = GLenum(0x9121);
pub const GL_MAX_VERTEX_OUTPUT_COMPONENTS: GLenum = GLenum(0x9122);
pub const GL_MAX_GEOMETRY_INPUT_COMPONENTS: GLenum = GLenum(0x9123);
pub const GL_MAX_GEOMETRY_OUTPUT_COMPONENTS: GLenum = GLenum(0x9124);
pub const GL_MAX_FRAGMENT_INPUT_COMPONENTS: GLenum = GLenum(0x9125);
pub const GL_CONTEXT_PROFILE_MASK: GLenum = GLenum(0x9126);

// ── Command table ───────────────────────────────────────────
pub const COMMAND_COUNT: usize = 344;
pub const FEATURE_COUNT: usize = 12;

#[rustfmt::skip]
static FN_NAME_DATA: &[u8] = b"\
    glBlendFunc\0\
    glClear\0\
    glClearColor\0\
    glClearDepth\0\
    glClearStencil\0\
    glColorMask\0\
    glCullFace\0\
    glDepthFunc\0\
    glDepthMask\0\
    glDepthRange\0\
    glDisable\0\
    glDrawBuffer\0\
    glEnable\0\
    glFinish\0\
    glFlush\0\
    glFrontFace\0\
    glGetBooleanv\0\
    glGetDoublev\0\
    glGetError\0\
    glGetFloatv\0\
    glGetIntegerv\0\
    glGetString\0\
    glGetTexImage\0\
    glGetTexLevelParameterfv\0\
    glGetTexLevelParameteriv\0\
    glGetTexParameterfv\0\
    glGetTexParameteriv\0\
    glHint\0\
    glIsEnabled\0\
    glLineWidth\0\
    glLogicOp\0\
    glPixelStoref\0\
    glPixelStorei\0\
    glPointSize\0\
    glPolygonMode\0\
    glReadBuffer\0\
    glReadPixels\0\
    glScissor\0\
    glStencilFunc\0\
    glStencilMask\0\
    glStencilOp\0\
    glTexImage1D\0\
    glTexImage2D\0\
    glTexParameterf\0\
    glTexParameterfv\0\
    glTexParameteri\0\
    glTexParameteriv\0\
    glViewport\0\
    glBindTexture\0\
    glCopyTexImage1D\0\
    glCopyTexImage2D\0\
    glCopyTexSubImage1D\0\
    glCopyTexSubImage2D\0\
    glDeleteTextures\0\
    glDrawArrays\0\
    glDrawElements\0\
    glGenTextures\0\
    glIsTexture\0\
    glPolygonOffset\0\
    glTexSubImage1D\0\
    glTexSubImage2D\0\
    glCopyTexSubImage3D\0\
    glDrawRangeElements\0\
    glTexImage3D\0\
    glTexSubImage3D\0\
    glActiveTexture\0\
    glCompressedTexImage1D\0\
    glCompressedTexImage2D\0\
    glCompressedTexImage3D\0\
    glCompressedTexSubImage1D\0\
    glCompressedTexSubImage2D\0\
    glCompressedTexSubImage3D\0\
    glGetCompressedTexImage\0\
    glSampleCoverage\0\
    glBlendColor\0\
    glBlendEquation\0\
    glBlendFuncSeparate\0\
    glMultiDrawArrays\0\
    glMultiDrawElements\0\
    glPointParameterf\0\
    glPointParameterfv\0\
    glPointParameteri\0\
    glPointParameteriv\0\
    glBeginQuery\0\
    glBindBuffer\0\
    glBufferData\0\
    glBufferSubData\0\
    glDeleteBuffers\0\
    glDeleteQueries\0\
    glEndQuery\0\
    glGenBuffers\0\
    glGenQueries\0\
    glGetBufferParameteriv\0\
    glGetBufferPointerv\0\
    glGetBufferSubData\0\
    glGetQueryObjectiv\0\
    glGetQueryObjectuiv\0\
    glGetQueryiv\0\
    glIsBuffer\0\
    glIsQuery\0\
    glMapBuffer\0\
    glUnmapBuffer\0\
    glAttachShader\0\
    glBindAttribLocation\0\
    glBlendEquationSeparate\0\
    glCompileShader\0\
    glCreateProgram\0\
    glCreateShader\0\
    glDeleteProgram\0\
    glDeleteShader\0\
    glDetachShader\0\
    glDisableVertexAttribArray\0\
    glDrawBuffers\0\
    glEnableVertexAttribArray\0\
    glGetActiveAttrib\0\
    glGetActiveUniform\0\
    glGetAttachedShaders\0\
    glGetAttribLocation\0\
    glGetProgramInfoLog\0\
    glGetProgramiv\0\
    glGetShaderInfoLog\0\
    glGetShaderSource\0\
    glGetShaderiv\0\
    glGetUniformLocation\0\
    glGetUniformfv\0\
    glGetUniformiv\0\
    glGetVertexAttribPointerv\0\
    glGetVertexAttribdv\0\
    glGetVertexAttribfv\0\
    glGetVertexAttribiv\0\
    glIsProgram\0\
    glIsShader\0\
    glLinkProgram\0\
    glShaderSource\0\
    glStencilFuncSeparate\0\
    glStencilMaskSeparate\0\
    glStencilOpSeparate\0\
    glUniform1f\0\
    glUniform1fv\0\
    glUniform1i\0\
    glUniform1iv\0\
    glUniform2f\0\
    glUniform2fv\0\
    glUniform2i\0\
    glUniform2iv\0\
    glUniform3f\0\
    glUniform3fv\0\
    glUniform3i\0\
    glUniform3iv\0\
    glUniform4f\0\
    glUniform4fv\0\
    glUniform4i\0\
    glUniform4iv\0\
    glUniformMatrix2fv\0\
    glUniformMatrix3fv\0\
    glUniformMatrix4fv\0\
    glUseProgram\0\
    glValidateProgram\0\
    glVertexAttrib1d\0\
    glVertexAttrib1dv\0\
    glVertexAttrib1f\0\
    glVertexAttrib1fv\0\
    glVertexAttrib1s\0\
    glVertexAttrib1sv\0\
    glVertexAttrib2d\0\
    glVertexAttrib2dv\0\
    glVertexAttrib2f\0\
    glVertexAttrib2fv\0\
    glVertexAttrib2s\0\
    glVertexAttrib2sv\0\
    glVertexAttrib3d\0\
    glVertexAttrib3dv\0\
    glVertexAttrib3f\0\
    glVertexAttrib3fv\0\
    glVertexAttrib3s\0\
    glVertexAttrib3sv\0\
    glVertexAttrib4Nbv\0\
    glVertexAttrib4Niv\0\
    glVertexAttrib4Nsv\0\
    glVertexAttrib4Nub\0\
    glVertexAttrib4Nubv\0\
    glVertexAttrib4Nuiv\0\
    glVertexAttrib4Nusv\0\
    glVertexAttrib4bv\0\
    glVertexAttrib4d\0\
    glVertexAttrib4dv\0\
    glVertexAttrib4f\0\
    glVertexAttrib4fv\0\
    glVertexAttrib4iv\0\
    glVertexAttrib4s\0\
    glVertexAttrib4sv\0\
    glVertexAttrib4ubv\0\
    glVertexAttrib4uiv\0\
    glVertexAttrib4usv\0\
    glVertexAttribPointer\0\
    glUniformMatrix2x3fv\0\
    glUniformMatrix2x4fv\0\
    glUniformMatrix3x2fv\0\
    glUniformMatrix3x4fv\0\
    glUniformMatrix4x2fv\0\
    glUniformMatrix4x3fv\0\
    glBeginConditionalRender\0\
    glBeginTransformFeedback\0\
    glBindFragDataLocation\0\
    glBindFramebuffer\0\
    glBindRenderbuffer\0\
    glBindVertexArray\0\
    glBlitFramebuffer\0\
    glCheckFramebufferStatus\0\
    glClampColor\0\
    glClearBufferfi\0\
    glClearBufferfv\0\
    glClearBufferiv\0\
    glClearBufferuiv\0\
    glColorMaski\0\
    glDeleteFramebuffers\0\
    glDeleteRenderbuffers\0\
    glDeleteVertexArrays\0\
    glDisablei\0\
    glEnablei\0\
    glEndConditionalRender\0\
    glEndTransformFeedback\0\
    glFlushMappedBufferRange\0\
    glFramebufferRenderbuffer\0\
    glFramebufferTexture1D\0\
    glFramebufferTexture2D\0\
    glFramebufferTexture3D\0\
    glFramebufferTextureLayer\0\
    glGenFramebuffers\0\
    glGenRenderbuffers\0\
    glGenVertexArrays\0\
    glGenerateMipmap\0\
    glGetBooleani_v\0\
    glGetFragDataLocation\0\
    glGetFramebufferAttachmentParameteriv\0\
    glGetRenderbufferParameteriv\0\
    glGetStringi\0\
    glGetTexParameterIiv\0\
    glGetTexParameterIuiv\0\
    glGetTransformFeedbackVarying\0\
    glGetUniformuiv\0\
    glGetVertexAttribIiv\0\
    glGetVertexAttribIuiv\0\
    glIsEnabledi\0\
    glIsFramebuffer\0\
    glIsRenderbuffer\0\
    glIsVertexArray\0\
    glMapBufferRange\0\
    glRenderbufferStorage\0\
    glRenderbufferStorageMultisample\0\
    glTexParameterIiv\0\
    glTexParameterIuiv\0\
    glTransformFeedbackVaryings\0\
    glUniform1ui\0\
    glUniform1uiv\0\
    glUniform2ui\0\
    glUniform2uiv\0\
    glUniform3ui\0\
    glUniform3uiv\0\
    glUniform4ui\0\
    glUniform4uiv\0\
    glVertexAttribI1i\0\
    glVertexAttribI1iv\0\
    glVertexAttribI1ui\0\
    glVertexAttribI1uiv\0\
    glVertexAttribI2i\0\
    glVertexAttribI2iv\0\
    glVertexAttribI2ui\0\
    glVertexAttribI2uiv\0\
    glVertexAttribI3i\0\
    glVertexAttribI3iv\0\
    glVertexAttribI3ui\0\
    glVertexAttribI3uiv\0\
    glVertexAttribI4bv\0\
    glVertexAttribI4i\0\
    glVertexAttribI4iv\0\
    glVertexAttribI4sv\0\
    glVertexAttribI4ubv\0\
    glVertexAttribI4ui\0\
    glVertexAttribI4uiv\0\
    glVertexAttribI4usv\0\
    glVertexAttribIPointer\0\
    glBindBufferBase\0\
    glBindBufferRange\0\
    glGetIntegeri_v\0\
    glCopyBufferSubData\0\
    glDrawArraysInstanced\0\
    glDrawElementsInstanced\0\
    glGetActiveUniformBlockName\0\
    glGetActiveUniformBlockiv\0\
    glGetActiveUniformName\0\
    glGetActiveUniformsiv\0\
    glGetUniformBlockIndex\0\
    glGetUniformIndices\0\
    glPrimitiveRestartIndex\0\
    glTexBuffer\0\
    glUniformBlockBinding\0\
    glClientWaitSync\0\
    glDeleteSync\0\
    glDrawElementsBaseVertex\0\
    glDrawElementsInstancedBaseVertex\0\
    glDrawRangeElementsBaseVertex\0\
    glFenceSync\0\
    glFramebufferTexture\0\
    glGetBufferParameteri64v\0\
    glGetInteger64i_v\0\
    glGetInteger64v\0\
    glGetMultisamplefv\0\
    glGetSynciv\0\
    glIsSync\0\
    glMultiDrawElementsBaseVertex\0\
    glProvokingVertex\0\
    glSampleMaski\0\
    glTexImage2DMultisample\0\
    glTexImage3DMultisample\0\
    glWaitSync\0\
    glBindFragDataLocationIndexed\0\
    glBindSampler\0\
    glDeleteSamplers\0\
    glGenSamplers\0\
    glGetFragDataIndex\0\
    glGetQueryObjecti64v\0\
    glGetQueryObjectui64v\0\
    glGetSamplerParameterIiv\0\
    glGetSamplerParameterIuiv\0\
    glGetSamplerParameterfv\0\
    glGetSamplerParameteriv\0\
    glIsSampler\0\
    glQueryCounter\0\
    glSamplerParameterIiv\0\
    glSamplerParameterIuiv\0\
    glSamplerParameterf\0\
    glSamplerParameterfv\0\
    glSamplerParameteri\0\
    glSamplerParameteriv\0\
    glVertexAttribDivisor\0\
    glVertexAttribP1ui\0\
    glVertexAttribP1uiv\0\
    glVertexAttribP2ui\0\
    glVertexAttribP2uiv\0\
    glVertexAttribP3ui\0\
    glVertexAttribP3uiv\0\
    glVertexAttribP4ui\0\
    glVertexAttribP4uiv\0\
";

// Byte offset of each command name in FN_NAME_DATA, indexed in
// lockstep with the pfn table (slot [i] == command i).
#[rustfmt::skip]
static FN_NAME_OFFSETS: [u16; COMMAND_COUNT] = [
          0, // [0] glBlendFunc
         12, // [1] glClear
         20, // [2] glClearColor
         33, // [3] glClearDepth
         46, // [4] glClearStencil
         61, // [5] glColorMask
         73, // [6] glCullFace
         84, // [7] glDepthFunc
         96, // [8] glDepthMask
        108, // [9] glDepthRange
        121, // [10] glDisable
        131, // [11] glDrawBuffer
        144, // [12] glEnable
        153, // [13] glFinish
        162, // [14] glFlush
        170, // [15] glFrontFace
        182, // [16] glGetBooleanv
        196, // [17] glGetDoublev
        209, // [18] glGetError
        220, // [19] glGetFloatv
        232, // [20] glGetIntegerv
        246, // [21] glGetString
        258, // [22] glGetTexImage
        272, // [23] glGetTexLevelParameterfv
        297, // [24] glGetTexLevelParameteriv
        322, // [25] glGetTexParameterfv
        342, // [26] glGetTexParameteriv
        362, // [27] glHint
        369, // [28] glIsEnabled
        381, // [29] glLineWidth
        393, // [30] glLogicOp
        403, // [31] glPixelStoref
        417, // [32] glPixelStorei
        431, // [33] glPointSize
        443, // [34] glPolygonMode
        457, // [35] glReadBuffer
        470, // [36] glReadPixels
        483, // [37] glScissor
        493, // [38] glStencilFunc
        507, // [39] glStencilMask
        521, // [40] glStencilOp
        533, // [41] glTexImage1D
        546, // [42] glTexImage2D
        559, // [43] glTexParameterf
        575, // [44] glTexParameterfv
        592, // [45] glTexParameteri
        608, // [46] glTexParameteriv
        625, // [47] glViewport
        636, // [48] glBindTexture
        650, // [49] glCopyTexImage1D
        667, // [50] glCopyTexImage2D
        684, // [51] glCopyTexSubImage1D
        704, // [52] glCopyTexSubImage2D
        724, // [53] glDeleteTextures
        741, // [54] glDrawArrays
        754, // [55] glDrawElements
        769, // [56] glGenTextures
        783, // [57] glIsTexture
        795, // [58] glPolygonOffset
        811, // [59] glTexSubImage1D
        827, // [60] glTexSubImage2D
        843, // [61] glCopyTexSubImage3D
        863, // [62] glDrawRangeElements
        883, // [63] glTexImage3D
        896, // [64] glTexSubImage3D
        912, // [65] glActiveTexture
        928, // [66] glCompressedTexImage1D
        951, // [67] glCompressedTexImage2D
        974, // [68] glCompressedTexImage3D
        997, // [69] glCompressedTexSubImage1D
       1023, // [70] glCompressedTexSubImage2D
       1049, // [71] glCompressedTexSubImage3D
       1075, // [72] glGetCompressedTexImage
       1099, // [73] glSampleCoverage
       1116, // [74] glBlendColor
       1129, // [75] glBlendEquation
       1145, // [76] glBlendFuncSeparate
       1165, // [77] glMultiDrawArrays
       1183, // [78] glMultiDrawElements
       1203, // [79] glPointParameterf
       1221, // [80] glPointParameterfv
       1240, // [81] glPointParameteri
       1258, // [82] glPointParameteriv
       1277, // [83] glBeginQuery
       1290, // [84] glBindBuffer
       1303, // [85] glBufferData
       1316, // [86] glBufferSubData
       1332, // [87] glDeleteBuffers
       1348, // [88] glDeleteQueries
       1364, // [89] glEndQuery
       1375, // [90] glGenBuffers
       1388, // [91] glGenQueries
       1401, // [92] glGetBufferParameteriv
       1424, // [93] glGetBufferPointerv
       1444, // [94] glGetBufferSubData
       1463, // [95] glGetQueryObjectiv
       1482, // [96] glGetQueryObjectuiv
       1502, // [97] glGetQueryiv
       1515, // [98] glIsBuffer
       1526, // [99] glIsQuery
       1536, // [100] glMapBuffer
       1548, // [101] glUnmapBuffer
       1562, // [102] glAttachShader
       1577, // [103] glBindAttribLocation
       1598, // [104] glBlendEquationSeparate
       1622, // [105] glCompileShader
       1638, // [106] glCreateProgram
       1654, // [107] glCreateShader
       1669, // [108] glDeleteProgram
       1685, // [109] glDeleteShader
       1700, // [110] glDetachShader
       1715, // [111] glDisableVertexAttribArray
       1742, // [112] glDrawBuffers
       1756, // [113] glEnableVertexAttribArray
       1782, // [114] glGetActiveAttrib
       1800, // [115] glGetActiveUniform
       1819, // [116] glGetAttachedShaders
       1840, // [117] glGetAttribLocation
       1860, // [118] glGetProgramInfoLog
       1880, // [119] glGetProgramiv
       1895, // [120] glGetShaderInfoLog
       1914, // [121] glGetShaderSource
       1932, // [122] glGetShaderiv
       1946, // [123] glGetUniformLocation
       1967, // [124] glGetUniformfv
       1982, // [125] glGetUniformiv
       1997, // [126] glGetVertexAttribPointerv
       2023, // [127] glGetVertexAttribdv
       2043, // [128] glGetVertexAttribfv
       2063, // [129] glGetVertexAttribiv
       2083, // [130] glIsProgram
       2095, // [131] glIsShader
       2106, // [132] glLinkProgram
       2120, // [133] glShaderSource
       2135, // [134] glStencilFuncSeparate
       2157, // [135] glStencilMaskSeparate
       2179, // [136] glStencilOpSeparate
       2199, // [137] glUniform1f
       2211, // [138] glUniform1fv
       2224, // [139] glUniform1i
       2236, // [140] glUniform1iv
       2249, // [141] glUniform2f
       2261, // [142] glUniform2fv
       2274, // [143] glUniform2i
       2286, // [144] glUniform2iv
       2299, // [145] glUniform3f
       2311, // [146] glUniform3fv
       2324, // [147] glUniform3i
       2336, // [148] glUniform3iv
       2349, // [149] glUniform4f
       2361, // [150] glUniform4fv
       2374, // [151] glUniform4i
       2386, // [152] glUniform4iv
       2399, // [153] glUniformMatrix2fv
       2418, // [154] glUniformMatrix3fv
       2437, // [155] glUniformMatrix4fv
       2456, // [156] glUseProgram
       2469, // [157] glValidateProgram
       2487, // [158] glVertexAttrib1d
       2504, // [159] glVertexAttrib1dv
       2522, // [160] glVertexAttrib1f
       2539, // [161] glVertexAttrib1fv
       2557, // [162] glVertexAttrib1s
       2574, // [163] glVertexAttrib1sv
       2592, // [164] glVertexAttrib2d
       2609, // [165] glVertexAttrib2dv
       2627, // [166] glVertexAttrib2f
       2644, // [167] glVertexAttrib2fv
       2662, // [168] glVertexAttrib2s
       2679, // [169] glVertexAttrib2sv
       2697, // [170] glVertexAttrib3d
       2714, // [171] glVertexAttrib3dv
       2732, // [172] glVertexAttrib3f
       2749, // [173] glVertexAttrib3fv
       2767, // [174] glVertexAttrib3s
       2784, // [175] glVertexAttrib3sv
       2802, // [176] glVertexAttrib4Nbv
       2821, // [177] glVertexAttrib4Niv
       2840, // [178] glVertexAttrib4Nsv
       2859, // [179] glVertexAttrib4Nub
       2878, // [180] glVertexAttrib4Nubv
       2898, // [181] glVertexAttrib4Nuiv
       2918, // [182] glVertexAttrib4Nusv
       2938, // [183] glVertexAttrib4bv
       2956, // [184] glVertexAttrib4d
       2973, // [185] glVertexAttrib4dv
       2991, // [186] glVertexAttrib4f
       3008, // [187] glVertexAttrib4fv
       3026, // [188] glVertexAttrib4iv
       3044, // [189] glVertexAttrib4s
       3061, // [190] glVertexAttrib4sv
       3079, // [191] glVertexAttrib4ubv
       3098, // [192] glVertexAttrib4uiv
       3117, // [193] glVertexAttrib4usv
       3136, // [194] glVertexAttribPointer
       3158, // [195] glUniformMatrix2x3fv
       3179, // [196] glUniformMatrix2x4fv
       3200, // [197] glUniformMatrix3x2fv
       3221, // [198] glUniformMatrix3x4fv
       3242, // [199] glUniformMatrix4x2fv
       3263, // [200] glUniformMatrix4x3fv
       3284, // [201] glBeginConditionalRender
       3309, // [202] glBeginTransformFeedback
       3334, // [203] glBindFragDataLocation
       3357, // [204] glBindFramebuffer
       3375, // [205] glBindRenderbuffer
       3394, // [206] glBindVertexArray
       3412, // [207] glBlitFramebuffer
       3430, // [208] glCheckFramebufferStatus
       3455, // [209] glClampColor
       3468, // [210] glClearBufferfi
       3484, // [211] glClearBufferfv
       3500, // [212] glClearBufferiv
       3516, // [213] glClearBufferuiv
       3533, // [214] glColorMaski
       3546, // [215] glDeleteFramebuffers
       3567, // [216] glDeleteRenderbuffers
       3589, // [217] glDeleteVertexArrays
       3610, // [218] glDisablei
       3621, // [219] glEnablei
       3631, // [220] glEndConditionalRender
       3654, // [221] glEndTransformFeedback
       3677, // [222] glFlushMappedBufferRange
       3702, // [223] glFramebufferRenderbuffer
       3728, // [224] glFramebufferTexture1D
       3751, // [225] glFramebufferTexture2D
       3774, // [226] glFramebufferTexture3D
       3797, // [227] glFramebufferTextureLayer
       3823, // [228] glGenFramebuffers
       3841, // [229] glGenRenderbuffers
       3860, // [230] glGenVertexArrays
       3878, // [231] glGenerateMipmap
       3895, // [232] glGetBooleani_v
       3911, // [233] glGetFragDataLocation
       3933, // [234] glGetFramebufferAttachmentParameteriv
       3971, // [235] glGetRenderbufferParameteriv
       4000, // [236] glGetStringi
       4013, // [237] glGetTexParameterIiv
       4034, // [238] glGetTexParameterIuiv
       4056, // [239] glGetTransformFeedbackVarying
       4086, // [240] glGetUniformuiv
       4102, // [241] glGetVertexAttribIiv
       4123, // [242] glGetVertexAttribIuiv
       4145, // [243] glIsEnabledi
       4158, // [244] glIsFramebuffer
       4174, // [245] glIsRenderbuffer
       4191, // [246] glIsVertexArray
       4207, // [247] glMapBufferRange
       4224, // [248] glRenderbufferStorage
       4246, // [249] glRenderbufferStorageMultisample
       4279, // [250] glTexParameterIiv
       4297, // [251] glTexParameterIuiv
       4316, // [252] glTransformFeedbackVaryings
       4344, // [253] glUniform1ui
       4357, // [254] glUniform1uiv
       4371, // [255] glUniform2ui
       4384, // [256] glUniform2uiv
       4398, // [257] glUniform3ui
       4411, // [258] glUniform3uiv
       4425, // [259] glUniform4ui
       4438, // [260] glUniform4uiv
       4452, // [261] glVertexAttribI1i
       4470, // [262] glVertexAttribI1iv
       4489, // [263] glVertexAttribI1ui
       4508, // [264] glVertexAttribI1uiv
       4528, // [265] glVertexAttribI2i
       4546, // [266] glVertexAttribI2iv
       4565, // [267] glVertexAttribI2ui
       4584, // [268] glVertexAttribI2uiv
       4604, // [269] glVertexAttribI3i
       4622, // [270] glVertexAttribI3iv
       4641, // [271] glVertexAttribI3ui
       4660, // [272] glVertexAttribI3uiv
       4680, // [273] glVertexAttribI4bv
       4699, // [274] glVertexAttribI4i
       4717, // [275] glVertexAttribI4iv
       4736, // [276] glVertexAttribI4sv
       4755, // [277] glVertexAttribI4ubv
       4775, // [278] glVertexAttribI4ui
       4794, // [279] glVertexAttribI4uiv
       4814, // [280] glVertexAttribI4usv
       4834, // [281] glVertexAttribIPointer
       4857, // [282] glBindBufferBase
       4874, // [283] glBindBufferRange
       4892, // [284] glGetIntegeri_v
       4908, // [285] glCopyBufferSubData
       4928, // [286] glDrawArraysInstanced
       4950, // [287] glDrawElementsInstanced
       4974, // [288] glGetActiveUniformBlockName
       5002, // [289] glGetActiveUniformBlockiv
       5028, // [290] glGetActiveUniformName
       5051, // [291] glGetActiveUniformsiv
       5073, // [292] glGetUniformBlockIndex
       5096, // [293] glGetUniformIndices
       5116, // [294] glPrimitiveRestartIndex
       5140, // [295] glTexBuffer
       5152, // [296] glUniformBlockBinding
       5174, // [297] glClientWaitSync
       5191, // [298] glDeleteSync
       5204, // [299] glDrawElementsBaseVertex
       5229, // [300] glDrawElementsInstancedBaseVertex
       5263, // [301] glDrawRangeElementsBaseVertex
       5293, // [302] glFenceSync
       5305, // [303] glFramebufferTexture
       5326, // [304] glGetBufferParameteri64v
       5351, // [305] glGetInteger64i_v
       5369, // [306] glGetInteger64v
       5385, // [307] glGetMultisamplefv
       5404, // [308] glGetSynciv
       5416, // [309] glIsSync
       5425, // [310] glMultiDrawElementsBaseVertex
       5455, // [311] glProvokingVertex
       5473, // [312] glSampleMaski
       5487, // [313] glTexImage2DMultisample
       5511, // [314] glTexImage3DMultisample
       5535, // [315] glWaitSync
       5546, // [316] glBindFragDataLocationIndexed
       5576, // [317] glBindSampler
       5590, // [318] glDeleteSamplers
       5607, // [319] glGenSamplers
       5621, // [320] glGetFragDataIndex
       5640, // [321] glGetQueryObjecti64v
       5661, // [322] glGetQueryObjectui64v
       5683, // [323] glGetSamplerParameterIiv
       5708, // [324] glGetSamplerParameterIuiv
       5734, // [325] glGetSamplerParameterfv
       5758, // [326] glGetSamplerParameteriv
       5782, // [327] glIsSampler
       5794, // [328] glQueryCounter
       5809, // [329] glSamplerParameterIiv
       5831, // [330] glSamplerParameterIuiv
       5854, // [331] glSamplerParameterf
       5874, // [332] glSamplerParameterfv
       5895, // [333] glSamplerParameteri
       5915, // [334] glSamplerParameteriv
       5936, // [335] glVertexAttribDivisor
       5958, // [336] glVertexAttribP1ui
       5977, // [337] glVertexAttribP1uiv
       5997, // [338] glVertexAttribP2ui
       6016, // [339] glVertexAttribP2uiv
       6036, // [340] glVertexAttribP3ui
       6055, // [341] glVertexAttribP3uiv
       6075, // [342] glVertexAttribP4ui
       6094, // [343] glVertexAttribP4uiv
];

#[rustfmt::skip]
static FEATURE_RANGES: [(u16, u16, u16); 12] = [
    (   0,    0,   48), // GL_VERSION_1_0
    (   1,   48,   13), // GL_VERSION_1_1
    (   2,   61,    4), // GL_VERSION_1_2
    (   3,   65,    9), // GL_VERSION_1_3
    (   4,   74,    9), // GL_VERSION_1_4
    (   5,   83,   19), // GL_VERSION_1_5
    (   6,  102,   93), // GL_VERSION_2_0
    (   7,  195,    6), // GL_VERSION_2_1
    (   8,  201,   84), // GL_VERSION_3_0
    (   9,  282,   15), // GL_VERSION_3_1
    (  10,  297,   19), // GL_VERSION_3_2
    (  11,  316,   28), // GL_VERSION_3_3
];

#[rustfmt::skip]
static EXT_RANGES_gl: [(u16, u16, u16); 0] = [
];

// ── Extensions ──────────────────────────────────────────────
pub const EXT_COUNT: usize = 0;

// XXH3-64 of each extension name, sorted for binary search.
#[rustfmt::skip]
static EXT_HASH_KEYS: [u64; EXT_COUNT] = [
];
// extArray index for the correspondingly-ranked EXT_HASH_KEYS entry.
#[rustfmt::skip]
static EXT_HASH_IDX: [u16; EXT_COUNT] = [
];

/// XXH3-64 of a NUL-terminated driver extension string — the same hash
/// gloam pre-baked into EXT_HASH_KEYS, so driver names match the table.
#[inline]
unsafe fn __ext_hash(p: *const GLubyte) -> u64 {
    unsafe {
        let mut len = 0usize;
        while *p.add(len) != 0 {
            len += 1;
        }
        xxhash_rust::xxh3::xxh3_64(core::slice::from_raw_parts(p, len))
    }
}

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
    debug_assert!(false, "unloaded GL function called in a no-error build");
    unsafe { core::hint::unreachable_unchecked() }
}

// ── Version parsing ─────────────────────────────────────────
/// True if the NUL-terminated string at `p` begins with `pre`.
unsafe fn __starts_with(p: *const GLubyte, pre: &[u8]) -> bool {
    let mut i = 0;
    while i < pre.len() {
        if unsafe { *p.add(i) } != pre[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// Parse a GL_VERSION string (e.g. `"3.3.0 ..."` or `"OpenGL ES 3.0"`)
/// into a packed `major << 8 | minor`, or 0 if unparseable.
unsafe fn __parse_gl_version(mut p: *const GLubyte) -> u32 {
    if p.is_null() {
        return 0;
    }
    const PREFIXES: [&[u8]; 5] = [
        b"OpenGL ES-CM ",
        b"OpenGL ES-CL ",
        b"OpenGL ES ",
        b"OpenGL SC ",
        b"OpenGL ",
    ];
    unsafe {
        let mut k = 0;
        while k < PREFIXES.len() {
            if __starts_with(p, PREFIXES[k]) {
                p = p.add(PREFIXES[k].len());
                break;
            }
            k += 1;
        }
        let mut major: u32 = 0;
        while (*p).is_ascii_digit() {
            major = major * 10 + (*p - b'0') as u32;
            p = p.add(1);
        }
        let mut minor: u32 = 0;
        if *p == b'.' {
            p = p.add(1);
            while (*p).is_ascii_digit() {
                minor = minor * 10 + (*p - b'0') as u32;
                p = p.add(1);
            }
        }
        (major << 8) | minor
    }
}

// ── Context ─────────────────────────────────────────────────
/// Why a `load_<api>` constructor failed.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LoadError {
    /// `loader` returned null for `glGetString` — the current context
    /// exposes no GL at all (or `loader` is not a GL proc-address source).
    MissingGetString,
    /// `GL_VERSION` was null or unparseable, so no API level could be
    /// detected.
    UnparseableVersion,
}

impl core::fmt::Display for LoadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            LoadError::MissingGetString => "glGetString is not available in this context",
            LoadError::UnparseableVersion => "GL_VERSION missing or unparseable",
        })
    }
}

impl core::error::Error for LoadError {}

/// Loaded GL entry points plus detected feature/extension presence.
/// The PFN table is inline (not boxed) for single-indirection dispatch;
/// not `Clone`/`Copy` to avoid copying the whole table.
pub struct Gl {
    pfns: [*const c_void; COMMAND_COUNT],
    feat: [bool; FEATURE_COUNT],
    ext: [bool; EXT_COUNT],
    version: u32,
}

impl Gl {
    /// Load the gl API against `loader` (a GetProcAddress-style
    /// callback), detecting version then extensions.  `Err` when the
    /// current context has no usable gl.
    ///
    /// # Safety
    /// A matching GL context must be current and `loader` valid.
    #[inline]
    pub unsafe fn load_gl(
        mut loader: impl FnMut(&CStr) -> *const c_void,
    ) -> Result<Self, LoadError> {
        // Immediately erase to `&mut dyn` — the real loader is compiled
        // once, not once per closure type.
        unsafe { Self::load_gl_dyn(&mut loader) }
    }

    unsafe fn load_gl_dyn(
        loader: &mut dyn FnMut(&CStr) -> *const c_void,
    ) -> Result<Self, LoadError> {
        let mut gl = Self {
            pfns: [core::ptr::null(); COMMAND_COUNT],
            feat: [false; FEATURE_COUNT],
            ext: [false; EXT_COUNT],
            version: 0,
        };
        gl.pfns[21] = loader(c"glGetString");
        if gl.pfns[21].is_null() {
            return Err(LoadError::MissingGetString);
        }
        gl.version = unsafe { __parse_gl_version(gl.GetString(GL_VERSION)) };
        if gl.version == 0 {
            return Err(LoadError::UnparseableVersion);
        }
        // Feature presence for this API, from the parsed version.
        gl.feat[0] = gl.version >= 0x0100;
        gl.feat[1] = gl.version >= 0x0101;
        gl.feat[2] = gl.version >= 0x0102;
        gl.feat[3] = gl.version >= 0x0103;
        gl.feat[4] = gl.version >= 0x0104;
        gl.feat[5] = gl.version >= 0x0105;
        gl.feat[6] = gl.version >= 0x0200;
        gl.feat[7] = gl.version >= 0x0201;
        gl.feat[8] = gl.version >= 0x0300;
        gl.feat[9] = gl.version >= 0x0301;
        gl.feat[10] = gl.version >= 0x0302;
        gl.feat[11] = gl.version >= 0x0303;
        for &(fi, start, count) in FEATURE_RANGES.iter() {
            if gl.feat[fi as usize] {
                unsafe { gl.load_range(loader, start, count) };
            }
        }
        unsafe { gl.detect_extensions() };
        for &(ei, start, count) in EXT_RANGES_gl.iter() {
            if gl.ext[ei as usize] {
                unsafe { gl.load_range(loader, start, count) };
            }
        }
        gl.resolve_aliases();
        Ok(gl)
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

    /// # Safety: the loaded entry points must be callable.
    unsafe fn detect_extensions(&mut self) {
        if self.pfns[20].is_null() || self.pfns[236].is_null() {
            return;
        }
        let mut n: GLint = 0;
        unsafe { self.GetIntegerv(GL_NUM_EXTENSIONS, &mut n) };
        let mut i: GLuint = 0;
        while (i as GLint) < n {
            let name = unsafe { self.GetStringi(GL_EXTENSIONS, i) };
            if !name.is_null() {
                let h = unsafe { __ext_hash(name) };
                if let Ok(pos) = EXT_HASH_KEYS.binary_search(&h) {
                    self.ext[EXT_HASH_IDX[pos] as usize] = true;
                }
            }
            i += 1;
        }
    }

    fn resolve_aliases(&mut self) {}

    /// Detected API version, packed as `major << 8 | minor`.
    #[inline]
    pub fn version(&self) -> u32 {
        self.version
    }

    // Dispatch wrappers.  The pointer local is named `__pfn` because GL
    // parameter names (`f`, `n`, ...) could otherwise collide with it.

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BlendFunc(&self, sfactor: GLenum, dfactor: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(0)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(0) },
        };
        unsafe { __pfn(sfactor, dfactor) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Clear(&self, mask: GLbitfield) {
        let __pfn: Option<unsafe extern "system" fn(GLbitfield)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(1)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(1) },
        };
        unsafe { __pfn(mask) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ClearColor(&self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(2)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(2) },
        };
        unsafe { __pfn(red, green, blue, alpha) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ClearDepth(&self, depth: GLdouble) {
        let __pfn: Option<unsafe extern "system" fn(GLdouble)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(3)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(3) },
        };
        unsafe { __pfn(depth) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ClearStencil(&self, s: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(4)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(4) },
        };
        unsafe { __pfn(s) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ColorMask(&self, red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) {
        let __pfn: Option<unsafe extern "system" fn(GLboolean, GLboolean, GLboolean, GLboolean)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(5)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(5) },
        };
        unsafe { __pfn(red, green, blue, alpha) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CullFace(&self, mode: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(6)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(6) },
        };
        unsafe { __pfn(mode) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DepthFunc(&self, func: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(7)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(7) },
        };
        unsafe { __pfn(func) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DepthMask(&self, flag: GLboolean) {
        let __pfn: Option<unsafe extern "system" fn(GLboolean)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(8)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(8) },
        };
        unsafe { __pfn(flag) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DepthRange(&self, n: GLdouble, f: GLdouble) {
        let __pfn: Option<unsafe extern "system" fn(GLdouble, GLdouble)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(9)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(9) },
        };
        unsafe { __pfn(n, f) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Disable(&self, cap: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(10)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(10) },
        };
        unsafe { __pfn(cap) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DrawBuffer(&self, buf: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(11)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(11) },
        };
        unsafe { __pfn(buf) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Enable(&self, cap: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(12)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(12) },
        };
        unsafe { __pfn(cap) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Finish(&self) {
        let __pfn: Option<unsafe extern "system" fn()> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(13)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(13) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Flush(&self) {
        let __pfn: Option<unsafe extern "system" fn()> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(14)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(14) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn FrontFace(&self, mode: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(15)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(15) },
        };
        unsafe { __pfn(mode) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetBooleanv(&self, pname: GLenum, data: *mut GLboolean) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, *mut GLboolean)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(16)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(16) },
        };
        unsafe { __pfn(pname, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetDoublev(&self, pname: GLenum, data: *mut GLdouble) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, *mut GLdouble)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(17)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(17) },
        };
        unsafe { __pfn(pname, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetError(&self) -> GLenum {
        let __pfn: Option<unsafe extern "system" fn() -> GLenum> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(18)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(18) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetFloatv(&self, pname: GLenum, data: *mut GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, *mut GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(19)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(19) },
        };
        unsafe { __pfn(pname, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetIntegerv(&self, pname: GLenum, data: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(20)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(20) },
        };
        unsafe { __pfn(pname, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetString(&self, name: GLenum) -> *const GLubyte {
        let __pfn: Option<unsafe extern "system" fn(GLenum) -> *const GLubyte> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(21)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(21) },
        };
        unsafe { __pfn(name) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetTexImage(&self, target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLenum, GLenum, *mut c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(22)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(22) },
        };
        unsafe { __pfn(target, level, format, type_, pixels) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetTexLevelParameterfv(&self, target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLenum, *mut GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(23)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(23) },
        };
        unsafe { __pfn(target, level, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetTexLevelParameteriv(&self, target: GLenum, level: GLint, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(24)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(24) },
        };
        unsafe { __pfn(target, level, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetTexParameterfv(&self, target: GLenum, pname: GLenum, params: *mut GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *mut GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(25)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(25) },
        };
        unsafe { __pfn(target, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetTexParameteriv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(26)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(26) },
        };
        unsafe { __pfn(target, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Hint(&self, target: GLenum, mode: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(27)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(27) },
        };
        unsafe { __pfn(target, mode) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn IsEnabled(&self, cap: GLenum) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLenum) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(28)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(28) },
        };
        unsafe { __pfn(cap) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn LineWidth(&self, width: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(29)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(29) },
        };
        unsafe { __pfn(width) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn LogicOp(&self, opcode: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(30)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(30) },
        };
        unsafe { __pfn(opcode) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn PixelStoref(&self, pname: GLenum, param: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(31)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(31) },
        };
        unsafe { __pfn(pname, param) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn PixelStorei(&self, pname: GLenum, param: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(32)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(32) },
        };
        unsafe { __pfn(pname, param) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn PointSize(&self, size: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(33)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(33) },
        };
        unsafe { __pfn(size) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn PolygonMode(&self, face: GLenum, mode: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(34)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(34) },
        };
        unsafe { __pfn(face, mode) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ReadBuffer(&self, src: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(35)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(35) },
        };
        unsafe { __pfn(src) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ReadPixels(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *mut c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(36)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(36) },
        };
        unsafe { __pfn(x, y, width, height, format, type_, pixels) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Scissor(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLint, GLsizei, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(37)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(37) },
        };
        unsafe { __pfn(x, y, width, height) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn StencilFunc(&self, func: GLenum, ref_: GLint, mask: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(38)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(38) },
        };
        unsafe { __pfn(func, ref_, mask) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn StencilMask(&self, mask: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(39)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(39) },
        };
        unsafe { __pfn(mask) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn StencilOp(&self, fail: GLenum, zfail: GLenum, zpass: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(40)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(40) },
        };
        unsafe { __pfn(fail, zfail, zpass) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexImage1D(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLint, GLsizei, GLint, GLenum, GLenum, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(41)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(41) },
        };
        unsafe { __pfn(target, level, internalformat, width, border, format, type_, pixels) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexImage2D(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLint, GLsizei, GLsizei, GLint, GLenum, GLenum, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(42)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(42) },
        };
        unsafe { __pfn(target, level, internalformat, width, height, border, format, type_, pixels) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexParameterf(&self, target: GLenum, pname: GLenum, param: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(43)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(43) },
        };
        unsafe { __pfn(target, pname, param) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexParameterfv(&self, target: GLenum, pname: GLenum, params: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(44)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(44) },
        };
        unsafe { __pfn(target, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexParameteri(&self, target: GLenum, pname: GLenum, param: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(45)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(45) },
        };
        unsafe { __pfn(target, pname, param) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexParameteriv(&self, target: GLenum, pname: GLenum, params: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(46)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(46) },
        };
        unsafe { __pfn(target, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLint, GLsizei, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(47)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(47) },
        };
        unsafe { __pfn(x, y, width, height) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BindTexture(&self, target: GLenum, texture: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(48)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(48) },
        };
        unsafe { __pfn(target, texture) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CopyTexImage1D(&self, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(49)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(49) },
        };
        unsafe { __pfn(target, level, internalformat, x, y, width, border) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CopyTexImage2D(&self, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLsizei, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(50)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(50) },
        };
        unsafe { __pfn(target, level, internalformat, x, y, width, height, border) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CopyTexSubImage1D(&self, target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(51)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(51) },
        };
        unsafe { __pfn(target, level, xoffset, x, y, width) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CopyTexSubImage2D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(52)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(52) },
        };
        unsafe { __pfn(target, level, xoffset, yoffset, x, y, width, height) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DeleteTextures(&self, n: GLsizei, textures: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(53)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(53) },
        };
        unsafe { __pfn(n, textures) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DrawArrays(&self, mode: GLenum, first: GLint, count: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(54)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(54) },
        };
        unsafe { __pfn(mode, first, count) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DrawElements(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLsizei, GLenum, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(55)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(55) },
        };
        unsafe { __pfn(mode, count, type_, indices) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GenTextures(&self, n: GLsizei, textures: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(56)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(56) },
        };
        unsafe { __pfn(n, textures) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn IsTexture(&self, texture: GLuint) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLuint) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(57)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(57) },
        };
        unsafe { __pfn(texture) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn PolygonOffset(&self, factor: GLfloat, units: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLfloat, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(58)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(58) },
        };
        unsafe { __pfn(factor, units) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexSubImage1D(&self, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLint, GLsizei, GLenum, GLenum, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(59)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(59) },
        };
        unsafe { __pfn(target, level, xoffset, width, format, type_, pixels) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexSubImage2D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(60)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(60) },
        };
        unsafe { __pfn(target, level, xoffset, yoffset, width, height, format, type_, pixels) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CopyTexSubImage3D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(61)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(61) },
        };
        unsafe { __pfn(target, level, xoffset, yoffset, zoffset, x, y, width, height) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DrawRangeElements(&self, mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint, GLuint, GLsizei, GLenum, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(62)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(62) },
        };
        unsafe { __pfn(mode, start, end, count, type_, indices) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexImage3D(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLint, GLsizei, GLsizei, GLsizei, GLint, GLenum, GLenum, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(63)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(63) },
        };
        unsafe { __pfn(target, level, internalformat, width, height, depth, border, format, type_, pixels) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexSubImage3D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(64)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(64) },
        };
        unsafe { __pfn(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ActiveTexture(&self, texture: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(65)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(65) },
        };
        unsafe { __pfn(texture) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CompressedTexImage1D(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLint, GLsizei, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(66)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(66) },
        };
        unsafe { __pfn(target, level, internalformat, width, border, imageSize, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CompressedTexImage2D(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLsizei, GLint, GLsizei, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(67)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(67) },
        };
        unsafe { __pfn(target, level, internalformat, width, height, border, imageSize, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CompressedTexImage3D(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLsizei, GLsizei, GLint, GLsizei, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(68)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(68) },
        };
        unsafe { __pfn(target, level, internalformat, width, height, depth, border, imageSize, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CompressedTexSubImage1D(&self, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLint, GLsizei, GLenum, GLsizei, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(69)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(69) },
        };
        unsafe { __pfn(target, level, xoffset, width, format, imageSize, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CompressedTexSubImage2D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLsizei, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(70)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(70) },
        };
        unsafe { __pfn(target, level, xoffset, yoffset, width, height, format, imageSize, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CompressedTexSubImage3D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLsizei, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(71)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(71) },
        };
        unsafe { __pfn(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetCompressedTexImage(&self, target: GLenum, level: GLint, img: *mut c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, *mut c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(72)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(72) },
        };
        unsafe { __pfn(target, level, img) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn SampleCoverage(&self, value: GLfloat, invert: GLboolean) {
        let __pfn: Option<unsafe extern "system" fn(GLfloat, GLboolean)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(73)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(73) },
        };
        unsafe { __pfn(value, invert) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BlendColor(&self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(74)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(74) },
        };
        unsafe { __pfn(red, green, blue, alpha) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BlendEquation(&self, mode: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(75)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(75) },
        };
        unsafe { __pfn(mode) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BlendFuncSeparate(&self, sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLenum, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(76)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(76) },
        };
        unsafe { __pfn(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn MultiDrawArrays(&self, mode: GLenum, first: *const GLint, count: *const GLsizei, drawcount: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, *const GLint, *const GLsizei, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(77)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(77) },
        };
        unsafe { __pfn(mode, first, count, drawcount) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn MultiDrawElements(&self, mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, drawcount: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, *const GLsizei, GLenum, *const *const c_void, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(78)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(78) },
        };
        unsafe { __pfn(mode, count, type_, indices, drawcount) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn PointParameterf(&self, pname: GLenum, param: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(79)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(79) },
        };
        unsafe { __pfn(pname, param) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn PointParameterfv(&self, pname: GLenum, params: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(80)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(80) },
        };
        unsafe { __pfn(pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn PointParameteri(&self, pname: GLenum, param: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(81)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(81) },
        };
        unsafe { __pfn(pname, param) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn PointParameteriv(&self, pname: GLenum, params: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(82)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(82) },
        };
        unsafe { __pfn(pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BeginQuery(&self, target: GLenum, id: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(83)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(83) },
        };
        unsafe { __pfn(target, id) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BindBuffer(&self, target: GLenum, buffer: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(84)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(84) },
        };
        unsafe { __pfn(target, buffer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BufferData(&self, target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLsizeiptr, *const c_void, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(85)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(85) },
        };
        unsafe { __pfn(target, size, data, usage) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BufferSubData(&self, target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLintptr, GLsizeiptr, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(86)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(86) },
        };
        unsafe { __pfn(target, offset, size, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DeleteBuffers(&self, n: GLsizei, buffers: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(87)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(87) },
        };
        unsafe { __pfn(n, buffers) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DeleteQueries(&self, n: GLsizei, ids: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(88)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(88) },
        };
        unsafe { __pfn(n, ids) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn EndQuery(&self, target: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(89)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(89) },
        };
        unsafe { __pfn(target) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GenBuffers(&self, n: GLsizei, buffers: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(90)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(90) },
        };
        unsafe { __pfn(n, buffers) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GenQueries(&self, n: GLsizei, ids: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(91)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(91) },
        };
        unsafe { __pfn(n, ids) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetBufferParameteriv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(92)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(92) },
        };
        unsafe { __pfn(target, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetBufferPointerv(&self, target: GLenum, pname: GLenum, params: *mut *mut c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *mut *mut c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(93)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(93) },
        };
        unsafe { __pfn(target, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetBufferSubData(&self, target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLintptr, GLsizeiptr, *mut c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(94)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(94) },
        };
        unsafe { __pfn(target, offset, size, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetQueryObjectiv(&self, id: GLuint, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(95)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(95) },
        };
        unsafe { __pfn(id, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetQueryObjectuiv(&self, id: GLuint, pname: GLenum, params: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(96)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(96) },
        };
        unsafe { __pfn(id, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetQueryiv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(97)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(97) },
        };
        unsafe { __pfn(target, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn IsBuffer(&self, buffer: GLuint) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLuint) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(98)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(98) },
        };
        unsafe { __pfn(buffer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn IsQuery(&self, id: GLuint) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLuint) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(99)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(99) },
        };
        unsafe { __pfn(id) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn MapBuffer(&self, target: GLenum, access: GLenum) -> *mut c_void {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum) -> *mut c_void> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(100)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(100) },
        };
        unsafe { __pfn(target, access) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn UnmapBuffer(&self, target: GLenum) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLenum) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(101)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(101) },
        };
        unsafe { __pfn(target) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn AttachShader(&self, program: GLuint, shader: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(102)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(102) },
        };
        unsafe { __pfn(program, shader) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BindAttribLocation(&self, program: GLuint, index: GLuint, name: *const GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint, *const GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(103)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(103) },
        };
        unsafe { __pfn(program, index, name) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BlendEquationSeparate(&self, modeRGB: GLenum, modeAlpha: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(104)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(104) },
        };
        unsafe { __pfn(modeRGB, modeAlpha) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CompileShader(&self, shader: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(105)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(105) },
        };
        unsafe { __pfn(shader) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CreateProgram(&self) -> GLuint {
        let __pfn: Option<unsafe extern "system" fn() -> GLuint> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(106)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(106) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CreateShader(&self, type_: GLenum) -> GLuint {
        let __pfn: Option<unsafe extern "system" fn(GLenum) -> GLuint> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(107)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(107) },
        };
        unsafe { __pfn(type_) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DeleteProgram(&self, program: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(108)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(108) },
        };
        unsafe { __pfn(program) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DeleteShader(&self, shader: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(109)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(109) },
        };
        unsafe { __pfn(shader) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DetachShader(&self, program: GLuint, shader: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(110)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(110) },
        };
        unsafe { __pfn(program, shader) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DisableVertexAttribArray(&self, index: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(111)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(111) },
        };
        unsafe { __pfn(index) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DrawBuffers(&self, n: GLsizei, bufs: *const GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *const GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(112)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(112) },
        };
        unsafe { __pfn(n, bufs) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn EnableVertexAttribArray(&self, index: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(113)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(113) },
        };
        unsafe { __pfn(index) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetActiveAttrib(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(114)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(114) },
        };
        unsafe { __pfn(program, index, bufSize, length, size, type_, name) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetActiveUniform(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(115)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(115) },
        };
        unsafe { __pfn(program, index, bufSize, length, size, type_, name) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetAttachedShaders(&self, program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(116)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(116) },
        };
        unsafe { __pfn(program, maxCount, count, shaders) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetAttribLocation(&self, program: GLuint, name: *const GLchar) -> GLint {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLchar) -> GLint> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(117)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(117) },
        };
        unsafe { __pfn(program, name) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetProgramInfoLog(&self, program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(118)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(118) },
        };
        unsafe { __pfn(program, bufSize, length, infoLog) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetProgramiv(&self, program: GLuint, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(119)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(119) },
        };
        unsafe { __pfn(program, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetShaderInfoLog(&self, shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(120)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(120) },
        };
        unsafe { __pfn(shader, bufSize, length, infoLog) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetShaderSource(&self, shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(121)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(121) },
        };
        unsafe { __pfn(shader, bufSize, length, source) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetShaderiv(&self, shader: GLuint, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(122)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(122) },
        };
        unsafe { __pfn(shader, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetUniformLocation(&self, program: GLuint, name: *const GLchar) -> GLint {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLchar) -> GLint> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(123)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(123) },
        };
        unsafe { __pfn(program, name) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetUniformfv(&self, program: GLuint, location: GLint, params: *mut GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLint, *mut GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(124)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(124) },
        };
        unsafe { __pfn(program, location, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetUniformiv(&self, program: GLuint, location: GLint, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLint, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(125)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(125) },
        };
        unsafe { __pfn(program, location, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetVertexAttribPointerv(&self, index: GLuint, pname: GLenum, pointer: *mut *mut c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut *mut c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(126)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(126) },
        };
        unsafe { __pfn(index, pname, pointer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetVertexAttribdv(&self, index: GLuint, pname: GLenum, params: *mut GLdouble) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLdouble)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(127)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(127) },
        };
        unsafe { __pfn(index, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetVertexAttribfv(&self, index: GLuint, pname: GLenum, params: *mut GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(128)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(128) },
        };
        unsafe { __pfn(index, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetVertexAttribiv(&self, index: GLuint, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(129)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(129) },
        };
        unsafe { __pfn(index, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn IsProgram(&self, program: GLuint) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLuint) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(130)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(130) },
        };
        unsafe { __pfn(program) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn IsShader(&self, shader: GLuint) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLuint) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(131)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(131) },
        };
        unsafe { __pfn(shader) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn LinkProgram(&self, program: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(132)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(132) },
        };
        unsafe { __pfn(program) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ShaderSource(&self, shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLsizei, *const *const GLchar, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(133)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(133) },
        };
        unsafe { __pfn(shader, count, string, length) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn StencilFuncSeparate(&self, face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(134)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(134) },
        };
        unsafe { __pfn(face, func, ref_, mask) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn StencilMaskSeparate(&self, face: GLenum, mask: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(135)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(135) },
        };
        unsafe { __pfn(face, mask) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn StencilOpSeparate(&self, face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLenum, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(136)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(136) },
        };
        unsafe { __pfn(face, sfail, dpfail, dppass) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform1f(&self, location: GLint, v0: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(137)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(137) },
        };
        unsafe { __pfn(location, v0) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform1fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(138)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(138) },
        };
        unsafe { __pfn(location, count, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform1i(&self, location: GLint, v0: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(139)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(139) },
        };
        unsafe { __pfn(location, v0) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform1iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(140)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(140) },
        };
        unsafe { __pfn(location, count, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform2f(&self, location: GLint, v0: GLfloat, v1: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLfloat, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(141)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(141) },
        };
        unsafe { __pfn(location, v0, v1) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform2fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(142)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(142) },
        };
        unsafe { __pfn(location, count, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform2i(&self, location: GLint, v0: GLint, v1: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(143)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(143) },
        };
        unsafe { __pfn(location, v0, v1) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform2iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(144)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(144) },
        };
        unsafe { __pfn(location, count, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform3f(&self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLfloat, GLfloat, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(145)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(145) },
        };
        unsafe { __pfn(location, v0, v1, v2) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform3fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(146)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(146) },
        };
        unsafe { __pfn(location, count, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform3i(&self, location: GLint, v0: GLint, v1: GLint, v2: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLint, GLint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(147)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(147) },
        };
        unsafe { __pfn(location, v0, v1, v2) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform3iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(148)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(148) },
        };
        unsafe { __pfn(location, count, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform4f(&self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLfloat, GLfloat, GLfloat, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(149)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(149) },
        };
        unsafe { __pfn(location, v0, v1, v2, v3) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform4fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(150)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(150) },
        };
        unsafe { __pfn(location, count, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform4i(&self, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLint, GLint, GLint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(151)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(151) },
        };
        unsafe { __pfn(location, v0, v1, v2, v3) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform4iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(152)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(152) },
        };
        unsafe { __pfn(location, count, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn UniformMatrix2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(153)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(153) },
        };
        unsafe { __pfn(location, count, transpose, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn UniformMatrix3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(154)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(154) },
        };
        unsafe { __pfn(location, count, transpose, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn UniformMatrix4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(155)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(155) },
        };
        unsafe { __pfn(location, count, transpose, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn UseProgram(&self, program: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(156)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(156) },
        };
        unsafe { __pfn(program) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ValidateProgram(&self, program: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(157)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(157) },
        };
        unsafe { __pfn(program) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib1d(&self, index: GLuint, x: GLdouble) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLdouble)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(158)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(158) },
        };
        unsafe { __pfn(index, x) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib1dv(&self, index: GLuint, v: *const GLdouble) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLdouble)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(159)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(159) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib1f(&self, index: GLuint, x: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(160)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(160) },
        };
        unsafe { __pfn(index, x) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib1fv(&self, index: GLuint, v: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(161)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(161) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib1s(&self, index: GLuint, x: GLshort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLshort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(162)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(162) },
        };
        unsafe { __pfn(index, x) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib1sv(&self, index: GLuint, v: *const GLshort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLshort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(163)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(163) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib2d(&self, index: GLuint, x: GLdouble, y: GLdouble) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLdouble, GLdouble)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(164)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(164) },
        };
        unsafe { __pfn(index, x, y) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib2dv(&self, index: GLuint, v: *const GLdouble) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLdouble)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(165)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(165) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib2f(&self, index: GLuint, x: GLfloat, y: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLfloat, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(166)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(166) },
        };
        unsafe { __pfn(index, x, y) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib2fv(&self, index: GLuint, v: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(167)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(167) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib2s(&self, index: GLuint, x: GLshort, y: GLshort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLshort, GLshort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(168)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(168) },
        };
        unsafe { __pfn(index, x, y) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib2sv(&self, index: GLuint, v: *const GLshort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLshort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(169)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(169) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib3d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(170)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(170) },
        };
        unsafe { __pfn(index, x, y, z) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib3dv(&self, index: GLuint, v: *const GLdouble) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLdouble)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(171)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(171) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib3f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(172)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(172) },
        };
        unsafe { __pfn(index, x, y, z) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib3fv(&self, index: GLuint, v: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(173)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(173) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib3s(&self, index: GLuint, x: GLshort, y: GLshort, z: GLshort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLshort, GLshort, GLshort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(174)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(174) },
        };
        unsafe { __pfn(index, x, y, z) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib3sv(&self, index: GLuint, v: *const GLshort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLshort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(175)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(175) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4Nbv(&self, index: GLuint, v: *const GLbyte) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLbyte)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(176)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(176) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4Niv(&self, index: GLuint, v: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(177)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(177) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4Nsv(&self, index: GLuint, v: *const GLshort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLshort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(178)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(178) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4Nub(&self, index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLubyte, GLubyte, GLubyte, GLubyte)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(179)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(179) },
        };
        unsafe { __pfn(index, x, y, z, w) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4Nubv(&self, index: GLuint, v: *const GLubyte) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLubyte)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(180)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(180) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4Nuiv(&self, index: GLuint, v: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(181)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(181) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4Nusv(&self, index: GLuint, v: *const GLushort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLushort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(182)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(182) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4bv(&self, index: GLuint, v: *const GLbyte) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLbyte)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(183)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(183) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble, GLdouble)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(184)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(184) },
        };
        unsafe { __pfn(index, x, y, z, w) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4dv(&self, index: GLuint, v: *const GLdouble) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLdouble)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(185)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(185) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(186)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(186) },
        };
        unsafe { __pfn(index, x, y, z, w) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4fv(&self, index: GLuint, v: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(187)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(187) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4iv(&self, index: GLuint, v: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(188)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(188) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4s(&self, index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLshort, GLshort, GLshort, GLshort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(189)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(189) },
        };
        unsafe { __pfn(index, x, y, z, w) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4sv(&self, index: GLuint, v: *const GLshort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLshort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(190)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(190) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4ubv(&self, index: GLuint, v: *const GLubyte) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLubyte)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(191)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(191) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4uiv(&self, index: GLuint, v: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(192)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(192) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4usv(&self, index: GLuint, v: *const GLushort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLushort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(193)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(193) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribPointer(&self, index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLsizei, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(194)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(194) },
        };
        unsafe { __pfn(index, size, type_, normalized, stride, pointer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn UniformMatrix2x3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(195)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(195) },
        };
        unsafe { __pfn(location, count, transpose, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn UniformMatrix2x4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(196)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(196) },
        };
        unsafe { __pfn(location, count, transpose, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn UniformMatrix3x2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(197)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(197) },
        };
        unsafe { __pfn(location, count, transpose, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn UniformMatrix3x4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(198)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(198) },
        };
        unsafe { __pfn(location, count, transpose, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn UniformMatrix4x2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(199)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(199) },
        };
        unsafe { __pfn(location, count, transpose, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn UniformMatrix4x3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(200)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(200) },
        };
        unsafe { __pfn(location, count, transpose, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BeginConditionalRender(&self, id: GLuint, mode: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(201)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(201) },
        };
        unsafe { __pfn(id, mode) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BeginTransformFeedback(&self, primitiveMode: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(202)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(202) },
        };
        unsafe { __pfn(primitiveMode) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BindFragDataLocation(&self, program: GLuint, color: GLuint, name: *const GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint, *const GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(203)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(203) },
        };
        unsafe { __pfn(program, color, name) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BindFramebuffer(&self, target: GLenum, framebuffer: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(204)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(204) },
        };
        unsafe { __pfn(target, framebuffer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BindRenderbuffer(&self, target: GLenum, renderbuffer: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(205)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(205) },
        };
        unsafe { __pfn(target, renderbuffer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BindVertexArray(&self, array: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(206)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(206) },
        };
        unsafe { __pfn(array) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BlitFramebuffer(&self, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLbitfield, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(207)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(207) },
        };
        unsafe { __pfn(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CheckFramebufferStatus(&self, target: GLenum) -> GLenum {
        let __pfn: Option<unsafe extern "system" fn(GLenum) -> GLenum> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(208)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(208) },
        };
        unsafe { __pfn(target) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ClampColor(&self, target: GLenum, clamp: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(209)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(209) },
        };
        unsafe { __pfn(target, clamp) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ClearBufferfi(&self, buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLfloat, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(210)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(210) },
        };
        unsafe { __pfn(buffer, drawbuffer, depth, stencil) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ClearBufferfv(&self, buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(211)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(211) },
        };
        unsafe { __pfn(buffer, drawbuffer, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ClearBufferiv(&self, buffer: GLenum, drawbuffer: GLint, value: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(212)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(212) },
        };
        unsafe { __pfn(buffer, drawbuffer, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ClearBufferuiv(&self, buffer: GLenum, drawbuffer: GLint, value: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(213)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(213) },
        };
        unsafe { __pfn(buffer, drawbuffer, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ColorMaski(&self, index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLboolean, GLboolean, GLboolean, GLboolean)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(214)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(214) },
        };
        unsafe { __pfn(index, r, g, b, a) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DeleteFramebuffers(&self, n: GLsizei, framebuffers: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(215)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(215) },
        };
        unsafe { __pfn(n, framebuffers) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DeleteRenderbuffers(&self, n: GLsizei, renderbuffers: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(216)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(216) },
        };
        unsafe { __pfn(n, renderbuffers) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DeleteVertexArrays(&self, n: GLsizei, arrays: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(217)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(217) },
        };
        unsafe { __pfn(n, arrays) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Disablei(&self, target: GLenum, index: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(218)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(218) },
        };
        unsafe { __pfn(target, index) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Enablei(&self, target: GLenum, index: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(219)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(219) },
        };
        unsafe { __pfn(target, index) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn EndConditionalRender(&self) {
        let __pfn: Option<unsafe extern "system" fn()> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(220)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(220) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn EndTransformFeedback(&self) {
        let __pfn: Option<unsafe extern "system" fn()> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(221)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(221) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn FlushMappedBufferRange(&self, target: GLenum, offset: GLintptr, length: GLsizeiptr) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLintptr, GLsizeiptr)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(222)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(222) },
        };
        unsafe { __pfn(target, offset, length) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn FramebufferRenderbuffer(&self, target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLenum, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(223)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(223) },
        };
        unsafe { __pfn(target, attachment, renderbuffertarget, renderbuffer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn FramebufferTexture1D(&self, target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(224)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(224) },
        };
        unsafe { __pfn(target, attachment, textarget, texture, level) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn FramebufferTexture2D(&self, target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(225)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(225) },
        };
        unsafe { __pfn(target, attachment, textarget, texture, level) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn FramebufferTexture3D(&self, target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(226)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(226) },
        };
        unsafe { __pfn(target, attachment, textarget, texture, level, zoffset) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn FramebufferTextureLayer(&self, target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLuint, GLint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(227)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(227) },
        };
        unsafe { __pfn(target, attachment, texture, level, layer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GenFramebuffers(&self, n: GLsizei, framebuffers: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(228)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(228) },
        };
        unsafe { __pfn(n, framebuffers) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GenRenderbuffers(&self, n: GLsizei, renderbuffers: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(229)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(229) },
        };
        unsafe { __pfn(n, renderbuffers) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GenVertexArrays(&self, n: GLsizei, arrays: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(230)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(230) },
        };
        unsafe { __pfn(n, arrays) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GenerateMipmap(&self, target: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(231)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(231) },
        };
        unsafe { __pfn(target) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetBooleani_v(&self, target: GLenum, index: GLuint, data: *mut GLboolean) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint, *mut GLboolean)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(232)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(232) },
        };
        unsafe { __pfn(target, index, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetFragDataLocation(&self, program: GLuint, name: *const GLchar) -> GLint {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLchar) -> GLint> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(233)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(233) },
        };
        unsafe { __pfn(program, name) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetFramebufferAttachmentParameteriv(&self, target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(234)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(234) },
        };
        unsafe { __pfn(target, attachment, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetRenderbufferParameteriv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(235)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(235) },
        };
        unsafe { __pfn(target, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetStringi(&self, name: GLenum, index: GLuint) -> *const GLubyte {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint) -> *const GLubyte> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(236)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(236) },
        };
        unsafe { __pfn(name, index) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetTexParameterIiv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(237)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(237) },
        };
        unsafe { __pfn(target, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetTexParameterIuiv(&self, target: GLenum, pname: GLenum, params: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(238)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(238) },
        };
        unsafe { __pfn(target, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetTransformFeedbackVarying(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLsizei, *mut GLenum, *mut GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(239)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(239) },
        };
        unsafe { __pfn(program, index, bufSize, length, size, type_, name) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetUniformuiv(&self, program: GLuint, location: GLint, params: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLint, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(240)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(240) },
        };
        unsafe { __pfn(program, location, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetVertexAttribIiv(&self, index: GLuint, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(241)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(241) },
        };
        unsafe { __pfn(index, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetVertexAttribIuiv(&self, index: GLuint, pname: GLenum, params: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(242)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(242) },
        };
        unsafe { __pfn(index, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn IsEnabledi(&self, target: GLenum, index: GLuint) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(243)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(243) },
        };
        unsafe { __pfn(target, index) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn IsFramebuffer(&self, framebuffer: GLuint) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLuint) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(244)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(244) },
        };
        unsafe { __pfn(framebuffer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn IsRenderbuffer(&self, renderbuffer: GLuint) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLuint) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(245)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(245) },
        };
        unsafe { __pfn(renderbuffer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn IsVertexArray(&self, array: GLuint) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLuint) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(246)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(246) },
        };
        unsafe { __pfn(array) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn MapBufferRange(&self, target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut c_void {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLintptr, GLsizeiptr, GLbitfield) -> *mut c_void> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(247)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(247) },
        };
        unsafe { __pfn(target, offset, length, access) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn RenderbufferStorage(&self, target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLsizei, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(248)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(248) },
        };
        unsafe { __pfn(target, internalformat, width, height) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn RenderbufferStorageMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(249)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(249) },
        };
        unsafe { __pfn(target, samples, internalformat, width, height) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexParameterIiv(&self, target: GLenum, pname: GLenum, params: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(250)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(250) },
        };
        unsafe { __pfn(target, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexParameterIuiv(&self, target: GLenum, pname: GLenum, params: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(251)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(251) },
        };
        unsafe { __pfn(target, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TransformFeedbackVaryings(&self, program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLsizei, *const *const GLchar, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(252)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(252) },
        };
        unsafe { __pfn(program, count, varyings, bufferMode) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform1ui(&self, location: GLint, v0: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(253)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(253) },
        };
        unsafe { __pfn(location, v0) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform1uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(254)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(254) },
        };
        unsafe { __pfn(location, count, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform2ui(&self, location: GLint, v0: GLuint, v1: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLuint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(255)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(255) },
        };
        unsafe { __pfn(location, v0, v1) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform2uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(256)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(256) },
        };
        unsafe { __pfn(location, count, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform3ui(&self, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLuint, GLuint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(257)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(257) },
        };
        unsafe { __pfn(location, v0, v1, v2) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform3uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(258)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(258) },
        };
        unsafe { __pfn(location, count, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform4ui(&self, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLuint, GLuint, GLuint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(259)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(259) },
        };
        unsafe { __pfn(location, v0, v1, v2, v3) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform4uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(260)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(260) },
        };
        unsafe { __pfn(location, count, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI1i(&self, index: GLuint, x: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(261)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(261) },
        };
        unsafe { __pfn(index, x) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI1iv(&self, index: GLuint, v: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(262)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(262) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI1ui(&self, index: GLuint, x: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(263)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(263) },
        };
        unsafe { __pfn(index, x) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI1uiv(&self, index: GLuint, v: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(264)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(264) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI2i(&self, index: GLuint, x: GLint, y: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(265)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(265) },
        };
        unsafe { __pfn(index, x, y) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI2iv(&self, index: GLuint, v: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(266)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(266) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI2ui(&self, index: GLuint, x: GLuint, y: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(267)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(267) },
        };
        unsafe { __pfn(index, x, y) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI2uiv(&self, index: GLuint, v: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(268)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(268) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI3i(&self, index: GLuint, x: GLint, y: GLint, z: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLint, GLint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(269)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(269) },
        };
        unsafe { __pfn(index, x, y, z) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI3iv(&self, index: GLuint, v: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(270)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(270) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI3ui(&self, index: GLuint, x: GLuint, y: GLuint, z: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint, GLuint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(271)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(271) },
        };
        unsafe { __pfn(index, x, y, z) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI3uiv(&self, index: GLuint, v: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(272)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(272) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI4bv(&self, index: GLuint, v: *const GLbyte) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLbyte)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(273)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(273) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI4i(&self, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLint, GLint, GLint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(274)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(274) },
        };
        unsafe { __pfn(index, x, y, z, w) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI4iv(&self, index: GLuint, v: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(275)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(275) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI4sv(&self, index: GLuint, v: *const GLshort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLshort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(276)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(276) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI4ubv(&self, index: GLuint, v: *const GLubyte) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLubyte)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(277)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(277) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI4ui(&self, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint, GLuint, GLuint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(278)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(278) },
        };
        unsafe { __pfn(index, x, y, z, w) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI4uiv(&self, index: GLuint, v: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(279)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(279) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI4usv(&self, index: GLuint, v: *const GLushort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLushort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(280)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(280) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribIPointer(&self, index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLint, GLenum, GLsizei, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(281)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(281) },
        };
        unsafe { __pfn(index, size, type_, stride, pointer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BindBufferBase(&self, target: GLenum, index: GLuint, buffer: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(282)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(282) },
        };
        unsafe { __pfn(target, index, buffer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BindBufferRange(&self, target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint, GLuint, GLintptr, GLsizeiptr)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(283)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(283) },
        };
        unsafe { __pfn(target, index, buffer, offset, size) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetIntegeri_v(&self, target: GLenum, index: GLuint, data: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(284)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(284) },
        };
        unsafe { __pfn(target, index, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CopyBufferSubData(&self, readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLintptr, GLintptr, GLsizeiptr)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(285)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(285) },
        };
        unsafe { __pfn(readTarget, writeTarget, readOffset, writeOffset, size) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DrawArraysInstanced(&self, mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLsizei, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(286)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(286) },
        };
        unsafe { __pfn(mode, first, count, instancecount) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DrawElementsInstanced(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(287)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(287) },
        };
        unsafe { __pfn(mode, count, type_, indices, instancecount) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetActiveUniformBlockName(&self, program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(288)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(288) },
        };
        unsafe { __pfn(program, uniformBlockIndex, bufSize, length, uniformBlockName) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetActiveUniformBlockiv(&self, program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(289)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(289) },
        };
        unsafe { __pfn(program, uniformBlockIndex, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetActiveUniformName(&self, program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(290)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(290) },
        };
        unsafe { __pfn(program, uniformIndex, bufSize, length, uniformName) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetActiveUniformsiv(&self, program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLsizei, *const GLuint, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(291)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(291) },
        };
        unsafe { __pfn(program, uniformCount, uniformIndices, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetUniformBlockIndex(&self, program: GLuint, uniformBlockName: *const GLchar) -> GLuint {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLchar) -> GLuint> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(292)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(292) },
        };
        unsafe { __pfn(program, uniformBlockName) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetUniformIndices(&self, program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLsizei, *const *const GLchar, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(293)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(293) },
        };
        unsafe { __pfn(program, uniformCount, uniformNames, uniformIndices) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn PrimitiveRestartIndex(&self, index: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(294)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(294) },
        };
        unsafe { __pfn(index) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexBuffer(&self, target: GLenum, internalformat: GLenum, buffer: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(295)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(295) },
        };
        unsafe { __pfn(target, internalformat, buffer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn UniformBlockBinding(&self, program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(296)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(296) },
        };
        unsafe { __pfn(program, uniformBlockIndex, uniformBlockBinding) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ClientWaitSync(&self, sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum {
        let __pfn: Option<unsafe extern "system" fn(GLsync, GLbitfield, GLuint64) -> GLenum> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(297)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(297) },
        };
        unsafe { __pfn(sync, flags, timeout) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DeleteSync(&self, sync: GLsync) {
        let __pfn: Option<unsafe extern "system" fn(GLsync)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(298)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(298) },
        };
        unsafe { __pfn(sync) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DrawElementsBaseVertex(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, basevertex: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(299)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(299) },
        };
        unsafe { __pfn(mode, count, type_, indices, basevertex) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DrawElementsInstancedBaseVertex(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, basevertex: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(300)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(300) },
        };
        unsafe { __pfn(mode, count, type_, indices, instancecount, basevertex) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DrawRangeElementsBaseVertex(&self, mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const c_void, basevertex: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint, GLuint, GLsizei, GLenum, *const c_void, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(301)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(301) },
        };
        unsafe { __pfn(mode, start, end, count, type_, indices, basevertex) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn FenceSync(&self, condition: GLenum, flags: GLbitfield) -> GLsync {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLbitfield) -> GLsync> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(302)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(302) },
        };
        unsafe { __pfn(condition, flags) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn FramebufferTexture(&self, target: GLenum, attachment: GLenum, texture: GLuint, level: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLuint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(303)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(303) },
        };
        unsafe { __pfn(target, attachment, texture, level) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetBufferParameteri64v(&self, target: GLenum, pname: GLenum, params: *mut GLint64) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *mut GLint64)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(304)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(304) },
        };
        unsafe { __pfn(target, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetInteger64i_v(&self, target: GLenum, index: GLuint, data: *mut GLint64) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint, *mut GLint64)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(305)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(305) },
        };
        unsafe { __pfn(target, index, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetInteger64v(&self, pname: GLenum, data: *mut GLint64) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, *mut GLint64)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(306)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(306) },
        };
        unsafe { __pfn(pname, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetMultisamplefv(&self, pname: GLenum, index: GLuint, val: *mut GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint, *mut GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(307)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(307) },
        };
        unsafe { __pfn(pname, index, val) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetSynciv(&self, sync: GLsync, pname: GLenum, count: GLsizei, length: *mut GLsizei, values: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLsync, GLenum, GLsizei, *mut GLsizei, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(308)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(308) },
        };
        unsafe { __pfn(sync, pname, count, length, values) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn IsSync(&self, sync: GLsync) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLsync) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(309)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(309) },
        };
        unsafe { __pfn(sync) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn MultiDrawElementsBaseVertex(&self, mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, drawcount: GLsizei, basevertex: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, *const GLsizei, GLenum, *const *const c_void, GLsizei, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(310)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(310) },
        };
        unsafe { __pfn(mode, count, type_, indices, drawcount, basevertex) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ProvokingVertex(&self, mode: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(311)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(311) },
        };
        unsafe { __pfn(mode) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn SampleMaski(&self, maskNumber: GLuint, mask: GLbitfield) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLbitfield)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(312)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(312) },
        };
        unsafe { __pfn(maskNumber, mask) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexImage2DMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLboolean)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(313)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(313) },
        };
        unsafe { __pfn(target, samples, internalformat, width, height, fixedsamplelocations) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexImage3DMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(314)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(314) },
        };
        unsafe { __pfn(target, samples, internalformat, width, height, depth, fixedsamplelocations) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn WaitSync(&self, sync: GLsync, flags: GLbitfield, timeout: GLuint64) {
        let __pfn: Option<unsafe extern "system" fn(GLsync, GLbitfield, GLuint64)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(315)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(315) },
        };
        unsafe { __pfn(sync, flags, timeout) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BindFragDataLocationIndexed(&self, program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint, GLuint, *const GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(316)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(316) },
        };
        unsafe { __pfn(program, colorNumber, index, name) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BindSampler(&self, unit: GLuint, sampler: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(317)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(317) },
        };
        unsafe { __pfn(unit, sampler) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DeleteSamplers(&self, count: GLsizei, samplers: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(318)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(318) },
        };
        unsafe { __pfn(count, samplers) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GenSamplers(&self, count: GLsizei, samplers: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(319)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(319) },
        };
        unsafe { __pfn(count, samplers) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetFragDataIndex(&self, program: GLuint, name: *const GLchar) -> GLint {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLchar) -> GLint> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(320)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(320) },
        };
        unsafe { __pfn(program, name) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetQueryObjecti64v(&self, id: GLuint, pname: GLenum, params: *mut GLint64) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLint64)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(321)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(321) },
        };
        unsafe { __pfn(id, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetQueryObjectui64v(&self, id: GLuint, pname: GLenum, params: *mut GLuint64) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLuint64)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(322)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(322) },
        };
        unsafe { __pfn(id, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetSamplerParameterIiv(&self, sampler: GLuint, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(323)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(323) },
        };
        unsafe { __pfn(sampler, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetSamplerParameterIuiv(&self, sampler: GLuint, pname: GLenum, params: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(324)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(324) },
        };
        unsafe { __pfn(sampler, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetSamplerParameterfv(&self, sampler: GLuint, pname: GLenum, params: *mut GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(325)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(325) },
        };
        unsafe { __pfn(sampler, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetSamplerParameteriv(&self, sampler: GLuint, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(326)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(326) },
        };
        unsafe { __pfn(sampler, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn IsSampler(&self, sampler: GLuint) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLuint) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(327)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(327) },
        };
        unsafe { __pfn(sampler) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn QueryCounter(&self, id: GLuint, target: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(328)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(328) },
        };
        unsafe { __pfn(id, target) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn SamplerParameterIiv(&self, sampler: GLuint, pname: GLenum, param: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(329)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(329) },
        };
        unsafe { __pfn(sampler, pname, param) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn SamplerParameterIuiv(&self, sampler: GLuint, pname: GLenum, param: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(330)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(330) },
        };
        unsafe { __pfn(sampler, pname, param) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn SamplerParameterf(&self, sampler: GLuint, pname: GLenum, param: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(331)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(331) },
        };
        unsafe { __pfn(sampler, pname, param) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn SamplerParameterfv(&self, sampler: GLuint, pname: GLenum, param: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(332)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(332) },
        };
        unsafe { __pfn(sampler, pname, param) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn SamplerParameteri(&self, sampler: GLuint, pname: GLenum, param: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(333)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(333) },
        };
        unsafe { __pfn(sampler, pname, param) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn SamplerParameteriv(&self, sampler: GLuint, pname: GLenum, param: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(334)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(334) },
        };
        unsafe { __pfn(sampler, pname, param) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribDivisor(&self, index: GLuint, divisor: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(335)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(335) },
        };
        unsafe { __pfn(index, divisor) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribP1ui(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, GLboolean, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(336)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(336) },
        };
        unsafe { __pfn(index, type_, normalized, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribP1uiv(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(337)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(337) },
        };
        unsafe { __pfn(index, type_, normalized, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribP2ui(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, GLboolean, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(338)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(338) },
        };
        unsafe { __pfn(index, type_, normalized, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribP2uiv(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(339)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(339) },
        };
        unsafe { __pfn(index, type_, normalized, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribP3ui(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, GLboolean, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(340)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(340) },
        };
        unsafe { __pfn(index, type_, normalized, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribP3uiv(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(341)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(341) },
        };
        unsafe { __pfn(index, type_, normalized, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribP4ui(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, GLboolean, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(342)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(342) },
        };
        unsafe { __pfn(index, type_, normalized, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribP4uiv(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(343)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(343) },
        };
        unsafe { __pfn(index, type_, normalized, value) }
    }

    /// Whether the driver supports `GL_VERSION_1_0`.
    #[inline]
    pub fn VERSION_1_0(&self) -> bool {
        self.feat[0]
    }

    /// Whether the driver supports `GL_VERSION_1_1`.
    #[inline]
    pub fn VERSION_1_1(&self) -> bool {
        self.feat[1]
    }

    /// Whether the driver supports `GL_VERSION_1_2`.
    #[inline]
    pub fn VERSION_1_2(&self) -> bool {
        self.feat[2]
    }

    /// Whether the driver supports `GL_VERSION_1_3`.
    #[inline]
    pub fn VERSION_1_3(&self) -> bool {
        self.feat[3]
    }

    /// Whether the driver supports `GL_VERSION_1_4`.
    #[inline]
    pub fn VERSION_1_4(&self) -> bool {
        self.feat[4]
    }

    /// Whether the driver supports `GL_VERSION_1_5`.
    #[inline]
    pub fn VERSION_1_5(&self) -> bool {
        self.feat[5]
    }

    /// Whether the driver supports `GL_VERSION_2_0`.
    #[inline]
    pub fn VERSION_2_0(&self) -> bool {
        self.feat[6]
    }

    /// Whether the driver supports `GL_VERSION_2_1`.
    #[inline]
    pub fn VERSION_2_1(&self) -> bool {
        self.feat[7]
    }

    /// Whether the driver supports `GL_VERSION_3_0`.
    #[inline]
    pub fn VERSION_3_0(&self) -> bool {
        self.feat[8]
    }

    /// Whether the driver supports `GL_VERSION_3_1`.
    #[inline]
    pub fn VERSION_3_1(&self) -> bool {
        self.feat[9]
    }

    /// Whether the driver supports `GL_VERSION_3_2`.
    #[inline]
    pub fn VERSION_3_2(&self) -> bool {
        self.feat[10]
    }

    /// Whether the driver supports `GL_VERSION_3_3`.
    #[inline]
    pub fn VERSION_3_3(&self) -> bool {
        self.feat[11]
    }
}
