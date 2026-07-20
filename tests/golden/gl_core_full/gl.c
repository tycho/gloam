#include <gloam/gl.h>

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

/* Bijective alias pair: if canonical slot is null but secondary is loaded
 * (or vice versa), the loaded pointer is propagated to both slots.
 */
typedef struct {
    uint16_t first;  /* canonical (shortest name) pfnArray index */
    uint16_t second; /* alias pfnArray index                     */
} GloamAliasPair_t;

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
static const uint32_t kFnCount_GL = 355;

static const char kFnNameData_GL[] =
    /*     0 */ "glBlendFunc\0"
    /*    12 */ "glClear\0"
    /*    20 */ "glClearColor\0"
    /*    33 */ "glClearDepth\0"
    /*    46 */ "glClearStencil\0"
    /*    61 */ "glColorMask\0"
    /*    73 */ "glCullFace\0"
    /*    84 */ "glDepthFunc\0"
    /*    96 */ "glDepthMask\0"
    /*   108 */ "glDepthRange\0"
    /*   121 */ "glDisable\0"
    /*   131 */ "glDrawBuffer\0"
    /*   144 */ "glEnable\0"
    /*   153 */ "glFinish\0"
    /*   162 */ "glFlush\0"
    /*   170 */ "glFrontFace\0"
    /*   182 */ "glGetBooleanv\0"
    /*   196 */ "glGetDoublev\0"
    /*   209 */ "glGetError\0"
    /*   220 */ "glGetFloatv\0"
    /*   232 */ "glGetIntegerv\0"
    /*   246 */ "glGetString\0"
    /*   258 */ "glGetTexImage\0"
    /*   272 */ "glGetTexLevelParameterfv\0"
    /*   297 */ "glGetTexLevelParameteriv\0"
    /*   322 */ "glGetTexParameterfv\0"
    /*   342 */ "glGetTexParameteriv\0"
    /*   362 */ "glHint\0"
    /*   369 */ "glIsEnabled\0"
    /*   381 */ "glLineWidth\0"
    /*   393 */ "glLogicOp\0"
    /*   403 */ "glPixelStoref\0"
    /*   417 */ "glPixelStorei\0"
    /*   431 */ "glPointSize\0"
    /*   443 */ "glPolygonMode\0"
    /*   457 */ "glReadBuffer\0"
    /*   470 */ "glReadPixels\0"
    /*   483 */ "glScissor\0"
    /*   493 */ "glStencilFunc\0"
    /*   507 */ "glStencilMask\0"
    /*   521 */ "glStencilOp\0"
    /*   533 */ "glTexImage1D\0"
    /*   546 */ "glTexImage2D\0"
    /*   559 */ "glTexParameterf\0"
    /*   575 */ "glTexParameterfv\0"
    /*   592 */ "glTexParameteri\0"
    /*   608 */ "glTexParameteriv\0"
    /*   625 */ "glViewport\0"
    /*   636 */ "glBindTexture\0"
    /*   650 */ "glCopyTexImage1D\0"
    /*   667 */ "glCopyTexImage2D\0"
    /*   684 */ "glCopyTexSubImage1D\0"
    /*   704 */ "glCopyTexSubImage2D\0"
    /*   724 */ "glDeleteTextures\0"
    /*   741 */ "glDrawArrays\0"
    /*   754 */ "glDrawElements\0"
    /*   769 */ "glGenTextures\0"
    /*   783 */ "glIsTexture\0"
    /*   795 */ "glPolygonOffset\0"
    /*   811 */ "glTexSubImage1D\0"
    /*   827 */ "glTexSubImage2D\0"
    /*   843 */ "glCopyTexSubImage3D\0"
    /*   863 */ "glDrawRangeElements\0"
    /*   883 */ "glTexImage3D\0"
    /*   896 */ "glTexSubImage3D\0"
    /*   912 */ "glActiveTexture\0"
    /*   928 */ "glCompressedTexImage1D\0"
    /*   951 */ "glCompressedTexImage2D\0"
    /*   974 */ "glCompressedTexImage3D\0"
    /*   997 */ "glCompressedTexSubImage1D\0"
    /*  1023 */ "glCompressedTexSubImage2D\0"
    /*  1049 */ "glCompressedTexSubImage3D\0"
    /*  1075 */ "glGetCompressedTexImage\0"
    /*  1099 */ "glSampleCoverage\0"
    /*  1116 */ "glBlendColor\0"
    /*  1129 */ "glBlendEquation\0"
    /*  1145 */ "glBlendFuncSeparate\0"
    /*  1165 */ "glMultiDrawArrays\0"
    /*  1183 */ "glMultiDrawElements\0"
    /*  1203 */ "glPointParameterf\0"
    /*  1221 */ "glPointParameterfv\0"
    /*  1240 */ "glPointParameteri\0"
    /*  1258 */ "glPointParameteriv\0"
    /*  1277 */ "glBeginQuery\0"
    /*  1290 */ "glBindBuffer\0"
    /*  1303 */ "glBufferData\0"
    /*  1316 */ "glBufferSubData\0"
    /*  1332 */ "glDeleteBuffers\0"
    /*  1348 */ "glDeleteQueries\0"
    /*  1364 */ "glEndQuery\0"
    /*  1375 */ "glGenBuffers\0"
    /*  1388 */ "glGenQueries\0"
    /*  1401 */ "glGetBufferParameteriv\0"
    /*  1424 */ "glGetBufferPointerv\0"
    /*  1444 */ "glGetBufferSubData\0"
    /*  1463 */ "glGetQueryObjectiv\0"
    /*  1482 */ "glGetQueryObjectuiv\0"
    /*  1502 */ "glGetQueryiv\0"
    /*  1515 */ "glIsBuffer\0"
    /*  1526 */ "glIsQuery\0"
    /*  1536 */ "glMapBuffer\0"
    /*  1548 */ "glUnmapBuffer\0"
    /*  1562 */ "glAttachShader\0"
    /*  1577 */ "glBindAttribLocation\0"
    /*  1598 */ "glBlendEquationSeparate\0"
    /*  1622 */ "glCompileShader\0"
    /*  1638 */ "glCreateProgram\0"
    /*  1654 */ "glCreateShader\0"
    /*  1669 */ "glDeleteProgram\0"
    /*  1685 */ "glDeleteShader\0"
    /*  1700 */ "glDetachShader\0"
    /*  1715 */ "glDisableVertexAttribArray\0"
    /*  1742 */ "glDrawBuffers\0"
    /*  1756 */ "glEnableVertexAttribArray\0"
    /*  1782 */ "glGetActiveAttrib\0"
    /*  1800 */ "glGetActiveUniform\0"
    /*  1819 */ "glGetAttachedShaders\0"
    /*  1840 */ "glGetAttribLocation\0"
    /*  1860 */ "glGetProgramInfoLog\0"
    /*  1880 */ "glGetProgramiv\0"
    /*  1895 */ "glGetShaderInfoLog\0"
    /*  1914 */ "glGetShaderSource\0"
    /*  1932 */ "glGetShaderiv\0"
    /*  1946 */ "glGetUniformLocation\0"
    /*  1967 */ "glGetUniformfv\0"
    /*  1982 */ "glGetUniformiv\0"
    /*  1997 */ "glGetVertexAttribPointerv\0"
    /*  2023 */ "glGetVertexAttribdv\0"
    /*  2043 */ "glGetVertexAttribfv\0"
    /*  2063 */ "glGetVertexAttribiv\0"
    /*  2083 */ "glIsProgram\0"
    /*  2095 */ "glIsShader\0"
    /*  2106 */ "glLinkProgram\0"
    /*  2120 */ "glShaderSource\0"
    /*  2135 */ "glStencilFuncSeparate\0"
    /*  2157 */ "glStencilMaskSeparate\0"
    /*  2179 */ "glStencilOpSeparate\0"
    /*  2199 */ "glUniform1f\0"
    /*  2211 */ "glUniform1fv\0"
    /*  2224 */ "glUniform1i\0"
    /*  2236 */ "glUniform1iv\0"
    /*  2249 */ "glUniform2f\0"
    /*  2261 */ "glUniform2fv\0"
    /*  2274 */ "glUniform2i\0"
    /*  2286 */ "glUniform2iv\0"
    /*  2299 */ "glUniform3f\0"
    /*  2311 */ "glUniform3fv\0"
    /*  2324 */ "glUniform3i\0"
    /*  2336 */ "glUniform3iv\0"
    /*  2349 */ "glUniform4f\0"
    /*  2361 */ "glUniform4fv\0"
    /*  2374 */ "glUniform4i\0"
    /*  2386 */ "glUniform4iv\0"
    /*  2399 */ "glUniformMatrix2fv\0"
    /*  2418 */ "glUniformMatrix3fv\0"
    /*  2437 */ "glUniformMatrix4fv\0"
    /*  2456 */ "glUseProgram\0"
    /*  2469 */ "glValidateProgram\0"
    /*  2487 */ "glVertexAttrib1d\0"
    /*  2504 */ "glVertexAttrib1dv\0"
    /*  2522 */ "glVertexAttrib1f\0"
    /*  2539 */ "glVertexAttrib1fv\0"
    /*  2557 */ "glVertexAttrib1s\0"
    /*  2574 */ "glVertexAttrib1sv\0"
    /*  2592 */ "glVertexAttrib2d\0"
    /*  2609 */ "glVertexAttrib2dv\0"
    /*  2627 */ "glVertexAttrib2f\0"
    /*  2644 */ "glVertexAttrib2fv\0"
    /*  2662 */ "glVertexAttrib2s\0"
    /*  2679 */ "glVertexAttrib2sv\0"
    /*  2697 */ "glVertexAttrib3d\0"
    /*  2714 */ "glVertexAttrib3dv\0"
    /*  2732 */ "glVertexAttrib3f\0"
    /*  2749 */ "glVertexAttrib3fv\0"
    /*  2767 */ "glVertexAttrib3s\0"
    /*  2784 */ "glVertexAttrib3sv\0"
    /*  2802 */ "glVertexAttrib4Nbv\0"
    /*  2821 */ "glVertexAttrib4Niv\0"
    /*  2840 */ "glVertexAttrib4Nsv\0"
    /*  2859 */ "glVertexAttrib4Nub\0"
    /*  2878 */ "glVertexAttrib4Nubv\0"
    /*  2898 */ "glVertexAttrib4Nuiv\0"
    /*  2918 */ "glVertexAttrib4Nusv\0"
    /*  2938 */ "glVertexAttrib4bv\0"
    /*  2956 */ "glVertexAttrib4d\0"
    /*  2973 */ "glVertexAttrib4dv\0"
    /*  2991 */ "glVertexAttrib4f\0"
    /*  3008 */ "glVertexAttrib4fv\0"
    /*  3026 */ "glVertexAttrib4iv\0"
    /*  3044 */ "glVertexAttrib4s\0"
    /*  3061 */ "glVertexAttrib4sv\0"
    /*  3079 */ "glVertexAttrib4ubv\0"
    /*  3098 */ "glVertexAttrib4uiv\0"
    /*  3117 */ "glVertexAttrib4usv\0"
    /*  3136 */ "glVertexAttribPointer\0"
    /*  3158 */ "glUniformMatrix2x3fv\0"
    /*  3179 */ "glUniformMatrix2x4fv\0"
    /*  3200 */ "glUniformMatrix3x2fv\0"
    /*  3221 */ "glUniformMatrix3x4fv\0"
    /*  3242 */ "glUniformMatrix4x2fv\0"
    /*  3263 */ "glUniformMatrix4x3fv\0"
    /*  3284 */ "glBeginConditionalRender\0"
    /*  3309 */ "glBeginTransformFeedback\0"
    /*  3334 */ "glBindFragDataLocation\0"
    /*  3357 */ "glBindFramebuffer\0"
    /*  3375 */ "glBindRenderbuffer\0"
    /*  3394 */ "glBindVertexArray\0"
    /*  3412 */ "glBlitFramebuffer\0"
    /*  3430 */ "glCheckFramebufferStatus\0"
    /*  3455 */ "glClampColor\0"
    /*  3468 */ "glClearBufferfi\0"
    /*  3484 */ "glClearBufferfv\0"
    /*  3500 */ "glClearBufferiv\0"
    /*  3516 */ "glClearBufferuiv\0"
    /*  3533 */ "glColorMaski\0"
    /*  3546 */ "glDeleteFramebuffers\0"
    /*  3567 */ "glDeleteRenderbuffers\0"
    /*  3589 */ "glDeleteVertexArrays\0"
    /*  3610 */ "glDisablei\0"
    /*  3621 */ "glEnablei\0"
    /*  3631 */ "glEndConditionalRender\0"
    /*  3654 */ "glEndTransformFeedback\0"
    /*  3677 */ "glFlushMappedBufferRange\0"
    /*  3702 */ "glFramebufferRenderbuffer\0"
    /*  3728 */ "glFramebufferTexture1D\0"
    /*  3751 */ "glFramebufferTexture2D\0"
    /*  3774 */ "glFramebufferTexture3D\0"
    /*  3797 */ "glFramebufferTextureLayer\0"
    /*  3823 */ "glGenFramebuffers\0"
    /*  3841 */ "glGenRenderbuffers\0"
    /*  3860 */ "glGenVertexArrays\0"
    /*  3878 */ "glGenerateMipmap\0"
    /*  3895 */ "glGetBooleani_v\0"
    /*  3911 */ "glGetFragDataLocation\0"
    /*  3933 */ "glGetFramebufferAttachmentParameteriv\0"
    /*  3971 */ "glGetRenderbufferParameteriv\0"
    /*  4000 */ "glGetStringi\0"
    /*  4013 */ "glGetTexParameterIiv\0"
    /*  4034 */ "glGetTexParameterIuiv\0"
    /*  4056 */ "glGetTransformFeedbackVarying\0"
    /*  4086 */ "glGetUniformuiv\0"
    /*  4102 */ "glGetVertexAttribIiv\0"
    /*  4123 */ "glGetVertexAttribIuiv\0"
    /*  4145 */ "glIsEnabledi\0"
    /*  4158 */ "glIsFramebuffer\0"
    /*  4174 */ "glIsRenderbuffer\0"
    /*  4191 */ "glIsVertexArray\0"
    /*  4207 */ "glMapBufferRange\0"
    /*  4224 */ "glRenderbufferStorage\0"
    /*  4246 */ "glRenderbufferStorageMultisample\0"
    /*  4279 */ "glTexParameterIiv\0"
    /*  4297 */ "glTexParameterIuiv\0"
    /*  4316 */ "glTransformFeedbackVaryings\0"
    /*  4344 */ "glUniform1ui\0"
    /*  4357 */ "glUniform1uiv\0"
    /*  4371 */ "glUniform2ui\0"
    /*  4384 */ "glUniform2uiv\0"
    /*  4398 */ "glUniform3ui\0"
    /*  4411 */ "glUniform3uiv\0"
    /*  4425 */ "glUniform4ui\0"
    /*  4438 */ "glUniform4uiv\0"
    /*  4452 */ "glVertexAttribI1i\0"
    /*  4470 */ "glVertexAttribI1iv\0"
    /*  4489 */ "glVertexAttribI1ui\0"
    /*  4508 */ "glVertexAttribI1uiv\0"
    /*  4528 */ "glVertexAttribI2i\0"
    /*  4546 */ "glVertexAttribI2iv\0"
    /*  4565 */ "glVertexAttribI2ui\0"
    /*  4584 */ "glVertexAttribI2uiv\0"
    /*  4604 */ "glVertexAttribI3i\0"
    /*  4622 */ "glVertexAttribI3iv\0"
    /*  4641 */ "glVertexAttribI3ui\0"
    /*  4660 */ "glVertexAttribI3uiv\0"
    /*  4680 */ "glVertexAttribI4bv\0"
    /*  4699 */ "glVertexAttribI4i\0"
    /*  4717 */ "glVertexAttribI4iv\0"
    /*  4736 */ "glVertexAttribI4sv\0"
    /*  4755 */ "glVertexAttribI4ubv\0"
    /*  4775 */ "glVertexAttribI4ui\0"
    /*  4794 */ "glVertexAttribI4uiv\0"
    /*  4814 */ "glVertexAttribI4usv\0"
    /*  4834 */ "glVertexAttribIPointer\0"
    /*  4857 */ "glBindBufferBase\0"
    /*  4874 */ "glBindBufferRange\0"
    /*  4892 */ "glGetIntegeri_v\0"
    /*  4908 */ "glCopyBufferSubData\0"
    /*  4928 */ "glDrawArraysInstanced\0"
    /*  4950 */ "glDrawElementsInstanced\0"
    /*  4974 */ "glGetActiveUniformBlockName\0"
    /*  5002 */ "glGetActiveUniformBlockiv\0"
    /*  5028 */ "glGetActiveUniformName\0"
    /*  5051 */ "glGetActiveUniformsiv\0"
    /*  5073 */ "glGetUniformBlockIndex\0"
    /*  5096 */ "glGetUniformIndices\0"
    /*  5116 */ "glPrimitiveRestartIndex\0"
    /*  5140 */ "glTexBuffer\0"
    /*  5152 */ "glUniformBlockBinding\0"
    /*  5174 */ "glDrawElementsBaseVertex\0"
    /*  5199 */ "glDrawElementsInstancedBaseVertex\0"
    /*  5233 */ "glDrawRangeElementsBaseVertex\0"
    /*  5263 */ "glFramebufferTexture\0"
    /*  5284 */ "glGetBufferParameteri64v\0"
    /*  5309 */ "glGetInteger64i_v\0"
    /*  5327 */ "glGetMultisamplefv\0"
    /*  5346 */ "glMultiDrawElementsBaseVertex\0"
    /*  5376 */ "glProvokingVertex\0"
    /*  5394 */ "glSampleMaski\0"
    /*  5408 */ "glTexImage2DMultisample\0"
    /*  5432 */ "glTexImage3DMultisample\0"
    /*  5456 */ "glClientWaitSync\0"
    /*  5473 */ "glDeleteSync\0"
    /*  5486 */ "glFenceSync\0"
    /*  5498 */ "glGetInteger64v\0"
    /*  5514 */ "glGetSynciv\0"
    /*  5526 */ "glIsSync\0"
    /*  5535 */ "glWaitSync\0"
    /*  5546 */ "glBindFragDataLocationIndexed\0"
    /*  5576 */ "glBindSampler\0"
    /*  5590 */ "glDeleteSamplers\0"
    /*  5607 */ "glGenSamplers\0"
    /*  5621 */ "glGetFragDataIndex\0"
    /*  5640 */ "glGetQueryObjecti64v\0"
    /*  5661 */ "glGetQueryObjectui64v\0"
    /*  5683 */ "glGetSamplerParameterIiv\0"
    /*  5708 */ "glGetSamplerParameterIuiv\0"
    /*  5734 */ "glGetSamplerParameterfv\0"
    /*  5758 */ "glGetSamplerParameteriv\0"
    /*  5782 */ "glIsSampler\0"
    /*  5794 */ "glQueryCounter\0"
    /*  5809 */ "glSamplerParameterIiv\0"
    /*  5831 */ "glSamplerParameterIuiv\0"
    /*  5854 */ "glSamplerParameterf\0"
    /*  5874 */ "glSamplerParameterfv\0"
    /*  5895 */ "glSamplerParameteri\0"
    /*  5915 */ "glSamplerParameteriv\0"
    /*  5936 */ "glVertexAttribDivisor\0"
    /*  5958 */ "glVertexAttribP1ui\0"
    /*  5977 */ "glVertexAttribP1uiv\0"
    /*  5997 */ "glVertexAttribP2ui\0"
    /*  6016 */ "glVertexAttribP2uiv\0"
    /*  6036 */ "glVertexAttribP3ui\0"
    /*  6055 */ "glVertexAttribP3uiv\0"
    /*  6075 */ "glVertexAttribP4ui\0"
    /*  6094 */ "glVertexAttribP4uiv\0"
    /*  6114 */ "glGetPointerv\0"
    /*  6128 */ "glDebugMessageCallback\0"
    /*  6151 */ "glDebugMessageControl\0"
    /*  6173 */ "glDebugMessageInsert\0"
    /*  6194 */ "glGetDebugMessageLog\0"
    /*  6215 */ "glGetObjectLabel\0"
    /*  6232 */ "glGetObjectPtrLabel\0"
    /*  6252 */ "glObjectLabel\0"
    /*  6266 */ "glObjectPtrLabel\0"
    /*  6283 */ "glPopDebugGroup\0"
    /*  6299 */ "glPushDebugGroup\0"
