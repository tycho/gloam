/* vk-cube — a spinning cube through a gloam-generated Vulkan loader.
 *
 * The Vulkan analogue of gl-triangle: real rendering with the pieces a real
 * application needs — a swapchain (with recreation on resize), a depth
 * buffer, explicit image-layout barriers around Vulkan 1.3 dynamic
 * rendering, per-frame synchronization (acquire/render semaphores plus
 * in-flight fences), and push-constant transforms.
 *
 * Loading is the phased flow (gloamVulkanInitialize → vkCreateInstance →
 * gloamVulkanLoadInstance → vkCreateDevice → gloamVulkanLoadDevice), with
 * the built-in --loader opening the platform Vulkan library. SDL3 provides
 * the window and the surface (SDL_Vulkan_CreateSurface), so this loader
 * needs no platform surface extensions — the counterpart Rust example
 * (examples/rust/vk-cube) goes the other way and creates the surface
 * through the loader's own vkCreate*SurfaceKHR commands.
 *
 * The instance opts into portability enumeration when the loader offers it,
 * and the device enables VK_KHR_portability_subset when advertised
 * (spec-required), so MoltenVK works out of the box.
 *
 * Run with --ci to render one frame in a hidden window, copy it back
 * through a staging buffer, verify the center pixel, and exit.
 * Exit codes: 0 = pass, 1 = failure, 77 = skipped (no usable Vulkan).
 */

#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <gloam/vk.h>

#include <SDL3/SDL.h>
#include <SDL3/SDL_main.h>
#include <SDL3/SDL_vulkan.h>

#include "cube_shaders.h"

#define EXIT_SKIP 77
#define FRAMES_IN_FLIGHT 2
#define DEPTH_FORMAT VK_FORMAT_D32_SFLOAT

/* ------------------------------------------------------------------------- */
/* Column-major mat4 helpers (Vulkan clip space: Y down, depth 0..1).        */
/* ------------------------------------------------------------------------- */

typedef struct { float m[16]; } Mat4;

static Mat4 mat_mul(const Mat4 *a, const Mat4 *b)
{
    Mat4 r;
    int c, i, k;
    for (c = 0; c < 4; ++c)
        for (i = 0; i < 4; ++i) {
            float sum = 0.0f;
            for (k = 0; k < 4; ++k)
                sum += a->m[k * 4 + i] * b->m[c * 4 + k];
            r.m[c * 4 + i] = sum;
        }
    return r;
}

static Mat4 mat_identity(void)
{
    Mat4 r = { { 0 } };
    r.m[0] = r.m[5] = r.m[10] = r.m[15] = 1.0f;
    return r;
}

static Mat4 mat_perspective(float fov_y, float aspect, float near_z, float far_z)
{
    /* Vulkan conventions baked in: depth mapped to [0, 1] and Y flipped so
     * +Y is up in application space. */
    float f = 1.0f / tanf(fov_y * 0.5f);
    Mat4 r = { { 0 } };
    r.m[0] = f / aspect;
    r.m[5] = -f;
    r.m[10] = far_z / (near_z - far_z);
    r.m[11] = -1.0f;
    r.m[14] = (near_z * far_z) / (near_z - far_z);
    return r;
}

static Mat4 mat_rotate_y(float a)
{
    Mat4 r = mat_identity();
    float s = sinf(a), c = cosf(a);
    r.m[0] = c; r.m[2] = -s; r.m[8] = s; r.m[10] = c;
    return r;
}

static Mat4 mat_rotate_x(float a)
{
    Mat4 r = mat_identity();
    float s = sinf(a), c = cosf(a);
    r.m[5] = c; r.m[6] = s; r.m[9] = -s; r.m[10] = c;
    return r;
}

static Mat4 mat_translate_z(float z)
{
    Mat4 r = mat_identity();
    r.m[14] = z;
    return r;
}

/* ------------------------------------------------------------------------- */
/* Cube geometry: 36 vertices, position + face color.                        */
/* ------------------------------------------------------------------------- */

#define VR 0.90f, 0.24f, 0.24f
#define VG 0.24f, 0.80f, 0.33f
#define VB 0.25f, 0.42f, 0.94f
#define VY 0.95f, 0.78f, 0.20f
#define VC 0.20f, 0.80f, 0.83f
#define VM 0.83f, 0.30f, 0.80f

static const float CUBE[36 * 6] = {
    /* +Z front (red) */
    -1,-1, 1, VR,   1,-1, 1, VR,   1, 1, 1, VR,
    -1,-1, 1, VR,   1, 1, 1, VR,  -1, 1, 1, VR,
    /* -Z back (green) */
     1,-1,-1, VG,  -1,-1,-1, VG,  -1, 1,-1, VG,
     1,-1,-1, VG,  -1, 1,-1, VG,   1, 1,-1, VG,
    /* -X left (blue) */
    -1,-1,-1, VB,  -1,-1, 1, VB,  -1, 1, 1, VB,
    -1,-1,-1, VB,  -1, 1, 1, VB,  -1, 1,-1, VB,
    /* +X right (yellow) */
     1,-1, 1, VY,   1,-1,-1, VY,   1, 1,-1, VY,
     1,-1, 1, VY,   1, 1,-1, VY,   1, 1, 1, VY,
    /* +Y top (cyan) */
    -1, 1, 1, VC,   1, 1, 1, VC,   1, 1,-1, VC,
    -1, 1, 1, VC,   1, 1,-1, VC,  -1, 1,-1, VC,
    /* -Y bottom (magenta) */
    -1,-1,-1, VM,   1,-1,-1, VM,   1,-1, 1, VM,
    -1,-1,-1, VM,   1,-1, 1, VM,  -1,-1, 1, VM,
};

/* ------------------------------------------------------------------------- */
/* Application state.                                                        */
/* ------------------------------------------------------------------------- */

