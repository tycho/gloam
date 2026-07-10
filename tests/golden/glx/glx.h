#ifndef GLOAM_GLX_H
#define GLOAM_GLX_H

#ifdef GLX_H
  #error GLX (glx.h) header already included (API: glx), remove previous include!
#endif
#define GLX_H 1

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

#ifdef GLOAM_PLATFORM_LINUX

#include <gloam/gl.h>

#include <X11/X.h>
#include <X11/Xlib.h>
#include <X11/Xutil.h>

#include <stddef.h>
#include <stdint.h>

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
#define GLX_VERSION_1_0 1
#define GLX_VERSION_1_1 1
#define GLX_VERSION_1_2 1
#define GLX_VERSION_1_3 1
#define GLX_VERSION_1_4 1
/* ---- Constants ----------------------------------------------------------- */
#define GLX_EXTENSION_NAME "GLX" /* This is modest abuse of the enum tag mechanism, maybe a string tag? */
#define GLX_VENDOR 0x1
#define GLX_VERSION 0x2
#define GLX_EXTENSIONS 0x3
#define GLX_PbufferClobber 0
#define GLX_BufferSwapComplete 1
#define __GLX_NUMBER_EVENTS 17
#define GLX_BAD_SCREEN 1
#define GLX_BAD_ATTRIBUTE 2
#define GLX_NO_EXTENSION 3
#define GLX_BAD_VISUAL 4
#define GLX_BAD_CONTEXT 5
#define GLX_BAD_VALUE 6
#define GLX_BAD_ENUM 7
#define GLX_WINDOW_BIT 0x00000001
#define GLX_PIXMAP_BIT 0x00000002
#define GLX_PBUFFER_BIT 0x00000004
#define GLX_RGBA_BIT 0x00000001
#define GLX_COLOR_INDEX_BIT 0x00000002
#define GLX_PBUFFER_CLOBBER_MASK 0x08000000
#define GLX_FRONT_LEFT_BUFFER_BIT 0x00000001
#define GLX_FRONT_RIGHT_BUFFER_BIT 0x00000002
#define GLX_BACK_LEFT_BUFFER_BIT 0x00000004
#define GLX_BACK_RIGHT_BUFFER_BIT 0x00000008
#define GLX_AUX_BUFFERS_BIT 0x00000010
#define GLX_DEPTH_BUFFER_BIT 0x00000020
#define GLX_STENCIL_BUFFER_BIT 0x00000040
#define GLX_ACCUM_BUFFER_BIT 0x00000080
#define GLX_DONT_CARE 0xFFFFFFFF /* For ChooseFBConfig attributes */
#define GLX_USE_GL 1
#define GLX_BUFFER_SIZE 2
#define GLX_LEVEL 3
#define GLX_RGBA 4
#define GLX_DOUBLEBUFFER 5
#define GLX_STEREO 6
#define GLX_AUX_BUFFERS 7
#define GLX_RED_SIZE 8
#define GLX_GREEN_SIZE 9
#define GLX_BLUE_SIZE 10
#define GLX_ALPHA_SIZE 11
#define GLX_DEPTH_SIZE 12
#define GLX_STENCIL_SIZE 13
#define GLX_ACCUM_RED_SIZE 14
#define GLX_ACCUM_GREEN_SIZE 15
#define GLX_ACCUM_BLUE_SIZE 16
#define GLX_ACCUM_ALPHA_SIZE 17
#define GLX_CONFIG_CAVEAT 0x20
#define GLX_X_VISUAL_TYPE 0x22
#define GLX_TRANSPARENT_TYPE 0x23
#define GLX_TRANSPARENT_INDEX_VALUE 0x24
#define GLX_TRANSPARENT_RED_VALUE 0x25
#define GLX_TRANSPARENT_GREEN_VALUE 0x26
#define GLX_TRANSPARENT_BLUE_VALUE 0x27
#define GLX_TRANSPARENT_ALPHA_VALUE 0x28
#define GLX_NONE 0x8000 /* Attribute value */
#define GLX_SLOW_CONFIG 0x8001 /* CONFIG_CAVEAT attribute value */
#define GLX_TRUE_COLOR 0x8002 /* X_VISUAL_TYPE attribute value */
#define GLX_DIRECT_COLOR 0x8003 /* X_VISUAL_TYPE attribute value */
#define GLX_PSEUDO_COLOR 0x8004 /* X_VISUAL_TYPE attribute value */
#define GLX_STATIC_COLOR 0x8005 /* X_VISUAL_TYPE attribute value */
#define GLX_GRAY_SCALE 0x8006 /* X_VISUAL_TYPE attribute value */
#define GLX_STATIC_GRAY 0x8007 /* X_VISUAL_TYPE attribute value */
#define GLX_TRANSPARENT_RGB 0x8008 /* TRANSPARENT_TYPE attribute value */
#define GLX_TRANSPARENT_INDEX 0x8009 /* TRANSPARENT_TYPE attribute value */
#define GLX_VISUAL_ID 0x800B /* Context attribute */
#define GLX_SCREEN 0x800C /* Context attribute */
#define GLX_NON_CONFORMANT_CONFIG 0x800D /* CONFIG_CAVEAT attribute value */
#define GLX_DRAWABLE_TYPE 0x8010 /* FBConfig attribute */
#define GLX_RENDER_TYPE 0x8011 /* FBConfig attribute */
#define GLX_X_RENDERABLE 0x8012 /* FBConfig attribute */
#define GLX_FBCONFIG_ID 0x8013 /* FBConfig attribute */
#define GLX_RGBA_TYPE 0x8014 /* CreateNewContext render_type value */
#define GLX_COLOR_INDEX_TYPE 0x8015 /* CreateNewContext render_type value */
#define GLX_MAX_PBUFFER_WIDTH 0x8016 /* FBConfig attribute */
#define GLX_MAX_PBUFFER_HEIGHT 0x8017 /* FBConfig attribute */
#define GLX_MAX_PBUFFER_PIXELS 0x8018 /* FBConfig attribute */
#define GLX_PRESERVED_CONTENTS 0x801B /* CreateGLXPbuffer attribute */
#define GLX_LARGEST_PBUFFER 0x801C /* CreateGLXPbuffer attribute */
#define GLX_WIDTH 0x801D /* Drawable attribute */
#define GLX_HEIGHT 0x801E /* Drawable attribute */
#define GLX_EVENT_MASK 0x801F /* Drawable attribute */
#define GLX_DAMAGED 0x8020 /* PbufferClobber event_type value */
#define GLX_SAVED 0x8021 /* PbufferClobber event_type value */
#define GLX_WINDOW 0x8022 /* PbufferClobber draw_type value */
#define GLX_PBUFFER 0x8023 /* PbufferClobber draw_type value */
#define GLX_PBUFFER_HEIGHT 0x8040 /* CreateGLXPbuffer attribute */
#define GLX_PBUFFER_WIDTH 0x8041 /* CreateGLXPbuffer attribute */
#define GLX_SAMPLE_BUFFERS 100000
#define GLX_SAMPLES 100001

