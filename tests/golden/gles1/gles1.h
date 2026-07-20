#ifndef GLOAM_GLES1_H
#define GLOAM_GLES1_H

#ifdef __clang__
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreserved-id-macro"
#endif
#ifdef __gl_h_
  #error OpenGL ES 1 (gl.h) header already included (API: gles1), remove previous include!
#endif
#define __gl_h_ 1
#ifdef __gles1_gl_h_
  #error OpenGL ES 1 header already included (API: gles1), remove previous include!
#endif
#define __gles1_gl_h_ 1
#ifdef __clang__
#pragma clang diagnostic pop
#endif

/* ---- Platform detection ------------------------------------------------ */
#ifndef GLOAM_PLATFORM_DETECTED_
#define GLOAM_PLATFORM_DETECTED_
#if defined(__CYGWIN__) || defined(_WIN32)
#  define GLOAM_PLATFORM_WINDOWS 1
#endif
#if defined(__linux)
#  define GLOAM_PLATFORM_LINUX 1
#endif
#endif /* GLOAM_PLATFORM_DETECTED_ */

#ifndef GLOAM_HAS_ENUM_BASE_TYPE
#if defined(__cplusplus) || (defined(__STDC_VERSION__) && __STDC_VERSION__ >= 202311L)
#  define GLOAM_HAS_ENUM_BASE_TYPE 1
#else
#  define GLOAM_HAS_ENUM_BASE_TYPE 0
#endif
#endif

#include <stddef.h>
#include <stdint.h>

#include "KHR/khrplatform.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Calling convention */
#ifndef APIENTRY
#  if defined(GLOAM_PLATFORM_WINDOWS)
#    define APIENTRY __stdcall
#  else
#    define APIENTRY
#  endif
#endif
#ifndef APIENTRYP
#  define APIENTRYP APIENTRY *
#endif
#ifndef GLAPI
#  define GLAPI extern
#endif

/* Calling convention for GL callback function pointers (e.g. debug callbacks,
 * blob cache functions). Piggybacks on APIENTRY if already defined so that
 * code which defines APIENTRY before including this header gets consistent
 * behaviour. Guard against redefinition so multiple gloam headers included
 * in the same translation unit don't conflict.
 */
#ifndef GLOAM_API_PTR
#  ifdef APIENTRY
#    define GLOAM_API_PTR APIENTRY
#  elif defined(GLOAM_PLATFORM_WINDOWS)
#    define GLOAM_API_PTR __stdcall
#  else
#    define GLOAM_API_PTR
#  endif
#endif
#ifndef GLAPIENTRY
#  define GLAPIENTRY GLOAM_API_PTR
#endif

/* Forward declarations for OpenCL interop types. */
struct _cl_context;
struct _cl_event;

/* ---- Version feature guards ----------------------------------------------
 * These mirror the upstream vulkan_core.h / gl.h definitions so that code
 * guarded by e.g. #ifdef GL_VERSION_3_3 compiles correctly against this
 * header.
 */
#define GL_VERSION_ES_CM_1_0 1

/* ---- Extension compile-time guards ---------------------------------------
 * These mirror the definitions in standard glext.h/gl2ext.h/eglext.h so
 * that code guarded by e.g. #ifdef GL_ARB_draw_indirect compiles correctly
 * against this header.
 */
#define GL_OES_framebuffer_object 1

