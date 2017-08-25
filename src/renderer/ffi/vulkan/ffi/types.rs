// Willow Graphics API
//
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE
//
// src/renderer/ffi/vulkan/ffi/types.rs

use std::fmt;
use ami::Void;

pub type VkDeviceSize = u64;
pub type VkFlags = u32;
pub type VkBool32 = u32;

// Non-Dispatchable Handles
#[repr(C)] #[derive(Copy, Clone)] pub struct VkSurfaceKHR(u64);
#[repr(C)] #[derive(Copy, Clone)] pub struct VkDeviceMemory(u64);
#[repr(C)] #[derive(Copy, Clone)] pub struct VkImage(u64);
#[repr(C)] #[derive(Copy, Clone)] pub struct VkSwapchainKHR(u64);
#[repr(C)] #[derive(Copy, Clone)] pub struct VkRenderPass(u64);
#[repr(C)] #[derive(Copy, Clone)] pub struct VkFramebuffer(u64);
#[repr(C)] #[derive(Copy, Clone)] pub struct VkFence(u64);
#[repr(C)] #[derive(Copy, Clone)] pub struct VkSemaphore(u64);
#[repr(C)] #[derive(Copy, Clone)] pub struct VkImageView(u64);
#[repr(C)] #[derive(Copy, Clone)] pub struct VkBuffer(u64);
#[repr(C)] #[derive(Copy, Clone)] pub struct VkShaderModule(u64);
#[repr(C)] #[derive(Copy, Clone)] pub struct VkPipeline(u64);
#[repr(C)] #[derive(Copy, Clone)] pub struct VkDescriptorSetLayout(u64);
#[repr(C)] #[derive(Copy, Clone)] pub struct VkPipelineLayout(u64);
#[repr(C)] #[derive(Copy, Clone)] pub struct VkSampler(u64);
#[repr(C)] #[derive(Copy, Clone)] pub struct VkDescriptorSet(u64);
#[repr(C)] #[derive(Copy, Clone)] pub struct VkDescriptorPool(u64);

// Dispatchable Handles
#[repr(C)] #[derive(Copy, Clone)] pub struct VkDevice(*mut Void);
#[repr(C)] #[derive(Copy, Clone)] pub struct VkPhysicalDevice(*mut Void);
#[repr(C)] #[derive(Copy, Clone)] pub struct VkInstance(*mut Void);
#[repr(C)] #[derive(Copy, Clone)] pub struct VkCommandBuffer(*mut Void);
#[repr(C)] #[derive(Copy, Clone)] pub struct VkQueue(*mut Void);

#[repr(C)] pub struct VkDescriptorPoolCreateInfo {
	pub s_type: VkStructureType,
	pub next: *const Void,
	pub flags: VkFlags,
	pub max_sets: u32,
	pub pool_size_count: u32,
	pub pool_sizes: *const VkDescriptorPoolSize
}

#[repr(C)] pub struct VkDescriptorPoolSize {
	pub descriptor_type: VkDescriptorType,
	pub descriptor_count: u32,
}

#[repr(C)] pub struct VkDescriptorSetAllocateInfo {
	pub s_type: VkStructureType,
	pub next: *const Void,
	pub descriptor_pool: VkDescriptorPool,
	pub descriptor_set_count: u32,
	pub set_layouts: *const VkDescriptorSetLayout,
}

#[repr(C)] pub struct VkBufferCreateInfo {
	pub s_type: VkStructureType,
	pub next: *const Void,
	pub flags: VkFlags,
	pub size: VkDeviceSize,
	pub usage: VkBufferUsage,
	pub sharing_mode: VkSharingMode,
	pub queue_family_index_count: u32,
	pub queue_family_indices: *const u32,
}

#[repr(C)] pub struct VkDescriptorBufferInfo {
	pub buffer: VkBuffer,
	pub offset: VkDeviceSize,
	pub range: VkDeviceSize,
}

#[repr(C)] pub struct VkWriteDescriptorSet {
	pub s_type: VkStructureType,
	pub next: *const Void,
	pub dst_set: VkDescriptorSet,
	pub dst_binding: u32,
	pub dst_array_element: u32,
	pub descriptor_count: u32,
	pub descriptor_type: VkDescriptorType,
	pub image_info: *const VkDescriptorImageInfo,
	pub buffer_info: *const VkDescriptorBufferInfo,
	pub texel_buffer_view: *const Void,
}

