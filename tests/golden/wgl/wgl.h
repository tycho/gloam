#ifndef GLOAM_WGL_H
#define GLOAM_WGL_H

#ifdef __wgl_h_
  #error WGL (wgl.h) header already included (API: wgl), remove previous include!
#endif
#define __wgl_h_ 1

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

#ifdef GLOAM_PLATFORM_WINDOWS

#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#include <windows.h>

#include <gloam/gl.h>

/* These are defined in wingdi.h and we're redefining them. */
#undef wglUseFontBitmaps
#undef wglUseFontOutlines

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
#define WGL_VERSION_1_0 1

/* ---- Extension compile-time guards ---------------------------------------
 * These mirror the definitions in standard glext.h/gl2ext.h/eglext.h so
 * that code guarded by e.g. #ifdef GL_ARB_draw_indirect compiles correctly
 * against this header.
 */
#define WGL_ARB_extensions_string 1
#define WGL_EXT_extensions_string 1

/* ---- Constants ----------------------------------------------------------- */
#define WGL_SWAP_MAIN_PLANE 0x00000001
#define WGL_SWAP_OVERLAY1 0x00000002
#define WGL_SWAP_OVERLAY2 0x00000004
#define WGL_SWAP_OVERLAY3 0x00000008
#define WGL_SWAP_OVERLAY4 0x00000010
#define WGL_SWAP_OVERLAY5 0x00000020
#define WGL_SWAP_OVERLAY6 0x00000040
#define WGL_SWAP_OVERLAY7 0x00000080
#define WGL_SWAP_OVERLAY8 0x00000100
#define WGL_SWAP_OVERLAY9 0x00000200
#define WGL_SWAP_OVERLAY10 0x00000400
#define WGL_SWAP_OVERLAY11 0x00000800
#define WGL_SWAP_OVERLAY12 0x00001000
#define WGL_SWAP_OVERLAY13 0x00002000
#define WGL_SWAP_OVERLAY14 0x00004000
#define WGL_SWAP_OVERLAY15 0x00008000
#define WGL_SWAP_UNDERLAY1 0x00010000
#define WGL_SWAP_UNDERLAY2 0x00020000
#define WGL_SWAP_UNDERLAY3 0x00040000
#define WGL_SWAP_UNDERLAY4 0x00080000
#define WGL_SWAP_UNDERLAY5 0x00100000
#define WGL_SWAP_UNDERLAY6 0x00200000
#define WGL_SWAP_UNDERLAY7 0x00400000
#define WGL_SWAP_UNDERLAY8 0x00800000
#define WGL_SWAP_UNDERLAY9 0x01000000
#define WGL_SWAP_UNDERLAY10 0x02000000
#define WGL_SWAP_UNDERLAY11 0x04000000
#define WGL_SWAP_UNDERLAY12 0x08000000
#define WGL_SWAP_UNDERLAY13 0x10000000
#define WGL_SWAP_UNDERLAY14 0x20000000
#define WGL_SWAP_UNDERLAY15 0x40000000
#define WGL_FONT_LINES 0
#define WGL_FONT_POLYGONS 1

/* ---- Types ----------------------------------------------------------------
 * Emitted in topological dependency order. Consecutive types sharing the
 * same platform guard are coalesced into a single #ifdef/#endif block.
 */
DECLARE_HANDLE(HPBUFFERARB);

DECLARE_HANDLE(HPBUFFEREXT);

DECLARE_HANDLE(HVIDEOOUTPUTDEVICENV);

DECLARE_HANDLE(HPVIDEODEV);

DECLARE_HANDLE(HPGPUNV);

DECLARE_HANDLE(HGPUNV);

DECLARE_HANDLE(HVIDEOINPUTDEVICENV);

struct _GPU_DEVICE {
    DWORD  cb;
    CHAR   DeviceName[32];
    CHAR   DeviceString[128];
    DWORD  Flags;
    RECT   rcVirtualScreen;
};

