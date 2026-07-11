#include <gloam/vk.h>

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

/* Bijective alias pair: if canonical slot is null but secondary is loaded
 * (or vice versa), the loaded pointer is propagated to both slots.
 */
typedef struct {
    uint16_t first;  /* canonical (shortest name) pfnArray index */
    uint16_t second; /* alias pfnArray index                     */
} GloamAliasPair_t;

#endif /* GLOAM_IMPL_UTIL_C_ */



/*
 * Vulkan command scope — determines which vkGet*ProcAddr function and handle
 * is used when loading each command's function pointer.
 */
typedef enum {
    GloamCommandScopeUnknown  = 0,
    GloamCommandScopeGlobal   = 1,
    GloamCommandScopeInstance = 2,
    GloamCommandScopeDevice   = 3
} GloamCommandScope;


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


static const char * const gloam_vk_lib_names[] = {
#if defined(__APPLE__)
    "libvulkan.dylib", "libvulkan.1.dylib", "libMoltenVK.dylib",
#elif defined(GLOAM_PLATFORM_WINDOWS)
    "vulkan-1.dll",
#else
    "libvulkan.so.1", "libvulkan.so",
#endif
};
/* ---- Global context (zero-initialised at program startup) ---------------- */
#ifdef __cplusplus
GloamVulkanContext gloam_vk_context = {};
#else
GloamVulkanContext gloam_vk_context = { 0 };
#endif

/* ---- Function name table -------------------------------------------------
 * Command names stored as a single NUL-terminated string blob with a parallel
 * offset table for O(1) indexing. This avoids one pointer (8 bytes on 64-bit)
 * plus one relocation entry (~24 bytes in PIC builds) per command compared to
 * the traditional const char * const [] approach.
 */
static const uint32_t kFnCount_Vulkan = 137;

static const char kFnNameData_Vulkan[] =
    /*     0 */ "vkAllocateCommandBuffers\0"
    /*    25 */ "vkAllocateDescriptorSets\0"
    /*    50 */ "vkAllocateMemory\0"
    /*    67 */ "vkBeginCommandBuffer\0"
    /*    88 */ "vkBindBufferMemory\0"
    /*   107 */ "vkBindImageMemory\0"
    /*   125 */ "vkCmdBeginQuery\0"
    /*   141 */ "vkCmdBeginRenderPass\0"
    /*   162 */ "vkCmdBindDescriptorSets\0"
    /*   186 */ "vkCmdBindIndexBuffer\0"
    /*   207 */ "vkCmdBindPipeline\0"
    /*   225 */ "vkCmdBindVertexBuffers\0"
    /*   248 */ "vkCmdBlitImage\0"
    /*   263 */ "vkCmdClearAttachments\0"
    /*   285 */ "vkCmdClearColorImage\0"
    /*   306 */ "vkCmdClearDepthStencilImage\0"
    /*   334 */ "vkCmdCopyBuffer\0"
    /*   350 */ "vkCmdCopyBufferToImage\0"
    /*   373 */ "vkCmdCopyImage\0"
    /*   388 */ "vkCmdCopyImageToBuffer\0"
    /*   411 */ "vkCmdCopyQueryPoolResults\0"
    /*   437 */ "vkCmdDispatch\0"
    /*   451 */ "vkCmdDispatchIndirect\0"
    /*   473 */ "vkCmdDraw\0"
    /*   483 */ "vkCmdDrawIndexed\0"
    /*   500 */ "vkCmdDrawIndexedIndirect\0"
    /*   525 */ "vkCmdDrawIndirect\0"
    /*   543 */ "vkCmdEndQuery\0"
    /*   557 */ "vkCmdEndRenderPass\0"
    /*   576 */ "vkCmdExecuteCommands\0"
    /*   597 */ "vkCmdFillBuffer\0"
    /*   613 */ "vkCmdNextSubpass\0"
    /*   630 */ "vkCmdPipelineBarrier\0"
    /*   651 */ "vkCmdPushConstants\0"
    /*   670 */ "vkCmdResetEvent\0"
    /*   686 */ "vkCmdResetQueryPool\0"
    /*   706 */ "vkCmdResolveImage\0"
    /*   724 */ "vkCmdSetBlendConstants\0"
    /*   747 */ "vkCmdSetDepthBias\0"
    /*   765 */ "vkCmdSetDepthBounds\0"
    /*   785 */ "vkCmdSetEvent\0"
    /*   799 */ "vkCmdSetLineWidth\0"
    /*   817 */ "vkCmdSetScissor\0"
    /*   833 */ "vkCmdSetStencilCompareMask\0"
    /*   860 */ "vkCmdSetStencilReference\0"
    /*   885 */ "vkCmdSetStencilWriteMask\0"
    /*   910 */ "vkCmdSetViewport\0"
    /*   927 */ "vkCmdUpdateBuffer\0"
    /*   945 */ "vkCmdWaitEvents\0"
    /*   961 */ "vkCmdWriteTimestamp\0"
    /*   981 */ "vkCreateBuffer\0"
    /*   996 */ "vkCreateBufferView\0"
    /*  1015 */ "vkCreateCommandPool\0"
    /*  1035 */ "vkCreateComputePipelines\0"
    /*  1060 */ "vkCreateDescriptorPool\0"
    /*  1083 */ "vkCreateDescriptorSetLayout\0"
    /*  1111 */ "vkCreateDevice\0"
    /*  1126 */ "vkCreateEvent\0"
    /*  1140 */ "vkCreateFence\0"
    /*  1154 */ "vkCreateFramebuffer\0"
    /*  1174 */ "vkCreateGraphicsPipelines\0"
    /*  1200 */ "vkCreateImage\0"
    /*  1214 */ "vkCreateImageView\0"
    /*  1232 */ "vkCreateInstance\0"
    /*  1249 */ "vkCreatePipelineCache\0"
    /*  1271 */ "vkCreatePipelineLayout\0"
    /*  1294 */ "vkCreateQueryPool\0"
    /*  1312 */ "vkCreateRenderPass\0"
    /*  1331 */ "vkCreateSampler\0"
    /*  1347 */ "vkCreateSemaphore\0"
    /*  1365 */ "vkCreateShaderModule\0"
    /*  1386 */ "vkDestroyBuffer\0"
    /*  1402 */ "vkDestroyBufferView\0"
    /*  1422 */ "vkDestroyCommandPool\0"
    /*  1443 */ "vkDestroyDescriptorPool\0"
    /*  1467 */ "vkDestroyDescriptorSetLayout\0"
    /*  1496 */ "vkDestroyDevice\0"
    /*  1512 */ "vkDestroyEvent\0"
    /*  1527 */ "vkDestroyFence\0"
    /*  1542 */ "vkDestroyFramebuffer\0"
    /*  1563 */ "vkDestroyImage\0"
    /*  1578 */ "vkDestroyImageView\0"
    /*  1597 */ "vkDestroyInstance\0"
    /*  1615 */ "vkDestroyPipeline\0"
    /*  1633 */ "vkDestroyPipelineCache\0"
    /*  1656 */ "vkDestroyPipelineLayout\0"
    /*  1680 */ "vkDestroyQueryPool\0"
    /*  1699 */ "vkDestroyRenderPass\0"
    /*  1719 */ "vkDestroySampler\0"
    /*  1736 */ "vkDestroySemaphore\0"
    /*  1755 */ "vkDestroyShaderModule\0"
    /*  1777 */ "vkDeviceWaitIdle\0"
    /*  1794 */ "vkEndCommandBuffer\0"
    /*  1813 */ "vkEnumerateDeviceExtensionProperties\0"
    /*  1850 */ "vkEnumerateDeviceLayerProperties\0"
    /*  1883 */ "vkEnumerateInstanceExtensionProperties\0"
    /*  1922 */ "vkEnumerateInstanceLayerProperties\0"
    /*  1957 */ "vkEnumeratePhysicalDevices\0"
    /*  1984 */ "vkFlushMappedMemoryRanges\0"
    /*  2010 */ "vkFreeCommandBuffers\0"
    /*  2031 */ "vkFreeDescriptorSets\0"
    /*  2052 */ "vkFreeMemory\0"
    /*  2065 */ "vkGetBufferMemoryRequirements\0"
    /*  2095 */ "vkGetDeviceMemoryCommitment\0"
    /*  2123 */ "vkGetDeviceProcAddr\0"
    /*  2143 */ "vkGetDeviceQueue\0"
    /*  2160 */ "vkGetEventStatus\0"
    /*  2177 */ "vkGetFenceStatus\0"
    /*  2194 */ "vkGetImageMemoryRequirements\0"
    /*  2223 */ "vkGetImageSparseMemoryRequirements\0"
    /*  2258 */ "vkGetImageSubresourceLayout\0"
    /*  2286 */ "vkGetInstanceProcAddr\0"
    /*  2308 */ "vkGetPhysicalDeviceFeatures\0"
    /*  2336 */ "vkGetPhysicalDeviceFormatProperties\0"
    /*  2372 */ "vkGetPhysicalDeviceImageFormatProperties\0"
    /*  2413 */ "vkGetPhysicalDeviceMemoryProperties\0"
    /*  2449 */ "vkGetPhysicalDeviceProperties\0"
    /*  2479 */ "vkGetPhysicalDeviceQueueFamilyProperties\0"
    /*  2520 */ "vkGetPhysicalDeviceSparseImageFormatProperties\0"
    /*  2567 */ "vkGetPipelineCacheData\0"
    /*  2590 */ "vkGetQueryPoolResults\0"
    /*  2612 */ "vkGetRenderAreaGranularity\0"
    /*  2639 */ "vkInvalidateMappedMemoryRanges\0"
    /*  2670 */ "vkMapMemory\0"
    /*  2682 */ "vkMergePipelineCaches\0"
    /*  2704 */ "vkQueueBindSparse\0"
    /*  2722 */ "vkQueueSubmit\0"
    /*  2736 */ "vkQueueWaitIdle\0"
    /*  2752 */ "vkResetCommandBuffer\0"
    /*  2773 */ "vkResetCommandPool\0"
    /*  2792 */ "vkResetDescriptorPool\0"
    /*  2814 */ "vkResetEvent\0"
    /*  2827 */ "vkResetFences\0"
    /*  2841 */ "vkSetEvent\0"
    /*  2852 */ "vkUnmapMemory\0"
    /*  2866 */ "vkUpdateDescriptorSets\0"
    /*  2889 */ "vkWaitForFences\0"
