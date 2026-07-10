#include <gloam/glx.h>

#ifdef GLOAM_PLATFORM_LINUX
#if defined(__CYGWIN__) || defined(_WIN32)
#  ifndef WIN32_LEAN_AND_MEAN
#    define WIN32_LEAN_AND_MEAN
#  endif
#  undef APIENTRY       /* fix macro redefinition warning */
#  include <windows.h>  /* LoadLibrary, GetProcAddress */
#else
#  include <dlfcn.h> /* dlopen, dlsym, dlclose */
#endif

#include <stdlib.h>  /* calloc, free    */
#include <stddef.h>
#include <stdio.h>   /* sscanf          */
#include <string.h>  /* strlen, strncmp */


#ifndef GLOAM_IMPL_UTIL_C_
#define GLOAM_IMPL_UTIL_C_

/* MSVC vs GCC/Clang sscanf */
#ifdef _MSC_VER
#  define GLOAM_IMPL_UTIL_SSCANF sscanf_s
#else
#  define GLOAM_IMPL_UTIL_SSCANF sscanf
#endif

/* GLOAM_NO_INLINE — suppress inlining on functions that should stay out of
 * the hot path (sort, hash, etc.) to avoid code bloat at call sites.
 */
#ifdef _MSC_VER
#  define GLOAM_NO_INLINE __declspec(noinline)
#else
#  define GLOAM_NO_INLINE __attribute__((noinline))
#endif

#define GLOAM_ARRAYSIZE(x) (sizeof(x) / sizeof((x)[0]))
#define GLOAM_UNUSED(x)    ((void)(x))

/* Contiguous run of pfnArray slots belonging to one feature or extension.
 * Used by the range-based PFN loading loop.
 */
typedef struct {
    uint16_t extension; /* index into featArray or extArray  */
    uint16_t start;     /* first pfnArray index in this run  */
    uint16_t count;     /* number of consecutive slots       */
} GloamPfnRange_t;

#endif /* GLOAM_IMPL_UTIL_C_ */



/* ---- Global context (zero-initialised at program startup) ---------------- */
#ifdef __cplusplus
GloamGLXContext gloam_glx_context = {};
#else
GloamGLXContext gloam_glx_context = { 0 };
#endif

/* ---- Function name table -------------------------------------------------
 * Command names stored as a single NUL-terminated string blob with a parallel
 * offset table for O(1) indexing. This avoids one pointer (8 bytes on 64-bit)
 * plus one relocation entry (~24 bytes in PIC builds) per command compared to
 * the traditional const char * const [] approach.
 */
static const uint32_t kFnCount_GLX = 39;

static const char kFnNameData_GLX[] =
    /*     0 */ "glXChooseVisual\0"
    /*    16 */ "glXCopyContext\0"
    /*    31 */ "glXCreateContext\0"
    /*    48 */ "glXCreateGLXPixmap\0"
    /*    67 */ "glXDestroyContext\0"
    /*    85 */ "glXDestroyGLXPixmap\0"
    /*   105 */ "glXGetConfig\0"
    /*   118 */ "glXGetCurrentContext\0"
    /*   139 */ "glXGetCurrentDrawable\0"
    /*   161 */ "glXIsDirect\0"
    /*   173 */ "glXMakeCurrent\0"
    /*   188 */ "glXQueryExtension\0"
    /*   206 */ "glXQueryVersion\0"
    /*   222 */ "glXSwapBuffers\0"
    /*   237 */ "glXUseXFont\0"
    /*   249 */ "glXWaitGL\0"
    /*   259 */ "glXWaitX\0"
    /*   268 */ "glXGetClientString\0"
    /*   287 */ "glXQueryExtensionsString\0"
    /*   312 */ "glXQueryServerString\0"
    /*   333 */ "glXGetCurrentDisplay\0"
    /*   354 */ "glXChooseFBConfig\0"
    /*   372 */ "glXCreateNewContext\0"
    /*   392 */ "glXCreatePbuffer\0"
    /*   409 */ "glXCreatePixmap\0"
    /*   425 */ "glXCreateWindow\0"
    /*   441 */ "glXDestroyPbuffer\0"
    /*   459 */ "glXDestroyPixmap\0"
    /*   476 */ "glXDestroyWindow\0"
    /*   493 */ "glXGetCurrentReadDrawable\0"
    /*   519 */ "glXGetFBConfigAttrib\0"
    /*   540 */ "glXGetFBConfigs\0"
    /*   556 */ "glXGetSelectedEvent\0"
    /*   576 */ "glXGetVisualFromFBConfig\0"
    /*   601 */ "glXMakeContextCurrent\0"
    /*   623 */ "glXQueryContext\0"
    /*   639 */ "glXQueryDrawable\0"
    /*   656 */ "glXSelectEvent\0"
    /*   671 */ "glXGetProcAddress\0"