#[repr(C)] pub struct VkDescriptorImageInfo {
	pub sampler: VkSampler,
	pub image_view: VkImageView,
	pub image_layout: VkImageLayout,
}

#[repr(C)] pub struct VkFramebufferCreateInfo {
	pub s_type: VkStructureType,
	pub p_next: *const Void,
	pub flags: VkFlags,
	pub render_pass: VkRenderPass,
	pub attachment_count: u32,
	pub attachments: *const VkImageView,
	pub width: u32,
	pub height: u32,
	pub layers: u32,
}

#[repr(C)] pub struct VkRenderPassCreateInfo {
	pub s_type: VkStructureType,
	pub p_next: *const Void,
	pub flags: VkFlags,
	pub attachment_count: u32,
	pub attachments: *const VkAttachmentDescription,
	pub subpass_count: u32,
	pub subpasses: *const VkSubpassDescription,
	pub dependency_count: u32,
	pub dependencies: *const Void,
}

#[repr(C)] pub struct VkSubpassDescription {
	pub flags: VkFlags,
	pub pipeline_bind_point: VkPipelineBindPoint,
	pub input_attachment_count: u32,
	pub input_attachments: *const VkAttachmentReference,
	pub color_attachment_count: u32,
	pub color_attachments: *const VkAttachmentReference,
	pub resolve_attachments: *const VkAttachmentReference,
	pub depth_stencil_attachment: *const VkAttachmentReference,
	pub preserve_attachment_count: u32,
	pub preserve_attachments: *const u32,
}

#[repr(C)] pub struct VkAttachmentReference {
	pub attachment: u32,
	pub layout: VkImageLayout,
}

#[repr(C)] pub struct VkAttachmentDescription {
	pub flags: VkFlags,
	pub format: VkFormat,
	pub samples: VkSampleCount,
	pub load_op: VkAttachmentLoadOp,
	pub store_op: VkAttachmentStoreOp,
	pub stencil_load_op: VkAttachmentLoadOp,
	pub stencil_store_op: VkAttachmentStoreOp,
	pub initial_layout: VkImageLayout,
	pub final_layout: VkImageLayout,
}

#[repr(C)] pub struct VkMemoryRequirements {
	pub size: VkDeviceSize,
	pub alignment: VkDeviceSize,
	pub memory_type_bits: u32,
}

#[repr(C)] pub struct VkMemoryAllocateInfo {
	pub s_type: VkStructureType,
	pub next: *const Void,
	pub allocation_size: VkDeviceSize,
	pub memory_type_index: u32,
}

#[repr(C)] pub struct VkExtent3D {
	pub width: u32,
	pub height: u32,
	pub depth: u32,
}

#[repr(C)] pub struct VkImageCreateInfo {
	pub s_type: VkStructureType,
	pub p_next: *const Void,
	pub flags: VkFlags,
	pub image_type: VkImageType,
	pub format: VkFormat,
	pub extent: VkExtent3D,
	pub mip_levels: u32,
	pub array_layers: u32,
	pub samples: VkSampleCount,
	pub tiling: VkImageTiling,
	pub usage: VkImageUsage,
	pub sharing_mode: VkSharingMode,
	pub queue_family_index_count: u32,
	pub p_queue_family_indices: *const u32,
	pub initial_layout: VkImageLayout,
}

#[repr(C)] pub struct VkMemoryType {
	pub property_flags: VkFlags,
	pub heap_index: u32,
}

#[repr(C)] pub struct VkPhysicalDeviceMemoryProperties {
	pub memory_type_count: u32,
	pub memory_types: [VkMemoryType; 32],
	pub memory_heap_count: u32,
	pub memory_heaps: [VkMemoryType; 32],
}

#[repr(C)] pub struct VkImageMemoryBarrier {
	pub s_type: VkStructureType,
	pub p_next: *const Void,
	pub src_access_mask: VkAccess,
	pub dst_access_mask: VkAccess,
	pub old_layout: VkImageLayout,
	pub new_layout: VkImageLayout,
	pub src_queue_family_index: u32,
	pub dst_queue_family_index: u32,
	pub image: VkImage,
	pub subresource_range: VkImageSubresourceRange,
}