;
static const uint16_t kFnNameOffsets_Vulkan[] = {
    /*    0 */     0, /* vkAllocateCommandBuffers */
    /*    1 */    25, /* vkAllocateDescriptorSets */
    /*    2 */    50, /* vkAllocateMemory */
    /*    3 */    67, /* vkBeginCommandBuffer */
    /*    4 */    88, /* vkBindBufferMemory */
    /*    5 */   107, /* vkBindImageMemory */
    /*    6 */   125, /* vkCmdBeginQuery */
    /*    7 */   141, /* vkCmdBeginRenderPass */
    /*    8 */   162, /* vkCmdBindDescriptorSets */
    /*    9 */   186, /* vkCmdBindIndexBuffer */
    /*   10 */   207, /* vkCmdBindPipeline */
    /*   11 */   225, /* vkCmdBindVertexBuffers */
    /*   12 */   248, /* vkCmdBlitImage */
    /*   13 */   263, /* vkCmdClearAttachments */
    /*   14 */   285, /* vkCmdClearColorImage */
    /*   15 */   306, /* vkCmdClearDepthStencilImage */
    /*   16 */   334, /* vkCmdCopyBuffer */
    /*   17 */   350, /* vkCmdCopyBufferToImage */
    /*   18 */   373, /* vkCmdCopyImage */
    /*   19 */   388, /* vkCmdCopyImageToBuffer */
    /*   20 */   411, /* vkCmdCopyQueryPoolResults */
    /*   21 */   437, /* vkCmdDispatch */
    /*   22 */   451, /* vkCmdDispatchIndirect */
    /*   23 */   473, /* vkCmdDraw */
    /*   24 */   483, /* vkCmdDrawIndexed */
    /*   25 */   500, /* vkCmdDrawIndexedIndirect */
    /*   26 */   525, /* vkCmdDrawIndirect */
    /*   27 */   543, /* vkCmdEndQuery */
    /*   28 */   557, /* vkCmdEndRenderPass */
    /*   29 */   576, /* vkCmdExecuteCommands */
    /*   30 */   597, /* vkCmdFillBuffer */
    /*   31 */   613, /* vkCmdNextSubpass */
    /*   32 */   630, /* vkCmdPipelineBarrier */
    /*   33 */   651, /* vkCmdPushConstants */
    /*   34 */   670, /* vkCmdResetEvent */
    /*   35 */   686, /* vkCmdResetQueryPool */
    /*   36 */   706, /* vkCmdResolveImage */
    /*   37 */   724, /* vkCmdSetBlendConstants */
    /*   38 */   747, /* vkCmdSetDepthBias */
    /*   39 */   765, /* vkCmdSetDepthBounds */
    /*   40 */   785, /* vkCmdSetEvent */
    /*   41 */   799, /* vkCmdSetLineWidth */
    /*   42 */   817, /* vkCmdSetScissor */
    /*   43 */   833, /* vkCmdSetStencilCompareMask */
    /*   44 */   860, /* vkCmdSetStencilReference */
    /*   45 */   885, /* vkCmdSetStencilWriteMask */
    /*   46 */   910, /* vkCmdSetViewport */
    /*   47 */   927, /* vkCmdUpdateBuffer */
    /*   48 */   945, /* vkCmdWaitEvents */
    /*   49 */   961, /* vkCmdWriteTimestamp */
    /*   50 */   981, /* vkCreateBuffer */
    /*   51 */   996, /* vkCreateBufferView */
    /*   52 */  1015, /* vkCreateCommandPool */
    /*   53 */  1035, /* vkCreateComputePipelines */
    /*   54 */  1060, /* vkCreateDescriptorPool */
    /*   55 */  1083, /* vkCreateDescriptorSetLayout */
    /*   56 */  1111, /* vkCreateDevice */
    /*   57 */  1126, /* vkCreateEvent */
    /*   58 */  1140, /* vkCreateFence */
    /*   59 */  1154, /* vkCreateFramebuffer */
    /*   60 */  1174, /* vkCreateGraphicsPipelines */
    /*   61 */  1200, /* vkCreateImage */
    /*   62 */  1214, /* vkCreateImageView */
    /*   63 */  1232, /* vkCreateInstance */
    /*   64 */  1249, /* vkCreatePipelineCache */
    /*   65 */  1271, /* vkCreatePipelineLayout */
    /*   66 */  1294, /* vkCreateQueryPool */
    /*   67 */  1312, /* vkCreateRenderPass */
    /*   68 */  1331, /* vkCreateSampler */
    /*   69 */  1347, /* vkCreateSemaphore */
    /*   70 */  1365, /* vkCreateShaderModule */
    /*   71 */  1386, /* vkDestroyBuffer */
    /*   72 */  1402, /* vkDestroyBufferView */
    /*   73 */  1422, /* vkDestroyCommandPool */
    /*   74 */  1443, /* vkDestroyDescriptorPool */
    /*   75 */  1467, /* vkDestroyDescriptorSetLayout */
    /*   76 */  1496, /* vkDestroyDevice */
    /*   77 */  1512, /* vkDestroyEvent */
    /*   78 */  1527, /* vkDestroyFence */
    /*   79 */  1542, /* vkDestroyFramebuffer */
    /*   80 */  1563, /* vkDestroyImage */
    /*   81 */  1578, /* vkDestroyImageView */
    /*   82 */  1597, /* vkDestroyInstance */
    /*   83 */  1615, /* vkDestroyPipeline */
    /*   84 */  1633, /* vkDestroyPipelineCache */
    /*   85 */  1656, /* vkDestroyPipelineLayout */
    /*   86 */  1680, /* vkDestroyQueryPool */
    /*   87 */  1699, /* vkDestroyRenderPass */
    /*   88 */  1719, /* vkDestroySampler */
    /*   89 */  1736, /* vkDestroySemaphore */
    /*   90 */  1755, /* vkDestroyShaderModule */
    /*   91 */  1777, /* vkDeviceWaitIdle */
    /*   92 */  1794, /* vkEndCommandBuffer */
    /*   93 */  1813, /* vkEnumerateDeviceExtensionProperties */
    /*   94 */  1850, /* vkEnumerateDeviceLayerProperties */
    /*   95 */  1883, /* vkEnumerateInstanceExtensionProperties */
    /*   96 */  1922, /* vkEnumerateInstanceLayerProperties */
    /*   97 */  1957, /* vkEnumeratePhysicalDevices */
    /*   98 */  1984, /* vkFlushMappedMemoryRanges */
    /*   99 */  2010, /* vkFreeCommandBuffers */
    /*  100 */  2031, /* vkFreeDescriptorSets */
    /*  101 */  2052, /* vkFreeMemory */
    /*  102 */  2065, /* vkGetBufferMemoryRequirements */
    /*  103 */  2095, /* vkGetDeviceMemoryCommitment */
    /*  104 */  2123, /* vkGetDeviceProcAddr */
    /*  105 */  2143, /* vkGetDeviceQueue */
    /*  106 */  2160, /* vkGetEventStatus */
    /*  107 */  2177, /* vkGetFenceStatus */
    /*  108 */  2194, /* vkGetImageMemoryRequirements */
    /*  109 */  2223, /* vkGetImageSparseMemoryRequirements */
    /*  110 */  2258, /* vkGetImageSubresourceLayout */
    /*  111 */  2286, /* vkGetInstanceProcAddr */
    /*  112 */  2308, /* vkGetPhysicalDeviceFeatures */
    /*  113 */  2336, /* vkGetPhysicalDeviceFormatProperties */
    /*  114 */  2372, /* vkGetPhysicalDeviceImageFormatProperties */
    /*  115 */  2413, /* vkGetPhysicalDeviceMemoryProperties */
    /*  116 */  2449, /* vkGetPhysicalDeviceProperties */
    /*  117 */  2479, /* vkGetPhysicalDeviceQueueFamilyProperties */
    /*  118 */  2520, /* vkGetPhysicalDeviceSparseImageFormatProperties */
    /*  119 */  2567, /* vkGetPipelineCacheData */
    /*  120 */  2590, /* vkGetQueryPoolResults */
    /*  121 */  2612, /* vkGetRenderAreaGranularity */
    /*  122 */  2639, /* vkInvalidateMappedMemoryRanges */
    /*  123 */  2670, /* vkMapMemory */
    /*  124 */  2682, /* vkMergePipelineCaches */
    /*  125 */  2704, /* vkQueueBindSparse */
    /*  126 */  2722, /* vkQueueSubmit */
    /*  127 */  2736, /* vkQueueWaitIdle */
    /*  128 */  2752, /* vkResetCommandBuffer */
    /*  129 */  2773, /* vkResetCommandPool */
    /*  130 */  2792, /* vkResetDescriptorPool */
    /*  131 */  2814, /* vkResetEvent */
    /*  132 */  2827, /* vkResetFences */
    /*  133 */  2841, /* vkSetEvent */
    /*  134 */  2852, /* vkUnmapMemory */
    /*  135 */  2866, /* vkUpdateDescriptorSets */
    /*  136 */  2889 /* vkWaitForFences */
};
/* ---- Command scope table -------------------------------------------------
 * Indexed in lockstep with kFnNameOffsets_Vulkan[].
 * Each entry is the GloamCommandScope value for that slot — stored as uint8_t so
 * the whole table is one byte per command (compares well against the
 * alternatives: a parallel pointer array or a switch inside the loop).
 */
