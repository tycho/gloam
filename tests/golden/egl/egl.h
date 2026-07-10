#ifndef GLOAM_EGL_H
#define GLOAM_EGL_H

#ifdef __egl_h_
  #error EGL (egl.h) header already included (API: egl), remove previous include!
#endif
#define __egl_h_ 1

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
#include "EGL/eglplatform.h"

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

/* ---- Version feature guards ----------------------------------------------
 * These mirror the upstream vulkan_core.h / gl.h definitions so that code
 * guarded by e.g. #ifdef GL_VERSION_3_3 compiles correctly against this
 * header.
 */
#define EGL_VERSION_1_0 1
#define EGL_VERSION_1_1 1
#define EGL_VERSION_1_2 1
#define EGL_VERSION_1_3 1
#define EGL_VERSION_1_4 1
#define EGL_VERSION_1_5 1

/* ---- Extension compile-time guards ---------------------------------------
 * These mirror the definitions in standard glext.h/gl2ext.h/eglext.h so
 * that code guarded by e.g. #ifdef GL_ARB_draw_indirect compiles correctly
 * against this header.
 */
#define EGL_KHR_debug 1

/* ---- Constants ----------------------------------------------------------- */
#define EGL_PBUFFER_BIT 0x0001
#define EGL_PIXMAP_BIT 0x0002
#define EGL_WINDOW_BIT 0x0004
#define EGL_VG_COLORSPACE_LINEAR_BIT 0x0020
#define EGL_VG_ALPHA_FORMAT_PRE_BIT 0x0040
#define EGL_MULTISAMPLE_RESOLVE_BOX_BIT 0x0200
#define EGL_SWAP_BEHAVIOR_PRESERVED_BIT 0x0400
#define EGL_OPENGL_ES_BIT 0x0001
#define EGL_OPENVG_BIT 0x0002
#define EGL_OPENGL_ES2_BIT 0x0004
#define EGL_OPENGL_BIT 0x0008
#define EGL_OPENGL_ES3_BIT 0x00000040
#define EGL_SYNC_FLUSH_COMMANDS_BIT 0x0001
#define EGL_CONTEXT_OPENGL_CORE_PROFILE_BIT 0x00000001
#define EGL_CONTEXT_OPENGL_COMPATIBILITY_PROFILE_BIT 0x00000002
#define EGL_FALSE 0
#define EGL_TRUE 1
#define EGL_DONT_CARE EGL_CAST(EGLint,-1)
#define EGL_UNKNOWN EGL_CAST(EGLint,-1)
#define EGL_NO_CONTEXT EGL_CAST(EGLContext,0)
#define EGL_NO_DISPLAY EGL_CAST(EGLDisplay,0)
#define EGL_NO_IMAGE EGL_CAST(EGLImage,0)
#define EGL_DEFAULT_DISPLAY EGL_CAST(EGLNativeDisplayType,0)
#define EGL_NO_SURFACE EGL_CAST(EGLSurface,0)
#define EGL_NO_SYNC EGL_CAST(EGLSync,0)
#define EGL_DISPLAY_SCALING 10000
#define EGL_FOREVER 0xFFFFFFFFFFFFFFFF
#define EGL_SUCCESS 0x3000
#define EGL_NOT_INITIALIZED 0x3001
#define EGL_BAD_ACCESS 0x3002
#define EGL_BAD_ALLOC 0x3003
#define EGL_BAD_ATTRIBUTE 0x3004
#define EGL_BAD_CONFIG 0x3005
#define EGL_BAD_CONTEXT 0x3006
#define EGL_BAD_CURRENT_SURFACE 0x3007
#define EGL_BAD_DISPLAY 0x3008
#define EGL_BAD_MATCH 0x3009
#define EGL_BAD_NATIVE_PIXMAP 0x300A
#define EGL_BAD_NATIVE_WINDOW 0x300B
#define EGL_BAD_PARAMETER 0x300C
#define EGL_BAD_SURFACE 0x300D
#define EGL_CONTEXT_LOST 0x300E
#define EGL_BUFFER_SIZE 0x3020
#define EGL_ALPHA_SIZE 0x3021
#define EGL_BLUE_SIZE 0x3022
#define EGL_GREEN_SIZE 0x3023
#define EGL_RED_SIZE 0x3024
#define EGL_DEPTH_SIZE 0x3025
#define EGL_STENCIL_SIZE 0x3026
#define EGL_CONFIG_CAVEAT 0x3027
#define EGL_CONFIG_ID 0x3028
#define EGL_LEVEL 0x3029
#define EGL_MAX_PBUFFER_HEIGHT 0x302A
#define EGL_MAX_PBUFFER_PIXELS 0x302B
#define EGL_MAX_PBUFFER_WIDTH 0x302C
#define EGL_NATIVE_RENDERABLE 0x302D
#define EGL_NATIVE_VISUAL_ID 0x302E
#define EGL_NATIVE_VISUAL_TYPE 0x302F
#define EGL_SAMPLES 0x3031
#define EGL_SAMPLE_BUFFERS 0x3032
#define EGL_SURFACE_TYPE 0x3033
#define EGL_TRANSPARENT_TYPE 0x3034
#define EGL_TRANSPARENT_BLUE_VALUE 0x3035
#define EGL_TRANSPARENT_GREEN_VALUE 0x3036
#define EGL_TRANSPARENT_RED_VALUE 0x3037
#define EGL_NONE 0x3038 /* Attribute list terminator */
#define EGL_BIND_TO_TEXTURE_RGB 0x3039
#define EGL_BIND_TO_TEXTURE_RGBA 0x303A
#define EGL_MIN_SWAP_INTERVAL 0x303B
#define EGL_MAX_SWAP_INTERVAL 0x303C
#define EGL_LUMINANCE_SIZE 0x303D
#define EGL_ALPHA_MASK_SIZE 0x303E
#define EGL_COLOR_BUFFER_TYPE 0x303F
#define EGL_RENDERABLE_TYPE 0x3040
#define EGL_MATCH_NATIVE_PIXMAP 0x3041
#define EGL_CONFORMANT 0x3042
#define EGL_SLOW_CONFIG 0x3050
#define EGL_NON_CONFORMANT_CONFIG 0x3051
#define EGL_TRANSPARENT_RGB 0x3052
#define EGL_VENDOR 0x3053
#define EGL_VERSION 0x3054
#define EGL_EXTENSIONS 0x3055
#define EGL_HEIGHT 0x3056
#define EGL_WIDTH 0x3057
#define EGL_LARGEST_PBUFFER 0x3058
#define EGL_DRAW 0x3059
#define EGL_READ 0x305A
#define EGL_CORE_NATIVE_ENGINE 0x305B
#define EGL_NO_TEXTURE 0x305C
#define EGL_TEXTURE_RGB 0x305D
#define EGL_TEXTURE_RGBA 0x305E
#define EGL_TEXTURE_2D 0x305F
#define EGL_TEXTURE_FORMAT 0x3080
#define EGL_TEXTURE_TARGET 0x3081
#define EGL_MIPMAP_TEXTURE 0x3082
#define EGL_MIPMAP_LEVEL 0x3083
#define EGL_BACK_BUFFER 0x3084
#define EGL_SINGLE_BUFFER 0x3085
#define EGL_RENDER_BUFFER 0x3086
#define EGL_COLORSPACE 0x3087
#define EGL_VG_COLORSPACE 0x3087
#define EGL_ALPHA_FORMAT 0x3088
#define EGL_VG_ALPHA_FORMAT 0x3088
#define EGL_COLORSPACE_sRGB 0x3089
#define EGL_GL_COLORSPACE_SRGB 0x3089
#define EGL_VG_COLORSPACE_sRGB 0x3089
#define EGL_COLORSPACE_LINEAR 0x308A
#define EGL_GL_COLORSPACE_LINEAR 0x308A
#define EGL_VG_COLORSPACE_LINEAR 0x308A
#define EGL_ALPHA_FORMAT_NONPRE 0x308B
#define EGL_VG_ALPHA_FORMAT_NONPRE 0x308B
#define EGL_ALPHA_FORMAT_PRE 0x308C
#define EGL_VG_ALPHA_FORMAT_PRE 0x308C
#define EGL_CLIENT_APIS 0x308D
#define EGL_RGB_BUFFER 0x308E
#define EGL_LUMINANCE_BUFFER 0x308F
#define EGL_HORIZONTAL_RESOLUTION 0x3090
#define EGL_VERTICAL_RESOLUTION 0x3091
#define EGL_PIXEL_ASPECT_RATIO 0x3092
#define EGL_SWAP_BEHAVIOR 0x3093
#define EGL_BUFFER_PRESERVED 0x3094
#define EGL_BUFFER_DESTROYED 0x3095
#define EGL_OPENVG_IMAGE 0x3096
#define EGL_CONTEXT_CLIENT_TYPE 0x3097
#define EGL_CONTEXT_CLIENT_VERSION 0x3098
#define EGL_CONTEXT_MAJOR_VERSION 0x3098
#define EGL_MULTISAMPLE_RESOLVE 0x3099
#define EGL_MULTISAMPLE_RESOLVE_DEFAULT 0x309A
#define EGL_MULTISAMPLE_RESOLVE_BOX 0x309B
#define EGL_CL_EVENT_HANDLE 0x309C
#define EGL_GL_COLORSPACE 0x309D
#define EGL_OPENGL_ES_API 0x30A0
#define EGL_OPENVG_API 0x30A1
#define EGL_OPENGL_API 0x30A2
#define EGL_GL_TEXTURE_2D 0x30B1
#define EGL_GL_TEXTURE_3D 0x30B2
#define EGL_GL_TEXTURE_CUBE_MAP_POSITIVE_X 0x30B3
#define EGL_GL_TEXTURE_CUBE_MAP_NEGATIVE_X 0x30B4
#define EGL_GL_TEXTURE_CUBE_MAP_POSITIVE_Y 0x30B5
#define EGL_GL_TEXTURE_CUBE_MAP_NEGATIVE_Y 0x30B6
#define EGL_GL_TEXTURE_CUBE_MAP_POSITIVE_Z 0x30B7
#define EGL_GL_TEXTURE_CUBE_MAP_NEGATIVE_Z 0x30B8
#define EGL_GL_RENDERBUFFER 0x30B9
#define EGL_GL_TEXTURE_LEVEL 0x30BC
#define EGL_GL_TEXTURE_ZOFFSET 0x30BD
#define EGL_IMAGE_PRESERVED 0x30D2
#define EGL_SYNC_PRIOR_COMMANDS_COMPLETE 0x30F0
#define EGL_SYNC_STATUS 0x30F1
#define EGL_SIGNALED 0x30F2
#define EGL_UNSIGNALED 0x30F3
#define EGL_TIMEOUT_EXPIRED 0x30F5
#define EGL_CONDITION_SATISFIED 0x30F6
#define EGL_SYNC_TYPE 0x30F7
#define EGL_SYNC_CONDITION 0x30F8
#define EGL_SYNC_FENCE 0x30F9
#define EGL_CONTEXT_MINOR_VERSION 0x30FB
#define EGL_CONTEXT_OPENGL_PROFILE_MASK 0x30FD
#define EGL_SYNC_CL_EVENT 0x30FE
#define EGL_SYNC_CL_EVENT_COMPLETE 0x30FF
#define EGL_CONTEXT_OPENGL_DEBUG 0x31B0
#define EGL_CONTEXT_OPENGL_FORWARD_COMPATIBLE 0x31B1
#define EGL_CONTEXT_OPENGL_ROBUST_ACCESS 0x31B2
#define EGL_CONTEXT_OPENGL_RESET_NOTIFICATION_STRATEGY 0x31BD
#define EGL_NO_RESET_NOTIFICATION 0x31BE
#define EGL_LOSE_CONTEXT_ON_RESET 0x31BF
#define EGL_OBJECT_THREAD_KHR 0x33B0
#define EGL_OBJECT_DISPLAY_KHR 0x33B1
#define EGL_OBJECT_CONTEXT_KHR 0x33B2
#define EGL_OBJECT_SURFACE_KHR 0x33B3
#define EGL_OBJECT_IMAGE_KHR 0x33B4
#define EGL_OBJECT_SYNC_KHR 0x33B5
#define EGL_OBJECT_STREAM_KHR 0x33B6
#define EGL_DEBUG_CALLBACK_KHR 0x33B8
#define EGL_DEBUG_MSG_CRITICAL_KHR 0x33B9
#define EGL_DEBUG_MSG_ERROR_KHR 0x33BA
#define EGL_DEBUG_MSG_WARN_KHR 0x33BB
#define EGL_DEBUG_MSG_INFO_KHR 0x33BC

