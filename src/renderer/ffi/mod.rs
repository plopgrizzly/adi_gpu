// Willow Graphics API
//
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE
//
// src/renderer/ffi/mod.rs

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "android"))]
#[link(name = "vulkan-1")]
pub mod vulkan;

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
pub mod opengl;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub mod metal;

pub mod willow;

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "android"))]
pub enum NativeRenderer {
	OpenGL(opengl::OpenGLRenderer),
	Vulkan(vulkan::VulkanRenderer),
	Willow(willow::WillowRenderer),
}

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "android"))]
impl ::RenderOps for NativeRenderer {
	fn new(app_name: &str, window: ::window::WindowConnection) -> Self {
		NativeRenderer::Vulkan(
			vulkan::VulkanRenderer::new(app_name, window)
		)
	}

	fn update(&self) -> () {
		match *self {
			NativeRenderer::OpenGL(ref renderer) => renderer.update(),
			NativeRenderer::Vulkan(ref renderer) => renderer.update(),
			NativeRenderer::Willow(ref renderer) => renderer.update(),
		}
	}
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub enum NativeRenderer {
	OpenGL(opengl::OpenGLRenderer),
	Metal(vulkan::MetalRenderer),
	Willow(willow::WillowRenderer),
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
impl ::RenderOps for NativeRenderer {
	fn new(app_name: &str, window: ::window::WindowConnection) -> Self {
		NativeRenderer::Vulkan(
			vulkan::VulkanRenderer::new(app_name, window)
		)
	}

	fn update(&self) -> () {
		match *self {
			NativeRenderer::OpenGL(ref renderer) => renderer.update(),
			NativeRenderer::Metal(ref renderer) => renderer.update(),
			NativeRenderer::Willow(ref renderer) => renderer.update(),
		}
	}
}

#[cfg(any(target_os = "freebsd", target_os = "dragonfly", target_os = "bitrig",
	target_os = "openbsd", target_os = "netbsd",
	target_os = "web_assembly"))]
pub enum NativeRenderer {
	OpenGL(opengl::OpenGLRenderer),
	Willow(willow::WillowRenderer),
}

#[cfg(any(target_os = "freebsd", target_os = "dragonfly", target_os = "bitrig",
	target_os = "openbsd", target_os = "netbsd",
	target_os = "web_assembly"))]
impl ::RenderOps for NativeRenderer {
	fn new(app_name: &str, window: ::window::WindowConnection) -> Self {
		NativeRenderer::Vulkan(
			vulkan::OpenGLRenderer::new(app_name, window)
		)
	}

	fn update(&self) -> () {
		match *self {
			NativeRenderer::OpenGL(ref renderer) => renderer.update(),
			NativeRenderer::Willow(ref renderer) => renderer.update(),
		}
	}
}

#[cfg(any(target_os = "aldarons_os", target_os = "arduino",
	target_os = "none"))]
pub use WillowRenderer as NativeRenderer;

#[cfg(target_os = "nintendo_switch")]
pub enum NativeRenderer {
	Vulkan(vulkan::VulkanRenderer),
	Willow(willow::WillowRenderer),
}

#[cfg(target_os = "nintendo_switch")]
impl ::RenderOps for NativeRenderer {
	fn new(app_name: &str, window: ::window::WindowConnection) -> Self {
		NativeRenderer::Vulkan(
			vulkan::VulkanRenderer::new(app_name, window)
		)
	}

	fn update(&self) -> () {
		match *self {
			NativeRenderer::Vulkan(ref renderer) => renderer.update(),
			NativeRenderer::Willow(ref renderer) => renderer.update(),
		}
	}
}
