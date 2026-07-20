#include <gloam/egl.h>

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

#if defined(__x86_64__) || defined(__i386__) || defined(_M_IX86) || defined(_M_X64)
#  ifndef XXH_VECTOR
#    define XXH_VECTOR XXH_SSE2
#  endif
#  include <immintrin.h>
#elif defined(__aarch64__) || defined(__arm__) || defined(_M_ARM) || defined(_M_ARM64)
#  ifndef XXH_VECTOR
#    define XXH_VECTOR XXH_NEON
#  endif
#  include <arm_neon.h>
#endif
#ifndef GLOAM_EXTERNAL_XXHASH
#  define XXH_INLINE_ALL
#  define XXH_NO_STREAM
#endif
#include "xxhash.h"

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


#ifndef GLOAM_IMPL_HASHSEARCH_C_
#define GLOAM_IMPL_HASHSEARCH_C_

/* gloam_sort_hashes — in-place Shellsort on a uint64_t array.
 *
 * Ciura (2001) gap sequence.  Gaps larger than n are skipped at runtime so
 * small arrays (< 10 extensions) take only a couple of passes.  No heap
 * allocation; code size is ~80 bytes on x86-64.
 */
GLOAM_NO_INLINE static void gloam_sort_hashes(uint64_t *a, size_t n)
{
    static const size_t kGaps[] = { 701, 301, 132, 57, 23, 10, 4, 1 };
    size_t gi = 0;
    if (!a || n < 2) return;
    /* Skip gaps that are larger than the array. */
    while (gi < GLOAM_ARRAYSIZE(kGaps) && kGaps[gi] >= n) ++gi;
    for (; gi < GLOAM_ARRAYSIZE(kGaps); ++gi) {
        size_t gap = kGaps[gi], i;
        for (i = gap; i < n; ++i) {
            uint64_t v = a[i];
            size_t j = i;
            while (j >= gap && a[j - gap] > v) {
                a[j] = a[j - gap];
                j -= gap;
            }
            a[j] = v;
        }
    }
}

/* gloam_hash_search — binary search for `target` in a sorted uint64_t array.
 * Returns 1 if found, 0 otherwise.
 */
GLOAM_NO_INLINE static int gloam_hash_search(const uint64_t *arr, uint32_t size, uint64_t target)
{
    int32_t lo = 0, hi = (int32_t)size - 1;
    while (lo <= hi) {
        int32_t mid = lo + (hi - lo) / 2;
        if (arr[mid] == target) return 1;
        if (arr[mid] < target)  lo = mid + 1;
        else                    hi = mid - 1;
    }
    return 0;
}

/* gloam_hash_string — hash a NUL-terminated string with XXH3-64.
 * The same algorithm is used at generator time to pre-bake kExtHashes[],
 * guaranteeing that driver-reported names and the embedded table match.
 */
GLOAM_NO_INLINE static uint64_t gloam_hash_string(const char *str, size_t length)
{
    return XXH3_64bits(str, length);
}

/* ---- Extension string tokenizer ------------------------------------------
   Two-pass tokenize-and-hash for space-separated extension strings (GL, EGL,
   GLX, WGL).  First pass counts tokens, second pass hashes them.  Result is
   sorted for binary search in find_extensions. */
GLOAM_NO_INLINE static int gloam_hash_ext_string(const char *ext_str, uint64_t **out_exts, uint32_t *out_num_exts)
{
    const char *cur, *next;
    uint64_t *exts = NULL;
    uint32_t num_exts = 0, j;

    for (j = 0; j < 2; ++j) {
        num_exts = 0;
        cur  = ext_str;
        next = cur + strcspn(cur, " ");
        while (1) {
            size_t len;
            cur += strspn(cur, " ");
            if (!cur[0])
                break;
            len = (size_t)(next - cur);
            if (exts)
                exts[num_exts] = gloam_hash_string(cur, len);
            ++num_exts;
            cur  = next + strspn(next, " ");
            next = cur  + strcspn(cur,  " ");
        }
        if (!exts) {
            exts = (uint64_t *)calloc(num_exts, sizeof(uint64_t));
            if (!exts)
                return 0;
        }
    }

    gloam_sort_hashes(exts, num_exts);
    *out_exts     = exts;
    *out_num_exts = num_exts;
    return 1;
}
#endif /* GLOAM_IMPL_HASHSEARCH_C_ */


