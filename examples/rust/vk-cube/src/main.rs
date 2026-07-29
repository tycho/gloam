//! vk-cube-rs — a spinning cube through a gloam-generated Vulkan loader.
//!
//! The Vulkan analogue of gl-triangle: real rendering with the pieces a real
//! application needs — a swapchain (with recreation on resize), a depth
//! buffer, explicit image-layout barriers around Vulkan 1.3 dynamic
//! rendering, per-frame synchronization (acquire/render semaphores plus
//! in-flight fences), and push-constant transforms.  Windowing is the
//! Rust-native winit; the surface comes from the generated loader's own
//! platform commands (`vkCreateWin32SurfaceKHR` / Xlib / Wayland, selected
//! from the winit window handle).
//!
//! Loading follows the phased contract on an owned context (`Vk::initialize`
//! → `load_instance` → `load_device`); the instance opts into portability
//! enumeration when the loader offers it, and the device enables
//! `VK_KHR_portability_subset` when advertised (spec-required), so MoltenVK
//! works out of the box.
//!
//! Run with `--ci` to render one frame offscreen-style (hidden window), copy
//! it back through a staging buffer, verify the center pixel, and exit.
//! Exit codes: 0 = pass, 1 = failure, 77 = no usable Vulkan (skip).

use std::ffi::{c_char, c_void, CStr};
use std::process::ExitCode;
use std::ptr;

use gloam_vk as vk;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

const FRAMES_IN_FLIGHT: usize = 2;
const DEPTH_FORMAT: vk::VkFormat = vk::VK_FORMAT_D32_SFLOAT;

// ---------------------------------------------------------------------------
// Small column-major mat4 helpers (Vulkan clip space: Y down, depth 0..1).
// ---------------------------------------------------------------------------

type Mat4 = [f32; 16];

fn mat_mul(a: &Mat4, b: &Mat4) -> Mat4 {
    let mut m = [0.0f32; 16];
    for c in 0..4 {
        for r in 0..4 {
            m[c * 4 + r] = (0..4).map(|k| a[k * 4 + r] * b[c * 4 + k]).sum();
        }
    }
    m
}

fn mat_perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    // Vulkan conventions baked in: depth mapped to [0, 1] and Y flipped so
    // +Y is up in application space.
    let f = 1.0 / (fov_y * 0.5).tan();
    let mut m = [0.0f32; 16];
    m[0] = f / aspect;
    m[5] = -f;
    m[10] = far / (near - far);
    m[11] = -1.0;
    m[14] = (near * far) / (near - far);
    m
}

fn mat_rotate_y(a: f32) -> Mat4 {
    let (s, c) = a.sin_cos();
    let mut m = mat_identity();
    m[0] = c;
    m[2] = -s;
    m[8] = s;
    m[10] = c;
    m
}

fn mat_rotate_x(a: f32) -> Mat4 {
    let (s, c) = a.sin_cos();
    let mut m = mat_identity();
    m[5] = c;
    m[6] = s;
    m[9] = -s;
    m[10] = c;
    m
}

fn mat_translate_z(z: f32) -> Mat4 {
    let mut m = mat_identity();
    m[14] = z;
    m
}

fn mat_identity() -> Mat4 {
    let mut m = [0.0f32; 16];
    m[0] = 1.0;
    m[5] = 1.0;
    m[10] = 1.0;
    m[15] = 1.0;
    m
}

// ---------------------------------------------------------------------------
// Cube geometry: 36 vertices, position + face color.
// ---------------------------------------------------------------------------

#[rustfmt::skip]
const CUBE: [[f32; 6]; 36] = {
    const R: [f32; 3] = [0.90, 0.24, 0.24];
    const G: [f32; 3] = [0.24, 0.80, 0.33];
    const B: [f32; 3] = [0.25, 0.42, 0.94];
    const Y: [f32; 3] = [0.95, 0.78, 0.20];
    const C: [f32; 3] = [0.20, 0.80, 0.83];
    const M: [f32; 3] = [0.83, 0.30, 0.80];
    const fn v(p: [f32; 3], c: [f32; 3]) -> [f32; 6] {
        [p[0], p[1], p[2], c[0], c[1], c[2]]
    }
    [
        // +Z front (red)
        v([-1.0,-1.0, 1.0],R), v([ 1.0,-1.0, 1.0],R), v([ 1.0, 1.0, 1.0],R),
        v([-1.0,-1.0, 1.0],R), v([ 1.0, 1.0, 1.0],R), v([-1.0, 1.0, 1.0],R),
        // -Z back (green)
        v([ 1.0,-1.0,-1.0],G), v([-1.0,-1.0,-1.0],G), v([-1.0, 1.0,-1.0],G),
        v([ 1.0,-1.0,-1.0],G), v([-1.0, 1.0,-1.0],G), v([ 1.0, 1.0,-1.0],G),
        // -X left (blue)
        v([-1.0,-1.0,-1.0],B), v([-1.0,-1.0, 1.0],B), v([-1.0, 1.0, 1.0],B),
        v([-1.0,-1.0,-1.0],B), v([-1.0, 1.0, 1.0],B), v([-1.0, 1.0,-1.0],B),
        // +X right (yellow)
        v([ 1.0,-1.0, 1.0],Y), v([ 1.0,-1.0,-1.0],Y), v([ 1.0, 1.0,-1.0],Y),
        v([ 1.0,-1.0, 1.0],Y), v([ 1.0, 1.0,-1.0],Y), v([ 1.0, 1.0, 1.0],Y),
        // +Y top (cyan)
        v([-1.0, 1.0, 1.0],C), v([ 1.0, 1.0, 1.0],C), v([ 1.0, 1.0,-1.0],C),
        v([-1.0, 1.0, 1.0],C), v([ 1.0, 1.0,-1.0],C), v([-1.0, 1.0,-1.0],C),
        // -Y bottom (magenta)
        v([-1.0,-1.0,-1.0],M), v([ 1.0,-1.0,-1.0],M), v([ 1.0,-1.0, 1.0],M),
        v([-1.0,-1.0,-1.0],M), v([ 1.0,-1.0, 1.0],M), v([-1.0,-1.0, 1.0],M),
    ]
};

// SPIR-V binaries, compiled from the GLSL sources next to them (see
// README.md).  Re-packed into u32 words at startup: include_bytes! carries
// no alignment guarantee, and vkCreateShaderModule wants word-aligned code.
const VERT_SPV: &[u8] = include_bytes!("../shaders/cube.vert.spv");
const FRAG_SPV: &[u8] = include_bytes!("../shaders/cube.frag.spv");

fn spirv_words(bytes: &[u8]) -> Vec<u32> {
    assert!(bytes.len().is_multiple_of(4), "SPIR-V is a u32 stream");
    bytes
        .chunks_exact(4)
        .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect()
}

// ---------------------------------------------------------------------------

const LIB_NAMES: &[&str] = &[
    #[cfg(target_os = "windows")]
    "vulkan-1.dll",
    #[cfg(target_os = "macos")]
    "libvulkan.1.dylib",
    #[cfg(target_os = "macos")]
    "libMoltenVK.dylib",
    #[cfg(all(unix, not(target_os = "macos")))]
    "libvulkan.so.1",
    #[cfg(all(unix, not(target_os = "macos")))]
    "libvulkan.so",
];