/* ---- Types ----------------------------------------------------------------
 * Emitted in topological dependency order. Consecutive types sharing the
 * same platform guard are coalesced into a single #ifdef/#endif block.
 */
struct AHardwareBuffer;

struct wl_buffer;

struct wl_display;

struct wl_resource;

typedef unsigned int EGLBoolean;

typedef unsigned int EGLenum;

typedef void *EGLClientBuffer;

typedef void *EGLConfig;

typedef void *EGLContext;

typedef void *EGLDeviceEXT;

typedef void *EGLDisplay;

typedef void *EGLImage;

typedef void *EGLImageKHR;

typedef void *EGLLabelKHR;

typedef void *EGLObjectKHR;

typedef void *EGLOutputLayerEXT;

typedef void *EGLOutputPortEXT;

typedef void *EGLStreamKHR;

typedef void *EGLSurface;

typedef void *EGLSync;

typedef void *EGLSyncKHR;

typedef void *EGLSyncNV;

typedef void (*__eglMustCastToProperFunctionPointerType)(void);

typedef int EGLNativeFileDescriptorKHR;

#define PFNEGLBINDWAYLANDDISPLAYWL PFNEGLBINDWAYLANDDISPLAYWLPROC

#define PFNEGLUNBINDWAYLANDDISPLAYWL PFNEGLUNBINDWAYLANDDISPLAYWLPROC