;
static const uint16_t kFnNameOffsets_GL[] = {
    /*    0 */     0, /* glBlendFunc */
    /*    1 */    12, /* glClear */
    /*    2 */    20, /* glClearColor */
    /*    3 */    33, /* glClearDepth */
    /*    4 */    46, /* glClearStencil */
    /*    5 */    61, /* glColorMask */
    /*    6 */    73, /* glCullFace */
    /*    7 */    84, /* glDepthFunc */
    /*    8 */    96, /* glDepthMask */
    /*    9 */   108, /* glDepthRange */
    /*   10 */   121, /* glDisable */
    /*   11 */   131, /* glDrawBuffer */
    /*   12 */   144, /* glEnable */
    /*   13 */   153, /* glFinish */
    /*   14 */   162, /* glFlush */
    /*   15 */   170, /* glFrontFace */
    /*   16 */   182, /* glGetBooleanv */
    /*   17 */   196, /* glGetDoublev */
    /*   18 */   209, /* glGetError */
    /*   19 */   220, /* glGetFloatv */
    /*   20 */   232, /* glGetIntegerv */
    /*   21 */   246, /* glGetString */
    /*   22 */   258, /* glGetTexImage */
    /*   23 */   272, /* glGetTexLevelParameterfv */
    /*   24 */   297, /* glGetTexLevelParameteriv */
    /*   25 */   322, /* glGetTexParameterfv */
    /*   26 */   342, /* glGetTexParameteriv */
    /*   27 */   362, /* glHint */
    /*   28 */   369, /* glIsEnabled */
    /*   29 */   381, /* glLineWidth */
    /*   30 */   393, /* glLogicOp */
    /*   31 */   403, /* glPixelStoref */
    /*   32 */   417, /* glPixelStorei */
    /*   33 */   431, /* glPointSize */
    /*   34 */   443, /* glPolygonMode */
    /*   35 */   457, /* glReadBuffer */
    /*   36 */   470, /* glReadPixels */
    /*   37 */   483, /* glScissor */
    /*   38 */   493, /* glStencilFunc */
    /*   39 */   507, /* glStencilMask */
    /*   40 */   521, /* glStencilOp */
    /*   41 */   533, /* glTexImage1D */
    /*   42 */   546, /* glTexImage2D */
    /*   43 */   559, /* glTexParameterf */
    /*   44 */   575, /* glTexParameterfv */
    /*   45 */   592, /* glTexParameteri */
    /*   46 */   608, /* glTexParameteriv */
    /*   47 */   625, /* glViewport */
    /*   48 */   636, /* glBindTexture */
    /*   49 */   650, /* glCopyTexImage1D */
    /*   50 */   667, /* glCopyTexImage2D */
    /*   51 */   684, /* glCopyTexSubImage1D */
    /*   52 */   704, /* glCopyTexSubImage2D */
    /*   53 */   724, /* glDeleteTextures */
    /*   54 */   741, /* glDrawArrays */
    /*   55 */   754, /* glDrawElements */
    /*   56 */   769, /* glGenTextures */
    /*   57 */   783, /* glIsTexture */
    /*   58 */   795, /* glPolygonOffset */
    /*   59 */   811, /* glTexSubImage1D */
    /*   60 */   827, /* glTexSubImage2D */
    /*   61 */   843, /* glCopyTexSubImage3D */
    /*   62 */   863, /* glDrawRangeElements */
    /*   63 */   883, /* glTexImage3D */
    /*   64 */   896, /* glTexSubImage3D */
    /*   65 */   912, /* glActiveTexture */
    /*   66 */   928, /* glCompressedTexImage1D */
    /*   67 */   951, /* glCompressedTexImage2D */
    /*   68 */   974, /* glCompressedTexImage3D */
    /*   69 */   997, /* glCompressedTexSubImage1D */
    /*   70 */  1023, /* glCompressedTexSubImage2D */
    /*   71 */  1049, /* glCompressedTexSubImage3D */
    /*   72 */  1075, /* glGetCompressedTexImage */
    /*   73 */  1099, /* glSampleCoverage */
    /*   74 */  1116, /* glBlendColor */
    /*   75 */  1129, /* glBlendEquation */
    /*   76 */  1145, /* glBlendFuncSeparate */
    /*   77 */  1165, /* glMultiDrawArrays */
    /*   78 */  1183, /* glMultiDrawElements */
    /*   79 */  1203, /* glPointParameterf */
    /*   80 */  1221, /* glPointParameterfv */
    /*   81 */  1240, /* glPointParameteri */
    /*   82 */  1258, /* glPointParameteriv */
    /*   83 */  1277, /* glBeginQuery */
    /*   84 */  1290, /* glBindBuffer */
    /*   85 */  1303, /* glBufferData */
    /*   86 */  1316, /* glBufferSubData */
    /*   87 */  1332, /* glDeleteBuffers */
    /*   88 */  1348, /* glDeleteQueries */
    /*   89 */  1364, /* glEndQuery */
    /*   90 */  1375, /* glGenBuffers */
    /*   91 */  1388, /* glGenQueries */
    /*   92 */  1401, /* glGetBufferParameteriv */
    /*   93 */  1424, /* glGetBufferPointerv */
    /*   94 */  1444, /* glGetBufferSubData */
    /*   95 */  1463, /* glGetQueryObjectiv */
    /*   96 */  1482, /* glGetQueryObjectuiv */
    /*   97 */  1502, /* glGetQueryiv */
    /*   98 */  1515, /* glIsBuffer */
    /*   99 */  1526, /* glIsQuery */
    /*  100 */  1536, /* glMapBuffer */
    /*  101 */  1548, /* glUnmapBuffer */
    /*  102 */  1562, /* glAttachShader */
    /*  103 */  1577, /* glBindAttribLocation */
    /*  104 */  1598, /* glBlendEquationSeparate */
    /*  105 */  1622, /* glCompileShader */
    /*  106 */  1638, /* glCreateProgram */
    /*  107 */  1654, /* glCreateShader */
    /*  108 */  1669, /* glDeleteProgram */
    /*  109 */  1685, /* glDeleteShader */
    /*  110 */  1700, /* glDetachShader */
    /*  111 */  1715, /* glDisableVertexAttribArray */
    /*  112 */  1742, /* glDrawBuffers */
    /*  113 */  1756, /* glEnableVertexAttribArray */
    /*  114 */  1782, /* glGetActiveAttrib */
    /*  115 */  1800, /* glGetActiveUniform */
    /*  116 */  1819, /* glGetAttachedShaders */
    /*  117 */  1840, /* glGetAttribLocation */
    /*  118 */  1860, /* glGetProgramInfoLog */
    /*  119 */  1880, /* glGetProgramiv */
    /*  120 */  1895, /* glGetShaderInfoLog */
    /*  121 */  1914, /* glGetShaderSource */
    /*  122 */  1932, /* glGetShaderiv */
    /*  123 */  1946, /* glGetUniformLocation */
    /*  124 */  1967, /* glGetUniformfv */
    /*  125 */  1982, /* glGetUniformiv */
    /*  126 */  1997, /* glGetVertexAttribPointerv */
    /*  127 */  2023, /* glGetVertexAttribdv */
    /*  128 */  2043, /* glGetVertexAttribfv */
    /*  129 */  2063, /* glGetVertexAttribiv */
    /*  130 */  2083, /* glIsProgram */
    /*  131 */  2095, /* glIsShader */
    /*  132 */  2106, /* glLinkProgram */
    /*  133 */  2120, /* glShaderSource */
    /*  134 */  2135, /* glStencilFuncSeparate */
    /*  135 */  2157, /* glStencilMaskSeparate */
    /*  136 */  2179, /* glStencilOpSeparate */
    /*  137 */  2199, /* glUniform1f */
    /*  138 */  2211, /* glUniform1fv */
    /*  139 */  2224, /* glUniform1i */
    /*  140 */  2236, /* glUniform1iv */
    /*  141 */  2249, /* glUniform2f */
    /*  142 */  2261, /* glUniform2fv */
    /*  143 */  2274, /* glUniform2i */
    /*  144 */  2286, /* glUniform2iv */
    /*  145 */  2299, /* glUniform3f */
    /*  146 */  2311, /* glUniform3fv */
    /*  147 */  2324, /* glUniform3i */
    /*  148 */  2336, /* glUniform3iv */
    /*  149 */  2349, /* glUniform4f */
    /*  150 */  2361, /* glUniform4fv */
    /*  151 */  2374, /* glUniform4i */
    /*  152 */  2386, /* glUniform4iv */
    /*  153 */  2399, /* glUniformMatrix2fv */
    /*  154 */  2418, /* glUniformMatrix3fv */
    /*  155 */  2437, /* glUniformMatrix4fv */
    /*  156 */  2456, /* glUseProgram */
    /*  157 */  2469, /* glValidateProgram */
    /*  158 */  2487, /* glVertexAttrib1d */
    /*  159 */  2504, /* glVertexAttrib1dv */
    /*  160 */  2522, /* glVertexAttrib1f */
    /*  161 */  2539, /* glVertexAttrib1fv */
    /*  162 */  2557, /* glVertexAttrib1s */
    /*  163 */  2574, /* glVertexAttrib1sv */
    /*  164 */  2592, /* glVertexAttrib2d */
    /*  165 */  2609, /* glVertexAttrib2dv */
    /*  166 */  2627, /* glVertexAttrib2f */
    /*  167 */  2644, /* glVertexAttrib2fv */
    /*  168 */  2662, /* glVertexAttrib2s */
    /*  169 */  2679, /* glVertexAttrib2sv */
    /*  170 */  2697, /* glVertexAttrib3d */
    /*  171 */  2714, /* glVertexAttrib3dv */
    /*  172 */  2732, /* glVertexAttrib3f */
    /*  173 */  2749, /* glVertexAttrib3fv */
    /*  174 */  2767, /* glVertexAttrib3s */
    /*  175 */  2784, /* glVertexAttrib3sv */
    /*  176 */  2802, /* glVertexAttrib4Nbv */
    /*  177 */  2821, /* glVertexAttrib4Niv */
    /*  178 */  2840, /* glVertexAttrib4Nsv */
    /*  179 */  2859, /* glVertexAttrib4Nub */
    /*  180 */  2878, /* glVertexAttrib4Nubv */
    /*  181 */  2898, /* glVertexAttrib4Nuiv */
    /*  182 */  2918, /* glVertexAttrib4Nusv */
    /*  183 */  2938, /* glVertexAttrib4bv */
    /*  184 */  2956, /* glVertexAttrib4d */
    /*  185 */  2973, /* glVertexAttrib4dv */
    /*  186 */  2991, /* glVertexAttrib4f */
    /*  187 */  3008, /* glVertexAttrib4fv */
    /*  188 */  3026, /* glVertexAttrib4iv */
    /*  189 */  3044, /* glVertexAttrib4s */
    /*  190 */  3061, /* glVertexAttrib4sv */
    /*  191 */  3079, /* glVertexAttrib4ubv */
    /*  192 */  3098, /* glVertexAttrib4uiv */
    /*  193 */  3117, /* glVertexAttrib4usv */
    /*  194 */  3136, /* glVertexAttribPointer */
    /*  195 */  3158, /* glUniformMatrix2x3fv */
    /*  196 */  3179, /* glUniformMatrix2x4fv */
    /*  197 */  3200, /* glUniformMatrix3x2fv */
    /*  198 */  3221, /* glUniformMatrix3x4fv */
    /*  199 */  3242, /* glUniformMatrix4x2fv */
    /*  200 */  3263, /* glUniformMatrix4x3fv */
    /*  201 */  3284, /* glBeginConditionalRender */
    /*  202 */  3309, /* glBeginTransformFeedback */
    /*  203 */  3334, /* glBindFragDataLocation */
    /*  204 */  3357, /* glBindFramebuffer */
    /*  205 */  3375, /* glBindRenderbuffer */
    /*  206 */  3394, /* glBindVertexArray */
    /*  207 */  3412, /* glBlitFramebuffer */
    /*  208 */  3430, /* glCheckFramebufferStatus */
    /*  209 */  3455, /* glClampColor */
    /*  210 */  3468, /* glClearBufferfi */
    /*  211 */  3484, /* glClearBufferfv */
    /*  212 */  3500, /* glClearBufferiv */
    /*  213 */  3516, /* glClearBufferuiv */
    /*  214 */  3533, /* glColorMaski */
    /*  215 */  3546, /* glDeleteFramebuffers */
    /*  216 */  3567, /* glDeleteRenderbuffers */
    /*  217 */  3589, /* glDeleteVertexArrays */
    /*  218 */  3610, /* glDisablei */
    /*  219 */  3621, /* glEnablei */
    /*  220 */  3631, /* glEndConditionalRender */
    /*  221 */  3654, /* glEndTransformFeedback */
    /*  222 */  3677, /* glFlushMappedBufferRange */
    /*  223 */  3702, /* glFramebufferRenderbuffer */
    /*  224 */  3728, /* glFramebufferTexture1D */
    /*  225 */  3751, /* glFramebufferTexture2D */
    /*  226 */  3774, /* glFramebufferTexture3D */
    /*  227 */  3797, /* glFramebufferTextureLayer */
    /*  228 */  3823, /* glGenFramebuffers */
    /*  229 */  3841, /* glGenRenderbuffers */
    /*  230 */  3860, /* glGenVertexArrays */
    /*  231 */  3878, /* glGenerateMipmap */
    /*  232 */  3895, /* glGetBooleani_v */
    /*  233 */  3911, /* glGetFragDataLocation */
    /*  234 */  3933, /* glGetFramebufferAttachmentParameteriv */
    /*  235 */  3971, /* glGetRenderbufferParameteriv */
    /*  236 */  4000, /* glGetStringi */
    /*  237 */  4013, /* glGetTexParameterIiv */
    /*  238 */  4034, /* glGetTexParameterIuiv */
    /*  239 */  4056, /* glGetTransformFeedbackVarying */
    /*  240 */  4086, /* glGetUniformuiv */
    /*  241 */  4102, /* glGetVertexAttribIiv */
    /*  242 */  4123, /* glGetVertexAttribIuiv */
    /*  243 */  4145, /* glIsEnabledi */
    /*  244 */  4158, /* glIsFramebuffer */
    /*  245 */  4174, /* glIsRenderbuffer */
    /*  246 */  4191, /* glIsVertexArray */
    /*  247 */  4207, /* glMapBufferRange */
    /*  248 */  4224, /* glRenderbufferStorage */
    /*  249 */  4246, /* glRenderbufferStorageMultisample */
    /*  250 */  4279, /* glTexParameterIiv */
    /*  251 */  4297, /* glTexParameterIuiv */
    /*  252 */  4316, /* glTransformFeedbackVaryings */
    /*  253 */  4344, /* glUniform1ui */
    /*  254 */  4357, /* glUniform1uiv */
    /*  255 */  4371, /* glUniform2ui */
    /*  256 */  4384, /* glUniform2uiv */
    /*  257 */  4398, /* glUniform3ui */
    /*  258 */  4411, /* glUniform3uiv */
    /*  259 */  4425, /* glUniform4ui */
    /*  260 */  4438, /* glUniform4uiv */
    /*  261 */  4452, /* glVertexAttribI1i */
    /*  262 */  4470, /* glVertexAttribI1iv */
    /*  263 */  4489, /* glVertexAttribI1ui */
    /*  264 */  4508, /* glVertexAttribI1uiv */
    /*  265 */  4528, /* glVertexAttribI2i */
    /*  266 */  4546, /* glVertexAttribI2iv */
    /*  267 */  4565, /* glVertexAttribI2ui */
    /*  268 */  4584, /* glVertexAttribI2uiv */
    /*  269 */  4604, /* glVertexAttribI3i */
    /*  270 */  4622, /* glVertexAttribI3iv */
    /*  271 */  4641, /* glVertexAttribI3ui */
    /*  272 */  4660, /* glVertexAttribI3uiv */
    /*  273 */  4680, /* glVertexAttribI4bv */
    /*  274 */  4699, /* glVertexAttribI4i */
    /*  275 */  4717, /* glVertexAttribI4iv */
    /*  276 */  4736, /* glVertexAttribI4sv */
    /*  277 */  4755, /* glVertexAttribI4ubv */
    /*  278 */  4775, /* glVertexAttribI4ui */
    /*  279 */  4794, /* glVertexAttribI4uiv */
    /*  280 */  4814, /* glVertexAttribI4usv */
    /*  281 */  4834, /* glVertexAttribIPointer */
    /*  282 */  4857, /* glBindBufferBase */
    /*  283 */  4874, /* glBindBufferRange */
    /*  284 */  4892, /* glGetIntegeri_v */
    /*  285 */  4908, /* glCopyBufferSubData */
    /*  286 */  4928, /* glDrawArraysInstanced */
    /*  287 */  4950, /* glDrawElementsInstanced */
    /*  288 */  4974, /* glGetActiveUniformBlockName */
    /*  289 */  5002, /* glGetActiveUniformBlockiv */
    /*  290 */  5028, /* glGetActiveUniformName */
    /*  291 */  5051, /* glGetActiveUniformsiv */
    /*  292 */  5073, /* glGetUniformBlockIndex */
    /*  293 */  5096, /* glGetUniformIndices */
    /*  294 */  5116, /* glPrimitiveRestartIndex */
    /*  295 */  5140, /* glTexBuffer */
    /*  296 */  5152, /* glUniformBlockBinding */
    /*  297 */  5174, /* glDrawElementsBaseVertex */
    /*  298 */  5199, /* glDrawElementsInstancedBaseVertex */
    /*  299 */  5233, /* glDrawRangeElementsBaseVertex */
    /*  300 */  5263, /* glFramebufferTexture */
    /*  301 */  5284, /* glGetBufferParameteri64v */
    /*  302 */  5309, /* glGetInteger64i_v */
    /*  303 */  5327, /* glGetMultisamplefv */
    /*  304 */  5346, /* glMultiDrawElementsBaseVertex */
    /*  305 */  5376, /* glProvokingVertex */
    /*  306 */  5394, /* glSampleMaski */
    /*  307 */  5408, /* glTexImage2DMultisample */
    /*  308 */  5432, /* glTexImage3DMultisample */
    /*  309 */  5456, /* glClientWaitSync */
    /*  310 */  5473, /* glDeleteSync */
    /*  311 */  5486, /* glFenceSync */
    /*  312 */  5498, /* glGetInteger64v */
    /*  313 */  5514, /* glGetSynciv */
    /*  314 */  5526, /* glIsSync */
    /*  315 */  5535, /* glWaitSync */
    /*  316 */  5546, /* glBindFragDataLocationIndexed */
    /*  317 */  5576, /* glBindSampler */
    /*  318 */  5590, /* glDeleteSamplers */
    /*  319 */  5607, /* glGenSamplers */
    /*  320 */  5621, /* glGetFragDataIndex */
    /*  321 */  5640, /* glGetQueryObjecti64v */
    /*  322 */  5661, /* glGetQueryObjectui64v */
    /*  323 */  5683, /* glGetSamplerParameterIiv */
    /*  324 */  5708, /* glGetSamplerParameterIuiv */
    /*  325 */  5734, /* glGetSamplerParameterfv */
    /*  326 */  5758, /* glGetSamplerParameteriv */
    /*  327 */  5782, /* glIsSampler */
    /*  328 */  5794, /* glQueryCounter */
    /*  329 */  5809, /* glSamplerParameterIiv */
    /*  330 */  5831, /* glSamplerParameterIuiv */
    /*  331 */  5854, /* glSamplerParameterf */
    /*  332 */  5874, /* glSamplerParameterfv */
    /*  333 */  5895, /* glSamplerParameteri */
    /*  334 */  5915, /* glSamplerParameteriv */
    /*  335 */  5936, /* glVertexAttribDivisor */
    /*  336 */  5958, /* glVertexAttribP1ui */
    /*  337 */  5977, /* glVertexAttribP1uiv */
    /*  338 */  5997, /* glVertexAttribP2ui */
    /*  339 */  6016, /* glVertexAttribP2uiv */
    /*  340 */  6036, /* glVertexAttribP3ui */
    /*  341 */  6055, /* glVertexAttribP3uiv */
    /*  342 */  6075, /* glVertexAttribP4ui */
    /*  343 */  6094, /* glVertexAttribP4uiv */
    /*  344 */  6114, /* glGetPointerv */
    /*  345 */  6128, /* glDebugMessageCallback */
    /*  346 */  6151, /* glDebugMessageControl */
    /*  347 */  6173, /* glDebugMessageInsert */
    /*  348 */  6194, /* glGetDebugMessageLog */
    /*  349 */  6215, /* glGetObjectLabel */
    /*  350 */  6232, /* glGetObjectPtrLabel */
    /*  351 */  6252, /* glObjectLabel */
    /*  352 */  6266, /* glObjectPtrLabel */
    /*  353 */  6283, /* glPopDebugGroup */
    /*  354 */  6299 /* glPushDebugGroup */
};
/* ---- Extension hash table ------------------------------------------------
   One XXH3-64 hash per extension, in extArray index order.
   Pre-baked at generator time with the same algorithm used at load time. */