/* ---- Constants ----------------------------------------------------------- */
#define GL_DEPTH_BUFFER_BIT 0x00000100
#define GL_STENCIL_BUFFER_BIT 0x00000400
#define GL_COLOR_BUFFER_BIT 0x00004000
#define GL_FALSE 0
#define GL_NO_ERROR 0
#define GL_ZERO 0
#define GL_NONE_OES 0
#define GL_TRUE 1
#define GL_ONE 1
#define GL_VERSION_ES_CL_1_0 1 /* Not an API enum. API definition macro for ES 1.0/1.1 headers */
#define GL_VERSION_ES_CM_1_1 1 /* Not an API enum. API definition macro for ES 1.0/1.1 headers */
#define GL_VERSION_ES_CL_1_1 1 /* Not an API enum. API definition macro for ES 1.0/1.1 headers */
#define GL_POINTS 0x0000
#define GL_LINES 0x0001
#define GL_LINE_LOOP 0x0002
#define GL_LINE_STRIP 0x0003
#define GL_TRIANGLES 0x0004
#define GL_TRIANGLE_STRIP 0x0005
#define GL_TRIANGLE_FAN 0x0006
#define GL_ADD 0x0104
#define GL_NEVER 0x0200
#define GL_LESS 0x0201
#define GL_EQUAL 0x0202
#define GL_LEQUAL 0x0203
#define GL_GREATER 0x0204
#define GL_NOTEQUAL 0x0205
#define GL_GEQUAL 0x0206
#define GL_ALWAYS 0x0207
#define GL_SRC_COLOR 0x0300
#define GL_ONE_MINUS_SRC_COLOR 0x0301
#define GL_SRC_ALPHA 0x0302
#define GL_ONE_MINUS_SRC_ALPHA 0x0303
#define GL_DST_ALPHA 0x0304
#define GL_ONE_MINUS_DST_ALPHA 0x0305
#define GL_DST_COLOR 0x0306
#define GL_ONE_MINUS_DST_COLOR 0x0307
#define GL_SRC_ALPHA_SATURATE 0x0308
#define GL_FRONT 0x0404
#define GL_BACK 0x0405
#define GL_FRONT_AND_BACK 0x0408
#define GL_INVALID_ENUM 0x0500
#define GL_INVALID_VALUE 0x0501
#define GL_INVALID_OPERATION 0x0502
#define GL_STACK_OVERFLOW 0x0503
#define GL_STACK_UNDERFLOW 0x0504
#define GL_OUT_OF_MEMORY 0x0505
#define GL_INVALID_FRAMEBUFFER_OPERATION_OES 0x0506
#define GL_EXP 0x0800
#define GL_EXP2 0x0801
#define GL_CW 0x0900
#define GL_CCW 0x0901
#define GL_CURRENT_COLOR 0x0B00
#define GL_CURRENT_NORMAL 0x0B02
#define GL_CURRENT_TEXTURE_COORDS 0x0B03
#define GL_POINT_SMOOTH 0x0B10
#define GL_POINT_SIZE 0x0B11
#define GL_SMOOTH_POINT_SIZE_RANGE 0x0B12
#define GL_LINE_SMOOTH 0x0B20
#define GL_LINE_WIDTH 0x0B21
#define GL_SMOOTH_LINE_WIDTH_RANGE 0x0B22
#define GL_CULL_FACE 0x0B44
#define GL_CULL_FACE_MODE 0x0B45
#define GL_FRONT_FACE 0x0B46
#define GL_LIGHTING 0x0B50
#define GL_LIGHT_MODEL_TWO_SIDE 0x0B52
#define GL_LIGHT_MODEL_AMBIENT 0x0B53
#define GL_SHADE_MODEL 0x0B54
#define GL_COLOR_MATERIAL 0x0B57
#define GL_FOG 0x0B60
#define GL_FOG_DENSITY 0x0B62
#define GL_FOG_START 0x0B63
#define GL_FOG_END 0x0B64
#define GL_FOG_MODE 0x0B65
#define GL_FOG_COLOR 0x0B66
#define GL_DEPTH_RANGE 0x0B70
#define GL_DEPTH_TEST 0x0B71
#define GL_DEPTH_WRITEMASK 0x0B72
#define GL_DEPTH_CLEAR_VALUE 0x0B73
#define GL_DEPTH_FUNC 0x0B74
#define GL_STENCIL_TEST 0x0B90
#define GL_STENCIL_CLEAR_VALUE 0x0B91
#define GL_STENCIL_FUNC 0x0B92
#define GL_STENCIL_VALUE_MASK 0x0B93
#define GL_STENCIL_FAIL 0x0B94
#define GL_STENCIL_PASS_DEPTH_FAIL 0x0B95
#define GL_STENCIL_PASS_DEPTH_PASS 0x0B96
#define GL_STENCIL_REF 0x0B97
#define GL_STENCIL_WRITEMASK 0x0B98
#define GL_MATRIX_MODE 0x0BA0
#define GL_NORMALIZE 0x0BA1
#define GL_VIEWPORT 0x0BA2
#define GL_MODELVIEW_STACK_DEPTH 0x0BA3
#define GL_PROJECTION_STACK_DEPTH 0x0BA4
#define GL_TEXTURE_STACK_DEPTH 0x0BA5
#define GL_MODELVIEW_MATRIX 0x0BA6
#define GL_PROJECTION_MATRIX 0x0BA7
#define GL_TEXTURE_MATRIX 0x0BA8
#define GL_ALPHA_TEST 0x0BC0
#define GL_ALPHA_TEST_FUNC 0x0BC1
#define GL_ALPHA_TEST_REF 0x0BC2
#define GL_DITHER 0x0BD0
#define GL_BLEND_DST 0x0BE0
#define GL_BLEND_SRC 0x0BE1
#define GL_BLEND 0x0BE2
#define GL_LOGIC_OP_MODE 0x0BF0
#define GL_COLOR_LOGIC_OP 0x0BF2
#define GL_SCISSOR_BOX 0x0C10
#define GL_SCISSOR_TEST 0x0C11
#define GL_COLOR_CLEAR_VALUE 0x0C22
#define GL_COLOR_WRITEMASK 0x0C23
#define GL_PERSPECTIVE_CORRECTION_HINT 0x0C50
#define GL_POINT_SMOOTH_HINT 0x0C51
#define GL_LINE_SMOOTH_HINT 0x0C52
#define GL_FOG_HINT 0x0C54
#define GL_UNPACK_ALIGNMENT 0x0CF5
#define GL_PACK_ALIGNMENT 0x0D05
#define GL_ALPHA_SCALE 0x0D1C
#define GL_MAX_LIGHTS 0x0D31
#define GL_MAX_CLIP_PLANES 0x0D32
#define GL_MAX_TEXTURE_SIZE 0x0D33
#define GL_MAX_MODELVIEW_STACK_DEPTH 0x0D36
#define GL_MAX_PROJECTION_STACK_DEPTH 0x0D38
#define GL_MAX_TEXTURE_STACK_DEPTH 0x0D39
#define GL_MAX_VIEWPORT_DIMS 0x0D3A
#define GL_SUBPIXEL_BITS 0x0D50
#define GL_RED_BITS 0x0D52
#define GL_GREEN_BITS 0x0D53
#define GL_BLUE_BITS 0x0D54
#define GL_ALPHA_BITS 0x0D55
#define GL_DEPTH_BITS 0x0D56
#define GL_STENCIL_BITS 0x0D57
#define GL_TEXTURE_2D 0x0DE1
#define GL_DONT_CARE 0x1100
#define GL_FASTEST 0x1101
#define GL_NICEST 0x1102
#define GL_AMBIENT 0x1200
#define GL_DIFFUSE 0x1201
#define GL_SPECULAR 0x1202
#define GL_POSITION 0x1203
#define GL_SPOT_DIRECTION 0x1204
#define GL_SPOT_EXPONENT 0x1205
#define GL_SPOT_CUTOFF 0x1206
#define GL_CONSTANT_ATTENUATION 0x1207
#define GL_LINEAR_ATTENUATION 0x1208
#define GL_QUADRATIC_ATTENUATION 0x1209
#define GL_BYTE 0x1400
#define GL_UNSIGNED_BYTE 0x1401
#define GL_SHORT 0x1402
#define GL_UNSIGNED_SHORT 0x1403
#define GL_FLOAT 0x1406
#define GL_FIXED 0x140C
#define GL_CLEAR 0x1500
#define GL_AND 0x1501
#define GL_AND_REVERSE 0x1502
#define GL_COPY 0x1503
#define GL_AND_INVERTED 0x1504
#define GL_NOOP 0x1505
#define GL_XOR 0x1506
#define GL_OR 0x1507
#define GL_NOR 0x1508
#define GL_EQUIV 0x1509
#define GL_INVERT 0x150A
#define GL_OR_REVERSE 0x150B
#define GL_COPY_INVERTED 0x150C
#define GL_OR_INVERTED 0x150D
#define GL_NAND 0x150E
#define GL_SET 0x150F
#define GL_EMISSION 0x1600
#define GL_SHININESS 0x1601
#define GL_AMBIENT_AND_DIFFUSE 0x1602
#define GL_MODELVIEW 0x1700
#define GL_PROJECTION 0x1701
#define GL_TEXTURE 0x1702
#define GL_ALPHA 0x1906
#define GL_RGB 0x1907
#define GL_RGBA 0x1908
#define GL_LUMINANCE 0x1909
#define GL_LUMINANCE_ALPHA 0x190A
#define GL_FLAT 0x1D00
#define GL_SMOOTH 0x1D01
#define GL_KEEP 0x1E00
#define GL_REPLACE 0x1E01
#define GL_INCR 0x1E02
#define GL_DECR 0x1E03
#define GL_VENDOR 0x1F00
#define GL_RENDERER 0x1F01
#define GL_VERSION 0x1F02
#define GL_EXTENSIONS 0x1F03
#define GL_MODULATE 0x2100
#define GL_DECAL 0x2101
#define GL_TEXTURE_ENV_MODE 0x2200
#define GL_TEXTURE_ENV_COLOR 0x2201
#define GL_TEXTURE_ENV 0x2300
#define GL_NEAREST 0x2600
#define GL_LINEAR 0x2601
#define GL_NEAREST_MIPMAP_NEAREST 0x2700
#define GL_LINEAR_MIPMAP_NEAREST 0x2701
#define GL_NEAREST_MIPMAP_LINEAR 0x2702
#define GL_LINEAR_MIPMAP_LINEAR 0x2703
#define GL_TEXTURE_MAG_FILTER 0x2800
#define GL_TEXTURE_MIN_FILTER 0x2801
#define GL_TEXTURE_WRAP_S 0x2802
#define GL_TEXTURE_WRAP_T 0x2803
#define GL_REPEAT 0x2901
#define GL_POLYGON_OFFSET_UNITS 0x2A00
#define GL_CLIP_PLANE0 0x3000
#define GL_CLIP_PLANE1 0x3001
#define GL_CLIP_PLANE2 0x3002
#define GL_CLIP_PLANE3 0x3003
#define GL_CLIP_PLANE4 0x3004
#define GL_CLIP_PLANE5 0x3005
#define GL_LIGHT0 0x4000
#define GL_LIGHT1 0x4001
#define GL_LIGHT2 0x4002
#define GL_LIGHT3 0x4003
#define GL_LIGHT4 0x4004
#define GL_LIGHT5 0x4005
#define GL_LIGHT6 0x4006
#define GL_LIGHT7 0x4007
#define GL_UNSIGNED_SHORT_4_4_4_4 0x8033
#define GL_UNSIGNED_SHORT_5_5_5_1 0x8034
#define GL_POLYGON_OFFSET_FILL 0x8037
#define GL_POLYGON_OFFSET_FACTOR 0x8038
#define GL_RESCALE_NORMAL 0x803A
#define GL_RGBA4_OES 0x8056
#define GL_RGB5_A1_OES 0x8057
#define GL_TEXTURE_BINDING_2D 0x8069
#define GL_VERTEX_ARRAY 0x8074
#define GL_NORMAL_ARRAY 0x8075
#define GL_COLOR_ARRAY 0x8076
#define GL_TEXTURE_COORD_ARRAY 0x8078
#define GL_VERTEX_ARRAY_SIZE 0x807A
#define GL_VERTEX_ARRAY_TYPE 0x807B
#define GL_VERTEX_ARRAY_STRIDE 0x807C
#define GL_NORMAL_ARRAY_TYPE 0x807E
#define GL_NORMAL_ARRAY_STRIDE 0x807F
#define GL_COLOR_ARRAY_SIZE 0x8081
#define GL_COLOR_ARRAY_TYPE 0x8082
#define GL_COLOR_ARRAY_STRIDE 0x8083
#define GL_TEXTURE_COORD_ARRAY_SIZE 0x8088
#define GL_TEXTURE_COORD_ARRAY_TYPE 0x8089
#define GL_TEXTURE_COORD_ARRAY_STRIDE 0x808A
#define GL_VERTEX_ARRAY_POINTER 0x808E
#define GL_NORMAL_ARRAY_POINTER 0x808F
#define GL_COLOR_ARRAY_POINTER 0x8090
#define GL_TEXTURE_COORD_ARRAY_POINTER 0x8092
#define GL_MULTISAMPLE 0x809D
#define GL_SAMPLE_ALPHA_TO_COVERAGE 0x809E
#define GL_SAMPLE_ALPHA_TO_ONE 0x809F
#define GL_SAMPLE_COVERAGE 0x80A0
#define GL_SAMPLE_BUFFERS 0x80A8
#define GL_SAMPLES 0x80A9
#define GL_SAMPLE_COVERAGE_VALUE 0x80AA
#define GL_SAMPLE_COVERAGE_INVERT 0x80AB
#define GL_POINT_SIZE_MIN 0x8126
#define GL_POINT_SIZE_MAX 0x8127
#define GL_POINT_FADE_THRESHOLD_SIZE 0x8128
#define GL_POINT_DISTANCE_ATTENUATION 0x8129
#define GL_CLAMP_TO_EDGE 0x812F
#define GL_GENERATE_MIPMAP 0x8191
#define GL_GENERATE_MIPMAP_HINT 0x8192
#define GL_DEPTH_COMPONENT16_OES 0x81A5
#define GL_UNSIGNED_SHORT_5_6_5 0x8363
#define GL_ALIASED_POINT_SIZE_RANGE 0x846D
#define GL_ALIASED_LINE_WIDTH_RANGE 0x846E
#define GL_TEXTURE0 0x84C0
#define GL_TEXTURE1 0x84C1
#define GL_TEXTURE2 0x84C2
#define GL_TEXTURE3 0x84C3
#define GL_TEXTURE4 0x84C4
#define GL_TEXTURE5 0x84C5
#define GL_TEXTURE6 0x84C6
#define GL_TEXTURE7 0x84C7
#define GL_TEXTURE8 0x84C8
#define GL_TEXTURE9 0x84C9
#define GL_TEXTURE10 0x84CA
#define GL_TEXTURE11 0x84CB
#define GL_TEXTURE12 0x84CC
#define GL_TEXTURE13 0x84CD
#define GL_TEXTURE14 0x84CE
#define GL_TEXTURE15 0x84CF
#define GL_TEXTURE16 0x84D0
#define GL_TEXTURE17 0x84D1
#define GL_TEXTURE18 0x84D2
#define GL_TEXTURE19 0x84D3
#define GL_TEXTURE20 0x84D4
#define GL_TEXTURE21 0x84D5
#define GL_TEXTURE22 0x84D6
#define GL_TEXTURE23 0x84D7
#define GL_TEXTURE24 0x84D8
#define GL_TEXTURE25 0x84D9
#define GL_TEXTURE26 0x84DA
#define GL_TEXTURE27 0x84DB
#define GL_TEXTURE28 0x84DC
#define GL_TEXTURE29 0x84DD
#define GL_TEXTURE30 0x84DE
#define GL_TEXTURE31 0x84DF
#define GL_ACTIVE_TEXTURE 0x84E0
#define GL_CLIENT_ACTIVE_TEXTURE 0x84E1
#define GL_MAX_TEXTURE_UNITS 0x84E2
#define GL_SUBTRACT 0x84E7
#define GL_MAX_RENDERBUFFER_SIZE_OES 0x84E8
#define GL_COMBINE 0x8570
#define GL_COMBINE_RGB 0x8571
#define GL_COMBINE_ALPHA 0x8572
#define GL_RGB_SCALE 0x8573
#define GL_ADD_SIGNED 0x8574
#define GL_INTERPOLATE 0x8575
#define GL_CONSTANT 0x8576
#define GL_PRIMARY_COLOR 0x8577
#define GL_PREVIOUS 0x8578
#define GL_SRC0_RGB 0x8580
#define GL_SRC1_RGB 0x8581
#define GL_SRC2_RGB 0x8582
#define GL_SRC0_ALPHA 0x8588
#define GL_SRC1_ALPHA 0x8589
#define GL_SRC2_ALPHA 0x858A
#define GL_OPERAND0_RGB 0x8590
#define GL_OPERAND1_RGB 0x8591
#define GL_OPERAND2_RGB 0x8592
#define GL_OPERAND0_ALPHA 0x8598
#define GL_OPERAND1_ALPHA 0x8599
#define GL_OPERAND2_ALPHA 0x859A
#define GL_NUM_COMPRESSED_TEXTURE_FORMATS 0x86A2
#define GL_COMPRESSED_TEXTURE_FORMATS 0x86A3
#define GL_DOT3_RGB 0x86AE
#define GL_DOT3_RGBA 0x86AF
#define GL_BUFFER_SIZE 0x8764
#define GL_BUFFER_USAGE 0x8765
#define GL_ARRAY_BUFFER 0x8892
#define GL_ELEMENT_ARRAY_BUFFER 0x8893
#define GL_ARRAY_BUFFER_BINDING 0x8894
#define GL_ELEMENT_ARRAY_BUFFER_BINDING 0x8895
#define GL_VERTEX_ARRAY_BUFFER_BINDING 0x8896
#define GL_NORMAL_ARRAY_BUFFER_BINDING 0x8897
#define GL_COLOR_ARRAY_BUFFER_BINDING 0x8898
#define GL_TEXTURE_COORD_ARRAY_BUFFER_BINDING 0x889A
#define GL_STATIC_DRAW 0x88E4
#define GL_DYNAMIC_DRAW 0x88E8
#define GL_FRAMEBUFFER_BINDING_OES 0x8CA6
#define GL_RENDERBUFFER_BINDING_OES 0x8CA7
#define GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE_OES 0x8CD0
#define GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME_OES 0x8CD1
#define GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL_OES 0x8CD2
#define GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE_OES 0x8CD3
#define GL_FRAMEBUFFER_COMPLETE_OES 0x8CD5
#define GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT_OES 0x8CD6
#define GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT_OES 0x8CD7
#define GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS_OES 0x8CD9
#define GL_FRAMEBUFFER_INCOMPLETE_FORMATS_OES 0x8CDA
#define GL_FRAMEBUFFER_UNSUPPORTED_OES 0x8CDD
#define GL_COLOR_ATTACHMENT0_OES 0x8CE0
#define GL_DEPTH_ATTACHMENT_OES 0x8D00
#define GL_STENCIL_ATTACHMENT_OES 0x8D20
#define GL_FRAMEBUFFER_OES 0x8D40
#define GL_RENDERBUFFER_OES 0x8D41
#define GL_RENDERBUFFER_WIDTH_OES 0x8D42
#define GL_RENDERBUFFER_HEIGHT_OES 0x8D43
#define GL_RENDERBUFFER_INTERNAL_FORMAT_OES 0x8D44
#define GL_RENDERBUFFER_RED_SIZE_OES 0x8D50
#define GL_RENDERBUFFER_GREEN_SIZE_OES 0x8D51
#define GL_RENDERBUFFER_BLUE_SIZE_OES 0x8D52
#define GL_RENDERBUFFER_ALPHA_SIZE_OES 0x8D53
#define GL_RENDERBUFFER_DEPTH_SIZE_OES 0x8D54
#define GL_RENDERBUFFER_STENCIL_SIZE_OES 0x8D55
#define GL_RGB565_OES 0x8D62

