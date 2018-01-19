// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/renderer/ffi/vulkan/mod.rs

use std::mem;

use ami::*;

pub mod vulkan;
// TODO: absorb into ffi, only once internal todo is resolved.
pub mod create_surface;

use asi_vulkan;
use asi_vulkan::types::*;

/*pub struct VulkanRenderer {
	native: vulkan::Vulkan,
}

impl ::RenderOps for VulkanRenderer {
	#[allow(unused)]
	fn new(app_name: &str, window: ::awi::WindowConnection) -> Self {
		let native = vulkan::Vulkan::new(app_name).unwrap();

		VulkanRenderer { native }
	}

	fn update(&self) -> () {
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

pub fn copy_memory<T>(connection: &asi_vulkan::Connection, vk_device: VkDevice,
	vk_memory: VkDeviceMemory, data: *const T, size: usize) where T: Clone
{
	let mut mapped: *mut T = unsafe { mem::uninitialized() };

	unsafe {
		(connection.mapmem)(vk_device, vk_memory, 0, !0, 0,
			&mut mapped as *mut *mut _ as *mut *mut Void).unwrap();
	}

	if mapped.is_null() {
		panic!("Couldn't Map Buffer Memory?  Unknown cause.");
	}

	extern "C" {
		fn memcpy(dest: *mut Void, src: *const Void, n: usize)
			-> MemAddr<Void>;
	}

	unsafe {
		memcpy(cast_mut!(mapped), cast!(data), size);
	}

	unsafe {
//		ptr::copy_nonoverlapping(data.as_ptr(), mapped, data.len());
		asi_vulkan::unmap_memory(connection, vk_device, vk_memory);
	}
}

pub fn copy_memory_pitched<T>(connection: &asi_vulkan::Connection,
	vk_device: VkDevice, vk_memory: VkDeviceMemory, data: &[T],
	width: isize, height: isize, pitch: isize) where T: Clone
{
	let mapped : *mut T = unsafe {
		asi_vulkan::map_memory(connection, vk_device, vk_memory, !0)
	};

	if mapped.is_null() {
		panic!("Couldn't Map Buffer Memory?  Unknown cause.");
	}

	for i in 0..height {
		extern "C" {
			fn memcpy(dest: *mut Void, src: *const Void, n: usize)
				-> MemAddr<Void>;
		}

		unsafe {
			memcpy(cast_mut!(mapped.offset(i * pitch / size_of::<T>() as isize)),
				cast!(data.as_ptr().offset(i * width)),
				width as usize * size_of::<T>());
		}
	}

	unsafe {
		asi_vulkan::unmap_memory(connection, vk_device, vk_memory);
	}
}

pub fn cmd_draw(connection: &asi_vulkan::Connection,
	cmd_buffer: VkCommandBuffer, vertex_count: u32, vertex_offset: i32)
{
	unsafe {
		asi_vulkan::cmd_draw(connection, cmd_buffer, vertex_count, 1, 0,
			vertex_offset, 0);
	}
}