fn contains(props: &[vk::VkExtensionProperties], wanted: &CStr) -> bool {
    props
        .iter()
        .any(|p| unsafe { CStr::from_ptr(p.extensionName.as_ptr()) } == wanted)
}

/// Everything that lives for the whole app.
struct Ctx {
    vk: vk::Vk,
    instance: vk::VkInstance,
    surface: vk::VkSurfaceKHR,
    pd: vk::VkPhysicalDevice,
    device: vk::VkDevice,
    queue: vk::VkQueue,
    surface_format: vk::VkSurfaceFormatKHR,
    memory_props: vk::VkPhysicalDeviceMemoryProperties,
    pipeline_layout: vk::VkPipelineLayout,
    pipeline: vk::VkPipeline,
    vertex_buffer: vk::VkBuffer,
    vertex_memory: vk::VkDeviceMemory,
    command_pool: vk::VkCommandPool,
    frames: [Frame; FRAMES_IN_FLIGHT],
    frame_index: usize,
    swap: Option<Swapchain>,
    ci: bool,
    angle: f32,
    window: Window,
}

struct Frame {
    cmd: vk::VkCommandBuffer,
    acquire: vk::VkSemaphore,
    fence: vk::VkFence,
}

/// Everything that dies with the swapchain on resize.
struct Swapchain {
    handle: vk::VkSwapchainKHR,
    extent: vk::VkExtent2D,
    images: Vec<vk::VkImage>,
    views: Vec<vk::VkImageView>,
    /// One render-finished semaphore per swapchain image: present waits on
    /// the semaphore signaled by the submit that rendered that image.
    render_done: Vec<vk::VkSemaphore>,
    depth_image: vk::VkImage,
    depth_memory: vk::VkDeviceMemory,
    depth_view: vk::VkImageView,
}

fn find_memory_type(
    props: &vk::VkPhysicalDeviceMemoryProperties,
    type_bits: u32,
    required: vk::VkMemoryPropertyFlags,
) -> u32 {
    for i in 0..props.memoryTypeCount {
        if type_bits & (1 << i) != 0
            && props.memoryTypes[i as usize].propertyFlags & required == required
        {
            return i;
        }
    }
    panic!("no memory type matches 0x{type_bits:x} with flags 0x{required:x}");
}

impl Ctx {
    unsafe fn create_swapchain(&mut self) {
        let vk = &self.vk;
        let old = self.swap.take();

        let mut caps: vk::VkSurfaceCapabilitiesKHR = unsafe { std::mem::zeroed() };
        unsafe {
            vk.GetPhysicalDeviceSurfaceCapabilitiesKHR(self.pd, self.surface, &mut caps);
        }
        let extent = if caps.currentExtent.width != u32::MAX {
            caps.currentExtent
        } else {
            let size = self.window.inner_size();
            vk::VkExtent2D {
                width: size
                    .width
                    .clamp(caps.minImageExtent.width, caps.maxImageExtent.width),
                height: size
                    .height
                    .clamp(caps.minImageExtent.height, caps.maxImageExtent.height),
            }
        };
        let mut min_images = caps.minImageCount + 1;
        if caps.maxImageCount != 0 {
            min_images = min_images.min(caps.maxImageCount);
        }

        // TRANSFER_SRC lets --ci copy the rendered image back out.
        let mut usage = vk::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT;
        if caps.supportedUsageFlags & vk::VK_IMAGE_USAGE_TRANSFER_SRC_BIT != 0 {
            usage |= vk::VK_IMAGE_USAGE_TRANSFER_SRC_BIT;
        } else {
            assert!(!self.ci, "--ci needs a TRANSFER_SRC-capable swapchain");
        }

        let mut sci: vk::VkSwapchainCreateInfoKHR = unsafe { std::mem::zeroed() };
        sci.sType = vk::VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR;
        sci.surface = self.surface;
        sci.minImageCount = min_images;
        sci.imageFormat = self.surface_format.format;
        sci.imageColorSpace = self.surface_format.colorSpace;
        sci.imageExtent = extent;
        sci.imageArrayLayers = 1;
        sci.imageUsage = usage;
        sci.imageSharingMode = vk::VK_SHARING_MODE_EXCLUSIVE;
        sci.preTransform = caps.currentTransform;
        sci.compositeAlpha = vk::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR;
        sci.presentMode = vk::VK_PRESENT_MODE_FIFO_KHR; // always available
        sci.clipped = vk::VK_TRUE as vk::VkBool32;
        sci.oldSwapchain = old.as_ref().map_or(vk::VkSwapchainKHR(0), |s| s.handle);

        let mut handle = vk::VkSwapchainKHR(0);
        let r = unsafe { vk.CreateSwapchainKHR(self.device, &sci, ptr::null(), &mut handle) };
        assert!(r == vk::VK_SUCCESS, "vkCreateSwapchainKHR failed ({})", r.0);

        // The old swapchain (and its per-image objects) can be destroyed once
        // the device is idle; recreation only happens on resize, so a full
        // wait is the simple and correct choice.
        if let Some(old) = old {
            unsafe {
                vk.DeviceWaitIdle(self.device);
                self.destroy_swapchain_objects(&old);
            }
        }

        let mut count = 0u32;
        unsafe {
            vk.GetSwapchainImagesKHR(self.device, handle, &mut count, ptr::null_mut());
        }
        let mut images = vec![vk::VkImage(0); count as usize];
        unsafe {
            vk.GetSwapchainImagesKHR(self.device, handle, &mut count, images.as_mut_ptr());
        }

        let mut views = Vec::with_capacity(images.len());
        let mut render_done = Vec::with_capacity(images.len());
        for &image in &images {
            let mut ivci: vk::VkImageViewCreateInfo = unsafe { std::mem::zeroed() };
            ivci.sType = vk::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
            ivci.image = image;
            ivci.viewType = vk::VK_IMAGE_VIEW_TYPE_2D;
            ivci.format = self.surface_format.format;
            ivci.subresourceRange.aspectMask = vk::VK_IMAGE_ASPECT_COLOR_BIT;
            ivci.subresourceRange.levelCount = 1;
            ivci.subresourceRange.layerCount = 1;
            let mut view = vk::VkImageView(0);
            let r = unsafe { vk.CreateImageView(self.device, &ivci, ptr::null(), &mut view) };
            assert!(r == vk::VK_SUCCESS, "vkCreateImageView failed ({})", r.0);
            views.push(view);

            let mut sem_ci: vk::VkSemaphoreCreateInfo = unsafe { std::mem::zeroed() };
            sem_ci.sType = vk::VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO;
            let mut sem = vk::VkSemaphore(0);
            unsafe { vk.CreateSemaphore(self.device, &sem_ci, ptr::null(), &mut sem) };
            render_done.push(sem);
        }

        // Depth buffer, recreated with the swapchain (it tracks the extent).
        let mut ici: vk::VkImageCreateInfo = unsafe { std::mem::zeroed() };
        ici.sType = vk::VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO;
        ici.imageType = vk::VK_IMAGE_TYPE_2D;
        ici.format = DEPTH_FORMAT;
        ici.extent = vk::VkExtent3D {
            width: extent.width,
            height: extent.height,
            depth: 1,
        };
        ici.mipLevels = 1;
        ici.arrayLayers = 1;
        ici.samples = vk::VK_SAMPLE_COUNT_1_BIT;
        ici.tiling = vk::VK_IMAGE_TILING_OPTIMAL;
        ici.usage = vk::VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT;
        ici.initialLayout = vk::VK_IMAGE_LAYOUT_UNDEFINED;
        let mut depth_image = vk::VkImage(0);
        let r = unsafe { vk.CreateImage(self.device, &ici, ptr::null(), &mut depth_image) };
        assert!(r == vk::VK_SUCCESS, "depth vkCreateImage failed ({})", r.0);

        let mut reqs: vk::VkMemoryRequirements = unsafe { std::mem::zeroed() };
        unsafe { vk.GetImageMemoryRequirements(self.device, depth_image, &mut reqs) };
        let mut mai: vk::VkMemoryAllocateInfo = unsafe { std::mem::zeroed() };
        mai.sType = vk::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
        mai.allocationSize = reqs.size;
        mai.memoryTypeIndex = find_memory_type(
            &self.memory_props,
            reqs.memoryTypeBits,
            vk::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT,
        );
        let mut depth_memory = vk::VkDeviceMemory(0);
        unsafe {
            vk.AllocateMemory(self.device, &mai, ptr::null(), &mut depth_memory);
            vk.BindImageMemory(self.device, depth_image, depth_memory, 0);
        }

        let mut dvci: vk::VkImageViewCreateInfo = unsafe { std::mem::zeroed() };
        dvci.sType = vk::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
        dvci.image = depth_image;
        dvci.viewType = vk::VK_IMAGE_VIEW_TYPE_2D;
        dvci.format = DEPTH_FORMAT;
        dvci.subresourceRange.aspectMask = vk::VK_IMAGE_ASPECT_DEPTH_BIT;
        dvci.subresourceRange.levelCount = 1;
        dvci.subresourceRange.layerCount = 1;
        let mut depth_view = vk::VkImageView(0);
        unsafe { vk.CreateImageView(self.device, &dvci, ptr::null(), &mut depth_view) };

        self.swap = Some(Swapchain {
            handle,
            extent,
            images,
            views,
            render_done,
            depth_image,
            depth_memory,
            depth_view,
        });
    }