static const uint64_t kExtHashes_GL[] = {
    /*    0 */ 0x0d3c113e7ffc3be4ULL, /* GL_ARB_sync */
    /*    1 */ 0x5e0c5b9607ac8784ULL  /* GL_KHR_debug */
};

/* ---- Feature PFN range table ---------------------------------------------
 * Each entry maps one feature (by featArray index) to a contiguous run of
 * pfnArray slots. The loader iterates this table and bulk-loads the run
 * when featArray[entry.extension] is set.
 */
static const GloamPfnRange_t kFeatPfnRanges_GL[] = {
    {    0,    0,   48 }, /* GL_VERSION_1_0 */
    {    1,   48,   13 }, /* GL_VERSION_1_1 */
    {    1,  344,    1 }, /* GL_VERSION_1_1 */
    {    2,   61,    4 }, /* GL_VERSION_1_2 */
    {    3,   65,    9 }, /* GL_VERSION_1_3 */
    {    4,   74,    9 }, /* GL_VERSION_1_4 */
    {    5,   83,   19 }, /* GL_VERSION_1_5 */
    {    6,  102,   93 }, /* GL_VERSION_2_0 */
    {    7,  195,    6 }, /* GL_VERSION_2_1 */
    {    8,  201,   84 }, /* GL_VERSION_3_0 */
    {    9,  282,   15 }, /* GL_VERSION_3_1 */
    {   10,  297,   19 }, /* GL_VERSION_3_2 */
    {   11,  316,   28 }, /* GL_VERSION_3_3 */
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

/* Query driver-reported extension names, hash them, and fill *out_exts.
 * Returns 1 on success, 0 on failure. Caller must free(*out_exts).
 */
static int gloam_gl_get_extensions(GloamGLContext *context, uint64_t **out_exts, uint32_t *out_num_exts)
{
#if defined(GL_ES_VERSION_3_0) || defined(GL_VERSION_3_0)
    /* Modern path: glGetIntegerv(GL_NUM_EXTENSIONS) + glGetStringi. */
    if (context->GetStringi != NULL && context->GetIntegerv != NULL) {
        GLint n = 0;
        uint32_t num_exts, i;
        uint64_t *exts;
        context->GetIntegerv(GL_NUM_EXTENSIONS, &n);
        num_exts = (uint32_t)n;
        if (num_exts == 0) {
            *out_exts = NULL;
            *out_num_exts = 0;
            return 1;
        }
        exts = (uint64_t *)calloc(num_exts, sizeof(uint64_t));
        if (!exts)
            return 0;
        for (i = 0; i < num_exts; ++i) {
            const char *name = (const char *)context->GetStringi(GL_EXTENSIONS, i);
            if (name)
                exts[i] = gloam_hash_string(name, strlen(name));
        }
        gloam_sort_hashes(exts, num_exts);
        *out_exts     = exts;
        *out_num_exts = num_exts;
        return 1;
    }
#endif
    /* Legacy path: glGetString(GL_EXTENSIONS) — space-separated string. */
    {
        const char *ext_str;
        if (!context->GetString)
            return 0;
        ext_str = (const char *)context->GetString(GL_EXTENSIONS);
        if (!ext_str)
            return 0;
        return gloam_hash_ext_string(ext_str, out_exts, out_num_exts);
    }
}

/* ==========================================================================
 * Per-API sections
 * ==========================================================================
 */

/* --------------------------------------------------------------------------
 * API: gl
 * --------------------------------------------------------------------------
 */

/* Extension index subset for gl: extArray indices this API supports. */
static const uint16_t kExtIdx_gl[] = {
       0, /* GL_ARB_sync */
       1, /* GL_KHR_debug */
};

/* Extension PFN range table for gl. */
static const GloamPfnRange_t kExtPfnRanges_gl[] = {
    {    0,  309,    7 }, /* GL_ARB_sync */
    {    1,  344,   11 }, /* GL_KHR_debug */
};

/* Search pre-baked kExtHashes_GL against the sorted driver hash list and set
 * extArray flags for every matching extension.
 */
static int gloam_gl_find_extensions_gl(GloamGLContext *context)
{
    uint64_t *exts = NULL;
    uint32_t  num_exts = 0, i;

    if (!gloam_gl_get_extensions(context, &exts, &num_exts))
        return 0;

    for (i = 0; i < GLOAM_ARRAYSIZE(kExtIdx_gl); ++i) {
        const uint16_t extIdx = kExtIdx_gl[i];
        context->extArray[extIdx] = (unsigned char)gloam_hash_search(exts, num_exts, kExtHashes_GL[extIdx]);
    }

    free(exts);
    return 1;
}

/* Parse the GL_VERSION string and set featArray entries for this API. */
static int gloam_gl_find_core_gl(GloamGLContext *context)
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

    context->VERSION_1_0 = (unsigned char)(version_value >= 0x0100);
    context->VERSION_1_1 = (unsigned char)(version_value >= 0x0101);
    context->VERSION_1_2 = (unsigned char)(version_value >= 0x0102);
    context->VERSION_1_3 = (unsigned char)(version_value >= 0x0103);
    context->VERSION_1_4 = (unsigned char)(version_value >= 0x0104);
    context->VERSION_1_5 = (unsigned char)(version_value >= 0x0105);
    context->VERSION_2_0 = (unsigned char)(version_value >= 0x0200);
    context->VERSION_2_1 = (unsigned char)(version_value >= 0x0201);
    context->VERSION_3_0 = (unsigned char)(version_value >= 0x0300);
    context->VERSION_3_1 = (unsigned char)(version_value >= 0x0301);
    context->VERSION_3_2 = (unsigned char)(version_value >= 0x0302);
    context->VERSION_3_3 = (unsigned char)(version_value >= 0x0303);

    return (int)version_value;
}

