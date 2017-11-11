// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/renderer/ffi/opengl/mod.rs

pub struct OpenGLRenderer { }

impl ::RenderOps for OpenGLRenderer {
	#[allow(unused)]
	fn new(app_name: &str, window: ::awi::WindowConnection) -> Self {
		OpenGLRenderer { }
	}

	fn update(&self) -> () {
	}
}
