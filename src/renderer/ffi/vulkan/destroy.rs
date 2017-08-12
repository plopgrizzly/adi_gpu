// Willow Graphics API
//
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE
//
// src/renderer/ffi/vulkan/destroy.rs

use ami::*;

extern {
	fn vkDestroyInstance(instance: usize, pAllocator: *mut Void) -> ();
	fn vkDestroySurfaceKHR(instance: usize, surface: u64,
		pAllocator: *mut Void) -> ();
}

#[allow(dead_code)]
pub fn instance(instance: usize) -> () {
	unsafe {
		vkDestroyInstance(instance, NULL.as_mut_ptr());
	}
}

#[allow(dead_code)]
pub fn surface(instance: usize, surface: u64) -> () {
	unsafe {
		vkDestroySurfaceKHR(instance, surface, NULL.as_mut_ptr());
	}
}
