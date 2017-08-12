// Willow Graphics API
//
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE
//
// src/renderer/ffi/vulkan/mod.rs

use window::WindowConnection;
use ami::*;

mod ffi;
mod vulkan;

mod create_command_buffer;
mod create_gpu;
mod create_gpu_interface;
mod create_queue;
mod create_surface;
mod destroy;

pub struct VulkanRenderer {
	native: vulkan::Vulkan,
}

impl ::RenderOps for VulkanRenderer {
	fn new(app_name: &str, window: ::window::WindowConnection) -> Self {
		let native = vulkan::Vulkan::new(app_name);

		VulkanRenderer { native }
	}

	fn update(&self) -> () {
	}
}

// VkInstance
pub struct Instance { pub native: vulkan::Vulkan }
impl Instance {
	pub fn create(app_name: &str) -> Instance {
		Instance { native: vulkan::Vulkan::new(app_name) }
	}
}
// TODO: This drop causes crash because Instance goes out of scope when passed
//       to the FFI.
/* impl Drop for Instance {
	fn drop(&mut self) -> () {
		let instance = self.native;

		destroy::instance(instance);
	}
} */

// VkSurface
//#[allow(dead_code)]
pub struct Surface { pub native: u64, instance: *mut Void }
impl Surface {
	pub fn create(instance: &Instance, nwc: WindowConnection) -> Surface
	{
		let instance = instance.native.native();

		Surface {
			native: create_surface::create_surface(instance, nwc),
			instance: instance,
		}
	}
}
// TODO: This drop causes crash because Surface goes out of scope when passed
//       to the FFI.
/* impl Drop for Surface {
	fn drop(&mut self) -> () {
		let surface = self.native;
		let instance = self.instance;

		destroy::surface(instance, surface);
	}
} */

// VkPhysicalDevice
pub struct Gpu { pub native: usize, pub present_queue_index: u32 }
impl Gpu {
	pub fn create(surface: &Surface) -> Gpu {
		let instance = surface.instance;
		let surface = surface.native;
		let gpu = create_gpu::create_gpu(instance, surface);

		Gpu { native: gpu.0, present_queue_index: gpu.1 }
	}
}

// VkDevice
pub struct GpuInterface { pub native: *mut Void }
impl GpuInterface {
	pub fn create(instance: &Instance, gpu: &Gpu) -> GpuInterface {
		let instance = instance.native.native();
		let present_queue_index = gpu.present_queue_index;
		let gpu = gpu.native;

		GpuInterface {
			native: create_gpu_interface::create_gpu_interface(
				instance, gpu, present_queue_index)
		}
	}
}

pub struct Queue { pub native: usize }
impl Queue {
	pub fn create(gpu_interface: &GpuInterface, gpu: &Gpu) -> Queue {
		let present_queue_index = gpu.present_queue_index;
		let gpu_interface = gpu_interface.native;

		Queue {
			native: create_queue::create_queue(gpu_interface,
				present_queue_index)
		}
	}
}

pub struct CommandBuffer { pub native: *mut Void, pub command_pool: u64 }
impl CommandBuffer {
	pub fn create(gpu_interface: &GpuInterface,gpu: &Gpu) -> CommandBuffer {
		let present_queue_index = gpu.present_queue_index;
		let gpu_interface = gpu_interface.native;
		let cmd_buffer = create_command_buffer::create_command_buffer(
			gpu_interface, present_queue_index);

		CommandBuffer{ native: cmd_buffer.0,command_pool: cmd_buffer.1 }
	}
}

use std::fmt;
use std::{ u64, usize };

type VkDeviceMemory = u64;
// type VkDescriptorSet = u64;
// type VkDescriptorSetLayout = u64;
// type VkDescriptorPool = u64;

type VkDevice = *mut Void;
type VkCommandBuffer = *mut Void;

// type VkStructureType = u32; // Size of enum is 4 bytes
// type VkFlags = u32;

// const MAX_ELEMENTS : usize = usize::MAX;
const VK_WHOLE_SIZE : u64 = !0; // Bitwise complement of 0

