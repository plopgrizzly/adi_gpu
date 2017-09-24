// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/renderer/ffi/vulkan/ffi/mod.rs

pub mod types;

use self::types::*;

use ami::{ Void };
use std::{ mem, ptr, u64 };
use std::ffi::CString;

const VERSION: (u32, &'static str) = (4194304, "vulkan 1.0.0");

const VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT: VkFlags = 0x00000002;
const VK_MEMORY_PROPERTY_HOST_COHERENT_BIT: VkFlags = 0x00000004;

/*
// Link to Kernel32
extern "system" {
	fn HMODULE WINAPI LoadLibrary(
		_In_ LPCTSTR lpFileName
	);

	fn FARPROC WINAPI GetProcAddress(
		_In_ HMODULE hModule,
		_In_ LPCSTR  lpProcName
	);

	fn BOOL WINAPI FreeLibrary(
		_In_ HMODULE hModule
	);
} */

#[link = "dl"]
extern "C" {
	fn dlopen(filename: *const i8, flag: i32) -> *mut Void;
	fn dlsym(handle: *mut Void, symbol: *const i8) -> *mut Void;
}

pub struct Connection {
	pub vk: VkInstance,
	pub lib: *mut Void,
	vksym: unsafe extern "system" fn(VkInstance, *const i8) -> *mut Void,
	vkdsym: unsafe extern "system" fn(VkDevice, *const i8) -> *mut Void,
	mapmem: unsafe extern "system" fn(VkDevice, VkDeviceMemory,
		VkDeviceSize, VkDeviceSize, VkFlags, *mut *mut Void)
		-> VkResult,
	draw: unsafe extern "system" fn(VkCommandBuffer, u32, u32, u32, u32)
		-> (),
	unmap: unsafe extern "system" fn(VkDevice, VkDeviceMemory) -> (),
	new_swapchain: unsafe extern "system" fn(VkDevice,
		*const VkSwapchainCreateInfoKHR, *const Void,
		*mut VkSwapchainKHR) -> VkResult,
	get_swapcount: unsafe extern "system" fn(VkDevice, VkSwapchainKHR,
		*mut u32, *mut VkImage) -> VkResult,
	create_fence: unsafe extern "system" fn(VkDevice,
		*const VkFenceCreateInfo, *const Void, *mut VkFence)
		-> VkResult,
	begin_cmdbuff: unsafe extern "system" fn(VkCommandBuffer,
		*const VkCommandBufferBeginInfo) -> VkResult,
	pipeline_barrier: unsafe extern "system" fn(VkCommandBuffer,
		VkPipelineStage, VkPipelineStage, VkFlags, u32,
		*const VkMemoryBarrier, u32, *const VkBufferMemoryBarrier, u32,
		*const VkImageMemoryBarrier) -> (),
	end_cmdbuff: unsafe extern "system" fn(VkCommandBuffer) -> VkResult,
	queue_submit: unsafe extern "system" fn(VkQueue, u32,
		*const VkSubmitInfo, VkFence) -> VkResult,
	wait_fence: unsafe extern "system" fn(VkDevice, u32, *const VkFence,
		VkBool32, u64) -> VkResult,
	reset_fence: unsafe extern "system" fn(VkDevice, u32, *const VkFence)
		-> VkResult,
	reset_cmdbuff: unsafe extern "system" fn(VkCommandBuffer, VkFlags),
	create_imgview: unsafe extern "system" fn(VkDevice,
		*const VkImageViewCreateInfo, *const Void, *mut VkImageView)
		-> VkResult,
	get_memprops: unsafe extern "system" fn(VkPhysicalDevice,
		*mut VkPhysicalDeviceMemoryProperties) -> (),
	create_image: unsafe extern "system" fn(VkDevice,
		*const VkImageCreateInfo, *const Void, *mut VkImage)
		-> VkResult,
	get_imgmemreq: unsafe extern "system" fn(VkDevice, VkImage,
		*mut VkMemoryRequirements) -> (),
	mem_allocate: unsafe extern "system" fn(VkDevice,
		*const VkMemoryAllocateInfo, *const Void, *mut VkDeviceMemory)
		-> VkResult,
	bind_imgmem: unsafe extern "system" fn(VkDevice, VkImage,
		VkDeviceMemory, VkDeviceSize) -> VkResult,
	new_renderpass: unsafe extern "system" fn(VkDevice,
		*const VkRenderPassCreateInfo, *const Void, *mut VkRenderPass)
		-> VkResult,
	create_framebuffer: unsafe extern "system" fn(VkDevice,
		*const VkFramebufferCreateInfo, *const Void, *mut VkFramebuffer)
		-> VkResult,
	drop_framebuffer: unsafe extern "system" fn(VkDevice, VkFramebuffer,
		*const Void) -> (),
	drop_imgview: unsafe extern "system" fn(VkDevice, VkImageView,
		*const Void) -> (),
	drop_renderpass: unsafe extern "system" fn(VkDevice, VkRenderPass,
		*const Void) -> (),
	drop_image: unsafe extern "system" fn(VkDevice, VkImage, *const Void)
		-> (),
	drop_memory: unsafe extern "system" fn(VkDevice, VkDeviceMemory,
		*const Void) -> (),
	drop_swapchain: unsafe extern "system" fn(VkDevice, VkSwapchainKHR,
		*const Void) -> (),
	update_descsets: unsafe extern "system" fn(VkDevice, u32,
		*const VkWriteDescriptorSet, u32, *const Void) -> (),
	drop_descsets: unsafe extern "system" fn(VkDevice, VkDescriptorPool,
		u32, *const VkDescriptorSet) -> VkResult,
	drop_descpool: unsafe extern "system" fn(VkDevice, VkDescriptorPool,
		*const Void) -> (),
	drop_buffer: unsafe extern "system" fn(VkDevice, VkBuffer, *const Void)
		-> (),
	bind_buffer_mem: unsafe extern "system" fn(VkDevice, VkBuffer,
		VkDeviceMemory, VkDeviceSize) -> VkResult,
	get_bufmemreq: unsafe extern "system" fn(VkDevice, VkBuffer,
		*mut VkMemoryRequirements) -> (),
	new_buffer: unsafe extern "system" fn(VkDevice,
		*const VkBufferCreateInfo, *const Void, *mut VkBuffer)
		-> VkResult,
	new_descpool: unsafe extern "system" fn(VkDevice,
		*const VkDescriptorPoolCreateInfo, *const Void,
		*mut VkDescriptorPool) -> VkResult,
	new_descsets: unsafe extern "system" fn(VkDevice,
		*const VkDescriptorSetAllocateInfo, *mut VkDescriptorSet)
		-> VkResult,
	new_shademod: unsafe extern "system" fn(VkDevice,
		*const VkShaderModuleCreateInfo, *const Void,
		*mut VkShaderModule) -> VkResult,
	drop_shademod: unsafe extern "system" fn(VkDevice, VkShaderModule,
		*const Void) -> (),
	new_pipeline: unsafe extern "system" fn(VkDevice, VkPipelineCache, u32,
		*const VkGraphicsPipelineCreateInfo, *const Void,
		*mut VkPipeline) -> VkResult,
	new_pipeline_layout: unsafe extern "system" fn(VkDevice,
		*const VkPipelineLayoutCreateInfo, *const Void,
		*mut VkPipelineLayout) -> VkResult,
	new_descset_layout: unsafe extern "system" fn(VkDevice,
		*const VkDescriptorSetLayoutCreateInfo, *const Void,
		*mut VkDescriptorSetLayout) -> VkResult,
	bind_vb: unsafe extern "system" fn(VkCommandBuffer, u32, u32,
		*const VkBuffer, *const VkDeviceSize) -> (),
	bind_pipeline: unsafe extern "system" fn(VkCommandBuffer,
		VkPipelineBindPoint, VkPipeline) -> (),
	bind_descsets: unsafe extern "system" fn(VkCommandBuffer,
		VkPipelineBindPoint, VkPipelineLayout, u32, u32,
		*const VkDescriptorSet, u32, *const u32) -> (),
	new_semaphore: unsafe extern "system" fn(VkDevice,
		*const VkSemaphoreCreateInfo, *const Void, *mut VkSemaphore)
		-> VkResult,
	drop_semaphore: unsafe extern "system" fn(VkDevice, VkSemaphore,
		*const Void) -> (),
	get_next_image: unsafe extern "system" fn(VkDevice, VkSwapchainKHR, u64,
		VkSemaphore, VkFence, *mut u32) -> VkResult,
	copy_image: unsafe extern "system" fn(VkCommandBuffer, VkImage,
		VkImageLayout, VkImage, VkImageLayout, u32, *const VkImageCopy)
		-> (),
	gpu_props: unsafe extern "system" fn(VkPhysicalDevice, VkFormat,
		*mut VkFormatProperties) -> (),
	subres_layout: unsafe extern "system" fn(VkDevice, VkImage,
		*const VkImageSubresource, *mut VkSubresourceLayout) -> (),
	new_sampler: unsafe extern "system" fn(VkDevice,
		*const VkSamplerCreateInfo, *const Void, *mut VkSampler)
		-> VkResult,
	get_surface_capabilities: unsafe extern "system" fn(VkPhysicalDevice,
		VkSurfaceKHR, *mut VkSurfaceCapabilitiesKHR) -> VkResult
}

pub unsafe fn load(app_name: &str) -> Connection {
	let vulkan = b"libvulkan.so.1\0";

	let lib = dlopen(&vulkan[0] as *const _ as *const i8, 1);
	let vksym = dl_sym(lib, b"vkGetInstanceProcAddr\0");
	let vk = create_instance(
		vk_sym(mem::zeroed(), vksym, b"vkCreateInstance\0"), app_name
	);

	Connection {
		vk, lib, vksym,
		vkdsym: vk_sym(vk, vksym, b"vkGetDeviceProcAddr\0"),
		mapmem: vk_sym(vk, vksym, b"vkMapMemory\0"),
		draw: vk_sym(vk, vksym, b"vkCmdDraw\0"),
		unmap: vk_sym(vk, vksym, b"vkUnmapMemory\0"),
		new_swapchain: vk_sym(vk, vksym, b"vkCreateSwapchainKHR\0"),
		get_swapcount: vk_sym(vk, vksym, b"vkGetSwapchainImagesKHR\0"),
		create_fence: vk_sym(vk, vksym, b"vkCreateFence\0"),
		begin_cmdbuff: vk_sym(vk, vksym, b"vkBeginCommandBuffer\0"),
		pipeline_barrier: vk_sym(vk, vksym, b"vkCmdPipelineBarrier\0"),
		end_cmdbuff: vk_sym(vk, vksym, b"vkEndCommandBuffer\0"),
		queue_submit: vk_sym(vk, vksym, b"vkQueueSubmit\0"),
		wait_fence: vk_sym(vk, vksym, b"vkWaitForFences\0"),
		reset_fence: vk_sym(vk, vksym, b"vkResetFences\0"),
		reset_cmdbuff: vk_sym(vk, vksym, b"vkResetCommandBuffer\0"),
		create_imgview: vk_sym(vk, vksym, b"vkCreateImageView\0"),
		get_memprops: vk_sym(vk, vksym,
			b"vkGetPhysicalDeviceMemoryProperties\0"),
		create_image: vk_sym(vk, vksym, b"vkCreateImage\0"),
		get_imgmemreq: vk_sym(vk, vksym,
			b"vkGetImageMemoryRequirements\0"),
		mem_allocate: vk_sym(vk, vksym, b"vkAllocateMemory\0"),
		bind_imgmem: vk_sym(vk, vksym, b"vkBindImageMemory\0"),
		new_renderpass: vk_sym(vk, vksym, b"vkCreateRenderPass\0"),
		create_framebuffer: vk_sym(vk, vksym, b"vkCreateFramebuffer\0"),
		drop_framebuffer: vk_sym(vk, vksym, b"vkDestroyFramebuffer\0"),
		drop_imgview: vk_sym(vk, vksym, b"vkDestroyImageView\0"),
		drop_renderpass: vk_sym(vk, vksym, b"vkDestroyRenderPass\0"),
		drop_memory: vk_sym(vk, vksym, b"vkFreeMemory\0"),
		drop_image: vk_sym(vk, vksym, b"vkDestroyImage\0"),
		drop_swapchain: vk_sym(vk, vksym, b"vkDestroySwapchainKHR\0"),
		update_descsets: vk_sym(vk, vksym, b"vkUpdateDescriptorSets\0"),
		drop_descsets: vk_sym(vk, vksym, b"vkFreeDescriptorSets\0"),
		drop_descpool: vk_sym(vk, vksym, b"vkDestroyDescriptorPool\0"),
		drop_buffer: vk_sym(vk, vksym, b"vkDestroyBuffer\0"),
		bind_buffer_mem: vk_sym(vk, vksym, b"vkBindBufferMemory\0"),
		get_bufmemreq: vk_sym(vk, vksym,
			b"vkGetBufferMemoryRequirements\0"),
		new_buffer: vk_sym(vk, vksym, b"vkCreateBuffer\0"),
		new_descpool: vk_sym(vk, vksym, b"vkCreateDescriptorPool\0"),
		new_descsets: vk_sym(vk, vksym, b"vkAllocateDescriptorSets\0"),
		new_shademod: vk_sym(vk, vksym, b"vkCreateShaderModule\0"),
		drop_shademod: vk_sym(vk, vksym, b"vkDestroyShaderModule\0"),
		new_pipeline: vk_sym(vk, vksym, b"vkCreateGraphicsPipelines\0"),
		new_pipeline_layout:
			vk_sym(vk, vksym, b"vkCreatePipelineLayout\0"),
		new_descset_layout:
			vk_sym(vk, vksym, b"vkCreateDescriptorSetLayout\0"),
		bind_vb: vk_sym(vk, vksym, b"vkCmdBindVertexBuffers\0"),
		bind_pipeline: vk_sym(vk, vksym, b"vkCmdBindPipeline\0"),
		bind_descsets: vk_sym(vk, vksym, b"vkCmdBindDescriptorSets\0"),
		new_semaphore: vk_sym(vk, vksym, b"vkCreateSemaphore\0"),
		drop_semaphore: vk_sym(vk, vksym, b"vkDestroySemaphore\0"),
		get_next_image: vk_sym(vk, vksym, b"vkAcquireNextImageKHR\0"),
		copy_image: vk_sym(vk, vksym, b"vkCmdCopyImage\0"),
		gpu_props: vk_sym(vk, vksym,
			b"vkGetPhysicalDeviceFormatProperties\0"),
		subres_layout:
			vk_sym(vk, vksym, b"vkGetImageSubresourceLayout\0"),
		new_sampler: vk_sym(vk, vksym, b"vkCreateSampler\0"),
		get_surface_capabilities: vk_sym(vk, vksym,
			b"vkGetPhysicalDeviceSurfaceCapabilitiesKHR\0"),
	}
}

unsafe fn dl_sym<T>(lib: *mut Void, name: &[u8]) -> T {
	let fn_ptr = dlsym(lib, &name[0] as *const _ as *const i8);

	mem::transmute_copy::<*mut Void, T>(&fn_ptr)
}

#[inline(always)]
unsafe fn vk_sym<T>(vk: VkInstance, vksym: unsafe extern "system" fn(
	VkInstance, *const i8) -> *mut Void, name: &[u8]) -> T
{
	let fn_ptr = vksym(vk, &name[0] as *const _ as *const i8);

	mem::transmute_copy::<*mut Void, T>(&fn_ptr)
}

unsafe fn vkd_sym<T>(device: VkDevice, vkdsym: unsafe extern "system" fn(
	VkDevice, *const i8) -> *mut Void, name: &[u8]) -> T
{
	let fn_ptr = vkdsym(device, &name[0] as *const _ as *const i8);

	mem::transmute_copy::<*mut Void, T>(&fn_ptr)
}

unsafe fn sym<T>(connection: &Connection, name: &[u8]) -> T {
	vk_sym(connection.vk, connection.vksym, name)
}

unsafe fn dsym<T>(connection: &Connection, device: VkDevice, name: &[u8]) -> T {
	vkd_sym(device, connection.vkdsym, name)
}

unsafe fn create_instance(vk_create_instance: unsafe extern "system" fn(
	*const VkInstanceCreateInfo, *mut Void, *mut VkInstance) -> VkResult,
	name: &str) -> VkInstance
{
	let engine = concat!(
		env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION")
	);

	let program_name : CString = CString::new(name).unwrap();
	let engine_name : CString = CString::new(engine).unwrap();

	// This variables must be defined separately so it stays in scope.
	let validation = CString::new("VK_LAYER_LUNARG_standard_validation")
		.unwrap();
	let s1 = CString::new("VK_KHR_surface").unwrap();
	let s2 = CString::new(
		if cfg!(target_os = "linux") {
			"VK_KHR_xcb_surface"
		} else if cfg!(target_os = "android") {
			"VK_KHR_android_surface"
		} else if cfg!(target_os = "windows") {
			"VK_KHR_win32_surface"
		} else {
			panic!("No suitable surface for this platform.")
		}
	).unwrap();
	let s3 = CString::new("VK_EXT_debug_report").unwrap();

	let mut instance = mem::uninitialized();

	vk_create_instance(
		&VkInstanceCreateInfo {
			s_type: VkStructureType::InstanceCreateInfo,
			p_next: null_mut!(),
			flags: 0,
			p_application_info: &VkApplicationInfo {
				s_type: VkStructureType::ApplicationInfo,
				p_next: null_mut!(),
				p_application_name: program_name.as_ptr(),
				application_version: 2,
				p_engine_name: engine_name.as_ptr(),
				engine_version: 2,
				api_version: VERSION.0,
			},
			enabled_layer_count: {
				if cfg!(feature = "checks") { 1 } else { 0 }
			},
			pp_enabled_layer_names: {
				if cfg!(feature = "checks") {
					&validation.as_ptr()
				} else {
					ptr::null()
				}
			},
			enabled_extension_count: {
				if cfg!(feature = "checks") { 3 } else { 2 }
			},
			pp_enabled_extension_names: {
				if cfg!(feature = "checks") {
					[s1.as_ptr(), s2.as_ptr(), s3.as_ptr()]
						.as_ptr()
				} else {
					[s1.as_ptr(), s2.as_ptr()].as_ptr()
				}	
			},
		}, null_mut!(), &mut instance
	);

	println!("< adi_gpu: App: {}", name);
	println!("< adi_gpu: Engine: {}", engine);
	println!("< adi_gpu: Backend: {}", VERSION.1);
	println!("< adi_gpu: Checks: {}",
		if cfg!(feature = "checks") { "Enabled" } else { "Disabled" }
	);

	instance
}

pub unsafe fn get_gpu(connection: &Connection, instance: VkInstance,
	surface: VkSurfaceKHR) -> (VkPhysicalDevice, u32, bool)
{
	#[repr(C)]
	struct VkQueueFamilyProperties {
		queue_flags: u32,
		queue_count: u32,
		timestamp_valid_bits: u32,
		min_image_transfer_granularity: VkExtent3D,
	}

	// Load Function
	type ListGpus = unsafe extern "system" fn(VkInstance, *mut u32,
		*mut VkPhysicalDevice) -> VkResult;
	let vk_list_gpus: ListGpus = sym(connection,
		b"vkEnumeratePhysicalDevices\0");

	// Set Data
	let mut num_gpus = 0;

	// Run Function
	vk_list_gpus(instance, &mut num_gpus, ptr::null_mut());

	// Set Data
	let mut gpus = vec![mem::uninitialized(); num_gpus as usize];

	// Run function
	vk_list_gpus(instance, &mut num_gpus, gpus.as_mut_ptr());

	// Load functions
	type GetGpuQueueFamProps = unsafe extern "system" fn(VkPhysicalDevice,
		*mut u32, *mut VkQueueFamilyProperties) -> ();
	type GetGpuSurfaceSupport = unsafe extern "system" fn(VkPhysicalDevice,
		u32, VkSurfaceKHR, *mut u32) -> VkResult;

	let vk_get_props: GetGpuQueueFamProps = sym(connection,
		b"vkGetPhysicalDeviceQueueFamilyProperties\0");
	let vk_get_support: GetGpuSurfaceSupport = sym(connection,
		b"vkGetPhysicalDeviceSurfaceSupportKHR\0");

	// Process Data
	for i in 0..(num_gpus as usize) {
		let mut num_queue_families = 0;

		vk_get_props(gpus[i], &mut num_queue_families, ptr::null_mut());

		let queue_families_size = num_queue_families as usize;

		let mut properties = Vec::with_capacity(queue_families_size);

		properties.set_len(queue_families_size);

		vk_get_props(gpus[i], &mut num_queue_families,
			properties.as_mut_ptr());

		for j in 0..queue_families_size {
			let k = j as u32;
			let mut supports_present = 0;

			vk_get_support(gpus[i], k, surface,
				&mut supports_present);

			if supports_present != 0 &&
				(properties[j].queue_flags & 0x00000001) != 0
			{
				// 
				let mut props = mem::uninitialized();

				(connection.gpu_props)(gpus[i],
					VkFormat::R8g8b8a8Unorm, &mut props);

				return (gpus[i], k,
					props.linear_tiling_features &
					0x00000001 /* sampled image bit */ != 0);
			}
		}
	}

	panic!("Couldn't Create Gpu.");
}

pub unsafe fn create_device(connection: &Connection, gpu: VkPhysicalDevice,
	pqi: u32) -> VkDevice
{
	#[repr(C)]
	struct VkDeviceQueueCreateInfo {
		s_type: VkStructureType,
		p_next: *mut Void,
		flags: u32,
		queue_family_index: u32,
		queue_count: u32,
		p_queue_priorities: *const f32,
	}

	#[repr(C)]
	struct VkDeviceCreateInfo {
		s_type: VkStructureType,
		p_next: *mut Void,
		flags: u32,
		queue_create_info_count: u32,
		p_queue_create_infos: *const VkDeviceQueueCreateInfo,
		enabled_layer_count: u32,
		enabled_layer_names: *const *const i8,
		enabled_extension_count: u32,
		enabled_extension_names: *const *const i8,
		enabled_features: *mut Void,
	}

	// Load function
	type VkCreateDevice = extern "system" fn(
		physicalDevice: VkPhysicalDevice,
		pCreateInfo: *const VkDeviceCreateInfo,
		pAllocator: *mut Void,
		pDevice: *mut VkDevice) -> VkResult;
	let vk_create_device: VkCreateDevice = sym(connection,
		b"vkCreateDevice\0");

	// Set Data
	let validation = CString::new("VK_LAYER_LUNARG_standard_validation")
		.unwrap();

	let mut device = mem::uninitialized();
	let ext = CString::new("VK_KHR_swapchain").unwrap();
	let create_info = VkDeviceCreateInfo {
		s_type: VkStructureType::DeviceCreateInfo,
		p_next: null_mut!(),
		flags: 0,
		queue_create_info_count: 1,
		p_queue_create_infos: &VkDeviceQueueCreateInfo {
			s_type: VkStructureType::DeviceQueueCreateInfo,
			p_next: null_mut!(),
			flags: 0,
			queue_family_index: pqi,
			queue_count: 1,
			p_queue_priorities: &1.0,
		},
		enabled_layer_count: {
			if cfg!(feature = "checks") { 1 } else { 0 }
		},
		enabled_layer_names: {
			if cfg!(feature = "checks") {
				&validation.as_ptr()
			} else {
				ptr::null()
			}
		},
		enabled_extension_count: 1,
		enabled_extension_names: &ext.as_ptr(),
		enabled_features: null_mut!(),
	};

	vk_create_device(gpu, &create_info, null_mut!(), &mut device);

	device
}

pub unsafe fn create_queue(connection: &Connection, device: VkDevice, pqi: u32)
	-> VkQueue
{
	// Load function
	type VkGetDeviceQueue = extern "system" fn(device: VkDevice,
		queueFamilyIndex: u32, queueIndex: u32, pQueue: *mut VkQueue)
		-> ();
	let vk_get_device_queue: VkGetDeviceQueue = dsym(connection, device,
		b"vkGetDeviceQueue\0");

	// Set Data
	let mut queue = mem::uninitialized();

	// Run Function
	vk_get_device_queue(device, pqi, 0, &mut queue);

	// Return
	queue
}

pub unsafe fn create_command_buffer(connection: &Connection, device: VkDevice,
	pqi: u32) -> (VkCommandBuffer, u64)
{
	#[repr(C)]
	enum VkCommandBufferLevel {
		Primary = 0,
	}

	#[repr(C)]
	struct VkCommandPoolCreateInfo {
		s_type: VkStructureType,
		p_next: *mut Void,
		flags: u32,
		queue_family_index: u32,
	}

	#[repr(C)]
	struct VkCommandBufferAllocateInfo {
		s_type: VkStructureType,
		p_next: *mut Void,
		command_pool: u64,
		level: VkCommandBufferLevel,
		command_buffer_count: u32,
	}

	// Load function
	type VkCreateCommandPool = extern "system" fn(device: VkDevice,
		pCreateInfo: *const VkCommandPoolCreateInfo,
		pAllocator: *mut Void, pCommandPool: *mut u64) -> VkResult;
	let vk_create_command_pool: VkCreateCommandPool = dsym(connection,
		device, b"vkCreateCommandPool\0");

	// Set Data
	let mut command_pool = 0;
	let mut command_buffer = mem::uninitialized();

	let create_info = VkCommandPoolCreateInfo {
		s_type: VkStructureType::CommandPoolCreateInfo,
		p_next: null_mut!(),
		flags: 0x00000002, // Reset Command Buffer
		queue_family_index: pqi,
	};

	// Run Function
	vk_create_command_pool(device, &create_info, null_mut!(),
		&mut command_pool);

	// Load Function
	type VkAllocateCommandBuffers = extern "system" fn(device: VkDevice,
		ai: *const VkCommandBufferAllocateInfo,
		cmd_buffs: *mut VkCommandBuffer) -> VkResult;
	let vk_allocate_command_buffers: VkAllocateCommandBuffers = dsym(
		connection, device, b"vkAllocateCommandBuffers\0");

	// Set Data
	let allocate_info = VkCommandBufferAllocateInfo {
		s_type: VkStructureType::CommandBufferAllocateInfo,
		p_next: null_mut!(),
		command_pool: command_pool,
		level: VkCommandBufferLevel::Primary,
		command_buffer_count: 1,
	};

	// Run Function
	vk_allocate_command_buffers(device, &allocate_info,
		&mut command_buffer);

	// Return
	(command_buffer, command_pool)
}

pub unsafe fn new_sampler(connection: &Connection, device: VkDevice)
	-> VkSampler
{
	let mut sampler = mem::uninitialized();

	(connection.new_sampler)(
		device,
		&VkSamplerCreateInfo {
			s_type: VkStructureType::SamplerCreateInfo,
			next: ptr::null(),
			flags: 0,
			mag_filter: VkFilter::Nearest,
			min_filter: VkFilter::Nearest,
			mipmap_mode: VkSamplerMipmapMode::Nearest,
			address_mode_u: VkSamplerAddressMode::ClampToEdge,
			address_mode_v: VkSamplerAddressMode::ClampToEdge,
			address_mode_w: VkSamplerAddressMode::ClampToEdge,
			mip_lod_bias: 0.0,
			anisotropy_enable: 0,
			max_anisotropy: 1.0,
			compare_enable: 0,
			compare_op: VkCompareOp::Never,
			min_lod: 0.0,
			max_lod: 0.0,
			border_color: VkBorderColor::FloatOpaqueWhite,
			unnormalized_coordinates: 0,
		},
		ptr::null(),
		&mut sampler
	);

	sampler
}

pub unsafe fn subres_layout(connection: &Connection, device: VkDevice,
	image: VkImage) -> VkSubresourceLayout
{
	let mut layout = mem::uninitialized();

	(connection.subres_layout)(
		device,
		image,
		&VkImageSubresource {
			aspect_mask: VkImageAspectFlags::Color,
			mip_level: 0,
			array_layer: 0,
		},
		&mut layout
	);

	layout
}

pub unsafe fn map_memory<T>(connection: &Connection, device: VkDevice,
	vb_memory: VkDeviceMemory, size: u64) -> *mut T
{
	let mut mapped = mem::uninitialized();

	(connection.mapmem)(device, vb_memory, 0, size, 0,
		&mut mapped as *mut *mut _ as *mut *mut Void);

	mapped
}

pub unsafe fn unmap_memory(connection: &Connection, device: VkDevice,
	vb_memory: VkDeviceMemory) -> ()
{
	(connection.unmap)(device, vb_memory);
}

pub unsafe fn get_memory_type(connection: &Connection, gpu: VkPhysicalDevice,
	mut type_bits: u32, reqs_mask: VkFlags) -> u32
{
	let mut props = mem::uninitialized();
	// TODO; only needs to happen once
	(connection.get_memprops)(gpu, &mut props);

	for i in 0..(props.memory_type_count as usize) {
		// Memory type req's matches vkGetImageMemoryRequirements()?
		if (type_bits & 1) == 1
			&& (props.memory_types[i].property_flags & reqs_mask) ==
				reqs_mask
		{
			return i as u32;
		}
		// Check next bit from vkGetImageMemoryRequirements().
		type_bits >>= 1;
	}

	// Nothing works ... fallback to 0 and hope nothing bad happens.
	panic!(concat!(env!("CARGO_PKG_NAME"),
		"Couldn't find suitable memory type."))
}

pub unsafe fn cmd_bind_descsets(connection: &Connection,
	cmd_buf: VkCommandBuffer, pipeline_layout: VkPipelineLayout,
	desc_set: VkDescriptorSet)
{
	(connection.bind_descsets)(
		cmd_buf,
		VkPipelineBindPoint::Graphics,
		pipeline_layout,
		0,
		1,
		[desc_set].as_ptr(),
		0,
		ptr::null(),
	);
}

pub unsafe fn cmd_bind_pipeline(connection: &Connection,
	cmd_buf: VkCommandBuffer, pipeline: VkPipeline)
{
	(connection.bind_pipeline)(
		cmd_buf,
		VkPipelineBindPoint::Graphics,
		pipeline
	);
}

#[inline(always)] pub unsafe fn cmd_bind_vb(connection: &Connection,
	cmd_buf: VkCommandBuffer, vertex_buffer: VkBuffer,
	texc_buffer: VkBuffer, texc: bool)
{
	if texc {
		(connection.bind_vb)(
			cmd_buf,
			0,
			2,
			[vertex_buffer, texc_buffer].as_ptr(),
			[0, 0].as_ptr(),
		);
	} else {
		(connection.bind_vb)(
			cmd_buf,
			0,
			1,
			[vertex_buffer].as_ptr(),
			[0].as_ptr(),
		);
	}
}

