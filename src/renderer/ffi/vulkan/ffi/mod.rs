// Willow Graphics API
//
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE
//
// src/renderer/ffi/vulkan/ffi/mod.rs

pub mod types;

use self::types::*;

use ami::{ Void, NULL };
use std::{ mem, ptr, u64 };
use std::ffi::CString;

const VERSION: (u32, &'static str) = (4194304, "vulkan 1.0.0");

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
		VkDeviceSize, VkDeviceSize, VkFlags, *mut *mut f32)
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
	}
}

unsafe fn dl_sym<T>(lib: *mut Void, name: &[u8]) -> T {
	let fn_ptr = dlsym(lib, &name[0] as *const _ as *const i8);

	mem::transmute_copy::<*mut Void, T>(&fn_ptr)
}

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
			p_next: NULL.as_mut_ptr(),
			flags: 0,
			p_application_info: &VkApplicationInfo {
				s_type: VkStructureType::ApplicationInfo,
				p_next: NULL.as_mut_ptr(),
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
		}, NULL.as_mut_ptr(), &mut instance
	);

	println!("< WILLOW: App: {}", name);
	println!("< WILLOW: Engine: {}", engine);
	println!("< WILLOW: Backend: {}", VERSION.1);
	println!("< WILLOW: Checks: {}",
		if cfg!(feature = "checks") { "Enabled" } else { "Disabled" }
	);

	instance
}

pub unsafe fn get_gpu(connection: &Connection, instance: VkInstance,
	surface: VkSurfaceKHR) -> (VkPhysicalDevice, u32)
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
				return (gpus[i], k);
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
		p_next: NULL.as_mut_ptr(),
		flags: 0,
		queue_create_info_count: 1,
		p_queue_create_infos: &VkDeviceQueueCreateInfo {
			s_type: VkStructureType::DeviceQueueCreateInfo,
			p_next: NULL.as_mut_ptr(),
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
		enabled_features: NULL.as_mut_ptr(),
	};

	vk_create_device(gpu, &create_info, NULL.as_mut_ptr(), &mut device);

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
		p_next: NULL.as_mut_ptr(),
		flags: 0x00000002, // Reset Command Buffer
		queue_family_index: pqi,
	};

	// Run Function
	vk_create_command_pool(device, &create_info, NULL.as_mut_ptr(),
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
		p_next: NULL.as_mut_ptr(),
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

pub unsafe fn map_memory(connection: &Connection, device: VkDevice,
	vb_memory: VkDeviceMemory, size: u64) -> *mut f32
{
	let mut mapped = ptr::null_mut();

	/*vw_vulkan_error("Failed to test map buffer memory.", */
	(connection.mapmem)(device, vb_memory, 0, size, 0, &mut mapped);
	/*);*/
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

pub unsafe fn cmd_draw(connection: &Connection, cmd_buf: VkCommandBuffer,
	nvertices: u32, ninstances: u32, firstvertex: u32, firstinstance: u32)
	-> ()
{
	(connection.draw)(cmd_buf, nvertices, ninstances, firstvertex,
		firstinstance);
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
	// Load function
	type VkSurfaceInfo = extern "system" fn(
		physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR,
		pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR)
		-> VkResult;
	let vk_surface_info: VkSurfaceInfo = sym(connection,
		b"vkGetPhysicalDeviceSurfaceCapabilitiesKHR\0");

	// Set Data
	let mut surface_info = mem::uninitialized();

	// Run Function
	vk_surface_info(gpu, surface, &mut surface_info);

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
		1 => println!("< WILLOW: Buffering: Single"),
		2 => println!("< WILLOW: Buffering: Double"),
		3 => println!("< WILLOW: Buffering: Triple"),
		_ => panic!("< WILLOW: Image Count: {}", image_count)
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

#[inline(always)] pub(in renderer) unsafe fn create_swapchain(
	connection: &Connection, surface: VkSurfaceKHR, device: VkDevice,
	swapchain: &mut VkSwapchainKHR, width: u32, height: u32,
	image_count: &mut u32, color_format: VkFormat,
	present_mode: VkPresentModeKHR, swap_images: *mut VkImage)
{
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
		ptr::null_mut());
	(connection.get_swapcount)(device, *swapchain, image_count,
		swap_images);
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
	);

	for i in 0..(image_count as usize) {
		(connection.begin_cmdbuff)(
			command_buffer,
			&VkCommandBufferBeginInfo {
				s_type: VkStructureType::CommandBufferBeginInfo,
				p_next: ptr::null(),
				flags: VkCommandBufferUsage::OneTimeSubmitBit,
				p_inheritance_info: ptr::null(),
			}
		);

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

		(connection.end_cmdbuff)(command_buffer);

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
		);

		(connection.wait_fence)(device, 1, submit_fence, 1, u64::MAX);
		(connection.reset_fence)(device, 1, submit_fence);
		(connection.reset_cmdbuff)(command_buffer, 0);

		(connection.create_imgview)(
			device,
			&VkImageViewCreateInfo {
				s_type: VkStructureType::ImageViewCreateInfo,
				p_next: ptr::null(),
				flags: 0,
				view_type: VkImageViewType::SingleLayer2d,
				format: color_format.clone(),
				components: VkComponentMapping {
					r: VkComponentSwizzle::R,
					g: VkComponentSwizzle::G,
					b: VkComponentSwizzle::B,
					a: VkComponentSwizzle::A,
				},
				subresource_range: VkImageSubresourceRange {
					aspect_mask: VkImageAspectFlags::Color,
					base_mip_level: 0,
					level_count: 1,
					base_array_layer: 0,
					layer_count: 1,
				},
				image: swap_images[i],
			},
			ptr::null(),
			&mut image_views[i]
		);
	}
}