#[repr(C)] pub struct VkSubmitInfo {
	pub s_type: VkStructureType,
	pub p_next: *const Void,
	pub wait_semaphore_count: u32,
	pub wait_semaphores: *const VkSemaphore,
	pub wait_dst_stage_mask: *const VkPipelineStage,
	pub command_buffer_count: u32,
	pub p_command_buffers: *const VkCommandBuffer,
	pub signal_semaphore_count: u32,
	pub p_signal_semaphores: *const VkSemaphore,
}

#[repr(C)] pub struct VkBufferMemoryBarrier {
	pub s_type: VkStructureType,
	pub p_next: *const Void,
	pub src_access_mask: VkFlags,
	pub dst_access_mask: VkFlags,
	pub src_queue_family_index: u32,
	pub dst_queue_family_index: u32,
	pub buffer: VkBuffer,
	pub offset: VkDeviceSize,
	pub size: VkDeviceSize,
}

#[repr(C)] pub struct VkMemoryBarrier {
	pub s_type: VkStructureType,
	pub p_next: *const Void,
	pub src_access_mask: VkFlags,
	pub dst_access_mask: VkFlags,
}

#[repr(C)] pub struct VkCommandBufferInheritanceInfo {
	pub s_type: VkStructureType,
	pub p_next: *const Void,
	pub render_pass: VkRenderPass,
	pub subpass: u32,
	pub framebuffer: VkFramebuffer,
	pub occlusion_query_enable: VkBool32,
	pub query_flags: VkFlags,
	pub pipeline_statistics: VkFlags,
}

#[repr(C)] pub struct VkFenceCreateInfo {
	pub s_type: VkStructureType,
	pub p_next: *const Void,
	pub flags: VkFlags,
}

#[repr(C)] pub struct VkCommandBufferBeginInfo {
	pub s_type: VkStructureType,
	pub p_next: *const Void,
	pub flags: VkCommandBufferUsage,
	pub p_inheritance_info: *const VkCommandBufferInheritanceInfo,
}

#[repr(C)] pub struct VkImageSubresourceRange {
	pub aspect_mask: VkImageAspectFlags,
	pub base_mip_level: u32,
	pub level_count: u32,
	pub base_array_layer: u32,
	pub layer_count: u32,
}

#[repr(C)] pub struct VkComponentMapping {
	pub r: VkComponentSwizzle,
	pub g: VkComponentSwizzle,
	pub b: VkComponentSwizzle,
	pub a: VkComponentSwizzle,
}

#[repr(C)] pub struct VkImageViewCreateInfo {
	pub s_type: VkStructureType,
	pub p_next: *const Void,
	pub flags: VkFlags,
	pub image: VkImage,
	pub view_type: VkImageViewType,
	pub format: VkFormat,
	pub components: VkComponentMapping,
	pub subresource_range: VkImageSubresourceRange,
}

#[repr(C)] pub struct VkSwapchainCreateInfoKHR {
	pub s_type: VkStructureType,
	pub p_next: *const Void,
	pub flags: VkFlags,
	pub surface: VkSurfaceKHR,
	pub min_image_count: u32,
	pub image_format: VkFormat,
	pub image_color_space: VkColorSpaceKHR,
	pub image_extent: VkExtent2D,
	pub image_array_layers: u32,
	pub image_usage: VkImageUsage,
        pub image_sharing_mode: VkSharingMode,
	pub queue_family_index_count: u32,
	pub p_queue_family_indices: *const u32,
	pub pre_transform: VkSurfaceTransformFlagBitsKHR,
	pub composite_alpha: VkCompositeAlphaFlagBitsKHR,
	pub present_mode: VkPresentModeKHR,
	pub clipped: VkBool32,
	pub old_swapchain: VkSwapchainKHR,
}

#[repr(C)] pub struct VkExtent2D {
	pub width: u32,
	pub height: u32,
}

