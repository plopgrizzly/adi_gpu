// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/renderer/ffi/willow/mod.rs

pub struct WillowRenderer { }

impl ::RenderOps for WillowRenderer {
	#[allow(unused)]
	fn new(app_name: &str, window: ::awi::WindowConnection) -> Self {
		WillowRenderer { }
	}

	fn update(&self) -> () {
	}
}
