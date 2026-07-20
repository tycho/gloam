#include <gloam/gles1.h>

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
GloamGLContext gloam_gl_context = {};
#else
GloamGLContext gloam_gl_context = { 0 };
#endif

/* ---- Function name table -------------------------------------------------
 * Command names stored as a single NUL-terminated string blob with a parallel
 * offset table for O(1) indexing. This avoids one pointer (8 bytes on 64-bit)
 * plus one relocation entry (~24 bytes in PIC builds) per command compared to
 * the traditional const char * const [] approach.
 */
static const uint32_t kFnCount_GL = 159;

static const char kFnNameData_GL[] =
    /*     0 */ "glActiveTexture\0"
    /*    16 */ "glAlphaFunc\0"
    /*    28 */ "glAlphaFuncx\0"
    /*    41 */ "glBindBuffer\0"
    /*    54 */ "glBindTexture\0"
    /*    68 */ "glBlendFunc\0"
    /*    80 */ "glBufferData\0"
    /*    93 */ "glBufferSubData\0"
    /*   109 */ "glClear\0"
    /*   117 */ "glClearColor\0"
    /*   130 */ "glClearColorx\0"
    /*   144 */ "glClearDepthf\0"
    /*   158 */ "glClearDepthx\0"
    /*   172 */ "glClearStencil\0"
    /*   187 */ "glClientActiveTexture\0"
    /*   209 */ "glClipPlanef\0"
    /*   222 */ "glClipPlanex\0"
    /*   235 */ "glColor4f\0"
    /*   245 */ "glColor4ub\0"
    /*   256 */ "glColor4x\0"
    /*   266 */ "glColorMask\0"
    /*   278 */ "glColorPointer\0"
    /*   293 */ "glCompressedTexImage2D\0"
    /*   316 */ "glCompressedTexSubImage2D\0"
    /*   342 */ "glCopyTexImage2D\0"
    /*   359 */ "glCopyTexSubImage2D\0"
    /*   379 */ "glCullFace\0"
    /*   390 */ "glDeleteBuffers\0"
    /*   406 */ "glDeleteTextures\0"
    /*   423 */ "glDepthFunc\0"
    /*   435 */ "glDepthMask\0"
    /*   447 */ "glDepthRangef\0"
    /*   461 */ "glDepthRangex\0"
    /*   475 */ "glDisable\0"
    /*   485 */ "glDisableClientState\0"
    /*   506 */ "glDrawArrays\0"
    /*   519 */ "glDrawElements\0"
    /*   534 */ "glEnable\0"
    /*   543 */ "glEnableClientState\0"
    /*   563 */ "glFinish\0"
    /*   572 */ "glFlush\0"
    /*   580 */ "glFogf\0"
    /*   587 */ "glFogfv\0"
    /*   595 */ "glFogx\0"
    /*   602 */ "glFogxv\0"
    /*   610 */ "glFrontFace\0"
    /*   622 */ "glFrustumf\0"
    /*   633 */ "glFrustumx\0"
    /*   644 */ "glGenBuffers\0"
    /*   657 */ "glGenTextures\0"
    /*   671 */ "glGetBooleanv\0"
    /*   685 */ "glGetBufferParameteriv\0"
    /*   708 */ "glGetClipPlanef\0"
    /*   724 */ "glGetClipPlanex\0"
    /*   740 */ "glGetError\0"
    /*   751 */ "glGetFixedv\0"
    /*   763 */ "glGetFloatv\0"
    /*   775 */ "glGetIntegerv\0"
    /*   789 */ "glGetLightfv\0"
    /*   802 */ "glGetLightxv\0"
    /*   815 */ "glGetMaterialfv\0"
    /*   831 */ "glGetMaterialxv\0"
    /*   847 */ "glGetPointerv\0"
    /*   861 */ "glGetString\0"
    /*   873 */ "glGetTexEnvfv\0"
    /*   887 */ "glGetTexEnviv\0"
    /*   901 */ "glGetTexEnvxv\0"
    /*   915 */ "glGetTexParameterfv\0"
    /*   935 */ "glGetTexParameteriv\0"
    /*   955 */ "glGetTexParameterxv\0"
    /*   975 */ "glHint\0"
    /*   982 */ "glIsBuffer\0"
    /*   993 */ "glIsEnabled\0"
    /*  1005 */ "glIsTexture\0"
    /*  1017 */ "glLightModelf\0"
    /*  1031 */ "glLightModelfv\0"
    /*  1046 */ "glLightModelx\0"
    /*  1060 */ "glLightModelxv\0"
    /*  1075 */ "glLightf\0"
    /*  1084 */ "glLightfv\0"
    /*  1094 */ "glLightx\0"
    /*  1103 */ "glLightxv\0"
    /*  1113 */ "glLineWidth\0"
    /*  1125 */ "glLineWidthx\0"
    /*  1138 */ "glLoadIdentity\0"
    /*  1153 */ "glLoadMatrixf\0"
    /*  1167 */ "glLoadMatrixx\0"
    /*  1181 */ "glLogicOp\0"
    /*  1191 */ "glMaterialf\0"
    /*  1203 */ "glMaterialfv\0"
    /*  1216 */ "glMaterialx\0"
    /*  1228 */ "glMaterialxv\0"
    /*  1241 */ "glMatrixMode\0"
    /*  1254 */ "glMultMatrixf\0"
    /*  1268 */ "glMultMatrixx\0"
    /*  1282 */ "glMultiTexCoord4f\0"
    /*  1300 */ "glMultiTexCoord4x\0"
    /*  1318 */ "glNormal3f\0"
    /*  1329 */ "glNormal3x\0"
    /*  1340 */ "glNormalPointer\0"
    /*  1356 */ "glOrthof\0"
    /*  1365 */ "glOrthox\0"
    /*  1374 */ "glPixelStorei\0"
    /*  1388 */ "glPointParameterf\0"
    /*  1406 */ "glPointParameterfv\0"
    /*  1425 */ "glPointParameterx\0"
    /*  1443 */ "glPointParameterxv\0"
    /*  1462 */ "glPointSize\0"
    /*  1474 */ "glPointSizex\0"
    /*  1487 */ "glPolygonOffset\0"
    /*  1503 */ "glPolygonOffsetx\0"
    /*  1520 */ "glPopMatrix\0"
    /*  1532 */ "glPushMatrix\0"
    /*  1545 */ "glReadPixels\0"
    /*  1558 */ "glRotatef\0"
    /*  1568 */ "glRotatex\0"
    /*  1578 */ "glSampleCoverage\0"
    /*  1595 */ "glSampleCoveragex\0"
    /*  1613 */ "glScalef\0"
    /*  1622 */ "glScalex\0"
    /*  1631 */ "glScissor\0"
    /*  1641 */ "glShadeModel\0"
    /*  1654 */ "glStencilFunc\0"
    /*  1668 */ "glStencilMask\0"
    /*  1682 */ "glStencilOp\0"
    /*  1694 */ "glTexCoordPointer\0"
    /*  1712 */ "glTexEnvf\0"
    /*  1722 */ "glTexEnvfv\0"
    /*  1733 */ "glTexEnvi\0"
    /*  1743 */ "glTexEnviv\0"
    /*  1754 */ "glTexEnvx\0"
    /*  1764 */ "glTexEnvxv\0"
    /*  1775 */ "glTexImage2D\0"
    /*  1788 */ "glTexParameterf\0"
    /*  1804 */ "glTexParameterfv\0"
    /*  1821 */ "glTexParameteri\0"
    /*  1837 */ "glTexParameteriv\0"
    /*  1854 */ "glTexParameterx\0"
    /*  1870 */ "glTexParameterxv\0"
    /*  1887 */ "glTexSubImage2D\0"
    /*  1903 */ "glTranslatef\0"
    /*  1916 */ "glTranslatex\0"
    /*  1929 */ "glVertexPointer\0"
    /*  1945 */ "glViewport\0"
    /*  1956 */ "glBindFramebufferOES\0"
    /*  1977 */ "glBindRenderbufferOES\0"
    /*  1999 */ "glCheckFramebufferStatusOES\0"
    /*  2027 */ "glDeleteFramebuffersOES\0"
    /*  2051 */ "glDeleteRenderbuffersOES\0"
    /*  2076 */ "glFramebufferRenderbufferOES\0"
    /*  2105 */ "glFramebufferTexture2DOES\0"
    /*  2131 */ "glGenFramebuffersOES\0"
    /*  2152 */ "glGenRenderbuffersOES\0"
    /*  2174 */ "glGenerateMipmapOES\0"
    /*  2194 */ "glGetFramebufferAttachmentParameterivOES\0"
    /*  2235 */ "glGetRenderbufferParameterivOES\0"
    /*  2267 */ "glIsFramebufferOES\0"
    /*  2286 */ "glIsRenderbufferOES\0"
    /*  2306 */ "glRenderbufferStorageOES\0"