/* ---- Types ----------------------------------------------------------------
 * Emitted in topological dependency order. Consecutive types sharing the
 * same platform guard are coalesced into a single #ifdef/#endif block.
 */
typedef unsigned int GLenum;

typedef unsigned char GLboolean;

typedef unsigned int GLbitfield;

typedef void GLvoid;

typedef int GLint;

typedef unsigned int GLuint;

typedef int GLsizei;

typedef double GLdouble;

typedef double GLclampd;

typedef void *GLeglClientBufferEXT;

typedef void *GLeglImageOES;

typedef char GLchar;

typedef char GLcharARB;

#ifdef __APPLE__
typedef void *GLhandleARB;
#else
typedef unsigned int GLhandleARB;
#endif

typedef struct __GLsync *GLsync;

struct _cl_context;

struct _cl_event;

typedef unsigned short GLhalfNV;

typedef void (APIENTRY *GLVULKANPROCNV)(void);

typedef khronos_int8_t GLbyte;

typedef khronos_uint8_t GLubyte;

typedef khronos_int16_t GLshort;

typedef khronos_uint16_t GLushort;

typedef khronos_int32_t GLclampx;

typedef khronos_float_t GLfloat;

typedef khronos_float_t GLclampf;

typedef khronos_uint16_t GLhalf;

typedef khronos_uint16_t GLhalfARB;

typedef khronos_int32_t GLfixed;

#if defined(__ENVIRONMENT_MAC_OS_X_VERSION_MIN_REQUIRED__) \
&& (__ENVIRONMENT_MAC_OS_X_VERSION_MIN_REQUIRED__ > 1060)
typedef long GLintptr;
#else
typedef ptrdiff_t GLintptr;
#endif

#if defined(__ENVIRONMENT_MAC_OS_X_VERSION_MIN_REQUIRED__) \
&& (__ENVIRONMENT_MAC_OS_X_VERSION_MIN_REQUIRED__ > 1060)
typedef long GLintptrARB;
#else
typedef ptrdiff_t GLintptrARB;
#endif

#if defined(__ENVIRONMENT_MAC_OS_X_VERSION_MIN_REQUIRED__) \
&& (__ENVIRONMENT_MAC_OS_X_VERSION_MIN_REQUIRED__ > 1060)
typedef long GLsizeiptr;
#else
typedef ptrdiff_t GLsizeiptr;
#endif

#if defined(__ENVIRONMENT_MAC_OS_X_VERSION_MIN_REQUIRED__) \
&& (__ENVIRONMENT_MAC_OS_X_VERSION_MIN_REQUIRED__ > 1060)
typedef long GLsizeiptrARB;
#else
typedef ptrdiff_t GLsizeiptrARB;
#endif

typedef khronos_int64_t GLint64;

typedef khronos_int64_t GLint64EXT;

typedef khronos_uint64_t GLuint64;

typedef khronos_uint64_t GLuint64EXT;

typedef void (APIENTRY *GLDEBUGPROC)(GLenum source,GLenum type,GLuint id,GLenum severity,GLsizei length,const GLchar *message,const void *userParam);

typedef void (APIENTRY *GLDEBUGPROCARB)(GLenum source,GLenum type,GLuint id,GLenum severity,GLsizei length,const GLchar *message,const void *userParam);

typedef void (APIENTRY *GLDEBUGPROCKHR)(GLenum source,GLenum type,GLuint id,GLenum severity,GLsizei length,const GLchar *message,const void *userParam);

typedef void (APIENTRY *GLDEBUGPROCAMD)(GLuint id,GLenum category,GLenum severity,GLsizei length,const GLchar *message,void *userParam);

typedef GLintptr GLvdpauSurfaceNV;

typedef GLsizeiptr (APIENTRY *GLGETBLOBPROCANGLE)(const void *key, GLsizeiptr keySize, void *value, GLsizeiptr valueSize, const void *userParam);

typedef void (APIENTRY *GLSETBLOBPROCANGLE)(const void *key, GLsizeiptr keySize, const void *value, GLsizeiptr valueSize, const void *userParam);