pub unsafe fn cmd_draw(connection: &Connection, cmd_buf: VkCommandBuffer,
	nvertices: u32, ninstances: u32, firstvertex: u32, firstinstance: u32)
	-> ()
{
	(connection.draw)(cmd_buf, nvertices, ninstances, firstvertex,
		firstinstance);
}

pub unsafe fn new_semaphore(connection: &Connection, device: VkDevice)
	-> VkSemaphore
{
	let mut semaphore = mem::uninitialized();

	(connection.new_semaphore)(
		device,
		&VkSemaphoreCreateInfo {
			s_type: VkStructureType::SemaphoreCreateInfo,
			next: ptr::null(),
			flags: 0,
		},
		ptr::null(),
		&mut semaphore,
	);

	semaphore
}

pub unsafe fn drop_semaphore(connection: &Connection, device: VkDevice,
	semaphore: VkSemaphore) -> ()
{
	(connection.drop_semaphore)(
		device,
		semaphore,
		ptr::null(),
	);
}

pub unsafe fn get_next_image(connection: &Connection, device: VkDevice,
	presenting_complete_sem: &mut VkSemaphore, swapchain: VkSwapchainKHR)
	-> u32
{
	let mut image_id = mem::uninitialized();

	let result = (connection.get_next_image)(
		device,
		swapchain,
		u64::MAX,
		*presenting_complete_sem,
		mem::zeroed(),
		&mut image_id,
	);

	if result == VkResult::OutOfDateKhr {
		drop_semaphore(connection, device, *presenting_complete_sem);
		*presenting_complete_sem = new_semaphore(connection, device);

		let result = (connection.get_next_image)(
			device,
			swapchain,
			u64::MAX,
			*presenting_complete_sem,
			mem::zeroed(),
			&mut image_id,
		);

		if result != VkResult::Success {
			panic!("vkAcquireNextImageKHR Failed!");
		}
	} else if result != VkResult::Success {
		panic!("vkAcquireNextImageKHR Failed!");
	}

	image_id
}