/* ---- Types ----------------------------------------------------------------
 * Emitted in topological dependency order. Consecutive types sharing the
 * same platform guard are coalesced into a single #ifdef/#endif block.
 */
#if defined(__ENVIRONMENT_MAC_OS_X_VERSION_MIN_REQUIRED__) \
&& (__ENVIRONMENT_MAC_OS_X_VERSION_MIN_REQUIRED__ > 1060)
typedef long GLintptr;
#else
typedef ptrdiff_t GLintptr;
#endif

#if defined(__ENVIRONMENT_MAC_OS_X_VERSION_MIN_REQUIRED__) \
&& (__ENVIRONMENT_MAC_OS_X_VERSION_MIN_REQUIRED__ > 1060)
typedef long GLsizeiptr;
#else
typedef ptrdiff_t GLsizeiptr;
#endif

typedef struct __GLXFBConfigRec *GLXFBConfig;

typedef struct __GLXcontextRec *GLXContext;

typedef void (APIENTRY *__GLXextFuncPtr)(void);

typedef unsigned int GLXVideoDeviceNV;

typedef struct __GLXFBConfigRec *GLXFBConfigSGIX;

typedef struct {
    char    pipeName[80]; /* Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX] */
    int     networkId;
} GLXHyperpipeNetworkSGIX;