/* ---- Global context (zero-initialised at program startup) ---------------- */
#ifdef __cplusplus
GloamEGLContext gloam_egl_context = {};
#else
GloamEGLContext gloam_egl_context = { 0 };
#endif

/* ---- Function name table -------------------------------------------------
 * Command names stored as a single NUL-terminated string blob with a parallel
 * offset table for O(1) indexing. This avoids one pointer (8 bytes on 64-bit)
 * plus one relocation entry (~24 bytes in PIC builds) per command compared to
 * the traditional const char * const [] approach.
 */
static const uint32_t kFnCount_EGL = 47;

static const char kFnNameData_EGL[] =
    /*     0 */ "eglChooseConfig\0"
    /*    16 */ "eglCopyBuffers\0"
    /*    31 */ "eglCreateContext\0"
    /*    48 */ "eglCreatePbufferSurface\0"
    /*    72 */ "eglCreatePixmapSurface\0"
    /*    95 */ "eglCreateWindowSurface\0"
    /*   118 */ "eglDestroyContext\0"
    /*   136 */ "eglDestroySurface\0"
    /*   154 */ "eglGetConfigAttrib\0"
    /*   173 */ "eglGetConfigs\0"
    /*   187 */ "eglGetCurrentDisplay\0"
    /*   208 */ "eglGetCurrentSurface\0"
    /*   229 */ "eglGetDisplay\0"
    /*   243 */ "eglGetError\0"
    /*   255 */ "eglGetProcAddress\0"
    /*   273 */ "eglInitialize\0"
    /*   287 */ "eglMakeCurrent\0"
    /*   302 */ "eglQueryContext\0"
    /*   318 */ "eglQueryString\0"
    /*   333 */ "eglQuerySurface\0"
    /*   349 */ "eglSwapBuffers\0"
    /*   364 */ "eglTerminate\0"
    /*   377 */ "eglWaitGL\0"
    /*   387 */ "eglWaitNative\0"
    /*   401 */ "eglBindTexImage\0"
    /*   417 */ "eglReleaseTexImage\0"
    /*   436 */ "eglSurfaceAttrib\0"
    /*   453 */ "eglSwapInterval\0"
    /*   469 */ "eglBindAPI\0"
    /*   480 */ "eglCreatePbufferFromClientBuffer\0"
    /*   513 */ "eglQueryAPI\0"
    /*   525 */ "eglReleaseThread\0"
    /*   542 */ "eglWaitClient\0"
    /*   556 */ "eglGetCurrentContext\0"
    /*   577 */ "eglClientWaitSync\0"
    /*   595 */ "eglCreateImage\0"
    /*   610 */ "eglCreatePlatformPixmapSurface\0"
    /*   641 */ "eglCreatePlatformWindowSurface\0"
    /*   672 */ "eglCreateSync\0"
    /*   686 */ "eglDestroyImage\0"
    /*   702 */ "eglDestroySync\0"
    /*   717 */ "eglGetPlatformDisplay\0"
    /*   739 */ "eglGetSyncAttrib\0"
    /*   756 */ "eglWaitSync\0"
    /*   768 */ "eglDebugMessageControlKHR\0"
    /*   794 */ "eglLabelObjectKHR\0"
    /*   812 */ "eglQueryDebugKHR\0"
