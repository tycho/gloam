//! vk-info-rs — exercise the gloam-generated Rust Vulkan loader end to end.
//!
//! Mirrors examples/c/vk-info: phased loading against the `--mx-global`
//! process-global context (initialize_global → load_instance_global →
//! load_device_global) with free-function dispatch, then a summary of what
//! the loader detected.  (The owned `Vk` context API is always generated
//! too; this example exercises the global layer, like the C example's
//! `gloam_vk_context`.)  Headless — no
//! window or surface is created; VK_KHR_surface/VK_KHR_swapchain are enabled
//! when advertised purely to exercise extension-flag plumbing.
//!
//! Exit codes: 0 = pass, 1 = Vulkan present but something failed,
//! 77 = no usable Vulkan on this machine (skip).

use std::ffi::CStr;
use std::process::ExitCode;
use std::ptr;

use gloam_vk as vk;

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

fn open_vulkan() -> Option<libloading::Library> {
    LIB_NAMES
        .iter()
        .find_map(|name| unsafe { libloading::Library::new(name).ok() })
}

fn version_string(v: u32) -> String {
    format!(
        "{}.{}.{}",
        vk::VK_API_VERSION_MAJOR(v),
        vk::VK_API_VERSION_MINOR(v),
        vk::VK_API_VERSION_PATCH(v)
    )
}

fn contains(names: &[vk::VkExtensionProperties], wanted: &CStr) -> bool {
    names
        .iter()
        .any(|p| unsafe { CStr::from_ptr(p.extensionName.as_ptr()) } == wanted)
}

fn flag(label: &str, value: bool) {
    println!("  {label:<28} {}", if value { "yes" } else { "no" });
}