#[repr(C)]
enum VkStructureType {
	ApplicationInfo = 0,
	InstanceCreateInfo = 1,
	DeviceQueueCreateInfo = 2,
	DeviceCreateInfo = 3,
//	MemoryAllocateInfo = 5,
//	BufferCreateInfo = 12,
//	ImageCreateInfo = 14,
//	ImageViewCreateInfo = 15,
//	PipelineCacheCreateInfo = 17,
//	PipelineLayoutCreateInfo = 30,
//	SamplerCreateInfo = 31,
//	DescriptorSetLayoutCreateInfo = 32,
//	RenderPassCreateInfo = 38,
	CommandPoolCreateInfo = 39,
	CommandBufferAllocateInfo = 40,
//	SwapchainCreateInfo = 1000001000,
#[cfg(unix)]
	SurfaceCreateInfo = 1000005000, // XCB
#[cfg(target_os = "windows")]
	SurfaceCreateInfo = 1000009000, // Win32
#[cfg(target_os = "android")]
	SurfaceCreateInfo = 1000008000, // Android
}

#[repr(C)]
#[allow(dead_code)] // Never used because value set by vulkan.
enum VkResult {
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

/*#[repr(C)]
struct VkDescriptorSetAllocateInfo {
	s_type: VkStructureType,
	p_next: *const VkVoid,
	descriptor_pool: VkDescriptorPool,
	descriptor_set_count: u32,
	p_set_layouts: *const VkDescriptorSetLayout,
}*/

extern {
	fn test_map(vulkan: VkDevice, vertex_buffer_memory: VkDeviceMemory, c: u64) -> *mut f32;

//	fn vkAllocateDescriptorSets(device: VkDevice,
//		pAllocateInfo: *const VkDescriptorSetAllocateInfo,
//		pDescriptorSets: *mut VkDescriptorSet) -> VkResult;
	fn vw_cmd_draw(commandBuffer: VkCommandBuffer, vertexCount: u32,
		instanceCount: u32, firstVertex: u32, firstInstance: u32) -> ();
//	fn vkMapMemory(device: VkDevice, memory: VkDeviceMemory,
//		offset: u64, size: u64, flags: VkFlags,
//		ppData: *mut usize) -> VkResult;
	fn vkUnmapMemory(device: VkDevice, memory: VkDeviceMemory) -> ();
}

#[cfg(feature = "checks")]
fn check_error(name: &str, error: VkResult) {
	match error {
		VkResult::Success => {},
		_ => panic!("{} Failed {}", name, error),
	}
}

#[cfg(not(feature = "checks"))]
fn check_error(_: &str, _: VkResult) { }

pub fn copy_memory(vk_device: VkDevice, vk_memory: VkDeviceMemory, data: &[f32]) {
	// TODO: Figure out why test_map works and not vkMapMemory ffi?
	let mapped = unsafe { test_map(vk_device, vk_memory, VK_WHOLE_SIZE) };
/*	unsafe {
		check_error("Failed to map buffer memory.", vkMapMemory(
			vk_device, vk_memory, 0, VK_WHOLE_SIZE, 0, &mut mapped));
		println!("Mapped {}", mapped);
	}*/
//	let mapped = mapped as *mut _ as *mut f32;
	if mapped.is_null() {
		panic!("Couldn't Map Buffer Memory?  Unknown cause.");
	}
	for i in 0..data.len() {
		unsafe { *(mapped.offset(i as isize)) = data[i]; }
	}
	unsafe {
		vkUnmapMemory(vk_device, vk_memory);
	}
}

pub fn cmd_draw(cmd_buffer: VkCommandBuffer, vertex_count: u32) {
	unsafe {
		vw_cmd_draw(cmd_buffer, vertex_count, 1, 0, 0);
	}
}

/*pub fn allocate_descriptor_sets(device: VkDevice, shape: &Shape, vw: &Vw) -> VkDescriptorSet {
	let mut desc_set : VkDescriptorSet = 0;
	let allocate_info = VkDescriptorSetAllocateInfo {
		s_type: 34,
		p_next: null(),
		descriptor_pool: vw.desc_pool,
		descriptor_set_count: 1,
		p_set_layouts: unsafe { &shape.pipeline.descsetlayout }
	};
	println!("hah");
	check_error("Failed to allocate descriptor sets.", unsafe {
		vkAllocateDescriptorSets(device, &allocate_info, &mut desc_set)
	} );
	println!("ha");
	desc_set
}*/