;
static const uint16_t kFnNameOffsets_EGL[] = {
    /*    0 */     0, /* eglChooseConfig */
    /*    1 */    16, /* eglCopyBuffers */
    /*    2 */    31, /* eglCreateContext */
    /*    3 */    48, /* eglCreatePbufferSurface */
    /*    4 */    72, /* eglCreatePixmapSurface */
    /*    5 */    95, /* eglCreateWindowSurface */
    /*    6 */   118, /* eglDestroyContext */
    /*    7 */   136, /* eglDestroySurface */
    /*    8 */   154, /* eglGetConfigAttrib */
    /*    9 */   173, /* eglGetConfigs */
    /*   10 */   187, /* eglGetCurrentDisplay */
    /*   11 */   208, /* eglGetCurrentSurface */
    /*   12 */   229, /* eglGetDisplay */
    /*   13 */   243, /* eglGetError */
    /*   14 */   255, /* eglGetProcAddress */
    /*   15 */   273, /* eglInitialize */
    /*   16 */   287, /* eglMakeCurrent */
    /*   17 */   302, /* eglQueryContext */
    /*   18 */   318, /* eglQueryString */
    /*   19 */   333, /* eglQuerySurface */
    /*   20 */   349, /* eglSwapBuffers */
    /*   21 */   364, /* eglTerminate */
    /*   22 */   377, /* eglWaitGL */
    /*   23 */   387, /* eglWaitNative */
    /*   24 */   401, /* eglBindTexImage */
    /*   25 */   417, /* eglReleaseTexImage */
    /*   26 */   436, /* eglSurfaceAttrib */
    /*   27 */   453, /* eglSwapInterval */
    /*   28 */   469, /* eglBindAPI */
    /*   29 */   480, /* eglCreatePbufferFromClientBuffer */
    /*   30 */   513, /* eglQueryAPI */
    /*   31 */   525, /* eglReleaseThread */
    /*   32 */   542, /* eglWaitClient */
    /*   33 */   556, /* eglGetCurrentContext */
    /*   34 */   577, /* eglClientWaitSync */
    /*   35 */   595, /* eglCreateImage */
    /*   36 */   610, /* eglCreatePlatformPixmapSurface */
    /*   37 */   641, /* eglCreatePlatformWindowSurface */
    /*   38 */   672, /* eglCreateSync */
    /*   39 */   686, /* eglDestroyImage */
    /*   40 */   702, /* eglDestroySync */
    /*   41 */   717, /* eglGetPlatformDisplay */
    /*   42 */   739, /* eglGetSyncAttrib */
    /*   43 */   756, /* eglWaitSync */
    /*   44 */   768, /* eglDebugMessageControlKHR */
    /*   45 */   794, /* eglLabelObjectKHR */
    /*   46 */   812 /* eglQueryDebugKHR */
};
/* ---- Extension hash table ------------------------------------------------
   One XXH3-64 hash per extension, in extArray index order.
   Pre-baked at generator time with the same algorithm used at load time. */
static const uint64_t kExtHashes_EGL[] = {
    /*    0 */ 0x5b61d2012f7861b3ULL  /* EGL_KHR_debug */
};

/* ---- Feature PFN range table ---------------------------------------------
 * Each entry maps one feature (by featArray index) to a contiguous run of
 * pfnArray slots. The loader iterates this table and bulk-loads the run
 * when featArray[entry.extension] is set.
 */
static const GloamPfnRange_t kFeatPfnRanges_EGL[] = {
    {    0,    0,   24 }, /* EGL_VERSION_1_0 */
    {    1,   24,    4 }, /* EGL_VERSION_1_1 */
    {    2,   28,    5 }, /* EGL_VERSION_1_2 */
    {    4,   33,    1 }, /* EGL_VERSION_1_4 */
    {    5,   34,   10 }, /* EGL_VERSION_1_5 */
};

/* ---- PFN range helper (GL / EGL / GLX / WGL) ----------------------------
 * Walks a contiguous run of pfnArray slots and calls the plain load callback
 * (name only) for each one.
 */
static void gloam_load_pfn_range_egl(GloamEGLContext *context, GloamLoadFunc getProcAddr, uint16_t start, uint16_t count)
{
    uint16_t i;
    for (i = start; i < (uint16_t)(start + count); ++i) {
        const char *pfnName = &kFnNameData_EGL[kFnNameOffsets_EGL[i]];
        context->pfnArray[i] = (void *)getProcAddr(pfnName);
    }
}


/* ==========================================================================
 * Driver extension query (shared across per-API sections)
 * ==========================================================================
 */

/* EGL: concatenate client extensions (EGL_NO_DISPLAY) and display
 * extensions, then hash the combined space-separated list.
 */
