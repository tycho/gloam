#ifndef GLOAM_VK_H
#define GLOAM_VK_H


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

#ifndef GLOAM_FORCE_INLINE
#if defined(_MSC_VER)
#  define GLOAM_FORCE_INLINE static __forceinline
#elif defined(__GNUC__) || defined(__clang__)
#  define GLOAM_FORCE_INLINE static inline __attribute__((always_inline))
#else
#  define GLOAM_FORCE_INLINE static inline
#endif
#endif

#ifndef VK_NO_PROTOTYPES
#define VK_NO_PROTOTYPES
#endif

#ifndef VULKAN_H_
#include <vulkan/vk_platform.h>
#include <vulkan/vulkan_core.h>
#endif

#if !defined(VULKAN_H_)

#ifdef VK_USE_PLATFORM_ANDROID_KHR
#include <vulkan/vulkan_android.h>
#endif

#ifdef VK_USE_PLATFORM_FUCHSIA
#include <zircon/types.h>
#include <vulkan/vulkan_fuchsia.h>
#endif

#ifdef VK_USE_PLATFORM_IOS_MVK
#include <vulkan/vulkan_ios.h>
#endif

#ifdef VK_USE_PLATFORM_MACOS_MVK
#include <vulkan/vulkan_macos.h>
#endif

#ifdef VK_USE_PLATFORM_METAL_EXT
#include <vulkan/vulkan_metal.h>
#endif

#ifdef VK_USE_PLATFORM_VI_NN
#include <vulkan/vulkan_vi.h>
#endif

#ifdef VK_USE_PLATFORM_WAYLAND_KHR
#include <vulkan/vulkan_wayland.h>
#endif

#ifdef VK_USE_PLATFORM_WIN32_KHR
typedef unsigned long DWORD;
typedef const wchar_t* LPCWSTR;
typedef void* HANDLE;
typedef struct HINSTANCE__* HINSTANCE;
typedef struct HWND__* HWND;
typedef struct HMONITOR__* HMONITOR;
typedef struct _SECURITY_ATTRIBUTES SECURITY_ATTRIBUTES;
#include <vulkan/vulkan_win32.h>
#endif

#ifdef VK_USE_PLATFORM_XCB_KHR
#include <xcb/xcb.h>
#include <vulkan/vulkan_xcb.h>
#endif

#ifdef VK_USE_PLATFORM_XLIB_KHR
typedef struct _XDisplay Display;
typedef unsigned long Window;
typedef unsigned long VisualID;
#include <vulkan/vulkan_xlib.h>
#endif

#ifdef VK_USE_PLATFORM_DIRECTFB_EXT
#include <directfb.h>
#include <vulkan/vulkan_directfb.h>
#endif

#ifdef VK_USE_PLATFORM_XLIB_XRANDR_EXT
typedef struct _XDisplay Display;
typedef unsigned long RROutput;
#include <vulkan/vulkan_xlib_xrandr.h>
#endif

#ifdef VK_USE_PLATFORM_GGP
#include <ggp_c/vulkan_types.h>
#include <vulkan/vulkan_ggp.h>
#endif

#ifdef VK_USE_PLATFORM_SCREEN_QNX
#include <screen/screen.h>
#include <vulkan/vulkan_screen.h>
#endif

#ifdef VK_USE_PLATFORM_SCI
#include <nvscisync.h>
#include <nvscibuf.h>
#include <vulkan/vulkan_sci.h>
#endif

#ifdef VK_ENABLE_BETA_EXTENSIONS
#include <vulkan/vulkan_beta.h>
#endif

#ifdef VK_USE_PLATFORM_OHOS
#include <vulkan/vulkan_ohos.h>
#endif