pub unsafe fn get_color_format(connection: &Connection, gpu: VkPhysicalDevice,
	surface: VkSurfaceKHR) -> VkFormat
{
	// Load Function
	type VkGetPhysicalDeviceSurfaceFormatsKHR =
		unsafe extern "system" fn(VkPhysicalDevice, VkSurfaceKHR,
			*mut u32, *mut VkSurfaceFormatKHR) -> VkResult;
	let function_name = b"vkGetPhysicalDeviceSurfaceFormatsKHR\0";
	let get_gpu_surface_formats: VkGetPhysicalDeviceSurfaceFormatsKHR
		= sym(connection, function_name);

	// Set Data
	let mut nformats = 1;
	let mut format = mem::uninitialized();

	// Run Function
	get_gpu_surface_formats(gpu, surface, &mut nformats, &mut format);

	// Process data
	if format.format == VkFormat::Undefined {
		VkFormat::B8g8r8Unorm
	} else {
		format.format
	}
}

pub unsafe fn get_buffering(connection: &Connection, gpu: VkPhysicalDevice,
	surface: VkSurfaceKHR) -> u32
{
	// Set Data
	let mut surface_info = mem::uninitialized();

	// Run Function
	(connection.get_surface_capabilities)(gpu, surface, &mut surface_info);

	// Process data
	let min = surface_info.min_image_count;
	let max = surface_info.max_image_count;
	let image_count;

	if min >= max {
		// Gotta use at least the minimum.
		image_count = min;
	}else{
		// If double-buffering isn't supported, use single-buffering.
		if max < 2 {
			image_count = 1;
		} else {
			image_count = 2;
		}
	}

	match image_count {
		1 => println!("< adi_gpu: Buffering: Single"),
		2 => println!("< adi_gpu: Buffering: Double"),
		3 => println!("< adi_gpu: Buffering: Triple"),
		_ => panic!("< adi_gpu: Image Count: {}", image_count)
	}

	image_count
}

