// Willow Graphics API
//
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE
//
// src/renderer/ffi/vulkan/destroy.rs

use ami::void_pointer::*;

extern {
	fn vkDestroyInstance(instance: usize, pAllocator: VoidPointer) -> ();
	fn vkDestroySurfaceKHR(instance: usize, surface: u64,
		pAllocator: VoidPointer) -> ();
}

#[allow(dead_code)]
pub fn instance(instance: usize) -> () {
	unsafe {
		vkDestroyInstance(instance, NULL);
	}
}

#[allow(dead_code)]
pub fn surface(instance: usize, surface: u64) -> () {
	unsafe {
		vkDestroySurfaceKHR(instance, surface, NULL);
	}
}