#endif /* !defined(VULKAN_H_) */
#ifdef __cplusplus
extern "C" {
#endif

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
typedef struct GloamVulkanContext {
    union {
        unsigned char featArray[4];
        struct {
        /*    0 */ unsigned char VERSION_1_0;
        /*    1 */ unsigned char VERSION_1_1;
        /*    2 */ unsigned char VERSION_1_2;
        /*    3 */ unsigned char VERSION_1_3;
        };
    };

    union {
        void *pfnArray[215];
        struct {
        /*    0 */ PFN_vkAllocateCommandBuffers AllocateCommandBuffers;
        /*    1 */ PFN_vkAllocateDescriptorSets AllocateDescriptorSets;
        /*    2 */ PFN_vkAllocateMemory AllocateMemory;
        /*    3 */ PFN_vkBeginCommandBuffer BeginCommandBuffer;
        /*    4 */ PFN_vkBindBufferMemory BindBufferMemory;
        /*    5 */ PFN_vkBindImageMemory BindImageMemory;
        /*    6 */ PFN_vkCmdBeginQuery CmdBeginQuery;
        /*    7 */ PFN_vkCmdBeginRenderPass CmdBeginRenderPass;
        /*    8 */ PFN_vkCmdBindDescriptorSets CmdBindDescriptorSets;
        /*    9 */ PFN_vkCmdBindIndexBuffer CmdBindIndexBuffer;
        /*   10 */ PFN_vkCmdBindPipeline CmdBindPipeline;
        /*   11 */ PFN_vkCmdBindVertexBuffers CmdBindVertexBuffers;
        /*   12 */ PFN_vkCmdBlitImage CmdBlitImage;
        /*   13 */ PFN_vkCmdClearAttachments CmdClearAttachments;
        /*   14 */ PFN_vkCmdClearColorImage CmdClearColorImage;
        /*   15 */ PFN_vkCmdClearDepthStencilImage CmdClearDepthStencilImage;
        /*   16 */ PFN_vkCmdCopyBuffer CmdCopyBuffer;
        /*   17 */ PFN_vkCmdCopyBufferToImage CmdCopyBufferToImage;
        /*   18 */ PFN_vkCmdCopyImage CmdCopyImage;
        /*   19 */ PFN_vkCmdCopyImageToBuffer CmdCopyImageToBuffer;
        /*   20 */ PFN_vkCmdCopyQueryPoolResults CmdCopyQueryPoolResults;
        /*   21 */ PFN_vkCmdDispatch CmdDispatch;
        /*   22 */ PFN_vkCmdDispatchIndirect CmdDispatchIndirect;
        /*   23 */ PFN_vkCmdDraw CmdDraw;
        /*   24 */ PFN_vkCmdDrawIndexed CmdDrawIndexed;
        /*   25 */ PFN_vkCmdDrawIndexedIndirect CmdDrawIndexedIndirect;
        /*   26 */ PFN_vkCmdDrawIndirect CmdDrawIndirect;
        /*   27 */ PFN_vkCmdEndQuery CmdEndQuery;
        /*   28 */ PFN_vkCmdEndRenderPass CmdEndRenderPass;
        /*   29 */ PFN_vkCmdExecuteCommands CmdExecuteCommands;
        /*   30 */ PFN_vkCmdFillBuffer CmdFillBuffer;
        /*   31 */ PFN_vkCmdNextSubpass CmdNextSubpass;
        /*   32 */ PFN_vkCmdPipelineBarrier CmdPipelineBarrier;
        /*   33 */ PFN_vkCmdPushConstants CmdPushConstants;
        /*   34 */ PFN_vkCmdResetEvent CmdResetEvent;
        /*   35 */ PFN_vkCmdResetQueryPool CmdResetQueryPool;
        /*   36 */ PFN_vkCmdResolveImage CmdResolveImage;
        /*   37 */ PFN_vkCmdSetBlendConstants CmdSetBlendConstants;
        /*   38 */ PFN_vkCmdSetDepthBias CmdSetDepthBias;
        /*   39 */ PFN_vkCmdSetDepthBounds CmdSetDepthBounds;
        /*   40 */ PFN_vkCmdSetEvent CmdSetEvent;
        /*   41 */ PFN_vkCmdSetLineWidth CmdSetLineWidth;
        /*   42 */ PFN_vkCmdSetScissor CmdSetScissor;
        /*   43 */ PFN_vkCmdSetStencilCompareMask CmdSetStencilCompareMask;
        /*   44 */ PFN_vkCmdSetStencilReference CmdSetStencilReference;
        /*   45 */ PFN_vkCmdSetStencilWriteMask CmdSetStencilWriteMask;
        /*   46 */ PFN_vkCmdSetViewport CmdSetViewport;
        /*   47 */ PFN_vkCmdUpdateBuffer CmdUpdateBuffer;
        /*   48 */ PFN_vkCmdWaitEvents CmdWaitEvents;
        /*   49 */ PFN_vkCmdWriteTimestamp CmdWriteTimestamp;
        /*   50 */ PFN_vkCreateBuffer CreateBuffer;
        /*   51 */ PFN_vkCreateBufferView CreateBufferView;
        /*   52 */ PFN_vkCreateCommandPool CreateCommandPool;
        /*   53 */ PFN_vkCreateComputePipelines CreateComputePipelines;
        /*   54 */ PFN_vkCreateDescriptorPool CreateDescriptorPool;
        /*   55 */ PFN_vkCreateDescriptorSetLayout CreateDescriptorSetLayout;
        /*   56 */ PFN_vkCreateDevice CreateDevice;
        /*   57 */ PFN_vkCreateEvent CreateEvent;
        /*   58 */ PFN_vkCreateFence CreateFence;
        /*   59 */ PFN_vkCreateFramebuffer CreateFramebuffer;
        /*   60 */ PFN_vkCreateGraphicsPipelines CreateGraphicsPipelines;
        /*   61 */ PFN_vkCreateImage CreateImage;
        /*   62 */ PFN_vkCreateImageView CreateImageView;
        /*   63 */ PFN_vkCreateInstance CreateInstance;
        /*   64 */ PFN_vkCreatePipelineCache CreatePipelineCache;
        /*   65 */ PFN_vkCreatePipelineLayout CreatePipelineLayout;
        /*   66 */ PFN_vkCreateQueryPool CreateQueryPool;
        /*   67 */ PFN_vkCreateRenderPass CreateRenderPass;
        /*   68 */ PFN_vkCreateSampler CreateSampler;
        /*   69 */ PFN_vkCreateSemaphore CreateSemaphore;
        /*   70 */ PFN_vkCreateShaderModule CreateShaderModule;
        /*   71 */ PFN_vkDestroyBuffer DestroyBuffer;
        /*   72 */ PFN_vkDestroyBufferView DestroyBufferView;
        /*   73 */ PFN_vkDestroyCommandPool DestroyCommandPool;
        /*   74 */ PFN_vkDestroyDescriptorPool DestroyDescriptorPool;
        /*   75 */ PFN_vkDestroyDescriptorSetLayout DestroyDescriptorSetLayout;
        /*   76 */ PFN_vkDestroyDevice DestroyDevice;
        /*   77 */ PFN_vkDestroyEvent DestroyEvent;
        /*   78 */ PFN_vkDestroyFence DestroyFence;
        /*   79 */ PFN_vkDestroyFramebuffer DestroyFramebuffer;
        /*   80 */ PFN_vkDestroyImage DestroyImage;
        /*   81 */ PFN_vkDestroyImageView DestroyImageView;
        /*   82 */ PFN_vkDestroyInstance DestroyInstance;
        /*   83 */ PFN_vkDestroyPipeline DestroyPipeline;
        /*   84 */ PFN_vkDestroyPipelineCache DestroyPipelineCache;
        /*   85 */ PFN_vkDestroyPipelineLayout DestroyPipelineLayout;
        /*   86 */ PFN_vkDestroyQueryPool DestroyQueryPool;
        /*   87 */ PFN_vkDestroyRenderPass DestroyRenderPass;
        /*   88 */ PFN_vkDestroySampler DestroySampler;
        /*   89 */ PFN_vkDestroySemaphore DestroySemaphore;
        /*   90 */ PFN_vkDestroyShaderModule DestroyShaderModule;
        /*   91 */ PFN_vkDeviceWaitIdle DeviceWaitIdle;
        /*   92 */ PFN_vkEndCommandBuffer EndCommandBuffer;
        /*   93 */ PFN_vkEnumerateDeviceExtensionProperties EnumerateDeviceExtensionProperties;
        /*   94 */ PFN_vkEnumerateDeviceLayerProperties EnumerateDeviceLayerProperties;
        /*   95 */ PFN_vkEnumerateInstanceExtensionProperties EnumerateInstanceExtensionProperties;
        /*   96 */ PFN_vkEnumerateInstanceLayerProperties EnumerateInstanceLayerProperties;
        /*   97 */ PFN_vkEnumeratePhysicalDevices EnumeratePhysicalDevices;
        /*   98 */ PFN_vkFlushMappedMemoryRanges FlushMappedMemoryRanges;
        /*   99 */ PFN_vkFreeCommandBuffers FreeCommandBuffers;
        /*  100 */ PFN_vkFreeDescriptorSets FreeDescriptorSets;
        /*  101 */ PFN_vkFreeMemory FreeMemory;
        /*  102 */ PFN_vkGetBufferMemoryRequirements GetBufferMemoryRequirements;
        /*  103 */ PFN_vkGetDeviceMemoryCommitment GetDeviceMemoryCommitment;
        /*  104 */ PFN_vkGetDeviceProcAddr GetDeviceProcAddr;
        /*  105 */ PFN_vkGetDeviceQueue GetDeviceQueue;
        /*  106 */ PFN_vkGetEventStatus GetEventStatus;
        /*  107 */ PFN_vkGetFenceStatus GetFenceStatus;
        /*  108 */ PFN_vkGetImageMemoryRequirements GetImageMemoryRequirements;
        /*  109 */ PFN_vkGetImageSparseMemoryRequirements GetImageSparseMemoryRequirements;
        /*  110 */ PFN_vkGetImageSubresourceLayout GetImageSubresourceLayout;
        /*  111 */ PFN_vkGetInstanceProcAddr GetInstanceProcAddr;
        /*  112 */ PFN_vkGetPhysicalDeviceFeatures GetPhysicalDeviceFeatures;
        /*  113 */ PFN_vkGetPhysicalDeviceFormatProperties GetPhysicalDeviceFormatProperties;
        /*  114 */ PFN_vkGetPhysicalDeviceImageFormatProperties GetPhysicalDeviceImageFormatProperties;
        /*  115 */ PFN_vkGetPhysicalDeviceMemoryProperties GetPhysicalDeviceMemoryProperties;
        /*  116 */ PFN_vkGetPhysicalDeviceProperties GetPhysicalDeviceProperties;
        /*  117 */ PFN_vkGetPhysicalDeviceQueueFamilyProperties GetPhysicalDeviceQueueFamilyProperties;
        /*  118 */ PFN_vkGetPhysicalDeviceSparseImageFormatProperties GetPhysicalDeviceSparseImageFormatProperties;
        /*  119 */ PFN_vkGetPipelineCacheData GetPipelineCacheData;
        /*  120 */ PFN_vkGetQueryPoolResults GetQueryPoolResults;
        /*  121 */ PFN_vkGetRenderAreaGranularity GetRenderAreaGranularity;
        /*  122 */ PFN_vkInvalidateMappedMemoryRanges InvalidateMappedMemoryRanges;
        /*  123 */ PFN_vkMapMemory MapMemory;
        /*  124 */ PFN_vkMergePipelineCaches MergePipelineCaches;
        /*  125 */ PFN_vkQueueBindSparse QueueBindSparse;
        /*  126 */ PFN_vkQueueSubmit QueueSubmit;
        /*  127 */ PFN_vkQueueWaitIdle QueueWaitIdle;
        /*  128 */ PFN_vkResetCommandBuffer ResetCommandBuffer;
        /*  129 */ PFN_vkResetCommandPool ResetCommandPool;
        /*  130 */ PFN_vkResetDescriptorPool ResetDescriptorPool;
        /*  131 */ PFN_vkResetEvent ResetEvent;
        /*  132 */ PFN_vkResetFences ResetFences;
        /*  133 */ PFN_vkSetEvent SetEvent;
        /*  134 */ PFN_vkUnmapMemory UnmapMemory;
        /*  135 */ PFN_vkUpdateDescriptorSets UpdateDescriptorSets;
        /*  136 */ PFN_vkWaitForFences WaitForFences;
        /*  137 */ PFN_vkBindBufferMemory2 BindBufferMemory2;
        /*  138 */ PFN_vkBindImageMemory2 BindImageMemory2;
        /*  139 */ PFN_vkCmdDispatchBase CmdDispatchBase;
        /*  140 */ PFN_vkCmdSetDeviceMask CmdSetDeviceMask;
        /*  141 */ PFN_vkCreateDescriptorUpdateTemplate CreateDescriptorUpdateTemplate;
        /*  142 */ PFN_vkCreateSamplerYcbcrConversion CreateSamplerYcbcrConversion;
        /*  143 */ PFN_vkDestroyDescriptorUpdateTemplate DestroyDescriptorUpdateTemplate;
        /*  144 */ PFN_vkDestroySamplerYcbcrConversion DestroySamplerYcbcrConversion;
        /*  145 */ PFN_vkEnumerateInstanceVersion EnumerateInstanceVersion;
        /*  146 */ PFN_vkEnumeratePhysicalDeviceGroups EnumeratePhysicalDeviceGroups;
        /*  147 */ PFN_vkGetBufferMemoryRequirements2 GetBufferMemoryRequirements2;
        /*  148 */ PFN_vkGetDescriptorSetLayoutSupport GetDescriptorSetLayoutSupport;
        /*  149 */ PFN_vkGetDeviceGroupPeerMemoryFeatures GetDeviceGroupPeerMemoryFeatures;
        /*  150 */ PFN_vkGetDeviceQueue2 GetDeviceQueue2;
        /*  151 */ PFN_vkGetImageMemoryRequirements2 GetImageMemoryRequirements2;
        /*  152 */ PFN_vkGetImageSparseMemoryRequirements2 GetImageSparseMemoryRequirements2;
        /*  153 */ PFN_vkGetPhysicalDeviceExternalBufferProperties GetPhysicalDeviceExternalBufferProperties;
        /*  154 */ PFN_vkGetPhysicalDeviceExternalFenceProperties GetPhysicalDeviceExternalFenceProperties;
        /*  155 */ PFN_vkGetPhysicalDeviceExternalSemaphoreProperties GetPhysicalDeviceExternalSemaphoreProperties;
        /*  156 */ PFN_vkGetPhysicalDeviceFeatures2 GetPhysicalDeviceFeatures2;
        /*  157 */ PFN_vkGetPhysicalDeviceFormatProperties2 GetPhysicalDeviceFormatProperties2;
        /*  158 */ PFN_vkGetPhysicalDeviceImageFormatProperties2 GetPhysicalDeviceImageFormatProperties2;
        /*  159 */ PFN_vkGetPhysicalDeviceMemoryProperties2 GetPhysicalDeviceMemoryProperties2;
        /*  160 */ PFN_vkGetPhysicalDeviceProperties2 GetPhysicalDeviceProperties2;
        /*  161 */ PFN_vkGetPhysicalDeviceQueueFamilyProperties2 GetPhysicalDeviceQueueFamilyProperties2;
        /*  162 */ PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 GetPhysicalDeviceSparseImageFormatProperties2;
        /*  163 */ PFN_vkTrimCommandPool TrimCommandPool;
        /*  164 */ PFN_vkUpdateDescriptorSetWithTemplate UpdateDescriptorSetWithTemplate;
        /*  165 */ PFN_vkCmdBeginRenderPass2 CmdBeginRenderPass2;
        /*  166 */ PFN_vkCmdDrawIndexedIndirectCount CmdDrawIndexedIndirectCount;
        /*  167 */ PFN_vkCmdDrawIndirectCount CmdDrawIndirectCount;
        /*  168 */ PFN_vkCmdEndRenderPass2 CmdEndRenderPass2;
        /*  169 */ PFN_vkCmdNextSubpass2 CmdNextSubpass2;
        /*  170 */ PFN_vkCreateRenderPass2 CreateRenderPass2;
        /*  171 */ PFN_vkGetBufferDeviceAddress GetBufferDeviceAddress;
        /*  172 */ PFN_vkGetBufferOpaqueCaptureAddress GetBufferOpaqueCaptureAddress;
        /*  173 */ PFN_vkGetDeviceMemoryOpaqueCaptureAddress GetDeviceMemoryOpaqueCaptureAddress;
        /*  174 */ PFN_vkGetSemaphoreCounterValue GetSemaphoreCounterValue;
        /*  175 */ PFN_vkResetQueryPool ResetQueryPool;
        /*  176 */ PFN_vkSignalSemaphore SignalSemaphore;
        /*  177 */ PFN_vkWaitSemaphores WaitSemaphores;
        /*  178 */ PFN_vkCmdBeginRendering CmdBeginRendering;
        /*  179 */ PFN_vkCmdBindVertexBuffers2 CmdBindVertexBuffers2;
        /*  180 */ PFN_vkCmdBlitImage2 CmdBlitImage2;
        /*  181 */ PFN_vkCmdCopyBuffer2 CmdCopyBuffer2;
        /*  182 */ PFN_vkCmdCopyBufferToImage2 CmdCopyBufferToImage2;
        /*  183 */ PFN_vkCmdCopyImage2 CmdCopyImage2;
        /*  184 */ PFN_vkCmdCopyImageToBuffer2 CmdCopyImageToBuffer2;
        /*  185 */ PFN_vkCmdEndRendering CmdEndRendering;
        /*  186 */ PFN_vkCmdPipelineBarrier2 CmdPipelineBarrier2;
        /*  187 */ PFN_vkCmdResetEvent2 CmdResetEvent2;
        /*  188 */ PFN_vkCmdResolveImage2 CmdResolveImage2;
        /*  189 */ PFN_vkCmdSetCullMode CmdSetCullMode;
        /*  190 */ PFN_vkCmdSetDepthBiasEnable CmdSetDepthBiasEnable;
        /*  191 */ PFN_vkCmdSetDepthBoundsTestEnable CmdSetDepthBoundsTestEnable;
        /*  192 */ PFN_vkCmdSetDepthCompareOp CmdSetDepthCompareOp;
        /*  193 */ PFN_vkCmdSetDepthTestEnable CmdSetDepthTestEnable;
        /*  194 */ PFN_vkCmdSetDepthWriteEnable CmdSetDepthWriteEnable;
        /*  195 */ PFN_vkCmdSetEvent2 CmdSetEvent2;
        /*  196 */ PFN_vkCmdSetFrontFace CmdSetFrontFace;
        /*  197 */ PFN_vkCmdSetPrimitiveRestartEnable CmdSetPrimitiveRestartEnable;
        /*  198 */ PFN_vkCmdSetPrimitiveTopology CmdSetPrimitiveTopology;
        /*  199 */ PFN_vkCmdSetRasterizerDiscardEnable CmdSetRasterizerDiscardEnable;
        /*  200 */ PFN_vkCmdSetScissorWithCount CmdSetScissorWithCount;
        /*  201 */ PFN_vkCmdSetStencilOp CmdSetStencilOp;
        /*  202 */ PFN_vkCmdSetStencilTestEnable CmdSetStencilTestEnable;
        /*  203 */ PFN_vkCmdSetViewportWithCount CmdSetViewportWithCount;
        /*  204 */ PFN_vkCmdWaitEvents2 CmdWaitEvents2;
        /*  205 */ PFN_vkCmdWriteTimestamp2 CmdWriteTimestamp2;
        /*  206 */ PFN_vkCreatePrivateDataSlot CreatePrivateDataSlot;
        /*  207 */ PFN_vkDestroyPrivateDataSlot DestroyPrivateDataSlot;
        /*  208 */ PFN_vkGetDeviceBufferMemoryRequirements GetDeviceBufferMemoryRequirements;
        /*  209 */ PFN_vkGetDeviceImageMemoryRequirements GetDeviceImageMemoryRequirements;
        /*  210 */ PFN_vkGetDeviceImageSparseMemoryRequirements GetDeviceImageSparseMemoryRequirements;
        /*  211 */ PFN_vkGetPhysicalDeviceToolProperties GetPhysicalDeviceToolProperties;
        /*  212 */ PFN_vkGetPrivateData GetPrivateData;
        /*  213 */ PFN_vkQueueSubmit2 QueueSubmit2;
        /*  214 */ PFN_vkSetPrivateData SetPrivateData;
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

    /* The last VkInstance this context loaded entry points from */
    VkInstance vk_loaded_instance;
    /* The last VkDevice this context loaded entry points from */
    VkDevice vk_loaded_device;
    /* Vulkan discovery-path metadata — used by gloamLoaderLoadVulkanContext to make
     * repeated calls additive without re-enumerating already-cached scopes.
     */
    uint32_t vk_instance_version;    /* cached EnumerateInstanceVersion result        */
    uint32_t vk_device_version;      /* cached GetPhysicalDeviceProperties.apiVersion */
    uint8_t  vk_found_instance_exts; /* set once instance extensions enumerated       */
    uint8_t  vk_found_device_exts;   /* set once device extensions enumerated         */
} GloamVulkanContext;

/* Global context instance — a value, not a pointer, so the compiler knows
 * its address is fixed and does not re-load it on every access.
 */
extern GloamVulkanContext gloam_vk_context;

/* ---- Feature presence macros --------------------------------------------
 * Test whether a versioned feature was detected at load time.
 */
#define GLOAM_VK_VERSION_1_0 (gloam_vk_context.VERSION_1_0)
#define GLOAM_VK_VERSION_1_1 (gloam_vk_context.VERSION_1_1)
#define GLOAM_VK_VERSION_1_2 (gloam_vk_context.VERSION_1_2)
#define GLOAM_VK_VERSION_1_3 (gloam_vk_context.VERSION_1_3)

/* ---- Extension presence macros ------------------------------------------ */


/* ---- Dispatch ------------------------------------------------------------ */

GLOAM_FORCE_INLINE VkResult vkAllocateCommandBuffers(VkDevice device, const VkCommandBufferAllocateInfo* pAllocateInfo, VkCommandBuffer* pCommandBuffers) {
    return gloam_vk_context.AllocateCommandBuffers(device, pAllocateInfo, pCommandBuffers);
}
GLOAM_FORCE_INLINE VkResult vkAllocateDescriptorSets(VkDevice device, const VkDescriptorSetAllocateInfo* pAllocateInfo, VkDescriptorSet* pDescriptorSets) {
    return gloam_vk_context.AllocateDescriptorSets(device, pAllocateInfo, pDescriptorSets);
}
GLOAM_FORCE_INLINE VkResult vkAllocateMemory(VkDevice device, const VkMemoryAllocateInfo* pAllocateInfo, const VkAllocationCallbacks* pAllocator, VkDeviceMemory* pMemory) {
    return gloam_vk_context.AllocateMemory(device, pAllocateInfo, pAllocator, pMemory);
}
GLOAM_FORCE_INLINE VkResult vkBeginCommandBuffer(VkCommandBuffer commandBuffer, const VkCommandBufferBeginInfo* pBeginInfo) {
    return gloam_vk_context.BeginCommandBuffer(commandBuffer, pBeginInfo);
}
GLOAM_FORCE_INLINE VkResult vkBindBufferMemory(VkDevice device, VkBuffer buffer, VkDeviceMemory memory, VkDeviceSize memoryOffset) {
    return gloam_vk_context.BindBufferMemory(device, buffer, memory, memoryOffset);
}
GLOAM_FORCE_INLINE VkResult vkBindImageMemory(VkDevice device, VkImage image, VkDeviceMemory memory, VkDeviceSize memoryOffset) {
    return gloam_vk_context.BindImageMemory(device, image, memory, memoryOffset);
}
GLOAM_FORCE_INLINE void vkCmdBeginQuery(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t query, VkQueryControlFlags flags) {
    gloam_vk_context.CmdBeginQuery(commandBuffer, queryPool, query, flags);
}
GLOAM_FORCE_INLINE void vkCmdBeginRenderPass(VkCommandBuffer commandBuffer, const VkRenderPassBeginInfo* pRenderPassBegin, VkSubpassContents contents) {
    gloam_vk_context.CmdBeginRenderPass(commandBuffer, pRenderPassBegin, contents);
}
GLOAM_FORCE_INLINE void vkCmdBindDescriptorSets(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipelineLayout layout, uint32_t firstSet, uint32_t descriptorSetCount, const VkDescriptorSet* pDescriptorSets, uint32_t dynamicOffsetCount, const uint32_t* pDynamicOffsets) {
    gloam_vk_context.CmdBindDescriptorSets(commandBuffer, pipelineBindPoint, layout, firstSet, descriptorSetCount, pDescriptorSets, dynamicOffsetCount, pDynamicOffsets);
}
GLOAM_FORCE_INLINE void vkCmdBindIndexBuffer(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkIndexType indexType) {
    gloam_vk_context.CmdBindIndexBuffer(commandBuffer, buffer, offset, indexType);
}
GLOAM_FORCE_INLINE void vkCmdBindPipeline(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipeline pipeline) {
    gloam_vk_context.CmdBindPipeline(commandBuffer, pipelineBindPoint, pipeline);
}
GLOAM_FORCE_INLINE void vkCmdBindVertexBuffers(VkCommandBuffer commandBuffer, uint32_t firstBinding, uint32_t bindingCount, const VkBuffer* pBuffers, const VkDeviceSize* pOffsets) {
    gloam_vk_context.CmdBindVertexBuffers(commandBuffer, firstBinding, bindingCount, pBuffers, pOffsets);
}
GLOAM_FORCE_INLINE void vkCmdBlitImage(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkImage dstImage, VkImageLayout dstImageLayout, uint32_t regionCount, const VkImageBlit* pRegions, VkFilter filter) {
    gloam_vk_context.CmdBlitImage(commandBuffer, srcImage, srcImageLayout, dstImage, dstImageLayout, regionCount, pRegions, filter);
}
GLOAM_FORCE_INLINE void vkCmdClearAttachments(VkCommandBuffer commandBuffer, uint32_t attachmentCount, const VkClearAttachment* pAttachments, uint32_t rectCount, const VkClearRect* pRects) {
    gloam_vk_context.CmdClearAttachments(commandBuffer, attachmentCount, pAttachments, rectCount, pRects);
}
GLOAM_FORCE_INLINE void vkCmdClearColorImage(VkCommandBuffer commandBuffer, VkImage image, VkImageLayout imageLayout, const VkClearColorValue* pColor, uint32_t rangeCount, const VkImageSubresourceRange* pRanges) {
    gloam_vk_context.CmdClearColorImage(commandBuffer, image, imageLayout, pColor, rangeCount, pRanges);
}
GLOAM_FORCE_INLINE void vkCmdClearDepthStencilImage(VkCommandBuffer commandBuffer, VkImage image, VkImageLayout imageLayout, const VkClearDepthStencilValue* pDepthStencil, uint32_t rangeCount, const VkImageSubresourceRange* pRanges) {
    gloam_vk_context.CmdClearDepthStencilImage(commandBuffer, image, imageLayout, pDepthStencil, rangeCount, pRanges);
}
GLOAM_FORCE_INLINE void vkCmdCopyBuffer(VkCommandBuffer commandBuffer, VkBuffer srcBuffer, VkBuffer dstBuffer, uint32_t regionCount, const VkBufferCopy* pRegions) {
    gloam_vk_context.CmdCopyBuffer(commandBuffer, srcBuffer, dstBuffer, regionCount, pRegions);
}
GLOAM_FORCE_INLINE void vkCmdCopyBufferToImage(VkCommandBuffer commandBuffer, VkBuffer srcBuffer, VkImage dstImage, VkImageLayout dstImageLayout, uint32_t regionCount, const VkBufferImageCopy* pRegions) {
    gloam_vk_context.CmdCopyBufferToImage(commandBuffer, srcBuffer, dstImage, dstImageLayout, regionCount, pRegions);
}
GLOAM_FORCE_INLINE void vkCmdCopyImage(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkImage dstImage, VkImageLayout dstImageLayout, uint32_t regionCount, const VkImageCopy* pRegions) {
    gloam_vk_context.CmdCopyImage(commandBuffer, srcImage, srcImageLayout, dstImage, dstImageLayout, regionCount, pRegions);
}
GLOAM_FORCE_INLINE void vkCmdCopyImageToBuffer(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkBuffer dstBuffer, uint32_t regionCount, const VkBufferImageCopy* pRegions) {
    gloam_vk_context.CmdCopyImageToBuffer(commandBuffer, srcImage, srcImageLayout, dstBuffer, regionCount, pRegions);
}
GLOAM_FORCE_INLINE void vkCmdCopyQueryPoolResults(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t firstQuery, uint32_t queryCount, VkBuffer dstBuffer, VkDeviceSize dstOffset, VkDeviceSize stride, VkQueryResultFlags flags) {
    gloam_vk_context.CmdCopyQueryPoolResults(commandBuffer, queryPool, firstQuery, queryCount, dstBuffer, dstOffset, stride, flags);
}
GLOAM_FORCE_INLINE void vkCmdDispatch(VkCommandBuffer commandBuffer, uint32_t groupCountX, uint32_t groupCountY, uint32_t groupCountZ) {
    gloam_vk_context.CmdDispatch(commandBuffer, groupCountX, groupCountY, groupCountZ);
}
GLOAM_FORCE_INLINE void vkCmdDispatchIndirect(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset) {
    gloam_vk_context.CmdDispatchIndirect(commandBuffer, buffer, offset);
}
GLOAM_FORCE_INLINE void vkCmdDraw(VkCommandBuffer commandBuffer, uint32_t vertexCount, uint32_t instanceCount, uint32_t firstVertex, uint32_t firstInstance) {
    gloam_vk_context.CmdDraw(commandBuffer, vertexCount, instanceCount, firstVertex, firstInstance);
}
GLOAM_FORCE_INLINE void vkCmdDrawIndexed(VkCommandBuffer commandBuffer, uint32_t indexCount, uint32_t instanceCount, uint32_t firstIndex, int32_t vertexOffset, uint32_t firstInstance) {
    gloam_vk_context.CmdDrawIndexed(commandBuffer, indexCount, instanceCount, firstIndex, vertexOffset, firstInstance);
}
GLOAM_FORCE_INLINE void vkCmdDrawIndexedIndirect(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, uint32_t drawCount, uint32_t stride) {
    gloam_vk_context.CmdDrawIndexedIndirect(commandBuffer, buffer, offset, drawCount, stride);
}
GLOAM_FORCE_INLINE void vkCmdDrawIndirect(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, uint32_t drawCount, uint32_t stride) {
    gloam_vk_context.CmdDrawIndirect(commandBuffer, buffer, offset, drawCount, stride);
}
GLOAM_FORCE_INLINE void vkCmdEndQuery(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t query) {
    gloam_vk_context.CmdEndQuery(commandBuffer, queryPool, query);
}
GLOAM_FORCE_INLINE void vkCmdEndRenderPass(VkCommandBuffer commandBuffer) {
    gloam_vk_context.CmdEndRenderPass(commandBuffer);
}
GLOAM_FORCE_INLINE void vkCmdExecuteCommands(VkCommandBuffer commandBuffer, uint32_t commandBufferCount, const VkCommandBuffer* pCommandBuffers) {
    gloam_vk_context.CmdExecuteCommands(commandBuffer, commandBufferCount, pCommandBuffers);
}
GLOAM_FORCE_INLINE void vkCmdFillBuffer(VkCommandBuffer commandBuffer, VkBuffer dstBuffer, VkDeviceSize dstOffset, VkDeviceSize size, uint32_t data) {
    gloam_vk_context.CmdFillBuffer(commandBuffer, dstBuffer, dstOffset, size, data);
}
GLOAM_FORCE_INLINE void vkCmdNextSubpass(VkCommandBuffer commandBuffer, VkSubpassContents contents) {
    gloam_vk_context.CmdNextSubpass(commandBuffer, contents);
}
GLOAM_FORCE_INLINE void vkCmdPipelineBarrier(VkCommandBuffer commandBuffer, VkPipelineStageFlags srcStageMask, VkPipelineStageFlags dstStageMask, VkDependencyFlags dependencyFlags, uint32_t memoryBarrierCount, const VkMemoryBarrier* pMemoryBarriers, uint32_t bufferMemoryBarrierCount, const VkBufferMemoryBarrier* pBufferMemoryBarriers, uint32_t imageMemoryBarrierCount, const VkImageMemoryBarrier* pImageMemoryBarriers) {
    gloam_vk_context.CmdPipelineBarrier(commandBuffer, srcStageMask, dstStageMask, dependencyFlags, memoryBarrierCount, pMemoryBarriers, bufferMemoryBarrierCount, pBufferMemoryBarriers, imageMemoryBarrierCount, pImageMemoryBarriers);
}
GLOAM_FORCE_INLINE void vkCmdPushConstants(VkCommandBuffer commandBuffer, VkPipelineLayout layout, VkShaderStageFlags stageFlags, uint32_t offset, uint32_t size, const void* pValues) {
    gloam_vk_context.CmdPushConstants(commandBuffer, layout, stageFlags, offset, size, pValues);
}
GLOAM_FORCE_INLINE void vkCmdResetEvent(VkCommandBuffer commandBuffer, VkEvent event, VkPipelineStageFlags stageMask) {
    gloam_vk_context.CmdResetEvent(commandBuffer, event, stageMask);
}
GLOAM_FORCE_INLINE void vkCmdResetQueryPool(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t firstQuery, uint32_t queryCount) {
    gloam_vk_context.CmdResetQueryPool(commandBuffer, queryPool, firstQuery, queryCount);
}
GLOAM_FORCE_INLINE void vkCmdResolveImage(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkImage dstImage, VkImageLayout dstImageLayout, uint32_t regionCount, const VkImageResolve* pRegions) {
    gloam_vk_context.CmdResolveImage(commandBuffer, srcImage, srcImageLayout, dstImage, dstImageLayout, regionCount, pRegions);
}
GLOAM_FORCE_INLINE void vkCmdSetBlendConstants(VkCommandBuffer commandBuffer, const float blendConstants[4]) {
    gloam_vk_context.CmdSetBlendConstants(commandBuffer, blendConstants);
}
GLOAM_FORCE_INLINE void vkCmdSetDepthBias(VkCommandBuffer commandBuffer, float depthBiasConstantFactor, float depthBiasClamp, float depthBiasSlopeFactor) {
    gloam_vk_context.CmdSetDepthBias(commandBuffer, depthBiasConstantFactor, depthBiasClamp, depthBiasSlopeFactor);
}
GLOAM_FORCE_INLINE void vkCmdSetDepthBounds(VkCommandBuffer commandBuffer, float minDepthBounds, float maxDepthBounds) {
    gloam_vk_context.CmdSetDepthBounds(commandBuffer, minDepthBounds, maxDepthBounds);
}
GLOAM_FORCE_INLINE void vkCmdSetEvent(VkCommandBuffer commandBuffer, VkEvent event, VkPipelineStageFlags stageMask) {
    gloam_vk_context.CmdSetEvent(commandBuffer, event, stageMask);
}
GLOAM_FORCE_INLINE void vkCmdSetLineWidth(VkCommandBuffer commandBuffer, float lineWidth) {
    gloam_vk_context.CmdSetLineWidth(commandBuffer, lineWidth);
}
GLOAM_FORCE_INLINE void vkCmdSetScissor(VkCommandBuffer commandBuffer, uint32_t firstScissor, uint32_t scissorCount, const VkRect2D* pScissors) {
    gloam_vk_context.CmdSetScissor(commandBuffer, firstScissor, scissorCount, pScissors);
}
GLOAM_FORCE_INLINE void vkCmdSetStencilCompareMask(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, uint32_t compareMask) {
    gloam_vk_context.CmdSetStencilCompareMask(commandBuffer, faceMask, compareMask);
}
GLOAM_FORCE_INLINE void vkCmdSetStencilReference(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, uint32_t reference) {
    gloam_vk_context.CmdSetStencilReference(commandBuffer, faceMask, reference);
}
GLOAM_FORCE_INLINE void vkCmdSetStencilWriteMask(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, uint32_t writeMask) {
    gloam_vk_context.CmdSetStencilWriteMask(commandBuffer, faceMask, writeMask);
}
GLOAM_FORCE_INLINE void vkCmdSetViewport(VkCommandBuffer commandBuffer, uint32_t firstViewport, uint32_t viewportCount, const VkViewport* pViewports) {
    gloam_vk_context.CmdSetViewport(commandBuffer, firstViewport, viewportCount, pViewports);
}
GLOAM_FORCE_INLINE void vkCmdUpdateBuffer(VkCommandBuffer commandBuffer, VkBuffer dstBuffer, VkDeviceSize dstOffset, VkDeviceSize dataSize, const void* pData) {
    gloam_vk_context.CmdUpdateBuffer(commandBuffer, dstBuffer, dstOffset, dataSize, pData);
}
GLOAM_FORCE_INLINE void vkCmdWaitEvents(VkCommandBuffer commandBuffer, uint32_t eventCount, const VkEvent* pEvents, VkPipelineStageFlags srcStageMask, VkPipelineStageFlags dstStageMask, uint32_t memoryBarrierCount, const VkMemoryBarrier* pMemoryBarriers, uint32_t bufferMemoryBarrierCount, const VkBufferMemoryBarrier* pBufferMemoryBarriers, uint32_t imageMemoryBarrierCount, const VkImageMemoryBarrier* pImageMemoryBarriers) {
    gloam_vk_context.CmdWaitEvents(commandBuffer, eventCount, pEvents, srcStageMask, dstStageMask, memoryBarrierCount, pMemoryBarriers, bufferMemoryBarrierCount, pBufferMemoryBarriers, imageMemoryBarrierCount, pImageMemoryBarriers);
}
GLOAM_FORCE_INLINE void vkCmdWriteTimestamp(VkCommandBuffer commandBuffer, VkPipelineStageFlagBits pipelineStage, VkQueryPool queryPool, uint32_t query) {
    gloam_vk_context.CmdWriteTimestamp(commandBuffer, pipelineStage, queryPool, query);
}
GLOAM_FORCE_INLINE VkResult vkCreateBuffer(VkDevice device, const VkBufferCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkBuffer* pBuffer) {
    return gloam_vk_context.CreateBuffer(device, pCreateInfo, pAllocator, pBuffer);
}
GLOAM_FORCE_INLINE VkResult vkCreateBufferView(VkDevice device, const VkBufferViewCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkBufferView* pView) {
    return gloam_vk_context.CreateBufferView(device, pCreateInfo, pAllocator, pView);
}
GLOAM_FORCE_INLINE VkResult vkCreateCommandPool(VkDevice device, const VkCommandPoolCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkCommandPool* pCommandPool) {
    return gloam_vk_context.CreateCommandPool(device, pCreateInfo, pAllocator, pCommandPool);
}
GLOAM_FORCE_INLINE VkResult vkCreateComputePipelines(VkDevice device, VkPipelineCache pipelineCache, uint32_t createInfoCount, const VkComputePipelineCreateInfo* pCreateInfos, const VkAllocationCallbacks* pAllocator, VkPipeline* pPipelines) {
    return gloam_vk_context.CreateComputePipelines(device, pipelineCache, createInfoCount, pCreateInfos, pAllocator, pPipelines);
}
GLOAM_FORCE_INLINE VkResult vkCreateDescriptorPool(VkDevice device, const VkDescriptorPoolCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDescriptorPool* pDescriptorPool) {
    return gloam_vk_context.CreateDescriptorPool(device, pCreateInfo, pAllocator, pDescriptorPool);
}
GLOAM_FORCE_INLINE VkResult vkCreateDescriptorSetLayout(VkDevice device, const VkDescriptorSetLayoutCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDescriptorSetLayout* pSetLayout) {
    return gloam_vk_context.CreateDescriptorSetLayout(device, pCreateInfo, pAllocator, pSetLayout);
}
GLOAM_FORCE_INLINE VkResult vkCreateDevice(VkPhysicalDevice physicalDevice, const VkDeviceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDevice* pDevice) {
    return gloam_vk_context.CreateDevice(physicalDevice, pCreateInfo, pAllocator, pDevice);
}
GLOAM_FORCE_INLINE VkResult vkCreateEvent(VkDevice device, const VkEventCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkEvent* pEvent) {
    return gloam_vk_context.CreateEvent(device, pCreateInfo, pAllocator, pEvent);
}
GLOAM_FORCE_INLINE VkResult vkCreateFence(VkDevice device, const VkFenceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkFence* pFence) {
    return gloam_vk_context.CreateFence(device, pCreateInfo, pAllocator, pFence);
}
GLOAM_FORCE_INLINE VkResult vkCreateFramebuffer(VkDevice device, const VkFramebufferCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkFramebuffer* pFramebuffer) {
    return gloam_vk_context.CreateFramebuffer(device, pCreateInfo, pAllocator, pFramebuffer);
}
GLOAM_FORCE_INLINE VkResult vkCreateGraphicsPipelines(VkDevice device, VkPipelineCache pipelineCache, uint32_t createInfoCount, const VkGraphicsPipelineCreateInfo* pCreateInfos, const VkAllocationCallbacks* pAllocator, VkPipeline* pPipelines) {
    return gloam_vk_context.CreateGraphicsPipelines(device, pipelineCache, createInfoCount, pCreateInfos, pAllocator, pPipelines);
}
GLOAM_FORCE_INLINE VkResult vkCreateImage(VkDevice device, const VkImageCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkImage* pImage) {
    return gloam_vk_context.CreateImage(device, pCreateInfo, pAllocator, pImage);
}
GLOAM_FORCE_INLINE VkResult vkCreateImageView(VkDevice device, const VkImageViewCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkImageView* pView) {
    return gloam_vk_context.CreateImageView(device, pCreateInfo, pAllocator, pView);
}
GLOAM_FORCE_INLINE VkResult vkCreateInstance(const VkInstanceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkInstance* pInstance) {
    return gloam_vk_context.CreateInstance(pCreateInfo, pAllocator, pInstance);
}
GLOAM_FORCE_INLINE VkResult vkCreatePipelineCache(VkDevice device, const VkPipelineCacheCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkPipelineCache* pPipelineCache) {
    return gloam_vk_context.CreatePipelineCache(device, pCreateInfo, pAllocator, pPipelineCache);
}
GLOAM_FORCE_INLINE VkResult vkCreatePipelineLayout(VkDevice device, const VkPipelineLayoutCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkPipelineLayout* pPipelineLayout) {
    return gloam_vk_context.CreatePipelineLayout(device, pCreateInfo, pAllocator, pPipelineLayout);
}
GLOAM_FORCE_INLINE VkResult vkCreateQueryPool(VkDevice device, const VkQueryPoolCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkQueryPool* pQueryPool) {
    return gloam_vk_context.CreateQueryPool(device, pCreateInfo, pAllocator, pQueryPool);
}
GLOAM_FORCE_INLINE VkResult vkCreateRenderPass(VkDevice device, const VkRenderPassCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkRenderPass* pRenderPass) {
    return gloam_vk_context.CreateRenderPass(device, pCreateInfo, pAllocator, pRenderPass);
}
GLOAM_FORCE_INLINE VkResult vkCreateSampler(VkDevice device, const VkSamplerCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSampler* pSampler) {
    return gloam_vk_context.CreateSampler(device, pCreateInfo, pAllocator, pSampler);
}
GLOAM_FORCE_INLINE VkResult vkCreateSemaphore(VkDevice device, const VkSemaphoreCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSemaphore* pSemaphore) {
    return gloam_vk_context.CreateSemaphore(device, pCreateInfo, pAllocator, pSemaphore);
}
GLOAM_FORCE_INLINE VkResult vkCreateShaderModule(VkDevice device, const VkShaderModuleCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkShaderModule* pShaderModule) {
    return gloam_vk_context.CreateShaderModule(device, pCreateInfo, pAllocator, pShaderModule);
}
GLOAM_FORCE_INLINE void vkDestroyBuffer(VkDevice device, VkBuffer buffer, const VkAllocationCallbacks* pAllocator) {
    gloam_vk_context.DestroyBuffer(device, buffer, pAllocator);
}
GLOAM_FORCE_INLINE void vkDestroyBufferView(VkDevice device, VkBufferView bufferView, const VkAllocationCallbacks* pAllocator) {
    gloam_vk_context.DestroyBufferView(device, bufferView, pAllocator);
}
GLOAM_FORCE_INLINE void vkDestroyCommandPool(VkDevice device, VkCommandPool commandPool, const VkAllocationCallbacks* pAllocator) {
    gloam_vk_context.DestroyCommandPool(device, commandPool, pAllocator);
}
GLOAM_FORCE_INLINE void vkDestroyDescriptorPool(VkDevice device, VkDescriptorPool descriptorPool, const VkAllocationCallbacks* pAllocator) {
    gloam_vk_context.DestroyDescriptorPool(device, descriptorPool, pAllocator);
}
GLOAM_FORCE_INLINE void vkDestroyDescriptorSetLayout(VkDevice device, VkDescriptorSetLayout descriptorSetLayout, const VkAllocationCallbacks* pAllocator) {
    gloam_vk_context.DestroyDescriptorSetLayout(device, descriptorSetLayout, pAllocator);
}
GLOAM_FORCE_INLINE void vkDestroyDevice(VkDevice device, const VkAllocationCallbacks* pAllocator) {
    gloam_vk_context.DestroyDevice(device, pAllocator);
}
GLOAM_FORCE_INLINE void vkDestroyEvent(VkDevice device, VkEvent event, const VkAllocationCallbacks* pAllocator) {
    gloam_vk_context.DestroyEvent(device, event, pAllocator);
}
GLOAM_FORCE_INLINE void vkDestroyFence(VkDevice device, VkFence fence, const VkAllocationCallbacks* pAllocator) {
    gloam_vk_context.DestroyFence(device, fence, pAllocator);
}
GLOAM_FORCE_INLINE void vkDestroyFramebuffer(VkDevice device, VkFramebuffer framebuffer, const VkAllocationCallbacks* pAllocator) {
    gloam_vk_context.DestroyFramebuffer(device, framebuffer, pAllocator);
}
GLOAM_FORCE_INLINE void vkDestroyImage(VkDevice device, VkImage image, const VkAllocationCallbacks* pAllocator) {
    gloam_vk_context.DestroyImage(device, image, pAllocator);
}
GLOAM_FORCE_INLINE void vkDestroyImageView(VkDevice device, VkImageView imageView, const VkAllocationCallbacks* pAllocator) {
    gloam_vk_context.DestroyImageView(device, imageView, pAllocator);
}
GLOAM_FORCE_INLINE void vkDestroyInstance(VkInstance instance, const VkAllocationCallbacks* pAllocator) {
    gloam_vk_context.DestroyInstance(instance, pAllocator);
}
GLOAM_FORCE_INLINE void vkDestroyPipeline(VkDevice device, VkPipeline pipeline, const VkAllocationCallbacks* pAllocator) {
    gloam_vk_context.DestroyPipeline(device, pipeline, pAllocator);
}
GLOAM_FORCE_INLINE void vkDestroyPipelineCache(VkDevice device, VkPipelineCache pipelineCache, const VkAllocationCallbacks* pAllocator) {
    gloam_vk_context.DestroyPipelineCache(device, pipelineCache, pAllocator);
}
GLOAM_FORCE_INLINE void vkDestroyPipelineLayout(VkDevice device, VkPipelineLayout pipelineLayout, const VkAllocationCallbacks* pAllocator) {
    gloam_vk_context.DestroyPipelineLayout(device, pipelineLayout, pAllocator);
}
GLOAM_FORCE_INLINE void vkDestroyQueryPool(VkDevice device, VkQueryPool queryPool, const VkAllocationCallbacks* pAllocator) {
    gloam_vk_context.DestroyQueryPool(device, queryPool, pAllocator);
}
GLOAM_FORCE_INLINE void vkDestroyRenderPass(VkDevice device, VkRenderPass renderPass, const VkAllocationCallbacks* pAllocator) {
    gloam_vk_context.DestroyRenderPass(device, renderPass, pAllocator);
}
GLOAM_FORCE_INLINE void vkDestroySampler(VkDevice device, VkSampler sampler, const VkAllocationCallbacks* pAllocator) {
    gloam_vk_context.DestroySampler(device, sampler, pAllocator);
}
GLOAM_FORCE_INLINE void vkDestroySemaphore(VkDevice device, VkSemaphore semaphore, const VkAllocationCallbacks* pAllocator) {
    gloam_vk_context.DestroySemaphore(device, semaphore, pAllocator);
}
GLOAM_FORCE_INLINE void vkDestroyShaderModule(VkDevice device, VkShaderModule shaderModule, const VkAllocationCallbacks* pAllocator) {
    gloam_vk_context.DestroyShaderModule(device, shaderModule, pAllocator);
}
GLOAM_FORCE_INLINE VkResult vkDeviceWaitIdle(VkDevice device) {
    return gloam_vk_context.DeviceWaitIdle(device);
}
GLOAM_FORCE_INLINE VkResult vkEndCommandBuffer(VkCommandBuffer commandBuffer) {
    return gloam_vk_context.EndCommandBuffer(commandBuffer);
}
GLOAM_FORCE_INLINE VkResult vkEnumerateDeviceExtensionProperties(VkPhysicalDevice physicalDevice, const char* pLayerName, uint32_t* pPropertyCount, VkExtensionProperties* pProperties) {
    return gloam_vk_context.EnumerateDeviceExtensionProperties(physicalDevice, pLayerName, pPropertyCount, pProperties);
}
GLOAM_FORCE_INLINE VkResult vkEnumerateDeviceLayerProperties(VkPhysicalDevice physicalDevice, uint32_t* pPropertyCount, VkLayerProperties* pProperties) {
    return gloam_vk_context.EnumerateDeviceLayerProperties(physicalDevice, pPropertyCount, pProperties);
}
GLOAM_FORCE_INLINE VkResult vkEnumerateInstanceExtensionProperties(const char* pLayerName, uint32_t* pPropertyCount, VkExtensionProperties* pProperties) {
    return gloam_vk_context.EnumerateInstanceExtensionProperties(pLayerName, pPropertyCount, pProperties);
}
GLOAM_FORCE_INLINE VkResult vkEnumerateInstanceLayerProperties(uint32_t* pPropertyCount, VkLayerProperties* pProperties) {
    return gloam_vk_context.EnumerateInstanceLayerProperties(pPropertyCount, pProperties);
}
GLOAM_FORCE_INLINE VkResult vkEnumeratePhysicalDevices(VkInstance instance, uint32_t* pPhysicalDeviceCount, VkPhysicalDevice* pPhysicalDevices) {
    return gloam_vk_context.EnumeratePhysicalDevices(instance, pPhysicalDeviceCount, pPhysicalDevices);
}
GLOAM_FORCE_INLINE VkResult vkFlushMappedMemoryRanges(VkDevice device, uint32_t memoryRangeCount, const VkMappedMemoryRange* pMemoryRanges) {
    return gloam_vk_context.FlushMappedMemoryRanges(device, memoryRangeCount, pMemoryRanges);
}
GLOAM_FORCE_INLINE void vkFreeCommandBuffers(VkDevice device, VkCommandPool commandPool, uint32_t commandBufferCount, const VkCommandBuffer* pCommandBuffers) {
    gloam_vk_context.FreeCommandBuffers(device, commandPool, commandBufferCount, pCommandBuffers);
}
GLOAM_FORCE_INLINE VkResult vkFreeDescriptorSets(VkDevice device, VkDescriptorPool descriptorPool, uint32_t descriptorSetCount, const VkDescriptorSet* pDescriptorSets) {
    return gloam_vk_context.FreeDescriptorSets(device, descriptorPool, descriptorSetCount, pDescriptorSets);
}
GLOAM_FORCE_INLINE void vkFreeMemory(VkDevice device, VkDeviceMemory memory, const VkAllocationCallbacks* pAllocator) {
    gloam_vk_context.FreeMemory(device, memory, pAllocator);
}
GLOAM_FORCE_INLINE void vkGetBufferMemoryRequirements(VkDevice device, VkBuffer buffer, VkMemoryRequirements* pMemoryRequirements) {
    gloam_vk_context.GetBufferMemoryRequirements(device, buffer, pMemoryRequirements);
}
GLOAM_FORCE_INLINE void vkGetDeviceMemoryCommitment(VkDevice device, VkDeviceMemory memory, VkDeviceSize* pCommittedMemoryInBytes) {
    gloam_vk_context.GetDeviceMemoryCommitment(device, memory, pCommittedMemoryInBytes);
}
GLOAM_FORCE_INLINE PFN_vkVoidFunction vkGetDeviceProcAddr(VkDevice device, const char* pName) {
    return gloam_vk_context.GetDeviceProcAddr(device, pName);
}
GLOAM_FORCE_INLINE void vkGetDeviceQueue(VkDevice device, uint32_t queueFamilyIndex, uint32_t queueIndex, VkQueue* pQueue) {
    gloam_vk_context.GetDeviceQueue(device, queueFamilyIndex, queueIndex, pQueue);
}
GLOAM_FORCE_INLINE VkResult vkGetEventStatus(VkDevice device, VkEvent event) {
    return gloam_vk_context.GetEventStatus(device, event);
}
GLOAM_FORCE_INLINE VkResult vkGetFenceStatus(VkDevice device, VkFence fence) {
    return gloam_vk_context.GetFenceStatus(device, fence);
}
GLOAM_FORCE_INLINE void vkGetImageMemoryRequirements(VkDevice device, VkImage image, VkMemoryRequirements* pMemoryRequirements) {
    gloam_vk_context.GetImageMemoryRequirements(device, image, pMemoryRequirements);
}
GLOAM_FORCE_INLINE void vkGetImageSparseMemoryRequirements(VkDevice device, VkImage image, uint32_t* pSparseMemoryRequirementCount, VkSparseImageMemoryRequirements* pSparseMemoryRequirements) {
    gloam_vk_context.GetImageSparseMemoryRequirements(device, image, pSparseMemoryRequirementCount, pSparseMemoryRequirements);
}
GLOAM_FORCE_INLINE void vkGetImageSubresourceLayout(VkDevice device, VkImage image, const VkImageSubresource* pSubresource, VkSubresourceLayout* pLayout) {
    gloam_vk_context.GetImageSubresourceLayout(device, image, pSubresource, pLayout);
}
GLOAM_FORCE_INLINE PFN_vkVoidFunction vkGetInstanceProcAddr(VkInstance instance, const char* pName) {
    return gloam_vk_context.GetInstanceProcAddr(instance, pName);
}
GLOAM_FORCE_INLINE void vkGetPhysicalDeviceFeatures(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures* pFeatures) {
    gloam_vk_context.GetPhysicalDeviceFeatures(physicalDevice, pFeatures);
}
GLOAM_FORCE_INLINE void vkGetPhysicalDeviceFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties* pFormatProperties) {
    gloam_vk_context.GetPhysicalDeviceFormatProperties(physicalDevice, format, pFormatProperties);
}
GLOAM_FORCE_INLINE VkResult vkGetPhysicalDeviceImageFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkImageTiling tiling, VkImageUsageFlags usage, VkImageCreateFlags flags, VkImageFormatProperties* pImageFormatProperties) {
    return gloam_vk_context.GetPhysicalDeviceImageFormatProperties(physicalDevice, format, type, tiling, usage, flags, pImageFormatProperties);
}
GLOAM_FORCE_INLINE void vkGetPhysicalDeviceMemoryProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties* pMemoryProperties) {
    gloam_vk_context.GetPhysicalDeviceMemoryProperties(physicalDevice, pMemoryProperties);
}
GLOAM_FORCE_INLINE void vkGetPhysicalDeviceProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties* pProperties) {
    gloam_vk_context.GetPhysicalDeviceProperties(physicalDevice, pProperties);
}
GLOAM_FORCE_INLINE void vkGetPhysicalDeviceQueueFamilyProperties(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties* pQueueFamilyProperties) {
    gloam_vk_context.GetPhysicalDeviceQueueFamilyProperties(physicalDevice, pQueueFamilyPropertyCount, pQueueFamilyProperties);
}
GLOAM_FORCE_INLINE void vkGetPhysicalDeviceSparseImageFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkSampleCountFlagBits samples, VkImageUsageFlags usage, VkImageTiling tiling, uint32_t* pPropertyCount, VkSparseImageFormatProperties* pProperties) {
    gloam_vk_context.GetPhysicalDeviceSparseImageFormatProperties(physicalDevice, format, type, samples, usage, tiling, pPropertyCount, pProperties);
}
GLOAM_FORCE_INLINE VkResult vkGetPipelineCacheData(VkDevice device, VkPipelineCache pipelineCache, size_t* pDataSize, void* pData) {
    return gloam_vk_context.GetPipelineCacheData(device, pipelineCache, pDataSize, pData);
}
GLOAM_FORCE_INLINE VkResult vkGetQueryPoolResults(VkDevice device, VkQueryPool queryPool, uint32_t firstQuery, uint32_t queryCount, size_t dataSize, void* pData, VkDeviceSize stride, VkQueryResultFlags flags) {
    return gloam_vk_context.GetQueryPoolResults(device, queryPool, firstQuery, queryCount, dataSize, pData, stride, flags);
}
GLOAM_FORCE_INLINE void vkGetRenderAreaGranularity(VkDevice device, VkRenderPass renderPass, VkExtent2D* pGranularity) {
    gloam_vk_context.GetRenderAreaGranularity(device, renderPass, pGranularity);
}
GLOAM_FORCE_INLINE VkResult vkInvalidateMappedMemoryRanges(VkDevice device, uint32_t memoryRangeCount, const VkMappedMemoryRange* pMemoryRanges) {
    return gloam_vk_context.InvalidateMappedMemoryRanges(device, memoryRangeCount, pMemoryRanges);
}
GLOAM_FORCE_INLINE VkResult vkMapMemory(VkDevice device, VkDeviceMemory memory, VkDeviceSize offset, VkDeviceSize size, VkMemoryMapFlags flags, void** ppData) {
    return gloam_vk_context.MapMemory(device, memory, offset, size, flags, ppData);
}
GLOAM_FORCE_INLINE VkResult vkMergePipelineCaches(VkDevice device, VkPipelineCache dstCache, uint32_t srcCacheCount, const VkPipelineCache* pSrcCaches) {
    return gloam_vk_context.MergePipelineCaches(device, dstCache, srcCacheCount, pSrcCaches);
}
GLOAM_FORCE_INLINE VkResult vkQueueBindSparse(VkQueue queue, uint32_t bindInfoCount, const VkBindSparseInfo* pBindInfo, VkFence fence) {
    return gloam_vk_context.QueueBindSparse(queue, bindInfoCount, pBindInfo, fence);
}
GLOAM_FORCE_INLINE VkResult vkQueueSubmit(VkQueue queue, uint32_t submitCount, const VkSubmitInfo* pSubmits, VkFence fence) {
    return gloam_vk_context.QueueSubmit(queue, submitCount, pSubmits, fence);
}
GLOAM_FORCE_INLINE VkResult vkQueueWaitIdle(VkQueue queue) {
    return gloam_vk_context.QueueWaitIdle(queue);
}
GLOAM_FORCE_INLINE VkResult vkResetCommandBuffer(VkCommandBuffer commandBuffer, VkCommandBufferResetFlags flags) {
    return gloam_vk_context.ResetCommandBuffer(commandBuffer, flags);
}
GLOAM_FORCE_INLINE VkResult vkResetCommandPool(VkDevice device, VkCommandPool commandPool, VkCommandPoolResetFlags flags) {
    return gloam_vk_context.ResetCommandPool(device, commandPool, flags);
}
GLOAM_FORCE_INLINE VkResult vkResetDescriptorPool(VkDevice device, VkDescriptorPool descriptorPool, VkDescriptorPoolResetFlags flags) {
    return gloam_vk_context.ResetDescriptorPool(device, descriptorPool, flags);
}
GLOAM_FORCE_INLINE VkResult vkResetEvent(VkDevice device, VkEvent event) {
    return gloam_vk_context.ResetEvent(device, event);
}
GLOAM_FORCE_INLINE VkResult vkResetFences(VkDevice device, uint32_t fenceCount, const VkFence* pFences) {
    return gloam_vk_context.ResetFences(device, fenceCount, pFences);
}
GLOAM_FORCE_INLINE VkResult vkSetEvent(VkDevice device, VkEvent event) {
    return gloam_vk_context.SetEvent(device, event);
}
GLOAM_FORCE_INLINE void vkUnmapMemory(VkDevice device, VkDeviceMemory memory) {
    gloam_vk_context.UnmapMemory(device, memory);
}
GLOAM_FORCE_INLINE void vkUpdateDescriptorSets(VkDevice device, uint32_t descriptorWriteCount, const VkWriteDescriptorSet* pDescriptorWrites, uint32_t descriptorCopyCount, const VkCopyDescriptorSet* pDescriptorCopies) {
    gloam_vk_context.UpdateDescriptorSets(device, descriptorWriteCount, pDescriptorWrites, descriptorCopyCount, pDescriptorCopies);
}
GLOAM_FORCE_INLINE VkResult vkWaitForFences(VkDevice device, uint32_t fenceCount, const VkFence* pFences, VkBool32 waitAll, uint64_t timeout) {
    return gloam_vk_context.WaitForFences(device, fenceCount, pFences, waitAll, timeout);
}
GLOAM_FORCE_INLINE VkResult vkBindBufferMemory2(VkDevice device, uint32_t bindInfoCount, const VkBindBufferMemoryInfo* pBindInfos) {
    return gloam_vk_context.BindBufferMemory2(device, bindInfoCount, pBindInfos);
}
GLOAM_FORCE_INLINE VkResult vkBindImageMemory2(VkDevice device, uint32_t bindInfoCount, const VkBindImageMemoryInfo* pBindInfos) {
    return gloam_vk_context.BindImageMemory2(device, bindInfoCount, pBindInfos);
}
GLOAM_FORCE_INLINE void vkCmdDispatchBase(VkCommandBuffer commandBuffer, uint32_t baseGroupX, uint32_t baseGroupY, uint32_t baseGroupZ, uint32_t groupCountX, uint32_t groupCountY, uint32_t groupCountZ) {
    gloam_vk_context.CmdDispatchBase(commandBuffer, baseGroupX, baseGroupY, baseGroupZ, groupCountX, groupCountY, groupCountZ);
}
GLOAM_FORCE_INLINE void vkCmdSetDeviceMask(VkCommandBuffer commandBuffer, uint32_t deviceMask) {
    gloam_vk_context.CmdSetDeviceMask(commandBuffer, deviceMask);
}
GLOAM_FORCE_INLINE VkResult vkCreateDescriptorUpdateTemplate(VkDevice device, const VkDescriptorUpdateTemplateCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDescriptorUpdateTemplate* pDescriptorUpdateTemplate) {
    return gloam_vk_context.CreateDescriptorUpdateTemplate(device, pCreateInfo, pAllocator, pDescriptorUpdateTemplate);
}
GLOAM_FORCE_INLINE VkResult vkCreateSamplerYcbcrConversion(VkDevice device, const VkSamplerYcbcrConversionCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSamplerYcbcrConversion* pYcbcrConversion) {
    return gloam_vk_context.CreateSamplerYcbcrConversion(device, pCreateInfo, pAllocator, pYcbcrConversion);
}
GLOAM_FORCE_INLINE void vkDestroyDescriptorUpdateTemplate(VkDevice device, VkDescriptorUpdateTemplate descriptorUpdateTemplate, const VkAllocationCallbacks* pAllocator) {
    gloam_vk_context.DestroyDescriptorUpdateTemplate(device, descriptorUpdateTemplate, pAllocator);
}
GLOAM_FORCE_INLINE void vkDestroySamplerYcbcrConversion(VkDevice device, VkSamplerYcbcrConversion ycbcrConversion, const VkAllocationCallbacks* pAllocator) {
    gloam_vk_context.DestroySamplerYcbcrConversion(device, ycbcrConversion, pAllocator);
}
GLOAM_FORCE_INLINE VkResult vkEnumerateInstanceVersion(uint32_t* pApiVersion) {
    return gloam_vk_context.EnumerateInstanceVersion(pApiVersion);
}
GLOAM_FORCE_INLINE VkResult vkEnumeratePhysicalDeviceGroups(VkInstance instance, uint32_t* pPhysicalDeviceGroupCount, VkPhysicalDeviceGroupProperties* pPhysicalDeviceGroupProperties) {
    return gloam_vk_context.EnumeratePhysicalDeviceGroups(instance, pPhysicalDeviceGroupCount, pPhysicalDeviceGroupProperties);
}
GLOAM_FORCE_INLINE void vkGetBufferMemoryRequirements2(VkDevice device, const VkBufferMemoryRequirementsInfo2* pInfo, VkMemoryRequirements2* pMemoryRequirements) {
    gloam_vk_context.GetBufferMemoryRequirements2(device, pInfo, pMemoryRequirements);
}
GLOAM_FORCE_INLINE void vkGetDescriptorSetLayoutSupport(VkDevice device, const VkDescriptorSetLayoutCreateInfo* pCreateInfo, VkDescriptorSetLayoutSupport* pSupport) {
    gloam_vk_context.GetDescriptorSetLayoutSupport(device, pCreateInfo, pSupport);
}
GLOAM_FORCE_INLINE void vkGetDeviceGroupPeerMemoryFeatures(VkDevice device, uint32_t heapIndex, uint32_t localDeviceIndex, uint32_t remoteDeviceIndex, VkPeerMemoryFeatureFlags* pPeerMemoryFeatures) {
    gloam_vk_context.GetDeviceGroupPeerMemoryFeatures(device, heapIndex, localDeviceIndex, remoteDeviceIndex, pPeerMemoryFeatures);
}
GLOAM_FORCE_INLINE void vkGetDeviceQueue2(VkDevice device, const VkDeviceQueueInfo2* pQueueInfo, VkQueue* pQueue) {
    gloam_vk_context.GetDeviceQueue2(device, pQueueInfo, pQueue);
}
GLOAM_FORCE_INLINE void vkGetImageMemoryRequirements2(VkDevice device, const VkImageMemoryRequirementsInfo2* pInfo, VkMemoryRequirements2* pMemoryRequirements) {
    gloam_vk_context.GetImageMemoryRequirements2(device, pInfo, pMemoryRequirements);
}
GLOAM_FORCE_INLINE void vkGetImageSparseMemoryRequirements2(VkDevice device, const VkImageSparseMemoryRequirementsInfo2* pInfo, uint32_t* pSparseMemoryRequirementCount, VkSparseImageMemoryRequirements2* pSparseMemoryRequirements) {
    gloam_vk_context.GetImageSparseMemoryRequirements2(device, pInfo, pSparseMemoryRequirementCount, pSparseMemoryRequirements);
}
GLOAM_FORCE_INLINE void vkGetPhysicalDeviceExternalBufferProperties(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceExternalBufferInfo* pExternalBufferInfo, VkExternalBufferProperties* pExternalBufferProperties) {
    gloam_vk_context.GetPhysicalDeviceExternalBufferProperties(physicalDevice, pExternalBufferInfo, pExternalBufferProperties);
}
GLOAM_FORCE_INLINE void vkGetPhysicalDeviceExternalFenceProperties(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceExternalFenceInfo* pExternalFenceInfo, VkExternalFenceProperties* pExternalFenceProperties) {
    gloam_vk_context.GetPhysicalDeviceExternalFenceProperties(physicalDevice, pExternalFenceInfo, pExternalFenceProperties);
}
GLOAM_FORCE_INLINE void vkGetPhysicalDeviceExternalSemaphoreProperties(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceExternalSemaphoreInfo* pExternalSemaphoreInfo, VkExternalSemaphoreProperties* pExternalSemaphoreProperties) {
    gloam_vk_context.GetPhysicalDeviceExternalSemaphoreProperties(physicalDevice, pExternalSemaphoreInfo, pExternalSemaphoreProperties);
}
GLOAM_FORCE_INLINE void vkGetPhysicalDeviceFeatures2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures2* pFeatures) {
    gloam_vk_context.GetPhysicalDeviceFeatures2(physicalDevice, pFeatures);
}
GLOAM_FORCE_INLINE void vkGetPhysicalDeviceFormatProperties2(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties2* pFormatProperties) {
    gloam_vk_context.GetPhysicalDeviceFormatProperties2(physicalDevice, format, pFormatProperties);
}
GLOAM_FORCE_INLINE VkResult vkGetPhysicalDeviceImageFormatProperties2(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceImageFormatInfo2* pImageFormatInfo, VkImageFormatProperties2* pImageFormatProperties) {
    return gloam_vk_context.GetPhysicalDeviceImageFormatProperties2(physicalDevice, pImageFormatInfo, pImageFormatProperties);
}
GLOAM_FORCE_INLINE void vkGetPhysicalDeviceMemoryProperties2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties2* pMemoryProperties) {
    gloam_vk_context.GetPhysicalDeviceMemoryProperties2(physicalDevice, pMemoryProperties);
}
GLOAM_FORCE_INLINE void vkGetPhysicalDeviceProperties2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties2* pProperties) {
    gloam_vk_context.GetPhysicalDeviceProperties2(physicalDevice, pProperties);
}
GLOAM_FORCE_INLINE void vkGetPhysicalDeviceQueueFamilyProperties2(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties2* pQueueFamilyProperties) {
    gloam_vk_context.GetPhysicalDeviceQueueFamilyProperties2(physicalDevice, pQueueFamilyPropertyCount, pQueueFamilyProperties);
}
GLOAM_FORCE_INLINE void vkGetPhysicalDeviceSparseImageFormatProperties2(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSparseImageFormatInfo2* pFormatInfo, uint32_t* pPropertyCount, VkSparseImageFormatProperties2* pProperties) {
    gloam_vk_context.GetPhysicalDeviceSparseImageFormatProperties2(physicalDevice, pFormatInfo, pPropertyCount, pProperties);
}
GLOAM_FORCE_INLINE void vkTrimCommandPool(VkDevice device, VkCommandPool commandPool, VkCommandPoolTrimFlags flags) {
    gloam_vk_context.TrimCommandPool(device, commandPool, flags);
}
GLOAM_FORCE_INLINE void vkUpdateDescriptorSetWithTemplate(VkDevice device, VkDescriptorSet descriptorSet, VkDescriptorUpdateTemplate descriptorUpdateTemplate, const void* pData) {
    gloam_vk_context.UpdateDescriptorSetWithTemplate(device, descriptorSet, descriptorUpdateTemplate, pData);
}
GLOAM_FORCE_INLINE void vkCmdBeginRenderPass2(VkCommandBuffer commandBuffer, const VkRenderPassBeginInfo* pRenderPassBegin, const VkSubpassBeginInfo* pSubpassBeginInfo) {
    gloam_vk_context.CmdBeginRenderPass2(commandBuffer, pRenderPassBegin, pSubpassBeginInfo);
}
GLOAM_FORCE_INLINE void vkCmdDrawIndexedIndirectCount(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkBuffer countBuffer, VkDeviceSize countBufferOffset, uint32_t maxDrawCount, uint32_t stride) {
    gloam_vk_context.CmdDrawIndexedIndirectCount(commandBuffer, buffer, offset, countBuffer, countBufferOffset, maxDrawCount, stride);
}
GLOAM_FORCE_INLINE void vkCmdDrawIndirectCount(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkBuffer countBuffer, VkDeviceSize countBufferOffset, uint32_t maxDrawCount, uint32_t stride) {
    gloam_vk_context.CmdDrawIndirectCount(commandBuffer, buffer, offset, countBuffer, countBufferOffset, maxDrawCount, stride);
}
GLOAM_FORCE_INLINE void vkCmdEndRenderPass2(VkCommandBuffer commandBuffer, const VkSubpassEndInfo* pSubpassEndInfo) {
    gloam_vk_context.CmdEndRenderPass2(commandBuffer, pSubpassEndInfo);
}
GLOAM_FORCE_INLINE void vkCmdNextSubpass2(VkCommandBuffer commandBuffer, const VkSubpassBeginInfo* pSubpassBeginInfo, const VkSubpassEndInfo* pSubpassEndInfo) {
    gloam_vk_context.CmdNextSubpass2(commandBuffer, pSubpassBeginInfo, pSubpassEndInfo);
}
GLOAM_FORCE_INLINE VkResult vkCreateRenderPass2(VkDevice device, const VkRenderPassCreateInfo2* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkRenderPass* pRenderPass) {
    return gloam_vk_context.CreateRenderPass2(device, pCreateInfo, pAllocator, pRenderPass);
}
GLOAM_FORCE_INLINE VkDeviceAddress vkGetBufferDeviceAddress(VkDevice device, const VkBufferDeviceAddressInfo* pInfo) {
    return gloam_vk_context.GetBufferDeviceAddress(device, pInfo);
}
GLOAM_FORCE_INLINE uint64_t vkGetBufferOpaqueCaptureAddress(VkDevice device, const VkBufferDeviceAddressInfo* pInfo) {
    return gloam_vk_context.GetBufferOpaqueCaptureAddress(device, pInfo);
}
GLOAM_FORCE_INLINE uint64_t vkGetDeviceMemoryOpaqueCaptureAddress(VkDevice device, const VkDeviceMemoryOpaqueCaptureAddressInfo* pInfo) {
    return gloam_vk_context.GetDeviceMemoryOpaqueCaptureAddress(device, pInfo);
}
GLOAM_FORCE_INLINE VkResult vkGetSemaphoreCounterValue(VkDevice device, VkSemaphore semaphore, uint64_t* pValue) {
    return gloam_vk_context.GetSemaphoreCounterValue(device, semaphore, pValue);
}
GLOAM_FORCE_INLINE void vkResetQueryPool(VkDevice device, VkQueryPool queryPool, uint32_t firstQuery, uint32_t queryCount) {
    gloam_vk_context.ResetQueryPool(device, queryPool, firstQuery, queryCount);
}
GLOAM_FORCE_INLINE VkResult vkSignalSemaphore(VkDevice device, const VkSemaphoreSignalInfo* pSignalInfo) {
    return gloam_vk_context.SignalSemaphore(device, pSignalInfo);
}
GLOAM_FORCE_INLINE VkResult vkWaitSemaphores(VkDevice device, const VkSemaphoreWaitInfo* pWaitInfo, uint64_t timeout) {
    return gloam_vk_context.WaitSemaphores(device, pWaitInfo, timeout);
}
GLOAM_FORCE_INLINE void vkCmdBeginRendering(VkCommandBuffer commandBuffer, const VkRenderingInfo* pRenderingInfo) {
    gloam_vk_context.CmdBeginRendering(commandBuffer, pRenderingInfo);
}
GLOAM_FORCE_INLINE void vkCmdBindVertexBuffers2(VkCommandBuffer commandBuffer, uint32_t firstBinding, uint32_t bindingCount, const VkBuffer* pBuffers, const VkDeviceSize* pOffsets, const VkDeviceSize* pSizes, const VkDeviceSize* pStrides) {
    gloam_vk_context.CmdBindVertexBuffers2(commandBuffer, firstBinding, bindingCount, pBuffers, pOffsets, pSizes, pStrides);
}
GLOAM_FORCE_INLINE void vkCmdBlitImage2(VkCommandBuffer commandBuffer, const VkBlitImageInfo2* pBlitImageInfo) {
    gloam_vk_context.CmdBlitImage2(commandBuffer, pBlitImageInfo);
}
GLOAM_FORCE_INLINE void vkCmdCopyBuffer2(VkCommandBuffer commandBuffer, const VkCopyBufferInfo2* pCopyBufferInfo) {
    gloam_vk_context.CmdCopyBuffer2(commandBuffer, pCopyBufferInfo);
}
GLOAM_FORCE_INLINE void vkCmdCopyBufferToImage2(VkCommandBuffer commandBuffer, const VkCopyBufferToImageInfo2* pCopyBufferToImageInfo) {
    gloam_vk_context.CmdCopyBufferToImage2(commandBuffer, pCopyBufferToImageInfo);
}
GLOAM_FORCE_INLINE void vkCmdCopyImage2(VkCommandBuffer commandBuffer, const VkCopyImageInfo2* pCopyImageInfo) {
    gloam_vk_context.CmdCopyImage2(commandBuffer, pCopyImageInfo);
}
GLOAM_FORCE_INLINE void vkCmdCopyImageToBuffer2(VkCommandBuffer commandBuffer, const VkCopyImageToBufferInfo2* pCopyImageToBufferInfo) {
    gloam_vk_context.CmdCopyImageToBuffer2(commandBuffer, pCopyImageToBufferInfo);
}
GLOAM_FORCE_INLINE void vkCmdEndRendering(VkCommandBuffer commandBuffer) {
    gloam_vk_context.CmdEndRendering(commandBuffer);
}
GLOAM_FORCE_INLINE void vkCmdPipelineBarrier2(VkCommandBuffer commandBuffer, const VkDependencyInfo* pDependencyInfo) {
    gloam_vk_context.CmdPipelineBarrier2(commandBuffer, pDependencyInfo);
}
GLOAM_FORCE_INLINE void vkCmdResetEvent2(VkCommandBuffer commandBuffer, VkEvent event, VkPipelineStageFlags2 stageMask) {
    gloam_vk_context.CmdResetEvent2(commandBuffer, event, stageMask);
}
GLOAM_FORCE_INLINE void vkCmdResolveImage2(VkCommandBuffer commandBuffer, const VkResolveImageInfo2* pResolveImageInfo) {
    gloam_vk_context.CmdResolveImage2(commandBuffer, pResolveImageInfo);
}
GLOAM_FORCE_INLINE void vkCmdSetCullMode(VkCommandBuffer commandBuffer, VkCullModeFlags cullMode) {
    gloam_vk_context.CmdSetCullMode(commandBuffer, cullMode);
}
GLOAM_FORCE_INLINE void vkCmdSetDepthBiasEnable(VkCommandBuffer commandBuffer, VkBool32 depthBiasEnable) {
    gloam_vk_context.CmdSetDepthBiasEnable(commandBuffer, depthBiasEnable);
}
GLOAM_FORCE_INLINE void vkCmdSetDepthBoundsTestEnable(VkCommandBuffer commandBuffer, VkBool32 depthBoundsTestEnable) {
    gloam_vk_context.CmdSetDepthBoundsTestEnable(commandBuffer, depthBoundsTestEnable);
}
GLOAM_FORCE_INLINE void vkCmdSetDepthCompareOp(VkCommandBuffer commandBuffer, VkCompareOp depthCompareOp) {
    gloam_vk_context.CmdSetDepthCompareOp(commandBuffer, depthCompareOp);
}
GLOAM_FORCE_INLINE void vkCmdSetDepthTestEnable(VkCommandBuffer commandBuffer, VkBool32 depthTestEnable) {
    gloam_vk_context.CmdSetDepthTestEnable(commandBuffer, depthTestEnable);
}
GLOAM_FORCE_INLINE void vkCmdSetDepthWriteEnable(VkCommandBuffer commandBuffer, VkBool32 depthWriteEnable) {
    gloam_vk_context.CmdSetDepthWriteEnable(commandBuffer, depthWriteEnable);
}
GLOAM_FORCE_INLINE void vkCmdSetEvent2(VkCommandBuffer commandBuffer, VkEvent event, const VkDependencyInfo* pDependencyInfo) {
    gloam_vk_context.CmdSetEvent2(commandBuffer, event, pDependencyInfo);
}
GLOAM_FORCE_INLINE void vkCmdSetFrontFace(VkCommandBuffer commandBuffer, VkFrontFace frontFace) {
    gloam_vk_context.CmdSetFrontFace(commandBuffer, frontFace);
}
GLOAM_FORCE_INLINE void vkCmdSetPrimitiveRestartEnable(VkCommandBuffer commandBuffer, VkBool32 primitiveRestartEnable) {
    gloam_vk_context.CmdSetPrimitiveRestartEnable(commandBuffer, primitiveRestartEnable);
}
GLOAM_FORCE_INLINE void vkCmdSetPrimitiveTopology(VkCommandBuffer commandBuffer, VkPrimitiveTopology primitiveTopology) {
    gloam_vk_context.CmdSetPrimitiveTopology(commandBuffer, primitiveTopology);
}
GLOAM_FORCE_INLINE void vkCmdSetRasterizerDiscardEnable(VkCommandBuffer commandBuffer, VkBool32 rasterizerDiscardEnable) {
    gloam_vk_context.CmdSetRasterizerDiscardEnable(commandBuffer, rasterizerDiscardEnable);
}
GLOAM_FORCE_INLINE void vkCmdSetScissorWithCount(VkCommandBuffer commandBuffer, uint32_t scissorCount, const VkRect2D* pScissors) {
    gloam_vk_context.CmdSetScissorWithCount(commandBuffer, scissorCount, pScissors);
}
GLOAM_FORCE_INLINE void vkCmdSetStencilOp(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, VkStencilOp failOp, VkStencilOp passOp, VkStencilOp depthFailOp, VkCompareOp compareOp) {
    gloam_vk_context.CmdSetStencilOp(commandBuffer, faceMask, failOp, passOp, depthFailOp, compareOp);
}
GLOAM_FORCE_INLINE void vkCmdSetStencilTestEnable(VkCommandBuffer commandBuffer, VkBool32 stencilTestEnable) {
    gloam_vk_context.CmdSetStencilTestEnable(commandBuffer, stencilTestEnable);
}
GLOAM_FORCE_INLINE void vkCmdSetViewportWithCount(VkCommandBuffer commandBuffer, uint32_t viewportCount, const VkViewport* pViewports) {
    gloam_vk_context.CmdSetViewportWithCount(commandBuffer, viewportCount, pViewports);
}
GLOAM_FORCE_INLINE void vkCmdWaitEvents2(VkCommandBuffer commandBuffer, uint32_t eventCount, const VkEvent* pEvents, const VkDependencyInfo* pDependencyInfos) {
    gloam_vk_context.CmdWaitEvents2(commandBuffer, eventCount, pEvents, pDependencyInfos);
}
GLOAM_FORCE_INLINE void vkCmdWriteTimestamp2(VkCommandBuffer commandBuffer, VkPipelineStageFlags2 stage, VkQueryPool queryPool, uint32_t query) {
    gloam_vk_context.CmdWriteTimestamp2(commandBuffer, stage, queryPool, query);
}
GLOAM_FORCE_INLINE VkResult vkCreatePrivateDataSlot(VkDevice device, const VkPrivateDataSlotCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkPrivateDataSlot* pPrivateDataSlot) {
    return gloam_vk_context.CreatePrivateDataSlot(device, pCreateInfo, pAllocator, pPrivateDataSlot);
}
GLOAM_FORCE_INLINE void vkDestroyPrivateDataSlot(VkDevice device, VkPrivateDataSlot privateDataSlot, const VkAllocationCallbacks* pAllocator) {
    gloam_vk_context.DestroyPrivateDataSlot(device, privateDataSlot, pAllocator);
}
GLOAM_FORCE_INLINE void vkGetDeviceBufferMemoryRequirements(VkDevice device, const VkDeviceBufferMemoryRequirements* pInfo, VkMemoryRequirements2* pMemoryRequirements) {
    gloam_vk_context.GetDeviceBufferMemoryRequirements(device, pInfo, pMemoryRequirements);
}
GLOAM_FORCE_INLINE void vkGetDeviceImageMemoryRequirements(VkDevice device, const VkDeviceImageMemoryRequirements* pInfo, VkMemoryRequirements2* pMemoryRequirements) {
    gloam_vk_context.GetDeviceImageMemoryRequirements(device, pInfo, pMemoryRequirements);
}
GLOAM_FORCE_INLINE void vkGetDeviceImageSparseMemoryRequirements(VkDevice device, const VkDeviceImageMemoryRequirements* pInfo, uint32_t* pSparseMemoryRequirementCount, VkSparseImageMemoryRequirements2* pSparseMemoryRequirements) {
    gloam_vk_context.GetDeviceImageSparseMemoryRequirements(device, pInfo, pSparseMemoryRequirementCount, pSparseMemoryRequirements);
}
GLOAM_FORCE_INLINE VkResult vkGetPhysicalDeviceToolProperties(VkPhysicalDevice physicalDevice, uint32_t* pToolCount, VkPhysicalDeviceToolProperties* pToolProperties) {
    return gloam_vk_context.GetPhysicalDeviceToolProperties(physicalDevice, pToolCount, pToolProperties);
}
GLOAM_FORCE_INLINE void vkGetPrivateData(VkDevice device, VkObjectType objectType, uint64_t objectHandle, VkPrivateDataSlot privateDataSlot, uint64_t* pData) {
    gloam_vk_context.GetPrivateData(device, objectType, objectHandle, privateDataSlot, pData);
}
GLOAM_FORCE_INLINE VkResult vkQueueSubmit2(VkQueue queue, uint32_t submitCount, const VkSubmitInfo2* pSubmits, VkFence fence) {
    return gloam_vk_context.QueueSubmit2(queue, submitCount, pSubmits, fence);
}
GLOAM_FORCE_INLINE VkResult vkSetPrivateData(VkDevice device, VkObjectType objectType, uint64_t objectHandle, VkPrivateDataSlot privateDataSlot, uint64_t data) {
    return gloam_vk_context.SetPrivateData(device, objectType, objectHandle, privateDataSlot, data);
}
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