#[repr(C)] pub struct VkSurfaceCapabilitiesKHR {
	pub min_image_count: u32,
	pub max_image_count: u32,
	pub current_extent: VkExtent2D,
	pub min_image_extent: VkExtent2D,
	pub max_image_extent: VkExtent2D,
	pub max_image_array_layers: u32,
	pub supported_transforms: VkFlags,
	pub current_transform: VkSurfaceTransformFlagBitsKHR,
	pub supported_composite_alpha: VkFlags,
	pub supported_usage_flags: VkFlags
}

#[repr(C)] pub struct VkSurfaceFormatKHR {
	pub format: VkFormat,
	pub color_space: VkColorSpaceKHR,
}

#[repr(C)] pub struct VkApplicationInfo {
	pub s_type: VkStructureType,
	pub p_next: *mut Void,
	pub p_application_name: *const i8,
	pub application_version: u32,
	pub p_engine_name: *const i8,
	pub engine_version: u32,
	pub api_version: u32,
}

#[repr(C)] pub struct VkInstanceCreateInfo {
	pub s_type: VkStructureType,
	pub p_next: *mut Void,
	pub flags: u32,
	pub p_application_info: *const VkApplicationInfo,
	pub enabled_layer_count: u32,
	pub pp_enabled_layer_names: *const *const i8,
	pub enabled_extension_count: u32,
	pub pp_enabled_extension_names: *const *const i8,
}

#[repr(C)] #[allow(dead_code)] pub enum VkBufferUsage {
	TransferSrcBit = 0x00000001,
	TransferDstBit = 0x00000002,
	UniformTexelBufferBit = 0x00000004,
	StorageTexelBufferBit = 0x00000008,
	UniformBufferBit = 0x00000010,
	StorageBufferBit = 0x00000020,
	IndexBufferBit = 0x00000040,
	VertexBufferBit = 0x00000080,
	IndirectBufferBit = 0x00000100,
}

#[repr(C)] #[allow(dead_code)] pub enum VkDescriptorType {
	Sampler = 0,
	CombinedImageSampler = 1,
	SampledImage = 2,
	StorageImage = 3,
	UniformTexelBuffer = 4,
	StorageTexelBuffer = 5,
	UniformBuffer = 6,
	StorageBuffer = 7,
	UniformBufferDynamic = 8,
	StorageBufferDynamic = 9,
	InputAttachment = 10,
} 

#[repr(C)] #[allow(dead_code)] pub enum VkPipelineBindPoint {
	Graphics = 0,
	Compute = 1,
}

#[repr(C)] #[allow(dead_code)] pub enum VkAttachmentStoreOp {
	Store = 0,
	DontCare = 1,
}

#[repr(C)] #[allow(dead_code)] pub enum VkAttachmentLoadOp {
	Load = 0,
	Clear = 1,
	DontCare = 2,
}

#[repr(C)] #[allow(dead_code)] pub enum VkImageTiling {
	Optimal = 0,
	Linear = 1,
}

#[repr(C)] #[allow(dead_code)] pub enum VkSampleCount {
	Sc1 = 0x00000001,
	Sc2 = 0x00000002,
	Sc4 = 0x00000004,
	Sc8 = 0x00000008,
	Sc16 = 0x00000010,
	Sc32 = 0x00000020,
	Sc64 = 0x00000040,
}

#[repr(C)] #[allow(dead_code)] pub enum VkImageType {
	Dim1d = 0,
	Dim2d = 1,
	Dim3d = 2,
}

#[repr(C)] #[allow(dead_code)] pub enum VkAccess {
	NoFlags = 0x00000000,
	IndirectCommandReadBit = 0x00000001,
	IndexReadBit = 0x00000002,
	VertexAttributeReadBit = 0x00000004,
	UniformReadBit = 0x00000008,
	InputAttachmentReadBit = 0x00000010,
	ShaderReadBit = 0x00000020,
	ShaderWriteBit = 0x00000040,
	ColorAttachmentReadBit = 0x00000080,
	ColorAttachmentWriteBit = 0x00000100,
	DepthStencilAttachmentReadBit = 0x00000200,
	DepthStencilAttachmentWriteBit = 0x00000400,
	TransferReadBit = 0x00000800,
	TransferWriteBit = 0x00001000,
	HostReadBit = 0x00002000,
	HostWriteBit = 0x00004000,
	MemoryReadBit = 0x00008000,
	MemoryWriteBit = 0x00010000,
	// OR'D FLAGS
	DepthStencilAttachmentReadWrite = 0x00000200 | 0x00000400,
}