;
static const uint16_t kFnNameOffsets_GL[] = {
    /*    0 */     0, /* glActiveTexture */
    /*    1 */    16, /* glAlphaFunc */
    /*    2 */    28, /* glAlphaFuncx */
    /*    3 */    41, /* glBindBuffer */
    /*    4 */    54, /* glBindTexture */
    /*    5 */    68, /* glBlendFunc */
    /*    6 */    80, /* glBufferData */
    /*    7 */    93, /* glBufferSubData */
    /*    8 */   109, /* glClear */
    /*    9 */   117, /* glClearColor */
    /*   10 */   130, /* glClearColorx */
    /*   11 */   144, /* glClearDepthf */
    /*   12 */   158, /* glClearDepthx */
    /*   13 */   172, /* glClearStencil */
    /*   14 */   187, /* glClientActiveTexture */
    /*   15 */   209, /* glClipPlanef */
    /*   16 */   222, /* glClipPlanex */
    /*   17 */   235, /* glColor4f */
    /*   18 */   245, /* glColor4ub */
    /*   19 */   256, /* glColor4x */
    /*   20 */   266, /* glColorMask */
    /*   21 */   278, /* glColorPointer */
    /*   22 */   293, /* glCompressedTexImage2D */
    /*   23 */   316, /* glCompressedTexSubImage2D */
    /*   24 */   342, /* glCopyTexImage2D */
    /*   25 */   359, /* glCopyTexSubImage2D */
    /*   26 */   379, /* glCullFace */
    /*   27 */   390, /* glDeleteBuffers */
    /*   28 */   406, /* glDeleteTextures */
    /*   29 */   423, /* glDepthFunc */
    /*   30 */   435, /* glDepthMask */
    /*   31 */   447, /* glDepthRangef */
    /*   32 */   461, /* glDepthRangex */
    /*   33 */   475, /* glDisable */
    /*   34 */   485, /* glDisableClientState */
    /*   35 */   506, /* glDrawArrays */
    /*   36 */   519, /* glDrawElements */
    /*   37 */   534, /* glEnable */
    /*   38 */   543, /* glEnableClientState */
    /*   39 */   563, /* glFinish */
    /*   40 */   572, /* glFlush */
    /*   41 */   580, /* glFogf */
    /*   42 */   587, /* glFogfv */
    /*   43 */   595, /* glFogx */
    /*   44 */   602, /* glFogxv */
    /*   45 */   610, /* glFrontFace */
    /*   46 */   622, /* glFrustumf */
    /*   47 */   633, /* glFrustumx */
    /*   48 */   644, /* glGenBuffers */
    /*   49 */   657, /* glGenTextures */
    /*   50 */   671, /* glGetBooleanv */
    /*   51 */   685, /* glGetBufferParameteriv */
    /*   52 */   708, /* glGetClipPlanef */
    /*   53 */   724, /* glGetClipPlanex */
    /*   54 */   740, /* glGetError */
    /*   55 */   751, /* glGetFixedv */
    /*   56 */   763, /* glGetFloatv */
    /*   57 */   775, /* glGetIntegerv */
    /*   58 */   789, /* glGetLightfv */
    /*   59 */   802, /* glGetLightxv */
    /*   60 */   815, /* glGetMaterialfv */
    /*   61 */   831, /* glGetMaterialxv */
    /*   62 */   847, /* glGetPointerv */
    /*   63 */   861, /* glGetString */
    /*   64 */   873, /* glGetTexEnvfv */
    /*   65 */   887, /* glGetTexEnviv */
    /*   66 */   901, /* glGetTexEnvxv */
    /*   67 */   915, /* glGetTexParameterfv */
    /*   68 */   935, /* glGetTexParameteriv */
    /*   69 */   955, /* glGetTexParameterxv */
    /*   70 */   975, /* glHint */
    /*   71 */   982, /* glIsBuffer */
    /*   72 */   993, /* glIsEnabled */
    /*   73 */  1005, /* glIsTexture */
    /*   74 */  1017, /* glLightModelf */
    /*   75 */  1031, /* glLightModelfv */
    /*   76 */  1046, /* glLightModelx */
    /*   77 */  1060, /* glLightModelxv */
    /*   78 */  1075, /* glLightf */
    /*   79 */  1084, /* glLightfv */
    /*   80 */  1094, /* glLightx */
    /*   81 */  1103, /* glLightxv */
    /*   82 */  1113, /* glLineWidth */
    /*   83 */  1125, /* glLineWidthx */
    /*   84 */  1138, /* glLoadIdentity */
    /*   85 */  1153, /* glLoadMatrixf */
    /*   86 */  1167, /* glLoadMatrixx */
    /*   87 */  1181, /* glLogicOp */
    /*   88 */  1191, /* glMaterialf */
    /*   89 */  1203, /* glMaterialfv */
    /*   90 */  1216, /* glMaterialx */
    /*   91 */  1228, /* glMaterialxv */
    /*   92 */  1241, /* glMatrixMode */
    /*   93 */  1254, /* glMultMatrixf */
    /*   94 */  1268, /* glMultMatrixx */
    /*   95 */  1282, /* glMultiTexCoord4f */
    /*   96 */  1300, /* glMultiTexCoord4x */
    /*   97 */  1318, /* glNormal3f */
    /*   98 */  1329, /* glNormal3x */
    /*   99 */  1340, /* glNormalPointer */
    /*  100 */  1356, /* glOrthof */
    /*  101 */  1365, /* glOrthox */
    /*  102 */  1374, /* glPixelStorei */
    /*  103 */  1388, /* glPointParameterf */
    /*  104 */  1406, /* glPointParameterfv */
    /*  105 */  1425, /* glPointParameterx */
    /*  106 */  1443, /* glPointParameterxv */
    /*  107 */  1462, /* glPointSize */
    /*  108 */  1474, /* glPointSizex */
    /*  109 */  1487, /* glPolygonOffset */
    /*  110 */  1503, /* glPolygonOffsetx */
    /*  111 */  1520, /* glPopMatrix */
    /*  112 */  1532, /* glPushMatrix */
    /*  113 */  1545, /* glReadPixels */
    /*  114 */  1558, /* glRotatef */
    /*  115 */  1568, /* glRotatex */
    /*  116 */  1578, /* glSampleCoverage */
    /*  117 */  1595, /* glSampleCoveragex */
    /*  118 */  1613, /* glScalef */
    /*  119 */  1622, /* glScalex */
    /*  120 */  1631, /* glScissor */
    /*  121 */  1641, /* glShadeModel */
    /*  122 */  1654, /* glStencilFunc */
    /*  123 */  1668, /* glStencilMask */
    /*  124 */  1682, /* glStencilOp */
    /*  125 */  1694, /* glTexCoordPointer */
    /*  126 */  1712, /* glTexEnvf */
    /*  127 */  1722, /* glTexEnvfv */
    /*  128 */  1733, /* glTexEnvi */
    /*  129 */  1743, /* glTexEnviv */
    /*  130 */  1754, /* glTexEnvx */
    /*  131 */  1764, /* glTexEnvxv */
    /*  132 */  1775, /* glTexImage2D */
    /*  133 */  1788, /* glTexParameterf */
    /*  134 */  1804, /* glTexParameterfv */
    /*  135 */  1821, /* glTexParameteri */
    /*  136 */  1837, /* glTexParameteriv */
    /*  137 */  1854, /* glTexParameterx */
    /*  138 */  1870, /* glTexParameterxv */
    /*  139 */  1887, /* glTexSubImage2D */
    /*  140 */  1903, /* glTranslatef */
    /*  141 */  1916, /* glTranslatex */
    /*  142 */  1929, /* glVertexPointer */
    /*  143 */  1945, /* glViewport */
    /*  144 */  1956, /* glBindFramebufferOES */
    /*  145 */  1977, /* glBindRenderbufferOES */
    /*  146 */  1999, /* glCheckFramebufferStatusOES */
    /*  147 */  2027, /* glDeleteFramebuffersOES */
    /*  148 */  2051, /* glDeleteRenderbuffersOES */
    /*  149 */  2076, /* glFramebufferRenderbufferOES */
    /*  150 */  2105, /* glFramebufferTexture2DOES */
    /*  151 */  2131, /* glGenFramebuffersOES */
    /*  152 */  2152, /* glGenRenderbuffersOES */
    /*  153 */  2174, /* glGenerateMipmapOES */
    /*  154 */  2194, /* glGetFramebufferAttachmentParameterivOES */
    /*  155 */  2235, /* glGetRenderbufferParameterivOES */
    /*  156 */  2267, /* glIsFramebufferOES */
    /*  157 */  2286, /* glIsRenderbufferOES */
    /*  158 */  2306 /* glRenderbufferStorageOES */
};
/* ---- Extension hash table ------------------------------------------------
   One XXH3-64 hash per extension, in extArray index order.
   Pre-baked at generator time with the same algorithm used at load time. */