fn main() -> ExitCode {
    // ---- Phase 0: bootstrap vkGetInstanceProcAddr -------------------------
    let Some(lib) = open_vulkan() else {
        eprintln!("vk-info-rs: no Vulkan runtime found (skip)");
        return ExitCode::from(77);
    };
    let gipa: vk::PFN_vkGetInstanceProcAddr = unsafe {
        match lib.get::<vk::PFN_vkGetInstanceProcAddr>(b"vkGetInstanceProcAddr\0") {
            Ok(sym) => *sym,
            Err(_) => {
                eprintln!("vk-info-rs: vkGetInstanceProcAddr missing (skip)");
                return ExitCode::from(77);
            }
        }
    };

    unsafe { vk::initialize_global(gipa) };

    // Global-scope dispatch: instance version + instance extensions.
    let mut instance_version: u32 = vk::VK_API_VERSION_1_0;
    unsafe { vk::EnumerateInstanceVersion(&mut instance_version) };
    println!("instance version:  {}", version_string(instance_version));

    let mut n: u32 = 0;
    unsafe { vk::EnumerateInstanceExtensionProperties(ptr::null(), &mut n, ptr::null_mut()) };
    let mut inst_exts: Vec<vk::VkExtensionProperties> =
        vec![unsafe { std::mem::zeroed() }; n as usize];
    unsafe {
        vk::EnumerateInstanceExtensionProperties(ptr::null(), &mut n, inst_exts.as_mut_ptr())
    };
    println!("instance extensions advertised: {n}");

    // ---- Create an instance ------------------------------------------------
    let mut app_info: vk::VkApplicationInfo = unsafe { std::mem::zeroed() };
    app_info.sType = vk::VK_STRUCTURE_TYPE_APPLICATION_INFO;
    app_info.pApplicationName = c"vk-info-rs".as_ptr();
    app_info.apiVersion = vk::VK_API_VERSION_1_3;

    let mut enabled_instance_exts: Vec<&CStr> = Vec::new();
    if contains(&inst_exts, c"VK_KHR_surface") {
        enabled_instance_exts.push(c"VK_KHR_surface");
    }
    if contains(&inst_exts, c"VK_EXT_debug_utils") {
        enabled_instance_exts.push(c"VK_EXT_debug_utils");
    }
    // When the loader offers portability enumeration (MoltenVK via the
    // Khronos loader), opt in so portability-subset devices are visible.
    let mut instance_flags: vk::VkInstanceCreateFlags = 0;
    if contains(&inst_exts, vk::VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME) {
        enabled_instance_exts.push(vk::VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME);
        instance_flags |= vk::VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR;
    }
    let ext_ptrs: Vec<*const std::ffi::c_char> =
        enabled_instance_exts.iter().map(|e| e.as_ptr()).collect();

    let mut create_info: vk::VkInstanceCreateInfo = unsafe { std::mem::zeroed() };
    create_info.sType = vk::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
    create_info.flags = instance_flags;
    create_info.pApplicationInfo = &app_info;
    create_info.enabledExtensionCount = ext_ptrs.len() as u32;
    create_info.ppEnabledExtensionNames = ext_ptrs.as_ptr();

    let mut instance = vk::VkInstance(ptr::null_mut());
    let r = unsafe { vk::CreateInstance(&create_info, ptr::null(), &mut instance) };
    if r != vk::VK_SUCCESS {
        eprintln!("vk-info-rs: vkCreateInstance failed ({})", r.0);
        return ExitCode::from(1);
    }

    // ---- Phase 1: instance-scope loading -----------------------------------
    if !unsafe { vk::load_instance_global(instance, vk::VK_API_VERSION_1_3, &enabled_instance_exts) } {
        eprintln!("vk-info-rs: load_instance failed");
        return ExitCode::from(1);
    }

    // ---- Pick a physical device -------------------------------------------
    let mut count: u32 = 0;
    unsafe { vk::EnumeratePhysicalDevices(instance, &mut count, ptr::null_mut()) };
    if count == 0 {
        eprintln!("vk-info-rs: no physical devices (skip)");
        return ExitCode::from(77);
    }
    let mut devices = vec![vk::VkPhysicalDevice(ptr::null_mut()); count as usize];
    unsafe { vk::EnumeratePhysicalDevices(instance, &mut count, devices.as_mut_ptr()) };
    let pd = devices[0];

    let mut props: vk::VkPhysicalDeviceProperties = unsafe { std::mem::zeroed() };
    unsafe { vk::GetPhysicalDeviceProperties(pd, &mut props) };
    let name = unsafe { CStr::from_ptr(props.deviceName.as_ptr()) };
    println!("device:            {}", name.to_string_lossy());
    println!("device api:        {}", version_string(props.apiVersion));

    let mut dev_ext_count: u32 = 0;
    unsafe {
        vk::EnumerateDeviceExtensionProperties(pd, ptr::null(), &mut dev_ext_count, ptr::null_mut())
    };
    let mut dev_exts: Vec<vk::VkExtensionProperties> =
        vec![unsafe { std::mem::zeroed() }; dev_ext_count as usize];
    unsafe {
        vk::EnumerateDeviceExtensionProperties(
            pd,
            ptr::null(),
            &mut dev_ext_count,
            dev_exts.as_mut_ptr(),
        )
    };
    println!("device extensions advertised:   {dev_ext_count}");

    // ---- Create a device ----------------------------------------------------
    let mut qf_count: u32 = 0;
    unsafe { vk::GetPhysicalDeviceQueueFamilyProperties(pd, &mut qf_count, ptr::null_mut()) };
    if qf_count == 0 {
        eprintln!("vk-info-rs: no queue families");
        return ExitCode::from(1);
    }

    let mut enabled_device_exts: Vec<&CStr> = Vec::new();
    if enabled_instance_exts.contains(&c"VK_KHR_surface") && contains(&dev_exts, c"VK_KHR_swapchain")
    {
        enabled_device_exts.push(c"VK_KHR_swapchain");
    }
    if contains(&dev_exts, c"VK_KHR_timeline_semaphore") {
        enabled_device_exts.push(c"VK_KHR_timeline_semaphore");
    }
    if contains(&dev_exts, c"VK_KHR_synchronization2") {
        enabled_device_exts.push(c"VK_KHR_synchronization2");
    }
    let dev_ext_ptrs: Vec<*const std::ffi::c_char> =
        enabled_device_exts.iter().map(|e| e.as_ptr()).collect();

    let priority = 1.0f32;
    let mut queue_info: vk::VkDeviceQueueCreateInfo = unsafe { std::mem::zeroed() };
    queue_info.sType = vk::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
    queue_info.queueFamilyIndex = 0;
    queue_info.queueCount = 1;
    queue_info.pQueuePriorities = &priority;

    let mut device_info: vk::VkDeviceCreateInfo = unsafe { std::mem::zeroed() };
    device_info.sType = vk::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
    device_info.queueCreateInfoCount = 1;
    device_info.pQueueCreateInfos = &queue_info;
    device_info.enabledExtensionCount = dev_ext_ptrs.len() as u32;
    device_info.ppEnabledExtensionNames = dev_ext_ptrs.as_ptr();

    let mut device = vk::VkDevice(ptr::null_mut());
    let r = unsafe { vk::CreateDevice(pd, &device_info, ptr::null(), &mut device) };
    if r != vk::VK_SUCCESS {
        eprintln!("vk-info-rs: vkCreateDevice failed ({})", r.0);
        return ExitCode::from(1);
    }

    // ---- Phase 2: device-scope loading --------------------------------------
    if !unsafe { vk::load_device_global(device, pd, &enabled_device_exts) } {
        eprintln!("vk-info-rs: load_device failed");
        return ExitCode::from(1);
    }

    // Device-scope dispatch proof: vkGetDeviceQueue resolves through
    // vkGetDeviceProcAddr after load_device.
    let mut queue = vk::VkQueue(ptr::null_mut());
    unsafe { vk::GetDeviceQueue(device, 0, 0, &mut queue) };
    if queue.0.is_null() {
        eprintln!("vk-info-rs: vkGetDeviceQueue returned null");
        return ExitCode::from(1);
    }
    println!("device queue:      obtained (device-scope dispatch OK)");

    // ---- Loader state summary ----------------------------------------------
    println!(
        "loader version:    {}.{} (packed 0x{:04x})",
        vk::version() >> 8,
        vk::version() & 0xff,
        vk::version()
    );
    println!("features:");
    flag("VK_VERSION_1_0", vk::VERSION_1_0());
    flag("VK_VERSION_1_1", vk::VERSION_1_1());
    flag("VK_VERSION_1_2", vk::VERSION_1_2());
    flag("VK_VERSION_1_3", vk::VERSION_1_3());
    println!("extensions enabled by this run:");
    flag("VK_KHR_surface", vk::KHR_surface());
    flag("VK_KHR_swapchain", vk::KHR_swapchain());
    flag("VK_EXT_debug_utils", vk::EXT_debug_utils());
    flag("VK_KHR_timeline_semaphore", vk::KHR_timeline_semaphore());
    flag("VK_KHR_synchronization2", vk::KHR_synchronization2());
    flag(
        "VK_KHR_portability_enumeration",
        vk::KHR_portability_enumeration(),
    );

    unsafe {
        vk::DestroyDevice(device, ptr::null());
        vk::DestroyInstance(instance, ptr::null());
    }
    println!("vk-info-rs: PASS");
    ExitCode::SUCCESS
}