typedef struct {
    VkCommandBuffer cmd;
    VkSemaphore acquire;
    VkFence fence;
} Frame;

static SDL_Window *g_window;
static VkInstance g_instance;
static VkSurfaceKHR g_surface;
static VkPhysicalDevice g_pd;
static VkDevice g_device;
static VkQueue g_queue;
static VkSurfaceFormatKHR g_surface_format;
static VkPhysicalDeviceMemoryProperties g_memory_props;
static VkPipelineLayout g_pipeline_layout;
static VkPipeline g_pipeline;
static VkBuffer g_vertex_buffer;
static VkDeviceMemory g_vertex_memory;
static VkCommandPool g_command_pool;
static Frame g_frames[FRAMES_IN_FLIGHT];
static int g_frame_index;
static int g_ci;

/* Everything that dies with the swapchain on resize. */
static VkSwapchainKHR g_swapchain;
static VkExtent2D g_extent;
static uint32_t g_num_images;
static VkImage *g_images;
static VkImageView *g_views;
/* One render-finished semaphore per swapchain image: present waits on the
 * semaphore signaled by the submit that rendered that image. */
static VkSemaphore *g_render_done;
static VkImage g_depth_image;
static VkDeviceMemory g_depth_memory;
static VkImageView g_depth_view;

static uint32_t find_memory_type(uint32_t type_bits, VkMemoryPropertyFlags required)
{
    uint32_t i;
    for (i = 0; i < g_memory_props.memoryTypeCount; ++i)
        if ((type_bits & (1u << i)) &&
            (g_memory_props.memoryTypes[i].propertyFlags & required) == required)
            return i;
    fprintf(stderr, "vk-cube: no memory type matches 0x%x with flags 0x%x\n",
            type_bits, required);
    exit(1);
}

static void destroy_swapchain_objects(void)
{
    uint32_t i;
    if (!g_swapchain)
        return;
    for (i = 0; i < g_num_images; ++i) {
        vkDestroyImageView(g_device, g_views[i], NULL);
        vkDestroySemaphore(g_device, g_render_done[i], NULL);
    }
    free(g_images);
    free(g_views);
    free(g_render_done);
    vkDestroyImageView(g_device, g_depth_view, NULL);
    vkDestroyImage(g_device, g_depth_image, NULL);
    vkFreeMemory(g_device, g_depth_memory, NULL);
    vkDestroySwapchainKHR(g_device, g_swapchain, NULL);
    g_swapchain = VK_NULL_HANDLE;
}

static void create_swapchain(void)
{
    VkSwapchainKHR old = g_swapchain;
    VkSurfaceCapabilitiesKHR caps;
    VkImageUsageFlags usage;
    uint32_t min_images, i;
    VkResult res;

    vkGetPhysicalDeviceSurfaceCapabilitiesKHR(g_pd, g_surface, &caps);
    if (caps.currentExtent.width != UINT32_MAX) {
        g_extent = caps.currentExtent;
    } else {
        int w = 0, h = 0;
        SDL_GetWindowSizeInPixels(g_window, &w, &h);
        g_extent.width = (uint32_t)w;
        g_extent.height = (uint32_t)h;
        if (g_extent.width < caps.minImageExtent.width)
            g_extent.width = caps.minImageExtent.width;
        if (g_extent.width > caps.maxImageExtent.width)
            g_extent.width = caps.maxImageExtent.width;
        if (g_extent.height < caps.minImageExtent.height)
            g_extent.height = caps.minImageExtent.height;
        if (g_extent.height > caps.maxImageExtent.height)
            g_extent.height = caps.maxImageExtent.height;
    }
    min_images = caps.minImageCount + 1;
    if (caps.maxImageCount != 0 && min_images > caps.maxImageCount)
        min_images = caps.maxImageCount;

    /* TRANSFER_SRC lets --ci copy the rendered image back out. */
    usage = VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT;
    if (caps.supportedUsageFlags & VK_IMAGE_USAGE_TRANSFER_SRC_BIT) {
        usage |= VK_IMAGE_USAGE_TRANSFER_SRC_BIT;
    } else if (g_ci) {
        fprintf(stderr, "vk-cube: --ci needs a TRANSFER_SRC-capable swapchain\n");
        exit(1);
    }

    {
        VkSwapchainCreateInfoKHR sci = { .sType = VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR };
        sci.surface = g_surface;
        sci.minImageCount = min_images;
        sci.imageFormat = g_surface_format.format;
        sci.imageColorSpace = g_surface_format.colorSpace;
        sci.imageExtent = g_extent;
        sci.imageArrayLayers = 1;
        sci.imageUsage = usage;
        sci.imageSharingMode = VK_SHARING_MODE_EXCLUSIVE;
        sci.preTransform = caps.currentTransform;
        sci.compositeAlpha = VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR;
        sci.presentMode = VK_PRESENT_MODE_FIFO_KHR; /* always available */
        sci.clipped = VK_TRUE;
        sci.oldSwapchain = old;
        res = vkCreateSwapchainKHR(g_device, &sci, NULL, &g_swapchain);
    }
    if (res != VK_SUCCESS) {
        fprintf(stderr, "vk-cube: vkCreateSwapchainKHR failed (%d)\n", (int)res);
        exit(1);
    }

    /* The old swapchain (and its per-image objects) can be destroyed once
     * the device is idle; recreation only happens on resize, so a full
     * wait is the simple and correct choice. */
    if (old) {
        VkSwapchainKHR created = g_swapchain;
        vkDeviceWaitIdle(g_device);
        g_swapchain = old;
        destroy_swapchain_objects();
        g_swapchain = created;
    }

    vkGetSwapchainImagesKHR(g_device, g_swapchain, &g_num_images, NULL);
    g_images = (VkImage *)calloc(g_num_images, sizeof(*g_images));
    g_views = (VkImageView *)calloc(g_num_images, sizeof(*g_views));
    g_render_done = (VkSemaphore *)calloc(g_num_images, sizeof(*g_render_done));
    vkGetSwapchainImagesKHR(g_device, g_swapchain, &g_num_images, g_images);

    for (i = 0; i < g_num_images; ++i) {
        VkImageViewCreateInfo ivci = { .sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO };
        VkSemaphoreCreateInfo sem_ci = { .sType = VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO };
        ivci.image = g_images[i];
        ivci.viewType = VK_IMAGE_VIEW_TYPE_2D;
        ivci.format = g_surface_format.format;
        ivci.subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
        ivci.subresourceRange.levelCount = 1;
        ivci.subresourceRange.layerCount = 1;
        if (vkCreateImageView(g_device, &ivci, NULL, &g_views[i]) != VK_SUCCESS) {
            fprintf(stderr, "vk-cube: vkCreateImageView failed\n");
            exit(1);
        }
        vkCreateSemaphore(g_device, &sem_ci, NULL, &g_render_done[i]);
    }

    /* Depth buffer, recreated with the swapchain (it tracks the extent). */
    {
        VkImageCreateInfo ici = { .sType = VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO };
        VkMemoryRequirements reqs;
        VkMemoryAllocateInfo mai = { .sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO };
        VkImageViewCreateInfo dvci = { .sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO };

        ici.imageType = VK_IMAGE_TYPE_2D;
        ici.format = DEPTH_FORMAT;
        ici.extent.width = g_extent.width;
        ici.extent.height = g_extent.height;
        ici.extent.depth = 1;
        ici.mipLevels = 1;
        ici.arrayLayers = 1;
        ici.samples = VK_SAMPLE_COUNT_1_BIT;
        ici.tiling = VK_IMAGE_TILING_OPTIMAL;
        ici.usage = VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT;
        ici.initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
        if (vkCreateImage(g_device, &ici, NULL, &g_depth_image) != VK_SUCCESS) {
            fprintf(stderr, "vk-cube: depth vkCreateImage failed\n");
            exit(1);
        }

        vkGetImageMemoryRequirements(g_device, g_depth_image, &reqs);
        mai.allocationSize = reqs.size;
        mai.memoryTypeIndex = find_memory_type(reqs.memoryTypeBits,
                                               VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT);
        vkAllocateMemory(g_device, &mai, NULL, &g_depth_memory);
        vkBindImageMemory(g_device, g_depth_image, g_depth_memory, 0);

        dvci.image = g_depth_image;
        dvci.viewType = VK_IMAGE_VIEW_TYPE_2D;
        dvci.format = DEPTH_FORMAT;
        dvci.subresourceRange.aspectMask = VK_IMAGE_ASPECT_DEPTH_BIT;
        dvci.subresourceRange.levelCount = 1;
        dvci.subresourceRange.layerCount = 1;
        vkCreateImageView(g_device, &dvci, NULL, &g_depth_view);
    }
}

