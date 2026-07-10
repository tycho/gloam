#include <gloam/gl.h>
#include <gloam/wgl.h>

#ifdef GLOAM_PLATFORM_WINDOWS

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
GloamWGLContext gloam_wgl_context = {};
#else
GloamWGLContext gloam_wgl_context = { 0 };
#endif

/* ---- Function name table -------------------------------------------------
 * Command names stored as a single NUL-terminated string blob with a parallel
 * offset table for O(1) indexing. This avoids one pointer (8 bytes on 64-bit)
 * plus one relocation entry (~24 bytes in PIC builds) per command compared to
 * the traditional const char * const [] approach.
 */
static const uint32_t kFnCount_WGL = 28;

static const char kFnNameData_WGL[] =
    /*     0 */ "ChoosePixelFormat\0"
    /*    18 */ "DescribePixelFormat\0"
    /*    38 */ "GetEnhMetaFilePixelFormat\0"
    /*    64 */ "GetPixelFormat\0"
    /*    79 */ "SetPixelFormat\0"
    /*    94 */ "SwapBuffers\0"
    /*   106 */ "wglCopyContext\0"
    /*   121 */ "wglCreateContext\0"
    /*   138 */ "wglCreateLayerContext\0"
    /*   160 */ "wglDeleteContext\0"
    /*   177 */ "wglDescribeLayerPlane\0"
    /*   199 */ "wglGetCurrentContext\0"
    /*   220 */ "wglGetCurrentDC\0"
    /*   236 */ "wglGetLayerPaletteEntries\0"
    /*   262 */ "wglGetProcAddress\0"
    /*   280 */ "wglMakeCurrent\0"
    /*   295 */ "wglRealizeLayerPalette\0"
    /*   318 */ "wglSetLayerPaletteEntries\0"
    /*   344 */ "wglShareLists\0"
    /*   358 */ "wglSwapLayerBuffers\0"
    /*   378 */ "wglUseFontBitmaps\0"
    /*   396 */ "wglUseFontBitmapsA\0"
    /*   415 */ "wglUseFontBitmapsW\0"
    /*   434 */ "wglUseFontOutlines\0"
    /*   453 */ "wglUseFontOutlinesA\0"
    /*   473 */ "wglUseFontOutlinesW\0"
    /*   493 */ "wglGetExtensionsStringARB\0"
    /*   519 */ "wglGetExtensionsStringEXT\0"
;
static const uint16_t kFnNameOffsets_WGL[] = {
    /*    0 */     0, /* ChoosePixelFormat */
    /*    1 */    18, /* DescribePixelFormat */
    /*    2 */    38, /* GetEnhMetaFilePixelFormat */
    /*    3 */    64, /* GetPixelFormat */
    /*    4 */    79, /* SetPixelFormat */
    /*    5 */    94, /* SwapBuffers */
    /*    6 */   106, /* wglCopyContext */
    /*    7 */   121, /* wglCreateContext */
    /*    8 */   138, /* wglCreateLayerContext */
    /*    9 */   160, /* wglDeleteContext */
    /*   10 */   177, /* wglDescribeLayerPlane */
    /*   11 */   199, /* wglGetCurrentContext */
    /*   12 */   220, /* wglGetCurrentDC */
    /*   13 */   236, /* wglGetLayerPaletteEntries */
    /*   14 */   262, /* wglGetProcAddress */
    /*   15 */   280, /* wglMakeCurrent */
    /*   16 */   295, /* wglRealizeLayerPalette */
    /*   17 */   318, /* wglSetLayerPaletteEntries */
    /*   18 */   344, /* wglShareLists */
    /*   19 */   358, /* wglSwapLayerBuffers */
    /*   20 */   378, /* wglUseFontBitmaps */
    /*   21 */   396, /* wglUseFontBitmapsA */
    /*   22 */   415, /* wglUseFontBitmapsW */
    /*   23 */   434, /* wglUseFontOutlines */
    /*   24 */   453, /* wglUseFontOutlinesA */
    /*   25 */   473, /* wglUseFontOutlinesW */
    /*   26 */   493, /* wglGetExtensionsStringARB */
    /*   27 */   519 /* wglGetExtensionsStringEXT */
};
/* ---- Extension hash table ------------------------------------------------
   One XXH3-64 hash per extension, in extArray index order.
   Pre-baked at generator time with the same algorithm used at load time. */