static const uint8_t kCommandScopes_Vulkan[] = {
    /*    0 */ GloamCommandScopeDevice  , /* vkAllocateCommandBuffers */
    /*    1 */ GloamCommandScopeDevice  , /* vkAllocateDescriptorSets */
    /*    2 */ GloamCommandScopeDevice  , /* vkAllocateMemory */
    /*    3 */ GloamCommandScopeDevice  , /* vkBeginCommandBuffer */
    /*    4 */ GloamCommandScopeDevice  , /* vkBindBufferMemory */
    /*    5 */ GloamCommandScopeDevice  , /* vkBindImageMemory */
    /*    6 */ GloamCommandScopeDevice  , /* vkCmdBeginQuery */
    /*    7 */ GloamCommandScopeDevice  , /* vkCmdBeginRenderPass */
    /*    8 */ GloamCommandScopeDevice  , /* vkCmdBindDescriptorSets */
    /*    9 */ GloamCommandScopeDevice  , /* vkCmdBindIndexBuffer */
    /*   10 */ GloamCommandScopeDevice  , /* vkCmdBindPipeline */
    /*   11 */ GloamCommandScopeDevice  , /* vkCmdBindVertexBuffers */
    /*   12 */ GloamCommandScopeDevice  , /* vkCmdBlitImage */
    /*   13 */ GloamCommandScopeDevice  , /* vkCmdClearAttachments */
    /*   14 */ GloamCommandScopeDevice  , /* vkCmdClearColorImage */
    /*   15 */ GloamCommandScopeDevice  , /* vkCmdClearDepthStencilImage */
    /*   16 */ GloamCommandScopeDevice  , /* vkCmdCopyBuffer */
    /*   17 */ GloamCommandScopeDevice  , /* vkCmdCopyBufferToImage */
    /*   18 */ GloamCommandScopeDevice  , /* vkCmdCopyImage */
    /*   19 */ GloamCommandScopeDevice  , /* vkCmdCopyImageToBuffer */
    /*   20 */ GloamCommandScopeDevice  , /* vkCmdCopyQueryPoolResults */
    /*   21 */ GloamCommandScopeDevice  , /* vkCmdDispatch */
    /*   22 */ GloamCommandScopeDevice  , /* vkCmdDispatchIndirect */
    /*   23 */ GloamCommandScopeDevice  , /* vkCmdDraw */
    /*   24 */ GloamCommandScopeDevice  , /* vkCmdDrawIndexed */
    /*   25 */ GloamCommandScopeDevice  , /* vkCmdDrawIndexedIndirect */
    /*   26 */ GloamCommandScopeDevice  , /* vkCmdDrawIndirect */
    /*   27 */ GloamCommandScopeDevice  , /* vkCmdEndQuery */
    /*   28 */ GloamCommandScopeDevice  , /* vkCmdEndRenderPass */
    /*   29 */ GloamCommandScopeDevice  , /* vkCmdExecuteCommands */
    /*   30 */ GloamCommandScopeDevice  , /* vkCmdFillBuffer */
    /*   31 */ GloamCommandScopeDevice  , /* vkCmdNextSubpass */
    /*   32 */ GloamCommandScopeDevice  , /* vkCmdPipelineBarrier */
    /*   33 */ GloamCommandScopeDevice  , /* vkCmdPushConstants */
    /*   34 */ GloamCommandScopeDevice  , /* vkCmdResetEvent */
    /*   35 */ GloamCommandScopeDevice  , /* vkCmdResetQueryPool */
    /*   36 */ GloamCommandScopeDevice  , /* vkCmdResolveImage */
    /*   37 */ GloamCommandScopeDevice  , /* vkCmdSetBlendConstants */
    /*   38 */ GloamCommandScopeDevice  , /* vkCmdSetDepthBias */
    /*   39 */ GloamCommandScopeDevice  , /* vkCmdSetDepthBounds */
    /*   40 */ GloamCommandScopeDevice  , /* vkCmdSetEvent */
    /*   41 */ GloamCommandScopeDevice  , /* vkCmdSetLineWidth */
    /*   42 */ GloamCommandScopeDevice  , /* vkCmdSetScissor */
    /*   43 */ GloamCommandScopeDevice  , /* vkCmdSetStencilCompareMask */
    /*   44 */ GloamCommandScopeDevice  , /* vkCmdSetStencilReference */
    /*   45 */ GloamCommandScopeDevice  , /* vkCmdSetStencilWriteMask */
    /*   46 */ GloamCommandScopeDevice  , /* vkCmdSetViewport */
    /*   47 */ GloamCommandScopeDevice  , /* vkCmdUpdateBuffer */
    /*   48 */ GloamCommandScopeDevice  , /* vkCmdWaitEvents */
    /*   49 */ GloamCommandScopeDevice  , /* vkCmdWriteTimestamp */
    /*   50 */ GloamCommandScopeDevice  , /* vkCreateBuffer */
    /*   51 */ GloamCommandScopeDevice  , /* vkCreateBufferView */
    /*   52 */ GloamCommandScopeDevice  , /* vkCreateCommandPool */
    /*   53 */ GloamCommandScopeDevice  , /* vkCreateComputePipelines */
    /*   54 */ GloamCommandScopeDevice  , /* vkCreateDescriptorPool */
    /*   55 */ GloamCommandScopeDevice  , /* vkCreateDescriptorSetLayout */
    /*   56 */ GloamCommandScopeInstance, /* vkCreateDevice */
    /*   57 */ GloamCommandScopeDevice  , /* vkCreateEvent */
    /*   58 */ GloamCommandScopeDevice  , /* vkCreateFence */
    /*   59 */ GloamCommandScopeDevice  , /* vkCreateFramebuffer */
    /*   60 */ GloamCommandScopeDevice  , /* vkCreateGraphicsPipelines */
    /*   61 */ GloamCommandScopeDevice  , /* vkCreateImage */
    /*   62 */ GloamCommandScopeDevice  , /* vkCreateImageView */
    /*   63 */ GloamCommandScopeGlobal  , /* vkCreateInstance */
    /*   64 */ GloamCommandScopeDevice  , /* vkCreatePipelineCache */
    /*   65 */ GloamCommandScopeDevice  , /* vkCreatePipelineLayout */
    /*   66 */ GloamCommandScopeDevice  , /* vkCreateQueryPool */
    /*   67 */ GloamCommandScopeDevice  , /* vkCreateRenderPass */
    /*   68 */ GloamCommandScopeDevice  , /* vkCreateSampler */
    /*   69 */ GloamCommandScopeDevice  , /* vkCreateSemaphore */
    /*   70 */ GloamCommandScopeDevice  , /* vkCreateShaderModule */
    /*   71 */ GloamCommandScopeDevice  , /* vkDestroyBuffer */
    /*   72 */ GloamCommandScopeDevice  , /* vkDestroyBufferView */
    /*   73 */ GloamCommandScopeDevice  , /* vkDestroyCommandPool */
    /*   74 */ GloamCommandScopeDevice  , /* vkDestroyDescriptorPool */
    /*   75 */ GloamCommandScopeDevice  , /* vkDestroyDescriptorSetLayout */
    /*   76 */ GloamCommandScopeDevice  , /* vkDestroyDevice */
    /*   77 */ GloamCommandScopeDevice  , /* vkDestroyEvent */
    /*   78 */ GloamCommandScopeDevice  , /* vkDestroyFence */
    /*   79 */ GloamCommandScopeDevice  , /* vkDestroyFramebuffer */
    /*   80 */ GloamCommandScopeDevice  , /* vkDestroyImage */
    /*   81 */ GloamCommandScopeDevice  , /* vkDestroyImageView */
    /*   82 */ GloamCommandScopeInstance, /* vkDestroyInstance */
    /*   83 */ GloamCommandScopeDevice  , /* vkDestroyPipeline */
    /*   84 */ GloamCommandScopeDevice  , /* vkDestroyPipelineCache */
    /*   85 */ GloamCommandScopeDevice  , /* vkDestroyPipelineLayout */
    /*   86 */ GloamCommandScopeDevice  , /* vkDestroyQueryPool */
    /*   87 */ GloamCommandScopeDevice  , /* vkDestroyRenderPass */
    /*   88 */ GloamCommandScopeDevice  , /* vkDestroySampler */
    /*   89 */ GloamCommandScopeDevice  , /* vkDestroySemaphore */
    /*   90 */ GloamCommandScopeDevice  , /* vkDestroyShaderModule */
    /*   91 */ GloamCommandScopeDevice  , /* vkDeviceWaitIdle */
    /*   92 */ GloamCommandScopeDevice  , /* vkEndCommandBuffer */
    /*   93 */ GloamCommandScopeInstance, /* vkEnumerateDeviceExtensionProperties */
    /*   94 */ GloamCommandScopeInstance, /* vkEnumerateDeviceLayerProperties */
    /*   95 */ GloamCommandScopeGlobal  , /* vkEnumerateInstanceExtensionProperties */
    /*   96 */ GloamCommandScopeGlobal  , /* vkEnumerateInstanceLayerProperties */
    /*   97 */ GloamCommandScopeInstance, /* vkEnumeratePhysicalDevices */
    /*   98 */ GloamCommandScopeDevice  , /* vkFlushMappedMemoryRanges */
    /*   99 */ GloamCommandScopeDevice  , /* vkFreeCommandBuffers */
    /*  100 */ GloamCommandScopeDevice  , /* vkFreeDescriptorSets */
    /*  101 */ GloamCommandScopeDevice  , /* vkFreeMemory */
    /*  102 */ GloamCommandScopeDevice  , /* vkGetBufferMemoryRequirements */
    /*  103 */ GloamCommandScopeDevice  , /* vkGetDeviceMemoryCommitment */
    /*  104 */ GloamCommandScopeDevice  , /* vkGetDeviceProcAddr */
    /*  105 */ GloamCommandScopeDevice  , /* vkGetDeviceQueue */
    /*  106 */ GloamCommandScopeDevice  , /* vkGetEventStatus */
    /*  107 */ GloamCommandScopeDevice  , /* vkGetFenceStatus */
    /*  108 */ GloamCommandScopeDevice  , /* vkGetImageMemoryRequirements */
    /*  109 */ GloamCommandScopeDevice  , /* vkGetImageSparseMemoryRequirements */
    /*  110 */ GloamCommandScopeDevice  , /* vkGetImageSubresourceLayout */
    /*  111 */ GloamCommandScopeUnknown , /* vkGetInstanceProcAddr */
    /*  112 */ GloamCommandScopeInstance, /* vkGetPhysicalDeviceFeatures */
    /*  113 */ GloamCommandScopeInstance, /* vkGetPhysicalDeviceFormatProperties */
    /*  114 */ GloamCommandScopeInstance, /* vkGetPhysicalDeviceImageFormatProperties */
    /*  115 */ GloamCommandScopeInstance, /* vkGetPhysicalDeviceMemoryProperties */
    /*  116 */ GloamCommandScopeInstance, /* vkGetPhysicalDeviceProperties */
    /*  117 */ GloamCommandScopeInstance, /* vkGetPhysicalDeviceQueueFamilyProperties */
    /*  118 */ GloamCommandScopeInstance, /* vkGetPhysicalDeviceSparseImageFormatProperties */
    /*  119 */ GloamCommandScopeDevice  , /* vkGetPipelineCacheData */
    /*  120 */ GloamCommandScopeDevice  , /* vkGetQueryPoolResults */
    /*  121 */ GloamCommandScopeDevice  , /* vkGetRenderAreaGranularity */
    /*  122 */ GloamCommandScopeDevice  , /* vkInvalidateMappedMemoryRanges */
    /*  123 */ GloamCommandScopeDevice  , /* vkMapMemory */
    /*  124 */ GloamCommandScopeDevice  , /* vkMergePipelineCaches */
    /*  125 */ GloamCommandScopeDevice  , /* vkQueueBindSparse */
    /*  126 */ GloamCommandScopeDevice  , /* vkQueueSubmit */
    /*  127 */ GloamCommandScopeDevice  , /* vkQueueWaitIdle */
    /*  128 */ GloamCommandScopeDevice  , /* vkResetCommandBuffer */
    /*  129 */ GloamCommandScopeDevice  , /* vkResetCommandPool */
    /*  130 */ GloamCommandScopeDevice  , /* vkResetDescriptorPool */
    /*  131 */ GloamCommandScopeDevice  , /* vkResetEvent */
    /*  132 */ GloamCommandScopeDevice  , /* vkResetFences */
    /*  133 */ GloamCommandScopeDevice  , /* vkSetEvent */
    /*  134 */ GloamCommandScopeDevice  , /* vkUnmapMemory */
    /*  135 */ GloamCommandScopeDevice  , /* vkUpdateDescriptorSets */
    /*  136 */ GloamCommandScopeDevice  , /* vkWaitForFences */
};


