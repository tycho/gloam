#![no_std]
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, clippy::all)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::ffi::{CStr, c_char, c_void};

// ── GL base types ───────────────────────────────────────────
pub type GLvoid = c_void;
pub type GLboolean = u8;
pub type GLbyte = i8;
pub type GLubyte = u8;
pub type GLchar = c_char;
pub type GLcharARB = c_char;
pub type GLshort = i16;
pub type GLushort = u16;
pub type GLint = i32;
pub type GLuint = u32;
pub type GLfixed = i32;
pub type GLint64 = i64;
pub type GLuint64 = u64;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;
pub type GLintptr = isize;
pub type GLsizeiptr = isize;
pub type GLintptrARB = isize;
pub type GLsizeiptrARB = isize;
pub type GLsizei = i32;
pub type GLfloat = f32;
pub type GLclampf = f32;
pub type GLdouble = f64;
pub type GLclampd = f64;
pub type GLhalf = u16;
pub type GLhalfARB = u16;
pub type GLhalfNV = u16;
pub type GLhandleARB = u32;
pub type GLvdpauSurfaceNV = GLintptr;

// Opaque handle types.  `GLsync` is a pointer to an incomplete struct in C,
// so it gets its own zero-sized opaque type to keep the pointer distinct
// (a GLsync is not interchangeable with any other pointer, as in C).
// `GLeglImageOES`/`GLeglClientBufferEXT` are literally `void *` in the spec.
#[repr(C)]
pub struct __GLsync {
    _opaque: [u8; 0],
}
pub type GLsync = *mut __GLsync;
pub type GLeglImageOES = *mut c_void;
pub type GLeglClientBufferEXT = *mut c_void;
#[repr(C)]
pub struct _cl_context {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct _cl_event {
    _opaque: [u8; 0],
}

// Callback function-pointer types.
pub type GLDEBUGPROC =
    Option<unsafe extern "system" fn(GLenum, GLenum, GLuint, GLenum, GLsizei, *const GLchar, *const c_void)>;
pub type GLDEBUGPROCARB = GLDEBUGPROC;
pub type GLDEBUGPROCKHR = GLDEBUGPROC;
pub type GLDEBUGPROCAMD =
    Option<unsafe extern "system" fn(GLuint, GLenum, GLenum, GLsizei, *const GLchar, *mut c_void)>;
pub type GLVULKANPROCNV = Option<unsafe extern "system" fn()>;
pub type GLSETBLOBPROCANGLE =
    Option<unsafe extern "system" fn(*const c_void, GLsizeiptr, *const c_void, GLsizeiptr, *const c_void)>;
pub type GLGETBLOBPROCANGLE = Option<
    unsafe extern "system" fn(*const c_void, GLsizeiptr, *mut c_void, GLsizeiptr, *const c_void) -> GLsizeiptr,
>;

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
pub const GL_CONTEXT_FLAG_DEBUG_BIT: GLbitfield = GLbitfield(0x00000002);
pub const GL_CONTEXT_FLAG_DEBUG_BIT_KHR: GLbitfield = GLbitfield(0x00000002);
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
pub const GL_STACK_OVERFLOW: GLenum = GLenum(0x0503);
pub const GL_STACK_OVERFLOW_KHR: GLenum = GLenum(0x0503);
pub const GL_STACK_UNDERFLOW: GLenum = GLenum(0x0504);
pub const GL_STACK_UNDERFLOW_KHR: GLenum = GLenum(0x0504);
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
pub const GL_FIXED: GLenum = GLenum(0x140C);
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
pub const GL_VERTEX_ARRAY: GLenum = GLenum(0x8074);
pub const GL_VERTEX_ARRAY_KHR: GLenum = GLenum(0x8074);
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
pub const GL_DEBUG_OUTPUT_SYNCHRONOUS: GLenum = GLenum(0x8242);
pub const GL_DEBUG_OUTPUT_SYNCHRONOUS_KHR: GLenum = GLenum(0x8242);
pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: GLenum = GLenum(0x8243);
pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH_KHR: GLenum = GLenum(0x8243);
pub const GL_DEBUG_CALLBACK_FUNCTION: GLenum = GLenum(0x8244);
pub const GL_DEBUG_CALLBACK_FUNCTION_KHR: GLenum = GLenum(0x8244);
pub const GL_DEBUG_CALLBACK_USER_PARAM: GLenum = GLenum(0x8245);
pub const GL_DEBUG_CALLBACK_USER_PARAM_KHR: GLenum = GLenum(0x8245);
pub const GL_DEBUG_SOURCE_API: GLenum = GLenum(0x8246);
pub const GL_DEBUG_SOURCE_API_KHR: GLenum = GLenum(0x8246);
pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM: GLenum = GLenum(0x8247);
pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM_KHR: GLenum = GLenum(0x8247);
pub const GL_DEBUG_SOURCE_SHADER_COMPILER: GLenum = GLenum(0x8248);
pub const GL_DEBUG_SOURCE_SHADER_COMPILER_KHR: GLenum = GLenum(0x8248);
pub const GL_DEBUG_SOURCE_THIRD_PARTY: GLenum = GLenum(0x8249);
pub const GL_DEBUG_SOURCE_THIRD_PARTY_KHR: GLenum = GLenum(0x8249);
pub const GL_DEBUG_SOURCE_APPLICATION: GLenum = GLenum(0x824A);
pub const GL_DEBUG_SOURCE_APPLICATION_KHR: GLenum = GLenum(0x824A);
pub const GL_DEBUG_SOURCE_OTHER: GLenum = GLenum(0x824B);
pub const GL_DEBUG_SOURCE_OTHER_KHR: GLenum = GLenum(0x824B);
pub const GL_DEBUG_TYPE_ERROR: GLenum = GLenum(0x824C);
pub const GL_DEBUG_TYPE_ERROR_KHR: GLenum = GLenum(0x824C);
pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR: GLenum = GLenum(0x824D);
pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR_KHR: GLenum = GLenum(0x824D);
pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR: GLenum = GLenum(0x824E);
pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR_KHR: GLenum = GLenum(0x824E);
pub const GL_DEBUG_TYPE_PORTABILITY: GLenum = GLenum(0x824F);
pub const GL_DEBUG_TYPE_PORTABILITY_KHR: GLenum = GLenum(0x824F);
pub const GL_DEBUG_TYPE_PERFORMANCE: GLenum = GLenum(0x8250);
pub const GL_DEBUG_TYPE_PERFORMANCE_KHR: GLenum = GLenum(0x8250);
pub const GL_DEBUG_TYPE_OTHER: GLenum = GLenum(0x8251);
pub const GL_DEBUG_TYPE_OTHER_KHR: GLenum = GLenum(0x8251);
pub const GL_PROGRAM_BINARY_RETRIEVABLE_HINT: GLenum = GLenum(0x8257);
pub const GL_DEBUG_TYPE_MARKER: GLenum = GLenum(0x8268);
pub const GL_DEBUG_TYPE_MARKER_KHR: GLenum = GLenum(0x8268);
pub const GL_DEBUG_TYPE_PUSH_GROUP: GLenum = GLenum(0x8269);
pub const GL_DEBUG_TYPE_PUSH_GROUP_KHR: GLenum = GLenum(0x8269);
pub const GL_DEBUG_TYPE_POP_GROUP: GLenum = GLenum(0x826A);
pub const GL_DEBUG_TYPE_POP_GROUP_KHR: GLenum = GLenum(0x826A);
pub const GL_DEBUG_SEVERITY_NOTIFICATION: GLenum = GLenum(0x826B);
pub const GL_DEBUG_SEVERITY_NOTIFICATION_KHR: GLenum = GLenum(0x826B);
pub const GL_MAX_DEBUG_GROUP_STACK_DEPTH: GLenum = GLenum(0x826C);
pub const GL_MAX_DEBUG_GROUP_STACK_DEPTH_KHR: GLenum = GLenum(0x826C);
pub const GL_DEBUG_GROUP_STACK_DEPTH: GLenum = GLenum(0x826D);
pub const GL_DEBUG_GROUP_STACK_DEPTH_KHR: GLenum = GLenum(0x826D);
pub const GL_TEXTURE_IMMUTABLE_LEVELS: GLenum = GLenum(0x82DF);
pub const GL_BUFFER: GLenum = GLenum(0x82E0);
pub const GL_BUFFER_KHR: GLenum = GLenum(0x82E0);
pub const GL_SHADER: GLenum = GLenum(0x82E1);
pub const GL_SHADER_KHR: GLenum = GLenum(0x82E1);
pub const GL_PROGRAM: GLenum = GLenum(0x82E2);
pub const GL_PROGRAM_KHR: GLenum = GLenum(0x82E2);
pub const GL_QUERY: GLenum = GLenum(0x82E3);
pub const GL_QUERY_KHR: GLenum = GLenum(0x82E3);
pub const GL_PROGRAM_PIPELINE: GLenum = GLenum(0x82E4);
pub const GL_PROGRAM_PIPELINE_KHR: GLenum = GLenum(0x82E4);
pub const GL_SAMPLER: GLenum = GLenum(0x82E6);
pub const GL_SAMPLER_KHR: GLenum = GLenum(0x82E6);
pub const GL_DISPLAY_LIST: GLenum = GLenum(0x82E7);
pub const GL_MAX_LABEL_LENGTH: GLenum = GLenum(0x82E8);
pub const GL_MAX_LABEL_LENGTH_KHR: GLenum = GLenum(0x82E8);
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
pub const GL_PROGRAM_BINARY_LENGTH: GLenum = GLenum(0x8741);
pub const GL_BUFFER_SIZE: GLenum = GLenum(0x8764);
pub const GL_BUFFER_USAGE: GLenum = GLenum(0x8765);
pub const GL_NUM_PROGRAM_BINARY_FORMATS: GLenum = GLenum(0x87FE);
pub const GL_PROGRAM_BINARY_FORMATS: GLenum = GLenum(0x87FF);
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
pub const GL_IMPLEMENTATION_COLOR_READ_TYPE: GLenum = GLenum(0x8B9A);
pub const GL_IMPLEMENTATION_COLOR_READ_FORMAT: GLenum = GLenum(0x8B9B);
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
pub const GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS: GLenum = GLenum(0x8CD9);
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
pub const GL_RGB565: GLenum = GLenum(0x8D62);
pub const GL_PRIMITIVE_RESTART_FIXED_INDEX: GLenum = GLenum(0x8D69);
pub const GL_ANY_SAMPLES_PASSED_CONSERVATIVE: GLenum = GLenum(0x8D6A);
pub const GL_MAX_ELEMENT_INDEX: GLenum = GLenum(0x8D6B);
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
pub const GL_LOW_FLOAT: GLenum = GLenum(0x8DF0);
pub const GL_MEDIUM_FLOAT: GLenum = GLenum(0x8DF1);
pub const GL_HIGH_FLOAT: GLenum = GLenum(0x8DF2);
pub const GL_LOW_INT: GLenum = GLenum(0x8DF3);
pub const GL_MEDIUM_INT: GLenum = GLenum(0x8DF4);
pub const GL_HIGH_INT: GLenum = GLenum(0x8DF5);
pub const GL_SHADER_BINARY_FORMATS: GLenum = GLenum(0x8DF8);
pub const GL_NUM_SHADER_BINARY_FORMATS: GLenum = GLenum(0x8DF9);
pub const GL_SHADER_COMPILER: GLenum = GLenum(0x8DFA);
pub const GL_MAX_VERTEX_UNIFORM_VECTORS: GLenum = GLenum(0x8DFB);
pub const GL_MAX_VARYING_VECTORS: GLenum = GLenum(0x8DFC);
pub const GL_MAX_FRAGMENT_UNIFORM_VECTORS: GLenum = GLenum(0x8DFD);
pub const GL_QUERY_WAIT: GLenum = GLenum(0x8E13);
pub const GL_QUERY_NO_WAIT: GLenum = GLenum(0x8E14);
pub const GL_QUERY_BY_REGION_WAIT: GLenum = GLenum(0x8E15);
pub const GL_QUERY_BY_REGION_NO_WAIT: GLenum = GLenum(0x8E16);
pub const GL_TRANSFORM_FEEDBACK: GLenum = GLenum(0x8E22);
pub const GL_TRANSFORM_FEEDBACK_PAUSED: GLenum = GLenum(0x8E23);
pub const GL_TRANSFORM_FEEDBACK_ACTIVE: GLenum = GLenum(0x8E24);
pub const GL_TRANSFORM_FEEDBACK_BINDING: GLenum = GLenum(0x8E25);
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
pub const GL_COPY_READ_BUFFER_BINDING: GLenum = GLenum(0x8F36);
pub const GL_COPY_WRITE_BUFFER: GLenum = GLenum(0x8F37);
pub const GL_COPY_WRITE_BUFFER_BINDING: GLenum = GLenum(0x8F37);
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
pub const GL_TEXTURE_IMMUTABLE_FORMAT: GLenum = GLenum(0x912F);
pub const GL_MAX_DEBUG_MESSAGE_LENGTH: GLenum = GLenum(0x9143);
pub const GL_MAX_DEBUG_MESSAGE_LENGTH_KHR: GLenum = GLenum(0x9143);
pub const GL_MAX_DEBUG_LOGGED_MESSAGES: GLenum = GLenum(0x9144);
pub const GL_MAX_DEBUG_LOGGED_MESSAGES_KHR: GLenum = GLenum(0x9144);
pub const GL_DEBUG_LOGGED_MESSAGES: GLenum = GLenum(0x9145);
pub const GL_DEBUG_LOGGED_MESSAGES_KHR: GLenum = GLenum(0x9145);
pub const GL_DEBUG_SEVERITY_HIGH: GLenum = GLenum(0x9146);
pub const GL_DEBUG_SEVERITY_HIGH_KHR: GLenum = GLenum(0x9146);
pub const GL_DEBUG_SEVERITY_MEDIUM: GLenum = GLenum(0x9147);
pub const GL_DEBUG_SEVERITY_MEDIUM_KHR: GLenum = GLenum(0x9147);
pub const GL_DEBUG_SEVERITY_LOW: GLenum = GLenum(0x9148);
pub const GL_DEBUG_SEVERITY_LOW_KHR: GLenum = GLenum(0x9148);
pub const GL_COMPRESSED_R11_EAC: GLenum = GLenum(0x9270);
pub const GL_COMPRESSED_SIGNED_R11_EAC: GLenum = GLenum(0x9271);
pub const GL_COMPRESSED_RG11_EAC: GLenum = GLenum(0x9272);
pub const GL_COMPRESSED_SIGNED_RG11_EAC: GLenum = GLenum(0x9273);
pub const GL_COMPRESSED_RGB8_ETC2: GLenum = GLenum(0x9274);
pub const GL_COMPRESSED_SRGB8_ETC2: GLenum = GLenum(0x9275);
pub const GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = GLenum(0x9276);
pub const GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = GLenum(0x9277);
pub const GL_COMPRESSED_RGBA8_ETC2_EAC: GLenum = GLenum(0x9278);
pub const GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: GLenum = GLenum(0x9279);
pub const GL_DEBUG_OUTPUT: GLenum = GLenum(0x92E0);
pub const GL_DEBUG_OUTPUT_KHR: GLenum = GLenum(0x92E0);
pub const GL_NUM_SAMPLE_COUNTS: GLenum = GLenum(0x9380);

// ── Command table ───────────────────────────────────────────
pub const COMMAND_COUNT: usize = 385;
pub const FEATURE_COUNT: usize = 14;

#[rustfmt::skip]
static FN_NAME_DATA: &[u8] = b"\
    glClearDepth\0\
    glDepthRange\0\
    glDrawBuffer\0\
    glGetDoublev\0\
    glGetTexImage\0\
    glGetTexLevelParameterfv\0\
    glGetTexLevelParameteriv\0\
    glLogicOp\0\
    glPixelStoref\0\
    glPointSize\0\
    glPolygonMode\0\
    glTexImage1D\0\
    glBlendFunc\0\
    glClear\0\
    glClearColor\0\
    glClearStencil\0\
    glColorMask\0\
    glCullFace\0\
    glDepthFunc\0\
    glDepthMask\0\
    glDisable\0\
    glEnable\0\
    glFinish\0\
    glFlush\0\
    glFrontFace\0\
    glGetBooleanv\0\
    glGetError\0\
    glGetFloatv\0\
    glGetIntegerv\0\
    glGetString\0\
    glGetTexParameterfv\0\
    glGetTexParameteriv\0\
    glHint\0\
    glIsEnabled\0\
    glLineWidth\0\
    glPixelStorei\0\
    glReadPixels\0\
    glScissor\0\
    glStencilFunc\0\
    glStencilMask\0\
    glStencilOp\0\
    glTexImage2D\0\
    glTexParameterf\0\
    glTexParameterfv\0\
    glTexParameteri\0\
    glTexParameteriv\0\
    glViewport\0\
    glReadBuffer\0\
    glCopyTexImage1D\0\
    glCopyTexSubImage1D\0\
    glTexSubImage1D\0\
    glBindTexture\0\
    glCopyTexImage2D\0\
    glCopyTexSubImage2D\0\
    glDeleteTextures\0\
    glDrawArrays\0\
    glDrawElements\0\
    glGenTextures\0\
    glIsTexture\0\
    glPolygonOffset\0\
    glTexSubImage2D\0\
    glCopyTexSubImage3D\0\
    glDrawRangeElements\0\
    glTexImage3D\0\
    glTexSubImage3D\0\
    glCompressedTexImage1D\0\
    glCompressedTexSubImage1D\0\
    glGetCompressedTexImage\0\
    glActiveTexture\0\
    glCompressedTexImage2D\0\
    glCompressedTexSubImage2D\0\
    glSampleCoverage\0\
    glCompressedTexImage3D\0\
    glCompressedTexSubImage3D\0\
    glMultiDrawArrays\0\
    glMultiDrawElements\0\
    glPointParameterf\0\
    glPointParameterfv\0\
    glPointParameteri\0\
    glPointParameteriv\0\
    glBlendColor\0\
    glBlendEquation\0\
    glBlendFuncSeparate\0\
    glGetBufferSubData\0\
    glGetQueryObjectiv\0\
    glMapBuffer\0\
    glBindBuffer\0\
    glBufferData\0\
    glBufferSubData\0\
    glDeleteBuffers\0\
    glGenBuffers\0\
    glGetBufferParameteriv\0\
    glIsBuffer\0\
    glBeginQuery\0\
    glDeleteQueries\0\
    glEndQuery\0\
    glGenQueries\0\
    glGetBufferPointerv\0\
    glGetQueryObjectuiv\0\
    glGetQueryiv\0\
    glIsQuery\0\
    glUnmapBuffer\0\
    glGetVertexAttribdv\0\
    glVertexAttrib1d\0\
    glVertexAttrib1dv\0\
    glVertexAttrib1s\0\
    glVertexAttrib1sv\0\
    glVertexAttrib2d\0\
    glVertexAttrib2dv\0\
    glVertexAttrib2s\0\
    glVertexAttrib2sv\0\
    glVertexAttrib3d\0\
    glVertexAttrib3dv\0\
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
    glVertexAttrib4iv\0\
    glVertexAttrib4s\0\
    glVertexAttrib4sv\0\
    glVertexAttrib4ubv\0\
    glVertexAttrib4uiv\0\
    glVertexAttrib4usv\0\
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
    glVertexAttrib1f\0\
    glVertexAttrib1fv\0\
    glVertexAttrib2f\0\
    glVertexAttrib2fv\0\
    glVertexAttrib3f\0\
    glVertexAttrib3fv\0\
    glVertexAttrib4f\0\
    glVertexAttrib4fv\0\
    glVertexAttribPointer\0\
    glDrawBuffers\0\
    glUniformMatrix2x3fv\0\
    glUniformMatrix2x4fv\0\
    glUniformMatrix3x2fv\0\
    glUniformMatrix3x4fv\0\
    glUniformMatrix4x2fv\0\
    glUniformMatrix4x3fv\0\
    glBeginConditionalRender\0\
    glBindFragDataLocation\0\
    glClampColor\0\
    glColorMaski\0\
    glDisablei\0\
    glEnablei\0\
    glEndConditionalRender\0\
    glFramebufferTexture1D\0\
    glFramebufferTexture3D\0\
    glGetBooleani_v\0\
    glGetTexParameterIiv\0\
    glGetTexParameterIuiv\0\
    glIsEnabledi\0\
    glTexParameterIiv\0\
    glTexParameterIuiv\0\
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
    glVertexAttribI4sv\0\
    glVertexAttribI4ubv\0\
    glVertexAttribI4usv\0\
    glBindBufferBase\0\
    glBindBufferRange\0\
    glGetIntegeri_v\0\
    glBindFramebuffer\0\
    glBindRenderbuffer\0\
    glCheckFramebufferStatus\0\
    glDeleteFramebuffers\0\
    glDeleteRenderbuffers\0\
    glFramebufferRenderbuffer\0\
    glFramebufferTexture2D\0\
    glGenFramebuffers\0\
    glGenRenderbuffers\0\
    glGenerateMipmap\0\
    glGetFramebufferAttachmentParameteriv\0\
    glGetRenderbufferParameteriv\0\
    glIsFramebuffer\0\
    glIsRenderbuffer\0\
    glRenderbufferStorage\0\
    glBeginTransformFeedback\0\
    glBindVertexArray\0\
    glBlitFramebuffer\0\
    glClearBufferfi\0\
    glClearBufferfv\0\
    glClearBufferiv\0\
    glClearBufferuiv\0\
    glDeleteVertexArrays\0\
    glEndTransformFeedback\0\
    glFlushMappedBufferRange\0\
    glFramebufferTextureLayer\0\
    glGenVertexArrays\0\
    glGetFragDataLocation\0\
    glGetStringi\0\
    glGetTransformFeedbackVarying\0\
    glGetUniformuiv\0\
    glGetVertexAttribIiv\0\
    glGetVertexAttribIuiv\0\
    glIsVertexArray\0\
    glMapBufferRange\0\
    glRenderbufferStorageMultisample\0\
    glTransformFeedbackVaryings\0\
    glUniform1ui\0\
    glUniform1uiv\0\
    glUniform2ui\0\
    glUniform2uiv\0\
    glUniform3ui\0\
    glUniform3uiv\0\
    glUniform4ui\0\
    glUniform4uiv\0\
    glVertexAttribI4i\0\
    glVertexAttribI4iv\0\
    glVertexAttribI4ui\0\
    glVertexAttribI4uiv\0\
    glVertexAttribIPointer\0\
    glGetActiveUniformName\0\
    glPrimitiveRestartIndex\0\
    glTexBuffer\0\
    glCopyBufferSubData\0\
    glDrawArraysInstanced\0\
    glDrawElementsInstanced\0\
    glGetActiveUniformBlockName\0\
    glGetActiveUniformBlockiv\0\
    glGetActiveUniformsiv\0\
    glGetUniformBlockIndex\0\
    glGetUniformIndices\0\
    glUniformBlockBinding\0\
    glDrawElementsBaseVertex\0\
    glDrawElementsInstancedBaseVertex\0\
    glDrawRangeElementsBaseVertex\0\
    glFramebufferTexture\0\
    glGetMultisamplefv\0\
    glMultiDrawElementsBaseVertex\0\
    glProvokingVertex\0\
    glSampleMaski\0\
    glTexImage2DMultisample\0\
    glTexImage3DMultisample\0\
    glClientWaitSync\0\
    glDeleteSync\0\
    glFenceSync\0\
    glGetBufferParameteri64v\0\
    glGetInteger64i_v\0\
    glGetInteger64v\0\
    glGetSynciv\0\
    glIsSync\0\
    glWaitSync\0\
    glBindFragDataLocationIndexed\0\
    glGetFragDataIndex\0\
    glGetQueryObjecti64v\0\
    glGetQueryObjectui64v\0\
    glGetSamplerParameterIiv\0\
    glGetSamplerParameterIuiv\0\
    glQueryCounter\0\
    glSamplerParameterIiv\0\
    glSamplerParameterIuiv\0\
    glVertexAttribP1ui\0\
    glVertexAttribP1uiv\0\
    glVertexAttribP2ui\0\
    glVertexAttribP2uiv\0\
    glVertexAttribP3ui\0\
    glVertexAttribP3uiv\0\
    glVertexAttribP4ui\0\
    glVertexAttribP4uiv\0\
    glBindSampler\0\
    glDeleteSamplers\0\
    glGenSamplers\0\
    glGetSamplerParameterfv\0\
    glGetSamplerParameteriv\0\
    glIsSampler\0\
    glSamplerParameterf\0\
    glSamplerParameterfv\0\
    glSamplerParameteri\0\
    glSamplerParameteriv\0\
    glVertexAttribDivisor\0\
    glClearDepthf\0\
    glDepthRangef\0\
    glGetShaderPrecisionFormat\0\
    glReleaseShaderCompiler\0\
    glShaderBinary\0\
    glBindTransformFeedback\0\
    glDeleteTransformFeedbacks\0\
    glGenTransformFeedbacks\0\
    glGetInternalformativ\0\
    glGetProgramBinary\0\
    glInvalidateFramebuffer\0\
    glInvalidateSubFramebuffer\0\
    glIsTransformFeedback\0\
    glPauseTransformFeedback\0\
    glProgramBinary\0\
    glProgramParameteri\0\
    glResumeTransformFeedback\0\
    glTexStorage2D\0\
    glTexStorage3D\0\
    glGetPointerv\0\
    glDebugMessageCallback\0\
    glDebugMessageCallbackKHR\0\
    glDebugMessageControl\0\
    glDebugMessageControlKHR\0\
    glDebugMessageInsert\0\
    glDebugMessageInsertKHR\0\
    glGetDebugMessageLog\0\
    glGetDebugMessageLogKHR\0\
    glGetObjectLabel\0\
    glGetObjectLabelKHR\0\
    glGetObjectPtrLabel\0\
    glGetObjectPtrLabelKHR\0\
    glGetPointervKHR\0\
    glObjectLabel\0\
    glObjectLabelKHR\0\
    glObjectPtrLabel\0\
    glObjectPtrLabelKHR\0\
    glPopDebugGroup\0\
    glPopDebugGroupKHR\0\
    glPushDebugGroup\0\
    glPushDebugGroupKHR\0\
";

// Byte offset of each command name in FN_NAME_DATA, indexed in
// lockstep with the pfn table (slot [i] == command i).
#[rustfmt::skip]
static FN_NAME_OFFSETS: [u16; COMMAND_COUNT] = [
          0, // [0] glClearDepth
         13, // [1] glDepthRange
         26, // [2] glDrawBuffer
         39, // [3] glGetDoublev
         52, // [4] glGetTexImage
         66, // [5] glGetTexLevelParameterfv
         91, // [6] glGetTexLevelParameteriv
        116, // [7] glLogicOp
        126, // [8] glPixelStoref
        140, // [9] glPointSize
        152, // [10] glPolygonMode
        166, // [11] glTexImage1D
        179, // [12] glBlendFunc
        191, // [13] glClear
        199, // [14] glClearColor
        212, // [15] glClearStencil
        227, // [16] glColorMask
        239, // [17] glCullFace
        250, // [18] glDepthFunc
        262, // [19] glDepthMask
        274, // [20] glDisable
        284, // [21] glEnable
        293, // [22] glFinish
        302, // [23] glFlush
        310, // [24] glFrontFace
        322, // [25] glGetBooleanv
        336, // [26] glGetError
        347, // [27] glGetFloatv
        359, // [28] glGetIntegerv
        373, // [29] glGetString
        385, // [30] glGetTexParameterfv
        405, // [31] glGetTexParameteriv
        425, // [32] glHint
        432, // [33] glIsEnabled
        444, // [34] glLineWidth
        456, // [35] glPixelStorei
        470, // [36] glReadPixels
        483, // [37] glScissor
        493, // [38] glStencilFunc
        507, // [39] glStencilMask
        521, // [40] glStencilOp
        533, // [41] glTexImage2D
        546, // [42] glTexParameterf
        562, // [43] glTexParameterfv
        579, // [44] glTexParameteri
        595, // [45] glTexParameteriv
        612, // [46] glViewport
        623, // [47] glReadBuffer
        636, // [48] glCopyTexImage1D
        653, // [49] glCopyTexSubImage1D
        673, // [50] glTexSubImage1D
        689, // [51] glBindTexture
        703, // [52] glCopyTexImage2D
        720, // [53] glCopyTexSubImage2D
        740, // [54] glDeleteTextures
        757, // [55] glDrawArrays
        770, // [56] glDrawElements
        785, // [57] glGenTextures
        799, // [58] glIsTexture
        811, // [59] glPolygonOffset
        827, // [60] glTexSubImage2D
        843, // [61] glCopyTexSubImage3D
        863, // [62] glDrawRangeElements
        883, // [63] glTexImage3D
        896, // [64] glTexSubImage3D
        912, // [65] glCompressedTexImage1D
        935, // [66] glCompressedTexSubImage1D
        961, // [67] glGetCompressedTexImage
        985, // [68] glActiveTexture
       1001, // [69] glCompressedTexImage2D
       1024, // [70] glCompressedTexSubImage2D
       1050, // [71] glSampleCoverage
       1067, // [72] glCompressedTexImage3D
       1090, // [73] glCompressedTexSubImage3D
       1116, // [74] glMultiDrawArrays
       1134, // [75] glMultiDrawElements
       1154, // [76] glPointParameterf
       1172, // [77] glPointParameterfv
       1191, // [78] glPointParameteri
       1209, // [79] glPointParameteriv
       1228, // [80] glBlendColor
       1241, // [81] glBlendEquation
       1257, // [82] glBlendFuncSeparate
       1277, // [83] glGetBufferSubData
       1296, // [84] glGetQueryObjectiv
       1315, // [85] glMapBuffer
       1327, // [86] glBindBuffer
       1340, // [87] glBufferData
       1353, // [88] glBufferSubData
       1369, // [89] glDeleteBuffers
       1385, // [90] glGenBuffers
       1398, // [91] glGetBufferParameteriv
       1421, // [92] glIsBuffer
       1432, // [93] glBeginQuery
       1445, // [94] glDeleteQueries
       1461, // [95] glEndQuery
       1472, // [96] glGenQueries
       1485, // [97] glGetBufferPointerv
       1505, // [98] glGetQueryObjectuiv
       1525, // [99] glGetQueryiv
       1538, // [100] glIsQuery
       1548, // [101] glUnmapBuffer
       1562, // [102] glGetVertexAttribdv
       1582, // [103] glVertexAttrib1d
       1599, // [104] glVertexAttrib1dv
       1617, // [105] glVertexAttrib1s
       1634, // [106] glVertexAttrib1sv
       1652, // [107] glVertexAttrib2d
       1669, // [108] glVertexAttrib2dv
       1687, // [109] glVertexAttrib2s
       1704, // [110] glVertexAttrib2sv
       1722, // [111] glVertexAttrib3d
       1739, // [112] glVertexAttrib3dv
       1757, // [113] glVertexAttrib3s
       1774, // [114] glVertexAttrib3sv
       1792, // [115] glVertexAttrib4Nbv
       1811, // [116] glVertexAttrib4Niv
       1830, // [117] glVertexAttrib4Nsv
       1849, // [118] glVertexAttrib4Nub
       1868, // [119] glVertexAttrib4Nubv
       1888, // [120] glVertexAttrib4Nuiv
       1908, // [121] glVertexAttrib4Nusv
       1928, // [122] glVertexAttrib4bv
       1946, // [123] glVertexAttrib4d
       1963, // [124] glVertexAttrib4dv
       1981, // [125] glVertexAttrib4iv
       1999, // [126] glVertexAttrib4s
       2016, // [127] glVertexAttrib4sv
       2034, // [128] glVertexAttrib4ubv
       2053, // [129] glVertexAttrib4uiv
       2072, // [130] glVertexAttrib4usv
       2091, // [131] glAttachShader
       2106, // [132] glBindAttribLocation
       2127, // [133] glBlendEquationSeparate
       2151, // [134] glCompileShader
       2167, // [135] glCreateProgram
       2183, // [136] glCreateShader
       2198, // [137] glDeleteProgram
       2214, // [138] glDeleteShader
       2229, // [139] glDetachShader
       2244, // [140] glDisableVertexAttribArray
       2271, // [141] glEnableVertexAttribArray
       2297, // [142] glGetActiveAttrib
       2315, // [143] glGetActiveUniform
       2334, // [144] glGetAttachedShaders
       2355, // [145] glGetAttribLocation
       2375, // [146] glGetProgramInfoLog
       2395, // [147] glGetProgramiv
       2410, // [148] glGetShaderInfoLog
       2429, // [149] glGetShaderSource
       2447, // [150] glGetShaderiv
       2461, // [151] glGetUniformLocation
       2482, // [152] glGetUniformfv
       2497, // [153] glGetUniformiv
       2512, // [154] glGetVertexAttribPointerv
       2538, // [155] glGetVertexAttribfv
       2558, // [156] glGetVertexAttribiv
       2578, // [157] glIsProgram
       2590, // [158] glIsShader
       2601, // [159] glLinkProgram
       2615, // [160] glShaderSource
       2630, // [161] glStencilFuncSeparate
       2652, // [162] glStencilMaskSeparate
       2674, // [163] glStencilOpSeparate
       2694, // [164] glUniform1f
       2706, // [165] glUniform1fv
       2719, // [166] glUniform1i
       2731, // [167] glUniform1iv
       2744, // [168] glUniform2f
       2756, // [169] glUniform2fv
       2769, // [170] glUniform2i
       2781, // [171] glUniform2iv
       2794, // [172] glUniform3f
       2806, // [173] glUniform3fv
       2819, // [174] glUniform3i
       2831, // [175] glUniform3iv
       2844, // [176] glUniform4f
       2856, // [177] glUniform4fv
       2869, // [178] glUniform4i
       2881, // [179] glUniform4iv
       2894, // [180] glUniformMatrix2fv
       2913, // [181] glUniformMatrix3fv
       2932, // [182] glUniformMatrix4fv
       2951, // [183] glUseProgram
       2964, // [184] glValidateProgram
       2982, // [185] glVertexAttrib1f
       2999, // [186] glVertexAttrib1fv
       3017, // [187] glVertexAttrib2f
       3034, // [188] glVertexAttrib2fv
       3052, // [189] glVertexAttrib3f
       3069, // [190] glVertexAttrib3fv
       3087, // [191] glVertexAttrib4f
       3104, // [192] glVertexAttrib4fv
       3122, // [193] glVertexAttribPointer
       3144, // [194] glDrawBuffers
       3158, // [195] glUniformMatrix2x3fv
       3179, // [196] glUniformMatrix2x4fv
       3200, // [197] glUniformMatrix3x2fv
       3221, // [198] glUniformMatrix3x4fv
       3242, // [199] glUniformMatrix4x2fv
       3263, // [200] glUniformMatrix4x3fv
       3284, // [201] glBeginConditionalRender
       3309, // [202] glBindFragDataLocation
       3332, // [203] glClampColor
       3345, // [204] glColorMaski
       3358, // [205] glDisablei
       3369, // [206] glEnablei
       3379, // [207] glEndConditionalRender
       3402, // [208] glFramebufferTexture1D
       3425, // [209] glFramebufferTexture3D
       3448, // [210] glGetBooleani_v
       3464, // [211] glGetTexParameterIiv
       3485, // [212] glGetTexParameterIuiv
       3507, // [213] glIsEnabledi
       3520, // [214] glTexParameterIiv
       3538, // [215] glTexParameterIuiv
       3557, // [216] glVertexAttribI1i
       3575, // [217] glVertexAttribI1iv
       3594, // [218] glVertexAttribI1ui
       3613, // [219] glVertexAttribI1uiv
       3633, // [220] glVertexAttribI2i
       3651, // [221] glVertexAttribI2iv
       3670, // [222] glVertexAttribI2ui
       3689, // [223] glVertexAttribI2uiv
       3709, // [224] glVertexAttribI3i
       3727, // [225] glVertexAttribI3iv
       3746, // [226] glVertexAttribI3ui
       3765, // [227] glVertexAttribI3uiv
       3785, // [228] glVertexAttribI4bv
       3804, // [229] glVertexAttribI4sv
       3823, // [230] glVertexAttribI4ubv
       3843, // [231] glVertexAttribI4usv
       3863, // [232] glBindBufferBase
       3880, // [233] glBindBufferRange
       3898, // [234] glGetIntegeri_v
       3914, // [235] glBindFramebuffer
       3932, // [236] glBindRenderbuffer
       3951, // [237] glCheckFramebufferStatus
       3976, // [238] glDeleteFramebuffers
       3997, // [239] glDeleteRenderbuffers
       4019, // [240] glFramebufferRenderbuffer
       4045, // [241] glFramebufferTexture2D
       4068, // [242] glGenFramebuffers
       4086, // [243] glGenRenderbuffers
       4105, // [244] glGenerateMipmap
       4122, // [245] glGetFramebufferAttachmentParameteriv
       4160, // [246] glGetRenderbufferParameteriv
       4189, // [247] glIsFramebuffer
       4205, // [248] glIsRenderbuffer
       4222, // [249] glRenderbufferStorage
       4244, // [250] glBeginTransformFeedback
       4269, // [251] glBindVertexArray
       4287, // [252] glBlitFramebuffer
       4305, // [253] glClearBufferfi
       4321, // [254] glClearBufferfv
       4337, // [255] glClearBufferiv
       4353, // [256] glClearBufferuiv
       4370, // [257] glDeleteVertexArrays
       4391, // [258] glEndTransformFeedback
       4414, // [259] glFlushMappedBufferRange
       4439, // [260] glFramebufferTextureLayer
       4465, // [261] glGenVertexArrays
       4483, // [262] glGetFragDataLocation
       4505, // [263] glGetStringi
       4518, // [264] glGetTransformFeedbackVarying
       4548, // [265] glGetUniformuiv
       4564, // [266] glGetVertexAttribIiv
       4585, // [267] glGetVertexAttribIuiv
       4607, // [268] glIsVertexArray
       4623, // [269] glMapBufferRange
       4640, // [270] glRenderbufferStorageMultisample
       4673, // [271] glTransformFeedbackVaryings
       4701, // [272] glUniform1ui
       4714, // [273] glUniform1uiv
       4728, // [274] glUniform2ui
       4741, // [275] glUniform2uiv
       4755, // [276] glUniform3ui
       4768, // [277] glUniform3uiv
       4782, // [278] glUniform4ui
       4795, // [279] glUniform4uiv
       4809, // [280] glVertexAttribI4i
       4827, // [281] glVertexAttribI4iv
       4846, // [282] glVertexAttribI4ui
       4865, // [283] glVertexAttribI4uiv
       4885, // [284] glVertexAttribIPointer
       4908, // [285] glGetActiveUniformName
       4931, // [286] glPrimitiveRestartIndex
       4955, // [287] glTexBuffer
       4967, // [288] glCopyBufferSubData
       4987, // [289] glDrawArraysInstanced
       5009, // [290] glDrawElementsInstanced
       5033, // [291] glGetActiveUniformBlockName
       5061, // [292] glGetActiveUniformBlockiv
       5087, // [293] glGetActiveUniformsiv
       5109, // [294] glGetUniformBlockIndex
       5132, // [295] glGetUniformIndices
       5152, // [296] glUniformBlockBinding
       5174, // [297] glDrawElementsBaseVertex
       5199, // [298] glDrawElementsInstancedBaseVertex
       5233, // [299] glDrawRangeElementsBaseVertex
       5263, // [300] glFramebufferTexture
       5284, // [301] glGetMultisamplefv
       5303, // [302] glMultiDrawElementsBaseVertex
       5333, // [303] glProvokingVertex
       5351, // [304] glSampleMaski
       5365, // [305] glTexImage2DMultisample
       5389, // [306] glTexImage3DMultisample
       5413, // [307] glClientWaitSync
       5430, // [308] glDeleteSync
       5443, // [309] glFenceSync
       5455, // [310] glGetBufferParameteri64v
       5480, // [311] glGetInteger64i_v
       5498, // [312] glGetInteger64v
       5514, // [313] glGetSynciv
       5526, // [314] glIsSync
       5535, // [315] glWaitSync
       5546, // [316] glBindFragDataLocationIndexed
       5576, // [317] glGetFragDataIndex
       5595, // [318] glGetQueryObjecti64v
       5616, // [319] glGetQueryObjectui64v
       5638, // [320] glGetSamplerParameterIiv
       5663, // [321] glGetSamplerParameterIuiv
       5689, // [322] glQueryCounter
       5704, // [323] glSamplerParameterIiv
       5726, // [324] glSamplerParameterIuiv
       5749, // [325] glVertexAttribP1ui
       5768, // [326] glVertexAttribP1uiv
       5788, // [327] glVertexAttribP2ui
       5807, // [328] glVertexAttribP2uiv
       5827, // [329] glVertexAttribP3ui
       5846, // [330] glVertexAttribP3uiv
       5866, // [331] glVertexAttribP4ui
       5885, // [332] glVertexAttribP4uiv
       5905, // [333] glBindSampler
       5919, // [334] glDeleteSamplers
       5936, // [335] glGenSamplers
       5950, // [336] glGetSamplerParameterfv
       5974, // [337] glGetSamplerParameteriv
       5998, // [338] glIsSampler
       6010, // [339] glSamplerParameterf
       6030, // [340] glSamplerParameterfv
       6051, // [341] glSamplerParameteri
       6071, // [342] glSamplerParameteriv
       6092, // [343] glVertexAttribDivisor
       6114, // [344] glClearDepthf
       6128, // [345] glDepthRangef
       6142, // [346] glGetShaderPrecisionFormat
       6169, // [347] glReleaseShaderCompiler
       6193, // [348] glShaderBinary
       6208, // [349] glBindTransformFeedback
       6232, // [350] glDeleteTransformFeedbacks
       6259, // [351] glGenTransformFeedbacks
       6283, // [352] glGetInternalformativ
       6305, // [353] glGetProgramBinary
       6324, // [354] glInvalidateFramebuffer
       6348, // [355] glInvalidateSubFramebuffer
       6375, // [356] glIsTransformFeedback
       6397, // [357] glPauseTransformFeedback
       6422, // [358] glProgramBinary
       6438, // [359] glProgramParameteri
       6458, // [360] glResumeTransformFeedback
       6484, // [361] glTexStorage2D
       6499, // [362] glTexStorage3D
       6514, // [363] glGetPointerv
       6528, // [364] glDebugMessageCallback
       6551, // [365] glDebugMessageCallbackKHR
       6577, // [366] glDebugMessageControl
       6599, // [367] glDebugMessageControlKHR
       6624, // [368] glDebugMessageInsert
       6645, // [369] glDebugMessageInsertKHR
       6669, // [370] glGetDebugMessageLog
       6690, // [371] glGetDebugMessageLogKHR
       6714, // [372] glGetObjectLabel
       6731, // [373] glGetObjectLabelKHR
       6751, // [374] glGetObjectPtrLabel
       6771, // [375] glGetObjectPtrLabelKHR
       6794, // [376] glGetPointervKHR
       6811, // [377] glObjectLabel
       6825, // [378] glObjectLabelKHR
       6842, // [379] glObjectPtrLabel
       6859, // [380] glObjectPtrLabelKHR
       6879, // [381] glPopDebugGroup
       6895, // [382] glPopDebugGroupKHR
       6914, // [383] glPushDebugGroup
       6931, // [384] glPushDebugGroupKHR
];

#[rustfmt::skip]
static FEATURE_RANGES: [(u16, u16, u16); 33] = [
    (   0,    0,   48), // GL_VERSION_1_0
    (   1,   48,   13), // GL_VERSION_1_1
    (   1,  363,    1), // GL_VERSION_1_1
    (   2,   61,    4), // GL_VERSION_1_2
    (   3,   65,    9), // GL_VERSION_1_3
    (   4,   74,    9), // GL_VERSION_1_4
    (   5,   83,   19), // GL_VERSION_1_5
    (   6,  102,   93), // GL_VERSION_2_0
    (   7,  195,    6), // GL_VERSION_2_1
    (   8,  201,   84), // GL_VERSION_3_0
    (   9,  232,    3), // GL_VERSION_3_1
    (   9,  285,   12), // GL_VERSION_3_1
    (  10,  297,   19), // GL_VERSION_3_2
    (  11,  316,   28), // GL_VERSION_3_3
    (  12,   12,   35), // GL_ES_VERSION_2_0
    (  12,   51,   10), // GL_ES_VERSION_2_0
    (  12,   68,    4), // GL_ES_VERSION_2_0
    (  12,   80,    3), // GL_ES_VERSION_2_0
    (  12,   86,    7), // GL_ES_VERSION_2_0
    (  12,  131,   63), // GL_ES_VERSION_2_0
    (  12,  235,   15), // GL_ES_VERSION_2_0
    (  12,  344,    5), // GL_ES_VERSION_2_0
    (  13,   47,    1), // GL_ES_VERSION_3_0
    (  13,   61,    4), // GL_ES_VERSION_3_0
    (  13,   72,    2), // GL_ES_VERSION_3_0
    (  13,   93,    9), // GL_ES_VERSION_3_0
    (  13,  194,    7), // GL_ES_VERSION_3_0
    (  13,  232,    3), // GL_ES_VERSION_3_0
    (  13,  250,   35), // GL_ES_VERSION_3_0
    (  13,  288,    9), // GL_ES_VERSION_3_0
    (  13,  307,    9), // GL_ES_VERSION_3_0
    (  13,  333,   11), // GL_ES_VERSION_3_0
    (  13,  349,   14), // GL_ES_VERSION_3_0
];

#[rustfmt::skip]
static EXT_RANGES_gl: [(u16, u16, u16); 10] = [
    (   0,  363,    2), // GL_KHR_debug
    (   0,  366,    1), // GL_KHR_debug
    (   0,  368,    1), // GL_KHR_debug
    (   0,  370,    1), // GL_KHR_debug
    (   0,  372,    1), // GL_KHR_debug
    (   0,  374,    1), // GL_KHR_debug
    (   0,  377,    1), // GL_KHR_debug
    (   0,  379,    1), // GL_KHR_debug
    (   0,  381,    1), // GL_KHR_debug
    (   0,  383,    1), // GL_KHR_debug
];

#[rustfmt::skip]
static EXT_RANGES_gles2: [(u16, u16, u16); 10] = [
    (   0,  365,    1), // GL_KHR_debug
    (   0,  367,    1), // GL_KHR_debug
    (   0,  369,    1), // GL_KHR_debug
    (   0,  371,    1), // GL_KHR_debug
    (   0,  373,    1), // GL_KHR_debug
    (   0,  375,    2), // GL_KHR_debug
    (   0,  378,    1), // GL_KHR_debug
    (   0,  380,    1), // GL_KHR_debug
    (   0,  382,    1), // GL_KHR_debug
    (   0,  384,    1), // GL_KHR_debug
];

// ── Extensions ──────────────────────────────────────────────
pub const EXT_COUNT: usize = 1;

// XXH3-64 of each extension name, sorted for binary search.
#[rustfmt::skip]
static EXT_HASH_KEYS: [u64; EXT_COUNT] = [
    0x5e0c5b9607ac8784, // GL_KHR_debug
];
// extArray index for the correspondingly-ranked EXT_HASH_KEYS entry.
#[rustfmt::skip]
static EXT_HASH_IDX: [u16; EXT_COUNT] = [
    0,
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

// (canonical, secondary) command indices propagated by --alias.
#[rustfmt::skip]
static ALIAS_PAIRS: [(u16, u16); 11] = [
    ( 363,  376), // glGetPointerv <-> glGetPointervKHR
    ( 364,  365), // glDebugMessageCallback <-> glDebugMessageCallbackKHR
    ( 366,  367), // glDebugMessageControl <-> glDebugMessageControlKHR
    ( 368,  369), // glDebugMessageInsert <-> glDebugMessageInsertKHR
    ( 370,  371), // glGetDebugMessageLog <-> glGetDebugMessageLogKHR
    ( 372,  373), // glGetObjectLabel <-> glGetObjectLabelKHR
    ( 374,  375), // glGetObjectPtrLabel <-> glGetObjectPtrLabelKHR
    ( 377,  378), // glObjectLabel <-> glObjectLabelKHR
    ( 379,  380), // glObjectPtrLabel <-> glObjectPtrLabelKHR
    ( 381,  382), // glPopDebugGroup <-> glPopDebugGroupKHR
    ( 383,  384), // glPushDebugGroup <-> glPushDebugGroupKHR
];

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
        gl.pfns[29] = loader(c"glGetString");
        if gl.pfns[29].is_null() {
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
    /// Load the gles2 API against `loader` (a GetProcAddress-style
    /// callback), detecting version then extensions.  `Err` when the
    /// current context has no usable gles2.
    ///
    /// # Safety
    /// A matching GL context must be current and `loader` valid.
    #[inline]
    pub unsafe fn load_gles2(
        mut loader: impl FnMut(&CStr) -> *const c_void,
    ) -> Result<Self, LoadError> {
        // Immediately erase to `&mut dyn` — the real loader is compiled
        // once, not once per closure type.
        unsafe { Self::load_gles2_dyn(&mut loader) }
    }

    unsafe fn load_gles2_dyn(
        loader: &mut dyn FnMut(&CStr) -> *const c_void,
    ) -> Result<Self, LoadError> {
        let mut gl = Self {
            pfns: [core::ptr::null(); COMMAND_COUNT],
            feat: [false; FEATURE_COUNT],
            ext: [false; EXT_COUNT],
            version: 0,
        };
        gl.pfns[29] = loader(c"glGetString");
        if gl.pfns[29].is_null() {
            return Err(LoadError::MissingGetString);
        }
        gl.version = unsafe { __parse_gl_version(gl.GetString(GL_VERSION)) };
        if gl.version == 0 {
            return Err(LoadError::UnparseableVersion);
        }
        // Feature presence for this API, from the parsed version.
        gl.feat[12] = gl.version >= 0x0200;
        gl.feat[13] = gl.version >= 0x0300;
        for &(fi, start, count) in FEATURE_RANGES.iter() {
            if gl.feat[fi as usize] {
                unsafe { gl.load_range(loader, start, count) };
            }
        }
        unsafe { gl.detect_extensions() };
        for &(ei, start, count) in EXT_RANGES_gles2.iter() {
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
        if self.pfns[28].is_null() || self.pfns[263].is_null() {
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

    /// Propagate each loaded pointer to its unloaded alias slot.
    fn resolve_aliases(&mut self) {
        for &(ci, si) in ALIAS_PAIRS.iter() {
            let (c, d) = (self.pfns[ci as usize], self.pfns[si as usize]);
            if c.is_null() && !d.is_null() {
                self.pfns[ci as usize] = d;
            } else if !c.is_null() && d.is_null() {
                self.pfns[si as usize] = c;
            }
        }
    }

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
    pub unsafe fn ClearDepth(&self, depth: GLdouble) {
        let __pfn: Option<unsafe extern "system" fn(GLdouble)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(0)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(0) },
        };
        unsafe { __pfn(depth) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DepthRange(&self, n: GLdouble, f: GLdouble) {
        let __pfn: Option<unsafe extern "system" fn(GLdouble, GLdouble)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(1)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(1) },
        };
        unsafe { __pfn(n, f) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DrawBuffer(&self, buf: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(2)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(2) },
        };
        unsafe { __pfn(buf) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetDoublev(&self, pname: GLenum, data: *mut GLdouble) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, *mut GLdouble)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(3)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(3) },
        };
        unsafe { __pfn(pname, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetTexImage(&self, target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLenum, GLenum, *mut c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(4)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(4) },
        };
        unsafe { __pfn(target, level, format, type_, pixels) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetTexLevelParameterfv(&self, target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLenum, *mut GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(5)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(5) },
        };
        unsafe { __pfn(target, level, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetTexLevelParameteriv(&self, target: GLenum, level: GLint, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(6)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(6) },
        };
        unsafe { __pfn(target, level, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn LogicOp(&self, opcode: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(7)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(7) },
        };
        unsafe { __pfn(opcode) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn PixelStoref(&self, pname: GLenum, param: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(8)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(8) },
        };
        unsafe { __pfn(pname, param) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn PointSize(&self, size: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(9)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(9) },
        };
        unsafe { __pfn(size) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn PolygonMode(&self, face: GLenum, mode: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(10)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(10) },
        };
        unsafe { __pfn(face, mode) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexImage1D(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLint, GLsizei, GLint, GLenum, GLenum, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(11)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(11) },
        };
        unsafe { __pfn(target, level, internalformat, width, border, format, type_, pixels) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BlendFunc(&self, sfactor: GLenum, dfactor: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(12)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(12) },
        };
        unsafe { __pfn(sfactor, dfactor) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Clear(&self, mask: GLbitfield) {
        let __pfn: Option<unsafe extern "system" fn(GLbitfield)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(13)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(13) },
        };
        unsafe { __pfn(mask) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ClearColor(&self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(14)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(14) },
        };
        unsafe { __pfn(red, green, blue, alpha) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ClearStencil(&self, s: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(15)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(15) },
        };
        unsafe { __pfn(s) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ColorMask(&self, red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) {
        let __pfn: Option<unsafe extern "system" fn(GLboolean, GLboolean, GLboolean, GLboolean)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(16)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(16) },
        };
        unsafe { __pfn(red, green, blue, alpha) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CullFace(&self, mode: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(17)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(17) },
        };
        unsafe { __pfn(mode) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DepthFunc(&self, func: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(18)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(18) },
        };
        unsafe { __pfn(func) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DepthMask(&self, flag: GLboolean) {
        let __pfn: Option<unsafe extern "system" fn(GLboolean)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(19)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(19) },
        };
        unsafe { __pfn(flag) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Disable(&self, cap: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(20)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(20) },
        };
        unsafe { __pfn(cap) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Enable(&self, cap: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(21)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(21) },
        };
        unsafe { __pfn(cap) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Finish(&self) {
        let __pfn: Option<unsafe extern "system" fn()> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(22)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(22) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Flush(&self) {
        let __pfn: Option<unsafe extern "system" fn()> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(23)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(23) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn FrontFace(&self, mode: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(24)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(24) },
        };
        unsafe { __pfn(mode) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetBooleanv(&self, pname: GLenum, data: *mut GLboolean) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, *mut GLboolean)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(25)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(25) },
        };
        unsafe { __pfn(pname, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetError(&self) -> GLenum {
        let __pfn: Option<unsafe extern "system" fn() -> GLenum> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(26)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(26) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetFloatv(&self, pname: GLenum, data: *mut GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, *mut GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(27)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(27) },
        };
        unsafe { __pfn(pname, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetIntegerv(&self, pname: GLenum, data: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(28)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(28) },
        };
        unsafe { __pfn(pname, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetString(&self, name: GLenum) -> *const GLubyte {
        let __pfn: Option<unsafe extern "system" fn(GLenum) -> *const GLubyte> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(29)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(29) },
        };
        unsafe { __pfn(name) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetTexParameterfv(&self, target: GLenum, pname: GLenum, params: *mut GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *mut GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(30)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(30) },
        };
        unsafe { __pfn(target, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetTexParameteriv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(31)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(31) },
        };
        unsafe { __pfn(target, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Hint(&self, target: GLenum, mode: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(32)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(32) },
        };
        unsafe { __pfn(target, mode) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn IsEnabled(&self, cap: GLenum) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLenum) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(33)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(33) },
        };
        unsafe { __pfn(cap) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn LineWidth(&self, width: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(34)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(34) },
        };
        unsafe { __pfn(width) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn PixelStorei(&self, pname: GLenum, param: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(35)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(35) },
        };
        unsafe { __pfn(pname, param) }
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
    pub unsafe fn TexImage2D(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLint, GLsizei, GLsizei, GLint, GLenum, GLenum, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(41)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(41) },
        };
        unsafe { __pfn(target, level, internalformat, width, height, border, format, type_, pixels) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexParameterf(&self, target: GLenum, pname: GLenum, param: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(42)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(42) },
        };
        unsafe { __pfn(target, pname, param) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexParameterfv(&self, target: GLenum, pname: GLenum, params: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(43)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(43) },
        };
        unsafe { __pfn(target, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexParameteri(&self, target: GLenum, pname: GLenum, param: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(44)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(44) },
        };
        unsafe { __pfn(target, pname, param) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexParameteriv(&self, target: GLenum, pname: GLenum, params: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(45)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(45) },
        };
        unsafe { __pfn(target, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLint, GLsizei, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(46)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(46) },
        };
        unsafe { __pfn(x, y, width, height) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ReadBuffer(&self, src: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(47)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(47) },
        };
        unsafe { __pfn(src) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CopyTexImage1D(&self, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(48)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(48) },
        };
        unsafe { __pfn(target, level, internalformat, x, y, width, border) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CopyTexSubImage1D(&self, target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(49)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(49) },
        };
        unsafe { __pfn(target, level, xoffset, x, y, width) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexSubImage1D(&self, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLint, GLsizei, GLenum, GLenum, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(50)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(50) },
        };
        unsafe { __pfn(target, level, xoffset, width, format, type_, pixels) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BindTexture(&self, target: GLenum, texture: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(51)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(51) },
        };
        unsafe { __pfn(target, texture) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CopyTexImage2D(&self, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLsizei, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(52)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(52) },
        };
        unsafe { __pfn(target, level, internalformat, x, y, width, height, border) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CopyTexSubImage2D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(53)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(53) },
        };
        unsafe { __pfn(target, level, xoffset, yoffset, x, y, width, height) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DeleteTextures(&self, n: GLsizei, textures: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(54)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(54) },
        };
        unsafe { __pfn(n, textures) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DrawArrays(&self, mode: GLenum, first: GLint, count: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(55)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(55) },
        };
        unsafe { __pfn(mode, first, count) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DrawElements(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLsizei, GLenum, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(56)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(56) },
        };
        unsafe { __pfn(mode, count, type_, indices) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GenTextures(&self, n: GLsizei, textures: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(57)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(57) },
        };
        unsafe { __pfn(n, textures) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn IsTexture(&self, texture: GLuint) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLuint) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(58)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(58) },
        };
        unsafe { __pfn(texture) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn PolygonOffset(&self, factor: GLfloat, units: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLfloat, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(59)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(59) },
        };
        unsafe { __pfn(factor, units) }
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
    pub unsafe fn CompressedTexImage1D(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLint, GLsizei, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(65)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(65) },
        };
        unsafe { __pfn(target, level, internalformat, width, border, imageSize, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CompressedTexSubImage1D(&self, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLint, GLsizei, GLenum, GLsizei, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(66)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(66) },
        };
        unsafe { __pfn(target, level, xoffset, width, format, imageSize, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetCompressedTexImage(&self, target: GLenum, level: GLint, img: *mut c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, *mut c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(67)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(67) },
        };
        unsafe { __pfn(target, level, img) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ActiveTexture(&self, texture: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(68)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(68) },
        };
        unsafe { __pfn(texture) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CompressedTexImage2D(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLsizei, GLint, GLsizei, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(69)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(69) },
        };
        unsafe { __pfn(target, level, internalformat, width, height, border, imageSize, data) }
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
    pub unsafe fn SampleCoverage(&self, value: GLfloat, invert: GLboolean) {
        let __pfn: Option<unsafe extern "system" fn(GLfloat, GLboolean)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(71)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(71) },
        };
        unsafe { __pfn(value, invert) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CompressedTexImage3D(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLsizei, GLsizei, GLint, GLsizei, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(72)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(72) },
        };
        unsafe { __pfn(target, level, internalformat, width, height, depth, border, imageSize, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CompressedTexSubImage3D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLsizei, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(73)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(73) },
        };
        unsafe { __pfn(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn MultiDrawArrays(&self, mode: GLenum, first: *const GLint, count: *const GLsizei, drawcount: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, *const GLint, *const GLsizei, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(74)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(74) },
        };
        unsafe { __pfn(mode, first, count, drawcount) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn MultiDrawElements(&self, mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, drawcount: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, *const GLsizei, GLenum, *const *const c_void, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(75)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(75) },
        };
        unsafe { __pfn(mode, count, type_, indices, drawcount) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn PointParameterf(&self, pname: GLenum, param: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(76)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(76) },
        };
        unsafe { __pfn(pname, param) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn PointParameterfv(&self, pname: GLenum, params: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(77)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(77) },
        };
        unsafe { __pfn(pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn PointParameteri(&self, pname: GLenum, param: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(78)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(78) },
        };
        unsafe { __pfn(pname, param) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn PointParameteriv(&self, pname: GLenum, params: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(79)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(79) },
        };
        unsafe { __pfn(pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BlendColor(&self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(80)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(80) },
        };
        unsafe { __pfn(red, green, blue, alpha) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BlendEquation(&self, mode: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(81)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(81) },
        };
        unsafe { __pfn(mode) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BlendFuncSeparate(&self, sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLenum, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(82)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(82) },
        };
        unsafe { __pfn(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetBufferSubData(&self, target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLintptr, GLsizeiptr, *mut c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(83)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(83) },
        };
        unsafe { __pfn(target, offset, size, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetQueryObjectiv(&self, id: GLuint, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(84)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(84) },
        };
        unsafe { __pfn(id, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn MapBuffer(&self, target: GLenum, access: GLenum) -> *mut c_void {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum) -> *mut c_void> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(85)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(85) },
        };
        unsafe { __pfn(target, access) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BindBuffer(&self, target: GLenum, buffer: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(86)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(86) },
        };
        unsafe { __pfn(target, buffer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BufferData(&self, target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLsizeiptr, *const c_void, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(87)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(87) },
        };
        unsafe { __pfn(target, size, data, usage) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BufferSubData(&self, target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLintptr, GLsizeiptr, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(88)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(88) },
        };
        unsafe { __pfn(target, offset, size, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DeleteBuffers(&self, n: GLsizei, buffers: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(89)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(89) },
        };
        unsafe { __pfn(n, buffers) }
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
    pub unsafe fn GetBufferParameteriv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(91)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(91) },
        };
        unsafe { __pfn(target, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn IsBuffer(&self, buffer: GLuint) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLuint) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(92)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(92) },
        };
        unsafe { __pfn(buffer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BeginQuery(&self, target: GLenum, id: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(93)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(93) },
        };
        unsafe { __pfn(target, id) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DeleteQueries(&self, n: GLsizei, ids: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(94)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(94) },
        };
        unsafe { __pfn(n, ids) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn EndQuery(&self, target: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(95)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(95) },
        };
        unsafe { __pfn(target) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GenQueries(&self, n: GLsizei, ids: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(96)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(96) },
        };
        unsafe { __pfn(n, ids) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetBufferPointerv(&self, target: GLenum, pname: GLenum, params: *mut *mut c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *mut *mut c_void)> =
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
    pub unsafe fn GetQueryObjectuiv(&self, id: GLuint, pname: GLenum, params: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(98)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(98) },
        };
        unsafe { __pfn(id, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetQueryiv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(99)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(99) },
        };
        unsafe { __pfn(target, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn IsQuery(&self, id: GLuint) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLuint) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(100)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(100) },
        };
        unsafe { __pfn(id) }
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
    pub unsafe fn GetVertexAttribdv(&self, index: GLuint, pname: GLenum, params: *mut GLdouble) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLdouble)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(102)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(102) },
        };
        unsafe { __pfn(index, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib1d(&self, index: GLuint, x: GLdouble) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLdouble)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(103)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(103) },
        };
        unsafe { __pfn(index, x) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib1dv(&self, index: GLuint, v: *const GLdouble) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLdouble)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(104)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(104) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib1s(&self, index: GLuint, x: GLshort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLshort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(105)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(105) },
        };
        unsafe { __pfn(index, x) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib1sv(&self, index: GLuint, v: *const GLshort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLshort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(106)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(106) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib2d(&self, index: GLuint, x: GLdouble, y: GLdouble) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLdouble, GLdouble)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(107)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(107) },
        };
        unsafe { __pfn(index, x, y) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib2dv(&self, index: GLuint, v: *const GLdouble) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLdouble)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(108)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(108) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib2s(&self, index: GLuint, x: GLshort, y: GLshort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLshort, GLshort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(109)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(109) },
        };
        unsafe { __pfn(index, x, y) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib2sv(&self, index: GLuint, v: *const GLshort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLshort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(110)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(110) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib3d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(111)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(111) },
        };
        unsafe { __pfn(index, x, y, z) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib3dv(&self, index: GLuint, v: *const GLdouble) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLdouble)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(112)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(112) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib3s(&self, index: GLuint, x: GLshort, y: GLshort, z: GLshort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLshort, GLshort, GLshort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(113)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(113) },
        };
        unsafe { __pfn(index, x, y, z) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib3sv(&self, index: GLuint, v: *const GLshort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLshort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(114)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(114) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4Nbv(&self, index: GLuint, v: *const GLbyte) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLbyte)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(115)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(115) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4Niv(&self, index: GLuint, v: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(116)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(116) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4Nsv(&self, index: GLuint, v: *const GLshort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLshort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(117)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(117) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4Nub(&self, index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLubyte, GLubyte, GLubyte, GLubyte)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(118)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(118) },
        };
        unsafe { __pfn(index, x, y, z, w) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4Nubv(&self, index: GLuint, v: *const GLubyte) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLubyte)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(119)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(119) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4Nuiv(&self, index: GLuint, v: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(120)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(120) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4Nusv(&self, index: GLuint, v: *const GLushort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLushort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(121)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(121) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4bv(&self, index: GLuint, v: *const GLbyte) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLbyte)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(122)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(122) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble, GLdouble)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(123)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(123) },
        };
        unsafe { __pfn(index, x, y, z, w) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4dv(&self, index: GLuint, v: *const GLdouble) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLdouble)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(124)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(124) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4iv(&self, index: GLuint, v: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(125)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(125) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4s(&self, index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLshort, GLshort, GLshort, GLshort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(126)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(126) },
        };
        unsafe { __pfn(index, x, y, z, w) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4sv(&self, index: GLuint, v: *const GLshort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLshort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(127)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(127) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4ubv(&self, index: GLuint, v: *const GLubyte) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLubyte)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(128)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(128) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4uiv(&self, index: GLuint, v: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(129)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(129) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4usv(&self, index: GLuint, v: *const GLushort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLushort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(130)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(130) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn AttachShader(&self, program: GLuint, shader: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(131)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(131) },
        };
        unsafe { __pfn(program, shader) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BindAttribLocation(&self, program: GLuint, index: GLuint, name: *const GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint, *const GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(132)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(132) },
        };
        unsafe { __pfn(program, index, name) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BlendEquationSeparate(&self, modeRGB: GLenum, modeAlpha: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(133)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(133) },
        };
        unsafe { __pfn(modeRGB, modeAlpha) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CompileShader(&self, shader: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(134)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(134) },
        };
        unsafe { __pfn(shader) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CreateProgram(&self) -> GLuint {
        let __pfn: Option<unsafe extern "system" fn() -> GLuint> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(135)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(135) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CreateShader(&self, type_: GLenum) -> GLuint {
        let __pfn: Option<unsafe extern "system" fn(GLenum) -> GLuint> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(136)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(136) },
        };
        unsafe { __pfn(type_) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DeleteProgram(&self, program: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(137)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(137) },
        };
        unsafe { __pfn(program) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DeleteShader(&self, shader: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(138)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(138) },
        };
        unsafe { __pfn(shader) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DetachShader(&self, program: GLuint, shader: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(139)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(139) },
        };
        unsafe { __pfn(program, shader) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DisableVertexAttribArray(&self, index: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(140)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(140) },
        };
        unsafe { __pfn(index) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn EnableVertexAttribArray(&self, index: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(141)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(141) },
        };
        unsafe { __pfn(index) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetActiveAttrib(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(142)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(142) },
        };
        unsafe { __pfn(program, index, bufSize, length, size, type_, name) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetActiveUniform(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(143)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(143) },
        };
        unsafe { __pfn(program, index, bufSize, length, size, type_, name) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetAttachedShaders(&self, program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(144)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(144) },
        };
        unsafe { __pfn(program, maxCount, count, shaders) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetAttribLocation(&self, program: GLuint, name: *const GLchar) -> GLint {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLchar) -> GLint> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(145)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(145) },
        };
        unsafe { __pfn(program, name) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetProgramInfoLog(&self, program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(146)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(146) },
        };
        unsafe { __pfn(program, bufSize, length, infoLog) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetProgramiv(&self, program: GLuint, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(147)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(147) },
        };
        unsafe { __pfn(program, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetShaderInfoLog(&self, shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(148)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(148) },
        };
        unsafe { __pfn(shader, bufSize, length, infoLog) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetShaderSource(&self, shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(149)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(149) },
        };
        unsafe { __pfn(shader, bufSize, length, source) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetShaderiv(&self, shader: GLuint, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(150)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(150) },
        };
        unsafe { __pfn(shader, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetUniformLocation(&self, program: GLuint, name: *const GLchar) -> GLint {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLchar) -> GLint> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(151)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(151) },
        };
        unsafe { __pfn(program, name) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetUniformfv(&self, program: GLuint, location: GLint, params: *mut GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLint, *mut GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(152)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(152) },
        };
        unsafe { __pfn(program, location, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetUniformiv(&self, program: GLuint, location: GLint, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLint, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(153)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(153) },
        };
        unsafe { __pfn(program, location, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetVertexAttribPointerv(&self, index: GLuint, pname: GLenum, pointer: *mut *mut c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut *mut c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(154)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(154) },
        };
        unsafe { __pfn(index, pname, pointer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetVertexAttribfv(&self, index: GLuint, pname: GLenum, params: *mut GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(155)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(155) },
        };
        unsafe { __pfn(index, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetVertexAttribiv(&self, index: GLuint, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(156)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(156) },
        };
        unsafe { __pfn(index, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn IsProgram(&self, program: GLuint) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLuint) -> GLboolean> =
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
    pub unsafe fn IsShader(&self, shader: GLuint) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLuint) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(158)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(158) },
        };
        unsafe { __pfn(shader) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn LinkProgram(&self, program: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(159)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(159) },
        };
        unsafe { __pfn(program) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ShaderSource(&self, shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLsizei, *const *const GLchar, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(160)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(160) },
        };
        unsafe { __pfn(shader, count, string, length) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn StencilFuncSeparate(&self, face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(161)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(161) },
        };
        unsafe { __pfn(face, func, ref_, mask) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn StencilMaskSeparate(&self, face: GLenum, mask: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(162)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(162) },
        };
        unsafe { __pfn(face, mask) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn StencilOpSeparate(&self, face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLenum, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(163)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(163) },
        };
        unsafe { __pfn(face, sfail, dpfail, dppass) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform1f(&self, location: GLint, v0: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(164)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(164) },
        };
        unsafe { __pfn(location, v0) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform1fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(165)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(165) },
        };
        unsafe { __pfn(location, count, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform1i(&self, location: GLint, v0: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(166)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(166) },
        };
        unsafe { __pfn(location, v0) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform1iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(167)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(167) },
        };
        unsafe { __pfn(location, count, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform2f(&self, location: GLint, v0: GLfloat, v1: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLfloat, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(168)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(168) },
        };
        unsafe { __pfn(location, v0, v1) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform2fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(169)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(169) },
        };
        unsafe { __pfn(location, count, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform2i(&self, location: GLint, v0: GLint, v1: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(170)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(170) },
        };
        unsafe { __pfn(location, v0, v1) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform2iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(171)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(171) },
        };
        unsafe { __pfn(location, count, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform3f(&self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLfloat, GLfloat, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(172)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(172) },
        };
        unsafe { __pfn(location, v0, v1, v2) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform3fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(173)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(173) },
        };
        unsafe { __pfn(location, count, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform3i(&self, location: GLint, v0: GLint, v1: GLint, v2: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLint, GLint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(174)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(174) },
        };
        unsafe { __pfn(location, v0, v1, v2) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform3iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(175)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(175) },
        };
        unsafe { __pfn(location, count, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform4f(&self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLfloat, GLfloat, GLfloat, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(176)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(176) },
        };
        unsafe { __pfn(location, v0, v1, v2, v3) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform4fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(177)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(177) },
        };
        unsafe { __pfn(location, count, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform4i(&self, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLint, GLint, GLint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(178)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(178) },
        };
        unsafe { __pfn(location, v0, v1, v2, v3) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform4iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(179)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(179) },
        };
        unsafe { __pfn(location, count, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn UniformMatrix2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(180)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(180) },
        };
        unsafe { __pfn(location, count, transpose, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn UniformMatrix3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(181)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(181) },
        };
        unsafe { __pfn(location, count, transpose, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn UniformMatrix4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(182)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(182) },
        };
        unsafe { __pfn(location, count, transpose, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn UseProgram(&self, program: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(183)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(183) },
        };
        unsafe { __pfn(program) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ValidateProgram(&self, program: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(184)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(184) },
        };
        unsafe { __pfn(program) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib1f(&self, index: GLuint, x: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(185)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(185) },
        };
        unsafe { __pfn(index, x) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib1fv(&self, index: GLuint, v: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(186)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(186) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib2f(&self, index: GLuint, x: GLfloat, y: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLfloat, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(187)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(187) },
        };
        unsafe { __pfn(index, x, y) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib2fv(&self, index: GLuint, v: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLfloat)> =
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
    pub unsafe fn VertexAttrib3f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(189)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(189) },
        };
        unsafe { __pfn(index, x, y, z) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib3fv(&self, index: GLuint, v: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLfloat)> =
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
    pub unsafe fn VertexAttrib4f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(191)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(191) },
        };
        unsafe { __pfn(index, x, y, z, w) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttrib4fv(&self, index: GLuint, v: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLfloat)> =
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
    pub unsafe fn VertexAttribPointer(&self, index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLsizei, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(193)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(193) },
        };
        unsafe { __pfn(index, size, type_, normalized, stride, pointer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DrawBuffers(&self, n: GLsizei, bufs: *const GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *const GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(194)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(194) },
        };
        unsafe { __pfn(n, bufs) }
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
    pub unsafe fn BindFragDataLocation(&self, program: GLuint, color: GLuint, name: *const GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint, *const GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(202)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(202) },
        };
        unsafe { __pfn(program, color, name) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ClampColor(&self, target: GLenum, clamp: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(203)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(203) },
        };
        unsafe { __pfn(target, clamp) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ColorMaski(&self, index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLboolean, GLboolean, GLboolean, GLboolean)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(204)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(204) },
        };
        unsafe { __pfn(index, r, g, b, a) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Disablei(&self, target: GLenum, index: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(205)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(205) },
        };
        unsafe { __pfn(target, index) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Enablei(&self, target: GLenum, index: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(206)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(206) },
        };
        unsafe { __pfn(target, index) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn EndConditionalRender(&self) {
        let __pfn: Option<unsafe extern "system" fn()> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(207)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(207) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn FramebufferTexture1D(&self, target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(208)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(208) },
        };
        unsafe { __pfn(target, attachment, textarget, texture, level) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn FramebufferTexture3D(&self, target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(209)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(209) },
        };
        unsafe { __pfn(target, attachment, textarget, texture, level, zoffset) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetBooleani_v(&self, target: GLenum, index: GLuint, data: *mut GLboolean) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint, *mut GLboolean)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(210)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(210) },
        };
        unsafe { __pfn(target, index, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetTexParameterIiv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(211)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(211) },
        };
        unsafe { __pfn(target, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetTexParameterIuiv(&self, target: GLenum, pname: GLenum, params: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(212)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(212) },
        };
        unsafe { __pfn(target, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn IsEnabledi(&self, target: GLenum, index: GLuint) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(213)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(213) },
        };
        unsafe { __pfn(target, index) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexParameterIiv(&self, target: GLenum, pname: GLenum, params: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(214)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(214) },
        };
        unsafe { __pfn(target, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexParameterIuiv(&self, target: GLenum, pname: GLenum, params: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(215)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(215) },
        };
        unsafe { __pfn(target, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI1i(&self, index: GLuint, x: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(216)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(216) },
        };
        unsafe { __pfn(index, x) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI1iv(&self, index: GLuint, v: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(217)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(217) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI1ui(&self, index: GLuint, x: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(218)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(218) },
        };
        unsafe { __pfn(index, x) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI1uiv(&self, index: GLuint, v: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(219)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(219) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI2i(&self, index: GLuint, x: GLint, y: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(220)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(220) },
        };
        unsafe { __pfn(index, x, y) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI2iv(&self, index: GLuint, v: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(221)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(221) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI2ui(&self, index: GLuint, x: GLuint, y: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(222)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(222) },
        };
        unsafe { __pfn(index, x, y) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI2uiv(&self, index: GLuint, v: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(223)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(223) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI3i(&self, index: GLuint, x: GLint, y: GLint, z: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLint, GLint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(224)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(224) },
        };
        unsafe { __pfn(index, x, y, z) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI3iv(&self, index: GLuint, v: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(225)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(225) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI3ui(&self, index: GLuint, x: GLuint, y: GLuint, z: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint, GLuint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(226)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(226) },
        };
        unsafe { __pfn(index, x, y, z) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI3uiv(&self, index: GLuint, v: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(227)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(227) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI4bv(&self, index: GLuint, v: *const GLbyte) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLbyte)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(228)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(228) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI4sv(&self, index: GLuint, v: *const GLshort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLshort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(229)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(229) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI4ubv(&self, index: GLuint, v: *const GLubyte) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLubyte)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(230)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(230) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI4usv(&self, index: GLuint, v: *const GLushort) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLushort)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(231)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(231) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BindBufferBase(&self, target: GLenum, index: GLuint, buffer: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(232)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(232) },
        };
        unsafe { __pfn(target, index, buffer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BindBufferRange(&self, target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint, GLuint, GLintptr, GLsizeiptr)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(233)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(233) },
        };
        unsafe { __pfn(target, index, buffer, offset, size) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetIntegeri_v(&self, target: GLenum, index: GLuint, data: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(234)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(234) },
        };
        unsafe { __pfn(target, index, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BindFramebuffer(&self, target: GLenum, framebuffer: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(235)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(235) },
        };
        unsafe { __pfn(target, framebuffer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BindRenderbuffer(&self, target: GLenum, renderbuffer: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(236)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(236) },
        };
        unsafe { __pfn(target, renderbuffer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CheckFramebufferStatus(&self, target: GLenum) -> GLenum {
        let __pfn: Option<unsafe extern "system" fn(GLenum) -> GLenum> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(237)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(237) },
        };
        unsafe { __pfn(target) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DeleteFramebuffers(&self, n: GLsizei, framebuffers: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(238)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(238) },
        };
        unsafe { __pfn(n, framebuffers) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DeleteRenderbuffers(&self, n: GLsizei, renderbuffers: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(239)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(239) },
        };
        unsafe { __pfn(n, renderbuffers) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn FramebufferRenderbuffer(&self, target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLenum, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(240)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(240) },
        };
        unsafe { __pfn(target, attachment, renderbuffertarget, renderbuffer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn FramebufferTexture2D(&self, target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(241)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(241) },
        };
        unsafe { __pfn(target, attachment, textarget, texture, level) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GenFramebuffers(&self, n: GLsizei, framebuffers: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(242)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(242) },
        };
        unsafe { __pfn(n, framebuffers) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GenRenderbuffers(&self, n: GLsizei, renderbuffers: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(243)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(243) },
        };
        unsafe { __pfn(n, renderbuffers) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GenerateMipmap(&self, target: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(244)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(244) },
        };
        unsafe { __pfn(target) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetFramebufferAttachmentParameteriv(&self, target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(245)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(245) },
        };
        unsafe { __pfn(target, attachment, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetRenderbufferParameteriv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(246)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(246) },
        };
        unsafe { __pfn(target, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn IsFramebuffer(&self, framebuffer: GLuint) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLuint) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(247)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(247) },
        };
        unsafe { __pfn(framebuffer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn IsRenderbuffer(&self, renderbuffer: GLuint) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLuint) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(248)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(248) },
        };
        unsafe { __pfn(renderbuffer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn RenderbufferStorage(&self, target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLsizei, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(249)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(249) },
        };
        unsafe { __pfn(target, internalformat, width, height) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BeginTransformFeedback(&self, primitiveMode: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(250)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(250) },
        };
        unsafe { __pfn(primitiveMode) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BindVertexArray(&self, array: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(251)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(251) },
        };
        unsafe { __pfn(array) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BlitFramebuffer(&self, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLbitfield, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(252)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(252) },
        };
        unsafe { __pfn(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ClearBufferfi(&self, buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLfloat, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(253)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(253) },
        };
        unsafe { __pfn(buffer, drawbuffer, depth, stencil) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ClearBufferfv(&self, buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(254)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(254) },
        };
        unsafe { __pfn(buffer, drawbuffer, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ClearBufferiv(&self, buffer: GLenum, drawbuffer: GLint, value: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(255)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(255) },
        };
        unsafe { __pfn(buffer, drawbuffer, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ClearBufferuiv(&self, buffer: GLenum, drawbuffer: GLint, value: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(256)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(256) },
        };
        unsafe { __pfn(buffer, drawbuffer, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DeleteVertexArrays(&self, n: GLsizei, arrays: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(257)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(257) },
        };
        unsafe { __pfn(n, arrays) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn EndTransformFeedback(&self) {
        let __pfn: Option<unsafe extern "system" fn()> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(258)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(258) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn FlushMappedBufferRange(&self, target: GLenum, offset: GLintptr, length: GLsizeiptr) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLintptr, GLsizeiptr)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(259)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(259) },
        };
        unsafe { __pfn(target, offset, length) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn FramebufferTextureLayer(&self, target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLuint, GLint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(260)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(260) },
        };
        unsafe { __pfn(target, attachment, texture, level, layer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GenVertexArrays(&self, n: GLsizei, arrays: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(261)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(261) },
        };
        unsafe { __pfn(n, arrays) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetFragDataLocation(&self, program: GLuint, name: *const GLchar) -> GLint {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLchar) -> GLint> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(262)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(262) },
        };
        unsafe { __pfn(program, name) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetStringi(&self, name: GLenum, index: GLuint) -> *const GLubyte {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint) -> *const GLubyte> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(263)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(263) },
        };
        unsafe { __pfn(name, index) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetTransformFeedbackVarying(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLsizei, *mut GLenum, *mut GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(264)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(264) },
        };
        unsafe { __pfn(program, index, bufSize, length, size, type_, name) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetUniformuiv(&self, program: GLuint, location: GLint, params: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLint, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(265)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(265) },
        };
        unsafe { __pfn(program, location, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetVertexAttribIiv(&self, index: GLuint, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(266)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(266) },
        };
        unsafe { __pfn(index, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetVertexAttribIuiv(&self, index: GLuint, pname: GLenum, params: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(267)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(267) },
        };
        unsafe { __pfn(index, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn IsVertexArray(&self, array: GLuint) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLuint) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(268)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(268) },
        };
        unsafe { __pfn(array) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn MapBufferRange(&self, target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut c_void {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLintptr, GLsizeiptr, GLbitfield) -> *mut c_void> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(269)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(269) },
        };
        unsafe { __pfn(target, offset, length, access) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn RenderbufferStorageMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(270)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(270) },
        };
        unsafe { __pfn(target, samples, internalformat, width, height) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TransformFeedbackVaryings(&self, program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLsizei, *const *const GLchar, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(271)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(271) },
        };
        unsafe { __pfn(program, count, varyings, bufferMode) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform1ui(&self, location: GLint, v0: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(272)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(272) },
        };
        unsafe { __pfn(location, v0) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform1uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(273)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(273) },
        };
        unsafe { __pfn(location, count, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform2ui(&self, location: GLint, v0: GLuint, v1: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLuint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(274)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(274) },
        };
        unsafe { __pfn(location, v0, v1) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform2uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(275)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(275) },
        };
        unsafe { __pfn(location, count, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform3ui(&self, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLuint, GLuint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(276)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(276) },
        };
        unsafe { __pfn(location, v0, v1, v2) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform3uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(277)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(277) },
        };
        unsafe { __pfn(location, count, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform4ui(&self, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLuint, GLuint, GLuint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(278)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(278) },
        };
        unsafe { __pfn(location, v0, v1, v2, v3) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn Uniform4uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLint, GLsizei, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(279)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(279) },
        };
        unsafe { __pfn(location, count, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI4i(&self, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLint, GLint, GLint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(280)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(280) },
        };
        unsafe { __pfn(index, x, y, z, w) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI4iv(&self, index: GLuint, v: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(281)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(281) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI4ui(&self, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint, GLuint, GLuint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(282)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(282) },
        };
        unsafe { __pfn(index, x, y, z, w) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribI4uiv(&self, index: GLuint, v: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(283)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(283) },
        };
        unsafe { __pfn(index, v) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribIPointer(&self, index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLint, GLenum, GLsizei, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(284)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(284) },
        };
        unsafe { __pfn(index, size, type_, stride, pointer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetActiveUniformName(&self, program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(285)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(285) },
        };
        unsafe { __pfn(program, uniformIndex, bufSize, length, uniformName) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn PrimitiveRestartIndex(&self, index: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(286)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(286) },
        };
        unsafe { __pfn(index) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexBuffer(&self, target: GLenum, internalformat: GLenum, buffer: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(287)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(287) },
        };
        unsafe { __pfn(target, internalformat, buffer) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn CopyBufferSubData(&self, readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLintptr, GLintptr, GLsizeiptr)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(288)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(288) },
        };
        unsafe { __pfn(readTarget, writeTarget, readOffset, writeOffset, size) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DrawArraysInstanced(&self, mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLint, GLsizei, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(289)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(289) },
        };
        unsafe { __pfn(mode, first, count, instancecount) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DrawElementsInstanced(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(290)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(290) },
        };
        unsafe { __pfn(mode, count, type_, indices, instancecount) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetActiveUniformBlockName(&self, program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(291)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(291) },
        };
        unsafe { __pfn(program, uniformBlockIndex, bufSize, length, uniformBlockName) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetActiveUniformBlockiv(&self, program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(292)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(292) },
        };
        unsafe { __pfn(program, uniformBlockIndex, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetActiveUniformsiv(&self, program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLsizei, *const GLuint, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(293)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(293) },
        };
        unsafe { __pfn(program, uniformCount, uniformIndices, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetUniformBlockIndex(&self, program: GLuint, uniformBlockName: *const GLchar) -> GLuint {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLchar) -> GLuint> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(294)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(294) },
        };
        unsafe { __pfn(program, uniformBlockName) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetUniformIndices(&self, program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLsizei, *const *const GLchar, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(295)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(295) },
        };
        unsafe { __pfn(program, uniformCount, uniformNames, uniformIndices) }
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
    pub unsafe fn DrawElementsBaseVertex(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, basevertex: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(297)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(297) },
        };
        unsafe { __pfn(mode, count, type_, indices, basevertex) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DrawElementsInstancedBaseVertex(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, basevertex: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(298)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(298) },
        };
        unsafe { __pfn(mode, count, type_, indices, instancecount, basevertex) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DrawRangeElementsBaseVertex(&self, mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const c_void, basevertex: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint, GLuint, GLsizei, GLenum, *const c_void, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(299)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(299) },
        };
        unsafe { __pfn(mode, start, end, count, type_, indices, basevertex) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn FramebufferTexture(&self, target: GLenum, attachment: GLenum, texture: GLuint, level: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLuint, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(300)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(300) },
        };
        unsafe { __pfn(target, attachment, texture, level) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetMultisamplefv(&self, pname: GLenum, index: GLuint, val: *mut GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint, *mut GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(301)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(301) },
        };
        unsafe { __pfn(pname, index, val) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn MultiDrawElementsBaseVertex(&self, mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, drawcount: GLsizei, basevertex: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, *const GLsizei, GLenum, *const *const c_void, GLsizei, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(302)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(302) },
        };
        unsafe { __pfn(mode, count, type_, indices, drawcount, basevertex) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ProvokingVertex(&self, mode: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(303)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(303) },
        };
        unsafe { __pfn(mode) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn SampleMaski(&self, maskNumber: GLuint, mask: GLbitfield) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLbitfield)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(304)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(304) },
        };
        unsafe { __pfn(maskNumber, mask) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexImage2DMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLboolean)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(305)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(305) },
        };
        unsafe { __pfn(target, samples, internalformat, width, height, fixedsamplelocations) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexImage3DMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(306)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(306) },
        };
        unsafe { __pfn(target, samples, internalformat, width, height, depth, fixedsamplelocations) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ClientWaitSync(&self, sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum {
        let __pfn: Option<unsafe extern "system" fn(GLsync, GLbitfield, GLuint64) -> GLenum> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(307)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(307) },
        };
        unsafe { __pfn(sync, flags, timeout) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DeleteSync(&self, sync: GLsync) {
        let __pfn: Option<unsafe extern "system" fn(GLsync)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(308)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(308) },
        };
        unsafe { __pfn(sync) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn FenceSync(&self, condition: GLenum, flags: GLbitfield) -> GLsync {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLbitfield) -> GLsync> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(309)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(309) },
        };
        unsafe { __pfn(condition, flags) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetBufferParameteri64v(&self, target: GLenum, pname: GLenum, params: *mut GLint64) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *mut GLint64)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(310)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(310) },
        };
        unsafe { __pfn(target, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetInteger64i_v(&self, target: GLenum, index: GLuint, data: *mut GLint64) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint, *mut GLint64)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(311)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(311) },
        };
        unsafe { __pfn(target, index, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetInteger64v(&self, pname: GLenum, data: *mut GLint64) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, *mut GLint64)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(312)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(312) },
        };
        unsafe { __pfn(pname, data) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetSynciv(&self, sync: GLsync, pname: GLenum, count: GLsizei, length: *mut GLsizei, values: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLsync, GLenum, GLsizei, *mut GLsizei, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(313)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(313) },
        };
        unsafe { __pfn(sync, pname, count, length, values) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn IsSync(&self, sync: GLsync) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLsync) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(314)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(314) },
        };
        unsafe { __pfn(sync) }
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
    pub unsafe fn GetFragDataIndex(&self, program: GLuint, name: *const GLchar) -> GLint {
        let __pfn: Option<unsafe extern "system" fn(GLuint, *const GLchar) -> GLint> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(317)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(317) },
        };
        unsafe { __pfn(program, name) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetQueryObjecti64v(&self, id: GLuint, pname: GLenum, params: *mut GLint64) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLint64)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(318)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(318) },
        };
        unsafe { __pfn(id, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetQueryObjectui64v(&self, id: GLuint, pname: GLenum, params: *mut GLuint64) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLuint64)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(319)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(319) },
        };
        unsafe { __pfn(id, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetSamplerParameterIiv(&self, sampler: GLuint, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(320)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(320) },
        };
        unsafe { __pfn(sampler, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetSamplerParameterIuiv(&self, sampler: GLuint, pname: GLenum, params: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(321)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(321) },
        };
        unsafe { __pfn(sampler, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn QueryCounter(&self, id: GLuint, target: GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(322)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(322) },
        };
        unsafe { __pfn(id, target) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn SamplerParameterIiv(&self, sampler: GLuint, pname: GLenum, param: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(323)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(323) },
        };
        unsafe { __pfn(sampler, pname, param) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn SamplerParameterIuiv(&self, sampler: GLuint, pname: GLenum, param: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(324)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(324) },
        };
        unsafe { __pfn(sampler, pname, param) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribP1ui(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, GLboolean, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(325)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(325) },
        };
        unsafe { __pfn(index, type_, normalized, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribP1uiv(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(326)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(326) },
        };
        unsafe { __pfn(index, type_, normalized, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribP2ui(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, GLboolean, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(327)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(327) },
        };
        unsafe { __pfn(index, type_, normalized, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribP2uiv(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(328)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(328) },
        };
        unsafe { __pfn(index, type_, normalized, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribP3ui(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, GLboolean, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(329)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(329) },
        };
        unsafe { __pfn(index, type_, normalized, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribP3uiv(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(330)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(330) },
        };
        unsafe { __pfn(index, type_, normalized, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribP4ui(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, GLboolean, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(331)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(331) },
        };
        unsafe { __pfn(index, type_, normalized, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribP4uiv(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(332)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(332) },
        };
        unsafe { __pfn(index, type_, normalized, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BindSampler(&self, unit: GLuint, sampler: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(333)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(333) },
        };
        unsafe { __pfn(unit, sampler) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DeleteSamplers(&self, count: GLsizei, samplers: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(334)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(334) },
        };
        unsafe { __pfn(count, samplers) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GenSamplers(&self, count: GLsizei, samplers: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(335)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(335) },
        };
        unsafe { __pfn(count, samplers) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetSamplerParameterfv(&self, sampler: GLuint, pname: GLenum, params: *mut GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(336)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(336) },
        };
        unsafe { __pfn(sampler, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetSamplerParameteriv(&self, sampler: GLuint, pname: GLenum, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(337)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(337) },
        };
        unsafe { __pfn(sampler, pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn IsSampler(&self, sampler: GLuint) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLuint) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(338)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(338) },
        };
        unsafe { __pfn(sampler) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn SamplerParameterf(&self, sampler: GLuint, pname: GLenum, param: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(339)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(339) },
        };
        unsafe { __pfn(sampler, pname, param) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn SamplerParameterfv(&self, sampler: GLuint, pname: GLenum, param: *const GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *const GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(340)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(340) },
        };
        unsafe { __pfn(sampler, pname, param) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn SamplerParameteri(&self, sampler: GLuint, pname: GLenum, param: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(341)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(341) },
        };
        unsafe { __pfn(sampler, pname, param) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn SamplerParameteriv(&self, sampler: GLuint, pname: GLenum, param: *const GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *const GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(342)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(342) },
        };
        unsafe { __pfn(sampler, pname, param) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn VertexAttribDivisor(&self, index: GLuint, divisor: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(343)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(343) },
        };
        unsafe { __pfn(index, divisor) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ClearDepthf(&self, d: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(344)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(344) },
        };
        unsafe { __pfn(d) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DepthRangef(&self, n: GLfloat, f: GLfloat) {
        let __pfn: Option<unsafe extern "system" fn(GLfloat, GLfloat)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(345)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(345) },
        };
        unsafe { __pfn(n, f) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetShaderPrecisionFormat(&self, shadertype: GLenum, precisiontype: GLenum, range: *mut GLint, precision: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, *mut GLint, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(346)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(346) },
        };
        unsafe { __pfn(shadertype, precisiontype, range, precision) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ReleaseShaderCompiler(&self) {
        let __pfn: Option<unsafe extern "system" fn()> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(347)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(347) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ShaderBinary(&self, count: GLsizei, shaders: *const GLuint, binaryFormat: GLenum, binary: *const c_void, length: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *const GLuint, GLenum, *const c_void, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(348)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(348) },
        };
        unsafe { __pfn(count, shaders, binaryFormat, binary, length) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn BindTransformFeedback(&self, target: GLenum, id: GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(349)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(349) },
        };
        unsafe { __pfn(target, id) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DeleteTransformFeedbacks(&self, n: GLsizei, ids: *const GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *const GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(350)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(350) },
        };
        unsafe { __pfn(n, ids) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GenTransformFeedbacks(&self, n: GLsizei, ids: *mut GLuint) {
        let __pfn: Option<unsafe extern "system" fn(GLsizei, *mut GLuint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(351)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(351) },
        };
        unsafe { __pfn(n, ids) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetInternalformativ(&self, target: GLenum, internalformat: GLenum, pname: GLenum, count: GLsizei, params: *mut GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(352)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(352) },
        };
        unsafe { __pfn(target, internalformat, pname, count, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetProgramBinary(&self, program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLenum, *mut c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(353)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(353) },
        };
        unsafe { __pfn(program, bufSize, length, binaryFormat, binary) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn InvalidateFramebuffer(&self, target: GLenum, numAttachments: GLsizei, attachments: *const GLenum) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLsizei, *const GLenum)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(354)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(354) },
        };
        unsafe { __pfn(target, numAttachments, attachments) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn InvalidateSubFramebuffer(&self, target: GLenum, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLsizei, *const GLenum, GLint, GLint, GLsizei, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(355)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(355) },
        };
        unsafe { __pfn(target, numAttachments, attachments, x, y, width, height) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn IsTransformFeedback(&self, id: GLuint) -> GLboolean {
        let __pfn: Option<unsafe extern "system" fn(GLuint) -> GLboolean> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(356)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(356) },
        };
        unsafe { __pfn(id) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn PauseTransformFeedback(&self) {
        let __pfn: Option<unsafe extern "system" fn()> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(357)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(357) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ProgramBinary(&self, program: GLuint, binaryFormat: GLenum, binary: *const c_void, length: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, *const c_void, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(358)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(358) },
        };
        unsafe { __pfn(program, binaryFormat, binary, length) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ProgramParameteri(&self, program: GLuint, pname: GLenum, value: GLint) {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLenum, GLint)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(359)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(359) },
        };
        unsafe { __pfn(program, pname, value) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ResumeTransformFeedback(&self) {
        let __pfn: Option<unsafe extern "system" fn()> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(360)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(360) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexStorage2D(&self, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(361)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(361) },
        };
        unsafe { __pfn(target, levels, internalformat, width, height) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn TexStorage3D(&self, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(362)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(362) },
        };
        unsafe { __pfn(target, levels, internalformat, width, height, depth) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetPointerv(&self, pname: GLenum, params: *mut *mut c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, *mut *mut c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(363)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(363) },
        };
        unsafe { __pfn(pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DebugMessageCallback(&self, callback: GLDEBUGPROC, userParam: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLDEBUGPROC, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(364)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(364) },
        };
        unsafe { __pfn(callback, userParam) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DebugMessageCallbackKHR(&self, callback: GLDEBUGPROCKHR, userParam: *const c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLDEBUGPROCKHR, *const c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(365)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(365) },
        };
        unsafe { __pfn(callback, userParam) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DebugMessageControl(&self, source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *const GLuint, GLboolean)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(366)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(366) },
        };
        unsafe { __pfn(source, type_, severity, count, ids, enabled) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DebugMessageControlKHR(&self, source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *const GLuint, GLboolean)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(367)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(367) },
        };
        unsafe { __pfn(source, type_, severity, count, ids, enabled) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DebugMessageInsert(&self, source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLuint, GLenum, GLsizei, *const GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(368)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(368) },
        };
        unsafe { __pfn(source, type_, id, severity, length, buf) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn DebugMessageInsertKHR(&self, source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLenum, GLuint, GLenum, GLsizei, *const GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(369)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(369) },
        };
        unsafe { __pfn(source, type_, id, severity, length, buf) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetDebugMessageLog(&self, count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLsizei, *mut GLenum, *mut GLenum, *mut GLuint, *mut GLenum, *mut GLsizei, *mut GLchar) -> GLuint> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(370)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(370) },
        };
        unsafe { __pfn(count, bufSize, sources, types, ids, severities, lengths, messageLog) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetDebugMessageLogKHR(&self, count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint {
        let __pfn: Option<unsafe extern "system" fn(GLuint, GLsizei, *mut GLenum, *mut GLenum, *mut GLuint, *mut GLenum, *mut GLsizei, *mut GLchar) -> GLuint> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(371)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(371) },
        };
        unsafe { __pfn(count, bufSize, sources, types, ids, severities, lengths, messageLog) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetObjectLabel(&self, identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(372)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(372) },
        };
        unsafe { __pfn(identifier, name, bufSize, length, label) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetObjectLabelKHR(&self, identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(373)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(373) },
        };
        unsafe { __pfn(identifier, name, bufSize, length, label) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetObjectPtrLabel(&self, ptr: *const c_void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) {
        let __pfn: Option<unsafe extern "system" fn(*const c_void, GLsizei, *mut GLsizei, *mut GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(374)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(374) },
        };
        unsafe { __pfn(ptr, bufSize, length, label) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetObjectPtrLabelKHR(&self, ptr: *const c_void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) {
        let __pfn: Option<unsafe extern "system" fn(*const c_void, GLsizei, *mut GLsizei, *mut GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(375)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(375) },
        };
        unsafe { __pfn(ptr, bufSize, length, label) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn GetPointervKHR(&self, pname: GLenum, params: *mut *mut c_void) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, *mut *mut c_void)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(376)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(376) },
        };
        unsafe { __pfn(pname, params) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ObjectLabel(&self, identifier: GLenum, name: GLuint, length: GLsizei, label: *const GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(377)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(377) },
        };
        unsafe { __pfn(identifier, name, length, label) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ObjectLabelKHR(&self, identifier: GLenum, name: GLuint, length: GLsizei, label: *const GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(378)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(378) },
        };
        unsafe { __pfn(identifier, name, length, label) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ObjectPtrLabel(&self, ptr: *const c_void, length: GLsizei, label: *const GLchar) {
        let __pfn: Option<unsafe extern "system" fn(*const c_void, GLsizei, *const GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(379)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(379) },
        };
        unsafe { __pfn(ptr, length, label) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn ObjectPtrLabelKHR(&self, ptr: *const c_void, length: GLsizei, label: *const GLchar) {
        let __pfn: Option<unsafe extern "system" fn(*const c_void, GLsizei, *const GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(380)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(380) },
        };
        unsafe { __pfn(ptr, length, label) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn PopDebugGroup(&self) {
        let __pfn: Option<unsafe extern "system" fn()> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(381)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(381) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn PopDebugGroupKHR(&self) {
        let __pfn: Option<unsafe extern "system" fn()> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(382)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(382) },
        };
        unsafe { __pfn() }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn PushDebugGroup(&self, source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(383)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(383) },
        };
        unsafe { __pfn(source, id, length, message) }
    }

    /// # Safety
    /// The context must be loaded and current; see [`Gl::load_gl`].
    #[inline]
    pub unsafe fn PushDebugGroupKHR(&self, source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar) {
        let __pfn: Option<unsafe extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar)> =
            unsafe { core::mem::transmute(*self.pfns.get_unchecked(384)) };
        let __pfn = match __pfn {
            Some(__f) => __f,
            None => unsafe { __missing(384) },
        };
        unsafe { __pfn(source, id, length, message) }
    }

    /// Whether the driver advertises `GL_KHR_debug`.
    #[inline]
    pub fn KHR_debug(&self) -> bool {
        self.ext[0]
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

    /// Whether the driver supports `GL_ES_VERSION_2_0`.
    #[inline]
    pub fn ES_VERSION_2_0(&self) -> bool {
        self.feat[12]
    }

    /// Whether the driver supports `GL_ES_VERSION_3_0`.
    #[inline]
    pub fn ES_VERSION_3_0(&self) -> bool {
        self.feat[13]
    }
}

// ── Global context (--mx-global) ───────────────────────────
impl Gl {
    /// Every PFN null, every flag false, version 0.
    const EMPTY: Gl = Gl {
        pfns: [core::ptr::null(); COMMAND_COUNT],
        feat: [false; FEATURE_COUNT],
        ext: [false; EXT_COUNT],
        version: 0,
    };
}

struct GlobalCell(core::cell::UnsafeCell<Gl>);
// SAFETY: written only by `init_global` (whose contract forbids racing
// it against any other access), read-only afterwards.
unsafe impl Sync for GlobalCell {}

static GLOBAL: GlobalCell = GlobalCell(core::cell::UnsafeCell::new(Gl::EMPTY));

/// Install `gl` as the process-global context behind the free functions.
///
/// # Safety
/// Must complete before — and must never run concurrently with — any
/// other use of the global (free functions, [`global`]).  The write is
/// unsynchronized, so publish it to other threads through an ordinary
/// happens-before edge (spawn the render thread after this returns, send
/// a message, etc.), exactly as with the C loader's global context.
pub unsafe fn init_global(gl: Gl) {
    unsafe { *GLOBAL.0.get() = gl };
}

/// The process-global context installed by [`init_global`] (a zeroed
/// context beforehand: flags read false, dispatch is a contract
/// violation).
#[inline]
pub fn global() -> &'static Gl {
    // SAFETY: no `&mut` can exist outside `init_global`, whose contract
    // forbids concurrent calls to this.
    unsafe { &*GLOBAL.0.get() }
}

/// Load the gl API (see [`Gl::load_gl`]) and install it as the
/// process-global context.
///
/// # Safety
/// [`Gl::load_gl`]'s contract plus [`init_global`]'s.
pub unsafe fn load_gl_global(
    loader: impl FnMut(&CStr) -> *const c_void,
) -> Result<(), LoadError> {
    unsafe { init_global(Gl::load_gl(loader)?) };
    Ok(())
}

/// Load the gles2 API (see [`Gl::load_gles2`]) and install it as the
/// process-global context.
///
/// # Safety
/// [`Gl::load_gles2`]'s contract plus [`init_global`]'s.
pub unsafe fn load_gles2_global(
    loader: impl FnMut(&CStr) -> *const c_void,
) -> Result<(), LoadError> {
    unsafe { init_global(Gl::load_gles2(loader)?) };
    Ok(())
}

/// # Safety
/// See [`Gl::ClearDepth`]; the global context must be initialized.
#[inline]
pub unsafe fn ClearDepth(depth: GLdouble) {
    unsafe { global().ClearDepth(depth) }
}

/// # Safety
/// See [`Gl::DepthRange`]; the global context must be initialized.
#[inline]
pub unsafe fn DepthRange(n: GLdouble, f: GLdouble) {
    unsafe { global().DepthRange(n, f) }
}

/// # Safety
/// See [`Gl::DrawBuffer`]; the global context must be initialized.
#[inline]
pub unsafe fn DrawBuffer(buf: GLenum) {
    unsafe { global().DrawBuffer(buf) }
}

/// # Safety
/// See [`Gl::GetDoublev`]; the global context must be initialized.
#[inline]
pub unsafe fn GetDoublev(pname: GLenum, data: *mut GLdouble) {
    unsafe { global().GetDoublev(pname, data) }
}

/// # Safety
/// See [`Gl::GetTexImage`]; the global context must be initialized.
#[inline]
pub unsafe fn GetTexImage(target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut c_void) {
    unsafe { global().GetTexImage(target, level, format, type_, pixels) }
}

/// # Safety
/// See [`Gl::GetTexLevelParameterfv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetTexLevelParameterfv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat) {
    unsafe { global().GetTexLevelParameterfv(target, level, pname, params) }
}

/// # Safety
/// See [`Gl::GetTexLevelParameteriv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetTexLevelParameteriv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLint) {
    unsafe { global().GetTexLevelParameteriv(target, level, pname, params) }
}

/// # Safety
/// See [`Gl::LogicOp`]; the global context must be initialized.
#[inline]
pub unsafe fn LogicOp(opcode: GLenum) {
    unsafe { global().LogicOp(opcode) }
}

/// # Safety
/// See [`Gl::PixelStoref`]; the global context must be initialized.
#[inline]
pub unsafe fn PixelStoref(pname: GLenum, param: GLfloat) {
    unsafe { global().PixelStoref(pname, param) }
}

/// # Safety
/// See [`Gl::PointSize`]; the global context must be initialized.
#[inline]
pub unsafe fn PointSize(size: GLfloat) {
    unsafe { global().PointSize(size) }
}

/// # Safety
/// See [`Gl::PolygonMode`]; the global context must be initialized.
#[inline]
pub unsafe fn PolygonMode(face: GLenum, mode: GLenum) {
    unsafe { global().PolygonMode(face, mode) }
}

/// # Safety
/// See [`Gl::TexImage1D`]; the global context must be initialized.
#[inline]
pub unsafe fn TexImage1D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) {
    unsafe { global().TexImage1D(target, level, internalformat, width, border, format, type_, pixels) }
}

/// # Safety
/// See [`Gl::BlendFunc`]; the global context must be initialized.
#[inline]
pub unsafe fn BlendFunc(sfactor: GLenum, dfactor: GLenum) {
    unsafe { global().BlendFunc(sfactor, dfactor) }
}

/// # Safety
/// See [`Gl::Clear`]; the global context must be initialized.
#[inline]
pub unsafe fn Clear(mask: GLbitfield) {
    unsafe { global().Clear(mask) }
}

/// # Safety
/// See [`Gl::ClearColor`]; the global context must be initialized.
#[inline]
pub unsafe fn ClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
    unsafe { global().ClearColor(red, green, blue, alpha) }
}

/// # Safety
/// See [`Gl::ClearStencil`]; the global context must be initialized.
#[inline]
pub unsafe fn ClearStencil(s: GLint) {
    unsafe { global().ClearStencil(s) }
}

/// # Safety
/// See [`Gl::ColorMask`]; the global context must be initialized.
#[inline]
pub unsafe fn ColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) {
    unsafe { global().ColorMask(red, green, blue, alpha) }
}

/// # Safety
/// See [`Gl::CullFace`]; the global context must be initialized.
#[inline]
pub unsafe fn CullFace(mode: GLenum) {
    unsafe { global().CullFace(mode) }
}

/// # Safety
/// See [`Gl::DepthFunc`]; the global context must be initialized.
#[inline]
pub unsafe fn DepthFunc(func: GLenum) {
    unsafe { global().DepthFunc(func) }
}

/// # Safety
/// See [`Gl::DepthMask`]; the global context must be initialized.
#[inline]
pub unsafe fn DepthMask(flag: GLboolean) {
    unsafe { global().DepthMask(flag) }
}

/// # Safety
/// See [`Gl::Disable`]; the global context must be initialized.
#[inline]
pub unsafe fn Disable(cap: GLenum) {
    unsafe { global().Disable(cap) }
}

/// # Safety
/// See [`Gl::Enable`]; the global context must be initialized.
#[inline]
pub unsafe fn Enable(cap: GLenum) {
    unsafe { global().Enable(cap) }
}

/// # Safety
/// See [`Gl::Finish`]; the global context must be initialized.
#[inline]
pub unsafe fn Finish() {
    unsafe { global().Finish() }
}

/// # Safety
/// See [`Gl::Flush`]; the global context must be initialized.
#[inline]
pub unsafe fn Flush() {
    unsafe { global().Flush() }
}

/// # Safety
/// See [`Gl::FrontFace`]; the global context must be initialized.
#[inline]
pub unsafe fn FrontFace(mode: GLenum) {
    unsafe { global().FrontFace(mode) }
}

/// # Safety
/// See [`Gl::GetBooleanv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetBooleanv(pname: GLenum, data: *mut GLboolean) {
    unsafe { global().GetBooleanv(pname, data) }
}

/// # Safety
/// See [`Gl::GetError`]; the global context must be initialized.
#[inline]
pub unsafe fn GetError() -> GLenum {
    unsafe { global().GetError() }
}

/// # Safety
/// See [`Gl::GetFloatv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetFloatv(pname: GLenum, data: *mut GLfloat) {
    unsafe { global().GetFloatv(pname, data) }
}

/// # Safety
/// See [`Gl::GetIntegerv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetIntegerv(pname: GLenum, data: *mut GLint) {
    unsafe { global().GetIntegerv(pname, data) }
}

/// # Safety
/// See [`Gl::GetString`]; the global context must be initialized.
#[inline]
pub unsafe fn GetString(name: GLenum) -> *const GLubyte {
    unsafe { global().GetString(name) }
}

/// # Safety
/// See [`Gl::GetTexParameterfv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat) {
    unsafe { global().GetTexParameterfv(target, pname, params) }
}

/// # Safety
/// See [`Gl::GetTexParameteriv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    unsafe { global().GetTexParameteriv(target, pname, params) }
}

/// # Safety
/// See [`Gl::Hint`]; the global context must be initialized.
#[inline]
pub unsafe fn Hint(target: GLenum, mode: GLenum) {
    unsafe { global().Hint(target, mode) }
}

/// # Safety
/// See [`Gl::IsEnabled`]; the global context must be initialized.
#[inline]
pub unsafe fn IsEnabled(cap: GLenum) -> GLboolean {
    unsafe { global().IsEnabled(cap) }
}

/// # Safety
/// See [`Gl::LineWidth`]; the global context must be initialized.
#[inline]
pub unsafe fn LineWidth(width: GLfloat) {
    unsafe { global().LineWidth(width) }
}

/// # Safety
/// See [`Gl::PixelStorei`]; the global context must be initialized.
#[inline]
pub unsafe fn PixelStorei(pname: GLenum, param: GLint) {
    unsafe { global().PixelStorei(pname, param) }
}

/// # Safety
/// See [`Gl::ReadPixels`]; the global context must be initialized.
#[inline]
pub unsafe fn ReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut c_void) {
    unsafe { global().ReadPixels(x, y, width, height, format, type_, pixels) }
}

/// # Safety
/// See [`Gl::Scissor`]; the global context must be initialized.
#[inline]
pub unsafe fn Scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    unsafe { global().Scissor(x, y, width, height) }
}