static int gloam_egl_get_extensions(GloamEGLContext *context, EGLDisplay display, uint64_t **out_exts, uint32_t *out_num_exts)
{
    const char *client_str, *display_str;
    char *concat = NULL;
    size_t client_len, display_len;
    int result;

    if (!context->QueryString)
        return 0;

    /* Client extensions live at EGL_NO_DISPLAY. */
    client_str  = (const char *)context->QueryString(EGL_NO_DISPLAY, EGL_EXTENSIONS);
    display_str = (display == EGL_NO_DISPLAY) ? "" :
                  (const char *)context->QueryString(display, EGL_EXTENSIONS);

    if (!client_str)
        return 0;
    if (!display_str)
        return 0;

    client_len  = strlen(client_str);
    display_len = strlen(display_str);

    /* Concatenate with a space separator. */
    concat = (char *)malloc(client_len + display_len + 2);
    if (!concat)
        return 0;
    memcpy(concat, client_str, client_len);
    size_t pos = client_len;
    if (display_len) {
        if (client_len && client_str[client_len - 1] != ' ')
            concat[pos++] = ' ';
        memcpy(concat + pos, display_str, display_len);
        pos += display_len;
    }
    concat[pos] = '\0';

    result = gloam_hash_ext_string(concat, out_exts, out_num_exts);
    free(concat);
    return result;
}

/* ==========================================================================
 * Per-API sections
 * ==========================================================================
 */

/* --------------------------------------------------------------------------
 * API: egl
 * --------------------------------------------------------------------------
 */

/* Extension index subset for egl: extArray indices this API supports. */
static const uint16_t kExtIdx_egl[] = {
       0, /* EGL_KHR_debug */
};

/* Extension PFN range table for egl. */
static const GloamPfnRange_t kExtPfnRanges_egl[] = {
    {    0,   44,    3 }, /* EGL_KHR_debug */
};

/* Search pre-baked kExtHashes_EGL against the sorted driver hash list and set
 * extArray flags for every matching extension.
 */
static int gloam_egl_find_extensions_egl(GloamEGLContext *context, EGLDisplay display)
{
    uint64_t *exts = NULL;
    uint32_t  num_exts = 0, i;

    if (!gloam_egl_get_extensions(context, display, &exts, &num_exts))
        return 0;

    for (i = 0; i < GLOAM_ARRAYSIZE(kExtIdx_egl); ++i) {
        const uint16_t extIdx = kExtIdx_egl[i];
        context->extArray[extIdx] = (unsigned char)gloam_hash_search(exts, num_exts, kExtHashes_EGL[extIdx]);
    }

    free(exts);
    return 1;
}

/* Parse the EGL version string from eglQueryString(display, EGL_VERSION). */
static int gloam_egl_find_core_egl(GloamEGLContext *context, EGLDisplay display)
{
    int major = 0, minor = 0;
    unsigned short version_value;
    const char *version;
    if (!context->QueryString)
        return 0;
    version = (const char *)context->QueryString(display, EGL_VERSION);
    if (!version)
        return 0;
    GLOAM_IMPL_UTIL_SSCANF(version, "%d.%d", &major, &minor);
    version_value = (unsigned short)((major << 8) | minor);

    context->VERSION_1_0 = (unsigned char)(version_value >= 0x0100);
    context->VERSION_1_1 = (unsigned char)(version_value >= 0x0101);
    context->VERSION_1_2 = (unsigned char)(version_value >= 0x0102);
    context->VERSION_1_3 = (unsigned char)(version_value >= 0x0103);
    context->VERSION_1_4 = (unsigned char)(version_value >= 0x0104);
    context->VERSION_1_5 = (unsigned char)(version_value >= 0x0105);

    return (int)version_value;
}