#[repr(C)] #[allow(dead_code)] pub enum VkImageLayout {
	Undefined = 0,
	General = 1,
	ColorAttachmentOptimal = 2,
	DepthStencilAttachmentOptimal = 3,
	DepthStencilReadOnlyOptimal = 4,
	ShaderReadOnlyOptimal = 5,
	TransferSrcOptimal = 6,
	TransferDstOptimal = 7,
	Preinitialized = 8,
	PresentSrc = 1000001002,
}

#[repr(C)] #[allow(dead_code)] pub enum VkPipelineStage {
	TopOfPipe = 0x00000001,
	DrawIndirect = 0x00000002,
	VertexInput = 0x00000004,
	VertexShader = 0x00000008,
	TessellationControlShader = 0x00000010,
	TessellationEvaluationShader = 0x00000020,
	GeometryShader = 0x00000040,
	FragmentShader = 0x00000080,
	EarlyFragmentTests = 0x00000100,
	LateFragmentTests = 0x00000200,
	ColorAttachmentOutput = 0x00000400,
	ComputeShader = 0x00000800,
	Transfer = 0x00001000,
	BottomOfPipe = 0x00002000,
	Host = 0x00004000,
	AllGraphics = 0x00008000,
	AllCommands = 0x00010000,
}

#[repr(C)] #[allow(dead_code)] pub enum VkCommandBufferUsage {
	OneTimeSubmitBit = 0x00000001,
	RenderPassContinueBit = 0x00000002,
	SimultaneousUseBit = 0x00000004,
}

#[repr(C)] #[allow(dead_code)] pub enum VkImageAspectFlags {
	Color = 0x00000001,
	Depth = 0x00000002,
	Stencil = 0x00000004,
	Metadata = 0x00000008,
}

#[repr(C)] #[allow(dead_code)] pub enum VkImageViewType {
	SingleLayer1d = 0,
	SingleLayer2d = 1,
	SingleLayer3d = 2,
	Cube = 3,
	LayerArray1d = 4,
	LayerArray2d = 5,
	LayerArrayCube = 6,
}

#[repr(C)] #[allow(dead_code)] pub enum VkComponentSwizzle {
	Identity = 0,
	Zero = 1,
	One = 2,
	R = 3,
	G = 4,
	B = 5,
	A = 6,
}

#[repr(C)] #[allow(dead_code)] pub enum VkSharingMode {
	Exclusive = 0,
	Concurrent = 1,
}

#[repr(C)] #[allow(dead_code)] pub enum VkImageUsage {
	TransferSrcBit = 0x00000001,
	TransferDstBit = 0x00000002,
	SampledBit = 0x00000004,
	StorageBit = 0x00000008,
	ColorAttachmentBit = 0x00000010,
	DepthStencilAttachmentBit = 0x00000020,
	TransientAttachmentBit = 0x00000040,
	InputAttachmentBit = 0x00000080,
}

#[repr(C)] #[allow(dead_code)] #[derive(PartialEq, Clone)]
pub enum VkPresentModeKHR {
	Immediate = 0,
	Mailbox = 1,
	Fifo = 2,
	FifoRelaxed = 3,
}

#[repr(C)] #[allow(dead_code)] pub enum VkCompositeAlphaFlagBitsKHR {
	Opaque = 0x00000001,
	PreMultiplied = 0x00000002,
	PostMultiplied = 0x00000004,
	Inherit = 0x00000008,
}

#[repr(C)] #[allow(dead_code)] pub enum VkSurfaceTransformFlagBitsKHR {
	Identity = 0x00000001,
	Rotate90 = 0x00000002,
	Rotate180 = 0x00000004,
	Rotate270 = 0x00000008,
	HorizontalMirror = 0x00000010,
	HorizontalMirrorRotate90 = 0x00000020,
	HorizontalMirrorRotate180 = 0x00000040,
	HorizontalMirrorRotate270 = 0x00000080,
	Inherit = 0x00000100,
}

#[repr(C)] #[allow(dead_code)] pub enum VkColorSpaceKHR {
	SrgbNonlinearKhr = 0,
}

