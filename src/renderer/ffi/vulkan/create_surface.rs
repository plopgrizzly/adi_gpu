// Willow Graphics API
//
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE
//
// src/renderer/ffi/vulkan/create_surface.rs

// TODO: Make surface a buffer and blit onto screen with window manager.

use std::mem;

use ami::*;
use awi::WindowConnection;

use super::ffi::types::*;
use super::check_error;

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[repr(C)]
struct SurfaceCreateInfo {
	s_type: VkStructureType,
	p_next: *mut Void,
	flags: u32,
	connection: *mut Void,
	window: u32,
}

#[cfg(target_os = "windows")]
#[repr(C)]
struct SurfaceCreateInfo {
	s_type: VkStructureType,
	p_next: *mut Void,
	flags: u32,
	// TODO
	hinstance: *mut Void,
	hwnd: *mut Void,
}

#[cfg(target_os = "android")]
#[repr(C)]
struct SurfaceCreateInfo {
	s_type: VkStructureType,
	p_next: *mut Void,
	flags: u32,
	window: *mut ANativeWindow,
}

const ERROR : &'static str = "Failed to create surface.";

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn create_surface_xcb(instance: VkInstance, connection: *mut Void,
	window: u32) -> VkSurfaceKHR
{
	let mut surface = unsafe { mem::uninitialized() };
	let surface_create_info = SurfaceCreateInfo {
		s_type: VkStructureType::SurfaceCreateInfo,
		p_next: NULL.as_mut_ptr(),
		flags: 0,
		connection: connection,
		window: window,
	};

	unsafe {
		extern "system" {
			fn vkCreateXcbSurfaceKHR(
				instance: VkInstance,
				pCreateInfo: *const SurfaceCreateInfo,
				pAllocator: *mut Void,
				surface: *mut VkSurfaceKHR) -> VkResult;
		}
		check_error(ERROR, vkCreateXcbSurfaceKHR(instance,
			&surface_create_info, NULL.as_mut_ptr(), &mut surface));
	};

	surface
}

#[cfg(target_os = "windows")]
pub fn create_surface(instance: VkInstance, native_window: &::AwiWindow)
	-> VkSurfaceKHR
{
	let mut surface = unsafe { mem::uninitialized() };
	let surface_create_info = SurfaceCreateInfo {
		s_type: VkStructureType::SurfaceCreateInfo,
		p_next: NULL,
		flags: 0,
		hinstance: native_window.get_connection(),
		hwnd: native_window.get_window(),
	};

	unsafe {
		extern "system" {
			fn vkCreateWin32SurfaceKHR(
				instance: VkInstance,
				pCreateInfo: *const SurfaceCreateInfo,
				pAllocator: *mut Void,
				surface: *mut VkSurfaceKHR) -> VkResult;
		}
		check_error(ERROR, vkCreateWin32SurfaceKHR(
			instance, &surface_create_info, NULL, &mut surface
		));
	};

	surface
}

#[cfg(target_os = "android")]
pub fn create_surface(instance: VkInstance, native_window: &::AwiWindow)
	-> VkSurfaceKHR
{
	let mut surface = unsafe { mem::uninitialized() };
	let surface_create_info = SurfaceCreateInfo {
		s_type: VkStructureType::SurfaceCreateInfo,
		p_next: NULL,
		flags: 0,
		window: native_window.get_window(),
	};

	unsafe {
		extern "system" {
			fn vkCreateAndroidSurfaceKHR(instance: VkInstance,
				pCreateInfo: *const SurfaceCreateInfo,
				pAllocator: *mut Void,
				surface: *mut VkSurfaceKHR) -> VkResult;
		}
		check_error(ERROR, vkCreateAndroidSurfaceKHR(
			instance, &surface_create_info, NULL, &mut surface
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
		WindowConnection::Windows => panic!("Windows Rendering Not Supported Yet"),
		WindowConnection::Android => panic!("Android Rendering Not Supported Yet"),
		WindowConnection::IOS => panic!("IOS Rendering Not Supported Yet"),
		WindowConnection::AldaronsOS => panic!("AldaronsOS Rendering Not Supported Yet"),
		WindowConnection::Arduino => panic!("Arduino Rendering Not Supported Yet"),
		WindowConnection::Switch => panic!("Switch Rendering Not Supported Yet"),
		WindowConnection::Web => panic!("Web Assembly Rendering Not Supported Yet"),
		WindowConnection::NoOS => panic!("No OS Rendering Not Supported Yet"),
	}
}