/* --ci: copy the just-rendered swapchain image into a host-visible buffer
 * (one more barrier: PRESENT-bound image → TRANSFER_SRC) and check that the
 * center pixel is cube, not background. */
static int verify_center_pixel(VkImage image)
{
    const Frame *frame = &g_frames[g_frame_index];
    VkDeviceSize size = (VkDeviceSize)g_extent.width * g_extent.height * 4;
    VkBuffer buffer;
    VkDeviceMemory memory;
    VkMemoryRequirements reqs;
    void *mapped = NULL;
    const unsigned char *px;
    size_t center;
    unsigned b0, b1, b2;

    vkWaitForFences(g_device, 1, &frame->fence, VK_TRUE, UINT64_MAX);

    {
        VkBufferCreateInfo bci = { .sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO };
        VkMemoryAllocateInfo mai = { .sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO };
        bci.size = size;
        bci.usage = VK_BUFFER_USAGE_TRANSFER_DST_BIT;
        bci.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
        vkCreateBuffer(g_device, &bci, NULL, &buffer);
        vkGetBufferMemoryRequirements(g_device, buffer, &reqs);
        mai.allocationSize = reqs.size;
        mai.memoryTypeIndex = find_memory_type(reqs.memoryTypeBits,
                                               VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT |
                                               VK_MEMORY_PROPERTY_HOST_COHERENT_BIT);
        vkAllocateMemory(g_device, &mai, NULL, &memory);
        vkBindBufferMemory(g_device, buffer, memory, 0);
    }

    {
        VkCommandBufferBeginInfo begin = { .sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO };
        VkImageMemoryBarrier to_src = { .sType = VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER };
        VkBufferImageCopy region = { 0 };
        VkSubmitInfo submit = { .sType = VK_STRUCTURE_TYPE_SUBMIT_INFO };

        begin.flags = VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT;
        vkBeginCommandBuffer(frame->cmd, &begin);

        to_src.srcAccessMask = VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT;
        to_src.dstAccessMask = VK_ACCESS_TRANSFER_READ_BIT;
        to_src.oldLayout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
        to_src.newLayout = VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL;
        to_src.srcQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
        to_src.dstQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
        to_src.image = image;
        to_src.subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
        to_src.subresourceRange.levelCount = 1;
        to_src.subresourceRange.layerCount = 1;
        vkCmdPipelineBarrier(frame->cmd,
                             VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
                             VK_PIPELINE_STAGE_TRANSFER_BIT,
                             0, 0, NULL, 0, NULL, 1, &to_src);

        region.imageSubresource.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
        region.imageSubresource.layerCount = 1;
        region.imageExtent.width = g_extent.width;
        region.imageExtent.height = g_extent.height;
        region.imageExtent.depth = 1;
        vkCmdCopyImageToBuffer(frame->cmd, image, VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
                               buffer, 1, &region);
        vkEndCommandBuffer(frame->cmd);

        submit.commandBufferCount = 1;
        submit.pCommandBuffers = &frame->cmd;
        vkResetFences(g_device, 1, &frame->fence);
        vkQueueSubmit(g_queue, 1, &submit, frame->fence);
        vkWaitForFences(g_device, 1, &frame->fence, VK_TRUE, UINT64_MAX);
    }

    vkMapMemory(g_device, memory, 0, size, 0, &mapped);
    px = (const unsigned char *)mapped;
    center = ((size_t)(g_extent.height / 2) * g_extent.width + g_extent.width / 2) * 4;
    /* The surface format is BGRA or RGBA; a brightness check keeps the
     * assertion order-independent. */
    b0 = px[center]; b1 = px[center + 1]; b2 = px[center + 2];
    printf("center pixel: %u %u %u\n", b0, b1, b2);
    vkUnmapMemory(g_device, memory);
    vkDestroyBuffer(g_device, buffer, NULL);
    vkFreeMemory(g_device, memory, NULL);

    if (b0 + b1 + b2 <= 60) {
        fprintf(stderr, "vk-cube: FAIL — center pixel is background\n");
        return 0;
    }
    printf("\nvk-cube (C, gloam Vulkan loader): PASS\n");
    return 1;
}