#[repr(C)] #[allow(dead_code)] #[derive(PartialEq, Clone)] pub enum VkFormat {
	Undefined = 0,
	R4g4UnormPack8 = 1,
	R4g4b4a4UnormPack16 = 2,
	B4g4r4a4UnormPack16 = 3,
	R5g6b5UnormPack16 = 4,
	B5g6r5UnormPack16 = 5,
	R5g5b5a1UnormPack16 = 6,
	B5g5r5a1UnormPack16 = 7,
	A1r5g5b5UnormPack16 = 8,
	R8Unorm = 9,
	R8Snorm = 10,
	R8Uscaled = 11,
	R8Sscaled = 12,
	R8Uint = 13,
	R8Sint = 14,
	R8Srgb = 15,
	R8g8Unorm = 16,
	R8g8Snorm = 17,
	R8g8Uscaled = 18,
	R8g8Sscaled = 19,
	R8g8Uint = 20,
	R8g8Sint = 21,
	R8g8Srgb = 22,
	R8g8b8Unorm = 23,
	R8g8b8Snorm = 24,
	R8g8b8Uscaled = 25,
	R8g8b8Sscaled = 26,
	R8g8b8Uint = 27,
	R8g8b8Sint = 28,
	R8g8b8Srgb = 29,
	B8g8r8Unorm = 30,
	B8g8r8Snorm = 31,
	B8g8r8Uscaled = 32,
	B8g8r8Sscaled = 33,
	B8g8r8Uint = 34,
	B8g8r8Sint = 35,
	B8g8r8Srgb = 36,
	R8g8b8a8Unorm = 37,
	R8g8b8a8Snorm = 38,
	R8g8b8a8Uscaled = 39,
	R8g8b8a8Sscaled = 40,
	R8g8b8a8Uint = 41,
	R8g8b8a8Sint = 42,
	R8g8b8a8Srgb = 43,
	B8g8r8a8Unorm = 44,
	B8g8r8a8Snorm = 45,
	B8g8r8a8Uscaled = 46,
	B8g8r8a8Sscaled = 47,
	B8g8r8a8Uint = 48,
	B8g8r8a8Sint = 49,
	B8g8r8a8Srgb = 50,
	A8b8g8r8UnormPack32 = 51,
	A8b8g8r8SnormPack32 = 52,
	A8b8g8r8UscaledPack32 = 53,
	A8b8g8r8SscaledPack32 = 54,
	A8b8g8r8UintPack32 = 55,
	A8b8g8r8SintPack32 = 56,
	A8b8g8r8SrgbPack32 = 57,
	A2r10g10b10UnormPack32 = 58,
	A2r10g10b10SnormPack32 = 59,
	A2r10g10b10UscaledPack32 = 60,
	A2r10g10b10SscaledPack32 = 61,
	A2r10g10b10UintPack32 = 62,
	A2r10g10b10SintPack32 = 63,
	A2b10g10r10UnormPack32 = 64,
	A2b10g10r10SnormPack32 = 65,
	A2b10g10r10UscaledPack32 = 66,
	A2b10g10r10SscaledPack32 = 67,
	A2b10g10r10UintPack32 = 68,
	A2b10g10r10SintPack32 = 69,
	R16Unorm = 70,
	R16Snorm = 71,
	R16Uscaled = 72,
	R16Sscaled = 73,
	R16Uint = 74,
	R16Sint = 75,
	R16Sfloat = 76,
	R16g16Unorm = 77,
	R16g16Snorm = 78,
	R16g16Uscaled = 79,
	R16g16Sscaled = 80,
	R16g16Uint = 81,
	R16g16Sint = 82,
	R16g16Sfloat = 83,
	R16g16b16Unorm = 84,
	R16g16b16Snorm = 85,
	R16g16b16Uscaled = 86,
	R16g16b16Sscaled = 87,
	R16g16b16Uint = 88,
	R16g16b16Sint = 89,
	R16g16b16Sfloat = 90,
	R16g16b16a16Unorm = 91,
	R16g16b16a16Snorm = 92,
	R16g16b16a16Uscaled = 93,
	R16g16b16a16Sscaled = 94,
	R16g16b16a16Uint = 95,
	R16g16b16a16Sint = 96,
	R16g16b16a16Sfloat = 97,
	R32Uint = 98,
	R32Sint = 99,
	R32Sfloat = 100,
	R32g32Uint = 101,
	R32g32Sint = 102,
	R32g32Sfloat = 103,
	R32g32b32Uint = 104,
	R32g32b32Sint = 105,
	R32g32b32Sfloat = 106,
	R32g32b32a32Uint = 107,
	R32g32b32a32Sint = 108,
	R32g32b32a32Sfloat = 109,
	R64Uint = 110,
	R64Sint = 111,
	R64Sfloat = 112,
	R64g64Uint = 113,
	R64g64Sint = 114,
	R64g64Sfloat = 115,
	R64g64b64Uint = 116,
	R64g64b64Sint = 117,
	R64g64b64Sfloat = 118,
	R64g64b64a64Uint = 119,
	R64g64b64a64Sint = 120,
	R64g64b64a64Sfloat = 121,
	B10g11r11UfloatPack32 = 122,
	E5b9g9r9UfloatPack32 = 123,
	D16Unorm = 124,
	X8D24UnormPack32 = 125,
	D32Sfloat = 126,
	S8Uint = 127,
	D16UnormS8Uint = 128,
	D24UnormS8Uint = 129,
	D32SfloatS8Uint = 130,
	Bc1RgbUnormBlock = 131,
	Bc1RgbSrgbBlock = 132,
	Bc1RgbaUnormBlock = 133,
	Bc1RgbaSrgbBlock = 134,
	Bc2UnormBlock = 135,
	Bc2SrgbBlock = 136,
	Bc3UnormBlock = 137,
	Bc3SrgbBlock = 138,
	Bc4UnormBlock = 139,
	Bc4SnormBlock = 140,
	Bc5UnormBlock = 141,
	Bc5SnormBlock = 142,
	Bc6hUfloatBlock = 143,
	Bc6hSfloatBlock = 144,
	Bc7UnormBlock = 145,
	Bc7SrgbBlock = 146,
	Etc2R8g8b8UnormBlock = 147,
	Etc2R8g8b8SrgbBlock = 148,
	Etc2R8g8b8a1UnormBlock = 149,
	Etc2R8g8b8a1SrgbBlock = 150,
	Etc2R8g8b8a8UnormBlock = 151,
	Etc2R8g8b8a8SrgbBlock = 152,
	EacR11UnormBlock = 153,
	EacR11SnormBlock = 154,
	EacR11g11UnormBlock = 155,
	EacR11g11SnormBlock = 156,
	Astc4x4UnormBlock = 157,
	Astc4x4SrgbBlock = 158,
	Astc5x4UnormBlock = 159,
	Astc5x4SrgbBlock = 160,
	Astc5x5UnormBlock = 161,
	Astc5x5SrgbBlock = 162,
	Astc6x5UnormBlock = 163,
	Astc6x5SrgbBlock = 164,
	Astc6x6UnormBlock = 165,
	Astc6x6SrgbBlock = 166,
	Astc8x5UnormBlock = 167,
	Astc8x5SrgbBlock = 168,
	Astc8x6UnormBlock = 169,
	Astc8x6SrgbBlock = 170,
	Astc8x8UnormBlock = 171,
	Astc8x8SrgbBlock = 172,
	Astc10x5UnormBlock = 173,
	Astc10x5SrgbBlock = 174,
	Astc10x6UnormBlock = 175,
	Astc10x6SrgbBlock = 176,
	Astc10x8UnormBlock = 177,
	Astc10x8SrgbBlock = 178,
	Astc10x10UnormBlock = 179,
	Astc10x10SrgbBlock = 180,
	Astc12x10UnormBlock = 181,
	Astc12x10SrgbBlock = 182,
	Astc12x12UnormBlock = 183,
	Astc12x12SrgbBlock = 184,
}