;
static const uint16_t kFnNameOffsets_GLX[] = {
    /*    0 */     0, /* glXChooseVisual */
    /*    1 */    16, /* glXCopyContext */
    /*    2 */    31, /* glXCreateContext */
    /*    3 */    48, /* glXCreateGLXPixmap */
    /*    4 */    67, /* glXDestroyContext */
    /*    5 */    85, /* glXDestroyGLXPixmap */
    /*    6 */   105, /* glXGetConfig */
    /*    7 */   118, /* glXGetCurrentContext */
    /*    8 */   139, /* glXGetCurrentDrawable */
    /*    9 */   161, /* glXIsDirect */
    /*   10 */   173, /* glXMakeCurrent */
    /*   11 */   188, /* glXQueryExtension */
    /*   12 */   206, /* glXQueryVersion */
    /*   13 */   222, /* glXSwapBuffers */
    /*   14 */   237, /* glXUseXFont */
    /*   15 */   249, /* glXWaitGL */
    /*   16 */   259, /* glXWaitX */
    /*   17 */   268, /* glXGetClientString */
    /*   18 */   287, /* glXQueryExtensionsString */
    /*   19 */   312, /* glXQueryServerString */
    /*   20 */   333, /* glXGetCurrentDisplay */
    /*   21 */   354, /* glXChooseFBConfig */
    /*   22 */   372, /* glXCreateNewContext */
    /*   23 */   392, /* glXCreatePbuffer */
    /*   24 */   409, /* glXCreatePixmap */
    /*   25 */   425, /* glXCreateWindow */
    /*   26 */   441, /* glXDestroyPbuffer */
    /*   27 */   459, /* glXDestroyPixmap */
    /*   28 */   476, /* glXDestroyWindow */
    /*   29 */   493, /* glXGetCurrentReadDrawable */
    /*   30 */   519, /* glXGetFBConfigAttrib */
    /*   31 */   540, /* glXGetFBConfigs */
    /*   32 */   556, /* glXGetSelectedEvent */
    /*   33 */   576, /* glXGetVisualFromFBConfig */
    /*   34 */   601, /* glXMakeContextCurrent */
    /*   35 */   623, /* glXQueryContext */
    /*   36 */   639, /* glXQueryDrawable */
    /*   37 */   656, /* glXSelectEvent */
    /*   38 */   671 /* glXGetProcAddress */
};

/* ---- Feature PFN range table ---------------------------------------------
 * Each entry maps one feature (by featArray index) to a contiguous run of
 * pfnArray slots. The loader iterates this table and bulk-loads the run
 * when featArray[entry.extension] is set.
 */
static const GloamPfnRange_t kFeatPfnRanges_GLX[] = {
    {    0,    0,   17 }, /* GLX_VERSION_1_0 */
    {    1,   17,    3 }, /* GLX_VERSION_1_1 */
    {    2,   20,    1 }, /* GLX_VERSION_1_2 */
    {    3,   21,   17 }, /* GLX_VERSION_1_3 */
    {    4,   38,    1 }, /* GLX_VERSION_1_4 */
};