int gloamLoadGLContext(GloamGLContext *context, GloamLoadFunc getProcAddr)
{
    int version;
    uint32_t i;
    GLOAM_UNUSED(kFnCount_GL);

    memset(context, 0, sizeof(*context));

    /* Bootstrap: glGetString must be loaded before find_core can run. */
    context->GetString = (PFNGLGETSTRINGPROC)getProcAddr("glGetString");
    if (!context->GetString)
        return 0;

    version = gloam_gl_find_core_gl(context);
    if (!version)
        return 0;

    /* Load PFNs for each enabled feature via the range table. */
    for (i = 0; i < GLOAM_ARRAYSIZE(kFeatPfnRanges_GL); ++i) {
        const GloamPfnRange_t *r = &kFeatPfnRanges_GL[i];
        if (context->featArray[r->extension])
            gloam_load_pfn_range_gl(context, getProcAddr, r->start, r->count);
    }

    if (!gloam_gl_find_extensions_gl(context))
        return 0;

    /* Load PFNs for each detected extension via the range table. */
    for (i = 0; i < GLOAM_ARRAYSIZE(kExtPfnRanges_gl); ++i) {
        const GloamPfnRange_t *r = &kExtPfnRanges_gl[i];
        if (context->extArray[r->extension])
            gloam_load_pfn_range_gl(context, getProcAddr, r->start, r->count);
    }

    return version;
}

