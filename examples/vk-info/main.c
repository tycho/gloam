/* vk-info — headless Vulkan device info via a gloam-generated loader.
 *
 * Demonstrates the phased Vulkan loading flow (Volk-like: the application
 * owns extension discovery and tells gloam what it enabled):
 *
 *   gloamVulkanInitialize   open the platform Vulkan library (--loader)
 *   vkCreateInstance        created with the known extensions the driver has
 *   gloamVulkanLoadInstance load instance-scope PFNs, set extension flags
 *   vkCreateDevice          same dance for device extensions
 *   gloamVulkanLoadDevice   load device-scope PFNs, set extension flags
 *
 * As it goes it cross-checks gloam's per-extension context flags against the
 * ground truth it enabled, and prints the comparison table.
 *
 * Exit codes: 0 = pass, 1 = failure, 77 = skipped (no Vulkan runtime/device).
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <gloam/vk.h>

#define EXIT_SKIP 77

/* The extensions this loader was generated with (see gloam/.gloam/manifest.json). */
typedef struct {
    const char *name;
    const unsigned char *flag; /* gloam context member for this extension */
    int device_scope;          /* 0 = instance extension, 1 = device extension */
} KnownExt;

static const KnownExt kKnownExts[] = {
    { "VK_KHR_get_physical_device_properties2", &GLOAM_VK_KHR_get_physical_device_properties2, 0 },
    { "VK_EXT_debug_utils",                     &GLOAM_VK_EXT_debug_utils,                     0 },
    { "VK_KHR_swapchain",                       &GLOAM_VK_KHR_swapchain,                       1 },
    { "VK_KHR_timeline_semaphore",              &GLOAM_VK_KHR_timeline_semaphore,              1 },
    { "VK_KHR_synchronization2",                &GLOAM_VK_KHR_synchronization2,                1 },
};
#define NUM_KNOWN (sizeof(kKnownExts) / sizeof(kKnownExts[0]))

static int list_contains(const VkExtensionProperties *props, uint32_t n, const char *name)
{
    uint32_t i;
    for (i = 0; i < n; ++i)
        if (strcmp(props[i].extensionName, name) == 0)
            return 1;
    return 0;
}

/* Compare gloam's flags against what we enabled; print the table. */
static int check_flags(const VkExtensionProperties *avail, uint32_t num_avail,
                       const char *const *enabled, uint32_t num_enabled,
                       int device_scope, const char *scope_name)
{
    int mismatches = 0;
    uint32_t i, j;

    printf("\n%-42s %-7s %-8s %s\n", scope_name, "driver", "enabled", "gloam");
    for (i = 0; i < NUM_KNOWN; ++i) {
        const KnownExt *e = &kKnownExts[i];
        int in_driver, want, got;

        if (e->device_scope != device_scope)
            continue;

        in_driver = list_contains(avail, num_avail, e->name);
        want = 0;
        for (j = 0; j < num_enabled; ++j)
            if (strcmp(enabled[j], e->name) == 0)
                want = 1;
        got = *e->flag != 0;

        printf("%-42s %-7s %-8s %-5s %s\n", e->name,
               in_driver ? "yes" : "no",
               want ? "yes" : "no",
               got ? "yes" : "no",
               want == got ? "OK" : "MISMATCH");
        if (want != got)
            ++mismatches;
    }
    return mismatches;
}