/* ---- PFN range helper (GL / EGL / GLX / WGL) ----------------------------
 * Walks a contiguous run of pfnArray slots and calls the plain load callback
 * (name only) for each one.
 */
static void gloam_load_pfn_range_glx(GloamGLXContext *context, GloamLoadFunc getProcAddr, uint16_t start, uint16_t count)
{
    uint16_t i;
    for (i = start; i < (uint16_t)(start + count); ++i) {
        const char *pfnName = &kFnNameData_GLX[kFnNameOffsets_GLX[i]];
        context->pfnArray[i] = (void *)getProcAddr(pfnName);
    }
}


/* ==========================================================================
 * Per-API sections
 * ==========================================================================
 */

/* --------------------------------------------------------------------------
 * API: glx
 * --------------------------------------------------------------------------
 */
/* Query the GLX version via glXQueryVersion and set featArray entries for GLX.
 * Also returns the packed version (major << 8 | minor) for loader use.
 */
static int gloam_glx_find_core_glx(GloamGLXContext *context, Display **display, int *screen)
{
    int major = 0, minor = 0;
    unsigned short version_value;
    if(*display == NULL) {
        *display = XOpenDisplay(0);
        if (*display == NULL) {
            return 0;
        }
        *screen = XScreenNumberOfScreen(XDefaultScreenOfDisplay(*display));
    }
    context->QueryVersion(*display, &major, &minor);
    version_value = (major << 8U) | minor;
    context->VERSION_1_0 = (unsigned char)(version_value >= 0x0100);
    context->VERSION_1_1 = (unsigned char)(version_value >= 0x0101);
    context->VERSION_1_2 = (unsigned char)(version_value >= 0x0102);
    context->VERSION_1_3 = (unsigned char)(version_value >= 0x0103);
    context->VERSION_1_4 = (unsigned char)(version_value >= 0x0104);
    return version_value;
}
int gloamLoadGLXContext(GloamGLXContext *context, Display *display, int screen, GloamLoadFunc getProcAddr)
{
    int version;
    uint32_t i;
    memset(context, 0, sizeof(*context));

    context->QueryVersion = (PFNGLXQUERYVERSIONPROC)getProcAddr("glXQueryVersion");

    version = gloam_glx_find_core_glx(context, &display, &screen);
    if (!version)
        return 0;

    /* Load all PFNs upfront. */
    for (i = 0; i < kFnCount_GLX; ++i)
        context->pfnArray[i] = (void *)getProcAddr((kFnNameData_GLX + kFnNameOffsets_GLX[i]));

    /* Mark features based on PFN availability. */
    for (i = 0; i < GLOAM_ARRAYSIZE(kFeatPfnRanges_GLX); ++i) {
        const GloamPfnRange_t *r = &kFeatPfnRanges_GLX[i];
        uint16_t j; int ok = 1;
        for (j = r->start; j < (uint16_t)(r->start + r->count); ++j)
            ok &= (context->pfnArray[j] != NULL);
        if (ok)
            context->featArray[r->extension] = 1;
    }

    return 1;
}

int gloamLoadGLX(Display *display, int screen, GloamLoadFunc getProcAddr)
{
    return gloamLoadGLXContext(&gloam_glx_context, display, screen, getProcAddr);
}

/* ==========================================================================
 * Built-in loader (--loader)
 * ==========================================================================
 */
#ifndef GLOAM_LOADER_LIBRARY_C_
#define GLOAM_LOADER_LIBRARY_C_

#if defined(GLOAM_PLATFORM_WINDOWS)

static void *gloam_dlopen(const char *name)
{
    return (void *)LoadLibraryA(name);
}
static void gloam_dlclose(void *handle)
{
    FreeLibrary((HMODULE)handle);
}
static void *gloam_dlsym(void *handle, const char *name)
{
    return (void *)GetProcAddress((HMODULE)handle, name);
}