/// # Safety
/// See [`Gl::StencilFunc`]; the global context must be initialized.
#[inline]
pub unsafe fn StencilFunc(func: GLenum, ref_: GLint, mask: GLuint) {
    unsafe { global().StencilFunc(func, ref_, mask) }
}

/// # Safety
/// See [`Gl::StencilMask`]; the global context must be initialized.
#[inline]
pub unsafe fn StencilMask(mask: GLuint) {
    unsafe { global().StencilMask(mask) }
}

/// # Safety
/// See [`Gl::StencilOp`]; the global context must be initialized.
#[inline]
pub unsafe fn StencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum) {
    unsafe { global().StencilOp(fail, zfail, zpass) }
}

/// # Safety
/// See [`Gl::TexImage2D`]; the global context must be initialized.
#[inline]
pub unsafe fn TexImage2D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) {
    unsafe { global().TexImage2D(target, level, internalformat, width, height, border, format, type_, pixels) }
}

/// # Safety
/// See [`Gl::TexParameterf`]; the global context must be initialized.
#[inline]
pub unsafe fn TexParameterf(target: GLenum, pname: GLenum, param: GLfloat) {
    unsafe { global().TexParameterf(target, pname, param) }
}

/// # Safety
/// See [`Gl::TexParameterfv`]; the global context must be initialized.
#[inline]
pub unsafe fn TexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat) {
    unsafe { global().TexParameterfv(target, pname, params) }
}