/* ---- PFN typedefs -------------------------------------------------------- */
typedef void (APIENTRYP PFNGLACTIVETEXTUREPROC)(GLenum texture);
typedef void (APIENTRYP PFNGLALPHAFUNCPROC)(GLenum func, GLfloat ref);
typedef void (APIENTRYP PFNGLALPHAFUNCXPROC)(GLenum func, GLfixed ref);
typedef void (APIENTRYP PFNGLBINDBUFFERPROC)(GLenum target, GLuint buffer);
typedef void (APIENTRYP PFNGLBINDTEXTUREPROC)(GLenum target, GLuint texture);
typedef void (APIENTRYP PFNGLBLENDFUNCPROC)(GLenum sfactor, GLenum dfactor);
typedef void (APIENTRYP PFNGLBUFFERDATAPROC)(GLenum target, GLsizeiptr size, const void * data, GLenum usage);
typedef void (APIENTRYP PFNGLBUFFERSUBDATAPROC)(GLenum target, GLintptr offset, GLsizeiptr size, const void * data);
typedef void (APIENTRYP PFNGLCLEARPROC)(GLbitfield mask);
typedef void (APIENTRYP PFNGLCLEARCOLORPROC)(GLfloat red, GLfloat green, GLfloat blue, GLfloat alpha);
typedef void (APIENTRYP PFNGLCLEARCOLORXPROC)(GLfixed red, GLfixed green, GLfixed blue, GLfixed alpha);
typedef void (APIENTRYP PFNGLCLEARDEPTHFPROC)(GLfloat d);
typedef void (APIENTRYP PFNGLCLEARDEPTHXPROC)(GLfixed depth);
typedef void (APIENTRYP PFNGLCLEARSTENCILPROC)(GLint s);
typedef void (APIENTRYP PFNGLCLIENTACTIVETEXTUREPROC)(GLenum texture);
typedef void (APIENTRYP PFNGLCLIPPLANEFPROC)(GLenum p, const GLfloat * eqn);
typedef void (APIENTRYP PFNGLCLIPPLANEXPROC)(GLenum plane, const GLfixed * equation);
typedef void (APIENTRYP PFNGLCOLOR4FPROC)(GLfloat red, GLfloat green, GLfloat blue, GLfloat alpha);
typedef void (APIENTRYP PFNGLCOLOR4UBPROC)(GLubyte red, GLubyte green, GLubyte blue, GLubyte alpha);
typedef void (APIENTRYP PFNGLCOLOR4XPROC)(GLfixed red, GLfixed green, GLfixed blue, GLfixed alpha);
typedef void (APIENTRYP PFNGLCOLORMASKPROC)(GLboolean red, GLboolean green, GLboolean blue, GLboolean alpha);
typedef void (APIENTRYP PFNGLCOLORPOINTERPROC)(GLint size, GLenum type, GLsizei stride, const void * pointer);
typedef void (APIENTRYP PFNGLCOMPRESSEDTEXIMAGE2DPROC)(GLenum target, GLint level, GLenum internalformat, GLsizei width, GLsizei height, GLint border, GLsizei imageSize, const void * data);
typedef void (APIENTRYP PFNGLCOMPRESSEDTEXSUBIMAGE2DPROC)(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLsizei imageSize, const void * data);
typedef void (APIENTRYP PFNGLCOPYTEXIMAGE2DPROC)(GLenum target, GLint level, GLenum internalformat, GLint x, GLint y, GLsizei width, GLsizei height, GLint border);
typedef void (APIENTRYP PFNGLCOPYTEXSUBIMAGE2DPROC)(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint x, GLint y, GLsizei width, GLsizei height);
typedef void (APIENTRYP PFNGLCULLFACEPROC)(GLenum mode);
typedef void (APIENTRYP PFNGLDELETEBUFFERSPROC)(GLsizei n, const GLuint * buffers);
typedef void (APIENTRYP PFNGLDELETETEXTURESPROC)(GLsizei n, const GLuint * textures);
typedef void (APIENTRYP PFNGLDEPTHFUNCPROC)(GLenum func);
typedef void (APIENTRYP PFNGLDEPTHMASKPROC)(GLboolean flag);
typedef void (APIENTRYP PFNGLDEPTHRANGEFPROC)(GLfloat n, GLfloat f);
typedef void (APIENTRYP PFNGLDEPTHRANGEXPROC)(GLfixed n, GLfixed f);
typedef void (APIENTRYP PFNGLDISABLEPROC)(GLenum cap);
typedef void (APIENTRYP PFNGLDISABLECLIENTSTATEPROC)(GLenum array);
typedef void (APIENTRYP PFNGLDRAWARRAYSPROC)(GLenum mode, GLint first, GLsizei count);
typedef void (APIENTRYP PFNGLDRAWELEMENTSPROC)(GLenum mode, GLsizei count, GLenum type, const void * indices);
typedef void (APIENTRYP PFNGLENABLEPROC)(GLenum cap);
typedef void (APIENTRYP PFNGLENABLECLIENTSTATEPROC)(GLenum array);
typedef void (APIENTRYP PFNGLFINISHPROC)(void);
typedef void (APIENTRYP PFNGLFLUSHPROC)(void);
typedef void (APIENTRYP PFNGLFOGFPROC)(GLenum pname, GLfloat param);
typedef void (APIENTRYP PFNGLFOGFVPROC)(GLenum pname, const GLfloat * params);
typedef void (APIENTRYP PFNGLFOGXPROC)(GLenum pname, GLfixed param);
typedef void (APIENTRYP PFNGLFOGXVPROC)(GLenum pname, const GLfixed * param);
typedef void (APIENTRYP PFNGLFRONTFACEPROC)(GLenum mode);
typedef void (APIENTRYP PFNGLFRUSTUMFPROC)(GLfloat l, GLfloat r, GLfloat b, GLfloat t, GLfloat n, GLfloat f);
typedef void (APIENTRYP PFNGLFRUSTUMXPROC)(GLfixed l, GLfixed r, GLfixed b, GLfixed t, GLfixed n, GLfixed f);
typedef void (APIENTRYP PFNGLGENBUFFERSPROC)(GLsizei n, GLuint * buffers);
typedef void (APIENTRYP PFNGLGENTEXTURESPROC)(GLsizei n, GLuint * textures);
typedef void (APIENTRYP PFNGLGETBOOLEANVPROC)(GLenum pname, GLboolean * data);
typedef void (APIENTRYP PFNGLGETBUFFERPARAMETERIVPROC)(GLenum target, GLenum pname, GLint * params);
typedef void (APIENTRYP PFNGLGETCLIPPLANEFPROC)(GLenum plane, GLfloat * equation);
typedef void (APIENTRYP PFNGLGETCLIPPLANEXPROC)(GLenum plane, GLfixed * equation);
typedef GLenum (APIENTRYP PFNGLGETERRORPROC)(void);
typedef void (APIENTRYP PFNGLGETFIXEDVPROC)(GLenum pname, GLfixed * data);
typedef void (APIENTRYP PFNGLGETFLOATVPROC)(GLenum pname, GLfloat * data);
typedef void (APIENTRYP PFNGLGETINTEGERVPROC)(GLenum pname, GLint * data);
typedef void (APIENTRYP PFNGLGETLIGHTFVPROC)(GLenum light, GLenum pname, GLfloat * params);
typedef void (APIENTRYP PFNGLGETLIGHTXVPROC)(GLenum light, GLenum pname, GLfixed * params);
typedef void (APIENTRYP PFNGLGETMATERIALFVPROC)(GLenum face, GLenum pname, GLfloat * params);
typedef void (APIENTRYP PFNGLGETMATERIALXVPROC)(GLenum face, GLenum pname, GLfixed * params);
typedef void (APIENTRYP PFNGLGETPOINTERVPROC)(GLenum pname, void ** params);
typedef const GLubyte * (APIENTRYP PFNGLGETSTRINGPROC)(GLenum name);
typedef void (APIENTRYP PFNGLGETTEXENVFVPROC)(GLenum target, GLenum pname, GLfloat * params);
typedef void (APIENTRYP PFNGLGETTEXENVIVPROC)(GLenum target, GLenum pname, GLint * params);
typedef void (APIENTRYP PFNGLGETTEXENVXVPROC)(GLenum target, GLenum pname, GLfixed * params);
typedef void (APIENTRYP PFNGLGETTEXPARAMETERFVPROC)(GLenum target, GLenum pname, GLfloat * params);
typedef void (APIENTRYP PFNGLGETTEXPARAMETERIVPROC)(GLenum target, GLenum pname, GLint * params);
typedef void (APIENTRYP PFNGLGETTEXPARAMETERXVPROC)(GLenum target, GLenum pname, GLfixed * params);
typedef void (APIENTRYP PFNGLHINTPROC)(GLenum target, GLenum mode);
typedef GLboolean (APIENTRYP PFNGLISBUFFERPROC)(GLuint buffer);
typedef GLboolean (APIENTRYP PFNGLISENABLEDPROC)(GLenum cap);
typedef GLboolean (APIENTRYP PFNGLISTEXTUREPROC)(GLuint texture);
typedef void (APIENTRYP PFNGLLIGHTMODELFPROC)(GLenum pname, GLfloat param);
typedef void (APIENTRYP PFNGLLIGHTMODELFVPROC)(GLenum pname, const GLfloat * params);
typedef void (APIENTRYP PFNGLLIGHTMODELXPROC)(GLenum pname, GLfixed param);
typedef void (APIENTRYP PFNGLLIGHTMODELXVPROC)(GLenum pname, const GLfixed * param);
typedef void (APIENTRYP PFNGLLIGHTFPROC)(GLenum light, GLenum pname, GLfloat param);
typedef void (APIENTRYP PFNGLLIGHTFVPROC)(GLenum light, GLenum pname, const GLfloat * params);
typedef void (APIENTRYP PFNGLLIGHTXPROC)(GLenum light, GLenum pname, GLfixed param);
typedef void (APIENTRYP PFNGLLIGHTXVPROC)(GLenum light, GLenum pname, const GLfixed * params);
typedef void (APIENTRYP PFNGLLINEWIDTHPROC)(GLfloat width);
typedef void (APIENTRYP PFNGLLINEWIDTHXPROC)(GLfixed width);
typedef void (APIENTRYP PFNGLLOADIDENTITYPROC)(void);
typedef void (APIENTRYP PFNGLLOADMATRIXFPROC)(const GLfloat * m);
typedef void (APIENTRYP PFNGLLOADMATRIXXPROC)(const GLfixed * m);
typedef void (APIENTRYP PFNGLLOGICOPPROC)(GLenum opcode);
typedef void (APIENTRYP PFNGLMATERIALFPROC)(GLenum face, GLenum pname, GLfloat param);
typedef void (APIENTRYP PFNGLMATERIALFVPROC)(GLenum face, GLenum pname, const GLfloat * params);
typedef void (APIENTRYP PFNGLMATERIALXPROC)(GLenum face, GLenum pname, GLfixed param);
typedef void (APIENTRYP PFNGLMATERIALXVPROC)(GLenum face, GLenum pname, const GLfixed * param);
typedef void (APIENTRYP PFNGLMATRIXMODEPROC)(GLenum mode);
typedef void (APIENTRYP PFNGLMULTMATRIXFPROC)(const GLfloat * m);
typedef void (APIENTRYP PFNGLMULTMATRIXXPROC)(const GLfixed * m);
typedef void (APIENTRYP PFNGLMULTITEXCOORD4FPROC)(GLenum target, GLfloat s, GLfloat t, GLfloat r, GLfloat q);
typedef void (APIENTRYP PFNGLMULTITEXCOORD4XPROC)(GLenum texture, GLfixed s, GLfixed t, GLfixed r, GLfixed q);
typedef void (APIENTRYP PFNGLNORMAL3FPROC)(GLfloat nx, GLfloat ny, GLfloat nz);
typedef void (APIENTRYP PFNGLNORMAL3XPROC)(GLfixed nx, GLfixed ny, GLfixed nz);
typedef void (APIENTRYP PFNGLNORMALPOINTERPROC)(GLenum type, GLsizei stride, const void * pointer);
typedef void (APIENTRYP PFNGLORTHOFPROC)(GLfloat l, GLfloat r, GLfloat b, GLfloat t, GLfloat n, GLfloat f);
typedef void (APIENTRYP PFNGLORTHOXPROC)(GLfixed l, GLfixed r, GLfixed b, GLfixed t, GLfixed n, GLfixed f);
typedef void (APIENTRYP PFNGLPIXELSTOREIPROC)(GLenum pname, GLint param);
typedef void (APIENTRYP PFNGLPOINTPARAMETERFPROC)(GLenum pname, GLfloat param);
typedef void (APIENTRYP PFNGLPOINTPARAMETERFVPROC)(GLenum pname, const GLfloat * params);
typedef void (APIENTRYP PFNGLPOINTPARAMETERXPROC)(GLenum pname, GLfixed param);
typedef void (APIENTRYP PFNGLPOINTPARAMETERXVPROC)(GLenum pname, const GLfixed * params);
typedef void (APIENTRYP PFNGLPOINTSIZEPROC)(GLfloat size);
typedef void (APIENTRYP PFNGLPOINTSIZEXPROC)(GLfixed size);
typedef void (APIENTRYP PFNGLPOLYGONOFFSETPROC)(GLfloat factor, GLfloat units);
typedef void (APIENTRYP PFNGLPOLYGONOFFSETXPROC)(GLfixed factor, GLfixed units);
typedef void (APIENTRYP PFNGLPOPMATRIXPROC)(void);
typedef void (APIENTRYP PFNGLPUSHMATRIXPROC)(void);
typedef void (APIENTRYP PFNGLREADPIXELSPROC)(GLint x, GLint y, GLsizei width, GLsizei height, GLenum format, GLenum type, void * pixels);
typedef void (APIENTRYP PFNGLROTATEFPROC)(GLfloat angle, GLfloat x, GLfloat y, GLfloat z);
typedef void (APIENTRYP PFNGLROTATEXPROC)(GLfixed angle, GLfixed x, GLfixed y, GLfixed z);
typedef void (APIENTRYP PFNGLSAMPLECOVERAGEPROC)(GLfloat value, GLboolean invert);
typedef void (APIENTRYP PFNGLSAMPLECOVERAGEXPROC)(GLclampx value, GLboolean invert);
typedef void (APIENTRYP PFNGLSCALEFPROC)(GLfloat x, GLfloat y, GLfloat z);
typedef void (APIENTRYP PFNGLSCALEXPROC)(GLfixed x, GLfixed y, GLfixed z);
typedef void (APIENTRYP PFNGLSCISSORPROC)(GLint x, GLint y, GLsizei width, GLsizei height);
typedef void (APIENTRYP PFNGLSHADEMODELPROC)(GLenum mode);
typedef void (APIENTRYP PFNGLSTENCILFUNCPROC)(GLenum func, GLint ref, GLuint mask);
typedef void (APIENTRYP PFNGLSTENCILMASKPROC)(GLuint mask);
typedef void (APIENTRYP PFNGLSTENCILOPPROC)(GLenum fail, GLenum zfail, GLenum zpass);
typedef void (APIENTRYP PFNGLTEXCOORDPOINTERPROC)(GLint size, GLenum type, GLsizei stride, const void * pointer);
typedef void (APIENTRYP PFNGLTEXENVFPROC)(GLenum target, GLenum pname, GLfloat param);
typedef void (APIENTRYP PFNGLTEXENVFVPROC)(GLenum target, GLenum pname, const GLfloat * params);
typedef void (APIENTRYP PFNGLTEXENVIPROC)(GLenum target, GLenum pname, GLint param);
typedef void (APIENTRYP PFNGLTEXENVIVPROC)(GLenum target, GLenum pname, const GLint * params);
typedef void (APIENTRYP PFNGLTEXENVXPROC)(GLenum target, GLenum pname, GLfixed param);
typedef void (APIENTRYP PFNGLTEXENVXVPROC)(GLenum target, GLenum pname, const GLfixed * params);
typedef void (APIENTRYP PFNGLTEXIMAGE2DPROC)(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLint border, GLenum format, GLenum type, const void * pixels);
typedef void (APIENTRYP PFNGLTEXPARAMETERFPROC)(GLenum target, GLenum pname, GLfloat param);
typedef void (APIENTRYP PFNGLTEXPARAMETERFVPROC)(GLenum target, GLenum pname, const GLfloat * params);
typedef void (APIENTRYP PFNGLTEXPARAMETERIPROC)(GLenum target, GLenum pname, GLint param);
typedef void (APIENTRYP PFNGLTEXPARAMETERIVPROC)(GLenum target, GLenum pname, const GLint * params);
typedef void (APIENTRYP PFNGLTEXPARAMETERXPROC)(GLenum target, GLenum pname, GLfixed param);
typedef void (APIENTRYP PFNGLTEXPARAMETERXVPROC)(GLenum target, GLenum pname, const GLfixed * params);
typedef void (APIENTRYP PFNGLTEXSUBIMAGE2DPROC)(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLenum type, const void * pixels);
typedef void (APIENTRYP PFNGLTRANSLATEFPROC)(GLfloat x, GLfloat y, GLfloat z);
typedef void (APIENTRYP PFNGLTRANSLATEXPROC)(GLfixed x, GLfixed y, GLfixed z);
typedef void (APIENTRYP PFNGLVERTEXPOINTERPROC)(GLint size, GLenum type, GLsizei stride, const void * pointer);
typedef void (APIENTRYP PFNGLVIEWPORTPROC)(GLint x, GLint y, GLsizei width, GLsizei height);
typedef void (APIENTRYP PFNGLBINDFRAMEBUFFEROESPROC)(GLenum target, GLuint framebuffer);
typedef void (APIENTRYP PFNGLBINDRENDERBUFFEROESPROC)(GLenum target, GLuint renderbuffer);
typedef GLenum (APIENTRYP PFNGLCHECKFRAMEBUFFERSTATUSOESPROC)(GLenum target);
typedef void (APIENTRYP PFNGLDELETEFRAMEBUFFERSOESPROC)(GLsizei n, const GLuint * framebuffers);
typedef void (APIENTRYP PFNGLDELETERENDERBUFFERSOESPROC)(GLsizei n, const GLuint * renderbuffers);
typedef void (APIENTRYP PFNGLFRAMEBUFFERRENDERBUFFEROESPROC)(GLenum target, GLenum attachment, GLenum renderbuffertarget, GLuint renderbuffer);
typedef void (APIENTRYP PFNGLFRAMEBUFFERTEXTURE2DOESPROC)(GLenum target, GLenum attachment, GLenum textarget, GLuint texture, GLint level);
typedef void (APIENTRYP PFNGLGENFRAMEBUFFERSOESPROC)(GLsizei n, GLuint * framebuffers);
typedef void (APIENTRYP PFNGLGENRENDERBUFFERSOESPROC)(GLsizei n, GLuint * renderbuffers);
typedef void (APIENTRYP PFNGLGENERATEMIPMAPOESPROC)(GLenum target);
typedef void (APIENTRYP PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVOESPROC)(GLenum target, GLenum attachment, GLenum pname, GLint * params);
typedef void (APIENTRYP PFNGLGETRENDERBUFFERPARAMETERIVOESPROC)(GLenum target, GLenum pname, GLint * params);
typedef GLboolean (APIENTRYP PFNGLISFRAMEBUFFEROESPROC)(GLuint framebuffer);
typedef GLboolean (APIENTRYP PFNGLISRENDERBUFFEROESPROC)(GLuint renderbuffer);
typedef void (APIENTRYP PFNGLRENDERBUFFERSTORAGEOESPROC)(GLenum target, GLenum internalformat, GLsizei width, GLsizei height);