/* ---- Feature PFN range table ---------------------------------------------
 * Each entry maps one feature (by featArray index) to a contiguous run of
 * pfnArray slots. The loader iterates this table and bulk-loads the run
 * when featArray[entry.extension] is set.
 */
static const GloamPfnRange_t kFeatPfnRanges_Vulkan[] = {
    {    0,    0,  137 }, /* VK_VERSION_1_0 */
};

/* ---- Vulkan scope-aware PFN range helper ---------------------------------
 * Loads a contiguous range of pfnArray slots, consulting kCommandScopes to
 * pick the right Vulkan proc-addr for each command:
 *   Global   → vkGetInstanceProcAddr(NULL, name)
 *   Instance → vkGetInstanceProcAddr(instance, name)  [skipped if instance is NULL]
 *   Device   → vkGetDeviceProcAddr(device, name)      [skipped if device is NULL]
 * Pass NULL for instance or device to skip commands of that scope.
 */
static void gloam_load_pfn_range_vk(GloamVulkanContext *context, VkInstance instance, VkDevice device, uint16_t start, uint16_t count)
{
    PFN_vkGetInstanceProcAddr gipa = context->GetInstanceProcAddr;
    PFN_vkGetDeviceProcAddr gdpa = context->GetDeviceProcAddr;
    uint16_t i;
    for (i = start; i < (uint16_t)(start + count); ++i) {
        const char *pfnName = &kFnNameData_Vulkan[kFnNameOffsets_Vulkan[i]];
        const GloamCommandScope cmdScope = (GloamCommandScope)kCommandScopes_Vulkan[i];
        GloamAPIProc pfn = NULL;
        switch (cmdScope) {
        case GloamCommandScopeGlobal:
            pfn = (GloamAPIProc)gipa(NULL, pfnName);
            break;
        case GloamCommandScopeInstance:
            if (instance)
                pfn = (GloamAPIProc)gipa(instance, pfnName);
            break;
        case GloamCommandScopeDevice:
            if (device && gdpa)
                pfn = (GloamAPIProc)gdpa(device, pfnName);
            break;
        default:
            break;
        }
        if (pfn)
            context->pfnArray[i] = (void *)pfn;
    }
}