/// # Safety
/// See [`Gl::TexParameteri`]; the global context must be initialized.
#[inline]
pub unsafe fn TexParameteri(target: GLenum, pname: GLenum, param: GLint) {
    unsafe { global().TexParameteri(target, pname, param) }
}

/// # Safety
/// See [`Gl::TexParameteriv`]; the global context must be initialized.
#[inline]
pub unsafe fn TexParameteriv(target: GLenum, pname: GLenum, params: *const GLint) {
    unsafe { global().TexParameteriv(target, pname, params) }
}

/// # Safety
/// See [`Gl::Viewport`]; the global context must be initialized.
#[inline]
pub unsafe fn Viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    unsafe { global().Viewport(x, y, width, height) }
}

/// # Safety
/// See [`Gl::ReadBuffer`]; the global context must be initialized.
#[inline]
pub unsafe fn ReadBuffer(src: GLenum) {
    unsafe { global().ReadBuffer(src) }
}

/// # Safety
/// See [`Gl::CopyTexImage1D`]; the global context must be initialized.
#[inline]
pub unsafe fn CopyTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint) {
    unsafe { global().CopyTexImage1D(target, level, internalformat, x, y, width, border) }
}

/// # Safety
/// See [`Gl::CopyTexSubImage1D`]; the global context must be initialized.
#[inline]
pub unsafe fn CopyTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) {
    unsafe { global().CopyTexSubImage1D(target, level, xoffset, x, y, width) }
}