/* ---- Context struct ------------------------------------------------------
 * Three anonymous unions give both indexed (array) and named (struct member)
 * access to the same memory, at zero runtime cost.
 *
 * featArray / extArray: unsigned char flags, one per feature / extension.
 * pfnArray: function pointer slots, one per command.
 *
 * Anonymous structs inside the unions are a C11 / GCC extension; they are
 * universally supported on our target compilers.
 */
typedef struct GloamGLContext {
    union {
        unsigned char featArray[1];
        struct {
        /*    0 */ unsigned char VERSION_ES_CM_1_0;
        };
    };

    union {
        unsigned char extArray[1];
        struct {
        /*    0 */ unsigned char OES_framebuffer_object;
        };
    };

    union {
        void *pfnArray[159];
        struct {
        /*    0 */ PFNGLACTIVETEXTUREPROC ActiveTexture;
        /*    1 */ PFNGLALPHAFUNCPROC AlphaFunc;
        /*    2 */ PFNGLALPHAFUNCXPROC AlphaFuncx;
        /*    3 */ PFNGLBINDBUFFERPROC BindBuffer;
        /*    4 */ PFNGLBINDTEXTUREPROC BindTexture;
        /*    5 */ PFNGLBLENDFUNCPROC BlendFunc;
        /*    6 */ PFNGLBUFFERDATAPROC BufferData;
        /*    7 */ PFNGLBUFFERSUBDATAPROC BufferSubData;
        /*    8 */ PFNGLCLEARPROC Clear;
        /*    9 */ PFNGLCLEARCOLORPROC ClearColor;
        /*   10 */ PFNGLCLEARCOLORXPROC ClearColorx;
        /*   11 */ PFNGLCLEARDEPTHFPROC ClearDepthf;
        /*   12 */ PFNGLCLEARDEPTHXPROC ClearDepthx;
        /*   13 */ PFNGLCLEARSTENCILPROC ClearStencil;
        /*   14 */ PFNGLCLIENTACTIVETEXTUREPROC ClientActiveTexture;
        /*   15 */ PFNGLCLIPPLANEFPROC ClipPlanef;
        /*   16 */ PFNGLCLIPPLANEXPROC ClipPlanex;
        /*   17 */ PFNGLCOLOR4FPROC Color4f;
        /*   18 */ PFNGLCOLOR4UBPROC Color4ub;
        /*   19 */ PFNGLCOLOR4XPROC Color4x;
        /*   20 */ PFNGLCOLORMASKPROC ColorMask;
        /*   21 */ PFNGLCOLORPOINTERPROC ColorPointer;
        /*   22 */ PFNGLCOMPRESSEDTEXIMAGE2DPROC CompressedTexImage2D;
        /*   23 */ PFNGLCOMPRESSEDTEXSUBIMAGE2DPROC CompressedTexSubImage2D;
        /*   24 */ PFNGLCOPYTEXIMAGE2DPROC CopyTexImage2D;
        /*   25 */ PFNGLCOPYTEXSUBIMAGE2DPROC CopyTexSubImage2D;
        /*   26 */ PFNGLCULLFACEPROC CullFace;
        /*   27 */ PFNGLDELETEBUFFERSPROC DeleteBuffers;
        /*   28 */ PFNGLDELETETEXTURESPROC DeleteTextures;
        /*   29 */ PFNGLDEPTHFUNCPROC DepthFunc;
        /*   30 */ PFNGLDEPTHMASKPROC DepthMask;
        /*   31 */ PFNGLDEPTHRANGEFPROC DepthRangef;
        /*   32 */ PFNGLDEPTHRANGEXPROC DepthRangex;
        /*   33 */ PFNGLDISABLEPROC Disable;
        /*   34 */ PFNGLDISABLECLIENTSTATEPROC DisableClientState;
        /*   35 */ PFNGLDRAWARRAYSPROC DrawArrays;
        /*   36 */ PFNGLDRAWELEMENTSPROC DrawElements;
        /*   37 */ PFNGLENABLEPROC Enable;
        /*   38 */ PFNGLENABLECLIENTSTATEPROC EnableClientState;
        /*   39 */ PFNGLFINISHPROC Finish;
        /*   40 */ PFNGLFLUSHPROC Flush;
        /*   41 */ PFNGLFOGFPROC Fogf;
        /*   42 */ PFNGLFOGFVPROC Fogfv;
        /*   43 */ PFNGLFOGXPROC Fogx;
        /*   44 */ PFNGLFOGXVPROC Fogxv;
        /*   45 */ PFNGLFRONTFACEPROC FrontFace;
        /*   46 */ PFNGLFRUSTUMFPROC Frustumf;
        /*   47 */ PFNGLFRUSTUMXPROC Frustumx;
        /*   48 */ PFNGLGENBUFFERSPROC GenBuffers;
        /*   49 */ PFNGLGENTEXTURESPROC GenTextures;
        /*   50 */ PFNGLGETBOOLEANVPROC GetBooleanv;
        /*   51 */ PFNGLGETBUFFERPARAMETERIVPROC GetBufferParameteriv;
        /*   52 */ PFNGLGETCLIPPLANEFPROC GetClipPlanef;
        /*   53 */ PFNGLGETCLIPPLANEXPROC GetClipPlanex;
        /*   54 */ PFNGLGETERRORPROC GetError;
        /*   55 */ PFNGLGETFIXEDVPROC GetFixedv;
        /*   56 */ PFNGLGETFLOATVPROC GetFloatv;
        /*   57 */ PFNGLGETINTEGERVPROC GetIntegerv;
        /*   58 */ PFNGLGETLIGHTFVPROC GetLightfv;
        /*   59 */ PFNGLGETLIGHTXVPROC GetLightxv;
        /*   60 */ PFNGLGETMATERIALFVPROC GetMaterialfv;
        /*   61 */ PFNGLGETMATERIALXVPROC GetMaterialxv;
        /*   62 */ PFNGLGETPOINTERVPROC GetPointerv;
        /*   63 */ PFNGLGETSTRINGPROC GetString;
        /*   64 */ PFNGLGETTEXENVFVPROC GetTexEnvfv;
        /*   65 */ PFNGLGETTEXENVIVPROC GetTexEnviv;
        /*   66 */ PFNGLGETTEXENVXVPROC GetTexEnvxv;
        /*   67 */ PFNGLGETTEXPARAMETERFVPROC GetTexParameterfv;
        /*   68 */ PFNGLGETTEXPARAMETERIVPROC GetTexParameteriv;
        /*   69 */ PFNGLGETTEXPARAMETERXVPROC GetTexParameterxv;
        /*   70 */ PFNGLHINTPROC Hint;
        /*   71 */ PFNGLISBUFFERPROC IsBuffer;
        /*   72 */ PFNGLISENABLEDPROC IsEnabled;
        /*   73 */ PFNGLISTEXTUREPROC IsTexture;
        /*   74 */ PFNGLLIGHTMODELFPROC LightModelf;
        /*   75 */ PFNGLLIGHTMODELFVPROC LightModelfv;
        /*   76 */ PFNGLLIGHTMODELXPROC LightModelx;
        /*   77 */ PFNGLLIGHTMODELXVPROC LightModelxv;
        /*   78 */ PFNGLLIGHTFPROC Lightf;
        /*   79 */ PFNGLLIGHTFVPROC Lightfv;
        /*   80 */ PFNGLLIGHTXPROC Lightx;
        /*   81 */ PFNGLLIGHTXVPROC Lightxv;
        /*   82 */ PFNGLLINEWIDTHPROC LineWidth;
        /*   83 */ PFNGLLINEWIDTHXPROC LineWidthx;
        /*   84 */ PFNGLLOADIDENTITYPROC LoadIdentity;
        /*   85 */ PFNGLLOADMATRIXFPROC LoadMatrixf;
        /*   86 */ PFNGLLOADMATRIXXPROC LoadMatrixx;
        /*   87 */ PFNGLLOGICOPPROC LogicOp;
        /*   88 */ PFNGLMATERIALFPROC Materialf;
        /*   89 */ PFNGLMATERIALFVPROC Materialfv;
        /*   90 */ PFNGLMATERIALXPROC Materialx;
        /*   91 */ PFNGLMATERIALXVPROC Materialxv;
        /*   92 */ PFNGLMATRIXMODEPROC MatrixMode;
        /*   93 */ PFNGLMULTMATRIXFPROC MultMatrixf;
        /*   94 */ PFNGLMULTMATRIXXPROC MultMatrixx;
        /*   95 */ PFNGLMULTITEXCOORD4FPROC MultiTexCoord4f;
        /*   96 */ PFNGLMULTITEXCOORD4XPROC MultiTexCoord4x;
        /*   97 */ PFNGLNORMAL3FPROC Normal3f;
        /*   98 */ PFNGLNORMAL3XPROC Normal3x;
        /*   99 */ PFNGLNORMALPOINTERPROC NormalPointer;
        /*  100 */ PFNGLORTHOFPROC Orthof;
        /*  101 */ PFNGLORTHOXPROC Orthox;
        /*  102 */ PFNGLPIXELSTOREIPROC PixelStorei;
        /*  103 */ PFNGLPOINTPARAMETERFPROC PointParameterf;
        /*  104 */ PFNGLPOINTPARAMETERFVPROC PointParameterfv;
        /*  105 */ PFNGLPOINTPARAMETERXPROC PointParameterx;
        /*  106 */ PFNGLPOINTPARAMETERXVPROC PointParameterxv;
        /*  107 */ PFNGLPOINTSIZEPROC PointSize;
        /*  108 */ PFNGLPOINTSIZEXPROC PointSizex;
        /*  109 */ PFNGLPOLYGONOFFSETPROC PolygonOffset;
        /*  110 */ PFNGLPOLYGONOFFSETXPROC PolygonOffsetx;
        /*  111 */ PFNGLPOPMATRIXPROC PopMatrix;
        /*  112 */ PFNGLPUSHMATRIXPROC PushMatrix;
        /*  113 */ PFNGLREADPIXELSPROC ReadPixels;
        /*  114 */ PFNGLROTATEFPROC Rotatef;
        /*  115 */ PFNGLROTATEXPROC Rotatex;
        /*  116 */ PFNGLSAMPLECOVERAGEPROC SampleCoverage;
        /*  117 */ PFNGLSAMPLECOVERAGEXPROC SampleCoveragex;
        /*  118 */ PFNGLSCALEFPROC Scalef;
        /*  119 */ PFNGLSCALEXPROC Scalex;
        /*  120 */ PFNGLSCISSORPROC Scissor;
        /*  121 */ PFNGLSHADEMODELPROC ShadeModel;
        /*  122 */ PFNGLSTENCILFUNCPROC StencilFunc;
        /*  123 */ PFNGLSTENCILMASKPROC StencilMask;
        /*  124 */ PFNGLSTENCILOPPROC StencilOp;
        /*  125 */ PFNGLTEXCOORDPOINTERPROC TexCoordPointer;
        /*  126 */ PFNGLTEXENVFPROC TexEnvf;
        /*  127 */ PFNGLTEXENVFVPROC TexEnvfv;
        /*  128 */ PFNGLTEXENVIPROC TexEnvi;
        /*  129 */ PFNGLTEXENVIVPROC TexEnviv;
        /*  130 */ PFNGLTEXENVXPROC TexEnvx;
        /*  131 */ PFNGLTEXENVXVPROC TexEnvxv;
        /*  132 */ PFNGLTEXIMAGE2DPROC TexImage2D;
        /*  133 */ PFNGLTEXPARAMETERFPROC TexParameterf;
        /*  134 */ PFNGLTEXPARAMETERFVPROC TexParameterfv;
        /*  135 */ PFNGLTEXPARAMETERIPROC TexParameteri;
        /*  136 */ PFNGLTEXPARAMETERIVPROC TexParameteriv;
        /*  137 */ PFNGLTEXPARAMETERXPROC TexParameterx;
        /*  138 */ PFNGLTEXPARAMETERXVPROC TexParameterxv;
        /*  139 */ PFNGLTEXSUBIMAGE2DPROC TexSubImage2D;
        /*  140 */ PFNGLTRANSLATEFPROC Translatef;
        /*  141 */ PFNGLTRANSLATEXPROC Translatex;
        /*  142 */ PFNGLVERTEXPOINTERPROC VertexPointer;
        /*  143 */ PFNGLVIEWPORTPROC Viewport;
        /*  144 */ PFNGLBINDFRAMEBUFFEROESPROC BindFramebufferOES;
        /*  145 */ PFNGLBINDRENDERBUFFEROESPROC BindRenderbufferOES;
        /*  146 */ PFNGLCHECKFRAMEBUFFERSTATUSOESPROC CheckFramebufferStatusOES;
        /*  147 */ PFNGLDELETEFRAMEBUFFERSOESPROC DeleteFramebuffersOES;
        /*  148 */ PFNGLDELETERENDERBUFFERSOESPROC DeleteRenderbuffersOES;
        /*  149 */ PFNGLFRAMEBUFFERRENDERBUFFEROESPROC FramebufferRenderbufferOES;
        /*  150 */ PFNGLFRAMEBUFFERTEXTURE2DOESPROC FramebufferTexture2DOES;
        /*  151 */ PFNGLGENFRAMEBUFFERSOESPROC GenFramebuffersOES;
        /*  152 */ PFNGLGENRENDERBUFFERSOESPROC GenRenderbuffersOES;
        /*  153 */ PFNGLGENERATEMIPMAPOESPROC GenerateMipmapOES;
        /*  154 */ PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVOESPROC GetFramebufferAttachmentParameterivOES;
        /*  155 */ PFNGLGETRENDERBUFFERPARAMETERIVOESPROC GetRenderbufferParameterivOES;
        /*  156 */ PFNGLISFRAMEBUFFEROESPROC IsFramebufferOES;
        /*  157 */ PFNGLISRENDERBUFFEROESPROC IsRenderbufferOES;
        /*  158 */ PFNGLRENDERBUFFERSTORAGEOESPROC RenderbufferStorageOES;
        };
    };

    /* Built-in loader library handle. Set by gloamVulkanInitialize*Context /
     * gloamLoaderLoad*Context when it opens the platform library. If the
     * caller pre-populates this field, the open is skipped and ownership is
     * NOT taken (gloam will not close it). Present on all context types so
     * user code can use a single field name regardless of API.
     */
    void *gloam_loader_handle;

    /* Non-zero if gloam opened the library handle itself and is responsible
     * for closing it in gloamVulkanFinalize / gloamLoaderUnload.
     */
    uint8_t gloam_loader_owns_handle;
} GloamGLContext;

