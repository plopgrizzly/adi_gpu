// Willow Graphics API
//
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE
//
// src/renderer/ffi/vulkan/mod.rs

use std::ptr;

use window::WindowConnection;
use ami::*;

pub mod ffi;
pub mod vulkan;
// TODO: absorb into ffi, only once internal todo is resolved.
pub mod create_surface;

use self::ffi::types::*;

pub struct VulkanRenderer {
	native: vulkan::Vulkan,
}

impl ::RenderOps for VulkanRenderer {
	fn new(app_name: &str, window: ::window::WindowConnection) -> Self {
		let native = vulkan::Vulkan::new(app_name).unwrap();

		VulkanRenderer { native }
	}

	fn update(&self) -> () {
	}
}

/*pub struct Queue { pub native: usize }
impl Queue {
	pub fn create(gpu_interface: VkDevice, gpu: VkPhysicalDevice,
		pqi: u32) -> Queue
	{
		Queue {
			native: create_queue::create_queue(gpu_interface, pqi)
		}
	}
}*/

/*pub struct CommandBuffer { pub native: VkCommandBuffer, pub command_pool: u64 }
impl CommandBuffer {
	pub fn create(gpu_interface: VkDevice, gpu: VkPhysicalDevice,
		pqi: u32) -> CommandBuffer
	{
		let cmd_buffer = create_command_buffer::create_command_buffer(
			gpu_interface, pqi);

		CommandBuffer{ native: cmd_buffer.0,command_pool: cmd_buffer.1 }
	}
}*/

// TODO: MAKE SURE WINDOWS DOESNT BREAK WITHOUT THIS
/* extern {
	fn test_map(vulkan: VkDevice, vertex_buffer_memory: VkDeviceMemory, c: u64) -> *mut f32;
}*/

#[cfg(feature = "checks")]
fn check_error(name: &str, error: VkResult) {
	match error {
		VkResult::Success => {},
		_ => panic!("{} Failed {}", name, error),
	}
}

#[cfg(not(feature = "checks"))]
fn check_error(_: &str, _: VkResult) { }

pub fn copy_memory(connection: &ffi::Connection, vk_device: VkDevice,
	vk_memory: VkDeviceMemory, data: &[f32])
{
	let mapped = unsafe {
		ffi::map_memory(connection, vk_device, vk_memory,
			(data.len() * size_of::<f32>()) as u64)
	};

	if mapped.is_null() {
		panic!("Couldn't Map Buffer Memory?  Unknown cause.");
	}

	unsafe {
		ptr::copy_nonoverlapping(data.as_ptr(), mapped, data.len());
		ffi::unmap_memory(connection, vk_device, vk_memory);
	}
}

pub fn cmd_draw(connection: &ffi::Connection, cmd_buffer: VkCommandBuffer,
	vertex_count: u32)
{
	unsafe {
		ffi::cmd_draw(connection, cmd_buffer, vertex_count, 1, 0, 0);
	}
}
