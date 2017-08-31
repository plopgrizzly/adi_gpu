// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/render_ops.rs

pub trait RenderOps {
	// Create the renderer.
	fn new(app_name: &str, window: ::awi::WindowConnection) -> Self;
	// Do the rendering.
	fn update(&self) -> ();
}