/* ==========================================================================
 * Vulkan enabled-path helpers (shared across per-API sections)
 * ==========================================================================
 */

/* Set featArray bits from a packed Vulkan API version (VK_MAKE_API_VERSION or
 * VK_API_VERSION_x_y). Extracts major.minor, packs as (major << 8 | minor),
 * and compares against the threshold for each feature.
 */
static void gloam_vk_apply_version(GloamVulkanContext *context, uint32_t api_version)
{
    const uint16_t version_value = (uint16_t)(
        (((api_version >> 22) & 0x7fu) << 8) | ((api_version >> 12) & 0x3ffu));

    context->VERSION_1_0 = (unsigned char)(version_value >= 0x0100);
}

/* Load Global-scope PFNs via vkGetInstanceProcAddr(NULL, name).
 * context->GetInstanceProcAddr must already be set before calling.
 */
static void gloam_vk_load_global_pfns(GloamVulkanContext *context, PFN_vkGetInstanceProcAddr gipa)
{
    uint32_t i;
    for (i = 0; i < kFnCount_Vulkan; ++i) {
        if ((GloamCommandScope)kCommandScopes_Vulkan[i] == GloamCommandScopeGlobal) {
            const char *pfnName = &kFnNameData_Vulkan[kFnNameOffsets_Vulkan[i]];
            context->pfnArray[i] = (void *)gipa(NULL, pfnName);
        }
    }
}