int gloamLoadGL(GloamLoadFunc getProcAddr)
{
    return gloamLoadGLContext(&gloam_gl_context, getProcAddr);
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

int gloamLoaderLoadGLContext(GloamGLContext *context)
{
    int did_open = 0;
    int version;
    void *handle;

    static const char * const kLibNames[] = {
#if defined(__APPLE__)
        "../Frameworks/OpenGL.framework/OpenGL",
        "/Library/Frameworks/OpenGL.framework/OpenGL",
        "/System/Library/Frameworks/OpenGL.framework/OpenGL",
        "/System/Library/Frameworks/OpenGL.framework/Versions/Current/OpenGL",
#elif defined(GLOAM_PLATFORM_WINDOWS)
        "opengl32.dll",
#else
        "libGL.so.1",
        "libGL.so",
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


    version = gloamLoadGLContext(context, gloam_gl_get_proc);

    gloam_gl_load_state.handle = NULL;

    if (!version && did_open) {
        gloam_dlclose(handle);
        return 0;
    }

    context->gloam_loader_handle = handle;
    context->gloam_loader_owns_handle |= (uint8_t)did_open;

    return version;
}

int gloamLoaderLoadGL(void)
{
    return gloamLoaderLoadGLContext(&gloam_gl_context);
}

void gloamLoaderUnloadGLContext(GloamGLContext *context)
{
    if (context->gloam_loader_handle && context->gloam_loader_owns_handle) {
        gloam_dlclose(context->gloam_loader_handle);
    }
    gloamLoaderResetGLContext(context);
}

void gloamLoaderUnloadGL(void)
{
    gloamLoaderUnloadGLContext(&gloam_gl_context);
}

void gloamLoaderResetGLContext(GloamGLContext *context)
{
    memset(context, 0, sizeof(*context));
}

void gloamLoaderResetGL(void)
{
    gloamLoaderResetGLContext(&gloam_gl_context);
}
