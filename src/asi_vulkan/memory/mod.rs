// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/memory/mod.rs

mod buffer;

use std::{ mem, ptr };
use ami::Void;

use super::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT;
use super::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT;
use super::Connection;
use super::types::*;

pub struct Memory<T> where T: Clone {
	pub data: T,
	pub memory: VkDeviceMemory,
	pub buffer: buffer::Buffer,
	device: VkDevice,
	dropfn: unsafe extern "system" fn(VkDevice, VkDeviceMemory, *const Void)
		-> ()
}

impl<T> Memory<T> where T: Clone {
	/// Allocate memory in a GPU buffer.
	#[inline(always)]
	pub fn new(c: &Connection, device: VkDevice, gpu: VkPhysicalDevice,
		data: T) -> Memory<T>
	{
		let buffer = buffer::Buffer::new(c, device, mem::size_of::<T>());
		let mut memory = unsafe { mem::uninitialized() };
		let mem_reqs = buffer.get_reqs(c, device);
		unsafe {
			(c.mem_allocate)(
				device,
				&VkMemoryAllocateInfo {
					s_type: VkStructureType::MemoryAllocateInfo,
					next: ptr::null(),
					allocation_size: mem_reqs.size,
					memory_type_index: super::get_memory_type(
						c,
						gpu,
						mem_reqs.memory_type_bits,
						VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT |
						VK_MEMORY_PROPERTY_HOST_COHERENT_BIT),
				},
				ptr::null(),
				&mut memory
			).unwrap();
		}
		let dropfn = unsafe {
			super::vkd_sym(device, c.vkdsym, b"vkFreeMemory\0")
		};

		unsafe {
			(c.bind_buffer_mem)(device, buffer.buffer, memory, 0)
				.unwrap();
		}

		let memory = Memory { data, memory, buffer, device, dropfn };

		memory.update(c);

		memory
	}

	/// Update the contents of the memory.
	#[inline(always)]
	pub fn update(&self, c: &Connection) {
		let size = mem::size_of::<T>();
		let mut mapped: *mut T = unsafe { mem::uninitialized() }; // void *

		unsafe {
			(c.mapmem)(self.device, self.memory, 0, !0, 0,
				&mut mapped as *mut *mut _ as *mut *mut Void).unwrap();
		}

		if mapped.is_null() {
			panic!("Couldn't Map Buffer Memory?  Unknown cause.");
		}

		let write = self.data.clone();

		unsafe {
			*mapped = write;
		}

//		let mut map = mapped as *mut _ as *mut T;

//		unsafe {
//			*map = self.data.clone();
//		}

/*		extern "C" {
			fn memcpy(dest: *mut Void, src: *const Void, n: usize)
				-> ::ami::MemAddr<Void>;
		}*/

		unsafe {
	//		memcpy(mapped, cast!(&self.data), size);
	//		ptr::copy_nonoverlapping(data.as_ptr(), mapped, data.len());

			(c.unmap)(self.device, self.memory);

//			asi_vulkan::unmap_memory(c, self.device, self.memory);
		}
	}

	#[inline(always)]
	/// Update the contents of image memory.
	pub fn update_pitched() {
		
	}
}

impl<T> Drop for Memory<T> where T: Clone {
	#[inline(always)]
	fn drop(&mut self) {
		// TODO: Drop at correct time as for no segfault.
/*		unsafe {
			(self.dropfn)(self.device, self.memory, ptr::null());
		}
		self.buffer.drop(self.device);*/
	}
}
