// Willow Graphics API
//
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE
//
// src/renderer/ffi/vulkan/vulkan.rs

use ami::Void;
use super::ffi as vulkan;
use std::ptr;

pub struct Vulkan(pub vulkan::Connection);

impl Vulkan {
	pub fn new(app_name: &str) -> Self {
		let dl = unsafe { vulkan::load_dl() };

		if dl.dl_handle.is_null() {
			return Vulkan((ptr::null_mut(), dl));
		}

		let instance = unsafe {
			vulkan::create_instance(dl.dl_handle, app_name)
		};

		Vulkan((instance, dl))
	}

	pub fn native(&self) -> *mut Void {
		(self.0).0
	}
}