    /// Full teardown, in reverse creation order.  Waits for the device to go
    /// idle first so nothing is destroyed out from under in-flight work.
    unsafe fn destroy(&mut self) {
        unsafe {
            self.vk.DeviceWaitIdle(self.device);
            if let Some(s) = self.swap.take() {
                self.destroy_swapchain_objects(&s);
            }
            let vk = &self.vk;
            for f in &self.frames {
                vk.DestroySemaphore(self.device, f.acquire, ptr::null());
                vk.DestroyFence(self.device, f.fence, ptr::null());
            }
            vk.DestroyCommandPool(self.device, self.command_pool, ptr::null());
            vk.DestroyBuffer(self.device, self.vertex_buffer, ptr::null());
            vk.FreeMemory(self.device, self.vertex_memory, ptr::null());
            vk.DestroyPipeline(self.device, self.pipeline, ptr::null());
            vk.DestroyPipelineLayout(self.device, self.pipeline_layout, ptr::null());
            vk.DestroyDevice(self.device, ptr::null());
            vk.DestroySurfaceKHR(self.instance, self.surface, ptr::null());
            vk.DestroyInstance(self.instance, ptr::null());
        }
    }

    unsafe fn destroy_swapchain_objects(&self, s: &Swapchain) {
        let vk = &self.vk;
        unsafe {
            for &v in &s.views {
                vk.DestroyImageView(self.device, v, ptr::null());
            }
            for &sem in &s.render_done {
                vk.DestroySemaphore(self.device, sem, ptr::null());
            }
            vk.DestroyImageView(self.device, s.depth_view, ptr::null());
            vk.DestroyImage(self.device, s.depth_image, ptr::null());
            vk.FreeMemory(self.device, s.depth_memory, ptr::null());
            vk.DestroySwapchainKHR(self.device, s.handle, ptr::null());
        }
    }