/* Record and submit one frame; returns 0 when the swapchain needs
 * recreation (out-of-date / suboptimal). */
static int draw(float angle)
{
    const Frame *frame = &g_frames[g_frame_index];
    uint32_t image_index = 0;
    VkImage image;
    VkSemaphore render_done;
    VkCommandBuffer cmd = frame->cmd;
    VkResult res;

    vkWaitForFences(g_device, 1, &frame->fence, VK_TRUE, UINT64_MAX);

    res = vkAcquireNextImageKHR(g_device, g_swapchain, UINT64_MAX,
                                frame->acquire, VK_NULL_HANDLE, &image_index);
    if (res == VK_ERROR_OUT_OF_DATE_KHR)
        return 0;
    if (res != VK_SUCCESS && res != VK_SUBOPTIMAL_KHR) {
        fprintf(stderr, "vk-cube: vkAcquireNextImageKHR failed (%d)\n", (int)res);
        exit(1);
    }
    vkResetFences(g_device, 1, &frame->fence);

    image = g_images[image_index];
    render_done = g_render_done[image_index];

    {
        VkCommandBufferBeginInfo begin = { .sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO };
        begin.flags = VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT;
        vkBeginCommandBuffer(cmd, &begin);
    }

    /* Layout transitions into rendering: the color target goes UNDEFINED →
     * COLOR_ATTACHMENT_OPTIMAL (contents are cleared, so the previous
     * frame's contents can be discarded), and the depth buffer UNDEFINED →
     * DEPTH_ATTACHMENT_OPTIMAL likewise.  Dynamic rendering has no render
     * pass to do implicit transitions — these barriers are the
     * application's job. */
    {
        VkImageMemoryBarrier barriers[2];
        memset(barriers, 0, sizeof(barriers));
        barriers[0].sType = VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER;
        barriers[0].srcAccessMask = 0;
        barriers[0].dstAccessMask = VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT;
        barriers[0].oldLayout = VK_IMAGE_LAYOUT_UNDEFINED;
        barriers[0].newLayout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL;
        barriers[0].srcQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
        barriers[0].dstQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
        barriers[0].image = image;
        barriers[0].subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
        barriers[0].subresourceRange.levelCount = 1;
        barriers[0].subresourceRange.layerCount = 1;

        barriers[1] = barriers[0];
        barriers[1].dstAccessMask = VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT;
        barriers[1].newLayout = VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL;
        barriers[1].image = g_depth_image;
        barriers[1].subresourceRange.aspectMask = VK_IMAGE_ASPECT_DEPTH_BIT;

        vkCmdPipelineBarrier(cmd,
                             VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT,
                             VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT |
                             VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT,
                             0, 0, NULL, 0, NULL, 2, barriers);
    }

    /* Dynamic rendering: attachments described inline, no render pass. */
    {
        VkRenderingAttachmentInfo color_att = { .sType = VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO };
        VkRenderingAttachmentInfo depth_att = { .sType = VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO };
        VkRenderingInfo rendering = { .sType = VK_STRUCTURE_TYPE_RENDERING_INFO };
        VkViewport viewport = { 0 };
        VkRect2D scissor = { 0 };
        VkDeviceSize offset = 0;
        Mat4 mvp;

        color_att.imageView = g_views[image_index];
        color_att.imageLayout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL;
        color_att.loadOp = VK_ATTACHMENT_LOAD_OP_CLEAR;
        color_att.storeOp = VK_ATTACHMENT_STORE_OP_STORE;
        color_att.clearValue.color.float32[0] = 0.05f;
        color_att.clearValue.color.float32[1] = 0.05f;
        color_att.clearValue.color.float32[2] = 0.08f;
        color_att.clearValue.color.float32[3] = 1.0f;

        depth_att.imageView = g_depth_view;
        depth_att.imageLayout = VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL;
        depth_att.loadOp = VK_ATTACHMENT_LOAD_OP_CLEAR;
        depth_att.storeOp = VK_ATTACHMENT_STORE_OP_DONT_CARE;
        depth_att.clearValue.depthStencil.depth = 1.0f;

        rendering.renderArea.extent = g_extent;
        rendering.layerCount = 1;
        rendering.colorAttachmentCount = 1;
        rendering.pColorAttachments = &color_att;
        rendering.pDepthAttachment = &depth_att;

        {
            float aspect = (float)g_extent.width /
                           (float)(g_extent.height ? g_extent.height : 1);
            Mat4 proj = mat_perspective(3.14159265f / 3.0f, aspect, 0.1f, 10.0f);
            Mat4 view = mat_translate_z(-4.5f);
            Mat4 ry = mat_rotate_y(angle), rx = mat_rotate_x(angle * 0.7f);
            Mat4 model = mat_mul(&ry, &rx);
            Mat4 vm = mat_mul(&view, &model);
            mvp = mat_mul(&proj, &vm);
        }

        vkCmdBeginRendering(cmd, &rendering);
        viewport.width = (float)g_extent.width;
        viewport.height = (float)g_extent.height;
        viewport.maxDepth = 1.0f;
        scissor.extent = g_extent;
        vkCmdSetViewport(cmd, 0, 1, &viewport);
        vkCmdSetScissor(cmd, 0, 1, &scissor);
        vkCmdBindPipeline(cmd, VK_PIPELINE_BIND_POINT_GRAPHICS, g_pipeline);
        vkCmdPushConstants(cmd, g_pipeline_layout, VK_SHADER_STAGE_VERTEX_BIT,
                           0, sizeof(mvp), &mvp);
        vkCmdBindVertexBuffers(cmd, 0, 1, &g_vertex_buffer, &offset);
        vkCmdDraw(cmd, 36, 1, 0, 0);
        vkCmdEndRendering(cmd);
    }

    /* Out of rendering: COLOR_ATTACHMENT → PRESENT_SRC before the
     * presentation engine may read the image. */
    {
        VkImageMemoryBarrier to_present = { .sType = VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER };
        to_present.srcAccessMask = VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT;
        to_present.dstAccessMask = 0;
        to_present.oldLayout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL;
        to_present.newLayout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
        to_present.srcQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
        to_present.dstQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
        to_present.image = image;
        to_present.subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
        to_present.subresourceRange.levelCount = 1;
        to_present.subresourceRange.layerCount = 1;
        vkCmdPipelineBarrier(cmd,
                             VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
                             VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT,
                             0, 0, NULL, 0, NULL, 1, &to_present);
        vkEndCommandBuffer(cmd);
    }

    {
        VkPipelineStageFlags wait_stage = VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT;
        VkSubmitInfo submit = { .sType = VK_STRUCTURE_TYPE_SUBMIT_INFO };
        submit.waitSemaphoreCount = 1;
        submit.pWaitSemaphores = &frame->acquire;
        submit.pWaitDstStageMask = &wait_stage;
        submit.commandBufferCount = 1;
        submit.pCommandBuffers = &cmd;
        submit.signalSemaphoreCount = 1;
        submit.pSignalSemaphores = &render_done;
        res = vkQueueSubmit(g_queue, 1, &submit, frame->fence);
        if (res != VK_SUCCESS) {
            fprintf(stderr, "vk-cube: vkQueueSubmit failed (%d)\n", (int)res);
            exit(1);
        }
    }

    if (g_ci) {
        /* Verify before presenting: wait for the render, copy the center
         * pixel out through a staging buffer, and check it. */
        if (!verify_center_pixel(image))
            exit(1);
        return 1;
    }

    {
        VkPresentInfoKHR present = { .sType = VK_STRUCTURE_TYPE_PRESENT_INFO_KHR };
        present.waitSemaphoreCount = 1;
        present.pWaitSemaphores = &render_done;
        present.swapchainCount = 1;
        present.pSwapchains = &g_swapchain;
        present.pImageIndices = &image_index;
        res = vkQueuePresentKHR(g_queue, &present);
    }
    g_frame_index = (g_frame_index + 1) % FRAMES_IN_FLIGHT;
    if (res == VK_ERROR_OUT_OF_DATE_KHR || res == VK_SUBOPTIMAL_KHR)
        return 0;
    if (res != VK_SUCCESS) {
        fprintf(stderr, "vk-cube: vkQueuePresentKHR failed (%d)\n", (int)res);
        exit(1);
    }
    return 1;
}