/// # Safety
/// See [`Gl::TexSubImage1D`]; the global context must be initialized.
#[inline]
pub unsafe fn TexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) {
    unsafe { global().TexSubImage1D(target, level, xoffset, width, format, type_, pixels) }
}

/// # Safety
/// See [`Gl::BindTexture`]; the global context must be initialized.
#[inline]
pub unsafe fn BindTexture(target: GLenum, texture: GLuint) {
    unsafe { global().BindTexture(target, texture) }
}

/// # Safety
/// See [`Gl::CopyTexImage2D`]; the global context must be initialized.
#[inline]
pub unsafe fn CopyTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) {
    unsafe { global().CopyTexImage2D(target, level, internalformat, x, y, width, height, border) }
}

/// # Safety
/// See [`Gl::CopyTexSubImage2D`]; the global context must be initialized.
#[inline]
pub unsafe fn CopyTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    unsafe { global().CopyTexSubImage2D(target, level, xoffset, yoffset, x, y, width, height) }
}

/// # Safety
/// See [`Gl::DeleteTextures`]; the global context must be initialized.
#[inline]
pub unsafe fn DeleteTextures(n: GLsizei, textures: *const GLuint) {
    unsafe { global().DeleteTextures(n, textures) }
}

/// # Safety
/// See [`Gl::DrawArrays`]; the global context must be initialized.
#[inline]
pub unsafe fn DrawArrays(mode: GLenum, first: GLint, count: GLsizei) {
    unsafe { global().DrawArrays(mode, first, count) }
}