typedef struct _GPU_DEVICE GPU_DEVICE;

typedef struct _GPU_DEVICE *PGPU_DEVICE;


/* ---- PFN typedefs -------------------------------------------------------- */
typedef int (APIENTRYP PFNWGLCHOOSEPIXELFORMATPROC)(HDC hDc, const PIXELFORMATDESCRIPTOR * pPfd);
typedef int (APIENTRYP PFNWGLDESCRIBEPIXELFORMATPROC)(HDC hdc, int ipfd, UINT cjpfd, PIXELFORMATDESCRIPTOR * ppfd);
typedef UINT (APIENTRYP PFNWGLGETENHMETAFILEPIXELFORMATPROC)(HENHMETAFILE hemf, UINT cbBuffer, PIXELFORMATDESCRIPTOR * ppfd);
typedef int (APIENTRYP PFNWGLGETPIXELFORMATPROC)(HDC hdc);
typedef BOOL (APIENTRYP PFNWGLSETPIXELFORMATPROC)(HDC hdc, int ipfd, const PIXELFORMATDESCRIPTOR * ppfd);
typedef BOOL (APIENTRYP PFNWGLSWAPBUFFERSPROC)(HDC hdc);
typedef BOOL (APIENTRYP PFNWGLCOPYCONTEXTPROC)(HGLRC hglrcSrc, HGLRC hglrcDst, UINT mask);
typedef HGLRC (APIENTRYP PFNWGLCREATECONTEXTPROC)(HDC hDc);
typedef HGLRC (APIENTRYP PFNWGLCREATELAYERCONTEXTPROC)(HDC hDc, int level);
typedef BOOL (APIENTRYP PFNWGLDELETECONTEXTPROC)(HGLRC oldContext);
typedef BOOL (APIENTRYP PFNWGLDESCRIBELAYERPLANEPROC)(HDC hDc, int pixelFormat, int layerPlane, UINT nBytes, LAYERPLANEDESCRIPTOR * plpd);
typedef HGLRC (APIENTRYP PFNWGLGETCURRENTCONTEXTPROC)(void);
typedef HDC (APIENTRYP PFNWGLGETCURRENTDCPROC)(void);
typedef int (APIENTRYP PFNWGLGETLAYERPALETTEENTRIESPROC)(HDC hdc, int iLayerPlane, int iStart, int cEntries, COLORREF * pcr);
typedef PROC (APIENTRYP PFNWGLGETPROCADDRESSPROC)(LPCSTR lpszProc);
typedef BOOL (APIENTRYP PFNWGLMAKECURRENTPROC)(HDC hDc, HGLRC newContext);
typedef BOOL (APIENTRYP PFNWGLREALIZELAYERPALETTEPROC)(HDC hdc, int iLayerPlane, BOOL bRealize);
typedef int (APIENTRYP PFNWGLSETLAYERPALETTEENTRIESPROC)(HDC hdc, int iLayerPlane, int iStart, int cEntries, const COLORREF * pcr);
typedef BOOL (APIENTRYP PFNWGLSHARELISTSPROC)(HGLRC hrcSrvShare, HGLRC hrcSrvSource);
typedef BOOL (APIENTRYP PFNWGLSWAPLAYERBUFFERSPROC)(HDC hdc, UINT fuFlags);
typedef BOOL (APIENTRYP PFNWGLUSEFONTBITMAPSPROC)(HDC hDC, DWORD first, DWORD count, DWORD listBase);
typedef BOOL (APIENTRYP PFNWGLUSEFONTBITMAPSAPROC)(HDC hDC, DWORD first, DWORD count, DWORD listBase);
typedef BOOL (APIENTRYP PFNWGLUSEFONTBITMAPSWPROC)(HDC hDC, DWORD first, DWORD count, DWORD listBase);
typedef BOOL (APIENTRYP PFNWGLUSEFONTOUTLINESPROC)(HDC hDC, DWORD first, DWORD count, DWORD listBase, FLOAT deviation, FLOAT extrusion, int format, LPGLYPHMETRICSFLOAT lpgmf);
typedef BOOL (APIENTRYP PFNWGLUSEFONTOUTLINESAPROC)(HDC hDC, DWORD first, DWORD count, DWORD listBase, FLOAT deviation, FLOAT extrusion, int format, LPGLYPHMETRICSFLOAT lpgmf);
typedef BOOL (APIENTRYP PFNWGLUSEFONTOUTLINESWPROC)(HDC hDC, DWORD first, DWORD count, DWORD listBase, FLOAT deviation, FLOAT extrusion, int format, LPGLYPHMETRICSFLOAT lpgmf);
typedef const char * (APIENTRYP PFNWGLGETEXTENSIONSSTRINGARBPROC)(HDC hdc);
typedef const char * (APIENTRYP PFNWGLGETEXTENSIONSSTRINGEXTPROC)(void);


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
typedef struct GloamWGLContext {
    union {
        unsigned char featArray[1];
        struct {
        /*    0 */ unsigned char VERSION_1_0;
        };
    };

    union {
        unsigned char extArray[2];
        struct {
        /*    0 */ unsigned char ARB_extensions_string;
        /*    1 */ unsigned char EXT_extensions_string;
        };
    };

    union {
        void *pfnArray[28];
        struct {
        /*    0 */ PFNWGLCHOOSEPIXELFORMATPROC ChoosePixelFormat;
        /*    1 */ PFNWGLDESCRIBEPIXELFORMATPROC DescribePixelFormat;
        /*    2 */ PFNWGLGETENHMETAFILEPIXELFORMATPROC GetEnhMetaFilePixelFormat;
        /*    3 */ PFNWGLGETPIXELFORMATPROC GetPixelFormat;
        /*    4 */ PFNWGLSETPIXELFORMATPROC SetPixelFormat;
        /*    5 */ PFNWGLSWAPBUFFERSPROC SwapBuffers;
        /*    6 */ PFNWGLCOPYCONTEXTPROC CopyContext;
        /*    7 */ PFNWGLCREATECONTEXTPROC CreateContext;
        /*    8 */ PFNWGLCREATELAYERCONTEXTPROC CreateLayerContext;
        /*    9 */ PFNWGLDELETECONTEXTPROC DeleteContext;
        /*   10 */ PFNWGLDESCRIBELAYERPLANEPROC DescribeLayerPlane;
        /*   11 */ PFNWGLGETCURRENTCONTEXTPROC GetCurrentContext;
        /*   12 */ PFNWGLGETCURRENTDCPROC GetCurrentDC;
        /*   13 */ PFNWGLGETLAYERPALETTEENTRIESPROC GetLayerPaletteEntries;
        /*   14 */ PFNWGLGETPROCADDRESSPROC GetProcAddress;
        /*   15 */ PFNWGLMAKECURRENTPROC MakeCurrent;
        /*   16 */ PFNWGLREALIZELAYERPALETTEPROC RealizeLayerPalette;
        /*   17 */ PFNWGLSETLAYERPALETTEENTRIESPROC SetLayerPaletteEntries;
        /*   18 */ PFNWGLSHARELISTSPROC ShareLists;
        /*   19 */ PFNWGLSWAPLAYERBUFFERSPROC SwapLayerBuffers;
        /*   20 */ PFNWGLUSEFONTBITMAPSPROC UseFontBitmaps;
        /*   21 */ PFNWGLUSEFONTBITMAPSAPROC UseFontBitmapsA;
        /*   22 */ PFNWGLUSEFONTBITMAPSWPROC UseFontBitmapsW;
        /*   23 */ PFNWGLUSEFONTOUTLINESPROC UseFontOutlines;
        /*   24 */ PFNWGLUSEFONTOUTLINESAPROC UseFontOutlinesA;
        /*   25 */ PFNWGLUSEFONTOUTLINESWPROC UseFontOutlinesW;
        /*   26 */ PFNWGLGETEXTENSIONSSTRINGARBPROC GetExtensionsStringARB;
        /*   27 */ PFNWGLGETEXTENSIONSSTRINGEXTPROC GetExtensionsStringEXT;
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
} GloamWGLContext;

/* Global context instance — a value, not a pointer, so the compiler knows
 * its address is fixed and does not re-load it on every access.
 */
extern GloamWGLContext gloam_wgl_context;

/* ---- Feature presence macros --------------------------------------------
 * Test whether a versioned feature was detected at load time.
 */
#define GLOAM_WGL_VERSION_1_0 (gloam_wgl_context.VERSION_1_0)

/* ---- Extension presence macros ------------------------------------------ */
#define GLOAM_WGL_ARB_extensions_string (gloam_wgl_context.ARB_extensions_string)
#define GLOAM_WGL_EXT_extensions_string (gloam_wgl_context.EXT_extensions_string)


/* ---- Dispatch ------------------------------------------------------------ */

#ifdef __INTELLISENSE__
int ChoosePixelFormat(HDC hDc, const PIXELFORMATDESCRIPTOR * pPfd);
int DescribePixelFormat(HDC hdc, int ipfd, UINT cjpfd, PIXELFORMATDESCRIPTOR * ppfd);
UINT GetEnhMetaFilePixelFormat(HENHMETAFILE hemf, UINT cbBuffer, PIXELFORMATDESCRIPTOR * ppfd);
int GetPixelFormat(HDC hdc);
BOOL SetPixelFormat(HDC hdc, int ipfd, const PIXELFORMATDESCRIPTOR * ppfd);
BOOL SwapBuffers(HDC hdc);
BOOL wglCopyContext(HGLRC hglrcSrc, HGLRC hglrcDst, UINT mask);
HGLRC wglCreateContext(HDC hDc);
HGLRC wglCreateLayerContext(HDC hDc, int level);
BOOL wglDeleteContext(HGLRC oldContext);
BOOL wglDescribeLayerPlane(HDC hDc, int pixelFormat, int layerPlane, UINT nBytes, LAYERPLANEDESCRIPTOR * plpd);
HGLRC wglGetCurrentContext(void);
HDC wglGetCurrentDC(void);
int wglGetLayerPaletteEntries(HDC hdc, int iLayerPlane, int iStart, int cEntries, COLORREF * pcr);
PROC wglGetProcAddress(LPCSTR lpszProc);
BOOL wglMakeCurrent(HDC hDc, HGLRC newContext);
BOOL wglRealizeLayerPalette(HDC hdc, int iLayerPlane, BOOL bRealize);
int wglSetLayerPaletteEntries(HDC hdc, int iLayerPlane, int iStart, int cEntries, const COLORREF * pcr);
BOOL wglShareLists(HGLRC hrcSrvShare, HGLRC hrcSrvSource);
BOOL wglSwapLayerBuffers(HDC hdc, UINT fuFlags);
BOOL wglUseFontBitmaps(HDC hDC, DWORD first, DWORD count, DWORD listBase);
BOOL wglUseFontBitmapsA(HDC hDC, DWORD first, DWORD count, DWORD listBase);
BOOL wglUseFontBitmapsW(HDC hDC, DWORD first, DWORD count, DWORD listBase);
BOOL wglUseFontOutlines(HDC hDC, DWORD first, DWORD count, DWORD listBase, FLOAT deviation, FLOAT extrusion, int format, LPGLYPHMETRICSFLOAT lpgmf);
BOOL wglUseFontOutlinesA(HDC hDC, DWORD first, DWORD count, DWORD listBase, FLOAT deviation, FLOAT extrusion, int format, LPGLYPHMETRICSFLOAT lpgmf);
BOOL wglUseFontOutlinesW(HDC hDC, DWORD first, DWORD count, DWORD listBase, FLOAT deviation, FLOAT extrusion, int format, LPGLYPHMETRICSFLOAT lpgmf);
const char * wglGetExtensionsStringARB(HDC hdc);
const char * wglGetExtensionsStringEXT(void);
#else
#define ChoosePixelFormat (gloam_wgl_context.ChoosePixelFormat)
#define DescribePixelFormat (gloam_wgl_context.DescribePixelFormat)
#define GetEnhMetaFilePixelFormat (gloam_wgl_context.GetEnhMetaFilePixelFormat)
#define GetPixelFormat (gloam_wgl_context.GetPixelFormat)
#define SetPixelFormat (gloam_wgl_context.SetPixelFormat)
#define SwapBuffers (gloam_wgl_context.SwapBuffers)
#define wglCopyContext (gloam_wgl_context.CopyContext)
#define wglCreateContext (gloam_wgl_context.CreateContext)
#define wglCreateLayerContext (gloam_wgl_context.CreateLayerContext)
#define wglDeleteContext (gloam_wgl_context.DeleteContext)
#define wglDescribeLayerPlane (gloam_wgl_context.DescribeLayerPlane)
#define wglGetCurrentContext (gloam_wgl_context.GetCurrentContext)
#define wglGetCurrentDC (gloam_wgl_context.GetCurrentDC)
#define wglGetLayerPaletteEntries (gloam_wgl_context.GetLayerPaletteEntries)
#define wglGetProcAddress (gloam_wgl_context.GetProcAddress)
#define wglMakeCurrent (gloam_wgl_context.MakeCurrent)
#define wglRealizeLayerPalette (gloam_wgl_context.RealizeLayerPalette)
#define wglSetLayerPaletteEntries (gloam_wgl_context.SetLayerPaletteEntries)
#define wglShareLists (gloam_wgl_context.ShareLists)
#define wglSwapLayerBuffers (gloam_wgl_context.SwapLayerBuffers)
#define wglUseFontBitmaps (gloam_wgl_context.UseFontBitmaps)
#define wglUseFontBitmapsA (gloam_wgl_context.UseFontBitmapsA)
#define wglUseFontBitmapsW (gloam_wgl_context.UseFontBitmapsW)
#define wglUseFontOutlines (gloam_wgl_context.UseFontOutlines)
#define wglUseFontOutlinesA (gloam_wgl_context.UseFontOutlinesA)
#define wglUseFontOutlinesW (gloam_wgl_context.UseFontOutlinesW)
#define wglGetExtensionsStringARB (gloam_wgl_context.GetExtensionsStringARB)
#define wglGetExtensionsStringEXT (gloam_wgl_context.GetExtensionsStringEXT)
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

int gloamLoadWGLContext(GloamWGLContext *context, HDC hdc, GloamLoadFunc getProcAddr);
int gloamLoadWGL(HDC hdc, GloamLoadFunc getProcAddr);
/* Built-in loader: opens the platform library if needed and calls the
 * appropriate load function for you. Non-Vulkan loaders call the detection-
 * based gloamLoad* functions. Vulkan loaders handle all extension detection
 * and PFN loading internally.
 * Each Load function may be called multiple times (additive).
 */
int  gloamLoaderLoadWGLContext(GloamWGLContext *context, HDC hdc);
int  gloamLoaderLoadWGL(HDC hdc);
void gloamLoaderUnloadWGLContext(GloamWGLContext *context);
void gloamLoaderUnloadWGL(void);
void gloamLoaderResetWGLContext(GloamWGLContext *context);
void gloamLoaderResetWGL(void);

#ifdef __cplusplus
}
#endif

#endif /* GLOAM_PLATFORM_WINDOWS */

#endif /* GLOAM_WGL_H */