int main(void)
{
    uint32_t instance_version, api_version;
    uint32_t num_inst_props = 0, num_dev_props = 0, num_devices = 0, num_queue_families = 0;
    VkExtensionProperties *inst_props = NULL, *dev_props = NULL;
    const char *enabled_inst[NUM_KNOWN], *enabled_dev[NUM_KNOWN];
    uint32_t num_enabled_inst = 0, num_enabled_dev = 0;
    VkInstance instance = VK_NULL_HANDLE;
    VkPhysicalDevice physical_device = VK_NULL_HANDLE;
    VkPhysicalDeviceProperties props;
    VkDevice device = VK_NULL_HANDLE;
    VkQueue queue = VK_NULL_HANDLE;
    VkResult res;
    uint32_t i;
    int mismatches = 0;

    /* Phase 0: open the platform Vulkan library (vulkan-1.dll /
     * libvulkan.so.1 / libvulkan.dylib) and load the global-scope PFNs. */
    if (!gloamVulkanInitialize(NULL)) {
        fprintf(stderr, "vk-info: no Vulkan runtime available, skipping\n");
        return EXIT_SKIP;
    }

    instance_version = gloamVulkanGetInstanceVersion();
    printf("Vulkan instance version: %u.%u.%u\n",
           VK_API_VERSION_MAJOR(instance_version),
           VK_API_VERSION_MINOR(instance_version),
           VK_API_VERSION_PATCH(instance_version));

    /* Ground truth: what the driver actually offers at instance scope. */
    vkEnumerateInstanceExtensionProperties(NULL, &num_inst_props, NULL);
    inst_props = (VkExtensionProperties *)calloc(num_inst_props ? num_inst_props : 1, sizeof(*inst_props));
    vkEnumerateInstanceExtensionProperties(NULL, &num_inst_props, inst_props);

    /* Enable the known instance extensions the driver has. */
    for (i = 0; i < NUM_KNOWN; ++i)
        if (!kKnownExts[i].device_scope && list_contains(inst_props, num_inst_props, kKnownExts[i].name))
            enabled_inst[num_enabled_inst++] = kKnownExts[i].name;

    /* Don't request an API version newer than this loader was generated for. */
    api_version = instance_version < VK_API_VERSION_1_3 ? instance_version : VK_API_VERSION_1_3;

    {
        VkApplicationInfo app = { .sType = VK_STRUCTURE_TYPE_APPLICATION_INFO };
        VkInstanceCreateInfo ci = { .sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO };
        app.pApplicationName = "gloam vk-info";
        app.apiVersion = api_version;
        ci.pApplicationInfo = &app;
        ci.enabledExtensionCount = num_enabled_inst;
        ci.ppEnabledExtensionNames = enabled_inst;
        res = vkCreateInstance(&ci, NULL, &instance);
    }
    if (res == VK_ERROR_INCOMPATIBLE_DRIVER) {
        fprintf(stderr, "vk-info: no compatible Vulkan driver, skipping\n");
        free(inst_props);
        return EXIT_SKIP;
    }
    if (res != VK_SUCCESS) {
        fprintf(stderr, "vk-info: vkCreateInstance failed (%d)\n", (int)res);
        free(inst_props);
        return 1;
    }

    /* Phase 1: instance-scope PFNs + extension flags for what we enabled. */
    if (!gloamVulkanLoadInstance(instance, api_version, num_enabled_inst, enabled_inst)) {
        fprintf(stderr, "vk-info: gloamVulkanLoadInstance failed\n");
        return 1;
    }

    mismatches += check_flags(inst_props, num_inst_props, enabled_inst, num_enabled_inst,
                              0, "instance extension");

    vkEnumeratePhysicalDevices(instance, &num_devices, NULL);
    if (num_devices == 0) {
        fprintf(stderr, "vk-info: no Vulkan devices, skipping\n");
        vkDestroyInstance(instance, NULL);
        gloamVulkanFinalize();
        free(inst_props);
        return EXIT_SKIP;
    }
    num_devices = 1; /* first device is enough for the demo */
    vkEnumeratePhysicalDevices(instance, &num_devices, &physical_device);

    vkGetPhysicalDeviceProperties(physical_device, &props);
    printf("\nDevice: %s\n", props.deviceName);
    printf("  API version:    %u.%u.%u\n",
           VK_API_VERSION_MAJOR(props.apiVersion),
           VK_API_VERSION_MINOR(props.apiVersion),
           VK_API_VERSION_PATCH(props.apiVersion));
    printf("  Driver version: 0x%08x\n", props.driverVersion);

    /* Ground truth at device scope. */
    vkEnumerateDeviceExtensionProperties(physical_device, NULL, &num_dev_props, NULL);
    dev_props = (VkExtensionProperties *)calloc(num_dev_props ? num_dev_props : 1, sizeof(*dev_props));
    vkEnumerateDeviceExtensionProperties(physical_device, NULL, &num_dev_props, dev_props);
    printf("  Device extensions reported by driver: %u\n", num_dev_props);

    for (i = 0; i < NUM_KNOWN; ++i)
        if (kKnownExts[i].device_scope && list_contains(dev_props, num_dev_props, kKnownExts[i].name))
            enabled_dev[num_enabled_dev++] = kKnownExts[i].name;

    {
        float prio = 1.0f;
        VkDeviceQueueCreateInfo qci = { .sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO };
        VkDeviceCreateInfo dci = { .sType = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO };

        vkGetPhysicalDeviceQueueFamilyProperties(physical_device, &num_queue_families, NULL);
        if (num_queue_families == 0) {
            fprintf(stderr, "vk-info: device has no queue families\n");
            return 1;
        }
        qci.queueFamilyIndex = 0;
        qci.queueCount = 1;
        qci.pQueuePriorities = &prio;
        dci.queueCreateInfoCount = 1;
        dci.pQueueCreateInfos = &qci;
        dci.enabledExtensionCount = num_enabled_dev;
        dci.ppEnabledExtensionNames = enabled_dev;
        res = vkCreateDevice(physical_device, &dci, NULL, &device);
    }
    if (res != VK_SUCCESS) {
        fprintf(stderr, "vk-info: vkCreateDevice failed (%d)\n", (int)res);
        return 1;
    }

    /* Phase 2: device-scope PFNs + extension flags for what we enabled. */
    if (!gloamVulkanLoadDevice(device, physical_device, num_enabled_dev, enabled_dev)) {
        fprintf(stderr, "vk-info: gloamVulkanLoadDevice failed\n");
        return 1;
    }

    mismatches += check_flags(dev_props, num_dev_props, enabled_dev, num_enabled_dev,
                              1, "device extension");

    /* Smoke-test a device-scope dispatch through the loaded table. */
    vkGetDeviceQueue(device, 0, 0, &queue);
    if (queue == VK_NULL_HANDLE) {
        fprintf(stderr, "vk-info: vkGetDeviceQueue returned no queue\n");
        return 1;
    }

    vkDestroyDevice(device, NULL);
    vkDestroyInstance(instance, NULL);
    gloamVulkanFinalize();
    free(inst_props);
    free(dev_props);

    if (mismatches != 0) {
        fprintf(stderr, "\nvk-info: FAIL — %d extension flag mismatch(es)\n", mismatches);
        return 1;
    }
    printf("\nvk-info: PASS — gloam extension flags match the enabled sets\n");
    return 0;
}