    /// Record and submit one frame; returns false when the swapchain needs
    /// recreation (out-of-date / suboptimal).
    unsafe fn draw(&mut self) -> bool {
        let vk = &self.vk;
        let swap = self.swap.as_ref().expect("swapchain");
        let frame = &self.frames[self.frame_index];

        unsafe {
            vk.WaitForFences(
                self.device,
                1,
                &frame.fence,
                vk::VK_TRUE as vk::VkBool32,
                u64::MAX,
            );
        }

        let mut image_index = 0u32;
        let r = unsafe {
            vk.AcquireNextImageKHR(
                self.device,
                swap.handle,
                u64::MAX,
                frame.acquire,
                vk::VkFence(0),
                &mut image_index,
            )
        };
        if r == vk::VK_ERROR_OUT_OF_DATE_KHR {
            return false;
        }
        assert!(
            r == vk::VK_SUCCESS || r == vk::VK_SUBOPTIMAL_KHR,
            "vkAcquireNextImageKHR failed ({})",
            r.0
        );
        unsafe { vk.ResetFences(self.device, 1, &frame.fence) };

        let image = swap.images[image_index as usize];
        let view = swap.views[image_index as usize];
        let render_done = swap.render_done[image_index as usize];
        let extent = swap.extent;
        let cmd = frame.cmd;

        let mut begin: vk::VkCommandBufferBeginInfo = unsafe { std::mem::zeroed() };
        begin.sType = vk::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;
        begin.flags = vk::VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT;
        unsafe { vk.BeginCommandBuffer(cmd, &begin) };

        // Layout transitions into rendering: the color target goes
        // UNDEFINED → COLOR_ATTACHMENT_OPTIMAL (contents are cleared, so the
        // previous frame's contents can be discarded), and the depth buffer
        // UNDEFINED → DEPTH_ATTACHMENT_OPTIMAL likewise.  Dynamic rendering
        // has no render pass to do implicit transitions — these barriers are
        // the application's job.
        let mut to_color: vk::VkImageMemoryBarrier = unsafe { std::mem::zeroed() };
        to_color.sType = vk::VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER;
        to_color.srcAccessMask = 0;
        to_color.dstAccessMask = vk::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT;
        to_color.oldLayout = vk::VK_IMAGE_LAYOUT_UNDEFINED;
        to_color.newLayout = vk::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL;
        to_color.srcQueueFamilyIndex = vk::VK_QUEUE_FAMILY_IGNORED;
        to_color.dstQueueFamilyIndex = vk::VK_QUEUE_FAMILY_IGNORED;
        to_color.image = image;
        to_color.subresourceRange.aspectMask = vk::VK_IMAGE_ASPECT_COLOR_BIT;
        to_color.subresourceRange.levelCount = 1;
        to_color.subresourceRange.layerCount = 1;

        let mut to_depth = to_color;
        to_depth.dstAccessMask = vk::VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT;
        to_depth.newLayout = vk::VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL;
        to_depth.image = swap.depth_image;
        to_depth.subresourceRange.aspectMask = vk::VK_IMAGE_ASPECT_DEPTH_BIT;

        let barriers = [to_color, to_depth];
        unsafe {
            vk.CmdPipelineBarrier(
                cmd,
                vk::VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT,
                vk::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT
                    | vk::VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT,
                0,
                0,
                ptr::null(),
                0,
                ptr::null(),
                barriers.len() as u32,
                barriers.as_ptr(),
            );
        }

        // Dynamic rendering: attachments described inline, no render pass.
        let mut color_att: vk::VkRenderingAttachmentInfo = unsafe { std::mem::zeroed() };
        color_att.sType = vk::VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO;
        color_att.imageView = view;
        color_att.imageLayout = vk::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL;
        color_att.loadOp = vk::VK_ATTACHMENT_LOAD_OP_CLEAR;
        color_att.storeOp = vk::VK_ATTACHMENT_STORE_OP_STORE;
        color_att.clearValue.color.float32 = [0.05, 0.05, 0.08, 1.0];

        let mut depth_att: vk::VkRenderingAttachmentInfo = unsafe { std::mem::zeroed() };
        depth_att.sType = vk::VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO;
        depth_att.imageView = swap.depth_view;
        depth_att.imageLayout = vk::VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL;
        depth_att.loadOp = vk::VK_ATTACHMENT_LOAD_OP_CLEAR;
        depth_att.storeOp = vk::VK_ATTACHMENT_STORE_OP_DONT_CARE;
        depth_att.clearValue.depthStencil.depth = 1.0;

        let mut rendering: vk::VkRenderingInfo = unsafe { std::mem::zeroed() };
        rendering.sType = vk::VK_STRUCTURE_TYPE_RENDERING_INFO;
        rendering.renderArea.extent = extent;
        rendering.layerCount = 1;
        rendering.colorAttachmentCount = 1;
        rendering.pColorAttachments = &color_att;
        rendering.pDepthAttachment = &depth_att;

        let mvp = {
            let aspect = extent.width as f32 / extent.height.max(1) as f32;
            let proj = mat_perspective(std::f32::consts::FRAC_PI_3, aspect, 0.1, 10.0);
            let view_m = mat_translate_z(-4.5);
            let model = mat_mul(&mat_rotate_y(self.angle), &mat_rotate_x(self.angle * 0.7));
            mat_mul(&proj, &mat_mul(&view_m, &model))
        };

        unsafe {
            vk.CmdBeginRendering(cmd, &rendering);
            let viewport = vk::VkViewport {
                x: 0.0,
                y: 0.0,
                width: extent.width as f32,
                height: extent.height as f32,
                minDepth: 0.0,
                maxDepth: 1.0,
            };
            let scissor = vk::VkRect2D {
                offset: vk::VkOffset2D { x: 0, y: 0 },
                extent,
            };
            vk.CmdSetViewport(cmd, 0, 1, &viewport);
            vk.CmdSetScissor(cmd, 0, 1, &scissor);
            vk.CmdBindPipeline(cmd, vk::VK_PIPELINE_BIND_POINT_GRAPHICS, self.pipeline);
            vk.CmdPushConstants(
                cmd,
                self.pipeline_layout,
                vk::VK_SHADER_STAGE_VERTEX_BIT,
                0,
                std::mem::size_of::<Mat4>() as u32,
                mvp.as_ptr() as *const c_void,
            );
            let offset: vk::VkDeviceSize = 0;
            vk.CmdBindVertexBuffers(cmd, 0, 1, &self.vertex_buffer, &offset);
            vk.CmdDraw(cmd, CUBE.len() as u32, 1, 0, 0);
            vk.CmdEndRendering(cmd);
        }

        // Out of rendering: COLOR_ATTACHMENT → PRESENT_SRC before the
        // presentation engine may read the image.
        let mut to_present = to_color;
        to_present.srcAccessMask = vk::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT;
        to_present.dstAccessMask = 0;
        to_present.oldLayout = vk::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL;
        to_present.newLayout = vk::VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
        unsafe {
            vk.CmdPipelineBarrier(
                cmd,
                vk::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
                vk::VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT,
                0,
                0,
                ptr::null(),
                0,
                ptr::null(),
                1,
                &to_present,
            );
            vk.EndCommandBuffer(cmd);
        }

        let wait_stage: vk::VkPipelineStageFlags =
            vk::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT;
        let mut submit: vk::VkSubmitInfo = unsafe { std::mem::zeroed() };
        submit.sType = vk::VK_STRUCTURE_TYPE_SUBMIT_INFO;
        submit.waitSemaphoreCount = 1;
        submit.pWaitSemaphores = &frame.acquire;
        submit.pWaitDstStageMask = &wait_stage;
        submit.commandBufferCount = 1;
        submit.pCommandBuffers = &cmd;
        submit.signalSemaphoreCount = 1;
        submit.pSignalSemaphores = &render_done;
        let r = unsafe { vk.QueueSubmit(self.queue, 1, &submit, frame.fence) };
        assert!(r == vk::VK_SUCCESS, "vkQueueSubmit failed ({})", r.0);

        if self.ci {
            // Verify before presenting: wait for the render, copy the center
            // pixel out through a staging buffer, and check it.
            unsafe { self.verify_center_pixel(image, extent, image_index) };
            return true;
        }

        let mut present: vk::VkPresentInfoKHR = unsafe { std::mem::zeroed() };
        present.sType = vk::VK_STRUCTURE_TYPE_PRESENT_INFO_KHR;
        present.waitSemaphoreCount = 1;
        present.pWaitSemaphores = &render_done;
        present.swapchainCount = 1;
        present.pSwapchains = &swap.handle;
        present.pImageIndices = &image_index;
        let r = unsafe { vk.QueuePresentKHR(self.queue, &present) };
        self.frame_index = (self.frame_index + 1) % FRAMES_IN_FLIGHT;
        if r == vk::VK_ERROR_OUT_OF_DATE_KHR || r == vk::VK_SUBOPTIMAL_KHR {
            return false;
        }
        assert!(r == vk::VK_SUCCESS, "vkQueuePresentKHR failed ({})", r.0);
        true
    }