/* Global context instance — a value, not a pointer, so the compiler knows
 * its address is fixed and does not re-load it on every access.
 */
extern GloamGLContext gloam_gl_context;

/* ---- Feature presence macros --------------------------------------------
 * Test whether a versioned feature was detected at load time.
 */
#define GLOAM_GL_VERSION_ES_CM_1_0 (gloam_gl_context.VERSION_ES_CM_1_0)

/* ---- Extension presence macros ------------------------------------------ */
#define GLOAM_GL_OES_framebuffer_object (gloam_gl_context.OES_framebuffer_object)


/* ---- Dispatch ------------------------------------------------------------ */

#ifdef __INTELLISENSE__
void glActiveTexture(GLenum texture);
void glAlphaFunc(GLenum func, GLfloat ref);
void glAlphaFuncx(GLenum func, GLfixed ref);
void glBindBuffer(GLenum target, GLuint buffer);
void glBindTexture(GLenum target, GLuint texture);
void glBlendFunc(GLenum sfactor, GLenum dfactor);
void glBufferData(GLenum target, GLsizeiptr size, const void * data, GLenum usage);
void glBufferSubData(GLenum target, GLintptr offset, GLsizeiptr size, const void * data);
void glClear(GLbitfield mask);
void glClearColor(GLfloat red, GLfloat green, GLfloat blue, GLfloat alpha);
void glClearColorx(GLfixed red, GLfixed green, GLfixed blue, GLfixed alpha);
void glClearDepthf(GLfloat d);
void glClearDepthx(GLfixed depth);
void glClearStencil(GLint s);
void glClientActiveTexture(GLenum texture);
void glClipPlanef(GLenum p, const GLfloat * eqn);
void glClipPlanex(GLenum plane, const GLfixed * equation);
void glColor4f(GLfloat red, GLfloat green, GLfloat blue, GLfloat alpha);
void glColor4ub(GLubyte red, GLubyte green, GLubyte blue, GLubyte alpha);
void glColor4x(GLfixed red, GLfixed green, GLfixed blue, GLfixed alpha);
void glColorMask(GLboolean red, GLboolean green, GLboolean blue, GLboolean alpha);
void glColorPointer(GLint size, GLenum type, GLsizei stride, const void * pointer);
void glCompressedTexImage2D(GLenum target, GLint level, GLenum internalformat, GLsizei width, GLsizei height, GLint border, GLsizei imageSize, const void * data);
void glCompressedTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLsizei imageSize, const void * data);
void glCopyTexImage2D(GLenum target, GLint level, GLenum internalformat, GLint x, GLint y, GLsizei width, GLsizei height, GLint border);
void glCopyTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint x, GLint y, GLsizei width, GLsizei height);
void glCullFace(GLenum mode);
void glDeleteBuffers(GLsizei n, const GLuint * buffers);
void glDeleteTextures(GLsizei n, const GLuint * textures);
void glDepthFunc(GLenum func);
void glDepthMask(GLboolean flag);
void glDepthRangef(GLfloat n, GLfloat f);
void glDepthRangex(GLfixed n, GLfixed f);
void glDisable(GLenum cap);
void glDisableClientState(GLenum array);
void glDrawArrays(GLenum mode, GLint first, GLsizei count);
void glDrawElements(GLenum mode, GLsizei count, GLenum type, const void * indices);
void glEnable(GLenum cap);
void glEnableClientState(GLenum array);
void glFinish(void);
void glFlush(void);
void glFogf(GLenum pname, GLfloat param);
void glFogfv(GLenum pname, const GLfloat * params);
void glFogx(GLenum pname, GLfixed param);
void glFogxv(GLenum pname, const GLfixed * param);
void glFrontFace(GLenum mode);
void glFrustumf(GLfloat l, GLfloat r, GLfloat b, GLfloat t, GLfloat n, GLfloat f);
void glFrustumx(GLfixed l, GLfixed r, GLfixed b, GLfixed t, GLfixed n, GLfixed f);
void glGenBuffers(GLsizei n, GLuint * buffers);
void glGenTextures(GLsizei n, GLuint * textures);
void glGetBooleanv(GLenum pname, GLboolean * data);
void glGetBufferParameteriv(GLenum target, GLenum pname, GLint * params);
void glGetClipPlanef(GLenum plane, GLfloat * equation);
void glGetClipPlanex(GLenum plane, GLfixed * equation);
GLenum glGetError(void);
void glGetFixedv(GLenum pname, GLfixed * data);
void glGetFloatv(GLenum pname, GLfloat * data);
void glGetIntegerv(GLenum pname, GLint * data);
void glGetLightfv(GLenum light, GLenum pname, GLfloat * params);
void glGetLightxv(GLenum light, GLenum pname, GLfixed * params);
void glGetMaterialfv(GLenum face, GLenum pname, GLfloat * params);
void glGetMaterialxv(GLenum face, GLenum pname, GLfixed * params);
void glGetPointerv(GLenum pname, void ** params);
const GLubyte * glGetString(GLenum name);
void glGetTexEnvfv(GLenum target, GLenum pname, GLfloat * params);
void glGetTexEnviv(GLenum target, GLenum pname, GLint * params);
void glGetTexEnvxv(GLenum target, GLenum pname, GLfixed * params);
void glGetTexParameterfv(GLenum target, GLenum pname, GLfloat * params);
void glGetTexParameteriv(GLenum target, GLenum pname, GLint * params);
void glGetTexParameterxv(GLenum target, GLenum pname, GLfixed * params);
void glHint(GLenum target, GLenum mode);
GLboolean glIsBuffer(GLuint buffer);
GLboolean glIsEnabled(GLenum cap);
GLboolean glIsTexture(GLuint texture);
void glLightModelf(GLenum pname, GLfloat param);
void glLightModelfv(GLenum pname, const GLfloat * params);
void glLightModelx(GLenum pname, GLfixed param);
void glLightModelxv(GLenum pname, const GLfixed * param);
void glLightf(GLenum light, GLenum pname, GLfloat param);
void glLightfv(GLenum light, GLenum pname, const GLfloat * params);
void glLightx(GLenum light, GLenum pname, GLfixed param);
void glLightxv(GLenum light, GLenum pname, const GLfixed * params);
void glLineWidth(GLfloat width);
void glLineWidthx(GLfixed width);
void glLoadIdentity(void);
void glLoadMatrixf(const GLfloat * m);
void glLoadMatrixx(const GLfixed * m);
void glLogicOp(GLenum opcode);
void glMaterialf(GLenum face, GLenum pname, GLfloat param);
void glMaterialfv(GLenum face, GLenum pname, const GLfloat * params);
void glMaterialx(GLenum face, GLenum pname, GLfixed param);
void glMaterialxv(GLenum face, GLenum pname, const GLfixed * param);
void glMatrixMode(GLenum mode);
void glMultMatrixf(const GLfloat * m);
void glMultMatrixx(const GLfixed * m);
void glMultiTexCoord4f(GLenum target, GLfloat s, GLfloat t, GLfloat r, GLfloat q);
void glMultiTexCoord4x(GLenum texture, GLfixed s, GLfixed t, GLfixed r, GLfixed q);
void glNormal3f(GLfloat nx, GLfloat ny, GLfloat nz);
void glNormal3x(GLfixed nx, GLfixed ny, GLfixed nz);
void glNormalPointer(GLenum type, GLsizei stride, const void * pointer);
void glOrthof(GLfloat l, GLfloat r, GLfloat b, GLfloat t, GLfloat n, GLfloat f);
void glOrthox(GLfixed l, GLfixed r, GLfixed b, GLfixed t, GLfixed n, GLfixed f);
void glPixelStorei(GLenum pname, GLint param);
void glPointParameterf(GLenum pname, GLfloat param);
void glPointParameterfv(GLenum pname, const GLfloat * params);
void glPointParameterx(GLenum pname, GLfixed param);
void glPointParameterxv(GLenum pname, const GLfixed * params);
void glPointSize(GLfloat size);
void glPointSizex(GLfixed size);
void glPolygonOffset(GLfloat factor, GLfloat units);
void glPolygonOffsetx(GLfixed factor, GLfixed units);
void glPopMatrix(void);
void glPushMatrix(void);
void glReadPixels(GLint x, GLint y, GLsizei width, GLsizei height, GLenum format, GLenum type, void * pixels);
void glRotatef(GLfloat angle, GLfloat x, GLfloat y, GLfloat z);
void glRotatex(GLfixed angle, GLfixed x, GLfixed y, GLfixed z);
void glSampleCoverage(GLfloat value, GLboolean invert);
void glSampleCoveragex(GLclampx value, GLboolean invert);
void glScalef(GLfloat x, GLfloat y, GLfloat z);
void glScalex(GLfixed x, GLfixed y, GLfixed z);
void glScissor(GLint x, GLint y, GLsizei width, GLsizei height);
void glShadeModel(GLenum mode);
void glStencilFunc(GLenum func, GLint ref, GLuint mask);
void glStencilMask(GLuint mask);
void glStencilOp(GLenum fail, GLenum zfail, GLenum zpass);
void glTexCoordPointer(GLint size, GLenum type, GLsizei stride, const void * pointer);
void glTexEnvf(GLenum target, GLenum pname, GLfloat param);
void glTexEnvfv(GLenum target, GLenum pname, const GLfloat * params);
void glTexEnvi(GLenum target, GLenum pname, GLint param);
void glTexEnviv(GLenum target, GLenum pname, const GLint * params);
void glTexEnvx(GLenum target, GLenum pname, GLfixed param);
void glTexEnvxv(GLenum target, GLenum pname, const GLfixed * params);
void glTexImage2D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLint border, GLenum format, GLenum type, const void * pixels);
void glTexParameterf(GLenum target, GLenum pname, GLfloat param);
void glTexParameterfv(GLenum target, GLenum pname, const GLfloat * params);
void glTexParameteri(GLenum target, GLenum pname, GLint param);
void glTexParameteriv(GLenum target, GLenum pname, const GLint * params);
void glTexParameterx(GLenum target, GLenum pname, GLfixed param);
void glTexParameterxv(GLenum target, GLenum pname, const GLfixed * params);
void glTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLenum type, const void * pixels);
void glTranslatef(GLfloat x, GLfloat y, GLfloat z);
void glTranslatex(GLfixed x, GLfixed y, GLfixed z);
void glVertexPointer(GLint size, GLenum type, GLsizei stride, const void * pointer);
void glViewport(GLint x, GLint y, GLsizei width, GLsizei height);
void glBindFramebufferOES(GLenum target, GLuint framebuffer);
void glBindRenderbufferOES(GLenum target, GLuint renderbuffer);
GLenum glCheckFramebufferStatusOES(GLenum target);
void glDeleteFramebuffersOES(GLsizei n, const GLuint * framebuffers);
void glDeleteRenderbuffersOES(GLsizei n, const GLuint * renderbuffers);
void glFramebufferRenderbufferOES(GLenum target, GLenum attachment, GLenum renderbuffertarget, GLuint renderbuffer);
void glFramebufferTexture2DOES(GLenum target, GLenum attachment, GLenum textarget, GLuint texture, GLint level);
void glGenFramebuffersOES(GLsizei n, GLuint * framebuffers);
void glGenRenderbuffersOES(GLsizei n, GLuint * renderbuffers);
void glGenerateMipmapOES(GLenum target);
void glGetFramebufferAttachmentParameterivOES(GLenum target, GLenum attachment, GLenum pname, GLint * params);
void glGetRenderbufferParameterivOES(GLenum target, GLenum pname, GLint * params);
GLboolean glIsFramebufferOES(GLuint framebuffer);
GLboolean glIsRenderbufferOES(GLuint renderbuffer);
void glRenderbufferStorageOES(GLenum target, GLenum internalformat, GLsizei width, GLsizei height);
#else
#define glActiveTexture (gloam_gl_context.ActiveTexture)
#define glAlphaFunc (gloam_gl_context.AlphaFunc)
#define glAlphaFuncx (gloam_gl_context.AlphaFuncx)
#define glBindBuffer (gloam_gl_context.BindBuffer)
#define glBindTexture (gloam_gl_context.BindTexture)
#define glBlendFunc (gloam_gl_context.BlendFunc)
#define glBufferData (gloam_gl_context.BufferData)
#define glBufferSubData (gloam_gl_context.BufferSubData)
#define glClear (gloam_gl_context.Clear)
#define glClearColor (gloam_gl_context.ClearColor)
#define glClearColorx (gloam_gl_context.ClearColorx)
#define glClearDepthf (gloam_gl_context.ClearDepthf)
#define glClearDepthx (gloam_gl_context.ClearDepthx)
#define glClearStencil (gloam_gl_context.ClearStencil)
#define glClientActiveTexture (gloam_gl_context.ClientActiveTexture)
#define glClipPlanef (gloam_gl_context.ClipPlanef)
#define glClipPlanex (gloam_gl_context.ClipPlanex)
#define glColor4f (gloam_gl_context.Color4f)
#define glColor4ub (gloam_gl_context.Color4ub)
#define glColor4x (gloam_gl_context.Color4x)
#define glColorMask (gloam_gl_context.ColorMask)
#define glColorPointer (gloam_gl_context.ColorPointer)
#define glCompressedTexImage2D (gloam_gl_context.CompressedTexImage2D)
#define glCompressedTexSubImage2D (gloam_gl_context.CompressedTexSubImage2D)
#define glCopyTexImage2D (gloam_gl_context.CopyTexImage2D)
#define glCopyTexSubImage2D (gloam_gl_context.CopyTexSubImage2D)
#define glCullFace (gloam_gl_context.CullFace)
#define glDeleteBuffers (gloam_gl_context.DeleteBuffers)
#define glDeleteTextures (gloam_gl_context.DeleteTextures)
#define glDepthFunc (gloam_gl_context.DepthFunc)
#define glDepthMask (gloam_gl_context.DepthMask)
#define glDepthRangef (gloam_gl_context.DepthRangef)
#define glDepthRangex (gloam_gl_context.DepthRangex)
#define glDisable (gloam_gl_context.Disable)
#define glDisableClientState (gloam_gl_context.DisableClientState)
#define glDrawArrays (gloam_gl_context.DrawArrays)
#define glDrawElements (gloam_gl_context.DrawElements)
#define glEnable (gloam_gl_context.Enable)
#define glEnableClientState (gloam_gl_context.EnableClientState)
#define glFinish (gloam_gl_context.Finish)
#define glFlush (gloam_gl_context.Flush)
#define glFogf (gloam_gl_context.Fogf)
#define glFogfv (gloam_gl_context.Fogfv)
#define glFogx (gloam_gl_context.Fogx)
#define glFogxv (gloam_gl_context.Fogxv)
#define glFrontFace (gloam_gl_context.FrontFace)
#define glFrustumf (gloam_gl_context.Frustumf)
#define glFrustumx (gloam_gl_context.Frustumx)
#define glGenBuffers (gloam_gl_context.GenBuffers)
#define glGenTextures (gloam_gl_context.GenTextures)
#define glGetBooleanv (gloam_gl_context.GetBooleanv)
#define glGetBufferParameteriv (gloam_gl_context.GetBufferParameteriv)
#define glGetClipPlanef (gloam_gl_context.GetClipPlanef)
#define glGetClipPlanex (gloam_gl_context.GetClipPlanex)
#define glGetError (gloam_gl_context.GetError)
#define glGetFixedv (gloam_gl_context.GetFixedv)
#define glGetFloatv (gloam_gl_context.GetFloatv)
#define glGetIntegerv (gloam_gl_context.GetIntegerv)
#define glGetLightfv (gloam_gl_context.GetLightfv)
#define glGetLightxv (gloam_gl_context.GetLightxv)
#define glGetMaterialfv (gloam_gl_context.GetMaterialfv)
#define glGetMaterialxv (gloam_gl_context.GetMaterialxv)
#define glGetPointerv (gloam_gl_context.GetPointerv)
#define glGetString (gloam_gl_context.GetString)
#define glGetTexEnvfv (gloam_gl_context.GetTexEnvfv)
#define glGetTexEnviv (gloam_gl_context.GetTexEnviv)
#define glGetTexEnvxv (gloam_gl_context.GetTexEnvxv)
#define glGetTexParameterfv (gloam_gl_context.GetTexParameterfv)
#define glGetTexParameteriv (gloam_gl_context.GetTexParameteriv)
#define glGetTexParameterxv (gloam_gl_context.GetTexParameterxv)
#define glHint (gloam_gl_context.Hint)
#define glIsBuffer (gloam_gl_context.IsBuffer)
#define glIsEnabled (gloam_gl_context.IsEnabled)
#define glIsTexture (gloam_gl_context.IsTexture)
#define glLightModelf (gloam_gl_context.LightModelf)
#define glLightModelfv (gloam_gl_context.LightModelfv)
#define glLightModelx (gloam_gl_context.LightModelx)
#define glLightModelxv (gloam_gl_context.LightModelxv)
#define glLightf (gloam_gl_context.Lightf)
#define glLightfv (gloam_gl_context.Lightfv)
#define glLightx (gloam_gl_context.Lightx)
#define glLightxv (gloam_gl_context.Lightxv)
#define glLineWidth (gloam_gl_context.LineWidth)
#define glLineWidthx (gloam_gl_context.LineWidthx)
#define glLoadIdentity (gloam_gl_context.LoadIdentity)
#define glLoadMatrixf (gloam_gl_context.LoadMatrixf)
#define glLoadMatrixx (gloam_gl_context.LoadMatrixx)
#define glLogicOp (gloam_gl_context.LogicOp)
#define glMaterialf (gloam_gl_context.Materialf)
#define glMaterialfv (gloam_gl_context.Materialfv)
#define glMaterialx (gloam_gl_context.Materialx)
#define glMaterialxv (gloam_gl_context.Materialxv)
#define glMatrixMode (gloam_gl_context.MatrixMode)
#define glMultMatrixf (gloam_gl_context.MultMatrixf)
#define glMultMatrixx (gloam_gl_context.MultMatrixx)
#define glMultiTexCoord4f (gloam_gl_context.MultiTexCoord4f)
#define glMultiTexCoord4x (gloam_gl_context.MultiTexCoord4x)
#define glNormal3f (gloam_gl_context.Normal3f)
#define glNormal3x (gloam_gl_context.Normal3x)
#define glNormalPointer (gloam_gl_context.NormalPointer)
#define glOrthof (gloam_gl_context.Orthof)
#define glOrthox (gloam_gl_context.Orthox)
#define glPixelStorei (gloam_gl_context.PixelStorei)
#define glPointParameterf (gloam_gl_context.PointParameterf)
#define glPointParameterfv (gloam_gl_context.PointParameterfv)
#define glPointParameterx (gloam_gl_context.PointParameterx)
#define glPointParameterxv (gloam_gl_context.PointParameterxv)
#define glPointSize (gloam_gl_context.PointSize)
#define glPointSizex (gloam_gl_context.PointSizex)
#define glPolygonOffset (gloam_gl_context.PolygonOffset)
#define glPolygonOffsetx (gloam_gl_context.PolygonOffsetx)
#define glPopMatrix (gloam_gl_context.PopMatrix)
#define glPushMatrix (gloam_gl_context.PushMatrix)
#define glReadPixels (gloam_gl_context.ReadPixels)
#define glRotatef (gloam_gl_context.Rotatef)
#define glRotatex (gloam_gl_context.Rotatex)
#define glSampleCoverage (gloam_gl_context.SampleCoverage)
#define glSampleCoveragex (gloam_gl_context.SampleCoveragex)
#define glScalef (gloam_gl_context.Scalef)
#define glScalex (gloam_gl_context.Scalex)
#define glScissor (gloam_gl_context.Scissor)
#define glShadeModel (gloam_gl_context.ShadeModel)
#define glStencilFunc (gloam_gl_context.StencilFunc)
#define glStencilMask (gloam_gl_context.StencilMask)
#define glStencilOp (gloam_gl_context.StencilOp)
#define glTexCoordPointer (gloam_gl_context.TexCoordPointer)
#define glTexEnvf (gloam_gl_context.TexEnvf)
#define glTexEnvfv (gloam_gl_context.TexEnvfv)
#define glTexEnvi (gloam_gl_context.TexEnvi)
#define glTexEnviv (gloam_gl_context.TexEnviv)
#define glTexEnvx (gloam_gl_context.TexEnvx)
#define glTexEnvxv (gloam_gl_context.TexEnvxv)
#define glTexImage2D (gloam_gl_context.TexImage2D)
#define glTexParameterf (gloam_gl_context.TexParameterf)
#define glTexParameterfv (gloam_gl_context.TexParameterfv)
#define glTexParameteri (gloam_gl_context.TexParameteri)
#define glTexParameteriv (gloam_gl_context.TexParameteriv)
#define glTexParameterx (gloam_gl_context.TexParameterx)
#define glTexParameterxv (gloam_gl_context.TexParameterxv)
#define glTexSubImage2D (gloam_gl_context.TexSubImage2D)
#define glTranslatef (gloam_gl_context.Translatef)
#define glTranslatex (gloam_gl_context.Translatex)
#define glVertexPointer (gloam_gl_context.VertexPointer)
#define glViewport (gloam_gl_context.Viewport)
#define glBindFramebufferOES (gloam_gl_context.BindFramebufferOES)
#define glBindRenderbufferOES (gloam_gl_context.BindRenderbufferOES)
#define glCheckFramebufferStatusOES (gloam_gl_context.CheckFramebufferStatusOES)
#define glDeleteFramebuffersOES (gloam_gl_context.DeleteFramebuffersOES)
#define glDeleteRenderbuffersOES (gloam_gl_context.DeleteRenderbuffersOES)
#define glFramebufferRenderbufferOES (gloam_gl_context.FramebufferRenderbufferOES)
#define glFramebufferTexture2DOES (gloam_gl_context.FramebufferTexture2DOES)
#define glGenFramebuffersOES (gloam_gl_context.GenFramebuffersOES)
#define glGenRenderbuffersOES (gloam_gl_context.GenRenderbuffersOES)
#define glGenerateMipmapOES (gloam_gl_context.GenerateMipmapOES)
#define glGetFramebufferAttachmentParameterivOES (gloam_gl_context.GetFramebufferAttachmentParameterivOES)
#define glGetRenderbufferParameterivOES (gloam_gl_context.GetRenderbufferParameterivOES)
#define glIsFramebufferOES (gloam_gl_context.IsFramebufferOES)
#define glIsRenderbufferOES (gloam_gl_context.IsRenderbufferOES)
#define glRenderbufferStorageOES (gloam_gl_context.RenderbufferStorageOES)
#endif /* __INTELLISENSE__ */
/* ---- API declarations ---------------------------------------------------- */