/// # Safety
/// See [`Gl::DrawElements`]; the global context must be initialized.
#[inline]
pub unsafe fn DrawElements(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void) {
    unsafe { global().DrawElements(mode, count, type_, indices) }
}

/// # Safety
/// See [`Gl::GenTextures`]; the global context must be initialized.
#[inline]
pub unsafe fn GenTextures(n: GLsizei, textures: *mut GLuint) {
    unsafe { global().GenTextures(n, textures) }
}

/// # Safety
/// See [`Gl::IsTexture`]; the global context must be initialized.
#[inline]
pub unsafe fn IsTexture(texture: GLuint) -> GLboolean {
    unsafe { global().IsTexture(texture) }
}

/// # Safety
/// See [`Gl::PolygonOffset`]; the global context must be initialized.
#[inline]
pub unsafe fn PolygonOffset(factor: GLfloat, units: GLfloat) {
    unsafe { global().PolygonOffset(factor, units) }
}

/// # Safety
/// See [`Gl::TexSubImage2D`]; the global context must be initialized.
#[inline]
pub unsafe fn TexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) {
    unsafe { global().TexSubImage2D(target, level, xoffset, yoffset, width, height, format, type_, pixels) }
}

/// # Safety
/// See [`Gl::CopyTexSubImage3D`]; the global context must be initialized.
#[inline]
pub unsafe fn CopyTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    unsafe { global().CopyTexSubImage3D(target, level, xoffset, yoffset, zoffset, x, y, width, height) }
}

/// # Safety
/// See [`Gl::DrawRangeElements`]; the global context must be initialized.
#[inline]
pub unsafe fn DrawRangeElements(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const c_void) {
    unsafe { global().DrawRangeElements(mode, start, end, count, type_, indices) }
}

/// # Safety
/// See [`Gl::TexImage3D`]; the global context must be initialized.
#[inline]
pub unsafe fn TexImage3D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) {
    unsafe { global().TexImage3D(target, level, internalformat, width, height, depth, border, format, type_, pixels) }
}

/// # Safety
/// See [`Gl::TexSubImage3D`]; the global context must be initialized.
#[inline]
pub unsafe fn TexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) {
    unsafe { global().TexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels) }
}

/// # Safety
/// See [`Gl::CompressedTexImage1D`]; the global context must be initialized.
#[inline]
pub unsafe fn CompressedTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) {
    unsafe { global().CompressedTexImage1D(target, level, internalformat, width, border, imageSize, data) }
}

/// # Safety
/// See [`Gl::CompressedTexSubImage1D`]; the global context must be initialized.
#[inline]
pub unsafe fn CompressedTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) {
    unsafe { global().CompressedTexSubImage1D(target, level, xoffset, width, format, imageSize, data) }
}

/// # Safety
/// See [`Gl::GetCompressedTexImage`]; the global context must be initialized.
#[inline]
pub unsafe fn GetCompressedTexImage(target: GLenum, level: GLint, img: *mut c_void) {
    unsafe { global().GetCompressedTexImage(target, level, img) }
}

/// # Safety
/// See [`Gl::ActiveTexture`]; the global context must be initialized.
#[inline]
pub unsafe fn ActiveTexture(texture: GLenum) {
    unsafe { global().ActiveTexture(texture) }
}

/// # Safety
/// See [`Gl::CompressedTexImage2D`]; the global context must be initialized.
#[inline]
pub unsafe fn CompressedTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) {
    unsafe { global().CompressedTexImage2D(target, level, internalformat, width, height, border, imageSize, data) }
}

/// # Safety
/// See [`Gl::CompressedTexSubImage2D`]; the global context must be initialized.
#[inline]
pub unsafe fn CompressedTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) {
    unsafe { global().CompressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, imageSize, data) }
}

/// # Safety
/// See [`Gl::SampleCoverage`]; the global context must be initialized.
#[inline]
pub unsafe fn SampleCoverage(value: GLfloat, invert: GLboolean) {
    unsafe { global().SampleCoverage(value, invert) }
}

/// # Safety
/// See [`Gl::CompressedTexImage3D`]; the global context must be initialized.
#[inline]
pub unsafe fn CompressedTexImage3D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) {
    unsafe { global().CompressedTexImage3D(target, level, internalformat, width, height, depth, border, imageSize, data) }
}

/// # Safety
/// See [`Gl::CompressedTexSubImage3D`]; the global context must be initialized.
#[inline]
pub unsafe fn CompressedTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) {
    unsafe { global().CompressedTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data) }
}

/// # Safety
/// See [`Gl::MultiDrawArrays`]; the global context must be initialized.
#[inline]
pub unsafe fn MultiDrawArrays(mode: GLenum, first: *const GLint, count: *const GLsizei, drawcount: GLsizei) {
    unsafe { global().MultiDrawArrays(mode, first, count, drawcount) }
}

/// # Safety
/// See [`Gl::MultiDrawElements`]; the global context must be initialized.
#[inline]
pub unsafe fn MultiDrawElements(mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, drawcount: GLsizei) {
    unsafe { global().MultiDrawElements(mode, count, type_, indices, drawcount) }
}

/// # Safety
/// See [`Gl::PointParameterf`]; the global context must be initialized.
#[inline]
pub unsafe fn PointParameterf(pname: GLenum, param: GLfloat) {
    unsafe { global().PointParameterf(pname, param) }
}