#[repr(C)] #[allow(dead_code)] pub enum VkStructureType {
	ApplicationInfo = 0,
	InstanceCreateInfo = 1,
	DeviceQueueCreateInfo = 2,
	DeviceCreateInfo = 3,
	SubmitInfo = 4,
	MemoryAllocateInfo = 5,
	MappedMemoryRange = 6,
	BindSparseInfo = 7,
	FenceCreateInfo = 8,
	SemaphoreCreateInfo = 9,
	EventCreateInfo = 10,
	QueryPoolCreateInfo = 11,
	BufferCreateInfo = 12,
	BufferViewCreateInfo = 13,
	ImageCreateInfo = 14,
	ImageViewCreateInfo = 15,
	ShaderModuleCreateInfo = 16,
	PipelineCacheCreateInfo = 17,
	PipelineShaderStageCreateInfo = 18,
	PipelineVertexInputStateCreateInfo = 19,
	PipelineInputAssemlyStateCreateInfo = 20,
	PipelineTessellationStateCreateInfo = 21,
	PipelineViewportStateCreateInfo = 22,
	PipelineRasterizationStateCreateInfo = 23,
	PipelineMultisampleStateCreateInfo = 24,
	PipelineDepthStencilStateCreateInfo = 25,
	PipelineColorBlendStateCreateInfo = 26,
	PipelineDynamicStateCreateInfo = 27,
	GraphicsPipelineCreateInfo = 28,
	ComputePipelineCreateInfo = 29,
	PipelineLayoutCreateInfo = 30,
	SamplerCreateInfo = 31,
	DescriptorSetLayoutCreateInfo = 32,
	DescriptorPoolCreateInfo = 33,
	DescriptorSetAllocateInfo = 34,
	WriteDescriptorSet = 35,
	FramebufferCreateInfo = 37,
	RenderPassCreateInfo = 38,
	CommandPoolCreateInfo = 39,
	CommandBufferAllocateInfo = 40,
	CommandBufferInheritanceInfo = 41,
	CommandBufferBeginInfo = 42,
	RenderPassBeginInfo = 43,
	BufferMemoryBarrier = 44,
	ImageMemoryBarrier = 45,
	MemoryBarrier = 46,
	LoaderInstanceCreateInfo = 47,
	LoaderDeviceCreateInfo = 48,
	SwapchainCreateInfo = 1000001000,
	#[cfg(unix)]
	SurfaceCreateInfo = 1000005000, // XCB
	#[cfg(target_os = "windows")]
	SurfaceCreateInfo = 1000009000, // Win32
	#[cfg(target_os = "android")]
	SurfaceCreateInfo = 1000008000, // Android
}