#else /* POSIX */

static void *gloam_dlopen(const char *name)
{
    return dlopen(name, RTLD_LAZY | RTLD_LOCAL);
}
static void gloam_dlclose(void *handle)
{
    dlclose(handle);
}
static void *gloam_dlsym(void *handle, const char *name)
{
    return dlsym(handle, name);
}

#endif /* GLOAM_PLATFORM_WINDOWS */

/* Try each name in turn; return the first handle that opens successfully. */
static void *gloam_open_library(const char * const *names, int count)
{
    int i;
    for (i = 0; i < count; ++i) {
        void *h = gloam_dlopen(names[i]);
        if (h) return h;
    }
    return NULL;
}

#endif /* GLOAM_LOADER_LIBRARY_C_ */


/* ---- GLX built-in loader ------------------------------------------------ */

static const char * const gloam_glx_lib_names[] = {
#if defined(__CYGWIN__)
    "libGL-1.so",
#endif
    "libGL.so.1", "libGL.so",
};

/* glXGetProcAddressARB returns __GLXextFuncPtr = void(*)(void); store with the
 * correct function-pointer type so the call goes through the right ABI.
 */
struct gloam_glx_load_userptr {
    void *handle;
    GloamAPIProc (*get_proc_address)(const char *);
};
static struct gloam_glx_load_userptr gloam_glx_load_state;

static GloamAPIProc gloam_glx_get_proc(const char *name)
{
    struct gloam_glx_load_userptr *u = &gloam_glx_load_state;
    GloamAPIProc result = NULL;
    if (u->get_proc_address)
        result = u->get_proc_address(name);
    if (!result)
        result = (GloamAPIProc)gloam_dlsym(u->handle, name);
    return result;
}

int gloamLoaderLoadGLXContext(GloamGLXContext *context, Display *display, int screen)
{
    int did_open = 0, version;
    void *handle;

    handle = context->gloam_loader_handle;

    if (!handle) {
        handle = gloam_open_library(gloam_glx_lib_names, GLOAM_ARRAYSIZE(gloam_glx_lib_names));
        did_open = 1;
    }

    if (!handle)
        return 0;

    gloam_glx_load_state.handle = handle;
    gloam_glx_load_state.get_proc_address = (GloamAPIProc (*)(const char *))gloam_dlsym(handle, "glXGetProcAddressARB");

    if (!gloam_glx_load_state.get_proc_address) {
        if (did_open)
            gloam_dlclose(handle);
        return 0;
    }

    version = gloamLoadGLXContext(context, display, screen, gloam_glx_get_proc);
    gloam_glx_load_state.handle = NULL;

    if (!version && did_open) {
        gloam_dlclose(handle);
        return 0;
    }

    context->gloam_loader_handle = handle;
    context->gloam_loader_owns_handle |= (uint8_t)did_open;

    return version;
}

int gloamLoaderLoadGLX(Display *display, int screen)
{
    return gloamLoaderLoadGLXContext(&gloam_glx_context, display, screen);
}

void gloamLoaderUnloadGLXContext(GloamGLXContext *context)
{
    if (context->gloam_loader_handle && context->gloam_loader_owns_handle) {
        gloam_dlclose(context->gloam_loader_handle);
    }
    gloamLoaderResetGLXContext(context);
}

void gloamLoaderUnloadGLX(void)
{
    gloamLoaderUnloadGLXContext(&gloam_glx_context);
}

void gloamLoaderResetGLXContext(GloamGLXContext *context)
{
    memset(context, 0, sizeof(*context));
}

void gloamLoaderResetGLX(void)
{
    gloamLoaderResetGLXContext(&gloam_glx_context);
}

#endif /* GLOAM_PLATFORM_LINUX */
