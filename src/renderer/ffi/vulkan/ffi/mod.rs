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

pub struct Dl {
	pub dl_handle: *mut Void,
	vk_sym: unsafe extern "system" fn(instance: *mut Void, name: *const i8)
		-> *mut Void,
}

pub type Connection = (*mut Void, Dl);

pub unsafe fn load_dl() -> Dl {
	let vulkan = b"libvulkan.so.1\0";

	let dl_handle = dlopen(&vulkan[0] as *const _ as *const i8, 1);

	Dl {
		dl_handle: dl_handle,
		vk_sym: dl_sym(dl_handle, b"vkGetInstanceProcAddr\0"),
	}
}

unsafe fn dl_sym<T>(lib: *mut Void, name: &[u8]) -> T {
	let function_ptr = dlsym(lib, &name[0] as *const _ as *const i8);
	mem::transmute_copy::<*mut Void, T>(&function_ptr)
}

unsafe fn vk_sym<T>(connection: Connection, name: &[u8]) -> T {
	let function_ptr = (connection.1.vk_sym)(connection.0,
		&name[0] as *const _ as *const i8);
	mem::transmute_copy::<*mut Void, T>(&function_ptr)
}

pub unsafe fn create_instance(dl: *mut Void, name: &str) -> VkInstance {
	#[repr(C)]
	struct VkApplicationInfo {
		s_type: VkStructureType,
		p_next: *mut Void,
		p_application_name: *const i8,
		application_version: u32,
		p_engine_name: *const i8,
		engine_version: u32,
		api_version: u32,
	}

	#[repr(C)]
	struct VkInstanceCreateInfo {
		s_type: VkStructureType,
		p_next: *mut Void,
		flags: u32,
		p_application_info: *const VkApplicationInfo,
		enabled_layer_count: u32,
		pp_enabled_layer_names: *const *const i8,
		enabled_extension_count: u32,
		pp_enabled_extension_names: *const *const i8,
	}

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

	// Get & Run vkCreateInstance
	let vk_create_instance: unsafe extern "system" fn(
		pCreateInfo: *const VkInstanceCreateInfo,
		pAllocator: *mut Void,
		pInstance: *mut VkInstance) -> VkResult
		= dl_sym(dl, b"vkCreateInstance\0");

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

pub unsafe fn get_color_format(connection: &Connection, gpu: VkPhysicalDevice,
	surface: VkSurfaceKHR) -> VkFormat
{
	// Load Function
	type VkGetPhysicalDeviceSurfaceFormatsKHR =
		unsafe extern "system" fn(VkPhysicalDevice, VkSurfaceKHR,
			*mut u32, *mut VkSurfaceFormatKHR) -> VkResult;
	let function_name = b"vkGetPhysicalDeviceSurfaceFormatsKHR\0";
	let get_gpu_surface_formats: VkGetPhysicalDeviceSurfaceFormatsKHR
		= dl_sym(connection.1.dl_handle, function_name);

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