    /// --ci: copy the just-rendered swapchain image into a host-visible
    /// buffer (one more barrier pair: PRESENT-bound image → TRANSFER_SRC)
    /// and assert the center pixel is cube, not background.
    unsafe fn verify_center_pixel(
        &self,
        image: vk::VkImage,
        extent: vk::VkExtent2D,
        _image_index: u32,
    ) {
        let vk = &self.vk;
        let frame = &self.frames[self.frame_index];
        unsafe {
            vk.WaitForFences(
                self.device,
                1,
                &frame.fence,
                vk::VK_TRUE as vk::VkBool32,
                u64::MAX,
            );
        }

        // Staging buffer for one RGBA pixel row read (whole image kept
        // simple: 4 bytes/pixel).
        let size = (extent.width as u64) * (extent.height as u64) * 4;
        let mut bci: vk::VkBufferCreateInfo = unsafe { std::mem::zeroed() };
        bci.sType = vk::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
        bci.size = size;
        bci.usage = vk::VK_BUFFER_USAGE_TRANSFER_DST_BIT;
        bci.sharingMode = vk::VK_SHARING_MODE_EXCLUSIVE;
        let mut buffer = vk::VkBuffer(0);
        unsafe { vk.CreateBuffer(self.device, &bci, ptr::null(), &mut buffer) };
        let mut reqs: vk::VkMemoryRequirements = unsafe { std::mem::zeroed() };
        unsafe { vk.GetBufferMemoryRequirements(self.device, buffer, &mut reqs) };
        let mut mai: vk::VkMemoryAllocateInfo = unsafe { std::mem::zeroed() };
        mai.sType = vk::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
        mai.allocationSize = reqs.size;
        mai.memoryTypeIndex = find_memory_type(
            &self.memory_props,
            reqs.memoryTypeBits,
            vk::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | vk::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT,
        );
        let mut memory = vk::VkDeviceMemory(0);
        unsafe {
            vk.AllocateMemory(self.device, &mai, ptr::null(), &mut memory);
            vk.BindBufferMemory(self.device, buffer, memory, 0);
        }

        let cmd = frame.cmd;
        let mut begin: vk::VkCommandBufferBeginInfo = unsafe { std::mem::zeroed() };
        begin.sType = vk::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;
        begin.flags = vk::VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT;
        unsafe { vk.BeginCommandBuffer(cmd, &begin) };

        let mut to_src: vk::VkImageMemoryBarrier = unsafe { std::mem::zeroed() };
        to_src.sType = vk::VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER;
        to_src.srcAccessMask = vk::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT;
        to_src.dstAccessMask = vk::VK_ACCESS_TRANSFER_READ_BIT;
        to_src.oldLayout = vk::VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
        to_src.newLayout = vk::VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL;
        to_src.srcQueueFamilyIndex = vk::VK_QUEUE_FAMILY_IGNORED;
        to_src.dstQueueFamilyIndex = vk::VK_QUEUE_FAMILY_IGNORED;
        to_src.image = image;
        to_src.subresourceRange.aspectMask = vk::VK_IMAGE_ASPECT_COLOR_BIT;
        to_src.subresourceRange.levelCount = 1;
        to_src.subresourceRange.layerCount = 1;

        let mut region: vk::VkBufferImageCopy = unsafe { std::mem::zeroed() };
        region.imageSubresource.aspectMask = vk::VK_IMAGE_ASPECT_COLOR_BIT;
        region.imageSubresource.layerCount = 1;
        region.imageExtent = vk::VkExtent3D {
            width: extent.width,
            height: extent.height,
            depth: 1,
        };

        unsafe {
            vk.CmdPipelineBarrier(
                cmd,
                vk::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
                vk::VK_PIPELINE_STAGE_TRANSFER_BIT,
                0,
                0,
                ptr::null(),
                0,
                ptr::null(),
                1,
                &to_src,
            );
            vk.CmdCopyImageToBuffer(
                cmd,
                image,
                vk::VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
                buffer,
                1,
                &region,
            );
            vk.EndCommandBuffer(cmd);
        }

        let mut submit: vk::VkSubmitInfo = unsafe { std::mem::zeroed() };
        submit.sType = vk::VK_STRUCTURE_TYPE_SUBMIT_INFO;
        submit.commandBufferCount = 1;
        submit.pCommandBuffers = &cmd;
        unsafe {
            vk.ResetFences(self.device, 1, &frame.fence);
            vk.QueueSubmit(self.queue, 1, &submit, frame.fence);
            vk.WaitForFences(
                self.device,
                1,
                &frame.fence,
                vk::VK_TRUE as vk::VkBool32,
                u64::MAX,
            );
        }

        let mut mapped: *mut c_void = ptr::null_mut();
        unsafe { vk.MapMemory(self.device, memory, 0, size, 0, &mut mapped) };
        let center = ((extent.height / 2) * extent.width + extent.width / 2) as usize * 4;
        let px = unsafe { std::slice::from_raw_parts(mapped as *const u8, size as usize) };
        // The surface format is BGRA or RGBA; a brightness check makes the
        // assertion order-independent.
        let (b0, b1, b2) = (px[center], px[center + 1], px[center + 2]);
        println!("center pixel: {b0} {b1} {b2}");
        unsafe {
            vk.UnmapMemory(self.device, memory);
            vk.DestroyBuffer(self.device, buffer, ptr::null());
            vk.FreeMemory(self.device, memory, ptr::null());
        }
        assert!(
            b0 as u32 + b1 as u32 + b2 as u32 > 60,
            "center pixel is background — cube did not render"
        );
        println!("\nvk-cube (Rust, gloam Vulkan loader): PASS");
    }
}

// ---------------------------------------------------------------------------

struct App {
    ci: bool,
    ctx: Option<Ctx>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.ctx.is_some() {
            return;
        }
        let attrs = Window::default_attributes()
            .with_title("gloam vk-cube (Rust)")
            .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
            .with_visible(!self.ci);
        let window = event_loop.create_window(attrs).expect("window");

