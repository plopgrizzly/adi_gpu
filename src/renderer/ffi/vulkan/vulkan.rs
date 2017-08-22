// Willow Graphics API
//
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE
//
// src/renderer/ffi/vulkan/vulkan.rs

use super::ffi as vulkan;
use super::ffi::types::*;

pub struct Vulkan(pub vulkan::Connection);

impl Vulkan {
	pub fn new(app_name: &str) -> Result<Self, String> {
		let connection = unsafe { vulkan::load(app_name) };

		if connection.lib.is_null() {
			return Err("Failed to link to Vulkan.".to_string());
		}

		Ok(Vulkan(connection))
	}

	pub fn native(&self) -> VkInstance {
		self.0.vk
	}
}