/// # Safety
/// See [`Gl::PointParameterfv`]; the global context must be initialized.
#[inline]
pub unsafe fn PointParameterfv(pname: GLenum, params: *const GLfloat) {
    unsafe { global().PointParameterfv(pname, params) }
}

/// # Safety
/// See [`Gl::PointParameteri`]; the global context must be initialized.
#[inline]
pub unsafe fn PointParameteri(pname: GLenum, param: GLint) {
    unsafe { global().PointParameteri(pname, param) }
}

/// # Safety
/// See [`Gl::PointParameteriv`]; the global context must be initialized.
#[inline]
pub unsafe fn PointParameteriv(pname: GLenum, params: *const GLint) {
    unsafe { global().PointParameteriv(pname, params) }
}

/// # Safety
/// See [`Gl::BlendColor`]; the global context must be initialized.
#[inline]
pub unsafe fn BlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
    unsafe { global().BlendColor(red, green, blue, alpha) }
}

/// # Safety
/// See [`Gl::BlendEquation`]; the global context must be initialized.
#[inline]
pub unsafe fn BlendEquation(mode: GLenum) {
    unsafe { global().BlendEquation(mode) }
}

/// # Safety
/// See [`Gl::BlendFuncSeparate`]; the global context must be initialized.
#[inline]
pub unsafe fn BlendFuncSeparate(sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum) {
    unsafe { global().BlendFuncSeparate(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha) }
}

/// # Safety
/// See [`Gl::GetBufferSubData`]; the global context must be initialized.
#[inline]
pub unsafe fn GetBufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut c_void) {
    unsafe { global().GetBufferSubData(target, offset, size, data) }
}

/// # Safety
/// See [`Gl::GetQueryObjectiv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetQueryObjectiv(id: GLuint, pname: GLenum, params: *mut GLint) {
    unsafe { global().GetQueryObjectiv(id, pname, params) }
}

/// # Safety
/// See [`Gl::MapBuffer`]; the global context must be initialized.
#[inline]
pub unsafe fn MapBuffer(target: GLenum, access: GLenum) -> *mut c_void {
    unsafe { global().MapBuffer(target, access) }
}

/// # Safety
/// See [`Gl::BindBuffer`]; the global context must be initialized.
#[inline]
pub unsafe fn BindBuffer(target: GLenum, buffer: GLuint) {
    unsafe { global().BindBuffer(target, buffer) }
}

/// # Safety
/// See [`Gl::BufferData`]; the global context must be initialized.
#[inline]
pub unsafe fn BufferData(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum) {
    unsafe { global().BufferData(target, size, data, usage) }
}

/// # Safety
/// See [`Gl::BufferSubData`]; the global context must be initialized.
#[inline]
pub unsafe fn BufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const c_void) {
    unsafe { global().BufferSubData(target, offset, size, data) }
}

/// # Safety
/// See [`Gl::DeleteBuffers`]; the global context must be initialized.
#[inline]
pub unsafe fn DeleteBuffers(n: GLsizei, buffers: *const GLuint) {
    unsafe { global().DeleteBuffers(n, buffers) }
}

/// # Safety
/// See [`Gl::GenBuffers`]; the global context must be initialized.
#[inline]
pub unsafe fn GenBuffers(n: GLsizei, buffers: *mut GLuint) {
    unsafe { global().GenBuffers(n, buffers) }
}

/// # Safety
/// See [`Gl::GetBufferParameteriv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    unsafe { global().GetBufferParameteriv(target, pname, params) }
}

/// # Safety
/// See [`Gl::IsBuffer`]; the global context must be initialized.
#[inline]
pub unsafe fn IsBuffer(buffer: GLuint) -> GLboolean {
    unsafe { global().IsBuffer(buffer) }
}

/// # Safety
/// See [`Gl::BeginQuery`]; the global context must be initialized.
#[inline]
pub unsafe fn BeginQuery(target: GLenum, id: GLuint) {
    unsafe { global().BeginQuery(target, id) }
}

/// # Safety
/// See [`Gl::DeleteQueries`]; the global context must be initialized.
#[inline]
pub unsafe fn DeleteQueries(n: GLsizei, ids: *const GLuint) {
    unsafe { global().DeleteQueries(n, ids) }
}

/// # Safety
/// See [`Gl::EndQuery`]; the global context must be initialized.
#[inline]
pub unsafe fn EndQuery(target: GLenum) {
    unsafe { global().EndQuery(target) }
}

/// # Safety
/// See [`Gl::GenQueries`]; the global context must be initialized.
#[inline]
pub unsafe fn GenQueries(n: GLsizei, ids: *mut GLuint) {
    unsafe { global().GenQueries(n, ids) }
}

/// # Safety
/// See [`Gl::GetBufferPointerv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetBufferPointerv(target: GLenum, pname: GLenum, params: *mut *mut c_void) {
    unsafe { global().GetBufferPointerv(target, pname, params) }
}

/// # Safety
/// See [`Gl::GetQueryObjectuiv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetQueryObjectuiv(id: GLuint, pname: GLenum, params: *mut GLuint) {
    unsafe { global().GetQueryObjectuiv(id, pname, params) }
}

/// # Safety
/// See [`Gl::GetQueryiv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetQueryiv(target: GLenum, pname: GLenum, params: *mut GLint) {
    unsafe { global().GetQueryiv(target, pname, params) }
}

/// # Safety
/// See [`Gl::IsQuery`]; the global context must be initialized.
#[inline]
pub unsafe fn IsQuery(id: GLuint) -> GLboolean {
    unsafe { global().IsQuery(id) }
}

/// # Safety
/// See [`Gl::UnmapBuffer`]; the global context must be initialized.
#[inline]
pub unsafe fn UnmapBuffer(target: GLenum) -> GLboolean {
    unsafe { global().UnmapBuffer(target) }
}

/// # Safety
/// See [`Gl::GetVertexAttribdv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetVertexAttribdv(index: GLuint, pname: GLenum, params: *mut GLdouble) {
    unsafe { global().GetVertexAttribdv(index, pname, params) }
}

/// # Safety
/// See [`Gl::VertexAttrib1d`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib1d(index: GLuint, x: GLdouble) {
    unsafe { global().VertexAttrib1d(index, x) }
}