        match unsafe { setup(window, self.ci) } {
            Ok(ctx) => {
                self.ctx = Some(ctx);
                if self.ci {
                    let ctx = self.ctx.as_mut().unwrap();
                    unsafe {
                        ctx.draw();
                        ctx.destroy();
                    }
                    event_loop.exit();
                } else {
                    self.ctx.as_ref().unwrap().window.request_redraw();
                }
            }
            Err(code) => {
                // 77 = skip (no Vulkan / no adequate device), like the C
                // examples.
                std::process::exit(code);
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(ctx) = self.ctx.as_mut() else {
            return;
        };
        match event {
            WindowEvent::CloseRequested => {
                unsafe { ctx.destroy() };
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                ctx.angle += 0.015;
                if !unsafe { ctx.draw() } {
                    unsafe { ctx.create_swapchain() };
                }
                ctx.window.request_redraw();
            }
            _ => {}
        }
    }
}

/// All one-time setup: library → instance → surface → device → pipeline →
/// buffers → per-frame objects → first swapchain.  Returns the exit code on
/// environment problems (77 = skip).
unsafe fn setup(window: Window, ci: bool) -> Result<Ctx, i32> {
    let Some(lib) = LIB_NAMES
        .iter()
        .find_map(|n| unsafe { libloading::Library::new(n).ok() })
    else {
        eprintln!("vk-cube-rs: no Vulkan runtime found (skip)");
        return Err(77);
    };
    let gipa: vk::PFN_vkGetInstanceProcAddr = unsafe {
        match lib.get::<vk::PFN_vkGetInstanceProcAddr>(b"vkGetInstanceProcAddr\0") {
            Ok(sym) => *sym,
            Err(_) => {
                eprintln!("vk-cube-rs: vkGetInstanceProcAddr missing (skip)");
                return Err(77);
            }
        }
    };
    // The loader must outlive every Vulkan object; leak the handle for the
    // process lifetime, exactly like linking against the loader in C.
    std::mem::forget(lib);

    let mut vkc = vk::Vk::new();
    unsafe { vkc.initialize(gipa) };

    // ---- Instance -----------------------------------------------------
    let mut n = 0u32;
    unsafe { vkc.EnumerateInstanceExtensionProperties(ptr::null(), &mut n, ptr::null_mut()) };
    let mut inst_props: Vec<vk::VkExtensionProperties> =
        vec![unsafe { std::mem::zeroed() }; n as usize];
    unsafe {
        vkc.EnumerateInstanceExtensionProperties(ptr::null(), &mut n, inst_props.as_mut_ptr())
    };

    let platform_surface_ext: &CStr = match window.display_handle().map(|h| h.as_raw()) {
        Ok(RawDisplayHandle::Windows(_)) => c"VK_KHR_win32_surface",
        Ok(RawDisplayHandle::Xlib(_)) => c"VK_KHR_xlib_surface",
        Ok(RawDisplayHandle::Wayland(_)) => c"VK_KHR_wayland_surface",
        other => {
            eprintln!("vk-cube-rs: unsupported display system {other:?} (skip)");
            return Err(77);
        }
    };
    let mut enabled_instance: Vec<&CStr> = vec![c"VK_KHR_surface", platform_surface_ext];
    for e in &enabled_instance {
        if !contains(&inst_props, e) {
            eprintln!("vk-cube-rs: required instance extension {e:?} unavailable (skip)");
            return Err(77);
        }
    }
    let mut instance_flags: vk::VkInstanceCreateFlags = 0;
    if contains(
        &inst_props,
        vk::VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME,
    ) {
        enabled_instance.push(vk::VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME);
        instance_flags |= vk::VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR;
    }
    let inst_ptrs: Vec<*const c_char> = enabled_instance.iter().map(|e| e.as_ptr()).collect();

    let mut app_info: vk::VkApplicationInfo = unsafe { std::mem::zeroed() };
    app_info.sType = vk::VK_STRUCTURE_TYPE_APPLICATION_INFO;
    app_info.pApplicationName = c"gloam vk-cube".as_ptr();
    app_info.apiVersion = vk::VK_API_VERSION_1_3;
    let mut ici: vk::VkInstanceCreateInfo = unsafe { std::mem::zeroed() };
    ici.sType = vk::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
    ici.flags = instance_flags;
    ici.pApplicationInfo = &app_info;
    ici.enabledExtensionCount = inst_ptrs.len() as u32;
    ici.ppEnabledExtensionNames = inst_ptrs.as_ptr();
    let mut instance = vk::VkInstance(ptr::null_mut());
    let r = unsafe { vkc.CreateInstance(&ici, ptr::null(), &mut instance) };
    if r == vk::VK_ERROR_INCOMPATIBLE_DRIVER {
        eprintln!("vk-cube-rs: no compatible Vulkan driver (skip)");
        return Err(77);
    }
    assert!(r == vk::VK_SUCCESS, "vkCreateInstance failed ({})", r.0);

    if !unsafe { vkc.load_instance(instance, vk::VK_API_VERSION_1_3, &enabled_instance) } {
        eprintln!("vk-cube-rs: load_instance failed");
        return Err(1);
    }

    // ---- Surface (through the loader's own platform commands) ----------
    let surface = unsafe { create_surface(&vkc, instance, &window) };

    // ---- Physical device: graphics queue family + present support ------
    let mut count = 0u32;
    unsafe { vkc.EnumeratePhysicalDevices(instance, &mut count, ptr::null_mut()) };
    if count == 0 {
        eprintln!("vk-cube-rs: no physical devices (skip)");
        return Err(77);
    }
    let mut devices = vec![vk::VkPhysicalDevice(ptr::null_mut()); count as usize];
    unsafe { vkc.EnumeratePhysicalDevices(instance, &mut count, devices.as_mut_ptr()) };

    let mut chosen: Option<(vk::VkPhysicalDevice, u32)> = None;
    'outer: for &pd in &devices {
        let mut qf_count = 0u32;
        unsafe {
            vkc.GetPhysicalDeviceQueueFamilyProperties(pd, &mut qf_count, ptr::null_mut());
        }
        let mut qf: Vec<vk::VkQueueFamilyProperties> =
            vec![unsafe { std::mem::zeroed() }; qf_count as usize];
        unsafe {
            vkc.GetPhysicalDeviceQueueFamilyProperties(pd, &mut qf_count, qf.as_mut_ptr());
        }
        for (i, f) in qf.iter().enumerate() {
            let mut supports_present: vk::VkBool32 = 0;
            unsafe {
                vkc.GetPhysicalDeviceSurfaceSupportKHR(
                    pd,
                    i as u32,
                    surface,
                    &mut supports_present,
                );
            }
            if f.queueFlags & vk::VK_QUEUE_GRAPHICS_BIT != 0 && supports_present != 0 {
                chosen = Some((pd, i as u32));
                break 'outer;
            }
        }
    }
    let Some((pd, queue_family)) = chosen else {
        eprintln!("vk-cube-rs: no graphics+present queue (skip)");
        return Err(77);
    };

    let mut props: vk::VkPhysicalDeviceProperties = unsafe { std::mem::zeroed() };
    unsafe { vkc.GetPhysicalDeviceProperties(pd, &mut props) };
    println!(
        "device: {}",
        unsafe { CStr::from_ptr(props.deviceName.as_ptr()) }.to_string_lossy()
    );

    let mut memory_props: vk::VkPhysicalDeviceMemoryProperties = unsafe { std::mem::zeroed() };
    unsafe { vkc.GetPhysicalDeviceMemoryProperties(pd, &mut memory_props) };

    // ---- Device (dynamic rendering + portability subset when advertised) --
    let mut de_count = 0u32;
    unsafe {
        vkc.EnumerateDeviceExtensionProperties(pd, ptr::null(), &mut de_count, ptr::null_mut());
    }
    let mut dev_props: Vec<vk::VkExtensionProperties> =
        vec![unsafe { std::mem::zeroed() }; de_count as usize];
    unsafe {
        vkc.EnumerateDeviceExtensionProperties(
            pd,
            ptr::null(),
            &mut de_count,
            dev_props.as_mut_ptr(),
        );
    }
    let mut enabled_device: Vec<&CStr> = vec![c"VK_KHR_swapchain"];
    // Spec requirement: a portability-subset device must have the extension
    // enabled.  (The name is passed through; the loader needs no types from
    // it.)
    if contains(&dev_props, c"VK_KHR_portability_subset") {
        enabled_device.push(c"VK_KHR_portability_subset");
    }
    let dev_ptrs: Vec<*const c_char> = enabled_device.iter().map(|e| e.as_ptr()).collect();

    let priority = 1.0f32;
    let mut qci: vk::VkDeviceQueueCreateInfo = unsafe { std::mem::zeroed() };
    qci.sType = vk::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
    qci.queueFamilyIndex = queue_family;
    qci.queueCount = 1;
    qci.pQueuePriorities = &priority;

    let mut dyn_rendering: vk::VkPhysicalDeviceDynamicRenderingFeatures =
        unsafe { std::mem::zeroed() };
    dyn_rendering.sType = vk::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES;
    dyn_rendering.dynamicRendering = vk::VK_TRUE as vk::VkBool32;

    let mut dci: vk::VkDeviceCreateInfo = unsafe { std::mem::zeroed() };
    dci.sType = vk::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
    dci.pNext = &dyn_rendering as *const _ as *const c_void;
    dci.queueCreateInfoCount = 1;
    dci.pQueueCreateInfos = &qci;
    dci.enabledExtensionCount = dev_ptrs.len() as u32;
    dci.ppEnabledExtensionNames = dev_ptrs.as_ptr();
    let mut device = vk::VkDevice(ptr::null_mut());
    let r = unsafe { vkc.CreateDevice(pd, &dci, ptr::null(), &mut device) };
    assert!(r == vk::VK_SUCCESS, "vkCreateDevice failed ({})", r.0);

    if !unsafe { vkc.load_device(device, pd, &enabled_device) } {
        eprintln!("vk-cube-rs: load_device failed");
        return Err(1);
    }

    let mut queue = vk::VkQueue(ptr::null_mut());
    unsafe { vkc.GetDeviceQueue(device, queue_family, 0, &mut queue) };

    // ---- Surface format --------------------------------------------------
    let mut fmt_count = 0u32;
    unsafe {
        vkc.GetPhysicalDeviceSurfaceFormatsKHR(pd, surface, &mut fmt_count, ptr::null_mut());
    }
    let mut formats: Vec<vk::VkSurfaceFormatKHR> =
        vec![unsafe { std::mem::zeroed() }; fmt_count as usize];
    unsafe {
        vkc.GetPhysicalDeviceSurfaceFormatsKHR(pd, surface, &mut fmt_count, formats.as_mut_ptr());
    }
    let surface_format = formats
        .iter()
        .find(|f| {
            (f.format == vk::VK_FORMAT_B8G8R8A8_UNORM || f.format == vk::VK_FORMAT_R8G8B8A8_UNORM)
                && f.colorSpace == vk::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR
        })
        .copied()
        .unwrap_or(formats[0]);