pub unsafe fn get_present_mode(connection: &Connection, gpu: VkPhysicalDevice,
	surface: VkSurfaceKHR) -> VkPresentModeKHR
{
	// Load Function
	type VkGetPresentModes = extern "system" fn(VkPhysicalDevice,
		VkSurfaceKHR, *mut u32, *mut VkPresentModeKHR) -> VkResult;
	let vk_get_present_modes: VkGetPresentModes = sym(connection,
		b"vkGetPhysicalDeviceSurfacePresentModesKHR\0");

	// Set Data
	let mut npresentmodes = mem::uninitialized();

	// Run Function
	vk_get_present_modes(gpu, surface, &mut npresentmodes, ptr::null_mut());

	// Set Data
	let npresentmodes_usize = npresentmodes as usize;
	let mut present_modes = vec![mem::uninitialized(); npresentmodes_usize];

	// Run Function
	vk_get_present_modes(gpu, surface, &mut npresentmodes,
		present_modes.as_mut_ptr());

	// Process Data
	for i in 0..npresentmodes_usize {
		if present_modes[i] == VkPresentModeKHR::Mailbox {
			return VkPresentModeKHR::Mailbox; // optimal
		}
	}

	// Fallback
	VkPresentModeKHR::Fifo
}

#[inline(always)] pub(in renderer) unsafe fn copy_image(connection: &Connection,
	cmd_buff: VkCommandBuffer, src_image: VkImage, dst_image: VkImage,
	width: u32, height: u32)
{
	(connection.copy_image)(
		cmd_buff, src_image, VkImageLayout::TransferSrcOptimal,
		dst_image, VkImageLayout::TransferDstOptimal, 1,
		&VkImageCopy {
			src_subresource: VkImageSubresourceLayers {
				aspect_mask: VkImageAspectFlags::Color,
				mip_level: 0,
				base_array_layer: 0,
				layer_count: 1,
			},
			src_offset: VkOffset3D { x: 0, y: 0, z: 0 },
			dst_subresource: VkImageSubresourceLayers {
				aspect_mask: VkImageAspectFlags::Color,
				mip_level: 0,
				base_array_layer: 0,
				layer_count: 1,
			},
			dst_offset: VkOffset3D { x: 0, y: 0, z: 0 },
			extent: VkExtent3D { width, height, depth: 1 },
		}
	);
}