#[inline(always)] pub(in renderer) unsafe fn create_depth_buffer(
	connection: &Connection, device: VkDevice, gpu: VkPhysicalDevice,
	command_buffer: VkCommandBuffer, submit_fence: VkFence,
	present_queue: VkQueue, width: u32, height: u32)
	-> (VkImage, VkImageView, VkDeviceMemory)
{
	let mut image = mem::uninitialized();
	let mut image_view = mem::uninitialized();
	let mut image_memory = mem::uninitialized();

	let mut memory_reqs = mem::uninitialized();

	(connection.create_image)(
		device,
		&VkImageCreateInfo {
			s_type: VkStructureType::ImageCreateInfo,
			p_next: ptr::null(),
			flags: 0,
			image_type: VkImageType::Dim2d,
			format: VkFormat::D16Unorm,
			extent: VkExtent3D {
				width: width,
				height: height,
				depth: 1,
			},
			mip_levels: 1,
			array_layers: 1,
			samples: VkSampleCount::Sc1,
			tiling: VkImageTiling::Optimal,
			usage: VkImageUsage::DepthStencilAttachmentBit,
			sharing_mode: VkSharingMode::Exclusive,
			queue_family_index_count: 0,
			p_queue_family_indices: ptr::null(),
			initial_layout: VkImageLayout::Undefined,
		},
		ptr::null(),
		&mut image
	);

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
				0
			),
		},
		ptr::null(),
		&mut image_memory
	);

	(connection.bind_imgmem)(device, image, image_memory, 0);

	// before using this depth buffer we must change it's layout:
	(connection.begin_cmdbuff)(
		command_buffer,
		&VkCommandBufferBeginInfo {
			s_type: VkStructureType::CommandBufferBeginInfo,
			p_next: ptr::null(),
			flags: VkCommandBufferUsage::OneTimeSubmitBit,
			p_inheritance_info: ptr::null(),
		}
	);

	(connection.pipeline_barrier)(
		command_buffer, 
		VkPipelineStage::TopOfPipe, 
		VkPipelineStage::TopOfPipe,
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

	(connection.end_cmdbuff)(command_buffer);

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
	);

	(connection.wait_fence)(device, 1, &submit_fence, 1, u64::MAX);
	(connection.reset_fence)(device, 1, &submit_fence);
	(connection.reset_cmdbuff)(command_buffer, 0);

	// create the depth image view:
	(connection.create_imgview)(
		device,
		&VkImageViewCreateInfo {
			s_type: VkStructureType::ImageViewCreateInfo,
			p_next: ptr::null(),
			flags: 0,
			image: image,
			view_type: VkImageViewType::SingleLayer2d,
			format: VkFormat::D16Unorm,
			components: VkComponentMapping {
				r: VkComponentSwizzle::Identity,
				g: VkComponentSwizzle::Identity,
				b: VkComponentSwizzle::Identity,
				a: VkComponentSwizzle::Identity,
			},
			subresource_range: VkImageSubresourceRange {
				aspect_mask: VkImageAspectFlags::Depth,
				base_mip_level: 0,
				level_count: 1,
				base_array_layer: 0,
				layer_count: 1,
			},
		},
		ptr::null(),
		&mut image_view
	);

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
	let num_writes = tex_count != 0;

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
		if num_writes { 2 } else { 1 },
		if num_writes { &write0 } else { [write0,
			VkWriteDescriptorSet {
				s_type: VkStructureType::WriteDescriptorSet,
				next: ptr::null(),
				dst_set: desc_set,
				dst_binding: 1,
				descriptor_count: tex_count,
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
	const VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT: VkFlags = 0x00000002;
	const VK_MEMORY_PROPERTY_HOST_COHERENT_BIT: VkFlags = 0x00000004;

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
				 descriptor_type: VkDescriptorType::UniformBuffer,
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
	destroy(connection.vk, NULL.as_mut_ptr());
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
	destroy(connection.vk, surface, NULL.as_mut_ptr());
}
