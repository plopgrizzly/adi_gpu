// "adi_gpu" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
//
//! Aldaron's Device Interface / GPU is a library developed by Plop Grizzly for
//! interfacing with the GPU to render graphics or do fast calculations.

extern crate adi_gpu_base;
extern crate ami;

pub use ami::Mat4;

use adi_gpu_base as base;

pub use base::{
	afi, Graphic, Model, TexCoords, Gradient, Shape, Input, Key, Click, Msg,
	Display, Texture
};

/// Create a new Vulkan / OpenGL Display.
pub fn new_display<G: AsRef<Graphic>>(title: &str, icon: G)
	-> Result<Box<Display>, String>
{
	let mut err = "".to_string();

	// Try Vulkan first.
	#[cfg(any(unix, target_os="windows", target_os="nintendo_switch"))] {
		extern crate adi_gpu_vulkan;

		match adi_gpu_vulkan::new(title, &icon) {
			Ok(vulkan) => return Ok(vulkan),
			Err(vulkan) => err.push_str(vulkan),
		}
		err.push('\n');
	}

	// Fallback on OpenGL/OpenGLES
	#[cfg(any(unix, target_os="windows", target_os="web"))] {
		extern crate adi_gpu_opengl;

		match adi_gpu_opengl::new(title, &icon) {
			Ok(opengl) => return Ok(opengl),
			Err(opengl) => err.push_str(opengl),
		}
		err.push('\n');
	}

	// Give up
	err.push_str("No more backend options");
	Err(err)
}