/* ==========================================================================
 * Per-API sections
 * ==========================================================================
 */

/* --------------------------------------------------------------------------
 * API: vk
 * --------------------------------------------------------------------------
 */
/* Determine the Vulkan API version and set featArray bits accordingly.
 *
 * Called on every gloamLoaderLoadVulkanContext call so that version fields can be
 * filled in incrementally:
 *   - Instance version: queried once via EnumerateInstanceVersion (VK 1.1+)
 *     or assumed 1.0. Cached in context->vk_instance_version.
 *   - Device version: queried via GetPhysicalDeviceProperties when
 *     physical_device is non-null. Cached in context->vk_device_version.
 *     Device version takes precedence over instance version when set.
 *
 * Returns the packed version (major << 8 | minor), or 0 on hard failure.
 */
static int gloam_vk_find_core(GloamVulkanContext *context, VkPhysicalDevice physical_device)
{
    /* The top 3 bits of apiVersion encode the variant — mask them off so
     * Vulkan SC (variant 1) and plain Vulkan (variant 0) compare the same.
     */
    const uint32_t kVariantMask = 0xe0000000u;
    int major = 1, minor = 0;
    uint16_t version_value;

#ifdef VK_VERSION_1_1
    /* EnumerateInstanceVersion is Global-scope and was bootstrapped in the
     * load function before we are called.
     */
    if (!context->vk_instance_version && context->EnumerateInstanceVersion != NULL) {
        VkResult r = context->EnumerateInstanceVersion(&context->vk_instance_version);
        if (r != VK_SUCCESS)
            context->vk_instance_version = 0;
        else
            context->vk_instance_version &= ~kVariantMask;
    }
    if (context->vk_instance_version) {
        major = (int)VK_VERSION_MAJOR(context->vk_instance_version);
        minor = (int)VK_VERSION_MINOR(context->vk_instance_version);
    }
#endif

    /* If a physical device is provided and we haven't cached its version yet,
     * query it. GetPhysicalDeviceProperties is Instance-scope and will have
     * been loaded from the feature ranges on the previous call.
     */
    if (!context->vk_device_version && physical_device != NULL &&
            context->GetPhysicalDeviceProperties != NULL) {
        VkPhysicalDeviceProperties props;
        context->GetPhysicalDeviceProperties(physical_device, &props);
        context->vk_device_version = props.apiVersion & ~kVariantMask;
    }
    /* Device version is the authoritative cap; prefer it over instance version. */
    if (context->vk_device_version) {
        major = (int)VK_VERSION_MAJOR(context->vk_device_version);
        minor = (int)VK_VERSION_MINOR(context->vk_device_version);
    }

    version_value = (uint16_t)((major << 8) | minor);

    context->VERSION_1_0 = (unsigned char)(version_value >= 0x0100);

    return (int)version_value;
}

/* gloamVulkanDiscoverContext — canonical Vulkan discovery loader.
 *
 * May be called multiple times on the same context as the application
 * progresses through Vulkan initialisation:
 *   1. (NULL, NULL, NULL) — loads Global-scope functions; detects instance
 *      extensions so the caller can choose which to enable.
 *   2. (instance, NULL, NULL) — loads Global + Instance-scope functions;
 *      detects device extensions given the now-live instance.
 *   3. (instance, physical_device, device) — loads all scopes; Device-scope
 *      commands get the fast vkGetDeviceProcAddr path.
 *
 * Each call is additive: context state from previous calls is preserved and
 * only new or better-scoped slots are updated.
 *
 * Requires context->GetInstanceProcAddr to be set before the first call.
 * context->GetDeviceProcAddr is resolved automatically when an instance is
 * provided.
 */
