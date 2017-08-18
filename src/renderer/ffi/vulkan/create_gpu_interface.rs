// Willow Graphics API
//
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE
//
// src/renderer/ffi/vulkan/create_gpu_interface.rs

use ami::*;
use std::ffi::CString;
use super::{ VkResult, VkStructureType, check_error };

#[cfg(feature = "checks")]
const NUM_LAYERS : u32 = 1;
#[cfg(not(feature = "checks"))]
const NUM_LAYERS : u32 = 0;

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
	enabled_layer_names: *const [*const i8; NUM_LAYERS as usize],
	enabled_extension_count: u32,
	enabled_extension_names: *const *const i8,
	enabled_features: *mut Void,
}

extern {
//	fn vkCreateDevice(;
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
fn layers() -> ( ) {
	( )
}

#[cfg(not(feature = "checks"))]
fn layer_names(_: &()) -> [*const i8; NUM_LAYERS as usize] {
	[ ]
}

pub fn create_gpu_interface(instance: *mut Void, gpu: *mut Void,
	present_queue_index: u32) -> *mut Void
{
	let mut device = NULL.as_mut_ptr();
	let ext = CString::new("VK_KHR_swapchain").unwrap();
	let lay = layers();
	let create_info = VkDeviceCreateInfo {
		s_type: VkStructureType::DeviceCreateInfo,
		p_next: NULL.as_mut_ptr(),
		flags: 0,
		queue_create_info_count: 1,
		p_queue_create_infos: &VkDeviceQueueCreateInfo {
			s_type: VkStructureType::DeviceQueueCreateInfo,
			p_next: NULL.as_mut_ptr(),
			flags: 0,
			queue_family_index: present_queue_index,
			queue_count: 1,
			p_queue_priorities: &1.0,
		},
		enabled_layer_count: NUM_LAYERS,
		enabled_layer_names: &layer_names(&lay),
		enabled_extension_count: 1,
		enabled_extension_names: &ext.as_ptr(),
		enabled_features: NULL.as_mut_ptr(),
	};

	unsafe {
		extern "system" {
			fn vkGetInstanceProcAddr(instance: *mut Void,
				name: *const i8)
			-> extern "system" fn(
				physicalDevice: *mut Void,
				pCreateInfo: *const VkDeviceCreateInfo,
				pAllocator: *mut Void,
				pDevice: *mut *mut Void) -> VkResult;
		}
		let name = CString::new("vkCreateDevice").unwrap();
		check_error("vkCreateDevice failure.",
			(vkGetInstanceProcAddr(instance, name.as_ptr()))
			(gpu, &create_info, NULL.as_mut_ptr(), &mut device)
		);
	};

	device
}