/* Build the pipeline layout (one vertex-stage push-constant range holding
 * the MVP) and the graphics pipeline, targeting dynamic rendering (no
 * render pass — the attachment formats ride in the pNext chain). */
static void create_pipeline(void)
{
    VkShaderModule vert, frag;
    VkResult res;

    {
        VkShaderModuleCreateInfo ci = { .sType = VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO };
        ci.codeSize = sizeof(cube_vert_spv);
        ci.pCode = cube_vert_spv;
        res = vkCreateShaderModule(g_device, &ci, NULL, &vert);
        if (res == VK_SUCCESS) {
            ci.codeSize = sizeof(cube_frag_spv);
            ci.pCode = cube_frag_spv;
            res = vkCreateShaderModule(g_device, &ci, NULL, &frag);
        }
        if (res != VK_SUCCESS) {
            fprintf(stderr, "vk-cube: vkCreateShaderModule failed (%d)\n", (int)res);
            exit(1);
        }
    }

    {
        VkPushConstantRange pc_range = { 0 };
        VkPipelineLayoutCreateInfo plci = { .sType = VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO };
        pc_range.stageFlags = VK_SHADER_STAGE_VERTEX_BIT;
        pc_range.size = sizeof(Mat4);
        plci.pushConstantRangeCount = 1;
        plci.pPushConstantRanges = &pc_range;
        vkCreatePipelineLayout(g_device, &plci, NULL, &g_pipeline_layout);
    }

    {
        VkPipelineShaderStageCreateInfo stages[2];
        VkVertexInputBindingDescription binding = { 0 };
        VkVertexInputAttributeDescription attrs[2];
        VkPipelineVertexInputStateCreateInfo vi = { .sType = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO };
        VkPipelineInputAssemblyStateCreateInfo ia = { .sType = VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO };
        VkPipelineViewportStateCreateInfo vp = { .sType = VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO };
        VkPipelineRasterizationStateCreateInfo rs = { .sType = VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO };
        VkPipelineMultisampleStateCreateInfo ms = { .sType = VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO };
        VkPipelineDepthStencilStateCreateInfo ds = { .sType = VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO };
        VkPipelineColorBlendAttachmentState blend_att = { 0 };
        VkPipelineColorBlendStateCreateInfo blend = { .sType = VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO };
        VkDynamicState dyn_states[2] = { VK_DYNAMIC_STATE_VIEWPORT, VK_DYNAMIC_STATE_SCISSOR };
        VkPipelineDynamicStateCreateInfo dynamic = { .sType = VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO };
        VkPipelineRenderingCreateInfo rendering = { .sType = VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO };
        VkGraphicsPipelineCreateInfo gpci = { .sType = VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO };

        memset(stages, 0, sizeof(stages));
        stages[0].sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
        stages[0].stage = VK_SHADER_STAGE_VERTEX_BIT;
        stages[0].module = vert;
        stages[0].pName = "main";
        stages[1].sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
        stages[1].stage = VK_SHADER_STAGE_FRAGMENT_BIT;
        stages[1].module = frag;
        stages[1].pName = "main";

        binding.stride = 6 * sizeof(float);
        binding.inputRate = VK_VERTEX_INPUT_RATE_VERTEX;
        memset(attrs, 0, sizeof(attrs));
        attrs[0].location = 0;
        attrs[0].format = VK_FORMAT_R32G32B32_SFLOAT;
        attrs[1].location = 1;
        attrs[1].format = VK_FORMAT_R32G32B32_SFLOAT;
        attrs[1].offset = 3 * sizeof(float);
        vi.vertexBindingDescriptionCount = 1;
        vi.pVertexBindingDescriptions = &binding;
        vi.vertexAttributeDescriptionCount = 2;
        vi.pVertexAttributeDescriptions = attrs;

        ia.topology = VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST;

        vp.viewportCount = 1;
        vp.scissorCount = 1;

        rs.polygonMode = VK_POLYGON_MODE_FILL;
        rs.cullMode = VK_CULL_MODE_BACK_BIT;
        rs.frontFace = VK_FRONT_FACE_COUNTER_CLOCKWISE;
        rs.lineWidth = 1.0f;

        ms.rasterizationSamples = VK_SAMPLE_COUNT_1_BIT;

        ds.depthTestEnable = VK_TRUE;
        ds.depthWriteEnable = VK_TRUE;
        ds.depthCompareOp = VK_COMPARE_OP_LESS;

        blend_att.colorWriteMask = VK_COLOR_COMPONENT_R_BIT | VK_COLOR_COMPONENT_G_BIT |
                                   VK_COLOR_COMPONENT_B_BIT | VK_COLOR_COMPONENT_A_BIT;
        blend.attachmentCount = 1;
        blend.pAttachments = &blend_att;

        dynamic.dynamicStateCount = 2;
        dynamic.pDynamicStates = dyn_states;

        rendering.colorAttachmentCount = 1;
        rendering.pColorAttachmentFormats = &g_surface_format.format;
        rendering.depthAttachmentFormat = DEPTH_FORMAT;

        gpci.pNext = &rendering;
        gpci.stageCount = 2;
        gpci.pStages = stages;
        gpci.pVertexInputState = &vi;
        gpci.pInputAssemblyState = &ia;
        gpci.pViewportState = &vp;
        gpci.pRasterizationState = &rs;
        gpci.pMultisampleState = &ms;
        gpci.pDepthStencilState = &ds;
        gpci.pColorBlendState = &blend;
        gpci.pDynamicState = &dynamic;
        gpci.layout = g_pipeline_layout;
        res = vkCreateGraphicsPipelines(g_device, VK_NULL_HANDLE, 1, &gpci, NULL, &g_pipeline);
        if (res != VK_SUCCESS) {
            fprintf(stderr, "vk-cube: vkCreateGraphicsPipelines failed (%d)\n", (int)res);
            exit(1);
        }
    }

    vkDestroyShaderModule(g_device, vert, NULL);
    vkDestroyShaderModule(g_device, frag, NULL);
}