#[repr(C)] #[allow(dead_code)] pub enum VkResult {
	Success = 0,
	NotReady = 1,
	Timeout = 2,
	EventSet = 3,
	EventReset = 4,
	Incomplete = 5,
	OutOfHostMemory = -1,
	OutOfDeviceMemory = -2,
	InitFailed = -3,
	DeviceLost = -4,
	MemoryMapFailed = -5,
	LayerNotPresent = -6,
	ExtNotPresent = -7,
	FeatureNotPresent = -8,
	IncompatDriver = -9,
	TooManyObjects = -10,
	BadFormat = -11,
	FragmentedPool = -12,
	Other = -1024,
}

// // //

impl fmt::Display for VkResult {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {

		VkResult::Success => write!(f, "Success"),
		VkResult::NotReady => write!(f, "Not Ready"),
		VkResult::Timeout => write!(f, "Timeout"),
		VkResult::EventSet => write!(f, "Event Set"),
		VkResult::EventReset => write!(f, "Event Reset"),
		VkResult::Incomplete => write!(f, "Incomplete"),
		VkResult::OutOfHostMemory => write!(f, "Out Of Host Memory"),
		VkResult::OutOfDeviceMemory => write!(f, "Out Of GPU Memory"),
		VkResult::InitFailed => write!(f, "Init Failed"),
		VkResult::DeviceLost => write!(f, "Device Lost"),
		VkResult::MemoryMapFailed => write!(f, "Memory Map Failed"),
		VkResult::LayerNotPresent => write!(f, "Layer Not Present"),
		VkResult::ExtNotPresent => write!(f, "Extension Not Present"),
		VkResult::FeatureNotPresent => write!(f, "Feature Not Present"),
		VkResult::IncompatDriver => write!(f, "Incompatible Driver"),
		VkResult::TooManyObjects => write!(f, "Too Many Objects"),
		VkResult::BadFormat => write!(f, "Format Not Supported"),
		VkResult::FragmentedPool => write!(f, "Fragmented Pool"),
		_ => write!(f, "Unknown Error"),

		}
	}
}