static int gloamVulkanDiscoverContext(GloamVulkanContext *context, VkInstance instance, VkPhysicalDevice physical_device, VkDevice device)
{
    int version;
    uint32_t i;
    GLOAM_UNUSED(kFnCount_Vulkan);
    GLOAM_UNUSED(gloam_hash_ext_string);

    if (!context->GetInstanceProcAddr)
        return 0;

    /* Resolve vkGetDeviceProcAddr through the instance when available. */
    if (instance && !context->GetDeviceProcAddr)
        context->GetDeviceProcAddr =
            (PFN_vkGetDeviceProcAddr)context->GetInstanceProcAddr(
                instance, "vkGetDeviceProcAddr");

    /* Bootstrap: EnumerateInstanceVersion is Global-scope — it can be loaded
     * before any instance exists — and must be available before find_core.
     */
#ifdef VK_VERSION_1_1
    if (!context->EnumerateInstanceVersion)
        context->EnumerateInstanceVersion = (PFN_vkEnumerateInstanceVersion)
            context->GetInstanceProcAddr(NULL, "vkEnumerateInstanceVersion");
#endif

    version = gloam_vk_find_core(context, physical_device);
    if (!version)
        return 0;

    /* Load PFNs for every enabled feature via the range table. */
    for (i = 0; i < GLOAM_ARRAYSIZE(kFeatPfnRanges_Vulkan); ++i) {
        const GloamPfnRange_t *r = &kFeatPfnRanges_Vulkan[i];
        if (context->featArray[r->extension])
            gloam_load_pfn_range_vk(context, instance, device, r->start, r->count);
    }

    return version;
}

/* ==========================================================================
 * Vulkan enabled API — phased loading
 * ==========================================================================
 */

/* gloamVulkanInitializeContext — Phase 0: open libvulkan, load Global-scope
 * PFNs (vkCreateInstance, vkEnumerateInstance*, vkGetInstanceProcAddr).
 * If library_handle is non-NULL, use it without taking ownership.
 * If NULL, dlopen the platform default and take ownership.
 */
int gloamVulkanInitializeContext(GloamVulkanContext *context, void *library_handle)
{
    PFN_vkGetInstanceProcAddr gipa;

    /* In case there's an open handle in the context that we own, we should
     * tear that down now before zeroing the context. Use Finalize to do that.
     */
    gloamVulkanFinalizeContext(context);

    if (library_handle) {
        context->gloam_loader_handle = library_handle;
        context->gloam_loader_owns_handle = 0;
    } else {
        context->gloam_loader_handle = gloam_open_library(
            gloam_vk_lib_names, GLOAM_ARRAYSIZE(gloam_vk_lib_names));
        if (!context->gloam_loader_handle)
            return 0;
        context->gloam_loader_owns_handle = 1;
    }

    gipa = (PFN_vkGetInstanceProcAddr)gloam_dlsym(
        context->gloam_loader_handle, "vkGetInstanceProcAddr");
    if (!gipa) {
        if (context->gloam_loader_owns_handle)
            gloam_dlclose(context->gloam_loader_handle);
        memset(context, 0, sizeof(*context));
        return 0;
    }

    /* Store GIPA in the context, then load Global-scope PFNs. */
    context->GetInstanceProcAddr = gipa;
    gloam_vk_load_global_pfns(context, gipa);
    return 1;
}

int gloamVulkanInitialize(void *library_handle)
{
    return gloamVulkanInitializeContext(&gloam_vk_context, library_handle);
}

/* gloamVulkanInitializeCustomContext — Phase 0 variant: caller provides
 * vkGetInstanceProcAddr directly. No library handle management.
 */
void gloamVulkanInitializeCustomContext(GloamVulkanContext *context, PFN_vkGetInstanceProcAddr getInstanceProcAddr)
{
    gloamVulkanFinalizeContext(context);
    context->GetInstanceProcAddr = getInstanceProcAddr;
    gloam_vk_load_global_pfns(context, getInstanceProcAddr);
}

void gloamVulkanInitializeCustom(PFN_vkGetInstanceProcAddr getInstanceProcAddr)
{
    gloamVulkanInitializeCustomContext(&gloam_vk_context, getInstanceProcAddr);
}

/* gloamVulkanLoadInstanceContext — Phase 1: load PFNs for core features and
 * instance extensions. Instance and Global scope commands are loaded via
 * vkGetInstanceProcAddr. Device-scope commands in instance extensions (e.g.
 * VK_EXT_debug_utils) are skipped here and picked up by LoadDevice.
 *
 * Sets featArray from api_version. Sets extArray for enabled instance
 * extensions. Runs alias resolution. Returns 1 on success.
 */
int gloamVulkanLoadInstanceContext(GloamVulkanContext *context, VkInstance instance, uint32_t api_version,
                                   uint32_t num_instance_extensions, const char *const *instance_extensions)
{
    uint32_t i;

    if (!context->GetInstanceProcAddr)
        return 0;

    gloam_vk_apply_version(context, api_version);

    /* Load PFNs for every enabled feature (instance + global scope). */
    for (i = 0; i < GLOAM_ARRAYSIZE(kFeatPfnRanges_Vulkan); ++i) {
        const GloamPfnRange_t *r = &kFeatPfnRanges_Vulkan[i];
        if (context->featArray[r->extension])
            gloam_load_pfn_range_vk(context, instance, NULL, r->start, r->count);
    }

    context->vk_loaded_instance = instance;
    GLOAM_UNUSED(num_instance_extensions);
    GLOAM_UNUSED(instance_extensions);
    return 1;
}

int gloamVulkanLoadInstance(VkInstance instance, uint32_t api_version, uint32_t num_instance_extensions, const char *const *instance_extensions)
{
    return gloamVulkanLoadInstanceContext(&gloam_vk_context, instance,
        api_version, num_instance_extensions, instance_extensions);
}

/* gloamVulkanLoadPhysicalDeviceExtensionsContext — Phase 1.5 (optional):
 * pre-load PFNs for device extensions before the VkDevice exists.
 *
 * Device extensions may provide Instance-scope query functions (e.g.
 * vkGetPhysicalDeviceFragmentShadingRatesKHR) that applications need to call
 * while selecting which device extensions to enable. This function loads
 * Global and Instance-scope commands for the listed extensions via
 * vkGetInstanceProcAddr. Device-scope commands are skipped because no
 * VkDevice exists yet.
 *
 * Does NOT set extArray — the GLOAM_VK_* support macros remain unchanged.
 * Call gloamVulkanLoadDevice after device creation for full extension
 * loading.
 */
void gloamVulkanLoadPhysicalDeviceExtensionsContext(GloamVulkanContext *context,  uint32_t num_device_extensions, const char *const *device_extensions)
{

    GLOAM_UNUSED(num_device_extensions);
    GLOAM_UNUSED(device_extensions);
}

void gloamVulkanLoadPhysicalDeviceExtensions(uint32_t num_device_extensions, const char *const *device_extensions)
{
    gloamVulkanLoadPhysicalDeviceExtensionsContext(&gloam_vk_context,
        num_device_extensions, device_extensions);
}