    // ---- Pipeline ----------------------------------------------------------
    let (pipeline_layout, pipeline) =
        unsafe { create_pipeline(&vkc, device, surface_format.format) };

    // ---- Vertex buffer (host-visible; a cube is 864 bytes) -----------------
    let vb_size = std::mem::size_of_val(&CUBE) as u64;
    let mut bci: vk::VkBufferCreateInfo = unsafe { std::mem::zeroed() };
    bci.sType = vk::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
    bci.size = vb_size;
    bci.usage = vk::VK_BUFFER_USAGE_VERTEX_BUFFER_BIT;
    bci.sharingMode = vk::VK_SHARING_MODE_EXCLUSIVE;
    let mut vertex_buffer = vk::VkBuffer(0);
    unsafe { vkc.CreateBuffer(device, &bci, ptr::null(), &mut vertex_buffer) };
    let mut reqs: vk::VkMemoryRequirements = unsafe { std::mem::zeroed() };
    unsafe { vkc.GetBufferMemoryRequirements(device, vertex_buffer, &mut reqs) };
    let mut mai: vk::VkMemoryAllocateInfo = unsafe { std::mem::zeroed() };
    mai.sType = vk::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
    mai.allocationSize = reqs.size;
    mai.memoryTypeIndex = find_memory_type(
        &memory_props,
        reqs.memoryTypeBits,
        vk::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | vk::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT,
    );
    let mut vertex_memory = vk::VkDeviceMemory(0);
    unsafe {
        vkc.AllocateMemory(device, &mai, ptr::null(), &mut vertex_memory);
        vkc.BindBufferMemory(device, vertex_buffer, vertex_memory, 0);
        let mut mapped: *mut c_void = ptr::null_mut();
        vkc.MapMemory(device, vertex_memory, 0, vb_size, 0, &mut mapped);
        ptr::copy_nonoverlapping(
            CUBE.as_ptr() as *const u8,
            mapped as *mut u8,
            vb_size as usize,
        );
        vkc.UnmapMemory(device, vertex_memory);
    }

    // ---- Command pool + per-frame objects -----------------------------------
    let mut cpci: vk::VkCommandPoolCreateInfo = unsafe { std::mem::zeroed() };
    cpci.sType = vk::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO;
    cpci.flags = vk::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT;
    cpci.queueFamilyIndex = queue_family;
    let mut command_pool = vk::VkCommandPool(0);
    unsafe { vkc.CreateCommandPool(device, &cpci, ptr::null(), &mut command_pool) };

    let mut cbai: vk::VkCommandBufferAllocateInfo = unsafe { std::mem::zeroed() };
    cbai.sType = vk::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO;
    cbai.commandPool = command_pool;
    cbai.level = vk::VK_COMMAND_BUFFER_LEVEL_PRIMARY;
    cbai.commandBufferCount = FRAMES_IN_FLIGHT as u32;
    let mut cmds = [ptr::null_mut(); FRAMES_IN_FLIGHT].map(vk::VkCommandBuffer);
    unsafe { vkc.AllocateCommandBuffers(device, &cbai, cmds.as_mut_ptr()) };

    let frames = cmds.map(|cmd| {
        let mut sem_ci: vk::VkSemaphoreCreateInfo = unsafe { std::mem::zeroed() };
        sem_ci.sType = vk::VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO;
        let mut fci: vk::VkFenceCreateInfo = unsafe { std::mem::zeroed() };
        fci.sType = vk::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO;
        fci.flags = vk::VK_FENCE_CREATE_SIGNALED_BIT; // first wait passes
        let mut acquire = vk::VkSemaphore(0);
        let mut fence = vk::VkFence(0);
        unsafe {
            vkc.CreateSemaphore(device, &sem_ci, ptr::null(), &mut acquire);
            vkc.CreateFence(device, &fci, ptr::null(), &mut fence);
        }
        Frame {
            cmd,
            acquire,
            fence,
        }
    });

    let mut ctx = Ctx {
        vk: vkc,
        instance,
        surface,
        pd,
        device,
        queue,
        surface_format,
        memory_props,
        pipeline_layout,
        pipeline,
        vertex_buffer,
        vertex_memory,
        command_pool,
        frames,
        frame_index: 0,
        swap: None,
        ci,
        angle: 0.6,
        window,
    };
    unsafe { ctx.create_swapchain() };
    Ok(ctx)
}

/// Create the window surface through the generated loader's platform
/// commands, selected from the winit handle.
unsafe fn create_surface(
    vkc: &vk::Vk,
    instance: vk::VkInstance,
    window: &Window,
) -> vk::VkSurfaceKHR {
    let display = window.display_handle().expect("display handle").as_raw();
    let handle = window.window_handle().expect("window handle").as_raw();
    let mut surface = vk::VkSurfaceKHR(0);
    let r = match (display, handle) {
        (RawDisplayHandle::Windows(_), RawWindowHandle::Win32(w)) => {
            let mut ci: vk::VkWin32SurfaceCreateInfoKHR = unsafe { std::mem::zeroed() };
            ci.sType = vk::VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR;
            ci.hinstance = w
                .hinstance
                .map_or(ptr::null_mut(), |h| h.get() as vk::HINSTANCE);
            ci.hwnd = w.hwnd.get() as vk::HWND;
            unsafe { vkc.CreateWin32SurfaceKHR(instance, &ci, ptr::null(), &mut surface) }
        }
        (RawDisplayHandle::Xlib(d), RawWindowHandle::Xlib(w)) => {
            let mut ci: vk::VkXlibSurfaceCreateInfoKHR = unsafe { std::mem::zeroed() };
            ci.sType = vk::VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR;
            ci.dpy = d.display.map_or(ptr::null_mut(), |p| p.as_ptr()) as *mut vk::Display;
            ci.window = w.window as vk::Window;
            unsafe { vkc.CreateXlibSurfaceKHR(instance, &ci, ptr::null(), &mut surface) }
        }
        (RawDisplayHandle::Wayland(d), RawWindowHandle::Wayland(w)) => {
            let mut ci: vk::VkWaylandSurfaceCreateInfoKHR = unsafe { std::mem::zeroed() };
            ci.sType = vk::VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR;
            ci.display = d.display.as_ptr() as *mut vk::wl_display;
            ci.surface = w.surface.as_ptr() as *mut vk::wl_surface;
            unsafe { vkc.CreateWaylandSurfaceKHR(instance, &ci, ptr::null(), &mut surface) }
        }
        other => panic!("unsupported window system: {other:?}"),
    };
    assert!(r == vk::VK_SUCCESS, "surface creation failed ({})", r.0);
    surface
}