#define PFNEGLQUERYWAYLANDBUFFERWL PFNEGLQUERYWAYLANDBUFFERWLPROC

#define PFNEGLCREATEWAYLANDBUFFERFROMIMAGEWL PFNEGLCREATEWAYLANDBUFFERFROMIMAGEWLPROC

#include <EGL/eglplatform.h>

typedef intptr_t EGLAttribKHR;

typedef intptr_t EGLAttrib;

typedef khronos_utime_nanoseconds_t EGLTimeKHR;

typedef khronos_utime_nanoseconds_t EGLTime;

typedef khronos_utime_nanoseconds_t EGLTimeNV;

typedef khronos_utime_nanoseconds_t EGLuint64NV;

typedef khronos_stime_nanoseconds_t EGLnsecsANDROID;

typedef khronos_uint64_t EGLuint64KHR;

typedef khronos_ssize_t EGLsizeiANDROID;

struct EGLClientPixmapHI {
    void  *pData;
    EGLint iWidth;
    EGLint iHeight;
    EGLint iStride;
};

typedef void (APIENTRY *EGLDEBUGPROCKHR)(EGLenum error,const char *command,EGLint messageType,EGLLabelKHR threadLabel,EGLLabelKHR objectLabel,const char* message);

typedef void (*EGLSetBlobFuncANDROID) (const void *key, EGLsizeiANDROID keySize, const void *value, EGLsizeiANDROID valueSize);