static const uint64_t kExtHashes_WGL[] = {
    /*    0 */ 0xd6afdfe5c6fa3614ULL, /* WGL_ARB_extensions_string */
    /*    1 */ 0x675d378710334a65ULL  /* WGL_EXT_extensions_string */
};

/* ---- Feature PFN range table ---------------------------------------------
 * Each entry maps one feature (by featArray index) to a contiguous run of
 * pfnArray slots. The loader iterates this table and bulk-loads the run
 * when featArray[entry.extension] is set.
 */
static const GloamPfnRange_t kFeatPfnRanges_WGL[] = {
    {    0,    0,   26 }, /* WGL_VERSION_1_0 */
};

/* ---- PFN range helper (GL / EGL / GLX / WGL) ----------------------------
 * Walks a contiguous run of pfnArray slots and calls the plain load callback
 * (name only) for each one.
 */
static void gloam_load_pfn_range_wgl(GloamWGLContext *context, GloamLoadFunc getProcAddr, uint16_t start, uint16_t count)
{
    uint16_t i;
    for (i = start; i < (uint16_t)(start + count); ++i) {
        const char *pfnName = &kFnNameData_WGL[kFnNameOffsets_WGL[i]];
        context->pfnArray[i] = (void *)getProcAddr(pfnName);
    }
}


/* ==========================================================================
 * Per-API sections
 * ==========================================================================
 */

/* --------------------------------------------------------------------------
 * API: wgl
 * --------------------------------------------------------------------------
 */

/* Extension index subset for wgl: extArray indices this API supports. */
static const uint16_t kExtIdx_wgl[] = {
       0, /* WGL_ARB_extensions_string */
       1, /* WGL_EXT_extensions_string */
};

/* Extension PFN range table for wgl. */
static const GloamPfnRange_t kExtPfnRanges_wgl[] = {
    {    0,   26,    1 }, /* WGL_ARB_extensions_string */
    {    1,   27,    1 }, /* WGL_EXT_extensions_string */
};

/* WGL: wglGetExtensionsStringARB / wglGetExtensionsStringEXT. */
static int gloam_wgl_get_extensions_wgl(GloamWGLContext *context, HDC hdc, uint64_t **out_exts, uint32_t *out_num_exts)
{
    const char *ext_str = NULL;

    if (context->GetExtensionsStringARB)
        ext_str = (const char *)context->GetExtensionsStringARB(hdc);

    if (!ext_str && context->GetExtensionsStringEXT)
        ext_str = (const char *)context->GetExtensionsStringEXT();

    if (!ext_str)
        return 0;

    return gloam_hash_ext_string(ext_str, out_exts, out_num_exts);
}

/* Search pre-baked kExtHashes_WGL against the sorted driver hash list and set
 * extArray flags for every matching extension.
 */
static int gloam_wgl_find_extensions_wgl(GloamWGLContext *context, HDC hdc)
{
    uint64_t *exts = NULL;
    uint32_t  num_exts = 0, i;

    if (!gloam_wgl_get_extensions_wgl(context, hdc, &exts, &num_exts))
        return 0;

    for (i = 0; i < GLOAM_ARRAYSIZE(kExtIdx_wgl); ++i) {
        const uint16_t extIdx = kExtIdx_wgl[i];
        context->extArray[extIdx] = (unsigned char)gloam_hash_search(exts, num_exts, kExtHashes_WGL[extIdx]);
    }

    free(exts);
    return 1;
}