#[inline(always)] pub(in renderer) unsafe fn create_swapchain(
	connection: &Connection, surface: VkSurfaceKHR, gpu: VkPhysicalDevice,
	device: VkDevice, swapchain: &mut VkSwapchainKHR, width: u32,
	height: u32, image_count: &mut u32, color_format: VkFormat,
	present_mode: VkPresentModeKHR, swap_images: *mut VkImage)
{
	(connection.get_surface_capabilities)(gpu, surface,
		&mut mem::uninitialized());

	(connection.new_swapchain)(
		device,
		&VkSwapchainCreateInfoKHR {
			s_type: VkStructureType::SwapchainCreateInfo,
			p_next: ptr::null(),
			flags: 0,
			surface: surface,
			min_image_count: *image_count,
			image_format: color_format,
			image_color_space: VkColorSpaceKHR::SrgbNonlinearKhr,
			image_extent: VkExtent2D { width, height },
			image_array_layers: 1,
			image_usage: VkImageUsage::ColorAttachmentBit,
			image_sharing_mode: VkSharingMode::Exclusive,
			pre_transform: VkSurfaceTransformFlagBitsKHR::Identity,
			composite_alpha: VkCompositeAlphaFlagBitsKHR::Opaque,
			present_mode: present_mode,
			clipped: 1,
			old_swapchain: mem::zeroed(), // vulkan->swapchain,
			queue_family_index_count: 0,
			p_queue_family_indices: ptr::null(),
		},
		ptr::null(),
		swapchain
	);

	(connection.get_swapcount)(device, *swapchain, image_count,
		ptr::null_mut()).unwrap();
	(connection.get_swapcount)(device, *swapchain, image_count,
		swap_images).unwrap();
}

#[inline(always)] pub(in renderer) unsafe fn create_imgview(
	connection: &Connection, device: VkDevice, image: VkImage,
	format: VkFormat, has_color: bool) -> VkImageView
{
	let mut image_view = mem::uninitialized();

	let (components, aspect_mask) = if has_color {
		(
			VkComponentMapping {
				r: VkComponentSwizzle::R,
				g: VkComponentSwizzle::G,
				b: VkComponentSwizzle::B,
				a: VkComponentSwizzle::A,
			},
			VkImageAspectFlags::Color
		)
	} else {
		(
			VkComponentMapping {
				r: VkComponentSwizzle::Identity,
				g: VkComponentSwizzle::Identity,
				b: VkComponentSwizzle::Identity,
				a: VkComponentSwizzle::Identity,
			},
			VkImageAspectFlags::Depth
		)
	};

	(connection.create_imgview)(
		device,
		&VkImageViewCreateInfo {
			s_type: VkStructureType::ImageViewCreateInfo,
			p_next: ptr::null(),
			flags: 0,
			view_type: VkImageViewType::SingleLayer2d,
			format: format.clone(),
			components,
			subresource_range: VkImageSubresourceRange {
				aspect_mask,
				base_mip_level: 0,
				level_count: 1,
				base_array_layer: 0,
				layer_count: 1,
			},
			image,
		},
		ptr::null(),
		&mut image_view
	).unwrap();

	image_view
}

#[inline(always)] pub(in renderer) unsafe fn create_image_view(
	connection: &Connection, device: VkDevice, color_format: &VkFormat,
	submit_fence: &mut VkFence, image_count: u32,
	swap_images: &mut [VkImage; 2], image_views: &mut [VkImageView; 2],
	command_buffer: VkCommandBuffer, present_queue: VkQueue)
{
	(connection.create_fence)(
		device,
		&VkFenceCreateInfo {
			s_type: VkStructureType::FenceCreateInfo,
			p_next: ptr::null(),
			flags: 0,
		},
		ptr::null(),
		submit_fence
	).unwrap();

	for i in 0..(image_count as usize) {
		(connection.begin_cmdbuff)(
			command_buffer,
			&VkCommandBufferBeginInfo {
				s_type: VkStructureType::CommandBufferBeginInfo,
				p_next: ptr::null(),
				flags: VkCommandBufferUsage::OneTimeSubmitBit,
				p_inheritance_info: ptr::null(),
			}
		).unwrap();

		(connection.pipeline_barrier)(
			command_buffer,
			VkPipelineStage::TopOfPipe, 
			VkPipelineStage::TopOfPipe,
			0, 0, ptr::null(), 0, ptr::null(), 1,
			&VkImageMemoryBarrier {
				s_type: VkStructureType::ImageMemoryBarrier,
				p_next: ptr::null(),
				src_access_mask: VkAccess::NoFlags,
				dst_access_mask: VkAccess::MemoryReadBit,
				old_layout: VkImageLayout::Undefined,
				new_layout: VkImageLayout::PresentSrc,
				src_queue_family_index: !0,
				dst_queue_family_index: !0,
				image: swap_images[i],
				subresource_range: VkImageSubresourceRange {
					aspect_mask: VkImageAspectFlags::Color,
					base_mip_level: 0,
					level_count: 1,
					base_array_layer: 0,
					layer_count: 1,
				},
			}
		);

		(connection.end_cmdbuff)(command_buffer).unwrap();

		(connection.queue_submit)(
			present_queue,
			1,
			&VkSubmitInfo {
				s_type: VkStructureType::SubmitInfo,
				p_next: ptr::null(),
				wait_semaphore_count: 0,
				wait_semaphores: ptr::null(),
				wait_dst_stage_mask:
					&VkPipelineStage::ColorAttachmentOutput,
				command_buffer_count: 1,
				p_command_buffers: &command_buffer,
				signal_semaphore_count: 0,
				p_signal_semaphores: ptr::null(),
			},
			*submit_fence
		).unwrap();

		(connection.wait_fence)(device, 1, submit_fence, 1, u64::MAX)
			.unwrap();
		(connection.reset_fence)(device, 1, submit_fence).unwrap();
		(connection.reset_cmdbuff)(command_buffer, 0);

		image_views[i] = create_imgview(connection, device,
			swap_images[i], color_format.clone(), true);
	}
}

#[inline(always)] pub(in renderer) unsafe fn create_image(
	connection: &Connection, device: VkDevice, gpu: VkPhysicalDevice,
	width: u32, height: u32, format: VkFormat, tiling: VkImageTiling,
	usage: VkImageUsage, initial_layout: VkImageLayout, reqs_mask: VkFlags)
	-> (VkImage, VkDeviceMemory)
{
	let mut image = mem::uninitialized();
	let mut image_memory = mem::uninitialized();

	let mut memory_reqs = mem::uninitialized();

	(connection.create_image)(
		device,
		&VkImageCreateInfo {
			s_type: VkStructureType::ImageCreateInfo,
			p_next: ptr::null(),
			flags: 0,
			image_type: VkImageType::Dim2d,
			format,
			extent: VkExtent3D {
				width: width,
				height: height,
				depth: 1,
			},
			mip_levels: 1,
			array_layers: 1,
			samples: VkSampleCount::Sc1,
			tiling,
			usage,
			sharing_mode: VkSharingMode::Exclusive,
			queue_family_index_count: 0,
			p_queue_family_indices: ptr::null(),
			initial_layout,
		},
		ptr::null(),
		&mut image
	).unwrap();

	(connection.get_imgmemreq)(device, image, &mut memory_reqs);

	(connection.mem_allocate)(
		device,
		&VkMemoryAllocateInfo {
			s_type: VkStructureType::MemoryAllocateInfo,
			next: ptr::null(),
			allocation_size: memory_reqs.size,
			memory_type_index: get_memory_type(
				connection,
				gpu,
				memory_reqs.memory_type_bits,
				reqs_mask
			),
		},
		ptr::null(),
		&mut image_memory
	).unwrap();

	(connection.bind_imgmem)(device, image, image_memory, 0).unwrap();

	(image, image_memory)
}