static const uint64_t kExtHashes_GL[] = {
    /*    0 */ 0x524fb8b90ca87839ULL  /* GL_OES_framebuffer_object */
};

/* ---- Feature PFN range table ---------------------------------------------
 * Each entry maps one feature (by featArray index) to a contiguous run of
 * pfnArray slots. The loader iterates this table and bulk-loads the run
 * when featArray[entry.extension] is set.
 */
static const GloamPfnRange_t kFeatPfnRanges_GL[] = {
    {    0,    0,  144 }, /* GL_VERSION_ES_CM_1_0 */
};

/* ---- PFN range helper (GL / EGL / GLX / WGL) ----------------------------
 * Walks a contiguous run of pfnArray slots and calls the plain load callback
 * (name only) for each one.
 */
static void gloam_load_pfn_range_gl(GloamGLContext *context, GloamLoadFunc getProcAddr, uint16_t start, uint16_t count)
{
    uint16_t i;
    for (i = start; i < (uint16_t)(start + count); ++i) {
        const char *pfnName = &kFnNameData_GL[kFnNameOffsets_GL[i]];
        context->pfnArray[i] = (void *)getProcAddr(pfnName);
    }
}


/* ==========================================================================
 * Driver extension query (shared across per-API sections)
 * ==========================================================================
 */

