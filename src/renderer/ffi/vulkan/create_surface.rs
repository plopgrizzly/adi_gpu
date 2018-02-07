// Aldaron's Device Interface / GPU
// Copyright (c) 2017-2018 Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/renderer/ffi/vulkan/create_surface.rs

// TODO: Make surface a buffer and blit onto screen with window manager.

use awi::WindowConnection;

use asi_vulkan;
use asi_vulkan::types::*;
use asi_vulkan::Connection;

pub fn create_surface(c: &Connection, instance: VkInstance,
	connection: WindowConnection) -> VkSurfaceKHR
{
	match connection {
		WindowConnection::Xcb(connection,window) => {
			asi_vulkan::create_surface_xcb(c, instance, connection, window)
		}
		WindowConnection::Wayland => panic!("Wayland Rendering Not Supported Yet"),
		WindowConnection::DirectFB => panic!("DirectFB Rendering Not Supported Yet"),
		WindowConnection::Windows(connection, window) => {
			asi_vulkan::create_surface_windows(c, instance, connection, window)
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