void gloamVulkanLoadPhysicalDeviceExtensionContext(GloamVulkanContext *context,  const char *device_extension)
{
    gloamVulkanLoadPhysicalDeviceExtensionsContext(context, 1, &device_extension);
}

void gloamVulkanLoadPhysicalDeviceExtension(const char *device_extension)
{
    gloamVulkanLoadPhysicalDeviceExtensionsContext(&gloam_vk_context,
        1, &device_extension);
}

/* gloamVulkanLoadDeviceContext — Phase 2: load PFNs for device extensions and
 * reload Device-scope core PFNs via vkGetDeviceProcAddr (bypassing the loader
 * trampoline).
 *
 * Extensions may contain commands of any scope. The unified loader dispatches
 * each command to the correct proc-addr function based on scope: Instance-scope
 * via vkGetInstanceProcAddr, Device-scope via vkGetDeviceProcAddr.
 *
 * Updates featArray from the device's api_version. Sets extArray for enabled
 * device extensions. Runs alias resolution. Returns 1 on success.
 */
int gloamVulkanLoadDeviceContext(GloamVulkanContext *context, VkDevice device, VkPhysicalDevice physical_device,
                                 uint32_t num_device_extensions, const char *const *device_extensions)
{
    uint32_t i;
    VkPhysicalDeviceProperties props;
    VkInstance instance = context->vk_loaded_instance;

    if (instance && !context->GetDeviceProcAddr)
        context->GetDeviceProcAddr =
            (PFN_vkGetDeviceProcAddr)context->GetInstanceProcAddr(
                instance, "vkGetDeviceProcAddr");

    if (!context->GetDeviceProcAddr || !device)
        return 0;

    context->GetPhysicalDeviceProperties(physical_device, &props);
    gloam_vk_apply_version(context, props.apiVersion);

    /* Reload PFNs for enabled features — Device-scope gets the fast path. */
    for (i = 0; i < GLOAM_ARRAYSIZE(kFeatPfnRanges_Vulkan); ++i) {
        const GloamPfnRange_t *r = &kFeatPfnRanges_Vulkan[i];
        if (context->featArray[r->extension])
            gloam_load_pfn_range_vk(context, instance, device, r->start, r->count);
    }

    context->vk_loaded_device = device;
    GLOAM_UNUSED(num_device_extensions);
    GLOAM_UNUSED(device_extensions);
    return 1;
}

int gloamVulkanLoadDevice(VkDevice device, VkPhysicalDevice physical_device, uint32_t num_device_extensions, const char *const *device_extensions)
{
    return gloamVulkanLoadDeviceContext(&gloam_vk_context, device,
        physical_device, num_device_extensions, device_extensions);
}

uint32_t gloamVulkanGetInstanceVersionContext(GloamVulkanContext *context)
{
#if defined(VK_VERSION_1_1)
    uint32_t apiVersion = 0;
    if (context->EnumerateInstanceVersion && context->EnumerateInstanceVersion(&apiVersion) == VK_SUCCESS)
        return apiVersion;
#endif

    if (context->CreateInstance)
        return VK_API_VERSION_1_0;

    return 0;
}

uint32_t gloamVulkanGetInstanceVersion(void)
{
    return gloamVulkanGetInstanceVersionContext(&gloam_vk_context);
}

VkDevice gloamVulkanGetLoadedDeviceContext(GloamVulkanContext *context)
{
    return context->vk_loaded_device;
}

VkDevice gloamVulkanGetLoadedDevice(void)
{
    return gloamVulkanGetLoadedDeviceContext(&gloam_vk_context);
}

VkInstance gloamVulkanGetLoadedInstanceContext(GloamVulkanContext *context)
{
    return context->vk_loaded_instance;
}

VkInstance gloamVulkanGetLoadedInstance(void)
{
    return gloamVulkanGetLoadedInstanceContext(&gloam_vk_context);
}

/* gloamVulkanFinalizeContext — close library handle if gloam owns it, zero
 * the context.
 */
void gloamVulkanFinalizeContext(GloamVulkanContext *context)
{
    if (context->gloam_loader_owns_handle && context->gloam_loader_handle)
        gloam_dlclose(context->gloam_loader_handle);
    memset(context, 0, sizeof(*context));
}

void gloamVulkanFinalize(void)
{
    gloamVulkanFinalizeContext(&gloam_vk_context);
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


/* ---- Vulkan built-in discovery loader ----------------------------------- */

/* gloamLoaderLoadVulkanContext
 *
 * Opens the Vulkan library into context->gloam_loader_handle if it is not
 * already set, stores vkGetInstanceProcAddr in the context, then delegates to
 * gloamVulkanDiscoverContext. Follows the same additive multi-call contract
 * as the underlying discover function.
 */
int gloamLoaderLoadVulkanContext(GloamVulkanContext *context, VkInstance instance, VkPhysicalDevice physical_device, VkDevice device)
{
    int did_open = 0;
    int version;
    void *handle;

    handle = context->gloam_loader_handle;

    if (!handle) {
        handle = gloam_open_library(
            gloam_vk_lib_names, GLOAM_ARRAYSIZE(gloam_vk_lib_names));
        did_open = 1;
    }

    if (!handle)
        return 0;

    if (!context->GetInstanceProcAddr)
        context->GetInstanceProcAddr =
            (PFN_vkGetInstanceProcAddr)gloam_dlsym(handle, "vkGetInstanceProcAddr");

    if (!context->GetInstanceProcAddr) {
        if (did_open)
            gloam_dlclose(handle);
        return 0;
    }

    version = gloamVulkanDiscoverContext(context, instance, physical_device, device);

    if (!version && did_open) {
        gloam_dlclose(handle);
        return 0;
    }

    context->gloam_loader_handle = handle;
    context->gloam_loader_owns_handle |= (uint8_t)did_open;

    return version;
}

int gloamLoaderLoadVulkan(VkInstance instance, VkPhysicalDevice physical_device, VkDevice device)
{
    return gloamLoaderLoadVulkanContext(&gloam_vk_context, instance,
                                        physical_device, device);
}

/* Close the library handle (if set) then zero all context state. */
void gloamLoaderUnloadVulkanContext(GloamVulkanContext *context)
{
    if (context->gloam_loader_handle && context->gloam_loader_owns_handle) {
        gloam_dlclose(context->gloam_loader_handle);
    }
    gloamLoaderResetVulkanContext(context);
}

void gloamLoaderUnloadVulkan(void)
{
    gloamLoaderUnloadVulkanContext(&gloam_vk_context);
}

/* Zero all context state without touching the library handle. */
void gloamLoaderResetVulkanContext(GloamVulkanContext *context)
{
    memset(context, 0, sizeof(*context));
}

void gloamLoaderResetVulkan(void)
{
    gloamLoaderResetVulkanContext(&gloam_vk_context);
}