int gloamLoadEGLContext(GloamEGLContext *context, EGLDisplay display, GloamLoadFunc getProcAddr)
{
    int version;
    uint32_t i;
    GLOAM_UNUSED(kFnCount_EGL);

    memset(context, 0, sizeof(*context));

    /* Bootstrap: QueryString must be loaded before version detection. */
    context->QueryString = (PFNEGLQUERYSTRINGPROC)getProcAddr("eglQueryString");

    version = gloam_egl_find_core_egl(context, display);
    if (!version)
        return 0;

    /* Load PFNs for each enabled feature via the range table. */
    for (i = 0; i < GLOAM_ARRAYSIZE(kFeatPfnRanges_EGL); ++i) {
        const GloamPfnRange_t *r = &kFeatPfnRanges_EGL[i];
        if (context->featArray[r->extension])
            gloam_load_pfn_range_egl(context, getProcAddr, r->start, r->count);
    }

    if (!gloam_egl_find_extensions_egl(context, display))
        return 0;

    for (i = 0; i < GLOAM_ARRAYSIZE(kExtPfnRanges_egl); ++i) {
        const GloamPfnRange_t *r = &kExtPfnRanges_egl[i];
        if (context->extArray[r->extension])
            gloam_load_pfn_range_egl(context, getProcAddr, r->start, r->count);
    }

    return version;
}

int gloamLoadEGL(EGLDisplay display, GloamLoadFunc getProcAddr)
{
    return gloamLoadEGLContext(&gloam_egl_context, display, getProcAddr);
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


/* ---- EGL built-in loader ------------------------------------------------ */

static const char * const gloam_egl_lib_names[] = {
#if defined(__APPLE__)
    "libEGL.dylib",
#elif defined(GLOAM_PLATFORM_WINDOWS)
    "libEGL.dll", "EGL.dll",
#else
    "libEGL.so.1", "libEGL.so",
#endif
};

/* eglGetProcAddress has its own calling convention (declared in EGL headers as
 * PFNEGLGETPROCADDRESSPROC). Store it with the correct type so the call always
 * uses the right convention, even when the default differs (e.g. Win32).
 */
struct gloam_egl_load_userptr {
    void *handle;
    PFNEGLGETPROCADDRESSPROC get_proc_address;
};
static struct gloam_egl_load_userptr gloam_egl_load_state;

static GloamAPIProc gloam_egl_get_proc(const char *name)
{
    struct gloam_egl_load_userptr *u = &gloam_egl_load_state;
    GloamAPIProc result = (GloamAPIProc)gloam_dlsym(u->handle, name);
    if (!result && u->get_proc_address)
        result = u->get_proc_address(name);
    return result;
}


int gloamLoaderLoadEGLContext(GloamEGLContext *context, EGLDisplay display)
{
    int did_open = 0;
    int version;
    void *handle;

    handle = context->gloam_loader_handle;

    if (!handle) {
        handle = gloam_open_library(gloam_egl_lib_names, GLOAM_ARRAYSIZE(gloam_egl_lib_names));
        did_open = 1;
    }

    if (!handle)
        return 0;

    gloam_egl_load_state.handle = handle;
    gloam_egl_load_state.get_proc_address =
        (PFNEGLGETPROCADDRESSPROC)gloam_dlsym(handle, "eglGetProcAddress");

    if (!gloam_egl_load_state.get_proc_address) {
        if (did_open)
            gloam_dlclose(handle);
        return 0;
    }

    version = gloamLoadEGLContext(context, display, gloam_egl_get_proc);
    gloam_egl_load_state.handle = NULL;

    if (!version && did_open) {
        gloam_dlclose(handle);
        return 0;
    }

    context->gloam_loader_handle = handle;
    context->gloam_loader_owns_handle |= (uint8_t)did_open;

    return version;
}

int gloamLoaderLoadEGL(EGLDisplay display)
{
    return gloamLoaderLoadEGLContext(&gloam_egl_context, display);
}

void gloamLoaderUnloadEGLContext(GloamEGLContext *context)
{
    if (context->gloam_loader_handle && context->gloam_loader_owns_handle) {
        gloam_dlclose(context->gloam_loader_handle);
    }
    gloamLoaderResetEGLContext(context);
}

void gloamLoaderUnloadEGL(void)
{
    gloamLoaderUnloadEGLContext(&gloam_egl_context);
}

void gloamLoaderResetEGLContext(GloamEGLContext *context)
{
    memset(context, 0, sizeof(*context));
}

void gloamLoaderResetEGL(void)
{
    gloamLoaderResetEGLContext(&gloam_egl_context);
}
