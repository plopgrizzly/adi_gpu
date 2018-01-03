// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/renderer/ffi/vulkan/create_surface.rs

// TODO: Make surface a buffer and blit onto screen with window manager.

use std::mem;

use ami::*;
use awi::WindowConnection;

use asi_vulkan::types::*;
use super::check_error;

#[repr(C)]
struct SurfaceCreateInfoXcb {
	s_type: VkStructureType,
	p_next: *mut Void,
	flags: u32,
	connection: *mut Void,
	window: u32,
}

#[repr(C)]
struct SurfaceCreateInfoWindows {
	s_type: VkStructureType,
	p_next: *mut Void,
	flags: u32,
	// TODO
	hinstance: *mut Void,
	hwnd: *mut Void,
}

#[repr(C)]
struct SurfaceCreateInfoAndroid {
	s_type: VkStructureType,
	p_next: *mut Void,
	flags: u32,
	window: *mut Void, // ANativeWindow,
}

const ERROR : &'static str = "Failed to create surface.";

pub fn create_surface_xcb(instance: VkInstance, connection: *mut Void,
	window: u32) -> VkSurfaceKHR
{
	let mut surface = unsafe { mem::uninitialized() };
	let surface_create_info = SurfaceCreateInfoXcb {
		s_type: VkStructureType::SurfaceCreateInfoXcb,
		p_next: null_mut!(),
		flags: 0,
		connection: connection,
		window: window,
	};

	unsafe {
		extern "system" {
			fn vkCreateXcbSurfaceKHR(
				instance: VkInstance,
				pCreateInfo: *const SurfaceCreateInfoXcb,
				pAllocator: *mut Void,
				surface: *mut VkSurfaceKHR) -> VkResult;
		}
		check_error(ERROR, vkCreateXcbSurfaceKHR(instance,
			&surface_create_info, null_mut!(), &mut surface));
	};

	surface
}

#[cfg(not(target_os = "windows"))]
pub fn create_surface_windows(instance: VkInstance, connection: *mut Void,
	window: *mut Void) -> VkSurfaceKHR
{
	panic!("Can't create Windows surface on not Windows.");
}

#[cfg(target_os = "windows")]
pub fn create_surface_windows(instance: VkInstance, connection: *mut Void,
	window: *mut Void) -> VkSurfaceKHR
{
	let mut surface = unsafe { mem::uninitialized() };
	let surface_create_info = SurfaceCreateInfoWindows {
		s_type: VkStructureType::SurfaceCreateInfoWindows,
		p_next: null_mut!(),
		flags: 0,
		hinstance: connection,
		hwnd: window,
	};

	unsafe {
		extern "system" {
			fn vkCreateWin32SurfaceKHR(
				instance: VkInstance,
				pCreateInfo: *const SurfaceCreateInfoWindows,
				pAllocator: *mut Void,
				surface: *mut VkSurfaceKHR) -> VkResult;
		}
		check_error(ERROR, vkCreateWin32SurfaceKHR(
			instance, &surface_create_info, null_mut!(), &mut surface
		));
	};

	surface
}

pub fn create_surface_android(instance: VkInstance, window: *mut Void)
	-> VkSurfaceKHR
{
	let mut surface = unsafe { mem::uninitialized() };
	let surface_create_info = SurfaceCreateInfoAndroid {
		s_type: VkStructureType::SurfaceCreateInfoAndroid,
		p_next: null_mut!(),
		flags: 0,
		window: window,
	};

	unsafe {
		extern "system" {
			fn vkCreateAndroidSurfaceKHR(instance: VkInstance,
				pCreateInfo: *const SurfaceCreateInfoAndroid,
				pAllocator: *mut Void,
				surface: *mut VkSurfaceKHR) -> VkResult;
		}
		check_error(ERROR, vkCreateAndroidSurfaceKHR(
			instance, &surface_create_info, null_mut!(), &mut surface
		));
	};

	surface
}

pub fn create_surface(instance: VkInstance, connection: WindowConnection)
	-> VkSurfaceKHR
{
	match connection {
		WindowConnection::Xcb(connection,window) => {
			create_surface_xcb(instance, connection, window)
		}
		WindowConnection::Wayland => panic!("Wayland Rendering Not Supported Yet"),
		WindowConnection::DirectFB => panic!("DirectFB Rendering Not Supported Yet"),
		WindowConnection::Windows(connection, window) => {
			create_surface_windows(instance, connection, window)
		}
		WindowConnection::Android => panic!("Android Rendering Not Supported Yet"),
		WindowConnection::IOS => panic!("IOS Rendering Not Supported Yet"),
		WindowConnection::AldaronsOS => panic!("AldaronsOS Rendering Not Supported Yet"),
		WindowConnection::Arduino => panic!("Arduino Rendering Not Supported Yet"),
		WindowConnection::Switch => panic!("Switch Rendering Not Supported Yet"),
		WindowConnection::Web => panic!("Web Assembly Rendering Not Supported Yet"),
		WindowConnection::NoOS => panic!("No OS Rendering Not Supported Yet"),
	}
}