/* ---- Vulkan enabled API (Volk-like) ----------------------------------------
 * Phased loading: Initialize → LoadInstance → LoadDevice.
 * The caller owns extension discovery and tells gloam what was enabled.
 *
 * Phase 0 — Initialize: open libvulkan and load the handful of Global-scope
 * PFNs needed to create an instance (vkCreateInstance, vkEnumerateInstance*).
 * If library_handle is non-NULL, use it without taking ownership; if NULL,
 * dlopen the platform default and take ownership.
 *
 * Phase 1 — LoadInstance: load Global + Instance-scope PFNs for core features
 * and enabled instance extensions. Set featArray from api_version
 * (VK_MAKE_API_VERSION or VK_API_VERSION_x_y). Set extArray for enabled
 * instance extensions. Resolve aliases.
 *
 * Phase 1.5 (optional) — LoadPhysicalDeviceExtension(s): pre-load
 * Instance-scope PFNs for device extensions the application wants to query
 * before creating a VkDevice (e.g. vkGetPhysicalDeviceFragmentShadingRatesKHR
 * from VK_KHR_fragment_shading_rate). Does NOT set extArray.
 *
 * Phase 2 — LoadDevice: load PFNs for enabled device extensions (all scopes).
 * Device-scope commands use vkGetDeviceProcAddr for the fast path; Instance-
 * scope commands in device extensions use vkGetInstanceProcAddr. Update
 * featArray from the device's api_version. Set extArray for enabled device
 * extensions. Resolve aliases.
 *
 * Finalize: close library handle if gloam owns it, zero the context.
 */