/* GLES 1.x: only glGetString(GL_EXTENSIONS), space-separated. */
static int gloam_gl_get_extensions_gles1(GloamGLContext *context, uint64_t **out_exts, uint32_t *out_num_exts)
{
    const char *ext_str;

    if (!context->GetString)
        return 0;

    ext_str = (const char *)context->GetString(GL_EXTENSIONS);
    if (!ext_str)
        return 0;

    return gloam_hash_ext_string(ext_str, out_exts, out_num_exts);
}

/* ==========================================================================
 * Per-API sections
 * ==========================================================================
 */

/* --------------------------------------------------------------------------
 * API: gles1
 * --------------------------------------------------------------------------
 */

/* Extension index subset for gles1: extArray indices this API supports. */
static const uint16_t kExtIdx_gles1[] = {
       0, /* GL_OES_framebuffer_object */
};

/* Extension PFN range table for gles1. */
static const GloamPfnRange_t kExtPfnRanges_gles1[] = {
    {    0,  144,   15 }, /* GL_OES_framebuffer_object */
};

/* Search pre-baked kExtHashes_GL against the sorted driver hash list and set
 * extArray flags for every matching extension.
 */
static int gloam_gl_find_extensions_gles1(GloamGLContext *context)
{
    uint64_t *exts = NULL;
    uint32_t  num_exts = 0, i;

    if (!gloam_gl_get_extensions_gles1(context, &exts, &num_exts))
        return 0;

    for (i = 0; i < GLOAM_ARRAYSIZE(kExtIdx_gles1); ++i) {
        const uint16_t extIdx = kExtIdx_gles1[i];
        context->extArray[extIdx] = (unsigned char)gloam_hash_search(exts, num_exts, kExtHashes_GL[extIdx]);
    }

    free(exts);
    return 1;
}