typedef EGLsizeiANDROID (*EGLGetBlobFuncANDROID) (const void *key, EGLsizeiANDROID keySize, void *value, EGLsizeiANDROID valueSize);


/* ---- PFN typedefs -------------------------------------------------------- */
typedef EGLBoolean (APIENTRYP PFNEGLCHOOSECONFIGPROC)(EGLDisplay dpy, const EGLint * attrib_list, EGLConfig * configs, EGLint config_size, EGLint * num_config);
typedef EGLBoolean (APIENTRYP PFNEGLCOPYBUFFERSPROC)(EGLDisplay dpy, EGLSurface surface, EGLNativePixmapType target);
typedef EGLContext (APIENTRYP PFNEGLCREATECONTEXTPROC)(EGLDisplay dpy, EGLConfig config, EGLContext share_context, const EGLint * attrib_list);
typedef EGLSurface (APIENTRYP PFNEGLCREATEPBUFFERSURFACEPROC)(EGLDisplay dpy, EGLConfig config, const EGLint * attrib_list);
typedef EGLSurface (APIENTRYP PFNEGLCREATEPIXMAPSURFACEPROC)(EGLDisplay dpy, EGLConfig config, EGLNativePixmapType pixmap, const EGLint * attrib_list);
typedef EGLSurface (APIENTRYP PFNEGLCREATEWINDOWSURFACEPROC)(EGLDisplay dpy, EGLConfig config, EGLNativeWindowType win, const EGLint * attrib_list);
typedef EGLBoolean (APIENTRYP PFNEGLDESTROYCONTEXTPROC)(EGLDisplay dpy, EGLContext ctx);
typedef EGLBoolean (APIENTRYP PFNEGLDESTROYSURFACEPROC)(EGLDisplay dpy, EGLSurface surface);
typedef EGLBoolean (APIENTRYP PFNEGLGETCONFIGATTRIBPROC)(EGLDisplay dpy, EGLConfig config, EGLint attribute, EGLint * value);
typedef EGLBoolean (APIENTRYP PFNEGLGETCONFIGSPROC)(EGLDisplay dpy, EGLConfig * configs, EGLint config_size, EGLint * num_config);
typedef EGLDisplay (APIENTRYP PFNEGLGETCURRENTDISPLAYPROC)(void);
typedef EGLSurface (APIENTRYP PFNEGLGETCURRENTSURFACEPROC)(EGLint readdraw);
typedef EGLDisplay (APIENTRYP PFNEGLGETDISPLAYPROC)(EGLNativeDisplayType display_id);
typedef EGLint (APIENTRYP PFNEGLGETERRORPROC)(void);
typedef __eglMustCastToProperFunctionPointerType (APIENTRYP PFNEGLGETPROCADDRESSPROC)(const char * procname);
typedef EGLBoolean (APIENTRYP PFNEGLINITIALIZEPROC)(EGLDisplay dpy, EGLint * major, EGLint * minor);
typedef EGLBoolean (APIENTRYP PFNEGLMAKECURRENTPROC)(EGLDisplay dpy, EGLSurface draw, EGLSurface read, EGLContext ctx);
typedef EGLBoolean (APIENTRYP PFNEGLQUERYCONTEXTPROC)(EGLDisplay dpy, EGLContext ctx, EGLint attribute, EGLint * value);
typedef const char * (APIENTRYP PFNEGLQUERYSTRINGPROC)(EGLDisplay dpy, EGLint name);
typedef EGLBoolean (APIENTRYP PFNEGLQUERYSURFACEPROC)(EGLDisplay dpy, EGLSurface surface, EGLint attribute, EGLint * value);
typedef EGLBoolean (APIENTRYP PFNEGLSWAPBUFFERSPROC)(EGLDisplay dpy, EGLSurface surface);
typedef EGLBoolean (APIENTRYP PFNEGLTERMINATEPROC)(EGLDisplay dpy);
typedef EGLBoolean (APIENTRYP PFNEGLWAITGLPROC)(void);
typedef EGLBoolean (APIENTRYP PFNEGLWAITNATIVEPROC)(EGLint engine);
typedef EGLBoolean (APIENTRYP PFNEGLBINDTEXIMAGEPROC)(EGLDisplay dpy, EGLSurface surface, EGLint buffer);
typedef EGLBoolean (APIENTRYP PFNEGLRELEASETEXIMAGEPROC)(EGLDisplay dpy, EGLSurface surface, EGLint buffer);
typedef EGLBoolean (APIENTRYP PFNEGLSURFACEATTRIBPROC)(EGLDisplay dpy, EGLSurface surface, EGLint attribute, EGLint value);
typedef EGLBoolean (APIENTRYP PFNEGLSWAPINTERVALPROC)(EGLDisplay dpy, EGLint interval);
typedef EGLBoolean (APIENTRYP PFNEGLBINDAPIPROC)(EGLenum api);
typedef EGLSurface (APIENTRYP PFNEGLCREATEPBUFFERFROMCLIENTBUFFERPROC)(EGLDisplay dpy, EGLenum buftype, EGLClientBuffer buffer, EGLConfig config, const EGLint * attrib_list);
typedef EGLenum (APIENTRYP PFNEGLQUERYAPIPROC)(void);
typedef EGLBoolean (APIENTRYP PFNEGLRELEASETHREADPROC)(void);
typedef EGLBoolean (APIENTRYP PFNEGLWAITCLIENTPROC)(void);
typedef EGLContext (APIENTRYP PFNEGLGETCURRENTCONTEXTPROC)(void);
typedef EGLint (APIENTRYP PFNEGLCLIENTWAITSYNCPROC)(EGLDisplay dpy, EGLSync sync, EGLint flags, EGLTime timeout);
typedef EGLImage (APIENTRYP PFNEGLCREATEIMAGEPROC)(EGLDisplay dpy, EGLContext ctx, EGLenum target, EGLClientBuffer buffer, const EGLAttrib * attrib_list);
typedef EGLSurface (APIENTRYP PFNEGLCREATEPLATFORMPIXMAPSURFACEPROC)(EGLDisplay dpy, EGLConfig config, void * native_pixmap, const EGLAttrib * attrib_list);
typedef EGLSurface (APIENTRYP PFNEGLCREATEPLATFORMWINDOWSURFACEPROC)(EGLDisplay dpy, EGLConfig config, void * native_window, const EGLAttrib * attrib_list);
typedef EGLSync (APIENTRYP PFNEGLCREATESYNCPROC)(EGLDisplay dpy, EGLenum type, const EGLAttrib * attrib_list);
typedef EGLBoolean (APIENTRYP PFNEGLDESTROYIMAGEPROC)(EGLDisplay dpy, EGLImage image);
typedef EGLBoolean (APIENTRYP PFNEGLDESTROYSYNCPROC)(EGLDisplay dpy, EGLSync sync);
typedef EGLDisplay (APIENTRYP PFNEGLGETPLATFORMDISPLAYPROC)(EGLenum platform, void * native_display, const EGLAttrib * attrib_list);
typedef EGLBoolean (APIENTRYP PFNEGLGETSYNCATTRIBPROC)(EGLDisplay dpy, EGLSync sync, EGLint attribute, EGLAttrib * value);
typedef EGLBoolean (APIENTRYP PFNEGLWAITSYNCPROC)(EGLDisplay dpy, EGLSync sync, EGLint flags);
typedef EGLint (APIENTRYP PFNEGLDEBUGMESSAGECONTROLKHRPROC)(EGLDEBUGPROCKHR callback, const EGLAttrib * attrib_list);
typedef EGLint (APIENTRYP PFNEGLLABELOBJECTKHRPROC)(EGLDisplay display, EGLenum objectType, EGLObjectKHR object, EGLLabelKHR label);
typedef EGLBoolean (APIENTRYP PFNEGLQUERYDEBUGKHRPROC)(EGLint attribute, EGLAttrib * value);


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
typedef struct GloamEGLContext {
    union {
        unsigned char featArray[6];
        struct {
        /*    0 */ unsigned char VERSION_1_0;
        /*    1 */ unsigned char VERSION_1_1;
        /*    2 */ unsigned char VERSION_1_2;
        /*    3 */ unsigned char VERSION_1_3;
        /*    4 */ unsigned char VERSION_1_4;
        /*    5 */ unsigned char VERSION_1_5;
        };
    };

    union {
        unsigned char extArray[1];
        struct {
        /*    0 */ unsigned char KHR_debug;
        };
    };

    union {
        void *pfnArray[47];
        struct {
        /*    0 */ PFNEGLCHOOSECONFIGPROC ChooseConfig;
        /*    1 */ PFNEGLCOPYBUFFERSPROC CopyBuffers;
        /*    2 */ PFNEGLCREATECONTEXTPROC CreateContext;
        /*    3 */ PFNEGLCREATEPBUFFERSURFACEPROC CreatePbufferSurface;
        /*    4 */ PFNEGLCREATEPIXMAPSURFACEPROC CreatePixmapSurface;
        /*    5 */ PFNEGLCREATEWINDOWSURFACEPROC CreateWindowSurface;
        /*    6 */ PFNEGLDESTROYCONTEXTPROC DestroyContext;
        /*    7 */ PFNEGLDESTROYSURFACEPROC DestroySurface;
        /*    8 */ PFNEGLGETCONFIGATTRIBPROC GetConfigAttrib;
        /*    9 */ PFNEGLGETCONFIGSPROC GetConfigs;
        /*   10 */ PFNEGLGETCURRENTDISPLAYPROC GetCurrentDisplay;
        /*   11 */ PFNEGLGETCURRENTSURFACEPROC GetCurrentSurface;
        /*   12 */ PFNEGLGETDISPLAYPROC GetDisplay;
        /*   13 */ PFNEGLGETERRORPROC GetError;
        /*   14 */ PFNEGLGETPROCADDRESSPROC GetProcAddress;
        /*   15 */ PFNEGLINITIALIZEPROC Initialize;
        /*   16 */ PFNEGLMAKECURRENTPROC MakeCurrent;
        /*   17 */ PFNEGLQUERYCONTEXTPROC QueryContext;
        /*   18 */ PFNEGLQUERYSTRINGPROC QueryString;
        /*   19 */ PFNEGLQUERYSURFACEPROC QuerySurface;
        /*   20 */ PFNEGLSWAPBUFFERSPROC SwapBuffers;
        /*   21 */ PFNEGLTERMINATEPROC Terminate;
        /*   22 */ PFNEGLWAITGLPROC WaitGL;
        /*   23 */ PFNEGLWAITNATIVEPROC WaitNative;
        /*   24 */ PFNEGLBINDTEXIMAGEPROC BindTexImage;
        /*   25 */ PFNEGLRELEASETEXIMAGEPROC ReleaseTexImage;
        /*   26 */ PFNEGLSURFACEATTRIBPROC SurfaceAttrib;
        /*   27 */ PFNEGLSWAPINTERVALPROC SwapInterval;
        /*   28 */ PFNEGLBINDAPIPROC BindAPI;
        /*   29 */ PFNEGLCREATEPBUFFERFROMCLIENTBUFFERPROC CreatePbufferFromClientBuffer;
        /*   30 */ PFNEGLQUERYAPIPROC QueryAPI;
        /*   31 */ PFNEGLRELEASETHREADPROC ReleaseThread;
        /*   32 */ PFNEGLWAITCLIENTPROC WaitClient;
        /*   33 */ PFNEGLGETCURRENTCONTEXTPROC GetCurrentContext;
        /*   34 */ PFNEGLCLIENTWAITSYNCPROC ClientWaitSync;
        /*   35 */ PFNEGLCREATEIMAGEPROC CreateImage;
        /*   36 */ PFNEGLCREATEPLATFORMPIXMAPSURFACEPROC CreatePlatformPixmapSurface;
        /*   37 */ PFNEGLCREATEPLATFORMWINDOWSURFACEPROC CreatePlatformWindowSurface;
        /*   38 */ PFNEGLCREATESYNCPROC CreateSync;
        /*   39 */ PFNEGLDESTROYIMAGEPROC DestroyImage;
        /*   40 */ PFNEGLDESTROYSYNCPROC DestroySync;
        /*   41 */ PFNEGLGETPLATFORMDISPLAYPROC GetPlatformDisplay;
        /*   42 */ PFNEGLGETSYNCATTRIBPROC GetSyncAttrib;
        /*   43 */ PFNEGLWAITSYNCPROC WaitSync;
        /*   44 */ PFNEGLDEBUGMESSAGECONTROLKHRPROC DebugMessageControlKHR;
        /*   45 */ PFNEGLLABELOBJECTKHRPROC LabelObjectKHR;
        /*   46 */ PFNEGLQUERYDEBUGKHRPROC QueryDebugKHR;
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
} GloamEGLContext;

/* Global context instance — a value, not a pointer, so the compiler knows
 * its address is fixed and does not re-load it on every access.
 */
extern GloamEGLContext gloam_egl_context;

/* ---- Feature presence macros --------------------------------------------
 * Test whether a versioned feature was detected at load time.
 */
#define GLOAM_EGL_VERSION_1_0 (gloam_egl_context.VERSION_1_0)
#define GLOAM_EGL_VERSION_1_1 (gloam_egl_context.VERSION_1_1)
#define GLOAM_EGL_VERSION_1_2 (gloam_egl_context.VERSION_1_2)
#define GLOAM_EGL_VERSION_1_3 (gloam_egl_context.VERSION_1_3)
#define GLOAM_EGL_VERSION_1_4 (gloam_egl_context.VERSION_1_4)
#define GLOAM_EGL_VERSION_1_5 (gloam_egl_context.VERSION_1_5)

/* ---- Extension presence macros ------------------------------------------ */
#define GLOAM_EGL_KHR_debug (gloam_egl_context.KHR_debug)


/* ---- Dispatch ------------------------------------------------------------ */

#ifdef __INTELLISENSE__
EGLBoolean eglChooseConfig(EGLDisplay dpy, const EGLint * attrib_list, EGLConfig * configs, EGLint config_size, EGLint * num_config);
EGLBoolean eglCopyBuffers(EGLDisplay dpy, EGLSurface surface, EGLNativePixmapType target);
EGLContext eglCreateContext(EGLDisplay dpy, EGLConfig config, EGLContext share_context, const EGLint * attrib_list);
EGLSurface eglCreatePbufferSurface(EGLDisplay dpy, EGLConfig config, const EGLint * attrib_list);
EGLSurface eglCreatePixmapSurface(EGLDisplay dpy, EGLConfig config, EGLNativePixmapType pixmap, const EGLint * attrib_list);
EGLSurface eglCreateWindowSurface(EGLDisplay dpy, EGLConfig config, EGLNativeWindowType win, const EGLint * attrib_list);
EGLBoolean eglDestroyContext(EGLDisplay dpy, EGLContext ctx);
EGLBoolean eglDestroySurface(EGLDisplay dpy, EGLSurface surface);
EGLBoolean eglGetConfigAttrib(EGLDisplay dpy, EGLConfig config, EGLint attribute, EGLint * value);
EGLBoolean eglGetConfigs(EGLDisplay dpy, EGLConfig * configs, EGLint config_size, EGLint * num_config);
EGLDisplay eglGetCurrentDisplay(void);
EGLSurface eglGetCurrentSurface(EGLint readdraw);
EGLDisplay eglGetDisplay(EGLNativeDisplayType display_id);
EGLint eglGetError(void);
__eglMustCastToProperFunctionPointerType eglGetProcAddress(const char * procname);
EGLBoolean eglInitialize(EGLDisplay dpy, EGLint * major, EGLint * minor);
EGLBoolean eglMakeCurrent(EGLDisplay dpy, EGLSurface draw, EGLSurface read, EGLContext ctx);
EGLBoolean eglQueryContext(EGLDisplay dpy, EGLContext ctx, EGLint attribute, EGLint * value);
const char * eglQueryString(EGLDisplay dpy, EGLint name);
EGLBoolean eglQuerySurface(EGLDisplay dpy, EGLSurface surface, EGLint attribute, EGLint * value);
EGLBoolean eglSwapBuffers(EGLDisplay dpy, EGLSurface surface);
EGLBoolean eglTerminate(EGLDisplay dpy);
EGLBoolean eglWaitGL(void);
EGLBoolean eglWaitNative(EGLint engine);
EGLBoolean eglBindTexImage(EGLDisplay dpy, EGLSurface surface, EGLint buffer);
EGLBoolean eglReleaseTexImage(EGLDisplay dpy, EGLSurface surface, EGLint buffer);
EGLBoolean eglSurfaceAttrib(EGLDisplay dpy, EGLSurface surface, EGLint attribute, EGLint value);
EGLBoolean eglSwapInterval(EGLDisplay dpy, EGLint interval);
EGLBoolean eglBindAPI(EGLenum api);
EGLSurface eglCreatePbufferFromClientBuffer(EGLDisplay dpy, EGLenum buftype, EGLClientBuffer buffer, EGLConfig config, const EGLint * attrib_list);
EGLenum eglQueryAPI(void);
EGLBoolean eglReleaseThread(void);
EGLBoolean eglWaitClient(void);
EGLContext eglGetCurrentContext(void);
EGLint eglClientWaitSync(EGLDisplay dpy, EGLSync sync, EGLint flags, EGLTime timeout);
EGLImage eglCreateImage(EGLDisplay dpy, EGLContext ctx, EGLenum target, EGLClientBuffer buffer, const EGLAttrib * attrib_list);
EGLSurface eglCreatePlatformPixmapSurface(EGLDisplay dpy, EGLConfig config, void * native_pixmap, const EGLAttrib * attrib_list);
EGLSurface eglCreatePlatformWindowSurface(EGLDisplay dpy, EGLConfig config, void * native_window, const EGLAttrib * attrib_list);
EGLSync eglCreateSync(EGLDisplay dpy, EGLenum type, const EGLAttrib * attrib_list);
EGLBoolean eglDestroyImage(EGLDisplay dpy, EGLImage image);
EGLBoolean eglDestroySync(EGLDisplay dpy, EGLSync sync);
EGLDisplay eglGetPlatformDisplay(EGLenum platform, void * native_display, const EGLAttrib * attrib_list);
EGLBoolean eglGetSyncAttrib(EGLDisplay dpy, EGLSync sync, EGLint attribute, EGLAttrib * value);
EGLBoolean eglWaitSync(EGLDisplay dpy, EGLSync sync, EGLint flags);
EGLint eglDebugMessageControlKHR(EGLDEBUGPROCKHR callback, const EGLAttrib * attrib_list);
EGLint eglLabelObjectKHR(EGLDisplay display, EGLenum objectType, EGLObjectKHR object, EGLLabelKHR label);
EGLBoolean eglQueryDebugKHR(EGLint attribute, EGLAttrib * value);
#else
#define eglChooseConfig (gloam_egl_context.ChooseConfig)
#define eglCopyBuffers (gloam_egl_context.CopyBuffers)
#define eglCreateContext (gloam_egl_context.CreateContext)
#define eglCreatePbufferSurface (gloam_egl_context.CreatePbufferSurface)
#define eglCreatePixmapSurface (gloam_egl_context.CreatePixmapSurface)
#define eglCreateWindowSurface (gloam_egl_context.CreateWindowSurface)
#define eglDestroyContext (gloam_egl_context.DestroyContext)
#define eglDestroySurface (gloam_egl_context.DestroySurface)
#define eglGetConfigAttrib (gloam_egl_context.GetConfigAttrib)
#define eglGetConfigs (gloam_egl_context.GetConfigs)
#define eglGetCurrentDisplay (gloam_egl_context.GetCurrentDisplay)
#define eglGetCurrentSurface (gloam_egl_context.GetCurrentSurface)
#define eglGetDisplay (gloam_egl_context.GetDisplay)
#define eglGetError (gloam_egl_context.GetError)
#define eglGetProcAddress (gloam_egl_context.GetProcAddress)
#define eglInitialize (gloam_egl_context.Initialize)
#define eglMakeCurrent (gloam_egl_context.MakeCurrent)
#define eglQueryContext (gloam_egl_context.QueryContext)
#define eglQueryString (gloam_egl_context.QueryString)
#define eglQuerySurface (gloam_egl_context.QuerySurface)
#define eglSwapBuffers (gloam_egl_context.SwapBuffers)
#define eglTerminate (gloam_egl_context.Terminate)
#define eglWaitGL (gloam_egl_context.WaitGL)
#define eglWaitNative (gloam_egl_context.WaitNative)
#define eglBindTexImage (gloam_egl_context.BindTexImage)
#define eglReleaseTexImage (gloam_egl_context.ReleaseTexImage)
#define eglSurfaceAttrib (gloam_egl_context.SurfaceAttrib)
#define eglSwapInterval (gloam_egl_context.SwapInterval)
#define eglBindAPI (gloam_egl_context.BindAPI)
#define eglCreatePbufferFromClientBuffer (gloam_egl_context.CreatePbufferFromClientBuffer)
#define eglQueryAPI (gloam_egl_context.QueryAPI)
#define eglReleaseThread (gloam_egl_context.ReleaseThread)
#define eglWaitClient (gloam_egl_context.WaitClient)
#define eglGetCurrentContext (gloam_egl_context.GetCurrentContext)
#define eglClientWaitSync (gloam_egl_context.ClientWaitSync)
#define eglCreateImage (gloam_egl_context.CreateImage)
#define eglCreatePlatformPixmapSurface (gloam_egl_context.CreatePlatformPixmapSurface)
#define eglCreatePlatformWindowSurface (gloam_egl_context.CreatePlatformWindowSurface)
#define eglCreateSync (gloam_egl_context.CreateSync)
#define eglDestroyImage (gloam_egl_context.DestroyImage)
#define eglDestroySync (gloam_egl_context.DestroySync)
#define eglGetPlatformDisplay (gloam_egl_context.GetPlatformDisplay)
#define eglGetSyncAttrib (gloam_egl_context.GetSyncAttrib)
#define eglWaitSync (gloam_egl_context.WaitSync)
#define eglDebugMessageControlKHR (gloam_egl_context.DebugMessageControlKHR)
#define eglLabelObjectKHR (gloam_egl_context.LabelObjectKHR)
#define eglQueryDebugKHR (gloam_egl_context.QueryDebugKHR)
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

int gloamLoadEGLContext(GloamEGLContext *context, EGLDisplay display, GloamLoadFunc getProcAddr);
int gloamLoadEGL(EGLDisplay display, GloamLoadFunc getProcAddr);
/* Built-in loader: opens the platform library if needed and calls the
 * appropriate load function for you. Non-Vulkan loaders call the detection-
 * based gloamLoad* functions. Vulkan loaders handle all extension detection
 * and PFN loading internally.
 * Each Load function may be called multiple times (additive).
 */
int  gloamLoaderLoadEGLContext(GloamEGLContext *context, EGLDisplay display);
int  gloamLoaderLoadEGL(EGLDisplay display);
void gloamLoaderUnloadEGLContext(GloamEGLContext *context);
void gloamLoaderUnloadEGL(void);
void gloamLoaderResetEGLContext(GloamEGLContext *context);
void gloamLoaderResetEGL(void);

#ifdef __cplusplus
}
#endif

#endif /* GLOAM_EGL_H */
