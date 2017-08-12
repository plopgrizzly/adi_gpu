// Willow Graphics API
//
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE
//
// src/renderer/ffi/willow/mod.rs

pub struct WillowRenderer { }

impl ::RenderOps for WillowRenderer {
	fn new(app_name: &str, window: ::window::WindowConnection) -> Self {
		WillowRenderer { }
	}

	fn update(&self) -> () {
	}
}