void gloamVulkanInitializeCustomContext(GloamVulkanContext *context, PFN_vkGetInstanceProcAddr getInstanceProcAddr);
void gloamVulkanInitializeCustom(PFN_vkGetInstanceProcAddr getInstanceProcAddr);
uint32_t gloamVulkanGetInstanceVersionContext(GloamVulkanContext *context);
uint32_t gloamVulkanGetInstanceVersion(void);
int  gloamVulkanLoadInstanceContext(GloamVulkanContext *context, VkInstance instance, uint32_t api_version, uint32_t num_instance_extensions, const char *const *instance_extensions);
int  gloamVulkanLoadInstance(VkInstance instance, uint32_t api_version, uint32_t num_instance_extensions, const char *const *instance_extensions);
VkInstance gloamVulkanGetLoadedInstanceContext(GloamVulkanContext *context);
VkInstance gloamVulkanGetLoadedInstance(void);
void gloamVulkanLoadPhysicalDeviceExtensionContext(GloamVulkanContext *context, const char *device_extension);
void gloamVulkanLoadPhysicalDeviceExtension(const char *device_extension);
void gloamVulkanLoadPhysicalDeviceExtensionsContext(GloamVulkanContext *context, uint32_t num_device_extensions, const char *const *device_extensions);
void gloamVulkanLoadPhysicalDeviceExtensions(uint32_t num_device_extensions, const char *const *device_extensions);
int  gloamVulkanLoadDeviceContext(GloamVulkanContext *context, VkDevice device, VkPhysicalDevice physical_device, uint32_t num_device_extensions, const char *const *device_extensions);
int  gloamVulkanLoadDevice(VkDevice device, VkPhysicalDevice physical_device, uint32_t num_device_extensions, const char *const *device_extensions);
VkDevice gloamVulkanGetLoadedDeviceContext(GloamVulkanContext *context);
VkDevice gloamVulkanGetLoadedDevice(void);
void gloamVulkanFinalizeContext(GloamVulkanContext *context);
void gloamVulkanFinalize(void);


#ifdef __cplusplus
}
#endif

#endif /* GLOAM_VK_H */