#ifndef GLOAM_DEFINED_CALLBACK_TYPES_
#define GLOAM_DEFINED_CALLBACK_TYPES_
/* Opaque function pointer type — the common return type for all load
 * callbacks. Callers cast to the specific PFN type they need.
 */
typedef void (*GloamAPIProc)(void);

/* Load function pointer type (GL / EGL / GLX / WGL). */
typedef GloamAPIProc (*GloamLoadFunc)(const char *name);
#endif

int gloamLoadGLES1Context(GloamGLContext *context, GloamLoadFunc getProcAddr);
int gloamLoadGLES1(GloamLoadFunc getProcAddr);
/* Built-in loader: opens the platform library if needed and calls the
 * appropriate load function for you. Non-Vulkan loaders call the detection-
 * based gloamLoad* functions. Vulkan loaders handle all extension detection
 * and PFN loading internally.
 * Each Load function may be called multiple times (additive).
 */
int  gloamLoaderLoadGLES1Context(GloamGLContext *context);
int  gloamLoaderLoadGLES1(void);
void gloamLoaderUnloadGLES1Context(GloamGLContext *context);
void gloamLoaderUnloadGLES1(void);
void gloamLoaderResetGLES1Context(GloamGLContext *context);
void gloamLoaderResetGLES1(void);

#ifdef __cplusplus
}
#endif

#endif /* GLOAM_GLES1_H */