#[inline(always)] pub(in renderer) unsafe fn create_depth_buffer(
	connection: &Connection, device: VkDevice, gpu: VkPhysicalDevice,
	command_buffer: VkCommandBuffer, submit_fence: VkFence,
	present_queue: VkQueue, width: u32, height: u32)
	-> (VkImage, VkImageView, VkDeviceMemory)
{
	let (image, image_memory) = create_image(connection, device, gpu, width,
		height, VkFormat::D16Unorm, VkImageTiling::Optimal,
		VkImageUsage::DepthStencilAttachmentBit,
		VkImageLayout::Undefined, 0);

	// before using this depth buffer we must change it's layout:
	(connection.begin_cmdbuff)(
		command_buffer,
		&VkCommandBufferBeginInfo {
			s_type: VkStructureType::CommandBufferBeginInfo,
			p_next: ptr::null(),
			flags: VkCommandBufferUsage::OneTimeSubmitBit,
			p_inheritance_info: ptr::null(),
		}
	).unwrap();

	(connection.pipeline_barrier)(
		command_buffer, 
		VkPipelineStage::TopOfPipe, 
		VkPipelineStage::TopOfPipeAndEarlyFragmentTests,
		0,
		0,
		ptr::null(),
		0,
		ptr::null(),
		1,
		&VkImageMemoryBarrier {
			s_type: VkStructureType::ImageMemoryBarrier,
			p_next: ptr::null(),
			src_access_mask: VkAccess::NoFlags,
			dst_access_mask:
				VkAccess::DepthStencilAttachmentReadWrite,
			old_layout: VkImageLayout::Undefined,
			new_layout:
				VkImageLayout::DepthStencilAttachmentOptimal,
			src_queue_family_index: !0,
			dst_queue_family_index: !0,
			image: image,
			subresource_range: VkImageSubresourceRange {
				aspect_mask: VkImageAspectFlags::Depth,
				base_mip_level: 0,
				level_count: 1,
				base_array_layer: 0,
				layer_count: 1,
			},
		}
	);

	(connection.end_cmdbuff)(command_buffer).unwrap();

	(connection.queue_submit)(
		present_queue,
		1,
		&VkSubmitInfo {
			s_type: VkStructureType::SubmitInfo,
			p_next: ptr::null(),
			wait_semaphore_count: 0,
			wait_semaphores: ptr::null(),
			wait_dst_stage_mask:
				&VkPipelineStage::ColorAttachmentOutput,
			command_buffer_count: 1,
			p_command_buffers: &command_buffer,
			signal_semaphore_count: 0,
			p_signal_semaphores: ptr::null(),
		},
		submit_fence
	).unwrap();

	(connection.wait_fence)(device, 1, &submit_fence, 1, u64::MAX).unwrap();
	(connection.reset_fence)(device, 1, &submit_fence).unwrap();
	(connection.reset_cmdbuff)(command_buffer, 0);

	// create the depth image view:
	let image_view = create_imgview(connection, device, image,
		VkFormat::D16Unorm, false);

	(image, image_view, image_memory)
}

#[inline(always)] pub(in renderer) unsafe fn create_render_pass(
	connection: &Connection, device: VkDevice, color_format: &VkFormat)
	-> VkRenderPass
{
	let mut render_pass = mem::uninitialized();

	(connection.new_renderpass)(
		device,
		&VkRenderPassCreateInfo {
			s_type: VkStructureType::RenderPassCreateInfo,
			p_next: ptr::null(),
			flags: 0,
			attachment_count: 2,
			attachments: [
				// Color Buffer
				VkAttachmentDescription {
					flags: 0,
					format: color_format.clone(),
					samples: VkSampleCount::Sc1,
					load_op: VkAttachmentLoadOp::Clear,
					store_op: VkAttachmentStoreOp::Store,
					stencil_load_op:
						VkAttachmentLoadOp::DontCare,
					stencil_store_op:
						VkAttachmentStoreOp::DontCare,
					initial_layout:
					  VkImageLayout::ColorAttachmentOptimal,
					final_layout:
					  VkImageLayout::ColorAttachmentOptimal,
				},
				// Depth Buffer
				VkAttachmentDescription {
					flags: 0,
					format: VkFormat::D16Unorm,
					samples: VkSampleCount::Sc1,
					load_op: VkAttachmentLoadOp::Clear,
					store_op: VkAttachmentStoreOp::DontCare,
					stencil_load_op:
						VkAttachmentLoadOp::DontCare,
					stencil_store_op:
						VkAttachmentStoreOp::DontCare,
					initial_layout:
					 VkImageLayout::DepthStencilAttachmentOptimal,
					final_layout:
					 VkImageLayout::DepthStencilAttachmentOptimal,
				},
			].as_ptr(),
			subpass_count: 1,
			subpasses: &VkSubpassDescription {
				flags: 0,
				pipeline_bind_point: VkPipelineBindPoint::Graphics,
				color_attachment_count: 1,
				color_attachments: &VkAttachmentReference {
					attachment: 0,
					layout:
					  VkImageLayout::ColorAttachmentOptimal,
				},
				depth_stencil_attachment: &VkAttachmentReference
				{
					attachment: 1,
					layout:
					 VkImageLayout::DepthStencilAttachmentOptimal,
				},
				input_attachment_count: 0,
				input_attachments: ptr::null(),
				preserve_attachment_count: 0,
				preserve_attachments: ptr::null(),
				resolve_attachments: ptr::null(),
			},
			dependency_count: 0,
			dependencies: ptr::null(),
		},
		ptr::null(),
		&mut render_pass
	);

	render_pass
}

#[inline(always)] pub(in renderer) unsafe fn create_framebuffers(
	connection: &Connection, device: VkDevice, image_count: u32,
	render_pass: VkRenderPass, present_imgviews: &[VkImageView],
	depth_imgview: VkImageView, width: u32, height: u32,
	fbs: &mut[VkFramebuffer])
{
	// create a framebuffer per swap chain imageView:
	for i in 0..(image_count as usize) {
		(connection.create_framebuffer)(
			device,
			&VkFramebufferCreateInfo {
				s_type: VkStructureType::FramebufferCreateInfo,
				p_next: ptr::null(),
				flags: 0,
				attachment_count: 2,
				attachments: [
					present_imgviews[i],
					depth_imgview,
				].as_ptr(),
				layers: 1,
				render_pass, width, height,
			},
			ptr::null(),
			&mut fbs[i]
		);
	}
}

#[inline(always)] pub(in renderer) unsafe fn destroy_swapchain(
	connection: &Connection, device: VkDevice,
	frame_buffers: &[VkFramebuffer], present_imgviews: &[VkImageView],
	depth_imgview: VkImageView, render_pass: VkRenderPass, image_count: u32,
	depth_image: VkImage, swapchain: VkSwapchainKHR,
	depth_image_memory: VkDeviceMemory)
{
	// Free framebuffers & image view #1
	for i in 0..(image_count as usize) {
		(connection.drop_framebuffer)(device, frame_buffers[i],
			ptr::null());
		(connection.drop_imgview)(device, present_imgviews[i],
			ptr::null());
//		(connection.drop_image)(device, present_images[i], ptr::null());
	}
	// Free render pass
	(connection.drop_renderpass)(device, render_pass, ptr::null());
	// Free depth buffer
	(connection.drop_imgview)(device, depth_imgview, ptr::null());
	(connection.drop_image)(device, depth_image, ptr::null());
	(connection.drop_memory)(device, depth_image_memory, ptr::null());
	// Free image view #2
//	vkDestroyFence(vulkan->device, vulkan->submit_fence, NULL);  // TODO: Mem Error
	// Free swapchain
	(connection.drop_swapchain)(device, swapchain, ptr::null());
}