typedef struct {
    char    pipeName[80]; /* Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX] */
    int     channel;
    unsigned int participationType;
    int     timeSlice;
} GLXHyperpipeConfigSGIX;

typedef struct {
    char pipeName[80]; /* Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX] */
    int srcXOrigin, srcYOrigin, srcWidth, srcHeight;
    int destXOrigin, destYOrigin, destWidth, destHeight;
} GLXPipeRect;

typedef struct {
    char pipeName[80]; /* Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX] */
    int XOrigin, YOrigin, maxHeight, maxWidth;
} GLXPipeRectLimits;

typedef XID GLXFBConfigID;

typedef XID GLXContextID;

typedef XID GLXPixmap;

typedef XID GLXDrawable;

typedef XID GLXWindow;

typedef XID GLXPbuffer;

typedef XID GLXVideoCaptureDeviceNV;

typedef XID GLXVideoSourceSGIX;

typedef XID GLXFBConfigIDSGIX;

typedef XID GLXPbufferSGIX;

typedef struct {
    int event_type;             /* GLX_DAMAGED or GLX_SAVED */
    int draw_type;              /* GLX_WINDOW or GLX_PBUFFER */
    unsigned long serial;       /* # of last request processed by server */
    Bool send_event;            /* true if this came for SendEvent request */
    Display *display;           /* display the event was read from */
    GLXDrawable drawable;       /* XID of Drawable */
    unsigned int buffer_mask;   /* mask indicating which buffers are affected */
    unsigned int aux_buffer;    /* which aux buffer was affected */
    int x, y;
    int width, height;
    int count;                  /* if nonzero, at least this many more */
} GLXPbufferClobberEvent;

typedef struct {
    int type;
    unsigned long serial;
    Bool send_event;
    Display *display;
    int extension;
    int evtype;
    GLXDrawable window;
    Bool stereo_tree;
} GLXStereoNotifyEventEXT;

typedef struct {
    int type;
    unsigned long serial;   /* # of last request processed by server */
    Bool send_event;        /* true if this came for SendEvent request */
    Display *display;       /* display the event was read from */
    GLXDrawable drawable;   /* i.d. of Drawable */
    int event_type;         /* GLX_DAMAGED_SGIX or GLX_SAVED_SGIX */
    int draw_type;          /* GLX_WINDOW_SGIX or GLX_PBUFFER_SGIX */
    unsigned int mask;      /* mask indicating which buffers are affected*/
    int x, y;
    int width, height;
    int count;              /* if nonzero, at least this many more */
} GLXBufferClobberEventSGIX;

typedef struct {
    int type;
    unsigned long serial;       /* # of last request processed by server */
    Bool send_event;            /* true if this came from a SendEvent request */
    Display *display;           /* Display the event was read from */
    GLXDrawable drawable;       /* drawable on which event was requested in event mask */
    int event_type;
    int64_t ust;
    int64_t msc;
    int64_t sbc;
} GLXBufferSwapComplete;

typedef union __GLXEvent {
    GLXPbufferClobberEvent glxpbufferclobber;
    GLXBufferSwapComplete glxbufferswapcomplete;
    long pad[24];
} GLXEvent;