/// Build the pipeline layout (one vertex-stage push-constant range holding
/// the MVP) and the graphics pipeline, targeting dynamic rendering (no
/// render pass — the attachment formats ride in the pNext chain).
unsafe fn create_pipeline(
    vkc: &vk::Vk,
    device: vk::VkDevice,
    color_format: vk::VkFormat,
) -> (vk::VkPipelineLayout, vk::VkPipeline) {
    let vert_words = spirv_words(VERT_SPV);
    let frag_words = spirv_words(FRAG_SPV);

    let make_module = |words: &[u32]| -> vk::VkShaderModule {
        let mut ci: vk::VkShaderModuleCreateInfo = unsafe { std::mem::zeroed() };
        ci.sType = vk::VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO;
        ci.codeSize = words.len() * 4;
        ci.pCode = words.as_ptr();
        let mut module = vk::VkShaderModule(0);
        let r = unsafe { vkc.CreateShaderModule(device, &ci, ptr::null(), &mut module) };
        assert!(r == vk::VK_SUCCESS, "vkCreateShaderModule failed ({})", r.0);
        module
    };
    let vert = make_module(&vert_words);
    let frag = make_module(&frag_words);

    let mut pc_range: vk::VkPushConstantRange = unsafe { std::mem::zeroed() };
    pc_range.stageFlags = vk::VK_SHADER_STAGE_VERTEX_BIT;
    pc_range.size = std::mem::size_of::<Mat4>() as u32;
    let mut plci: vk::VkPipelineLayoutCreateInfo = unsafe { std::mem::zeroed() };
    plci.sType = vk::VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO;
    plci.pushConstantRangeCount = 1;
    plci.pPushConstantRanges = &pc_range;
    let mut layout = vk::VkPipelineLayout(0);
    unsafe { vkc.CreatePipelineLayout(device, &plci, ptr::null(), &mut layout) };

    let mut stages: [vk::VkPipelineShaderStageCreateInfo; 2] = unsafe { std::mem::zeroed() };
    for (stage, (module, flag)) in stages.iter_mut().zip([
        (vert, vk::VK_SHADER_STAGE_VERTEX_BIT),
        (frag, vk::VK_SHADER_STAGE_FRAGMENT_BIT),
    ]) {
        stage.sType = vk::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
        stage.stage = flag;
        stage.module = module;
        stage.pName = c"main".as_ptr();
    }

    let mut binding: vk::VkVertexInputBindingDescription = unsafe { std::mem::zeroed() };
    binding.stride = 6 * 4;
    binding.inputRate = vk::VK_VERTEX_INPUT_RATE_VERTEX;
    let mut attrs: [vk::VkVertexInputAttributeDescription; 2] = unsafe { std::mem::zeroed() };
    attrs[0].location = 0;
    attrs[0].format = vk::VK_FORMAT_R32G32B32_SFLOAT;
    attrs[1].location = 1;
    attrs[1].format = vk::VK_FORMAT_R32G32B32_SFLOAT;
    attrs[1].offset = 3 * 4;
    let mut vi: vk::VkPipelineVertexInputStateCreateInfo = unsafe { std::mem::zeroed() };
    vi.sType = vk::VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO;
    vi.vertexBindingDescriptionCount = 1;
    vi.pVertexBindingDescriptions = &binding;
    vi.vertexAttributeDescriptionCount = 2;
    vi.pVertexAttributeDescriptions = attrs.as_ptr();

    let mut ia: vk::VkPipelineInputAssemblyStateCreateInfo = unsafe { std::mem::zeroed() };
    ia.sType = vk::VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO;
    ia.topology = vk::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST;

    let mut vp: vk::VkPipelineViewportStateCreateInfo = unsafe { std::mem::zeroed() };
    vp.sType = vk::VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO;
    vp.viewportCount = 1;
    vp.scissorCount = 1;

    let mut rs: vk::VkPipelineRasterizationStateCreateInfo = unsafe { std::mem::zeroed() };
    rs.sType = vk::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO;
    rs.polygonMode = vk::VK_POLYGON_MODE_FILL;
    rs.cullMode = vk::VK_CULL_MODE_BACK_BIT;
    rs.frontFace = vk::VK_FRONT_FACE_COUNTER_CLOCKWISE;
    rs.lineWidth = 1.0;

    let mut ms: vk::VkPipelineMultisampleStateCreateInfo = unsafe { std::mem::zeroed() };
    ms.sType = vk::VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO;
    ms.rasterizationSamples = vk::VK_SAMPLE_COUNT_1_BIT;

    let mut ds: vk::VkPipelineDepthStencilStateCreateInfo = unsafe { std::mem::zeroed() };
    ds.sType = vk::VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO;
    ds.depthTestEnable = vk::VK_TRUE as vk::VkBool32;
    ds.depthWriteEnable = vk::VK_TRUE as vk::VkBool32;
    ds.depthCompareOp = vk::VK_COMPARE_OP_LESS;

    let mut blend_att: vk::VkPipelineColorBlendAttachmentState = unsafe { std::mem::zeroed() };
    blend_att.colorWriteMask = vk::VK_COLOR_COMPONENT_R_BIT
        | vk::VK_COLOR_COMPONENT_G_BIT
        | vk::VK_COLOR_COMPONENT_B_BIT
        | vk::VK_COLOR_COMPONENT_A_BIT;
    let mut blend: vk::VkPipelineColorBlendStateCreateInfo = unsafe { std::mem::zeroed() };
    blend.sType = vk::VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO;
    blend.attachmentCount = 1;
    blend.pAttachments = &blend_att;

    let dyn_states = [vk::VK_DYNAMIC_STATE_VIEWPORT, vk::VK_DYNAMIC_STATE_SCISSOR];
    let mut dynamic: vk::VkPipelineDynamicStateCreateInfo = unsafe { std::mem::zeroed() };
    dynamic.sType = vk::VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO;
    dynamic.dynamicStateCount = dyn_states.len() as u32;
    dynamic.pDynamicStates = dyn_states.as_ptr();

    let mut rendering: vk::VkPipelineRenderingCreateInfo = unsafe { std::mem::zeroed() };
    rendering.sType = vk::VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO;
    rendering.colorAttachmentCount = 1;
    rendering.pColorAttachmentFormats = &color_format;
    rendering.depthAttachmentFormat = DEPTH_FORMAT;

    let mut gpci: vk::VkGraphicsPipelineCreateInfo = unsafe { std::mem::zeroed() };
    gpci.sType = vk::VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO;
    gpci.pNext = &rendering as *const _ as *const c_void;
    gpci.stageCount = stages.len() as u32;
    gpci.pStages = stages.as_ptr();
    gpci.pVertexInputState = &vi;
    gpci.pInputAssemblyState = &ia;
    gpci.pViewportState = &vp;
    gpci.pRasterizationState = &rs;
    gpci.pMultisampleState = &ms;
    gpci.pDepthStencilState = &ds;
    gpci.pColorBlendState = &blend;
    gpci.pDynamicState = &dynamic;
    gpci.layout = layout;
    let mut pipeline = vk::VkPipeline(0);
    let r = unsafe {
        vkc.CreateGraphicsPipelines(
            device,
            vk::VkPipelineCache(0),
            1,
            &gpci,
            ptr::null(),
            &mut pipeline,
        )
    };
    assert!(
        r == vk::VK_SUCCESS,
        "vkCreateGraphicsPipelines failed ({})",
        r.0
    );

    unsafe {
        vkc.DestroyShaderModule(device, vert, ptr::null());
        vkc.DestroyShaderModule(device, frag, ptr::null());
    }
    (layout, pipeline)
}

fn main() -> ExitCode {
    let ci = std::env::args().any(|a| a == "--ci");
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App { ci, ctx: None };
    event_loop.run_app(&mut app).unwrap();
    ExitCode::SUCCESS
}
