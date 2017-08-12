// Willow Graphics API
//
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE
//
// src/render_ops.rs

pub trait RenderOps {
	// Create the renderer.
	fn new(app_name: &str, window: ::window::WindowConnection) -> Self;
	// Do the rendering.
	fn update(&self) -> ();
}
