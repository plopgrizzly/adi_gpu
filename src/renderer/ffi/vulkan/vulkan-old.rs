// Willow Graphics API
//
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE
//
// src/renderer/ffi/vulkan/vulkan-old.rs

use std::ffi::{CString};
use std;
use std::ptr::{null, null_mut};
use std::ptr::copy_nonoverlapping as memcpy;

type VkPhysicalDevice = usize;
type VkDevice = usize;
type VkQueue = usize;
type VkCommandBuffer = usize;
type VkSemaphore = u64;
type VkImage = u64;
type VkFence = u64;
type VkShaderModule = u64;
type VkPipelineCache = u64;
type VkRenderPass = u64;
type VkPipelineLayout = u64;
type VkDescriptorSetLayout = u64;
type VkBuffer = u64;
type VkSampler = u64;
type VkCommandPool = u64;
// VkSurface create_surface.rs
type VkSwapchain = u64;
type VkImageView = u64;
type VkFunction = unsafe extern "C" fn() -> ();
type VkColorSpace = u32;
type VkFlags = u32;

enum VkVoid {}
enum VkAllocationCallbacks {}
enum ANativeWindow {}

// vulkan.rs

// create_instance.rs

// select_gpu.rs

#[repr(C)]
struct VkExtent2D {
	width: u32,
	height: u32,
}

// select_gpu.rs

// create_surface.rs

// create_gpu_interface.rs

#[repr(C)]
#[derive(PartialEq, Copy, Clone)]
enum VkFormat {
	VK_FORMAT_D16_UNORM = 124,
	VK_FORMAT_R8G8B8A8_UNORM = 37,
	VK_FORMAT_UNDEFINED = 0,
	VK_FORMAT_B8G8R8A8_UNORM = 44,
}

#[repr(C)]
enum VkSharingMode {
	Exclusive = 0,
}

#[repr(C)]
enum VkImageLayout {
	VK_IMAGE_LAYOUT_UNDEFINED = 0,
	VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL = 2,
	VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL = 3,
	VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL = 5,
	VK_IMAGE_LAYOUT_PREINITIALIZED = 8,
}

#[repr(C)]
enum VkImageTiling {
	VK_IMAGE_TILING_OPTIMAL = 0,
	VK_IMAGE_TILING_LINEAR = 1,
}

#[repr(C)]
enum VkImageType {
	VK_IMAGE_TYPE_2D = 1,
}

#[repr(C)]
#[derive(PartialEq, Copy, Clone)]
enum VkPresentMode {
	Immediate = 0,
	Mailbox = 1,
	Fifo = 2,
	FifoRelaxed = 3,
}

#[repr(C)]
enum VkComponentSwizzle {
	Identity = 0,
	Zero = 1,
	One = 2,
	R = 3,
	G = 4,
	B = 5,
	A = 6,
}

#[repr(C)]
enum VkImageViewType {
	Vk1d = 0,
	Vk2d = 1,
	Vk3d = 2,
	VkCube = 3,
	Vk1dArray = 4,
	Vk2dArray = 5,
	VkCubeArray = 6,
}

#[repr(C)]
enum VkFilter {
	VK_FILTER_NEAREST = 0,
	VK_FILTER_LINEAR = 1,
}

#[repr(C)]
enum VkSamplerMipmapMode {
	VK_SAMPLER_MIPMAP_MODE_NEAREST = 0,
	VK_SAMPLER_MIPMAP_MODE_LINEAR = 1,
}

#[repr(C)]
enum VkSamplerAddressMode {
	VK_SAMPLER_ADDRESS_MODE_REPEAT = 0,
	VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT = 1,
	VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE = 2,
	VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER = 3,
	VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE = 4,
}

#[repr(C)]
enum VkBorderColor {
	VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK = 0,
	VK_BORDER_COLOR_INT_TRANSPARENT_BLACK = 1,
	VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK = 2,
	VK_BORDER_COLOR_INT_OPAQUE_BLACK = 3,
	VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE = 4,
	VK_BORDER_COLOR_INT_OPAQUE_WHITE = 5,
}

#[repr(C)]
enum VkCompareOp {
	Never = 0,
}

#[repr(C)]
enum VkShaderStageFlags {
	VertexBit = 0x00000001,
	FragmentBit = 0x00000010,
}

#[repr(C)]
enum VkDescriptorType {
	CombinedImageSampler = 1,
	UniformBuffer = 6,
}

#[repr(C)]
struct VkSwapchainCreateInfo {
	sType: VkStructureType,
	pNext: *const VkVoid,
	flags: VkFlags,
	surface: VkSurface,
	minImageCount: u32,
	imageFormat: VkFormat,
	imageColorSpace: VkColorSpace,
	imageExtent: VkExtent2D,
	imageArrayLayers: u32,
	imageUsage: VkFlags,
	imageSharingMode: VkSharingMode,
	queueFamilyIndexCount: u32,
	pQueueFamilyIndices: *const u32,
	preTransform: VkFlags,
	compositeAlpha: VkFlags,
	presentMode: VkPresentMode,
	clipped: u32,
	oldSwapchain: VkSwapchain,
}

