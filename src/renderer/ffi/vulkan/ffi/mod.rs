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
use std::mem;
use std::ffi::CString;
use std::fmt;
use std::ptr;

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
	mapmem: unsafe extern "system" fn(VkDevice, VkDeviceMemory,
		VkDeviceSize, VkDeviceSize, VkFlags, *mut *mut f32)
		-> VkResult,
	draw: unsafe extern "system" fn(VkCommandBuffer, u32, u32, u32, u32)
		-> (),
	unmap: unsafe extern "system" fn(VkDevice, VkDeviceMemory) -> (),
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
		mapmem: vk_sym(vk, vksym, b"vkMapMemory\0"),
		draw: vk_sym(vk, vksym, b"vkCmdDraw\0"),
		unmap: vk_sym(vk, vksym, b"vkUnmapMemory\0"),
	}
}

unsafe fn dl_sym<T>(lib: *mut Void, name: &[u8]) -> T {
	let fnPtr = dlsym(lib, &name[0] as *const _ as *const i8);

	mem::transmute_copy::<*mut Void, T>(&fnPtr)
}

unsafe fn vk_sym<T>(vk: VkInstance, vksym: unsafe extern "system" fn(
	VkInstance, *const i8) -> *mut Void, name: &[u8]) -> T
{
	let fnPtr = vksym(vk, &name[0] as *const _ as *const i8);

	mem::transmute_copy::<*mut Void, T>(&fnPtr)
}

unsafe fn sym<T>(connection: &Connection, name: &[u8]) -> T {
	vk_sym(connection.vk, connection.vksym, name)
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
	#[derive(Copy, Clone)]
	struct VkExtent3D {
		width: u32,
		height: u32,
		depth: u32,
	}

	#[repr(C)]
	#[derive(Copy, Clone)]
	struct VkQueueFamilyProperties {
		queue_flags: u32,
		queue_count: u32,
		timestamp_valid_bits: u32,
		min_image_transfer_granularity: VkExtent3D,
	}

	// Load Function
	type ListGpus = unsafe extern "system" fn(VkInstance, *mut u32,
		*mut VkPhysicalDevice) -> VkResult;
	let listGPUs: ListGpus = sym(connection,
		b"vkEnumeratePhysicalDevices\0");

	// Set Data
	let mut num_gpus = 0;

	// Run Function
	listGPUs(instance, &mut num_gpus, ptr::null_mut());

	// Set Data
	let mut gpus = vec![mem::uninitialized(); num_gpus as usize];

	// Run function
	listGPUs(instance, &mut num_gpus, gpus.as_mut_ptr());

	// Load functions
	type GetGpuQueueFamProps = unsafe extern "system" fn(VkPhysicalDevice,
		*mut u32, *mut VkQueueFamilyProperties) -> ();
	type GetGpuSurfaceSupport = unsafe extern "system" fn(VkPhysicalDevice,
		u32, VkSurfaceKHR, *mut u32) -> VkResult;

	let getProps: GetGpuQueueFamProps = sym(connection,
		b"vkGetPhysicalDeviceQueueFamilyProperties\0");
	let getSupport: GetGpuSurfaceSupport = sym(connection,
		b"vkGetPhysicalDeviceSurfaceSupportKHR\0");

	// Process Data
	for i in 0..(num_gpus as usize) {
		let mut num_queue_families = 0;

		getProps(gpus[i], &mut num_queue_families, ptr::null_mut());

		let mut properties = vec![VkQueueFamilyProperties {
			queue_flags: 0,
			queue_count: 0,
			timestamp_valid_bits: 0,
			min_image_transfer_granularity: VkExtent3D {
				width: 0, height: 0, depth: 0,
			},
		}; num_queue_families as usize];

		getProps(gpus[i], &mut num_queue_families,
			properties.as_mut_ptr());

		for j in 0..(num_queue_families as usize) {
			let k = j as u32;
			let mut supports_present = 0;

			getSupport(gpus[i], k, surface, &mut supports_present);

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

pub unsafe fn cmd_draw(connection: &Connection, cmdBuf: VkCommandBuffer,
	nvertices: u32, ninstances: u32, firstvertex: u32, firstinstance: u32)
	-> ()
{
	(connection.draw)(cmdBuf, nvertices, ninstances, firstvertex,
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
	if format.format == VkFormat::UNDEFINED {
		VkFormat::B8G8R8_UNORM
	} else {
		format.format
	}
}

pub unsafe fn create_swapchain(connection: &Connection) {
	
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
