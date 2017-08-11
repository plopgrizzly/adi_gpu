// Willow Graphics API
//
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE
//
// src/renderer/ffi/vulkan/create_instance.rs

use ami::*;
use std::ffi::CString;
use super::{ VkResult, VkStructureType, check_error };

const VULKAN_VERSION : (u32, &'static str) = (4194304, "VK_API_VERSION_1_0");

#[cfg(any(target_os = "linux", target_os = "macos"))]
const EXTENSION : &'static str = "VK_KHR_xcb_surface";
#[cfg(target_os = "android")]
const EXTENSION : &'static str = "VK_KHR_android_surface";
#[cfg(target_os = "windows")]
const EXTENSION : &'static str = "VK_KHR_win32_surface";

#[cfg(feature = "checks")]
const NUM_EXTENSIONS : u32 = 3;
#[cfg(not(feature = "checks"))]
const NUM_EXTENSIONS : u32 = 2;

#[cfg(feature = "checks")]
const NUM_LAYERS : u32 = 1;
#[cfg(not(feature = "checks"))]
const NUM_LAYERS : u32 = 0;

#[cfg(feature = "checks")]
const CHECKS : &'static str = "Yes ( Debug )";
#[cfg(not(feature = "checks"))]
const CHECKS : &'static str = "No ( Release )";

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
	pp_enabled_layer_names: *const [*const i8; NUM_LAYERS as usize],
	enabled_extension_count: u32,
	pp_enabled_extension_names: *const [*const i8; NUM_EXTENSIONS as usize],
}

fn required_extensions() -> (CString, CString) {
	let s1 = CString::new("VK_KHR_surface").unwrap();
	let s2 = CString::new(EXTENSION).unwrap();
	(s1, s2)
}

#[cfg(feature = "checks")]
fn extensions() -> (CString, CString, CString) {
	let req = required_extensions();
	let s3 = CString::new("VK_EXT_debug_report").unwrap();
	(req.0, req.1, s3)
}

#[cfg(feature = "checks")]
fn extension_names(ext: &(CString, CString, CString))
	-> [*const i8; NUM_EXTENSIONS as usize]
{
	[ ext.0.as_ptr(), ext.1.as_ptr(), ext.2.as_ptr() ]
}

#[cfg(feature = "checks")]
fn layers() -> (CString) {
	let s1 = CString::new("VK_LAYER_LUNARG_standard_validation").unwrap();
	(s1)
}

#[cfg(feature = "checks")]
fn layer_names(layer: &(CString)) -> [*const i8; NUM_LAYERS as usize] {
	[ layer.as_ptr() ]
}

#[cfg(not(feature = "checks"))]
fn extensions() -> (CString, CString) {
	required_extensions()
}

#[cfg(not(feature = "checks"))]
fn extension_names(ext: &(CString, CString))
	-> [*const i8; NUM_EXTENSIONS as usize]
{
	[ ext.0.as_ptr(), ext.1.as_ptr() ]
}

#[cfg(not(feature = "checks"))]
fn layers() -> ( ) {
	( )
}

#[cfg(not(feature = "checks"))]
fn layer_names(_: &()) -> [*const i8; NUM_LAYERS as usize] {
	[ ]
}

pub fn create_instance(app_name: &str) -> *mut Void {
	let version = concat!(env!("CARGO_PKG_NAME"), " ",
		env!("CARGO_PKG_VERSION"));

	let mut instance = NULL.as_mut_ptr();

	let program_name : CString = CString::new(app_name).unwrap();
	let engine_name : CString = CString::new(version).unwrap();

	// This variables must be defined separately so it stays in scope.
	let ext = extensions();
	let lay = layers();

	let instance_create_info = VkInstanceCreateInfo {
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
			api_version: VULKAN_VERSION.0,
		},
		enabled_layer_count: 0,
		pp_enabled_layer_names: &layer_names(&lay),
		enabled_extension_count: NUM_EXTENSIONS,
		pp_enabled_extension_names: &extension_names(&ext),
	};

	unsafe {
		extern "system" {
			fn vkGetInstanceProcAddr(instance: *mut Void,
				name: *const i8)
			-> extern "system" fn(
				pCreateInfo: *const VkInstanceCreateInfo,
				pAllocator: *mut Void,
				pInstance: *mut *mut Void) -> VkResult;
		}
		let name = CString::new("vkCreateInstance").unwrap();
		check_error("Failed to create vulkan instance.",
			(vkGetInstanceProcAddr(NULL.as_mut_ptr(), name.as_ptr()))
			(&instance_create_info, NULL.as_mut_ptr(), &mut instance)
		);
	};

	println!("adi_screen: Program: {}", app_name);
	println!("adi_screen: Engine: {}", version);
	println!("adi_screen: Backend: {}", VULKAN_VERSION.1);
	println!("adi_screen: Checks: {}", CHECKS);
	
	instance
}