/* Faux version detection on WGL */
static int gloam_wgl_find_core_wgl(GloamWGLContext *context)
{
    int major = 1, minor = 0;
    unsigned short version_value;
    version_value = (unsigned short)((major << 8) | minor);

    context->VERSION_1_0 = (unsigned char)(version_value >= 0x0100);

    return (int)version_value;
}int gloamLoadWGLContext(GloamWGLContext *context, HDC hdc, GloamLoadFunc getProcAddr)
{
    int version;
    uint32_t i;
    GLOAM_UNUSED(kFnCount_WGL);

    memset(context, 0, sizeof(*context));

    /* WGL mandatory extensions must be loaded first for extension detection. */
    context->GetExtensionsStringARB = (PFNWGLGETEXTENSIONSSTRINGARBPROC)getProcAddr("wglGetExtensionsStringARB");
    context->GetExtensionsStringEXT = (PFNWGLGETEXTENSIONSSTRINGEXTPROC)getProcAddr("wglGetExtensionsStringEXT");

    version = gloam_wgl_find_core_wgl(context);
    if (!version)
        return 0;

    /* Load all PFNs upfront. */
    for (i = 0; i < kFnCount_WGL; ++i)
        context->pfnArray[i] = (void *)getProcAddr((kFnNameData_WGL + kFnNameOffsets_WGL[i]));

    /* Mark features based on PFN availability. */
    for (i = 0; i < GLOAM_ARRAYSIZE(kFeatPfnRanges_WGL); ++i) {
        const GloamPfnRange_t *r = &kFeatPfnRanges_WGL[i];
        uint16_t j; int ok = 1;
        for (j = r->start; j < (uint16_t)(r->start + r->count); ++j)
            ok &= (context->pfnArray[j] != NULL);
        if (ok)
            context->featArray[r->extension] = 1;
    }


    if (!gloam_wgl_find_extensions_wgl(context, hdc))
        return 0;

    for (i = 0; i < GLOAM_ARRAYSIZE(kExtPfnRanges_wgl); ++i) {
        const GloamPfnRange_t *r = &kExtPfnRanges_wgl[i];
        if (context->extArray[r->extension])
            gloam_load_pfn_range_wgl(context, getProcAddr, r->start, r->count);
    }

    return 1;
}

int gloamLoadWGL(HDC hdc, GloamLoadFunc getProcAddr)
{
    return gloamLoadWGLContext(&gloam_wgl_context, hdc, getProcAddr);
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


/* ---- WGL built-in loader ------------------------------------------------ */

static const char * const gloam_wgl_lib_names[] = { "opengl32.dll" };

/* wglGetProcAddress is WINAPI (__stdcall); must not be stored or called through
 * a plain cdecl pointer. Store with the correct type in a userptr struct.
 */
struct gloam_wgl_load_userptr {
    void *handle;
    GloamAPIProc (WINAPI *wgl_get_proc)(const char *);
};
static struct gloam_wgl_load_userptr gloam_wgl_load_state;

static GloamAPIProc gloam_wgl_get_proc(const char *name)
{
    struct gloam_wgl_load_userptr *u = &gloam_wgl_load_state;
    GloamAPIProc result = NULL;
    if (u->wgl_get_proc)
        result = u->wgl_get_proc(name);
    if (!result)
        result = (GloamAPIProc)gloam_dlsym(u->handle, name);
    return result;
}
int gloamLoaderLoadWGLContext(GloamWGLContext *context, HDC hdc)
{
    int did_open = 0;
    int version;
    void *handle = context->gloam_loader_handle;

    if (!handle) {
        handle = gloam_open_library(
            gloam_wgl_lib_names, GLOAM_ARRAYSIZE(gloam_wgl_lib_names));
        did_open = 1;
    }

    if (!handle)
        return 0;

    gloam_wgl_load_state.handle = handle;
    gloam_wgl_load_state.wgl_get_proc =
        (GloamAPIProc (WINAPI *)(const char *))
            gloam_dlsym(handle, "wglGetProcAddress");

    if (!gloam_wgl_load_state.wgl_get_proc) {
        if (did_open)
            gloam_dlclose(handle);
        return 0;
    }

    version = gloamLoadWGLContext(context, hdc, gloam_wgl_get_proc);
    gloam_wgl_load_state.handle = NULL;

    if (!version && did_open) {
        gloam_dlclose(handle);
        return 0;
    }

    context->gloam_loader_handle = handle;
    context->gloam_loader_owns_handle |= (uint8_t)did_open;

    return version;
}

int gloamLoaderLoadWGL(HDC hdc)
{
    return gloamLoaderLoadWGLContext(&gloam_wgl_context, hdc);
}

void gloamLoaderUnloadWGLContext(GloamWGLContext *context)
{
    if (context->gloam_loader_handle && context->gloam_loader_owns_handle) {
        gloam_dlclose(context->gloam_loader_handle);
    }
    gloamLoaderResetWGLContext(context);
}

void gloamLoaderUnloadWGL(void)
{
    gloamLoaderUnloadWGLContext(&gloam_wgl_context);
}

void gloamLoaderResetWGLContext(GloamWGLContext *context)
{
    memset(context, 0, sizeof(*context));
}

void gloamLoaderResetWGL(void)
{
    gloamLoaderResetWGLContext(&gloam_wgl_context);
}

#endif /* GLOAM_PLATFORM_WINDOWS */