/* Parse the GL_VERSION string and set featArray entries for this API. */
static int gloam_gl_find_core_gles1(GloamGLContext *context)
{
    int i, major = 0, minor = 0;
    unsigned short version_value;
    static const char * const kPrefixes[] = {
        "OpenGL ES-CM ", "OpenGL ES-CL ", "OpenGL ES ",
        "OpenGL SC ",    "OpenGL ",       NULL
    };
    const char *version = (const char *)context->GetString(GL_VERSION);
    if (!version)
        return 0;
    for (i = 0; kPrefixes[i]; ++i) {
        const size_t len = strlen(kPrefixes[i]);
        if (strncmp(version, kPrefixes[i], len) == 0) {
            version += len;
            break;
        }
    }
    GLOAM_IMPL_UTIL_SSCANF(version, "%d.%d", &major, &minor);
    version_value = (unsigned short)((major << 8) | minor);

    context->VERSION_ES_CM_1_0 = (unsigned char)(version_value >= 0x0100);

    return (int)version_value;
}

int gloamLoadGLES1Context(GloamGLContext *context, GloamLoadFunc getProcAddr)
{
    int version;
    uint32_t i;
    GLOAM_UNUSED(kFnCount_GL);

    memset(context, 0, sizeof(*context));

    /* Bootstrap: glGetString must be loaded before find_core can run. */
    context->GetString = (PFNGLGETSTRINGPROC)getProcAddr("glGetString");
    if (!context->GetString)
        return 0;

    version = gloam_gl_find_core_gles1(context);
    if (!version)
        return 0;

    /* Load PFNs for each enabled feature via the range table. */
    for (i = 0; i < GLOAM_ARRAYSIZE(kFeatPfnRanges_GL); ++i) {
        const GloamPfnRange_t *r = &kFeatPfnRanges_GL[i];
        if (context->featArray[r->extension])
            gloam_load_pfn_range_gl(context, getProcAddr, r->start, r->count);
    }

    if (!gloam_gl_find_extensions_gles1(context))
        return 0;

    /* Load PFNs for each detected extension via the range table. */
    for (i = 0; i < GLOAM_ARRAYSIZE(kExtPfnRanges_gles1); ++i) {
        const GloamPfnRange_t *r = &kExtPfnRanges_gles1[i];
        if (context->extArray[r->extension])
            gloam_load_pfn_range_gl(context, getProcAddr, r->start, r->count);
    }

    return version;
}