pub(in renderer) unsafe fn txuniform(connection: &Connection, device: VkDevice,
	desc_set: VkDescriptorSet, tex_count: u32, tex_sampler: VkSampler,
	tex_view: VkImageView, matrix_buffer: VkBuffer)
{
	let num_writes = tex_count == 0;

	let write0 = VkWriteDescriptorSet {
		s_type: VkStructureType::WriteDescriptorSet,
		next: ptr::null(),
		dst_set: desc_set,
		descriptor_count: 1,
		descriptor_type: VkDescriptorType::UniformBuffer,
		buffer_info: &VkDescriptorBufferInfo {
			buffer: matrix_buffer,
			offset: 0,
			range: (mem::size_of::<f32>() * 4) as u64,
		},
		dst_array_element: 0,
		texel_buffer_view: ptr::null(),
		dst_binding: 0,
		image_info: ptr::null(),
	};

	(connection.update_descsets)(
		device,
		if num_writes { 1 } else { 1/*2*/ },
		if num_writes { &write0 } else { [/*write0,*/
			VkWriteDescriptorSet {
				s_type: VkStructureType::WriteDescriptorSet,
				next: ptr::null(),
				dst_set: desc_set,
				dst_binding: 0,
				descriptor_count: 1, //tex_count,
				descriptor_type: VkDescriptorType::CombinedImageSampler,
				image_info: &VkDescriptorImageInfo {
					sampler: tex_sampler,
					image_view: tex_view,
					image_layout: VkImageLayout::General,
				},
				buffer_info: ptr::null(),
				dst_array_element: 0,
				texel_buffer_view: ptr::null(),
			}].as_ptr()
		},
		0,
		ptr::null(),
	);
}

pub(in renderer) unsafe fn vw_instance_new(connection: &Connection,
	device: VkDevice, gpu: VkPhysicalDevice, pipeline: ::renderer::Style,
	num_floats: usize, tex_view: VkImageView, tex_sampler: VkSampler,
	tex_count: u32) -> ::renderer::VwInstance
{
	let mut uniform_buffer = mem::uninitialized();
	let mut desc_pool = mem::uninitialized();
	let mut desc_set = mem::uninitialized();
	let mut uniform_memory = mem::uninitialized();
	let mut mem_reqs = mem::uninitialized();

	// Buffers
	(connection.new_buffer)(
		device,
		&VkBufferCreateInfo {
			s_type: VkStructureType::BufferCreateInfo,
			next: ptr::null(),
			flags: 0,
			size: (mem::size_of::<f32>() * num_floats) as u64,
			usage: VkBufferUsage::UniformBufferBit,
			sharing_mode: VkSharingMode::Exclusive,
			queue_family_index_count: 0,
			queue_family_indices: ptr::null(),
		},
		ptr::null(),
		&mut uniform_buffer
	);

	// Descriptor Pool
	(connection.new_descpool)(
		device,
		&VkDescriptorPoolCreateInfo {
			s_type: VkStructureType::DescriptorPoolCreateInfo,
			next: ptr::null(),
			flags: 0,
			max_sets: 1,
			pool_size_count: 1,
			pool_sizes: &VkDescriptorPoolSize {
				descriptor_type: if tex_count == 0 {
					VkDescriptorType::UniformBuffer
				} else {
					VkDescriptorType::CombinedImageSampler
				},
				descriptor_count: 1,
			},
		},
		ptr::null(),
		&mut desc_pool
	);

	(connection.new_descsets)(
		device,
		&VkDescriptorSetAllocateInfo {
			s_type: VkStructureType::DescriptorSetAllocateInfo,
			next: ptr::null(),
			descriptor_pool: desc_pool,
			descriptor_set_count: 1,
			set_layouts: &pipeline.descsetlayout
		},
		&mut desc_set
	);

	// Allocate memory for uniform buffer.
	(connection.get_bufmemreq)(
		device,
		uniform_buffer,
		&mut mem_reqs
	);

	(connection.mem_allocate)(
		device,
		&VkMemoryAllocateInfo {
			s_type: VkStructureType::MemoryAllocateInfo,
			next: ptr::null(),
			allocation_size: mem_reqs.size,
			memory_type_index: get_memory_type(
				connection,
				gpu,
				mem_reqs.memory_type_bits,
				VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT |
				VK_MEMORY_PROPERTY_HOST_COHERENT_BIT),
		},
		ptr::null(),
		&mut uniform_memory
	);

	(connection.bind_buffer_mem)(
		device,
		uniform_buffer,
		uniform_memory,
		0
	);
// }
	txuniform(connection, device, desc_set, tex_count, tex_sampler,
		tex_view, uniform_buffer);

	::renderer::VwInstance {
		matrix_buffer: uniform_buffer,
		uniform_memory,
		desc_set,
		desc_pool,
		pipeline
	}
}

pub(in renderer) unsafe fn new_shape(connection: &Connection, device: VkDevice,
	gpu: VkPhysicalDevice, vertices: &[f32]) -> (VkBuffer, VkDeviceMemory)
{
	let size = (mem::size_of::<f32>() * vertices.len()) as u64;

	let mut vertex_input_buffer = mem::uninitialized();
	let mut vertex_buffer_memory = mem::uninitialized();
	let mut vb_memreqs = mem::uninitialized();

	// Create Vertex Buffer
	(connection.new_buffer)(
		device,
		&VkBufferCreateInfo {
			s_type: VkStructureType::BufferCreateInfo,
			next: ptr::null(),
			flags: 0,
			size: size, // size in Bytes
			usage: VkBufferUsage::VertexBufferBit,
			sharing_mode: VkSharingMode::Exclusive,
			queue_family_index_count: 0,
			queue_family_indices: ptr::null(),
		},
		ptr::null(),
		&mut vertex_input_buffer
	);

	// Allocate memory for vertex buffer.
	(connection.get_bufmemreq)(
		device,
		vertex_input_buffer,
		&mut vb_memreqs,
	);

	(connection.mem_allocate)(
		device,
		&VkMemoryAllocateInfo {
			s_type: VkStructureType::MemoryAllocateInfo,
			next: ptr::null(),
			allocation_size: vb_memreqs.size,
			memory_type_index: get_memory_type(
				connection,
				gpu,
				vb_memreqs.memory_type_bits,
				VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT ),
		},
		ptr::null(),
		&mut vertex_buffer_memory,
	);

	
	// Copy buffer data.
	let mut mapped = mem::uninitialized();

	(connection.mapmem)(
		device,
		vertex_buffer_memory,
		0,
		size,
		0,
		&mut mapped as *mut *mut _ as *mut *mut Void
	);

	ptr::copy_nonoverlapping(vertices.as_ptr(), mapped, vertices.len());

	(connection.unmap)(device, vertex_buffer_memory);

	(connection.bind_buffer_mem)(
		device,
		vertex_input_buffer,
		vertex_buffer_memory,
		0
	);

	(vertex_input_buffer, vertex_buffer_memory)
}

pub(in renderer) unsafe fn new_shader(connection: &Connection, device: VkDevice,
	vertex_shader: &[u8], fragment_shader: &[u8])
	-> (VkShaderModule, VkShaderModule)
{
	let mut vertex = mem::uninitialized();
	let mut fragment = mem::uninitialized();

	// Vertex Shader
	(connection.new_shademod)(
		device,
		&VkShaderModuleCreateInfo {
			s_type: VkStructureType::ShaderModuleCreateInfo,
			next: ptr::null(),
			flags: 0,
			code_size: vertex_shader.len(),
			code: vertex_shader.as_ptr(),
		},
		ptr::null(),
		&mut vertex
	);

	// Fragment Shader
	(connection.new_shademod)(
		device,
		&VkShaderModuleCreateInfo {
			s_type: VkStructureType::ShaderModuleCreateInfo,
			next: ptr::null(),
			flags: 0,
			code_size: fragment_shader.len(),
			code: fragment_shader.as_ptr(),
		},
		ptr::null(),
		&mut fragment
	);

	(vertex, fragment)
}