/* ---- PFN typedefs -------------------------------------------------------- */
typedef XVisualInfo * (APIENTRYP PFNGLXCHOOSEVISUALPROC)(Display * dpy, int screen, int * attribList);
typedef void (APIENTRYP PFNGLXCOPYCONTEXTPROC)(Display * dpy, GLXContext src, GLXContext dst, unsigned long mask);
typedef GLXContext (APIENTRYP PFNGLXCREATECONTEXTPROC)(Display * dpy, XVisualInfo * vis, GLXContext shareList, Bool direct);
typedef GLXPixmap (APIENTRYP PFNGLXCREATEGLXPIXMAPPROC)(Display * dpy, XVisualInfo * visual, Pixmap pixmap);
typedef void (APIENTRYP PFNGLXDESTROYCONTEXTPROC)(Display * dpy, GLXContext ctx);
typedef void (APIENTRYP PFNGLXDESTROYGLXPIXMAPPROC)(Display * dpy, GLXPixmap pixmap);
typedef int (APIENTRYP PFNGLXGETCONFIGPROC)(Display * dpy, XVisualInfo * visual, int attrib, int * value);
typedef GLXContext (APIENTRYP PFNGLXGETCURRENTCONTEXTPROC)(void);
typedef GLXDrawable (APIENTRYP PFNGLXGETCURRENTDRAWABLEPROC)(void);
typedef Bool (APIENTRYP PFNGLXISDIRECTPROC)(Display * dpy, GLXContext ctx);
typedef Bool (APIENTRYP PFNGLXMAKECURRENTPROC)(Display * dpy, GLXDrawable drawable, GLXContext ctx);
typedef Bool (APIENTRYP PFNGLXQUERYEXTENSIONPROC)(Display * dpy, int * errorb, int * event);
typedef Bool (APIENTRYP PFNGLXQUERYVERSIONPROC)(Display * dpy, int * maj, int * min);
typedef void (APIENTRYP PFNGLXSWAPBUFFERSPROC)(Display * dpy, GLXDrawable drawable);
typedef void (APIENTRYP PFNGLXUSEXFONTPROC)(Font font, int first, int count, int list);
typedef void (APIENTRYP PFNGLXWAITGLPROC)(void);
typedef void (APIENTRYP PFNGLXWAITXPROC)(void);
typedef const char * (APIENTRYP PFNGLXGETCLIENTSTRINGPROC)(Display * dpy, int name);
typedef const char * (APIENTRYP PFNGLXQUERYEXTENSIONSSTRINGPROC)(Display * dpy, int screen);
typedef const char * (APIENTRYP PFNGLXQUERYSERVERSTRINGPROC)(Display * dpy, int screen, int name);
typedef Display * (APIENTRYP PFNGLXGETCURRENTDISPLAYPROC)(void);
typedef GLXFBConfig * (APIENTRYP PFNGLXCHOOSEFBCONFIGPROC)(Display * dpy, int screen, const int * attrib_list, int * nelements);
typedef GLXContext (APIENTRYP PFNGLXCREATENEWCONTEXTPROC)(Display * dpy, GLXFBConfig config, int render_type, GLXContext share_list, Bool direct);
typedef GLXPbuffer (APIENTRYP PFNGLXCREATEPBUFFERPROC)(Display * dpy, GLXFBConfig config, const int * attrib_list);
typedef GLXPixmap (APIENTRYP PFNGLXCREATEPIXMAPPROC)(Display * dpy, GLXFBConfig config, Pixmap pixmap, const int * attrib_list);
typedef GLXWindow (APIENTRYP PFNGLXCREATEWINDOWPROC)(Display * dpy, GLXFBConfig config, Window win, const int * attrib_list);
typedef void (APIENTRYP PFNGLXDESTROYPBUFFERPROC)(Display * dpy, GLXPbuffer pbuf);
typedef void (APIENTRYP PFNGLXDESTROYPIXMAPPROC)(Display * dpy, GLXPixmap pixmap);
typedef void (APIENTRYP PFNGLXDESTROYWINDOWPROC)(Display * dpy, GLXWindow win);
typedef GLXDrawable (APIENTRYP PFNGLXGETCURRENTREADDRAWABLEPROC)(void);
typedef int (APIENTRYP PFNGLXGETFBCONFIGATTRIBPROC)(Display * dpy, GLXFBConfig config, int attribute, int * value);
typedef GLXFBConfig * (APIENTRYP PFNGLXGETFBCONFIGSPROC)(Display * dpy, int screen, int * nelements);
typedef void (APIENTRYP PFNGLXGETSELECTEDEVENTPROC)(Display * dpy, GLXDrawable draw, unsigned long * event_mask);
typedef XVisualInfo * (APIENTRYP PFNGLXGETVISUALFROMFBCONFIGPROC)(Display * dpy, GLXFBConfig config);
typedef Bool (APIENTRYP PFNGLXMAKECONTEXTCURRENTPROC)(Display * dpy, GLXDrawable draw, GLXDrawable read, GLXContext ctx);
typedef int (APIENTRYP PFNGLXQUERYCONTEXTPROC)(Display * dpy, GLXContext ctx, int attribute, int * value);
typedef void (APIENTRYP PFNGLXQUERYDRAWABLEPROC)(Display * dpy, GLXDrawable draw, int attribute, unsigned int * value);
typedef void (APIENTRYP PFNGLXSELECTEVENTPROC)(Display * dpy, GLXDrawable draw, unsigned long event_mask);
typedef __GLXextFuncPtr (APIENTRYP PFNGLXGETPROCADDRESSPROC)(const GLubyte * procName);


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
typedef struct GloamGLXContext {
    union {
        unsigned char featArray[5];
        struct {
        /*    0 */ unsigned char VERSION_1_0;
        /*    1 */ unsigned char VERSION_1_1;
        /*    2 */ unsigned char VERSION_1_2;
        /*    3 */ unsigned char VERSION_1_3;
        /*    4 */ unsigned char VERSION_1_4;
        };
    };

    union {
        void *pfnArray[39];
        struct {
        /*    0 */ PFNGLXCHOOSEVISUALPROC ChooseVisual;
        /*    1 */ PFNGLXCOPYCONTEXTPROC CopyContext;
        /*    2 */ PFNGLXCREATECONTEXTPROC CreateContext;
        /*    3 */ PFNGLXCREATEGLXPIXMAPPROC CreateGLXPixmap;
        /*    4 */ PFNGLXDESTROYCONTEXTPROC DestroyContext;
        /*    5 */ PFNGLXDESTROYGLXPIXMAPPROC DestroyGLXPixmap;
        /*    6 */ PFNGLXGETCONFIGPROC GetConfig;
        /*    7 */ PFNGLXGETCURRENTCONTEXTPROC GetCurrentContext;
        /*    8 */ PFNGLXGETCURRENTDRAWABLEPROC GetCurrentDrawable;
        /*    9 */ PFNGLXISDIRECTPROC IsDirect;
        /*   10 */ PFNGLXMAKECURRENTPROC MakeCurrent;
        /*   11 */ PFNGLXQUERYEXTENSIONPROC QueryExtension;
        /*   12 */ PFNGLXQUERYVERSIONPROC QueryVersion;
        /*   13 */ PFNGLXSWAPBUFFERSPROC SwapBuffers;
        /*   14 */ PFNGLXUSEXFONTPROC UseXFont;
        /*   15 */ PFNGLXWAITGLPROC WaitGL;
        /*   16 */ PFNGLXWAITXPROC WaitX;
        /*   17 */ PFNGLXGETCLIENTSTRINGPROC GetClientString;
        /*   18 */ PFNGLXQUERYEXTENSIONSSTRINGPROC QueryExtensionsString;
        /*   19 */ PFNGLXQUERYSERVERSTRINGPROC QueryServerString;
        /*   20 */ PFNGLXGETCURRENTDISPLAYPROC GetCurrentDisplay;
        /*   21 */ PFNGLXCHOOSEFBCONFIGPROC ChooseFBConfig;
        /*   22 */ PFNGLXCREATENEWCONTEXTPROC CreateNewContext;
        /*   23 */ PFNGLXCREATEPBUFFERPROC CreatePbuffer;
        /*   24 */ PFNGLXCREATEPIXMAPPROC CreatePixmap;
        /*   25 */ PFNGLXCREATEWINDOWPROC CreateWindow;
        /*   26 */ PFNGLXDESTROYPBUFFERPROC DestroyPbuffer;
        /*   27 */ PFNGLXDESTROYPIXMAPPROC DestroyPixmap;
        /*   28 */ PFNGLXDESTROYWINDOWPROC DestroyWindow;
        /*   29 */ PFNGLXGETCURRENTREADDRAWABLEPROC GetCurrentReadDrawable;
        /*   30 */ PFNGLXGETFBCONFIGATTRIBPROC GetFBConfigAttrib;
        /*   31 */ PFNGLXGETFBCONFIGSPROC GetFBConfigs;
        /*   32 */ PFNGLXGETSELECTEDEVENTPROC GetSelectedEvent;
        /*   33 */ PFNGLXGETVISUALFROMFBCONFIGPROC GetVisualFromFBConfig;
        /*   34 */ PFNGLXMAKECONTEXTCURRENTPROC MakeContextCurrent;
        /*   35 */ PFNGLXQUERYCONTEXTPROC QueryContext;
        /*   36 */ PFNGLXQUERYDRAWABLEPROC QueryDrawable;
        /*   37 */ PFNGLXSELECTEVENTPROC SelectEvent;
        /*   38 */ PFNGLXGETPROCADDRESSPROC GetProcAddress;
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
} GloamGLXContext;

/* Global context instance — a value, not a pointer, so the compiler knows
 * its address is fixed and does not re-load it on every access.
 */
extern GloamGLXContext gloam_glx_context;

/* ---- Feature presence macros --------------------------------------------
 * Test whether a versioned feature was detected at load time.
 */
#define GLOAM_GLX_VERSION_1_0 (gloam_glx_context.VERSION_1_0)
#define GLOAM_GLX_VERSION_1_1 (gloam_glx_context.VERSION_1_1)
#define GLOAM_GLX_VERSION_1_2 (gloam_glx_context.VERSION_1_2)
#define GLOAM_GLX_VERSION_1_3 (gloam_glx_context.VERSION_1_3)
#define GLOAM_GLX_VERSION_1_4 (gloam_glx_context.VERSION_1_4)

/* ---- Extension presence macros ------------------------------------------ */


/* ---- Dispatch ------------------------------------------------------------ */

#ifdef __INTELLISENSE__
XVisualInfo * glXChooseVisual(Display * dpy, int screen, int * attribList);
void glXCopyContext(Display * dpy, GLXContext src, GLXContext dst, unsigned long mask);
GLXContext glXCreateContext(Display * dpy, XVisualInfo * vis, GLXContext shareList, Bool direct);
GLXPixmap glXCreateGLXPixmap(Display * dpy, XVisualInfo * visual, Pixmap pixmap);
void glXDestroyContext(Display * dpy, GLXContext ctx);
void glXDestroyGLXPixmap(Display * dpy, GLXPixmap pixmap);
int glXGetConfig(Display * dpy, XVisualInfo * visual, int attrib, int * value);
GLXContext glXGetCurrentContext(void);
GLXDrawable glXGetCurrentDrawable(void);
Bool glXIsDirect(Display * dpy, GLXContext ctx);
Bool glXMakeCurrent(Display * dpy, GLXDrawable drawable, GLXContext ctx);
Bool glXQueryExtension(Display * dpy, int * errorb, int * event);
Bool glXQueryVersion(Display * dpy, int * maj, int * min);
void glXSwapBuffers(Display * dpy, GLXDrawable drawable);
void glXUseXFont(Font font, int first, int count, int list);
void glXWaitGL(void);
void glXWaitX(void);
const char * glXGetClientString(Display * dpy, int name);
const char * glXQueryExtensionsString(Display * dpy, int screen);
const char * glXQueryServerString(Display * dpy, int screen, int name);
Display * glXGetCurrentDisplay(void);
GLXFBConfig * glXChooseFBConfig(Display * dpy, int screen, const int * attrib_list, int * nelements);
GLXContext glXCreateNewContext(Display * dpy, GLXFBConfig config, int render_type, GLXContext share_list, Bool direct);
GLXPbuffer glXCreatePbuffer(Display * dpy, GLXFBConfig config, const int * attrib_list);
GLXPixmap glXCreatePixmap(Display * dpy, GLXFBConfig config, Pixmap pixmap, const int * attrib_list);
GLXWindow glXCreateWindow(Display * dpy, GLXFBConfig config, Window win, const int * attrib_list);
void glXDestroyPbuffer(Display * dpy, GLXPbuffer pbuf);
void glXDestroyPixmap(Display * dpy, GLXPixmap pixmap);
void glXDestroyWindow(Display * dpy, GLXWindow win);
GLXDrawable glXGetCurrentReadDrawable(void);
int glXGetFBConfigAttrib(Display * dpy, GLXFBConfig config, int attribute, int * value);
GLXFBConfig * glXGetFBConfigs(Display * dpy, int screen, int * nelements);
void glXGetSelectedEvent(Display * dpy, GLXDrawable draw, unsigned long * event_mask);
XVisualInfo * glXGetVisualFromFBConfig(Display * dpy, GLXFBConfig config);
Bool glXMakeContextCurrent(Display * dpy, GLXDrawable draw, GLXDrawable read, GLXContext ctx);
int glXQueryContext(Display * dpy, GLXContext ctx, int attribute, int * value);
void glXQueryDrawable(Display * dpy, GLXDrawable draw, int attribute, unsigned int * value);
void glXSelectEvent(Display * dpy, GLXDrawable draw, unsigned long event_mask);
__GLXextFuncPtr glXGetProcAddress(const GLubyte * procName);
#else
#define glXChooseVisual (gloam_glx_context.ChooseVisual)
#define glXCopyContext (gloam_glx_context.CopyContext)
#define glXCreateContext (gloam_glx_context.CreateContext)
#define glXCreateGLXPixmap (gloam_glx_context.CreateGLXPixmap)
#define glXDestroyContext (gloam_glx_context.DestroyContext)
#define glXDestroyGLXPixmap (gloam_glx_context.DestroyGLXPixmap)
#define glXGetConfig (gloam_glx_context.GetConfig)
#define glXGetCurrentContext (gloam_glx_context.GetCurrentContext)
#define glXGetCurrentDrawable (gloam_glx_context.GetCurrentDrawable)
#define glXIsDirect (gloam_glx_context.IsDirect)
#define glXMakeCurrent (gloam_glx_context.MakeCurrent)
#define glXQueryExtension (gloam_glx_context.QueryExtension)
#define glXQueryVersion (gloam_glx_context.QueryVersion)
#define glXSwapBuffers (gloam_glx_context.SwapBuffers)
#define glXUseXFont (gloam_glx_context.UseXFont)
#define glXWaitGL (gloam_glx_context.WaitGL)
#define glXWaitX (gloam_glx_context.WaitX)
#define glXGetClientString (gloam_glx_context.GetClientString)
#define glXQueryExtensionsString (gloam_glx_context.QueryExtensionsString)
#define glXQueryServerString (gloam_glx_context.QueryServerString)
#define glXGetCurrentDisplay (gloam_glx_context.GetCurrentDisplay)
#define glXChooseFBConfig (gloam_glx_context.ChooseFBConfig)
#define glXCreateNewContext (gloam_glx_context.CreateNewContext)
#define glXCreatePbuffer (gloam_glx_context.CreatePbuffer)
#define glXCreatePixmap (gloam_glx_context.CreatePixmap)
#define glXCreateWindow (gloam_glx_context.CreateWindow)
#define glXDestroyPbuffer (gloam_glx_context.DestroyPbuffer)
#define glXDestroyPixmap (gloam_glx_context.DestroyPixmap)
#define glXDestroyWindow (gloam_glx_context.DestroyWindow)
#define glXGetCurrentReadDrawable (gloam_glx_context.GetCurrentReadDrawable)
#define glXGetFBConfigAttrib (gloam_glx_context.GetFBConfigAttrib)
#define glXGetFBConfigs (gloam_glx_context.GetFBConfigs)
#define glXGetSelectedEvent (gloam_glx_context.GetSelectedEvent)
#define glXGetVisualFromFBConfig (gloam_glx_context.GetVisualFromFBConfig)
#define glXMakeContextCurrent (gloam_glx_context.MakeContextCurrent)
#define glXQueryContext (gloam_glx_context.QueryContext)
#define glXQueryDrawable (gloam_glx_context.QueryDrawable)
#define glXSelectEvent (gloam_glx_context.SelectEvent)
#define glXGetProcAddress (gloam_glx_context.GetProcAddress)
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

int gloamLoadGLXContext(GloamGLXContext *context, Display *display, int screen, GloamLoadFunc getProcAddr);
int gloamLoadGLX(Display *display, int screen, GloamLoadFunc getProcAddr);
/* Built-in loader: opens the platform library if needed and calls the
 * appropriate load function for you. Non-Vulkan loaders call the detection-
 * based gloamLoad* functions. Vulkan loaders handle all extension detection
 * and PFN loading internally.
 * Each Load function may be called multiple times (additive).
 */
int  gloamLoaderLoadGLXContext(GloamGLXContext *context, Display *display, int screen);
int  gloamLoaderLoadGLX(Display *display, int screen);
void gloamLoaderUnloadGLXContext(GloamGLXContext *context);
void gloamLoaderUnloadGLX(void);
void gloamLoaderResetGLXContext(GloamGLXContext *context);
void gloamLoaderResetGLX(void);

#ifdef __cplusplus
}
#endif

#endif /* GLOAM_PLATFORM_LINUX */

#endif /* GLOAM_GLX_H */