#[repr(C)]
struct VkPresentInfo {
	sType: VkStructureType,
	pNext: *const VkVoid,
	waitSemaphoreCount: u32,
	pWaitSemaphores: *const VkSemaphore,
	swapchainCount: u32,
	pSwapchains: *const VkSwapchain,
	pImageIndices: *const u32,
	pResults: *mut VkResult,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct VkSurfaceFormat {
	format : VkFormat,
	color_space: VkColorSpace,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct VkMemoryType {
	propertyFlags: VkFlags,
	heapIndex: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct VkMemoryHeap {
	size: u64,
	flags: VkFlags,
}

#[repr(C)]
struct VkPhysicalDeviceMemoryProperties {
	memoryTypeCount : u32,
	memoryTypes : [VkMemoryType; 32],
	memoryHeapCount : u32,
	memoryHeaps : [VkMemoryHeap; 16],
}

// create_command_buffer.rs

#[repr(C)]
struct VkSurfaceCapabilities {
	minImageCount: u32,
	maxImageCount: u32,
	currentExtent: VkExtent2D,
	minImageExtent: VkExtent2D,
	maxImageExtent: VkExtent2D,
	maxImageArrayLayers: u32,
	supportedTransforms: VkFlags,
	currentTransform: VkFlags,
	supportedCompositeAlpha: VkFlags,
	supportedUsageFlags: VkFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct VkSwapchainBuffers {
	image: VkImage,
	cmd: VkCommandBuffer,
	view: VkImageView,
}

#[repr(C)]
struct VkComponentMapping {
	r: VkComponentSwizzle,
	g: VkComponentSwizzle,
	b: VkComponentSwizzle,
	a: VkComponentSwizzle,
}

#[repr(C)]
struct VkImageSubresourceRange {
	aspectMask: VkFlags,
	baseMipLevel: u32,
	levelCount: u32,
	baseArrayLayer: u32,
	layerCount: u32,
}

#[repr(C)]
struct VkImageSubresource {
	aspectMask: VkFlags,
	mipLevel: u32,
	arrayLayer: u32,
}

#[repr(C)]
struct VkImageViewCreateInfo {
	sType: VkStructureType,
	pNext: *const VkVoid,
	flags: VkFlags,
	image: VkImage,
	viewType: VkImageViewType,
	format: VkFormat,
	components: VkComponentMapping,
	subresourceRange: VkImageSubresourceRange,
}

#[repr(C)]
struct VkImageCreateInfo {
	sType: VkStructureType,
	pNext: *const VkVoid,
	flags: VkFlags,
	imageType: VkImageType,
	format: VkFormat,
	extent: VkExtent3D,
	mipLevels: u32,
	arrayLayers: u32,
	samples: VkFlags,
	tiling: VkImageTiling,
	usage: VkFlags,
	sharingMode: VkSharingMode,
	queueFamilyIndexCount: u32,
	pQueueFamilyIndices: *const u32,
	initialLayout: VkImageLayout,
}

#[repr(C)]
struct VkMemoryRequirements {
	size: u64,
	alignment: u64,
	memoryTypeBits: u32,
}

#[repr(C)]
struct VkMemoryAllocateInfo {
	sType: VkStructureType,
	pNext: *const VkVoid,
	allocationSize: u64,
	memoryTypeIndex: u32,
}

#[repr(C)]
struct VkSubresourceLayout {
	offset: u64,
	size: u64,
	rowPitch: u64,
	arrayPitch: u64,
	depthPitch: u64,
}

#[repr(C)]
struct VkSamplerCreateInfo {
	sType: VkStructureType,
	pNext: *const VkVoid,
	flags: VkFlags,
	magFilter: VkFilter,
	minFilter: VkFilter,
	mipmapMode: VkSamplerMipmapMode,
	addressModeU: VkSamplerAddressMode,
	addressModeV: VkSamplerAddressMode,
	addressModeW: VkSamplerAddressMode,
	mipLodBias: f32,
	anisotropyEnable: u32, // 1 or 0
	maxAnisotropy: f32,
	compareEnable: u32, // 1 or 0
	compareOp: VkCompareOp,
	minLod: f32,
	maxLod: f32,
	borderColor: VkBorderColor,
	unnormalizedCoordinates: u32, // 1 or 0
}

#[repr(C)]
struct VkBufferCreateInfo {
	sType: VkStructureType,
	pNext: *const VkVoid,
	flags: VkFlags,
	size: u64,
	usage: VkFlags,
	sharingMode: VkSharingMode,
	queueFamilyIndexCount: u32,
	pQueueFamilyIndice: *const u32,
}

#[repr(C)]
struct VkDescriptorSetLayoutBinding {
	binding: u32,
	descriptorType: VkDescriptorType,
	descriptorCount: u32,
	stageFlags: VkShaderStageFlags,
	pImmutableSamplers: *const VkVoid,
}

#[repr(C)]
struct VkDescriptorSetLayoutCreateInfo {
	sType: VkStructureType,
	pNext: *const VkVoid,
	flags: VkFlags,
	bindingCount: u32,
	pBindings: *const VkDescriptorSetLayoutBinding,
}

#[repr(C)]
struct VkPushConstantRange {
	stageFlags: VkShaderStageFlags,
	offset: u32,
	size: u32,
}

#[repr(C)]
struct VkPipelineLayoutCreateInfo {
	sType: VkStructureType,
	pNext: *const VkVoid,
	flags: VkFlags,
	setLayoutCount: u32,
	pSetLayouts: *const VkDescriptorSetLayout,
	pushConstantRangeCount: u32,
	pPushConstantRanges: *const VkPushConstantRange,
}

#[repr(C)]
enum VkAttachmentDescriptionFlagBits {
	MayAliasBit = 0x00000001, // only value
}

#[repr(C)]
enum VkSampleCountFlagBits {
	Bit1 = 0x00000001, // only value
}

#[repr(C)]
enum VkAttachmentLoadOp {
    Load = 0,
    Clear = 1,
    DontCare = 2,
}

#[repr(C)]
enum VkAttachmentStoreOp {
	Store = 0,
	DontCare = 1,
}

#[repr(C)]
struct VkAttachmentDescription {
	flags: VkAttachmentDescriptionFlagBits,
	format: VkFormat,
	samples: VkSampleCountFlagBits,
	loadOp: VkAttachmentLoadOp,
	storeOp: VkAttachmentStoreOp,
	stencilLoadOp: VkAttachmentLoadOp,
	stencilStoreOp: VkAttachmentStoreOp,
	initialLayout: VkImageLayout,
	finalLayout: VkImageLayout,
}

#[repr(C)]
enum VkPipelineBindPoint {
	Graphics = 0,
	Compute = 1,
}

//enum VkDynamicState {
	
//}

#[repr(C)]
enum VkShaderStageFlagBits {
	Vertex = 0x00000001,
	TessellationControl = 0x00000002,
	TessellationEvaluation = 0x00000004,
	Geometry = 0x00000008,
	Fragment = 0x00000010,
	Compute = 0x00000020,
	AllGraphics = 0x0000001F,
	All = 0x7FFFFFFF,
}

#[repr(C)]
struct VkAttachmentReference {
	attachment: u32,
	layout: VkImageLayout,
}

#[repr(C)]
struct VkSubpassDescription {
	flags: VkFlags,
	pipelineBindPoint: VkPipelineBindPoint,
	inputAttachmentCount: u32,
	pInputAttachments: *const VkAttachmentReference,
	colorAttachmentCount: u32,
	pColorAttachments: *const VkAttachmentReference,
	pResolveAttachments: *const VkAttachmentReference,
	pDepthStencilAttachment: *const VkAttachmentReference,
	preserveAttachmentCount: u32,
	pPreserveAttachments: *const u32,
}

#[repr(C)]
struct VkSubpassDependency {
	srcSubpass: u32,
	dstSubpass: u32,
	srcStageMask: VkFlags,
	dstStageMask: VkFlags,
	srcAccessMask: VkFlags,
	dstAccessMask: VkFlags,
	dependencyFlags: VkFlags,
}

#[repr(C)]
struct VkRenderPassCreateInfo {
	sType: VkStructureType,
	pNext: *const VkVoid,
	flags: VkFlags,
	attachmentCount: u32,
	pAttachments: *const VkAttachmentDescription,
	subpassCount: u32,
	pSubpasses: *const VkSubpassDescription,
	dependencyCount: u32,
	pDependencies: *const VkSubpassDependency,
}

#[repr(C)]
struct VkPipelineCacheCreateInfo {
	sType: VkStructureType,
	pNext: *const VkVoid,
	flags: VkFlags,
	initialDataSize: usize,
	pInitialData: *const VkVoid,
}

/*#[repr(C)]
struct VkSpecializationInfo {
	mapEntryCount: u32,
    const *    pMapEntries: *const VkSpecializationMapEntry;
    size_t                             dataSize;
    const void*                        pData;
}

#[repr(C)]
struct VkPipelineShaderStageCreateInfo {
	sType: VkStructureType,
	pNext: *const VkVoid,
	flags: VkFlags,
	stage: VkShaderStageFlagBits,
	module: VkShaderModule,
	pName: *const i8,
    const VkSpecializationInfo*         pSpecializationInfo;
}*/

/*#[repr(C)]
struct VkGraphicsPipelineCreateInfo {
	sType: VkStructureType,
	pNext: *const VkVoid,
	flags: VkFlags,
	stageCount: u32,
	pStages: *const VkPipelineShaderStageCreateInfo,
	pVertexInputState: *const VkPipelineVertexInputStateCreateInfo,
	pInputAssemblyState: *const VkPipelineInputAssemblyStateCreateInfo,
	pTessellationState: *const VkPipelineTessellationStateCreateInfo,
	pViewportState: *const VkPipelineViewportStateCreateInfo,
	pRasterizationState: *const VkPipelineRasterizationStateCreateInfo,
	pMultisampleState: *const VkPipelineMultisampleStateCreateInfo,
	pDepthStencilState: *const VkPipelineDepthStencilStateCreateInfo,
	pColorBlendState: *const VkPipelineColorBlendStateCreateInfo,
	pDynamicState: *const VkPipelineDynamicStateCreateInfo,
	layout: VkPipelineLayout,
	renderPass: VkRenderPass,
	subpass: u32,
	basePipelineHandle: VkPipeline,
	basePipelineIndex: i32,
}*/

// Instance Function Types
type VkGetPhysicalDeviceSurfaceSupport = unsafe extern "C" fn(
	physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32,
	surface: VkSurface, psupported: *mut u32) -> VkResult;
type VkGetPhysicalDeviceSurfaceCapables = unsafe extern "C" fn(
	physicalDevice: VkPhysicalDevice, surface: VkSurface,
	pSurfaceCapabilities: *mut VkSurfaceCapabilities) -> VkResult;
type VkGetPhysicalDeviceSurfaceFormats = unsafe extern "C" fn(
	physicalDevice: VkPhysicalDevice, surface: VkSurface,
	pSurfaceFormatCount: *mut u32, pSurfaceFormats: *mut VkSurfaceFormat)
	-> VkResult;
type VkGetPhysicalDeviceSurfaceModes = unsafe extern "C" fn(
	physicalDevice: VkPhysicalDevice, surface: VkSurface,
	pPresentModeCount: *mut u32, pPresentModes: *mut VkPresentMode)
	-> VkResult;
type VkGetSwapchainImages = unsafe extern "C" fn(
	device: VkDevice, swapchain: VkSwapchain,
	pSwapchainImageCount: *mut u32, pSwapchainImages: *mut VkImage)
	-> VkResult;

// Device Function Types
type VkCreateSwapchain = unsafe extern "C" fn(
	device: VkDevice, pCreateInfo: *const VkSwapchainCreateInfo,
	pAllocator: *const VkAllocationCallbacks, pSwapchain: *mut VkSwapchain)
	-> VkResult;
type VkDestroySwapchain = unsafe extern "C" fn(
	device: VkDevice, swapchain: VkSwapchain,
	pAllocator: *const VkAllocationCallbacks) -> ();
type VkQueuePresent = unsafe extern "C" fn(
	queue: VkQueue, pPresentInfo: *const VkPresentInfo) -> VkResult;
type VkAcquireNextImage = unsafe extern "C" fn(
	device: VkDevice, swapchain: VkSwapchain, timeout: u64,
	semaphore: VkSemaphore, fence: VkFence, pImageIndex: *mut u32)
	-> VkResult;

#[link(name = "vulkan")]
extern {
	// select_gpu.rs
	fn vkGetInstanceProcAddr(instance : VkInstance, pname: *const i8)
		-> usize;
	fn vkGetDeviceProcAddr(device: VkDevice, pname: *const i8) -> usize;
// Xcb - Linux / Mac

// Windows

// Android

	// create_gpu_interface.rs
	
	// create_queue.rs
	fn vkGetPhysicalDeviceMemoryProperties(physicalDevice: VkPhysicalDevice,
		pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties) -> ();
	// create_command_buffer.rs
	fn vkCreateImageView(device: VkDevice,
		pCreateInfo: *const VkImageViewCreateInfo,
		pAllocator: *const VkAllocationCallbacks,
		pView: *mut VkImageView) -> VkResult;
	fn vkCreateImage(device: VkDevice,
		pCreateInfo: *const VkImageCreateInfo,
		pAllocator: *const VkAllocationCallbacks,
		pImage: *mut VkImage) -> VkResult;
	fn vkGetImageMemoryRequirements(device: VkDevice, image: VkImage,
		pMemoryRequirements: *mut VkMemoryRequirements) -> ();
	fn vkAllocateMemory(device: VkDevice,
		pAllocateInfo: *const VkMemoryAllocateInfo,
		pAllocator: *const VkAllocationCallbacks,
		pMemory: *mut VkDeviceMemory) -> VkResult;
	fn vkBindImageMemory(device: VkDevice, image: VkImage,
		memory: VkDeviceMemory, memoryOffset: u64) -> VkResult;
	fn vkGetImageSubresourceLayout(device: VkDevice, image: VkImage,
		pSubresource: *const VkImageSubresource,
		pLayout: *mut VkSubresourceLayout) -> ();
	fn vkCreateSampler(device: VkDevice,
		pCreateInfo: *const VkSamplerCreateInfo,
		pAllocator: *const VkAllocationCallbacks,
		pSampler: *mut VkSampler) -> VkResult;
	fn vkCreateBuffer(device: VkDevice,
		pCreateInfo: *const VkBufferCreateInfo,
		pAllocator: *const VkAllocationCallbacks,
		pBuffer: *mut VkBuffer) -> VkResult;
	fn vkGetBufferMemoryRequirements(device: VkDevice,
		buffer: VkBuffer,
		pMemoryRequirements: *mut VkMemoryRequirements) -> ();
	fn vkBindBufferMemory(device: VkDevice,
		buffer: VkBuffer, memory: VkDeviceMemory,
		memoryOffset: u64) -> VkResult;
	fn vkCreateDescriptorSetLayout(device: VkDevice,
		pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
		pAllocator: *const VkAllocationCallbacks,
		pSetLayout: *mut VkDescriptorSetLayout) -> VkResult;
	fn vkCreatePipelineLayout(device: VkDevice,
		pCreateInfo: *const VkPipelineLayoutCreateInfo,
		pAllocator: *const VkAllocationCallbacks,
		pPipelineLayout: *mut VkPipelineLayout) -> VkResult;
	fn vkCreateRenderPass(device: VkDevice,
		pCreateInfo: *const VkRenderPassCreateInfo,
		pAllocator: *const VkAllocationCallbacks,
		pRenderPass: *mut VkRenderPass) -> VkResult;
	fn vkCreatePipelineCache(device: VkDevice,
		pCreateInfo: *const VkPipelineCacheCreateInfo,
		pAllocator: *const VkAllocationCallbacks,
		pPipelineCache: *mut VkPipelineCache) -> VkResult;
/*	fn vkCreateGraphicsPipelines(device: VkDevice,
		pipelineCache: VkPipelineCache,
		createInfoCount: u32,
		pCreateInfos: *const VkGraphicsPipelineCreateInfo,
		pAllocator: *const VkAllocationCallbacks,
		pPipelines: *mut VkPipeline) -> VkResult;*/
}

struct VulkanTexture {
	sampler: VkSampler,

	image: VkImage,
	imageLayout: VkImageLayout,

	mem_alloc: VkMemoryAllocateInfo,
	mem: VkDeviceMemory,
	view: VkImageView,
	tex_width: i32,
	tex_height: i32,
}

pub struct Vulkan {
	instance : VkInstance,
	get_gpu_surface_support: VkGetPhysicalDeviceSurfaceSupport,
	get_gpu_surface_capabilities: VkGetPhysicalDeviceSurfaceCapables,
	get_gpu_surface_formats: VkGetPhysicalDeviceSurfaceFormats,
	get_gpu_surface_modes: VkGetPhysicalDeviceSurfaceModes,
	get_swapchain_images: VkGetSwapchainImages,
	qfc: u32,
	qfp: [VkQueueFamilyProperties; 64],
	gpu: VkPhysicalDevice,
	device: VkDevice,
}

fn load_instance_functions(instance: VkInstance, function_name: &str) -> usize {
	let function_pointer = unsafe {
		vkGetInstanceProcAddr(instance, CString::new(function_name)
			.unwrap().as_ptr())
	};
	if function_pointer == 0 {
		panic!("Load failed for function: {}", function_name);
	}
	function_pointer
}

fn load_device_functions(device: VkDevice, function_name: &str) -> usize {
	let function_pointer = unsafe {
		vkGetDeviceProcAddr(device, CString::new(function_name)
			.unwrap().as_ptr())
	};
	if function_pointer == 0 {
		panic!("Load failed for function: {}", function_name);
	}
	function_pointer
}

fn load_texture(vkc: &Vulkan, ppm_data: &'static [u8]) -> VulkanTexture {
	let image = super::image::load(ppm_data);
	let width = image.size.0;
	let height = image.size.1;
	let image_ci = VkImageCreateInfo {
		sType: VkStructureType::ImageCreateInfo,
		pNext: null(),
		flags: 0,
		imageType: VkImageType::VK_IMAGE_TYPE_2D,
		format: VkFormat::VK_FORMAT_R8G8B8A8_UNORM,
		extent: VkExtent3D { width: width, height: height, depth: 1 },
		mipLevels: 1,
		arrayLayers: 1,
		samples: 0x00000001,
		tiling: VkImageTiling::VK_IMAGE_TILING_LINEAR,
		usage: 0x00000004,
		sharingMode: VkSharingMode::Exclusive,
		queueFamilyIndexCount: 0,
		pQueueFamilyIndices: null(),
		initialLayout: VkImageLayout::VK_IMAGE_LAYOUT_PREINITIALIZED,
	};
	let mut texture_image: VkImage = 0;
	let mut memreqs = VkMemoryRequirements {
		size: 0, alignment: 0, memoryTypeBits: 0
	};
	check_error("load_texture(): vkCreateImage", unsafe {
		vkCreateImage(vkc.device, &image_ci, null(), &mut texture_image)
	});
	unsafe {
		vkGetImageMemoryRequirements(vkc.device, texture_image,
			&mut memreqs);
	}

//
	let mut memory : VkDeviceMemory = 0;
	let memory_info = VkMemoryAllocateInfo {
		sType: VkStructureType::MemoryAllocateInfo,
		pNext: null(),
		allocationSize: memreqs.size,
		memoryTypeIndex: 0,
	};
	check_error("load_texture(): vkAllocateMemory", unsafe {
		vkAllocateMemory(vkc.device, &memory_info, null(), &mut memory)
	});
	check_error("load_texture(): vkBindImageMemory", unsafe {
		vkBindImageMemory(vkc.device, texture_image, memory, 0)
	});
//
	let subres = VkImageSubresource {
		aspectMask: 0x00000001,
		mipLevel: 0,
		arrayLayer: 0,
	};
	let mut layout = VkSubresourceLayout {
		offset: 0,
		size: 0,
		rowPitch: 0,
		arrayPitch: 0,
		depthPitch: 0,
	};
	let mut data : *mut u8 = null_mut();
	check_error("load_texture(): vkMapMemory", unsafe {
		vkGetImageSubresourceLayout(vkc.device, texture_image, &subres,
			&mut layout);
		vkMapMemory(vkc.device, memory, 0, memory_info.allocationSize,
			0, (&mut data) as *mut *mut _ as *mut *mut VkVoid)
	});
	let a : u8 = 255;
	unsafe {
		for i in 0..width {
			for j in 0..height {
				memcpy(image.pixels.offset(3 * ((i * width) + j)
					as isize),
					data.offset(4 * ((i * width) + j)
					as isize), 3);
				*data.offset(3 + (4 * ((i * width) + j))
					as isize) = a;
			}
		}
		vkUnmapMemory(vkc.device, memory);
	}
	VulkanTexture {
		sampler: 0, // VkSampler

		image: texture_image,
		imageLayout: VkImageLayout::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,

		mem_alloc: memory_info,
		mem: memory,
		view: 0, // VkImageView
		tex_width: image.size.0 as i32,
		tex_height: image.size.1 as i32,
	}
}

pub fn init(app_name:&str) -> Vulkan {
	// vkCreateInstance()

	// select_gpu.rs

	// Get GPU object.
	let mut num_gpus : u32 = 0;
	let mut gpu : VkPhysicalDevice = 0;

	check_error("init_swapchain(): vkEnumeratePhysicalDevices A", unsafe {
		vkEnumeratePhysicalDevices(instance, &mut num_gpus, null_mut())
	});
	println!("vkEnumeratePhysicalDevices() returned {}", num_gpus);
	if num_gpus == 0 {
		panic!("No GPUS found! Do you have Vulkan ICD installed?");
	}
	num_gpus = 1;
	e = unsafe {
		vkEnumeratePhysicalDevices(instance, &mut num_gpus, &mut gpu)
	};
	match e {
		VkResult::Success => {},
		VkResult::Incomplete => {}, // If only 1 gpu is returned - ok.
		_ => panic!("vkEnumeratePhysicalDevices B Failed {}", e),
	}
	println!("vkEnumeratePhysicalDevices() completed!");

	// Get queue properties
	let mut qfc : u32 = 0;
	unsafe {
		vkGetPhysicalDeviceQueueFamilyProperties(gpu, &mut qfc,
			null_mut())
	};
	println!("vkGetPhysicalDeviceQueueFamilyProperties() returned {}", qfc);
	if qfc == 0 {
		panic!("No Device Queue Families Found!");
	}

	let mut qfp : [VkQueueFamilyProperties; 64] = [VkQueueFamilyProperties {
		queueFlags : 0, queueCount : 0, timestampValidBits : 0,
		minImageTransferGranularity : VkExtent3D {
			width:0, height:0, depth:0
		}
	}; 64];
	unsafe {
		vkGetPhysicalDeviceQueueFamilyProperties(gpu, &mut qfc, &mut qfp[0]);
	}

	// Get functions
	let vk_get_gpu_surface_support = unsafe {
		*(&load_instance_functions(instance,
		"vkGetPhysicalDeviceSurfaceSupportKHR") as *const _
		as *const VkGetPhysicalDeviceSurfaceSupport)
	};
	let vk_get_gpu_surface_capabilities = unsafe {
		*(&load_instance_functions(instance,
		"vkGetPhysicalDeviceSurfaceCapabilitiesKHR") as *const _
		as *const VkGetPhysicalDeviceSurfaceCapables)
	};
	let vk_get_gpu_surface_formats = unsafe {
		*(&load_instance_functions(instance,
		"vkGetPhysicalDeviceSurfaceFormatsKHR") as *const _
		as *const VkGetPhysicalDeviceSurfaceFormats)
	};
	let vk_get_gpu_surface_modes = unsafe {
		*(&load_instance_functions(instance,
		"vkGetPhysicalDeviceSurfacePresentModesKHR") as *const _
		as *const VkGetPhysicalDeviceSurfaceModes)
	};
	let vk_get_swapchain_images = unsafe {
		*(&load_instance_functions(instance,
		"vkGetSwapchainImagesKHR") as *const _
		as *const VkGetSwapchainImages)
	};
	println!("Loaded Functions!");

	Vulkan { instance: instance,
		get_gpu_surface_support: vk_get_gpu_surface_support,
		get_gpu_surface_capabilities: vk_get_gpu_surface_capabilities,
		get_gpu_surface_formats: vk_get_gpu_surface_formats,
		get_gpu_surface_modes: vk_get_gpu_surface_modes,
		get_swapchain_images: vk_get_swapchain_images,
		qfc: qfc, qfp: qfp, gpu: gpu,
		// Uninit'd
		device: 0,
	}
}

pub fn init_swapchain(vkc: &mut Vulkan, native: &mut super::NativeWindow) {
	// create_surface.rs

	let mut queue_gfx = -1;
	let mut queue_present = -1;
	for i in 0..((*vkc).qfc as usize) {
		let mut supports_present : u32 = 0;
		check_error("init_swapchain(): get_gpu_surface_support",unsafe {
			(vkc.get_gpu_surface_support)(vkc.gpu,
				i as u32, surface, &mut supports_present)
		});
		if supports_present != 0 {
			queue_present = i as i32;
		}

		if ((*vkc).qfp[i].queueFlags & 0x00000001) != 0 {
			queue_gfx = i as i32;
			if supports_present != 0 {
				break;
			}
		}
	}
	println!("Present Queue: {}", queue_present);
	println!("Graphics Queue: {}", queue_gfx);

	if queue_gfx == -1 {
		panic!("Couldn't find a queue that supports graphics ):");
	}
	if queue_present == -1 {
		panic!("Couldn't find a queue that supports presentation ):");
	}

	// Create Device
	let queue_priorities : f32 = 0.0;
	let device_queue_create_info = VkDeviceQueueCreateInfo {
		sType: VkStructureType::DeviceQueueCreateInfo,
		pNext: null(),
		flags: 0,
		queueFamilyIndex: queue_gfx as u32,
		queueCount: 1,
		pQueuePriorities: &queue_priorities,
	};
	let device_create_info = VkDeviceCreateInfo {
		sType: VkStructureType::DeviceCreateInfo,
		pNext: null(),
		flags: 0,
		queueCreateInfoCount: 1,
		pQueueCreateInfos: &device_queue_create_info,
		enabledLayerCount: 0,
		ppEnabledLayerNames: null(),
		enabledExtensionCount: 0,
		ppEnabledExtensionNames: null(),
		pEnabledFeatures: null(),
	};
	
	check_error("init_swapchain(): vkCreateDevice", unsafe {
		vkCreateDevice(vkc.gpu, &device_create_info, null(),
			&mut vkc.device)
	});

	// Get functions
	let vk_create_swapchain = unsafe {
		*(&load_device_functions(vkc.device, "vkCreateSwapchainKHR")
		as *const _ as *const VkCreateSwapchain)
	};
	let vk_destroy_swapchain = unsafe {
		*(&load_device_functions(vkc.device, "vkDestroySwapchainKHR")
		as *const _ as *const VkDestroySwapchain)
	};
	let vk_get_swapchain_images = unsafe {
		*(&load_device_functions(vkc.device, "vkGetSwapchainImagesKHR")
		as *const _ as *const VkGetSwapchainImages)
	};
	let vk_queue_present = unsafe {
		*(&load_device_functions(vkc.device, "vkQueuePresentKHR")
		as *const _ as *const VkQueuePresent)
	};
	let vk_aquire_next_image = unsafe {
		*(&load_device_functions(vkc.device, "vkAcquireNextImageKHR")
		as *const _ as *const VkAcquireNextImage)
	};

	// Get device queue
	let mut queue : VkQueue = 0;
	unsafe {
		vkGetDeviceQueue(vkc.device, queue_gfx as u32, 0, &mut queue);
	}

	// Find most preferred format
	let mut format_count = 0;
	let mut preferred_format : VkFormat;
	let mut color_space : VkColorSpace;
	check_error("init_swapchain(): get_gpu_surface_formats A", unsafe {
		(vkc.get_gpu_surface_formats)(vkc.gpu, surface,
			&mut format_count, null_mut())
	});

	let mut surface_formats : [VkSurfaceFormat; 64] = [VkSurfaceFormat {
		format : VkFormat::VK_FORMAT_UNDEFINED, color_space : 0,
	}; 64];
	check_error("init_swapchain(): get_gpu_surface_formats B", unsafe {
		(vkc.get_gpu_surface_formats)(vkc.gpu, surface,
			&mut format_count, &mut surface_formats[0])
	});
	if format_count == 0 {
		panic!("No formats found! ):");
	}
	if surface_formats[0].format == VkFormat::VK_FORMAT_UNDEFINED {
		preferred_format = VkFormat::VK_FORMAT_B8G8R8A8_UNORM;
	}else{
		preferred_format = surface_formats[0].format;
	}
	color_space = surface_formats[0].color_space;

	// Get memory info & properties
	let mut gpu_mem_props = VkPhysicalDeviceMemoryProperties {
		memoryTypeCount: 0,
		memoryTypes: [VkMemoryType { heapIndex: 0, propertyFlags: 0 }; 32],
		memoryHeapCount: 0,
		memoryHeaps: [VkMemoryHeap { flags: 0, size: 0 }; 16],
	};
	unsafe {
		vkGetPhysicalDeviceMemoryProperties(vkc.gpu, &mut gpu_mem_props)
	}

	// Prepare #0: Create the command pool
	let mut command_pool : VkCommandPool = 0;
	let command_pool_create_info = VkCommandPoolCreateInfo {
		sType: VkStructureType::CommandPoolCreateInfo,
		pNext: null(),
		flags: 0,
		queueFamilyIndex: queue_gfx as u32,
	};
	check_error("init_swapchain(): vkCreateCommandPool", unsafe {
		vkCreateCommandPool(vkc.device, &command_pool_create_info,
			null(), &mut command_pool)
	});

	// Prepare #1: Buffers
	let mut surface_capables = VkSurfaceCapabilities {
		minImageCount: 0,
		maxImageCount: 0,
		currentExtent: VkExtent2D { width: 0, height: 0 },
		minImageExtent: VkExtent2D { width: 0, height: 0 },
		maxImageExtent: VkExtent2D { width: 0, height: 0 },
		maxImageArrayLayers: 0,
		supportedTransforms: 0,
		currentTransform: 0,
		supportedCompositeAlpha: 0,
		supportedUsageFlags: 0,
	};

	check_error("init_swapchain(): get_gpu_surface_capabilities", unsafe {
		(vkc.get_gpu_surface_capabilities)(vkc.gpu, surface,
			&mut surface_capables)
	});

	let mut num_present_modes: u32 = 0;
	check_error("init_swapchain(): get_gpu_surface_modes A", unsafe {
		(vkc.get_gpu_surface_modes)(vkc.gpu, surface,
			&mut num_present_modes, null_mut())
	});

	let mut present_modes : [VkPresentMode; 32] =
		[VkPresentMode::Immediate; 32];
	check_error("init_swapchain(): get_gpu_surface_modes B", unsafe {
		(vkc.get_gpu_surface_modes)(vkc.gpu, surface,
			&mut num_present_modes, &mut present_modes[0])
	});

	let mut width = 0;
	let mut height = 0;
	if surface_capables.currentExtent.width == std::u32::MAX {
		width = 640;
		height = 360;
	}else{
		width = surface_capables.currentExtent.width;
		height = surface_capables.currentExtent.height;
	}

	// Choose ideal mode (mailbox) if available, Fifo is always available.
	let mut present_mode: VkPresentMode = VkPresentMode::Fifo;
	for i in 0..(num_present_modes as usize) {
		// Ideal mode
		if present_modes[i] == VkPresentMode::Mailbox {
			present_mode = VkPresentMode::Mailbox;
			break;
		}
	}

	let mut num_swapchain_images = surface_capables.minImageCount + 1;
	if num_swapchain_images > surface_capables.maxImageCount &&
		surface_capables.maxImageCount > 0
	{
		num_swapchain_images = surface_capables.maxImageCount;
	}

	let pre_transform_flags;

	if surface_capables.supportedTransforms & 0x00000001 != 0 {
		pre_transform_flags = 0x00000001;
	} else {
		pre_transform_flags = surface_capables.currentTransform;
	}

	let mut swapchain : VkSwapchain = 0;
	let swapchain_create_info = VkSwapchainCreateInfo {
		sType: VkStructureType::SwapchainCreateInfo,
		pNext: null(),
		flags: 0,
		surface: surface,
		minImageCount: num_swapchain_images,
		imageFormat: preferred_format,
		imageColorSpace: color_space,
		imageExtent: VkExtent2D { width: width, height: height},
		imageArrayLayers: 1,
		imageUsage: 0x00000010,
		imageSharingMode: VkSharingMode::Exclusive,
		queueFamilyIndexCount: 0,
		pQueueFamilyIndices: null(),
		preTransform: pre_transform_flags,
		compositeAlpha: 0x00000001,
		presentMode: present_mode,
		clipped: 0,
		oldSwapchain: swapchain,
	};
	check_error("init_swapchain(): vk_create_swapchain", unsafe {
		(vk_create_swapchain)(vkc.device, &swapchain_create_info,
			null(), &mut swapchain)
	});

	// TODO: Destroy old swapchain on resize here....
	let mut num_img : u32 = 0;
	check_error("init_swapchain(): vk_get_swapchain_images A", unsafe {
		(vk_get_swapchain_images)(vkc.device, swapchain, &mut num_img,
			null_mut())
	});

	let mut swapchain_images : [VkImage; 32] = [0; 32];
	check_error("init_swapchain(): vk_get_swapchain_images B", unsafe {
		(vk_get_swapchain_images)(vkc.device, swapchain, &mut num_img,
			&mut swapchain_images[0])
	});
	let mut swapchain_buffers : [VkSwapchainBuffers; 32] =
		[VkSwapchainBuffers { image:0, cmd:0, view:0 }; 32];
	for i in 0..(num_swapchain_images as usize) {
		swapchain_buffers[i].image = swapchain_images[i];

		let color_image_view = VkImageViewCreateInfo {
			sType: VkStructureType::ImageViewCreateInfo,
			pNext: null(),
			flags: 0,
			image: swapchain_buffers[i].image,
			viewType: VkImageViewType::Vk2d,
			format: preferred_format,
			components: VkComponentMapping {
				r: VkComponentSwizzle::R,
				g: VkComponentSwizzle::G,
				b: VkComponentSwizzle::B,
				a: VkComponentSwizzle::A,
			},
			subresourceRange: VkImageSubresourceRange {
				aspectMask: 0x00000001,
				baseMipLevel: 0,
				levelCount: 1,
				baseArrayLayer: 0,
				layerCount: 1,
			},
		};

		check_error("init_swapchain(): vkCreateImageView", unsafe {
			vkCreateImageView(vkc.device, &color_image_view, null(),
				&mut swapchain_buffers[i].view)
		});
	}

	// Prepare #2: Depth
	let image_create_info = VkImageCreateInfo {
		sType: VkStructureType::ImageCreateInfo,
		pNext: null(),
		flags: 0,
		imageType: VkImageType::VK_IMAGE_TYPE_2D,
		format: VkFormat::VK_FORMAT_D16_UNORM,
		extent: VkExtent3D { width: width, height: height, depth: 1 },
		mipLevels: 1,
		arrayLayers: 1,
		samples: 0x00000001,
		tiling: VkImageTiling::VK_IMAGE_TILING_OPTIMAL,
		usage: 0x00000020,
		sharingMode: VkSharingMode::Exclusive,
		queueFamilyIndexCount: 0,
		pQueueFamilyIndices: null(),
		initialLayout: VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED,
	};
	let mut depth_image : VkImage = 0;
	let mut memreqs = VkMemoryRequirements {
		size: 0, alignment: 0, memoryTypeBits: 0
	};
	check_error("init_swapchain(): vkCreateImage", unsafe {
		vkCreateImage(vkc.device, &image_create_info, null(),
			&mut depth_image)
	});
	unsafe {
		vkGetImageMemoryRequirements(vkc.device, depth_image,
			&mut memreqs);
	}

	let mut depth_memory : VkDeviceMemory = 0;
	let depth_memory_info = VkMemoryAllocateInfo {
		sType: VkStructureType::MemoryAllocateInfo,
		pNext: null(),
		allocationSize: memreqs.size,
		memoryTypeIndex: 0,
	};

	check_error("init_swapchain(): vkAllocateMemory", unsafe {
		vkAllocateMemory(vkc.device, &depth_memory_info, null(),
			&mut depth_memory)
	});

	check_error("init_swapchain(): vkBindImageMemory", unsafe {
		vkBindImageMemory(vkc.device, depth_image, depth_memory, 0)
	});

	let depth_view_create_info = VkImageViewCreateInfo {
		sType: VkStructureType::ImageViewCreateInfo,
		pNext: null(),
		flags: 0,
		image: depth_image,
		viewType: VkImageViewType::Vk2d,
		format: VkFormat::VK_FORMAT_D16_UNORM,
		components: VkComponentMapping {
			r: VkComponentSwizzle::Identity,
			g: VkComponentSwizzle::Identity,
			b: VkComponentSwizzle::Identity,
			a: VkComponentSwizzle::Identity,
		},
		subresourceRange: VkImageSubresourceRange {
			aspectMask: 0x00000002,
			baseMipLevel: 0,
			levelCount: 1,
			baseArrayLayer: 0,
			layerCount: 1,
		},
	};
	let mut depth_view : VkImageView = 0;
	check_error("init_swapchain(): vkCreateImageView", unsafe {
		vkCreateImageView(vkc.device, &depth_view_create_info, null(),
			&mut depth_view)
	});
	println!("Resize Step #2 complete!");

	// Prepare #3: Textures
	let mut texture = load_texture(vkc,
		include_bytes!("../../examples/resources/logo.ppm"));
	let sample_ci = VkSamplerCreateInfo {
		sType: VkStructureType::SamplerCreateInfo,
		pNext: null(),
		flags: 0,
		magFilter: VkFilter::VK_FILTER_NEAREST,
		minFilter: VkFilter::VK_FILTER_NEAREST,
		mipmapMode: VkSamplerMipmapMode::VK_SAMPLER_MIPMAP_MODE_NEAREST,
		addressModeU: VkSamplerAddressMode::VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE,
		addressModeV: VkSamplerAddressMode::VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE,
		addressModeW: VkSamplerAddressMode::VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE,
		mipLodBias: 0.0,
		anisotropyEnable: 0,
		maxAnisotropy: 1.0,
		compareEnable: 0,
		compareOp: VkCompareOp::Never,
		minLod: 0.0,
		maxLod: 0.0,
		borderColor: VkBorderColor::VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE,
		unnormalizedCoordinates: 0,
	};

	check_error("init_swapchain(): vkCreateSampler", unsafe {
		vkCreateSampler(vkc.device, &sample_ci, null(),
			&mut texture.sampler)
	});
		//
	let texture_view_ci = VkImageViewCreateInfo {
		sType: VkStructureType::ImageViewCreateInfo,
		pNext: null(),
		flags: 0,
		image: texture.image,
		viewType: VkImageViewType::Vk2d,
		format: VkFormat::VK_FORMAT_R8G8B8A8_UNORM,
		components: VkComponentMapping {
			r: VkComponentSwizzle::R,
			g: VkComponentSwizzle::G,
			b: VkComponentSwizzle::B,
			a: VkComponentSwizzle::A,
		},
		subresourceRange: VkImageSubresourceRange {
			aspectMask: 0x00000001,
			baseMipLevel: 0,
			levelCount: 1,
			baseArrayLayer: 0,
			layerCount: 1,
		},
	};
	let mut texture_view : VkImageView = 0;
	check_error("init_swapchain(): vkCreateImageView", unsafe {
		vkCreateImageView(vkc.device, &texture_view_ci, null(),
			&mut texture_view)
	});

	println!("Resize Step #3 complete: Load Textures!");
	// Prepare #4: Set up 3D Model Buffer ( Test: Cube ).
	let vertbuff_ci = VkBufferCreateInfo {
		sType: VkStructureType::BufferCreateInfo,
		pNext: null(),
		flags: 0,
		size: 4 * (( 4 * 4 ) + (12 * 3 * 4) + (12 * 3 * 4)),
		usage: 0x00000010,
		sharingMode: VkSharingMode::Exclusive,
		queueFamilyIndexCount: 0,
		pQueueFamilyIndice: null(),
	};
	let mut u_buffer : VkBuffer = 0;
	let mut u_memory : VkDeviceMemory = 0;
	let u_data_size : usize = ( 4 * 4 ) + (12 * 3 * 4) + (12 * 3 * 4);
//	let mut u_data = UData { mvp: [0.0;(4*4)], position: [0.0; (12 * 3 * 4)],
//		ac: [0.0; (12 * 3 * 4)] };
	check_error("init_swapchain(): vkCreateBuffer", unsafe {
		vkCreateBuffer(vkc.device, &vertbuff_ci, null(), &mut u_buffer)
	});
	let mut u_memreqs = VkMemoryRequirements {
		size: 0, alignment: 0, memoryTypeBits: 0
	};
	unsafe {
		vkGetBufferMemoryRequirements(vkc.device, u_buffer,
			&mut u_memreqs);
	}
	let u_memory_info = VkMemoryAllocateInfo {
		sType: VkStructureType::MemoryAllocateInfo,
		pNext: null(),
		allocationSize: u_memreqs.size,
		memoryTypeIndex: 0,
	};
	check_error("init_swapchain(): vkAllocateMemory", unsafe {
		vkAllocateMemory(vkc.device, &u_memory_info, null(), &mut u_memory)
	});
	let mut u_data : *mut f32 = null_mut();
	check_error("load_texture(): vkMapMemory", unsafe {
		vkMapMemory(vkc.device, u_memory, 0, u_memory_info.allocationSize,
			0, (&mut u_data) as *mut *mut _ as *mut *mut VkVoid)
	});
		//
	let matrix : [f32; 16] = [
		1.0, 0.0, 0.0, 0.0,
		0.0, 1.0, 0.0, 0.0,
		0.0, 0.0, 1.0, 0.0,
		0.0, 0.0, 0.0, 1.0,
	];
	let vertices : [f32; (3 * 6 * 6)] = [
		-1.0,-1.0,-1.0,  // -X side
		-1.0,-1.0, 1.0,
		-1.0, 1.0, 1.0,
		-1.0, 1.0, 1.0,
		-1.0, 1.0,-1.0,
		-1.0,-1.0,-1.0,

		-1.0,-1.0,-1.0,  // -Z side
		 1.0, 1.0,-1.0,
		 1.0,-1.0,-1.0,
		-1.0,-1.0,-1.0,
		-1.0, 1.0,-1.0,
		 1.0, 1.0,-1.0,

		-1.0,-1.0,-1.0,  // -Y side
		 1.0,-1.0,-1.0,
		 1.0,-1.0, 1.0,
		-1.0,-1.0,-1.0,
		 1.0,-1.0, 1.0,
		-1.0,-1.0, 1.0,

		-1.0, 1.0,-1.0,  // +Y side
		-1.0, 1.0, 1.0,
		 1.0, 1.0, 1.0,
		-1.0, 1.0,-1.0,
		 1.0, 1.0, 1.0,
		 1.0, 1.0,-1.0,

		 1.0, 1.0,-1.0,  // +X side
		 1.0, 1.0, 1.0,
		 1.0,-1.0, 1.0,
		 1.0,-1.0, 1.0,
		 1.0,-1.0,-1.0,
		 1.0, 1.0,-1.0,

		-1.0, 1.0, 1.0,  // +Z side
		-1.0,-1.0, 1.0,
		 1.0, 1.0, 1.0,
		-1.0,-1.0, 1.0,
		 1.0,-1.0, 1.0,
		 1.0, 1.0, 1.0,
	];
	let uv : [f32; (2 * 6 * 6)] = [
		0.0, 1.0,  // -X side
		1.0, 1.0,
		1.0, 0.0,
		1.0, 0.0,
		0.0, 0.0,
		0.0, 1.0,

		1.0, 1.0,  // -Z side
		0.0, 0.0,
		0.0, 1.0,
		1.0, 1.0,
		1.0, 0.0,
		0.0, 0.0,

		1.0, 0.0,  // -Y side
		1.0, 1.0,
		0.0, 1.0,
		1.0, 0.0,
		0.0, 1.0,
		0.0, 0.0,

		1.0, 0.0,  // +Y side
		0.0, 0.0,
		0.0, 1.0,
		1.0, 0.0,
		0.0, 1.0,
		1.0, 1.0,

		1.0, 0.0,  // +X side
		0.0, 0.0,
		0.0, 1.0,
		0.0, 1.0,
		1.0, 1.0,
		1.0, 0.0,

		0.0, 0.0,  // +Z side
		0.0, 1.0,
		1.0, 0.0,
		0.0, 1.0,
		1.0, 1.0,
		1.0, 0.0,
	];
	// MVP
	for i in 0..16 {
		let tocopy = matrix[i as usize];
		unsafe {
			memcpy(&tocopy, u_data.offset(4 * i), 4);
		}
	}
	// Position
	for i in 0..36 {
		let tocopy : [f32; 4] = [
			vertices[((i * 3) + 0) as usize],
			vertices[((i * 3) + 1) as usize],
			vertices[((i * 3) + 2) as usize],
			1.0,
		];
		unsafe {
			memcpy(&tocopy[0], u_data.offset(64).offset(4 * i), 16);
		}
	}
	// Vertex Attributes
	for i in 0..36 {
		let tocopy : [f32; 4] = [
			uv[((i * 2) + 0) as usize],
			uv[((i * 2) + 1) as usize],
			0.0,
			0.0,
		];
		unsafe {
			memcpy(&tocopy[0], u_data.offset(640).offset(4 * i), 16);
		}
	}
	check_error("load_texture(): vkBindBufferMemory", unsafe {
		vkUnmapMemory(vkc.device, u_memory);
		vkBindBufferMemory(vkc.device, u_buffer, u_memory, 0)
	});
	println!("Wrote Vertex Data.");

	// Prepare #5: Descriptor Layout.
	let num_images = 1;
	let layout_bindings = [
		VkDescriptorSetLayoutBinding {
			binding: 0,
			descriptorType: VkDescriptorType::UniformBuffer,
			descriptorCount: 1,
			stageFlags: VkShaderStageFlags::VertexBit,
			pImmutableSamplers: null(),
		},
		VkDescriptorSetLayoutBinding {
			binding: 1,
			descriptorType: VkDescriptorType::CombinedImageSampler,
			descriptorCount: num_images,
			stageFlags: VkShaderStageFlags::FragmentBit,
			pImmutableSamplers: null(),
		}
	];
	let descriptor_layout = VkDescriptorSetLayoutCreateInfo {
		sType: VkStructureType::DescriptorSetLayoutCreateInfo,
		pNext: null(),
		flags: 0,
		bindingCount: 2,
		pBindings: &layout_bindings[0],
	};
	let mut desc_layout = 0;
	check_error("P5: vkCreateDescriptorSetLayout", unsafe {
		vkCreateDescriptorSetLayout(vkc.device, &descriptor_layout,
			null(), &mut desc_layout)
	});

	let pipeline_layout_ci = VkPipelineLayoutCreateInfo {
		sType: VkStructureType::PipelineLayoutCreateInfo,
		pNext: null(),
		flags: 0,
		setLayoutCount: 1,
		pSetLayouts: &desc_layout,
		pushConstantRangeCount: 0,
		pPushConstantRanges: null(),
	};
	let mut pipeline_layout = 0;
	check_error("P5: vkCreatePipelineLayout", unsafe {
		vkCreatePipelineLayout(vkc.device, &pipeline_layout_ci, null(),
			&mut pipeline_layout)
	});
	println!("P5: Prepared Pipeline Descriptor Layout");

	// Prepare #6: Render Pass
	let render_pass_attachments = [
		VkAttachmentDescription {
			flags: VkAttachmentDescriptionFlagBits::MayAliasBit,
			format: preferred_format,
			samples: VkSampleCountFlagBits::Bit1,
			loadOp: VkAttachmentLoadOp::Clear,
			storeOp: VkAttachmentStoreOp::Store,
			stencilLoadOp: VkAttachmentLoadOp::DontCare,
			stencilStoreOp: VkAttachmentStoreOp::DontCare,
			initialLayout: VkImageLayout::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
			finalLayout: VkImageLayout::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
		},
		VkAttachmentDescription {
			flags: VkAttachmentDescriptionFlagBits::MayAliasBit,
			format: VkFormat::VK_FORMAT_D16_UNORM,
			samples: VkSampleCountFlagBits::Bit1,
			loadOp: VkAttachmentLoadOp::Clear,
			storeOp: VkAttachmentStoreOp::DontCare,
			stencilLoadOp: VkAttachmentLoadOp::DontCare,
			stencilStoreOp: VkAttachmentStoreOp::DontCare,
			initialLayout: VkImageLayout::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
			finalLayout: VkImageLayout::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
		},
	];
	let render_pass_color_reference = VkAttachmentReference {
		attachment: 0,
		layout: VkImageLayout::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
	};
	let render_pass_depth_reference = VkAttachmentReference {
		attachment: 1,
		layout: VkImageLayout::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
	};
	let render_pass_subpass = VkSubpassDescription {
		flags: 0,
		pipelineBindPoint: VkPipelineBindPoint::Graphics,
		inputAttachmentCount: 0,
		pInputAttachments: null(),
		colorAttachmentCount: 1,
		pColorAttachments: &render_pass_color_reference,
		pResolveAttachments: null(),
		pDepthStencilAttachment: &render_pass_depth_reference,
		preserveAttachmentCount: 0,
		pPreserveAttachments: null(),
	};
	let render_pass_ci = VkRenderPassCreateInfo {
		sType: VkStructureType::RenderPassCreateInfo,
		pNext: null(),
		flags: 0,
		attachmentCount: 2,
		pAttachments: &render_pass_attachments[0],
		subpassCount: 1,
		pSubpasses: &render_pass_subpass,
		dependencyCount: 0,
		pDependencies: null(),
	};
	let mut render_pass = 0;
	check_error("P6: vkCreatePipelineLayout", unsafe {
		vkCreateRenderPass(vkc.device, &render_pass_ci, null(),
			&mut render_pass)
	});
	println!("P6: Prepared Render Pass");

	// Prepare #7: Prepare Pipeline
/*	let pipeline_cache_ci = VkPipelineCacheCreateInfo {
		sType: VkStructureType::PipelineCacheCreateInfo,
		pNext: null(),
		flags: 0,
		initialDataSize: 0,
		pInitialData: null(),
	};
	let mut pipeline_cache = 0;
	check_error("P7: vkCreatePipelineCache", unsafe {
		vkCreatePipelineCache(vkc.device, &pipeline_cache_ci, null(),
			&mut pipeline_cache)
	});*/
}