int gloamLoadGLES1(GloamLoadFunc getProcAddr)
{
    return gloamLoadGLES1Context(&gloam_gl_context, getProcAddr);
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


/* ---- GL / GLES built-in loader ----------------------------------------- */

/* Transient userptr: set for the duration of a gloamLoaderLoad*Context call.
 * Holds the library handle and — for desktop GL — the correctly-typed
 * platform proc-addr function pointer so that the calling convention is
 * always right. Only valid between entry and return; never accessed
 * concurrently.
 */
struct gloam_gl_load_userptr {
    void *handle;
#if defined(GLOAM_PLATFORM_WINDOWS)
    /* wglGetProcAddress is WINAPI (__stdcall); must not be called through a
     * plain cdecl function pointer.
	 */
    GloamAPIProc (WINAPI *wgl_get_proc)(const char *);
#elif !defined(__APPLE__) && !defined(__HAIKU__)
    GloamAPIProc (*glx_get_proc)(const char *);
#endif
};
static struct gloam_gl_load_userptr gloam_gl_load_state;

/* GL desktop adapter — matches GloamLoadFunc (__cdecl), dispatches through
 * the correctly-typed platform proc-addr pointer stored in the userptr.
 */
static GloamAPIProc gloam_gl_get_proc(const char *name)
{
    GloamAPIProc result = NULL;
    struct gloam_gl_load_userptr *u = &gloam_gl_load_state;
    if (!u->handle)
        return NULL;
#if defined(__APPLE__) || defined(__HAIKU__)
    result = (GloamAPIProc)gloam_dlsym(u->handle, name);
#elif defined(GLOAM_PLATFORM_WINDOWS)
    if (u->wgl_get_proc)
        result = u->wgl_get_proc(name);
    if (!result)
        result = (GloamAPIProc)gloam_dlsym(u->handle, name);
#else
    if (u->glx_get_proc)
        result = u->glx_get_proc(name);
    if (!result)
        result = (GloamAPIProc)gloam_dlsym(u->handle, name);
#endif
    return result;
}

/* GLES adapter: all symbols (including extensions) are exported directly from
 * the GLES library so plain dlsym is sufficient — no platform indirection.
 */
static GloamAPIProc gloam_gles_get_proc(const char *name)
{
    struct gloam_gl_load_userptr *u = &gloam_gl_load_state;
    if (!u->handle)
        return NULL;
    return (GloamAPIProc)gloam_dlsym(u->handle, name);
}

int gloamLoaderLoadGLES1Context(GloamGLContext *context)
{
    int did_open = 0;
    int version;
    void *handle;

    static const char * const kLibNames[] = {
#if defined(__APPLE__)
        "libGLESv2.dylib",
#elif defined(GLOAM_PLATFORM_WINDOWS)
        "GLESv2.dll", "libGLESv2.dll",
#else
        "libGLESv2.so.2", "libGLESv2.so",
#endif
    };

    handle = context->gloam_loader_handle;

    if (!handle) {
        handle = gloam_open_library(kLibNames, GLOAM_ARRAYSIZE(kLibNames));
        did_open = 1;
    }

    if (!handle)
        return 0;

    gloam_gl_load_state.handle = handle;
#if defined(GLOAM_PLATFORM_WINDOWS)
    gloam_gl_load_state.wgl_get_proc = (GloamAPIProc (WINAPI *)(const char *))gloam_dlsym(handle, "wglGetProcAddress");
#elif !defined(__APPLE__) && !defined(__HAIKU__)
    gloam_gl_load_state.glx_get_proc = (GloamAPIProc (*)(const char *))gloam_dlsym(handle, "glXGetProcAddressARB");
#endif


    version = gloamLoadGLES1Context(context, gloam_gles_get_proc);

    gloam_gl_load_state.handle = NULL;

    if (!version && did_open) {
        gloam_dlclose(handle);
        return 0;
    }

    context->gloam_loader_handle = handle;
    context->gloam_loader_owns_handle |= (uint8_t)did_open;

    return version;
}

int gloamLoaderLoadGLES1(void)
{
    return gloamLoaderLoadGLES1Context(&gloam_gl_context);
}

void gloamLoaderUnloadGLES1Context(GloamGLContext *context)
{
    if (context->gloam_loader_handle && context->gloam_loader_owns_handle) {
        gloam_dlclose(context->gloam_loader_handle);
    }
    gloamLoaderResetGLES1Context(context);
}

void gloamLoaderUnloadGLES1(void)
{
    gloamLoaderUnloadGLES1Context(&gloam_gl_context);
}

void gloamLoaderResetGLES1Context(GloamGLContext *context)
{
    memset(context, 0, sizeof(*context));
}

void gloamLoaderResetGLES1(void)
{
    gloamLoaderResetGLES1Context(&gloam_gl_context);
}
