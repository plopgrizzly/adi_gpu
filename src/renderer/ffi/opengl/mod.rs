// Willow Graphics API
//
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE
//
// src/renderer/ffi/opengl/mod.rs

pub struct OpenGLRenderer { }

impl ::RenderOps for OpenGLRenderer {
	fn new(app_name: &str, window: ::awi::WindowConnection) -> Self {
		OpenGLRenderer { }
	}

	fn update(&self) -> () {
	}
}