/// # Safety
/// See [`Gl::VertexAttrib1dv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib1dv(index: GLuint, v: *const GLdouble) {
    unsafe { global().VertexAttrib1dv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttrib1s`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib1s(index: GLuint, x: GLshort) {
    unsafe { global().VertexAttrib1s(index, x) }
}

/// # Safety
/// See [`Gl::VertexAttrib1sv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib1sv(index: GLuint, v: *const GLshort) {
    unsafe { global().VertexAttrib1sv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttrib2d`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib2d(index: GLuint, x: GLdouble, y: GLdouble) {
    unsafe { global().VertexAttrib2d(index, x, y) }
}

/// # Safety
/// See [`Gl::VertexAttrib2dv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib2dv(index: GLuint, v: *const GLdouble) {
    unsafe { global().VertexAttrib2dv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttrib2s`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib2s(index: GLuint, x: GLshort, y: GLshort) {
    unsafe { global().VertexAttrib2s(index, x, y) }
}

/// # Safety
/// See [`Gl::VertexAttrib2sv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib2sv(index: GLuint, v: *const GLshort) {
    unsafe { global().VertexAttrib2sv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttrib3d`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) {
    unsafe { global().VertexAttrib3d(index, x, y, z) }
}

/// # Safety
/// See [`Gl::VertexAttrib3dv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib3dv(index: GLuint, v: *const GLdouble) {
    unsafe { global().VertexAttrib3dv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttrib3s`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib3s(index: GLuint, x: GLshort, y: GLshort, z: GLshort) {
    unsafe { global().VertexAttrib3s(index, x, y, z) }
}

/// # Safety
/// See [`Gl::VertexAttrib3sv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib3sv(index: GLuint, v: *const GLshort) {
    unsafe { global().VertexAttrib3sv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttrib4Nbv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib4Nbv(index: GLuint, v: *const GLbyte) {
    unsafe { global().VertexAttrib4Nbv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttrib4Niv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib4Niv(index: GLuint, v: *const GLint) {
    unsafe { global().VertexAttrib4Niv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttrib4Nsv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib4Nsv(index: GLuint, v: *const GLshort) {
    unsafe { global().VertexAttrib4Nsv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttrib4Nub`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib4Nub(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte) {
    unsafe { global().VertexAttrib4Nub(index, x, y, z, w) }
}

/// # Safety
/// See [`Gl::VertexAttrib4Nubv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib4Nubv(index: GLuint, v: *const GLubyte) {
    unsafe { global().VertexAttrib4Nubv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttrib4Nuiv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib4Nuiv(index: GLuint, v: *const GLuint) {
    unsafe { global().VertexAttrib4Nuiv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttrib4Nusv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib4Nusv(index: GLuint, v: *const GLushort) {
    unsafe { global().VertexAttrib4Nusv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttrib4bv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib4bv(index: GLuint, v: *const GLbyte) {
    unsafe { global().VertexAttrib4bv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttrib4d`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) {
    unsafe { global().VertexAttrib4d(index, x, y, z, w) }
}

/// # Safety
/// See [`Gl::VertexAttrib4dv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib4dv(index: GLuint, v: *const GLdouble) {
    unsafe { global().VertexAttrib4dv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttrib4iv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib4iv(index: GLuint, v: *const GLint) {
    unsafe { global().VertexAttrib4iv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttrib4s`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib4s(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort) {
    unsafe { global().VertexAttrib4s(index, x, y, z, w) }
}

/// # Safety
/// See [`Gl::VertexAttrib4sv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib4sv(index: GLuint, v: *const GLshort) {
    unsafe { global().VertexAttrib4sv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttrib4ubv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib4ubv(index: GLuint, v: *const GLubyte) {
    unsafe { global().VertexAttrib4ubv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttrib4uiv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib4uiv(index: GLuint, v: *const GLuint) {
    unsafe { global().VertexAttrib4uiv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttrib4usv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib4usv(index: GLuint, v: *const GLushort) {
    unsafe { global().VertexAttrib4usv(index, v) }
}

/// # Safety
/// See [`Gl::AttachShader`]; the global context must be initialized.
#[inline]
pub unsafe fn AttachShader(program: GLuint, shader: GLuint) {
    unsafe { global().AttachShader(program, shader) }
}

/// # Safety
/// See [`Gl::BindAttribLocation`]; the global context must be initialized.
#[inline]
pub unsafe fn BindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar) {
    unsafe { global().BindAttribLocation(program, index, name) }
}

/// # Safety
/// See [`Gl::BlendEquationSeparate`]; the global context must be initialized.
#[inline]
pub unsafe fn BlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum) {
    unsafe { global().BlendEquationSeparate(modeRGB, modeAlpha) }
}

/// # Safety
/// See [`Gl::CompileShader`]; the global context must be initialized.
#[inline]
pub unsafe fn CompileShader(shader: GLuint) {
    unsafe { global().CompileShader(shader) }
}

/// # Safety
/// See [`Gl::CreateProgram`]; the global context must be initialized.
#[inline]
pub unsafe fn CreateProgram() -> GLuint {
    unsafe { global().CreateProgram() }
}

/// # Safety
/// See [`Gl::CreateShader`]; the global context must be initialized.
#[inline]
pub unsafe fn CreateShader(type_: GLenum) -> GLuint {
    unsafe { global().CreateShader(type_) }
}

/// # Safety
/// See [`Gl::DeleteProgram`]; the global context must be initialized.
#[inline]
pub unsafe fn DeleteProgram(program: GLuint) {
    unsafe { global().DeleteProgram(program) }
}

/// # Safety
/// See [`Gl::DeleteShader`]; the global context must be initialized.
#[inline]
pub unsafe fn DeleteShader(shader: GLuint) {
    unsafe { global().DeleteShader(shader) }
}

/// # Safety
/// See [`Gl::DetachShader`]; the global context must be initialized.
#[inline]
pub unsafe fn DetachShader(program: GLuint, shader: GLuint) {
    unsafe { global().DetachShader(program, shader) }
}

/// # Safety
/// See [`Gl::DisableVertexAttribArray`]; the global context must be initialized.
#[inline]
pub unsafe fn DisableVertexAttribArray(index: GLuint) {
    unsafe { global().DisableVertexAttribArray(index) }
}

/// # Safety
/// See [`Gl::EnableVertexAttribArray`]; the global context must be initialized.
#[inline]
pub unsafe fn EnableVertexAttribArray(index: GLuint) {
    unsafe { global().EnableVertexAttribArray(index) }
}

/// # Safety
/// See [`Gl::GetActiveAttrib`]; the global context must be initialized.
#[inline]
pub unsafe fn GetActiveAttrib(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) {
    unsafe { global().GetActiveAttrib(program, index, bufSize, length, size, type_, name) }
}

/// # Safety
/// See [`Gl::GetActiveUniform`]; the global context must be initialized.
#[inline]
pub unsafe fn GetActiveUniform(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) {
    unsafe { global().GetActiveUniform(program, index, bufSize, length, size, type_, name) }
}

/// # Safety
/// See [`Gl::GetAttachedShaders`]; the global context must be initialized.
#[inline]
pub unsafe fn GetAttachedShaders(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint) {
    unsafe { global().GetAttachedShaders(program, maxCount, count, shaders) }
}

/// # Safety
/// See [`Gl::GetAttribLocation`]; the global context must be initialized.
#[inline]
pub unsafe fn GetAttribLocation(program: GLuint, name: *const GLchar) -> GLint {
    unsafe { global().GetAttribLocation(program, name) }
}

/// # Safety
/// See [`Gl::GetProgramInfoLog`]; the global context must be initialized.
#[inline]
pub unsafe fn GetProgramInfoLog(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) {
    unsafe { global().GetProgramInfoLog(program, bufSize, length, infoLog) }
}

/// # Safety
/// See [`Gl::GetProgramiv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) {
    unsafe { global().GetProgramiv(program, pname, params) }
}

/// # Safety
/// See [`Gl::GetShaderInfoLog`]; the global context must be initialized.
#[inline]
pub unsafe fn GetShaderInfoLog(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) {
    unsafe { global().GetShaderInfoLog(shader, bufSize, length, infoLog) }
}

/// # Safety
/// See [`Gl::GetShaderSource`]; the global context must be initialized.
#[inline]
pub unsafe fn GetShaderSource(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar) {
    unsafe { global().GetShaderSource(shader, bufSize, length, source) }
}

/// # Safety
/// See [`Gl::GetShaderiv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) {
    unsafe { global().GetShaderiv(shader, pname, params) }
}

/// # Safety
/// See [`Gl::GetUniformLocation`]; the global context must be initialized.
#[inline]
pub unsafe fn GetUniformLocation(program: GLuint, name: *const GLchar) -> GLint {
    unsafe { global().GetUniformLocation(program, name) }
}

/// # Safety
/// See [`Gl::GetUniformfv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat) {
    unsafe { global().GetUniformfv(program, location, params) }
}

/// # Safety
/// See [`Gl::GetUniformiv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetUniformiv(program: GLuint, location: GLint, params: *mut GLint) {
    unsafe { global().GetUniformiv(program, location, params) }
}

/// # Safety
/// See [`Gl::GetVertexAttribPointerv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetVertexAttribPointerv(index: GLuint, pname: GLenum, pointer: *mut *mut c_void) {
    unsafe { global().GetVertexAttribPointerv(index, pname, pointer) }
}

/// # Safety
/// See [`Gl::GetVertexAttribfv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat) {
    unsafe { global().GetVertexAttribfv(index, pname, params) }
}

/// # Safety
/// See [`Gl::GetVertexAttribiv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint) {
    unsafe { global().GetVertexAttribiv(index, pname, params) }
}

/// # Safety
/// See [`Gl::IsProgram`]; the global context must be initialized.
#[inline]
pub unsafe fn IsProgram(program: GLuint) -> GLboolean {
    unsafe { global().IsProgram(program) }
}

/// # Safety
/// See [`Gl::IsShader`]; the global context must be initialized.
#[inline]
pub unsafe fn IsShader(shader: GLuint) -> GLboolean {
    unsafe { global().IsShader(shader) }
}

/// # Safety
/// See [`Gl::LinkProgram`]; the global context must be initialized.
#[inline]
pub unsafe fn LinkProgram(program: GLuint) {
    unsafe { global().LinkProgram(program) }
}

/// # Safety
/// See [`Gl::ShaderSource`]; the global context must be initialized.
#[inline]
pub unsafe fn ShaderSource(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint) {
    unsafe { global().ShaderSource(shader, count, string, length) }
}

/// # Safety
/// See [`Gl::StencilFuncSeparate`]; the global context must be initialized.
#[inline]
pub unsafe fn StencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) {
    unsafe { global().StencilFuncSeparate(face, func, ref_, mask) }
}

/// # Safety
/// See [`Gl::StencilMaskSeparate`]; the global context must be initialized.
#[inline]
pub unsafe fn StencilMaskSeparate(face: GLenum, mask: GLuint) {
    unsafe { global().StencilMaskSeparate(face, mask) }
}

/// # Safety
/// See [`Gl::StencilOpSeparate`]; the global context must be initialized.
#[inline]
pub unsafe fn StencilOpSeparate(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum) {
    unsafe { global().StencilOpSeparate(face, sfail, dpfail, dppass) }
}

/// # Safety
/// See [`Gl::Uniform1f`]; the global context must be initialized.
#[inline]
pub unsafe fn Uniform1f(location: GLint, v0: GLfloat) {
    unsafe { global().Uniform1f(location, v0) }
}

/// # Safety
/// See [`Gl::Uniform1fv`]; the global context must be initialized.
#[inline]
pub unsafe fn Uniform1fv(location: GLint, count: GLsizei, value: *const GLfloat) {
    unsafe { global().Uniform1fv(location, count, value) }
}

/// # Safety
/// See [`Gl::Uniform1i`]; the global context must be initialized.
#[inline]
pub unsafe fn Uniform1i(location: GLint, v0: GLint) {
    unsafe { global().Uniform1i(location, v0) }
}

/// # Safety
/// See [`Gl::Uniform1iv`]; the global context must be initialized.
#[inline]
pub unsafe fn Uniform1iv(location: GLint, count: GLsizei, value: *const GLint) {
    unsafe { global().Uniform1iv(location, count, value) }
}

/// # Safety
/// See [`Gl::Uniform2f`]; the global context must be initialized.
#[inline]
pub unsafe fn Uniform2f(location: GLint, v0: GLfloat, v1: GLfloat) {
    unsafe { global().Uniform2f(location, v0, v1) }
}

/// # Safety
/// See [`Gl::Uniform2fv`]; the global context must be initialized.
#[inline]
pub unsafe fn Uniform2fv(location: GLint, count: GLsizei, value: *const GLfloat) {
    unsafe { global().Uniform2fv(location, count, value) }
}

/// # Safety
/// See [`Gl::Uniform2i`]; the global context must be initialized.
#[inline]
pub unsafe fn Uniform2i(location: GLint, v0: GLint, v1: GLint) {
    unsafe { global().Uniform2i(location, v0, v1) }
}

/// # Safety
/// See [`Gl::Uniform2iv`]; the global context must be initialized.
#[inline]
pub unsafe fn Uniform2iv(location: GLint, count: GLsizei, value: *const GLint) {
    unsafe { global().Uniform2iv(location, count, value) }
}

/// # Safety
/// See [`Gl::Uniform3f`]; the global context must be initialized.
#[inline]
pub unsafe fn Uniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
    unsafe { global().Uniform3f(location, v0, v1, v2) }
}

/// # Safety
/// See [`Gl::Uniform3fv`]; the global context must be initialized.
#[inline]
pub unsafe fn Uniform3fv(location: GLint, count: GLsizei, value: *const GLfloat) {
    unsafe { global().Uniform3fv(location, count, value) }
}

/// # Safety
/// See [`Gl::Uniform3i`]; the global context must be initialized.
#[inline]
pub unsafe fn Uniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint) {
    unsafe { global().Uniform3i(location, v0, v1, v2) }
}

/// # Safety
/// See [`Gl::Uniform3iv`]; the global context must be initialized.
#[inline]
pub unsafe fn Uniform3iv(location: GLint, count: GLsizei, value: *const GLint) {
    unsafe { global().Uniform3iv(location, count, value) }
}

/// # Safety
/// See [`Gl::Uniform4f`]; the global context must be initialized.
#[inline]
pub unsafe fn Uniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) {
    unsafe { global().Uniform4f(location, v0, v1, v2, v3) }
}

/// # Safety
/// See [`Gl::Uniform4fv`]; the global context must be initialized.
#[inline]
pub unsafe fn Uniform4fv(location: GLint, count: GLsizei, value: *const GLfloat) {
    unsafe { global().Uniform4fv(location, count, value) }
}

/// # Safety
/// See [`Gl::Uniform4i`]; the global context must be initialized.
#[inline]
pub unsafe fn Uniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) {
    unsafe { global().Uniform4i(location, v0, v1, v2, v3) }
}

/// # Safety
/// See [`Gl::Uniform4iv`]; the global context must be initialized.
#[inline]
pub unsafe fn Uniform4iv(location: GLint, count: GLsizei, value: *const GLint) {
    unsafe { global().Uniform4iv(location, count, value) }
}

/// # Safety
/// See [`Gl::UniformMatrix2fv`]; the global context must be initialized.
#[inline]
pub unsafe fn UniformMatrix2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    unsafe { global().UniformMatrix2fv(location, count, transpose, value) }
}

/// # Safety
/// See [`Gl::UniformMatrix3fv`]; the global context must be initialized.
#[inline]
pub unsafe fn UniformMatrix3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    unsafe { global().UniformMatrix3fv(location, count, transpose, value) }
}

/// # Safety
/// See [`Gl::UniformMatrix4fv`]; the global context must be initialized.
#[inline]
pub unsafe fn UniformMatrix4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    unsafe { global().UniformMatrix4fv(location, count, transpose, value) }
}

/// # Safety
/// See [`Gl::UseProgram`]; the global context must be initialized.
#[inline]
pub unsafe fn UseProgram(program: GLuint) {
    unsafe { global().UseProgram(program) }
}

/// # Safety
/// See [`Gl::ValidateProgram`]; the global context must be initialized.
#[inline]
pub unsafe fn ValidateProgram(program: GLuint) {
    unsafe { global().ValidateProgram(program) }
}

/// # Safety
/// See [`Gl::VertexAttrib1f`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib1f(index: GLuint, x: GLfloat) {
    unsafe { global().VertexAttrib1f(index, x) }
}

/// # Safety
/// See [`Gl::VertexAttrib1fv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib1fv(index: GLuint, v: *const GLfloat) {
    unsafe { global().VertexAttrib1fv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttrib2f`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat) {
    unsafe { global().VertexAttrib2f(index, x, y) }
}

/// # Safety
/// See [`Gl::VertexAttrib2fv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib2fv(index: GLuint, v: *const GLfloat) {
    unsafe { global().VertexAttrib2fv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttrib3f`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
    unsafe { global().VertexAttrib3f(index, x, y, z) }
}

/// # Safety
/// See [`Gl::VertexAttrib3fv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib3fv(index: GLuint, v: *const GLfloat) {
    unsafe { global().VertexAttrib3fv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttrib4f`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
    unsafe { global().VertexAttrib4f(index, x, y, z, w) }
}

/// # Safety
/// See [`Gl::VertexAttrib4fv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttrib4fv(index: GLuint, v: *const GLfloat) {
    unsafe { global().VertexAttrib4fv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttribPointer`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribPointer(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void) {
    unsafe { global().VertexAttribPointer(index, size, type_, normalized, stride, pointer) }
}

/// # Safety
/// See [`Gl::DrawBuffers`]; the global context must be initialized.
#[inline]
pub unsafe fn DrawBuffers(n: GLsizei, bufs: *const GLenum) {
    unsafe { global().DrawBuffers(n, bufs) }
}

/// # Safety
/// See [`Gl::UniformMatrix2x3fv`]; the global context must be initialized.
#[inline]
pub unsafe fn UniformMatrix2x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    unsafe { global().UniformMatrix2x3fv(location, count, transpose, value) }
}

/// # Safety
/// See [`Gl::UniformMatrix2x4fv`]; the global context must be initialized.
#[inline]
pub unsafe fn UniformMatrix2x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    unsafe { global().UniformMatrix2x4fv(location, count, transpose, value) }
}

/// # Safety
/// See [`Gl::UniformMatrix3x2fv`]; the global context must be initialized.
#[inline]
pub unsafe fn UniformMatrix3x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    unsafe { global().UniformMatrix3x2fv(location, count, transpose, value) }
}

/// # Safety
/// See [`Gl::UniformMatrix3x4fv`]; the global context must be initialized.
#[inline]
pub unsafe fn UniformMatrix3x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    unsafe { global().UniformMatrix3x4fv(location, count, transpose, value) }
}

/// # Safety
/// See [`Gl::UniformMatrix4x2fv`]; the global context must be initialized.
#[inline]
pub unsafe fn UniformMatrix4x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    unsafe { global().UniformMatrix4x2fv(location, count, transpose, value) }
}

/// # Safety
/// See [`Gl::UniformMatrix4x3fv`]; the global context must be initialized.
#[inline]
pub unsafe fn UniformMatrix4x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    unsafe { global().UniformMatrix4x3fv(location, count, transpose, value) }
}

/// # Safety
/// See [`Gl::BeginConditionalRender`]; the global context must be initialized.
#[inline]
pub unsafe fn BeginConditionalRender(id: GLuint, mode: GLenum) {
    unsafe { global().BeginConditionalRender(id, mode) }
}

/// # Safety
/// See [`Gl::BindFragDataLocation`]; the global context must be initialized.
#[inline]
pub unsafe fn BindFragDataLocation(program: GLuint, color: GLuint, name: *const GLchar) {
    unsafe { global().BindFragDataLocation(program, color, name) }
}

/// # Safety
/// See [`Gl::ClampColor`]; the global context must be initialized.
#[inline]
pub unsafe fn ClampColor(target: GLenum, clamp: GLenum) {
    unsafe { global().ClampColor(target, clamp) }
}

/// # Safety
/// See [`Gl::ColorMaski`]; the global context must be initialized.
#[inline]
pub unsafe fn ColorMaski(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean) {
    unsafe { global().ColorMaski(index, r, g, b, a) }
}

/// # Safety
/// See [`Gl::Disablei`]; the global context must be initialized.
#[inline]
pub unsafe fn Disablei(target: GLenum, index: GLuint) {
    unsafe { global().Disablei(target, index) }
}

/// # Safety
/// See [`Gl::Enablei`]; the global context must be initialized.
#[inline]
pub unsafe fn Enablei(target: GLenum, index: GLuint) {
    unsafe { global().Enablei(target, index) }
}

/// # Safety
/// See [`Gl::EndConditionalRender`]; the global context must be initialized.
#[inline]
pub unsafe fn EndConditionalRender() {
    unsafe { global().EndConditionalRender() }
}

/// # Safety
/// See [`Gl::FramebufferTexture1D`]; the global context must be initialized.
#[inline]
pub unsafe fn FramebufferTexture1D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) {
    unsafe { global().FramebufferTexture1D(target, attachment, textarget, texture, level) }
}

/// # Safety
/// See [`Gl::FramebufferTexture3D`]; the global context must be initialized.
#[inline]
pub unsafe fn FramebufferTexture3D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint) {
    unsafe { global().FramebufferTexture3D(target, attachment, textarget, texture, level, zoffset) }
}

/// # Safety
/// See [`Gl::GetBooleani_v`]; the global context must be initialized.
#[inline]
pub unsafe fn GetBooleani_v(target: GLenum, index: GLuint, data: *mut GLboolean) {
    unsafe { global().GetBooleani_v(target, index, data) }
}

/// # Safety
/// See [`Gl::GetTexParameterIiv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetTexParameterIiv(target: GLenum, pname: GLenum, params: *mut GLint) {
    unsafe { global().GetTexParameterIiv(target, pname, params) }
}

/// # Safety
/// See [`Gl::GetTexParameterIuiv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetTexParameterIuiv(target: GLenum, pname: GLenum, params: *mut GLuint) {
    unsafe { global().GetTexParameterIuiv(target, pname, params) }
}

/// # Safety
/// See [`Gl::IsEnabledi`]; the global context must be initialized.
#[inline]
pub unsafe fn IsEnabledi(target: GLenum, index: GLuint) -> GLboolean {
    unsafe { global().IsEnabledi(target, index) }
}

/// # Safety
/// See [`Gl::TexParameterIiv`]; the global context must be initialized.
#[inline]
pub unsafe fn TexParameterIiv(target: GLenum, pname: GLenum, params: *const GLint) {
    unsafe { global().TexParameterIiv(target, pname, params) }
}

/// # Safety
/// See [`Gl::TexParameterIuiv`]; the global context must be initialized.
#[inline]
pub unsafe fn TexParameterIuiv(target: GLenum, pname: GLenum, params: *const GLuint) {
    unsafe { global().TexParameterIuiv(target, pname, params) }
}

/// # Safety
/// See [`Gl::VertexAttribI1i`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribI1i(index: GLuint, x: GLint) {
    unsafe { global().VertexAttribI1i(index, x) }
}

/// # Safety
/// See [`Gl::VertexAttribI1iv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribI1iv(index: GLuint, v: *const GLint) {
    unsafe { global().VertexAttribI1iv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttribI1ui`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribI1ui(index: GLuint, x: GLuint) {
    unsafe { global().VertexAttribI1ui(index, x) }
}

/// # Safety
/// See [`Gl::VertexAttribI1uiv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribI1uiv(index: GLuint, v: *const GLuint) {
    unsafe { global().VertexAttribI1uiv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttribI2i`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribI2i(index: GLuint, x: GLint, y: GLint) {
    unsafe { global().VertexAttribI2i(index, x, y) }
}

/// # Safety
/// See [`Gl::VertexAttribI2iv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribI2iv(index: GLuint, v: *const GLint) {
    unsafe { global().VertexAttribI2iv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttribI2ui`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribI2ui(index: GLuint, x: GLuint, y: GLuint) {
    unsafe { global().VertexAttribI2ui(index, x, y) }
}

/// # Safety
/// See [`Gl::VertexAttribI2uiv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribI2uiv(index: GLuint, v: *const GLuint) {
    unsafe { global().VertexAttribI2uiv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttribI3i`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribI3i(index: GLuint, x: GLint, y: GLint, z: GLint) {
    unsafe { global().VertexAttribI3i(index, x, y, z) }
}

/// # Safety
/// See [`Gl::VertexAttribI3iv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribI3iv(index: GLuint, v: *const GLint) {
    unsafe { global().VertexAttribI3iv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttribI3ui`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribI3ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint) {
    unsafe { global().VertexAttribI3ui(index, x, y, z) }
}

/// # Safety
/// See [`Gl::VertexAttribI3uiv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribI3uiv(index: GLuint, v: *const GLuint) {
    unsafe { global().VertexAttribI3uiv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttribI4bv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribI4bv(index: GLuint, v: *const GLbyte) {
    unsafe { global().VertexAttribI4bv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttribI4sv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribI4sv(index: GLuint, v: *const GLshort) {
    unsafe { global().VertexAttribI4sv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttribI4ubv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribI4ubv(index: GLuint, v: *const GLubyte) {
    unsafe { global().VertexAttribI4ubv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttribI4usv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribI4usv(index: GLuint, v: *const GLushort) {
    unsafe { global().VertexAttribI4usv(index, v) }
}

/// # Safety
/// See [`Gl::BindBufferBase`]; the global context must be initialized.
#[inline]
pub unsafe fn BindBufferBase(target: GLenum, index: GLuint, buffer: GLuint) {
    unsafe { global().BindBufferBase(target, index, buffer) }
}

/// # Safety
/// See [`Gl::BindBufferRange`]; the global context must be initialized.
#[inline]
pub unsafe fn BindBufferRange(target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) {
    unsafe { global().BindBufferRange(target, index, buffer, offset, size) }
}

/// # Safety
/// See [`Gl::GetIntegeri_v`]; the global context must be initialized.
#[inline]
pub unsafe fn GetIntegeri_v(target: GLenum, index: GLuint, data: *mut GLint) {
    unsafe { global().GetIntegeri_v(target, index, data) }
}

/// # Safety
/// See [`Gl::BindFramebuffer`]; the global context must be initialized.
#[inline]
pub unsafe fn BindFramebuffer(target: GLenum, framebuffer: GLuint) {
    unsafe { global().BindFramebuffer(target, framebuffer) }
}

/// # Safety
/// See [`Gl::BindRenderbuffer`]; the global context must be initialized.
#[inline]
pub unsafe fn BindRenderbuffer(target: GLenum, renderbuffer: GLuint) {
    unsafe { global().BindRenderbuffer(target, renderbuffer) }
}

/// # Safety
/// See [`Gl::CheckFramebufferStatus`]; the global context must be initialized.
#[inline]
pub unsafe fn CheckFramebufferStatus(target: GLenum) -> GLenum {
    unsafe { global().CheckFramebufferStatus(target) }
}

/// # Safety
/// See [`Gl::DeleteFramebuffers`]; the global context must be initialized.
#[inline]
pub unsafe fn DeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) {
    unsafe { global().DeleteFramebuffers(n, framebuffers) }
}

/// # Safety
/// See [`Gl::DeleteRenderbuffers`]; the global context must be initialized.
#[inline]
pub unsafe fn DeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint) {
    unsafe { global().DeleteRenderbuffers(n, renderbuffers) }
}

/// # Safety
/// See [`Gl::FramebufferRenderbuffer`]; the global context must be initialized.
#[inline]
pub unsafe fn FramebufferRenderbuffer(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) {
    unsafe { global().FramebufferRenderbuffer(target, attachment, renderbuffertarget, renderbuffer) }
}

/// # Safety
/// See [`Gl::FramebufferTexture2D`]; the global context must be initialized.
#[inline]
pub unsafe fn FramebufferTexture2D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) {
    unsafe { global().FramebufferTexture2D(target, attachment, textarget, texture, level) }
}

/// # Safety
/// See [`Gl::GenFramebuffers`]; the global context must be initialized.
#[inline]
pub unsafe fn GenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) {
    unsafe { global().GenFramebuffers(n, framebuffers) }
}

/// # Safety
/// See [`Gl::GenRenderbuffers`]; the global context must be initialized.
#[inline]
pub unsafe fn GenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) {
    unsafe { global().GenRenderbuffers(n, renderbuffers) }
}

/// # Safety
/// See [`Gl::GenerateMipmap`]; the global context must be initialized.
#[inline]
pub unsafe fn GenerateMipmap(target: GLenum) {
    unsafe { global().GenerateMipmap(target) }
}

/// # Safety
/// See [`Gl::GetFramebufferAttachmentParameteriv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetFramebufferAttachmentParameteriv(target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint) {
    unsafe { global().GetFramebufferAttachmentParameteriv(target, attachment, pname, params) }
}

/// # Safety
/// See [`Gl::GetRenderbufferParameteriv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetRenderbufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    unsafe { global().GetRenderbufferParameteriv(target, pname, params) }
}

/// # Safety
/// See [`Gl::IsFramebuffer`]; the global context must be initialized.
#[inline]
pub unsafe fn IsFramebuffer(framebuffer: GLuint) -> GLboolean {
    unsafe { global().IsFramebuffer(framebuffer) }
}

/// # Safety
/// See [`Gl::IsRenderbuffer`]; the global context must be initialized.
#[inline]
pub unsafe fn IsRenderbuffer(renderbuffer: GLuint) -> GLboolean {
    unsafe { global().IsRenderbuffer(renderbuffer) }
}

/// # Safety
/// See [`Gl::RenderbufferStorage`]; the global context must be initialized.
#[inline]
pub unsafe fn RenderbufferStorage(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei) {
    unsafe { global().RenderbufferStorage(target, internalformat, width, height) }
}

/// # Safety
/// See [`Gl::BeginTransformFeedback`]; the global context must be initialized.
#[inline]
pub unsafe fn BeginTransformFeedback(primitiveMode: GLenum) {
    unsafe { global().BeginTransformFeedback(primitiveMode) }
}

/// # Safety
/// See [`Gl::BindVertexArray`]; the global context must be initialized.
#[inline]
pub unsafe fn BindVertexArray(array: GLuint) {
    unsafe { global().BindVertexArray(array) }
}

/// # Safety
/// See [`Gl::BlitFramebuffer`]; the global context must be initialized.
#[inline]
pub unsafe fn BlitFramebuffer(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) {
    unsafe { global().BlitFramebuffer(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) }
}

/// # Safety
/// See [`Gl::ClearBufferfi`]; the global context must be initialized.
#[inline]
pub unsafe fn ClearBufferfi(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) {
    unsafe { global().ClearBufferfi(buffer, drawbuffer, depth, stencil) }
}

/// # Safety
/// See [`Gl::ClearBufferfv`]; the global context must be initialized.
#[inline]
pub unsafe fn ClearBufferfv(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) {
    unsafe { global().ClearBufferfv(buffer, drawbuffer, value) }
}

/// # Safety
/// See [`Gl::ClearBufferiv`]; the global context must be initialized.
#[inline]
pub unsafe fn ClearBufferiv(buffer: GLenum, drawbuffer: GLint, value: *const GLint) {
    unsafe { global().ClearBufferiv(buffer, drawbuffer, value) }
}

/// # Safety
/// See [`Gl::ClearBufferuiv`]; the global context must be initialized.
#[inline]
pub unsafe fn ClearBufferuiv(buffer: GLenum, drawbuffer: GLint, value: *const GLuint) {
    unsafe { global().ClearBufferuiv(buffer, drawbuffer, value) }
}

/// # Safety
/// See [`Gl::DeleteVertexArrays`]; the global context must be initialized.
#[inline]
pub unsafe fn DeleteVertexArrays(n: GLsizei, arrays: *const GLuint) {
    unsafe { global().DeleteVertexArrays(n, arrays) }
}

/// # Safety
/// See [`Gl::EndTransformFeedback`]; the global context must be initialized.
#[inline]
pub unsafe fn EndTransformFeedback() {
    unsafe { global().EndTransformFeedback() }
}

/// # Safety
/// See [`Gl::FlushMappedBufferRange`]; the global context must be initialized.
#[inline]
pub unsafe fn FlushMappedBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr) {
    unsafe { global().FlushMappedBufferRange(target, offset, length) }
}

/// # Safety
/// See [`Gl::FramebufferTextureLayer`]; the global context must be initialized.
#[inline]
pub unsafe fn FramebufferTextureLayer(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint) {
    unsafe { global().FramebufferTextureLayer(target, attachment, texture, level, layer) }
}

/// # Safety
/// See [`Gl::GenVertexArrays`]; the global context must be initialized.
#[inline]
pub unsafe fn GenVertexArrays(n: GLsizei, arrays: *mut GLuint) {
    unsafe { global().GenVertexArrays(n, arrays) }
}

/// # Safety
/// See [`Gl::GetFragDataLocation`]; the global context must be initialized.
#[inline]
pub unsafe fn GetFragDataLocation(program: GLuint, name: *const GLchar) -> GLint {
    unsafe { global().GetFragDataLocation(program, name) }
}

/// # Safety
/// See [`Gl::GetStringi`]; the global context must be initialized.
#[inline]
pub unsafe fn GetStringi(name: GLenum, index: GLuint) -> *const GLubyte {
    unsafe { global().GetStringi(name, index) }
}

/// # Safety
/// See [`Gl::GetTransformFeedbackVarying`]; the global context must be initialized.
#[inline]
pub unsafe fn GetTransformFeedbackVarying(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar) {
    unsafe { global().GetTransformFeedbackVarying(program, index, bufSize, length, size, type_, name) }
}

/// # Safety
/// See [`Gl::GetUniformuiv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetUniformuiv(program: GLuint, location: GLint, params: *mut GLuint) {
    unsafe { global().GetUniformuiv(program, location, params) }
}

/// # Safety
/// See [`Gl::GetVertexAttribIiv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetVertexAttribIiv(index: GLuint, pname: GLenum, params: *mut GLint) {
    unsafe { global().GetVertexAttribIiv(index, pname, params) }
}

/// # Safety
/// See [`Gl::GetVertexAttribIuiv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetVertexAttribIuiv(index: GLuint, pname: GLenum, params: *mut GLuint) {
    unsafe { global().GetVertexAttribIuiv(index, pname, params) }
}

/// # Safety
/// See [`Gl::IsVertexArray`]; the global context must be initialized.
#[inline]
pub unsafe fn IsVertexArray(array: GLuint) -> GLboolean {
    unsafe { global().IsVertexArray(array) }
}

/// # Safety
/// See [`Gl::MapBufferRange`]; the global context must be initialized.
#[inline]
pub unsafe fn MapBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut c_void {
    unsafe { global().MapBufferRange(target, offset, length, access) }
}

/// # Safety
/// See [`Gl::RenderbufferStorageMultisample`]; the global context must be initialized.
#[inline]
pub unsafe fn RenderbufferStorageMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) {
    unsafe { global().RenderbufferStorageMultisample(target, samples, internalformat, width, height) }
}

/// # Safety
/// See [`Gl::TransformFeedbackVaryings`]; the global context must be initialized.
#[inline]
pub unsafe fn TransformFeedbackVaryings(program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: GLenum) {
    unsafe { global().TransformFeedbackVaryings(program, count, varyings, bufferMode) }
}

/// # Safety
/// See [`Gl::Uniform1ui`]; the global context must be initialized.
#[inline]
pub unsafe fn Uniform1ui(location: GLint, v0: GLuint) {
    unsafe { global().Uniform1ui(location, v0) }
}

/// # Safety
/// See [`Gl::Uniform1uiv`]; the global context must be initialized.
#[inline]
pub unsafe fn Uniform1uiv(location: GLint, count: GLsizei, value: *const GLuint) {
    unsafe { global().Uniform1uiv(location, count, value) }
}

/// # Safety
/// See [`Gl::Uniform2ui`]; the global context must be initialized.
#[inline]
pub unsafe fn Uniform2ui(location: GLint, v0: GLuint, v1: GLuint) {
    unsafe { global().Uniform2ui(location, v0, v1) }
}

/// # Safety
/// See [`Gl::Uniform2uiv`]; the global context must be initialized.
#[inline]
pub unsafe fn Uniform2uiv(location: GLint, count: GLsizei, value: *const GLuint) {
    unsafe { global().Uniform2uiv(location, count, value) }
}

/// # Safety
/// See [`Gl::Uniform3ui`]; the global context must be initialized.
#[inline]
pub unsafe fn Uniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) {
    unsafe { global().Uniform3ui(location, v0, v1, v2) }
}

/// # Safety
/// See [`Gl::Uniform3uiv`]; the global context must be initialized.
#[inline]
pub unsafe fn Uniform3uiv(location: GLint, count: GLsizei, value: *const GLuint) {
    unsafe { global().Uniform3uiv(location, count, value) }
}

/// # Safety
/// See [`Gl::Uniform4ui`]; the global context must be initialized.
#[inline]
pub unsafe fn Uniform4ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) {
    unsafe { global().Uniform4ui(location, v0, v1, v2, v3) }
}

/// # Safety
/// See [`Gl::Uniform4uiv`]; the global context must be initialized.
#[inline]
pub unsafe fn Uniform4uiv(location: GLint, count: GLsizei, value: *const GLuint) {
    unsafe { global().Uniform4uiv(location, count, value) }
}

/// # Safety
/// See [`Gl::VertexAttribI4i`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribI4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) {
    unsafe { global().VertexAttribI4i(index, x, y, z, w) }
}

/// # Safety
/// See [`Gl::VertexAttribI4iv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribI4iv(index: GLuint, v: *const GLint) {
    unsafe { global().VertexAttribI4iv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttribI4ui`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribI4ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) {
    unsafe { global().VertexAttribI4ui(index, x, y, z, w) }
}

/// # Safety
/// See [`Gl::VertexAttribI4uiv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribI4uiv(index: GLuint, v: *const GLuint) {
    unsafe { global().VertexAttribI4uiv(index, v) }
}

/// # Safety
/// See [`Gl::VertexAttribIPointer`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribIPointer(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void) {
    unsafe { global().VertexAttribIPointer(index, size, type_, stride, pointer) }
}

/// # Safety
/// See [`Gl::GetActiveUniformName`]; the global context must be initialized.
#[inline]
pub unsafe fn GetActiveUniformName(program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar) {
    unsafe { global().GetActiveUniformName(program, uniformIndex, bufSize, length, uniformName) }
}

/// # Safety
/// See [`Gl::PrimitiveRestartIndex`]; the global context must be initialized.
#[inline]
pub unsafe fn PrimitiveRestartIndex(index: GLuint) {
    unsafe { global().PrimitiveRestartIndex(index) }
}

/// # Safety
/// See [`Gl::TexBuffer`]; the global context must be initialized.
#[inline]
pub unsafe fn TexBuffer(target: GLenum, internalformat: GLenum, buffer: GLuint) {
    unsafe { global().TexBuffer(target, internalformat, buffer) }
}

/// # Safety
/// See [`Gl::CopyBufferSubData`]; the global context must be initialized.
#[inline]
pub unsafe fn CopyBufferSubData(readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) {
    unsafe { global().CopyBufferSubData(readTarget, writeTarget, readOffset, writeOffset, size) }
}

/// # Safety
/// See [`Gl::DrawArraysInstanced`]; the global context must be initialized.
#[inline]
pub unsafe fn DrawArraysInstanced(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei) {
    unsafe { global().DrawArraysInstanced(mode, first, count, instancecount) }
}

/// # Safety
/// See [`Gl::DrawElementsInstanced`]; the global context must be initialized.
#[inline]
pub unsafe fn DrawElementsInstanced(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei) {
    unsafe { global().DrawElementsInstanced(mode, count, type_, indices, instancecount) }
}

/// # Safety
/// See [`Gl::GetActiveUniformBlockName`]; the global context must be initialized.
#[inline]
pub unsafe fn GetActiveUniformBlockName(program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar) {
    unsafe { global().GetActiveUniformBlockName(program, uniformBlockIndex, bufSize, length, uniformBlockName) }
}

/// # Safety
/// See [`Gl::GetActiveUniformBlockiv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetActiveUniformBlockiv(program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint) {
    unsafe { global().GetActiveUniformBlockiv(program, uniformBlockIndex, pname, params) }
}

/// # Safety
/// See [`Gl::GetActiveUniformsiv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetActiveUniformsiv(program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: GLenum, params: *mut GLint) {
    unsafe { global().GetActiveUniformsiv(program, uniformCount, uniformIndices, pname, params) }
}

/// # Safety
/// See [`Gl::GetUniformBlockIndex`]; the global context must be initialized.
#[inline]
pub unsafe fn GetUniformBlockIndex(program: GLuint, uniformBlockName: *const GLchar) -> GLuint {
    unsafe { global().GetUniformBlockIndex(program, uniformBlockName) }
}

/// # Safety
/// See [`Gl::GetUniformIndices`]; the global context must be initialized.
#[inline]
pub unsafe fn GetUniformIndices(program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint) {
    unsafe { global().GetUniformIndices(program, uniformCount, uniformNames, uniformIndices) }
}

/// # Safety
/// See [`Gl::UniformBlockBinding`]; the global context must be initialized.
#[inline]
pub unsafe fn UniformBlockBinding(program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint) {
    unsafe { global().UniformBlockBinding(program, uniformBlockIndex, uniformBlockBinding) }
}

/// # Safety
/// See [`Gl::DrawElementsBaseVertex`]; the global context must be initialized.
#[inline]
pub unsafe fn DrawElementsBaseVertex(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, basevertex: GLint) {
    unsafe { global().DrawElementsBaseVertex(mode, count, type_, indices, basevertex) }
}

/// # Safety
/// See [`Gl::DrawElementsInstancedBaseVertex`]; the global context must be initialized.
#[inline]
pub unsafe fn DrawElementsInstancedBaseVertex(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, basevertex: GLint) {
    unsafe { global().DrawElementsInstancedBaseVertex(mode, count, type_, indices, instancecount, basevertex) }
}

/// # Safety
/// See [`Gl::DrawRangeElementsBaseVertex`]; the global context must be initialized.
#[inline]
pub unsafe fn DrawRangeElementsBaseVertex(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const c_void, basevertex: GLint) {
    unsafe { global().DrawRangeElementsBaseVertex(mode, start, end, count, type_, indices, basevertex) }
}

/// # Safety
/// See [`Gl::FramebufferTexture`]; the global context must be initialized.
#[inline]
pub unsafe fn FramebufferTexture(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint) {
    unsafe { global().FramebufferTexture(target, attachment, texture, level) }
}

/// # Safety
/// See [`Gl::GetMultisamplefv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetMultisamplefv(pname: GLenum, index: GLuint, val: *mut GLfloat) {
    unsafe { global().GetMultisamplefv(pname, index, val) }
}

/// # Safety
/// See [`Gl::MultiDrawElementsBaseVertex`]; the global context must be initialized.
#[inline]
pub unsafe fn MultiDrawElementsBaseVertex(mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, drawcount: GLsizei, basevertex: *const GLint) {
    unsafe { global().MultiDrawElementsBaseVertex(mode, count, type_, indices, drawcount, basevertex) }
}

/// # Safety
/// See [`Gl::ProvokingVertex`]; the global context must be initialized.
#[inline]
pub unsafe fn ProvokingVertex(mode: GLenum) {
    unsafe { global().ProvokingVertex(mode) }
}

/// # Safety
/// See [`Gl::SampleMaski`]; the global context must be initialized.
#[inline]
pub unsafe fn SampleMaski(maskNumber: GLuint, mask: GLbitfield) {
    unsafe { global().SampleMaski(maskNumber, mask) }
}

/// # Safety
/// See [`Gl::TexImage2DMultisample`]; the global context must be initialized.
#[inline]
pub unsafe fn TexImage2DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) {
    unsafe { global().TexImage2DMultisample(target, samples, internalformat, width, height, fixedsamplelocations) }
}

/// # Safety
/// See [`Gl::TexImage3DMultisample`]; the global context must be initialized.
#[inline]
pub unsafe fn TexImage3DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) {
    unsafe { global().TexImage3DMultisample(target, samples, internalformat, width, height, depth, fixedsamplelocations) }
}

/// # Safety
/// See [`Gl::ClientWaitSync`]; the global context must be initialized.
#[inline]
pub unsafe fn ClientWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum {
    unsafe { global().ClientWaitSync(sync, flags, timeout) }
}

/// # Safety
/// See [`Gl::DeleteSync`]; the global context must be initialized.
#[inline]
pub unsafe fn DeleteSync(sync: GLsync) {
    unsafe { global().DeleteSync(sync) }
}

/// # Safety
/// See [`Gl::FenceSync`]; the global context must be initialized.
#[inline]
pub unsafe fn FenceSync(condition: GLenum, flags: GLbitfield) -> GLsync {
    unsafe { global().FenceSync(condition, flags) }
}

/// # Safety
/// See [`Gl::GetBufferParameteri64v`]; the global context must be initialized.
#[inline]
pub unsafe fn GetBufferParameteri64v(target: GLenum, pname: GLenum, params: *mut GLint64) {
    unsafe { global().GetBufferParameteri64v(target, pname, params) }
}

/// # Safety
/// See [`Gl::GetInteger64i_v`]; the global context must be initialized.
#[inline]
pub unsafe fn GetInteger64i_v(target: GLenum, index: GLuint, data: *mut GLint64) {
    unsafe { global().GetInteger64i_v(target, index, data) }
}

/// # Safety
/// See [`Gl::GetInteger64v`]; the global context must be initialized.
#[inline]
pub unsafe fn GetInteger64v(pname: GLenum, data: *mut GLint64) {
    unsafe { global().GetInteger64v(pname, data) }
}

/// # Safety
/// See [`Gl::GetSynciv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetSynciv(sync: GLsync, pname: GLenum, count: GLsizei, length: *mut GLsizei, values: *mut GLint) {
    unsafe { global().GetSynciv(sync, pname, count, length, values) }
}

/// # Safety
/// See [`Gl::IsSync`]; the global context must be initialized.
#[inline]
pub unsafe fn IsSync(sync: GLsync) -> GLboolean {
    unsafe { global().IsSync(sync) }
}

/// # Safety
/// See [`Gl::WaitSync`]; the global context must be initialized.
#[inline]
pub unsafe fn WaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) {
    unsafe { global().WaitSync(sync, flags, timeout) }
}

/// # Safety
/// See [`Gl::BindFragDataLocationIndexed`]; the global context must be initialized.
#[inline]
pub unsafe fn BindFragDataLocationIndexed(program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar) {
    unsafe { global().BindFragDataLocationIndexed(program, colorNumber, index, name) }
}

/// # Safety
/// See [`Gl::GetFragDataIndex`]; the global context must be initialized.
#[inline]
pub unsafe fn GetFragDataIndex(program: GLuint, name: *const GLchar) -> GLint {
    unsafe { global().GetFragDataIndex(program, name) }
}

/// # Safety
/// See [`Gl::GetQueryObjecti64v`]; the global context must be initialized.
#[inline]
pub unsafe fn GetQueryObjecti64v(id: GLuint, pname: GLenum, params: *mut GLint64) {
    unsafe { global().GetQueryObjecti64v(id, pname, params) }
}

/// # Safety
/// See [`Gl::GetQueryObjectui64v`]; the global context must be initialized.
#[inline]
pub unsafe fn GetQueryObjectui64v(id: GLuint, pname: GLenum, params: *mut GLuint64) {
    unsafe { global().GetQueryObjectui64v(id, pname, params) }
}

/// # Safety
/// See [`Gl::GetSamplerParameterIiv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetSamplerParameterIiv(sampler: GLuint, pname: GLenum, params: *mut GLint) {
    unsafe { global().GetSamplerParameterIiv(sampler, pname, params) }
}

/// # Safety
/// See [`Gl::GetSamplerParameterIuiv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetSamplerParameterIuiv(sampler: GLuint, pname: GLenum, params: *mut GLuint) {
    unsafe { global().GetSamplerParameterIuiv(sampler, pname, params) }
}

/// # Safety
/// See [`Gl::QueryCounter`]; the global context must be initialized.
#[inline]
pub unsafe fn QueryCounter(id: GLuint, target: GLenum) {
    unsafe { global().QueryCounter(id, target) }
}

/// # Safety
/// See [`Gl::SamplerParameterIiv`]; the global context must be initialized.
#[inline]
pub unsafe fn SamplerParameterIiv(sampler: GLuint, pname: GLenum, param: *const GLint) {
    unsafe { global().SamplerParameterIiv(sampler, pname, param) }
}

/// # Safety
/// See [`Gl::SamplerParameterIuiv`]; the global context must be initialized.
#[inline]
pub unsafe fn SamplerParameterIuiv(sampler: GLuint, pname: GLenum, param: *const GLuint) {
    unsafe { global().SamplerParameterIuiv(sampler, pname, param) }
}

/// # Safety
/// See [`Gl::VertexAttribP1ui`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribP1ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) {
    unsafe { global().VertexAttribP1ui(index, type_, normalized, value) }
}

/// # Safety
/// See [`Gl::VertexAttribP1uiv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribP1uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) {
    unsafe { global().VertexAttribP1uiv(index, type_, normalized, value) }
}

/// # Safety
/// See [`Gl::VertexAttribP2ui`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribP2ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) {
    unsafe { global().VertexAttribP2ui(index, type_, normalized, value) }
}

/// # Safety
/// See [`Gl::VertexAttribP2uiv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribP2uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) {
    unsafe { global().VertexAttribP2uiv(index, type_, normalized, value) }
}

/// # Safety
/// See [`Gl::VertexAttribP3ui`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribP3ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) {
    unsafe { global().VertexAttribP3ui(index, type_, normalized, value) }
}

/// # Safety
/// See [`Gl::VertexAttribP3uiv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribP3uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) {
    unsafe { global().VertexAttribP3uiv(index, type_, normalized, value) }
}

/// # Safety
/// See [`Gl::VertexAttribP4ui`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribP4ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) {
    unsafe { global().VertexAttribP4ui(index, type_, normalized, value) }
}

/// # Safety
/// See [`Gl::VertexAttribP4uiv`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribP4uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) {
    unsafe { global().VertexAttribP4uiv(index, type_, normalized, value) }
}

/// # Safety
/// See [`Gl::BindSampler`]; the global context must be initialized.
#[inline]
pub unsafe fn BindSampler(unit: GLuint, sampler: GLuint) {
    unsafe { global().BindSampler(unit, sampler) }
}

/// # Safety
/// See [`Gl::DeleteSamplers`]; the global context must be initialized.
#[inline]
pub unsafe fn DeleteSamplers(count: GLsizei, samplers: *const GLuint) {
    unsafe { global().DeleteSamplers(count, samplers) }
}

/// # Safety
/// See [`Gl::GenSamplers`]; the global context must be initialized.
#[inline]
pub unsafe fn GenSamplers(count: GLsizei, samplers: *mut GLuint) {
    unsafe { global().GenSamplers(count, samplers) }
}

/// # Safety
/// See [`Gl::GetSamplerParameterfv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetSamplerParameterfv(sampler: GLuint, pname: GLenum, params: *mut GLfloat) {
    unsafe { global().GetSamplerParameterfv(sampler, pname, params) }
}

/// # Safety
/// See [`Gl::GetSamplerParameteriv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetSamplerParameteriv(sampler: GLuint, pname: GLenum, params: *mut GLint) {
    unsafe { global().GetSamplerParameteriv(sampler, pname, params) }
}

/// # Safety
/// See [`Gl::IsSampler`]; the global context must be initialized.
#[inline]
pub unsafe fn IsSampler(sampler: GLuint) -> GLboolean {
    unsafe { global().IsSampler(sampler) }
}

/// # Safety
/// See [`Gl::SamplerParameterf`]; the global context must be initialized.
#[inline]
pub unsafe fn SamplerParameterf(sampler: GLuint, pname: GLenum, param: GLfloat) {
    unsafe { global().SamplerParameterf(sampler, pname, param) }
}

/// # Safety
/// See [`Gl::SamplerParameterfv`]; the global context must be initialized.
#[inline]
pub unsafe fn SamplerParameterfv(sampler: GLuint, pname: GLenum, param: *const GLfloat) {
    unsafe { global().SamplerParameterfv(sampler, pname, param) }
}

/// # Safety
/// See [`Gl::SamplerParameteri`]; the global context must be initialized.
#[inline]
pub unsafe fn SamplerParameteri(sampler: GLuint, pname: GLenum, param: GLint) {
    unsafe { global().SamplerParameteri(sampler, pname, param) }
}

/// # Safety
/// See [`Gl::SamplerParameteriv`]; the global context must be initialized.
#[inline]
pub unsafe fn SamplerParameteriv(sampler: GLuint, pname: GLenum, param: *const GLint) {
    unsafe { global().SamplerParameteriv(sampler, pname, param) }
}

/// # Safety
/// See [`Gl::VertexAttribDivisor`]; the global context must be initialized.
#[inline]
pub unsafe fn VertexAttribDivisor(index: GLuint, divisor: GLuint) {
    unsafe { global().VertexAttribDivisor(index, divisor) }
}

/// # Safety
/// See [`Gl::ClearDepthf`]; the global context must be initialized.
#[inline]
pub unsafe fn ClearDepthf(d: GLfloat) {
    unsafe { global().ClearDepthf(d) }
}

/// # Safety
/// See [`Gl::DepthRangef`]; the global context must be initialized.
#[inline]
pub unsafe fn DepthRangef(n: GLfloat, f: GLfloat) {
    unsafe { global().DepthRangef(n, f) }
}

/// # Safety
/// See [`Gl::GetShaderPrecisionFormat`]; the global context must be initialized.
#[inline]
pub unsafe fn GetShaderPrecisionFormat(shadertype: GLenum, precisiontype: GLenum, range: *mut GLint, precision: *mut GLint) {
    unsafe { global().GetShaderPrecisionFormat(shadertype, precisiontype, range, precision) }
}

/// # Safety
/// See [`Gl::ReleaseShaderCompiler`]; the global context must be initialized.
#[inline]
pub unsafe fn ReleaseShaderCompiler() {
    unsafe { global().ReleaseShaderCompiler() }
}

/// # Safety
/// See [`Gl::ShaderBinary`]; the global context must be initialized.
#[inline]
pub unsafe fn ShaderBinary(count: GLsizei, shaders: *const GLuint, binaryFormat: GLenum, binary: *const c_void, length: GLsizei) {
    unsafe { global().ShaderBinary(count, shaders, binaryFormat, binary, length) }
}

/// # Safety
/// See [`Gl::BindTransformFeedback`]; the global context must be initialized.
#[inline]
pub unsafe fn BindTransformFeedback(target: GLenum, id: GLuint) {
    unsafe { global().BindTransformFeedback(target, id) }
}

/// # Safety
/// See [`Gl::DeleteTransformFeedbacks`]; the global context must be initialized.
#[inline]
pub unsafe fn DeleteTransformFeedbacks(n: GLsizei, ids: *const GLuint) {
    unsafe { global().DeleteTransformFeedbacks(n, ids) }
}

/// # Safety
/// See [`Gl::GenTransformFeedbacks`]; the global context must be initialized.
#[inline]
pub unsafe fn GenTransformFeedbacks(n: GLsizei, ids: *mut GLuint) {
    unsafe { global().GenTransformFeedbacks(n, ids) }
}

/// # Safety
/// See [`Gl::GetInternalformativ`]; the global context must be initialized.
#[inline]
pub unsafe fn GetInternalformativ(target: GLenum, internalformat: GLenum, pname: GLenum, count: GLsizei, params: *mut GLint) {
    unsafe { global().GetInternalformativ(target, internalformat, pname, count, params) }
}

/// # Safety
/// See [`Gl::GetProgramBinary`]; the global context must be initialized.
#[inline]
pub unsafe fn GetProgramBinary(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut c_void) {
    unsafe { global().GetProgramBinary(program, bufSize, length, binaryFormat, binary) }
}

/// # Safety
/// See [`Gl::InvalidateFramebuffer`]; the global context must be initialized.
#[inline]
pub unsafe fn InvalidateFramebuffer(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum) {
    unsafe { global().InvalidateFramebuffer(target, numAttachments, attachments) }
}

/// # Safety
/// See [`Gl::InvalidateSubFramebuffer`]; the global context must be initialized.
#[inline]
pub unsafe fn InvalidateSubFramebuffer(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    unsafe { global().InvalidateSubFramebuffer(target, numAttachments, attachments, x, y, width, height) }
}

/// # Safety
/// See [`Gl::IsTransformFeedback`]; the global context must be initialized.
#[inline]
pub unsafe fn IsTransformFeedback(id: GLuint) -> GLboolean {
    unsafe { global().IsTransformFeedback(id) }
}

/// # Safety
/// See [`Gl::PauseTransformFeedback`]; the global context must be initialized.
#[inline]
pub unsafe fn PauseTransformFeedback() {
    unsafe { global().PauseTransformFeedback() }
}

/// # Safety
/// See [`Gl::ProgramBinary`]; the global context must be initialized.
#[inline]
pub unsafe fn ProgramBinary(program: GLuint, binaryFormat: GLenum, binary: *const c_void, length: GLsizei) {
    unsafe { global().ProgramBinary(program, binaryFormat, binary, length) }
}

/// # Safety
/// See [`Gl::ProgramParameteri`]; the global context must be initialized.
#[inline]
pub unsafe fn ProgramParameteri(program: GLuint, pname: GLenum, value: GLint) {
    unsafe { global().ProgramParameteri(program, pname, value) }
}

/// # Safety
/// See [`Gl::ResumeTransformFeedback`]; the global context must be initialized.
#[inline]
pub unsafe fn ResumeTransformFeedback() {
    unsafe { global().ResumeTransformFeedback() }
}

/// # Safety
/// See [`Gl::TexStorage2D`]; the global context must be initialized.
#[inline]
pub unsafe fn TexStorage2D(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) {
    unsafe { global().TexStorage2D(target, levels, internalformat, width, height) }
}

/// # Safety
/// See [`Gl::TexStorage3D`]; the global context must be initialized.
#[inline]
pub unsafe fn TexStorage3D(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei) {
    unsafe { global().TexStorage3D(target, levels, internalformat, width, height, depth) }
}

/// # Safety
/// See [`Gl::GetPointerv`]; the global context must be initialized.
#[inline]
pub unsafe fn GetPointerv(pname: GLenum, params: *mut *mut c_void) {
    unsafe { global().GetPointerv(pname, params) }
}

/// # Safety
/// See [`Gl::DebugMessageCallback`]; the global context must be initialized.
#[inline]
pub unsafe fn DebugMessageCallback(callback: GLDEBUGPROC, userParam: *const c_void) {
    unsafe { global().DebugMessageCallback(callback, userParam) }
}

/// # Safety
/// See [`Gl::DebugMessageCallbackKHR`]; the global context must be initialized.
#[inline]
pub unsafe fn DebugMessageCallbackKHR(callback: GLDEBUGPROCKHR, userParam: *const c_void) {
    unsafe { global().DebugMessageCallbackKHR(callback, userParam) }
}

/// # Safety
/// See [`Gl::DebugMessageControl`]; the global context must be initialized.
#[inline]
pub unsafe fn DebugMessageControl(source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean) {
    unsafe { global().DebugMessageControl(source, type_, severity, count, ids, enabled) }
}

/// # Safety
/// See [`Gl::DebugMessageControlKHR`]; the global context must be initialized.
#[inline]
pub unsafe fn DebugMessageControlKHR(source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean) {
    unsafe { global().DebugMessageControlKHR(source, type_, severity, count, ids, enabled) }
}

/// # Safety
/// See [`Gl::DebugMessageInsert`]; the global context must be initialized.
#[inline]
pub unsafe fn DebugMessageInsert(source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar) {
    unsafe { global().DebugMessageInsert(source, type_, id, severity, length, buf) }
}

/// # Safety
/// See [`Gl::DebugMessageInsertKHR`]; the global context must be initialized.
#[inline]
pub unsafe fn DebugMessageInsertKHR(source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar) {
    unsafe { global().DebugMessageInsertKHR(source, type_, id, severity, length, buf) }
}

/// # Safety
/// See [`Gl::GetDebugMessageLog`]; the global context must be initialized.
#[inline]
pub unsafe fn GetDebugMessageLog(count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint {
    unsafe { global().GetDebugMessageLog(count, bufSize, sources, types, ids, severities, lengths, messageLog) }
}

/// # Safety
/// See [`Gl::GetDebugMessageLogKHR`]; the global context must be initialized.
#[inline]
pub unsafe fn GetDebugMessageLogKHR(count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint {
    unsafe { global().GetDebugMessageLogKHR(count, bufSize, sources, types, ids, severities, lengths, messageLog) }
}

/// # Safety
/// See [`Gl::GetObjectLabel`]; the global context must be initialized.
#[inline]
pub unsafe fn GetObjectLabel(identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) {
    unsafe { global().GetObjectLabel(identifier, name, bufSize, length, label) }
}

/// # Safety
/// See [`Gl::GetObjectLabelKHR`]; the global context must be initialized.
#[inline]
pub unsafe fn GetObjectLabelKHR(identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) {
    unsafe { global().GetObjectLabelKHR(identifier, name, bufSize, length, label) }
}

/// # Safety
/// See [`Gl::GetObjectPtrLabel`]; the global context must be initialized.
#[inline]
pub unsafe fn GetObjectPtrLabel(ptr: *const c_void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) {
    unsafe { global().GetObjectPtrLabel(ptr, bufSize, length, label) }
}

/// # Safety
/// See [`Gl::GetObjectPtrLabelKHR`]; the global context must be initialized.
#[inline]
pub unsafe fn GetObjectPtrLabelKHR(ptr: *const c_void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) {
    unsafe { global().GetObjectPtrLabelKHR(ptr, bufSize, length, label) }
}

/// # Safety
/// See [`Gl::GetPointervKHR`]; the global context must be initialized.
#[inline]
pub unsafe fn GetPointervKHR(pname: GLenum, params: *mut *mut c_void) {
    unsafe { global().GetPointervKHR(pname, params) }
}

/// # Safety
/// See [`Gl::ObjectLabel`]; the global context must be initialized.
#[inline]
pub unsafe fn ObjectLabel(identifier: GLenum, name: GLuint, length: GLsizei, label: *const GLchar) {
    unsafe { global().ObjectLabel(identifier, name, length, label) }
}

/// # Safety
/// See [`Gl::ObjectLabelKHR`]; the global context must be initialized.
#[inline]
pub unsafe fn ObjectLabelKHR(identifier: GLenum, name: GLuint, length: GLsizei, label: *const GLchar) {
    unsafe { global().ObjectLabelKHR(identifier, name, length, label) }
}

/// # Safety
/// See [`Gl::ObjectPtrLabel`]; the global context must be initialized.
#[inline]
pub unsafe fn ObjectPtrLabel(ptr: *const c_void, length: GLsizei, label: *const GLchar) {
    unsafe { global().ObjectPtrLabel(ptr, length, label) }
}

/// # Safety
/// See [`Gl::ObjectPtrLabelKHR`]; the global context must be initialized.
#[inline]
pub unsafe fn ObjectPtrLabelKHR(ptr: *const c_void, length: GLsizei, label: *const GLchar) {
    unsafe { global().ObjectPtrLabelKHR(ptr, length, label) }
}

/// # Safety
/// See [`Gl::PopDebugGroup`]; the global context must be initialized.
#[inline]
pub unsafe fn PopDebugGroup() {
    unsafe { global().PopDebugGroup() }
}

/// # Safety
/// See [`Gl::PopDebugGroupKHR`]; the global context must be initialized.
#[inline]
pub unsafe fn PopDebugGroupKHR() {
    unsafe { global().PopDebugGroupKHR() }
}

/// # Safety
/// See [`Gl::PushDebugGroup`]; the global context must be initialized.
#[inline]
pub unsafe fn PushDebugGroup(source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar) {
    unsafe { global().PushDebugGroup(source, id, length, message) }
}

/// # Safety
/// See [`Gl::PushDebugGroupKHR`]; the global context must be initialized.
#[inline]
pub unsafe fn PushDebugGroupKHR(source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar) {
    unsafe { global().PushDebugGroupKHR(source, id, length, message) }
}

/// Whether the global context's driver advertises `GL_KHR_debug`.
#[inline]
pub fn KHR_debug() -> bool {
    global().KHR_debug()
}

/// Whether the global context's driver supports `GL_VERSION_1_0`.
#[inline]
pub fn VERSION_1_0() -> bool {
    global().VERSION_1_0()
}

/// Whether the global context's driver supports `GL_VERSION_1_1`.
#[inline]
pub fn VERSION_1_1() -> bool {
    global().VERSION_1_1()
}

/// Whether the global context's driver supports `GL_VERSION_1_2`.
#[inline]
pub fn VERSION_1_2() -> bool {
    global().VERSION_1_2()
}

/// Whether the global context's driver supports `GL_VERSION_1_3`.
#[inline]
pub fn VERSION_1_3() -> bool {
    global().VERSION_1_3()
}

/// Whether the global context's driver supports `GL_VERSION_1_4`.
#[inline]
pub fn VERSION_1_4() -> bool {
    global().VERSION_1_4()
}

/// Whether the global context's driver supports `GL_VERSION_1_5`.
#[inline]
pub fn VERSION_1_5() -> bool {
    global().VERSION_1_5()
}

/// Whether the global context's driver supports `GL_VERSION_2_0`.
#[inline]
pub fn VERSION_2_0() -> bool {
    global().VERSION_2_0()
}

/// Whether the global context's driver supports `GL_VERSION_2_1`.
#[inline]
pub fn VERSION_2_1() -> bool {
    global().VERSION_2_1()
}

/// Whether the global context's driver supports `GL_VERSION_3_0`.
#[inline]
pub fn VERSION_3_0() -> bool {
    global().VERSION_3_0()
}

/// Whether the global context's driver supports `GL_VERSION_3_1`.
#[inline]
pub fn VERSION_3_1() -> bool {
    global().VERSION_3_1()
}

/// Whether the global context's driver supports `GL_VERSION_3_2`.
#[inline]
pub fn VERSION_3_2() -> bool {
    global().VERSION_3_2()
}

/// Whether the global context's driver supports `GL_VERSION_3_3`.
#[inline]
pub fn VERSION_3_3() -> bool {
    global().VERSION_3_3()
}

/// Whether the global context's driver supports `GL_ES_VERSION_2_0`.
#[inline]
pub fn ES_VERSION_2_0() -> bool {
    global().ES_VERSION_2_0()
}

/// Whether the global context's driver supports `GL_ES_VERSION_3_0`.
#[inline]
pub fn ES_VERSION_3_0() -> bool {
    global().ES_VERSION_3_0()
}

/// Detected API version of the global context (`major << 8 | minor`;
/// 0 before [`init_global`]).
#[inline]
pub fn version() -> u32 {
    global().version()
}