/* Full teardown, in reverse creation order. */
static void destroy_all(void)
{
    int i;
    vkDeviceWaitIdle(g_device);
    destroy_swapchain_objects();
    for (i = 0; i < FRAMES_IN_FLIGHT; ++i) {
        vkDestroySemaphore(g_device, g_frames[i].acquire, NULL);
        vkDestroyFence(g_device, g_frames[i].fence, NULL);
    }
    vkDestroyCommandPool(g_device, g_command_pool, NULL);
    vkDestroyBuffer(g_device, g_vertex_buffer, NULL);
    vkFreeMemory(g_device, g_vertex_memory, NULL);
    vkDestroyPipeline(g_device, g_pipeline, NULL);
    vkDestroyPipelineLayout(g_device, g_pipeline_layout, NULL);
    vkDestroyDevice(g_device, NULL);
    vkDestroySurfaceKHR(g_instance, g_surface, NULL);
    vkDestroyInstance(g_instance, NULL);
    gloamVulkanFinalize();
}

int main(int argc, char **argv)
{
    const char *enabled_inst[8];
    const char *enabled_dev[2];
    uint32_t num_enabled_inst = 0, num_enabled_dev = 0;
    uint32_t queue_family = 0;
    int i;

    for (i = 1; i < argc; ++i)
        if (strcmp(argv[i], "--ci") == 0)
            g_ci = 1;

    /* Phase 0: open the platform Vulkan library and load global PFNs. */
    if (!gloamVulkanInitialize(NULL)) {
        fprintf(stderr, "vk-cube: no Vulkan runtime available, skipping\n");
        return EXIT_SKIP;
    }

    if (!SDL_Init(SDL_INIT_VIDEO)) {
        fprintf(stderr, "vk-cube: SDL video init failed (%s), skipping\n", SDL_GetError());
        return EXIT_SKIP;
    }
    {
        SDL_WindowFlags flags = SDL_WINDOW_VULKAN | SDL_WINDOW_RESIZABLE;
        if (g_ci)
            flags |= SDL_WINDOW_HIDDEN;
        g_window = SDL_CreateWindow("gloam vk-cube (C)", 800, 600, flags);
    }
    if (!g_window) {
        fprintf(stderr, "vk-cube: no Vulkan-capable window (%s), skipping\n", SDL_GetError());
        SDL_Quit();
        return EXIT_SKIP;
    }

    /* Instance: SDL names the surface extensions it needs for this window
     * system; add portability enumeration when the loader offers it. */
    {
        uint32_t num_sdl_exts = 0, num_inst_props = 0;
        char const *const *sdl_exts = SDL_Vulkan_GetInstanceExtensions(&num_sdl_exts);
        VkExtensionProperties *inst_props;
        VkApplicationInfo app = { .sType = VK_STRUCTURE_TYPE_APPLICATION_INFO };
        VkInstanceCreateInfo ci = { .sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO };
        uint32_t j;
        VkResult res;

        for (j = 0; j < num_sdl_exts && num_enabled_inst < 7; ++j)
            enabled_inst[num_enabled_inst++] = sdl_exts[j];

        vkEnumerateInstanceExtensionProperties(NULL, &num_inst_props, NULL);
        inst_props = (VkExtensionProperties *)calloc(num_inst_props ? num_inst_props : 1,
                                                     sizeof(*inst_props));
        vkEnumerateInstanceExtensionProperties(NULL, &num_inst_props, inst_props);
        for (j = 0; j < num_inst_props; ++j)
            if (strcmp(inst_props[j].extensionName,
                       VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME) == 0) {
                enabled_inst[num_enabled_inst++] = VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME;
                ci.flags |= VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR;
                break;
            }
        free(inst_props);

        app.pApplicationName = "gloam vk-cube";
        app.apiVersion = VK_API_VERSION_1_3;
        ci.pApplicationInfo = &app;
        ci.enabledExtensionCount = num_enabled_inst;
        ci.ppEnabledExtensionNames = enabled_inst;
        res = vkCreateInstance(&ci, NULL, &g_instance);
        if (res == VK_ERROR_INCOMPATIBLE_DRIVER) {
            fprintf(stderr, "vk-cube: no compatible Vulkan driver, skipping\n");
            return EXIT_SKIP;
        }
        if (res != VK_SUCCESS) {
            fprintf(stderr, "vk-cube: vkCreateInstance failed (%d)\n", (int)res);
            return 1;
        }
    }

    /* Phase 1: instance-scope PFNs + extension flags. */
    if (!gloamVulkanLoadInstance(g_instance, VK_API_VERSION_1_3,
                                 num_enabled_inst, enabled_inst)) {
        fprintf(stderr, "vk-cube: gloamVulkanLoadInstance failed\n");
        return 1;
    }

    if (!SDL_Vulkan_CreateSurface(g_window, g_instance, NULL, &g_surface)) {
        fprintf(stderr, "vk-cube: SDL_Vulkan_CreateSurface failed (%s)\n", SDL_GetError());
        return 1;
    }

    /* Physical device: first one with a graphics queue that can present. */
    {
        uint32_t num_devices = 0, j, k;
        VkPhysicalDevice *devices;
        int found = 0;

        vkEnumeratePhysicalDevices(g_instance, &num_devices, NULL);
        if (num_devices == 0) {
            fprintf(stderr, "vk-cube: no Vulkan devices, skipping\n");
            return EXIT_SKIP;
        }
        devices = (VkPhysicalDevice *)calloc(num_devices, sizeof(*devices));
        vkEnumeratePhysicalDevices(g_instance, &num_devices, devices);

        for (j = 0; j < num_devices && !found; ++j) {
            uint32_t num_families = 0;
            VkQueueFamilyProperties *families;
            vkGetPhysicalDeviceQueueFamilyProperties(devices[j], &num_families, NULL);
            families = (VkQueueFamilyProperties *)calloc(num_families, sizeof(*families));
            vkGetPhysicalDeviceQueueFamilyProperties(devices[j], &num_families, families);
            for (k = 0; k < num_families; ++k) {
                VkBool32 present = VK_FALSE;
                vkGetPhysicalDeviceSurfaceSupportKHR(devices[j], k, g_surface, &present);
                if ((families[k].queueFlags & VK_QUEUE_GRAPHICS_BIT) && present) {
                    g_pd = devices[j];
                    queue_family = k;
                    found = 1;
                    break;
                }
            }
            free(families);
        }
        free(devices);
        if (!found) {
            fprintf(stderr, "vk-cube: no graphics+present queue, skipping\n");
            return EXIT_SKIP;
        }
    }

    {
        VkPhysicalDeviceProperties props;
        vkGetPhysicalDeviceProperties(g_pd, &props);
        printf("device: %s\n", props.deviceName);
    }
    vkGetPhysicalDeviceMemoryProperties(g_pd, &g_memory_props);

    /* Device: dynamic rendering, plus VK_KHR_portability_subset when the
     * implementation advertises it (spec requirement — the name is passed
     * through; the loader needs no types from it). */
    {
        uint32_t num_dev_props = 0, j;
        VkExtensionProperties *dev_props;
        float priority = 1.0f;
        VkDeviceQueueCreateInfo qci = { .sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO };
        VkPhysicalDeviceDynamicRenderingFeatures dyn_rendering =
            { .sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES };
        VkDeviceCreateInfo dci = { .sType = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO };
        VkResult res;

        enabled_dev[num_enabled_dev++] = VK_KHR_SWAPCHAIN_EXTENSION_NAME;
        vkEnumerateDeviceExtensionProperties(g_pd, NULL, &num_dev_props, NULL);
        dev_props = (VkExtensionProperties *)calloc(num_dev_props ? num_dev_props : 1,
                                                    sizeof(*dev_props));
        vkEnumerateDeviceExtensionProperties(g_pd, NULL, &num_dev_props, dev_props);
        for (j = 0; j < num_dev_props; ++j)
            if (strcmp(dev_props[j].extensionName, "VK_KHR_portability_subset") == 0) {
                enabled_dev[num_enabled_dev++] = "VK_KHR_portability_subset";
                break;
            }
        free(dev_props);

        qci.queueFamilyIndex = queue_family;
        qci.queueCount = 1;
        qci.pQueuePriorities = &priority;
        dyn_rendering.dynamicRendering = VK_TRUE;
        dci.pNext = &dyn_rendering;
        dci.queueCreateInfoCount = 1;
        dci.pQueueCreateInfos = &qci;
        dci.enabledExtensionCount = num_enabled_dev;
        dci.ppEnabledExtensionNames = enabled_dev;
        res = vkCreateDevice(g_pd, &dci, NULL, &g_device);
        if (res != VK_SUCCESS) {
            fprintf(stderr, "vk-cube: vkCreateDevice failed (%d)\n", (int)res);
            return 1;
        }
    }

    /* Phase 2: device-scope PFNs + extension flags. */
    if (!gloamVulkanLoadDevice(g_device, g_pd, num_enabled_dev, enabled_dev)) {
        fprintf(stderr, "vk-cube: gloamVulkanLoadDevice failed\n");
        return 1;
    }
    vkGetDeviceQueue(g_device, queue_family, 0, &g_queue);

    /* Surface format: prefer 8-bit BGRA/RGBA sRGB-nonlinear. */
    {
        uint32_t num_formats = 0, j;
        VkSurfaceFormatKHR *formats;
        vkGetPhysicalDeviceSurfaceFormatsKHR(g_pd, g_surface, &num_formats, NULL);
        formats = (VkSurfaceFormatKHR *)calloc(num_formats, sizeof(*formats));
        vkGetPhysicalDeviceSurfaceFormatsKHR(g_pd, g_surface, &num_formats, formats);
        g_surface_format = formats[0];
        for (j = 0; j < num_formats; ++j)
            if ((formats[j].format == VK_FORMAT_B8G8R8A8_UNORM ||
                 formats[j].format == VK_FORMAT_R8G8B8A8_UNORM) &&
                formats[j].colorSpace == VK_COLOR_SPACE_SRGB_NONLINEAR_KHR) {
                g_surface_format = formats[j];
                break;
            }
        free(formats);
    }

    create_pipeline();

    /* Vertex buffer (host-visible; a cube is 864 bytes). */
    {
        VkBufferCreateInfo bci = { .sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO };
        VkMemoryRequirements reqs;
        VkMemoryAllocateInfo mai = { .sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO };
        void *mapped = NULL;

        bci.size = sizeof(CUBE);
        bci.usage = VK_BUFFER_USAGE_VERTEX_BUFFER_BIT;
        bci.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
        vkCreateBuffer(g_device, &bci, NULL, &g_vertex_buffer);
        vkGetBufferMemoryRequirements(g_device, g_vertex_buffer, &reqs);
        mai.allocationSize = reqs.size;
        mai.memoryTypeIndex = find_memory_type(reqs.memoryTypeBits,
                                               VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT |
                                               VK_MEMORY_PROPERTY_HOST_COHERENT_BIT);
        vkAllocateMemory(g_device, &mai, NULL, &g_vertex_memory);
        vkBindBufferMemory(g_device, g_vertex_buffer, g_vertex_memory, 0);
        vkMapMemory(g_device, g_vertex_memory, 0, sizeof(CUBE), 0, &mapped);
        memcpy(mapped, CUBE, sizeof(CUBE));
        vkUnmapMemory(g_device, g_vertex_memory);
    }

    /* Command pool + per-frame objects. */
    {
        VkCommandPoolCreateInfo cpci = { .sType = VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO };
        VkCommandBufferAllocateInfo cbai = { .sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO };
        VkCommandBuffer cmds[FRAMES_IN_FLIGHT];

        cpci.flags = VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT;
        cpci.queueFamilyIndex = queue_family;
        vkCreateCommandPool(g_device, &cpci, NULL, &g_command_pool);

        cbai.commandPool = g_command_pool;
        cbai.level = VK_COMMAND_BUFFER_LEVEL_PRIMARY;
        cbai.commandBufferCount = FRAMES_IN_FLIGHT;
        vkAllocateCommandBuffers(g_device, &cbai, cmds);

        for (i = 0; i < FRAMES_IN_FLIGHT; ++i) {
            VkSemaphoreCreateInfo sem_ci = { .sType = VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO };
            VkFenceCreateInfo fci = { .sType = VK_STRUCTURE_TYPE_FENCE_CREATE_INFO };
            fci.flags = VK_FENCE_CREATE_SIGNALED_BIT; /* first wait passes */
            g_frames[i].cmd = cmds[i];
            vkCreateSemaphore(g_device, &sem_ci, NULL, &g_frames[i].acquire);
            vkCreateFence(g_device, &fci, NULL, &g_frames[i].fence);
        }
    }

    create_swapchain();

    for (;;) {
        SDL_Event ev;
        int quit = 0;
        float angle = g_ci ? 0.6f : (float)((double)SDL_GetTicks() * 0.001);

        while (SDL_PollEvent(&ev)) {
            if (ev.type == SDL_EVENT_QUIT ||
                (ev.type == SDL_EVENT_KEY_DOWN && ev.key.key == SDLK_ESCAPE))
                quit = 1;
        }
        if (quit)
            break;

        if (!draw(angle))
            create_swapchain();

        if (g_ci)
            break;
    }

    destroy_all();
    SDL_DestroyWindow(g_window);
    SDL_Quit();
    return 0;
}