pub(in renderer) unsafe fn new_pipeline(connection: &Connection,
	device: VkDevice, render_pass: VkRenderPass, width: u32, height: u32,
	shader: ::renderer::Shader) -> ::renderer::Style
{
	let mut pipeline = mem::uninitialized();
	let mut pipeline_layout = mem::uninitialized();
	let mut descsetlayout = mem::uninitialized();

	// depth/stencil config:
	const no_op_stencil_state: VkStencilOpState = VkStencilOpState {
		fail_op: VkStencilOp::Keep,
		pass_op: VkStencilOp::Keep,
		depth_fail_op: VkStencilOp::Keep,
		compare_op: VkCompareOp::Always,
		compare_mask: 0,
		write_mask: 0,
		reference: 0,
	};

	(connection.new_descset_layout)(
		device,
		&VkDescriptorSetLayoutCreateInfo {
			s_type: VkStructureType::DescriptorSetLayoutCreateInfo,
			next: ptr::null(),
			flags: 0,
			binding_count: 1 /*1 + *//*shader.textures*/,
			bindings: [
				if shader.textures == 0 {
					VkDescriptorSetLayoutBinding {
						binding: 0,
						descriptor_type: VkDescriptorType::UniformBuffer,
						descriptor_count: 1,
						stage_flags: VkShaderStage::Vertex,
						immutable_samplers: ptr::null(),
					}
				} else {
					VkDescriptorSetLayoutBinding {
						binding: 0,
						descriptor_type: VkDescriptorType::CombinedImageSampler,
						descriptor_count: 1, // Texture Count
						stage_flags: VkShaderStage::Fragment,
						immutable_samplers: ptr::null(),
					}
				},
			].as_ptr(),
		},
		ptr::null(),
		&mut descsetlayout
	);

	// pipeline layout:
	(connection.new_pipeline_layout)(
		device,
		&VkPipelineLayoutCreateInfo {
			s_type: VkStructureType::PipelineLayoutCreateInfo,
			next: ptr::null(),
			flags: 0,
			set_layout_count: 1,
			set_layouts: [descsetlayout].as_ptr(),
			push_constant_range_count: 0,
			push_constant_ranges: ptr::null(),
		},
		ptr::null(),
		&mut pipeline_layout
	);

	// setup shader stages:
	(connection.new_pipeline)(
		device,
		mem::zeroed(), 
		1,
		&VkGraphicsPipelineCreateInfo {
			s_type: VkStructureType::GraphicsPipelineCreateInfo,
			next: ptr::null(),
			flags: 0,
			stage_count: 2,
			stages: [
				VkPipelineShaderStageCreateInfo {
					s_type: VkStructureType::PipelineShaderStageCreateInfo,
					next: ptr::null(),
					flags: 0,
					stage: VkShaderStage::Vertex,
					module: shader.vertex,
					name: b"main\0".as_ptr(), // shader main function name
					specialization_info: ptr::null(),
				},
				VkPipelineShaderStageCreateInfo {
					s_type: VkStructureType::PipelineShaderStageCreateInfo,
					next: ptr::null(),
					flags: 0,
					stage: VkShaderStage::Fragment,
					module: shader.fragment,
					name: b"main\0".as_ptr(), // shader main function name
					specialization_info: ptr::null(),
				},
			].as_ptr(),
			vertex_input_state: &VkPipelineVertexInputStateCreateInfo {
				s_type: VkStructureType::PipelineVertexInputStateCreateInfo,
				next: ptr::null(),
				flags: 0,
				vertex_binding_description_count: 1 + shader.textures,
				vertex_binding_descriptions: [
					// Vertices
					VkVertexInputBindingDescription {
						binding: 0,
						stride: (mem::size_of::<f32>() * 4) as u32,
						input_rate: VkVertexInputRate::Vertex,
					},
					// Texture Coordinates
					VkVertexInputBindingDescription {
						binding: 1,
						stride: (mem::size_of::<f32>() * 4) as u32,
						input_rate: VkVertexInputRate::Vertex,
					},
				].as_ptr(),
				vertex_attribute_description_count: 1 + shader.textures/*2*/,
				vertex_attribute_descriptions: [
					VkVertexInputAttributeDescription {
						location: 0,
						binding: 0,
						format: VkFormat::R32g32b32a32Sfloat,
						offset: 0,
					},
					VkVertexInputAttributeDescription {
						location: 1,
						binding: 1,
						format: VkFormat::R32g32b32a32Sfloat,
						offset: 0, //4 * sizeof(float),
					},
				].as_ptr(),
			},
			input_assembly_state: &VkPipelineInputAssemblyStateCreateInfo {
				s_type: VkStructureType::PipelineInputAssemblyStateCreateInfo,
				next: ptr::null(),
				flags: 0,
				topology: VkPrimitiveTopology::TriangleList,
				primitive_restart_enable: 0,
			},
			tessellation_state: ptr::null(),
			viewport_state: &VkPipelineViewportStateCreateInfo {
				s_type: VkStructureType::PipelineViewportStateCreateInfo,
				next: ptr::null(),
				flags: 0,
				viewport_count: 1,
				viewports: &VkViewport {
					x: 0.0, y: 0.0,
					width: width as f32,
					height: height as f32,
					min_depth: 0.0,
					max_depth: 1.0,
				},
				scissor_count: 1,
				scissors: &VkRect2D {
					offset: VkOffset2D { x: 0, y: 0 },
					extent: VkExtent2D { width, height },
				},
			},
			rasterization_state: &VkPipelineRasterizationStateCreateInfo {
				s_type: VkStructureType::PipelineRasterizationStateCreateInfo,
				next: ptr::null(),
				flags: 0,
				depth_clamp_enable: 0,
				rasterizer_discard_enable: 0,
				polygon_mode: VkPolygonMode::Fill,
				cull_mode: VkCullMode::Back,
				front_face: VkFrontFace::CounterClockwise,
				depth_bias_enable: 0,
				depth_bias_constant_factor: 0.0,
				depth_bias_clamp: 0.0,
				depth_bias_slope_factor: 0.0,
				line_width: 1.0,
			},
			multisample_state: &VkPipelineMultisampleStateCreateInfo {
				s_type: VkStructureType::PipelineMultisampleStateCreateInfo,
				next: ptr::null(),
				flags: 0,
				rasterization_samples: VkSampleCount::Sc1,
				sample_shading_enable: 0,
				min_sample_shading: 0.0,
				sample_mask: ptr::null(),
				alpha_to_coverage_enable: 0,
				alpha_to_one_enable: 0,
			},
			depth_stencil_state: &VkPipelineDepthStencilStateCreateInfo {
				s_type: VkStructureType::PipelineDepthStencilStateCreateInfo,
				next: ptr::null(),
				flags: 0,
				depth_test_enable: 1,
				depth_write_enable: 1,
				depth_compare_op: VkCompareOp::LessOrEqual,
				depth_bounds_test_enable: 0,
				stencil_test_enable: 0,
				front: no_op_stencil_state,
				back: no_op_stencil_state,
				min_depth_bounds: 0.0,
				max_depth_bounds: 0.0,
			},
			color_blend_state: &VkPipelineColorBlendStateCreateInfo {
				s_type: VkStructureType::PipelineColorBlendStateCreateInfo,
				next: ptr::null(),
				flags: 0,
				logic_op_enable: 0,
				logic_op: VkLogicOp::Clear,
				attachment_count: 1,
				attachments: &VkPipelineColorBlendAttachmentState {
					blend_enable: 1,
					src_color_blend_factor: VkBlendFactor::SrcAlpha,
					dst_color_blend_factor: VkBlendFactor::OneMinusSrcAlpha,
					color_blend_op: VkBlendOp::Add,
					src_alpha_blend_factor: VkBlendFactor::SrcAlpha,
					dst_alpha_blend_factor: VkBlendFactor::One,
					alpha_blend_op: VkBlendOp::Add,
					color_write_mask: 0xf, // RGBA
				},
				blend_constants: [0.0, 0.0, 0.0, 0.0],
			},
			dynamic_state: &VkPipelineDynamicStateCreateInfo {
				s_type: VkStructureType::PipelineDynamicStateCreateInfo,
				next: ptr::null(),
				flags: 0,
				dynamic_state_count: 2,
				dynamic_states: [
					VkDynamicState::Viewport, VkDynamicState::Scissor
				].as_ptr(),
			},
			layout: pipeline_layout,
			render_pass: render_pass,
			subpass: 0,
			base_pipeline_handle: mem::zeroed(), // NULL TODO: ?
			base_pipeline_index: 0,
		},
		ptr::null(),
		&mut pipeline
	);

	(connection.drop_shademod)(
		device,
		shader.vertex,
		ptr::null(),
	);
	(connection.drop_shademod)(
		device,
		shader.fragment,
		ptr::null(),
	);

	::renderer::Style {
		pipeline,
		pipeline_layout,
		descsetlayout,
	}
}

pub unsafe fn destroy_uniforms(connection: &Connection,
	device: VkDevice, uniform_memory: VkDeviceMemory,
	desc_set: VkDescriptorSet, desc_pool: VkDescriptorPool,
	uniform_buffer: VkBuffer) -> ()
{
	(connection.drop_memory)(device, uniform_memory, ptr::null());
	(connection.drop_descsets)(device, desc_pool, 1, &desc_set);
	(connection.drop_descpool)(device, desc_pool, ptr::null());
	(connection.drop_buffer)(device, uniform_buffer, ptr::null());
}

pub unsafe fn destroy_instance(connection: &Connection) -> () {
	// Load Function
	type VkDestroyInstance = unsafe extern "system" fn(instance: VkInstance,
		pAllocator: *mut Void) -> ();
	let function_name = b"vkDestroyInstance\0";
	let destroy: VkDestroyInstance =
		sym(connection, function_name);

	// Run Function
	destroy(connection.vk, null_mut!());
}

pub unsafe fn destroy_surface(connection: &Connection, surface: VkSurfaceKHR)
	-> ()
{
	// Load Function
	type VkDestroySurface = unsafe extern "system" fn(instance: VkInstance,
		surface: VkSurfaceKHR, pAllocator: *mut Void) -> ();
	let destroy: VkDestroySurface = sym(connection,
		b"vkDestroySurfaceKHR\0");

	// Run Function
	destroy(connection.vk, surface, null_mut!());
}
