// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/lib.rs

//! Aldaron's Device Interface / GPU is a library developed by Plop Grizzly for
//! interfacing with the GPU to render graphics or do fast calculations.

// #![no_std]

#[macro_use]
extern crate ami;
extern crate awi;

/// Transform represents a transformation matrix.
#[must_use]
pub struct Transform(pub [f32; 16]);

mod renderer;
mod render_ops;
mod ali_vulkan;

pub mod input {
	pub use awi::InputQueue as Queue;
	pub use awi::Msg;
	pub use awi::Input;
}

pub use render_ops::RenderOps;
pub use renderer::Texture;

/// To render anything with adi_gpu, you have to make a `Display`
pub struct Display {
	window: awi::Window,
	renderer: renderer::Renderer,
}

impl Display {
	/// Connect to the display as a window with a name and icon.  Icon is in
	/// aci image format: `[ width, height, bgra pixels ]`.
	pub fn new(name: &str, icon: &Vec<u32>) -> Display {
		let window = awi::Window::new(name, icon);
		let renderer = renderer::Renderer::new(name,
			window.get_connection());

		Display { window, renderer }
	}

	/// Update the display / window.
	pub fn update(&mut self, input_queue: &mut input::Queue) {
		self.renderer.update();
		self.window.update(input_queue);

		if input_queue.get_resized() {
			self.renderer.resize(self.window.get_dimensions());
		}
	}
}

impl Texture {
	/// Create a new texture.
	pub fn new(display: &mut Display, image_data: &[u32]) -> Texture {
		display.renderer.texture(image_data[0], image_data[1],
			&image_data[2..])
	}
}

pub struct Shape(usize);

impl Shape {
	/// `Transform` the `Shape`.
	pub fn transform(&self, display: &mut Display, transform: &Transform)
		-> Shape
	{
		Shape(display.renderer.transform(self.0, transform))
	}
}

pub struct ShapeBuilder<'a> {
	vertices: &'a [f32],
}

impl<'a> ShapeBuilder<'a> {
	/// Obtain a new `ShapeBuilder` with `vertices`.
	#[inline(always)]
	pub fn new(vertices: &'a [f32]) -> ShapeBuilder {
		ShapeBuilder { vertices }
	}

	/// Push a shape with a solid color.
	#[inline(always)]
	pub fn push_solid(&self, display: &mut Display, color: [f32; 4])
		-> Shape
	{
		Shape(display.renderer.solid(self.vertices, color))
	}

	/// Push a shape with shaded by a gradient (1 color per vertex).
	#[inline(always)]
	pub fn push_gradient(&self, display: &mut Display, color: &[f32])
		-> Shape
	{
		Shape(display.renderer.gradient(self.vertices, color))
	}

	/// Push a shape with a texture and texture coordinates.
	/// Texture Coordinates follow this format `(X, Y, UNUSED(1.0), ALPHA)`.
	#[inline(always)]
	pub fn push_texture(&self, display: &mut Display, texture: Texture,
		tc: &[f32]) -> Shape
	{
		Shape(display.renderer.textured(self.vertices, texture, tc))
	}

	/// Push a shape with a texture, texture coordinates and alpha.
	/// Texture Coordinates follow this format `(X, Y, UNUSED(1.0), ALPHA)`.
	#[inline(always)]
	pub fn push_faded(&self, display: &mut Display, texture: Texture,
		tc: &[f32], alpha: f32) -> Shape
	{
		Shape(display.renderer.faded(self.vertices, texture, tc, alpha))
	}

	/// Push a shape with a texture and texture coordinates and tint.
	/// Texture Coordinates follow this format `(X, Y, UNUSED(1.0), ALPHA)`.
	#[inline(always)]
	pub fn push_tinted(&self, display: &mut Display, texture: Texture,
		tc: &[f32], tint: [f32; 4]) -> Shape
	{
		Shape(display.renderer.tinted(self.vertices, texture, tc, tint))
	}

	/// Push a shape with a texture and texture coordinates and tint per
	/// vertex.
	/// Texture Coordinates follow this format `(X, Y, UNUSED(1.0), ALPHA)`.
	#[inline(always)]
	pub fn push_complex(&self, display: &mut Display, texture: Texture,
		tc: &[f32], tints: &[f32]) -> Shape
	{
		Shape(display.renderer.complex(self.vertices, texture, tc,
			tints))
	}
}

impl Transform {
	/// A no-op transform (identity matrix).
	pub fn new() -> Transform {
		Transform([
			1.0, 0.0, 0.0, 0.0,
			0.0, 1.0, 0.0, 0.0,
			0.0, 0.0, 1.0, 0.0,
			0.0, 0.0, 0.0, 1.0,
		])
	}

	/// Multiply `self` by a matrix.
	pub fn matrix(self, matrix: [f32; 16]) -> Transform {
		self * Transform(matrix)
	}

	/// Multiply `self` by a scale transformation matrix.
	pub fn scale(self, x: f32, y: f32, z: f32) -> Transform {
		self.matrix([
			x,   0.0, 0.0, 0.0,
			0.0, y,   0.0, 0.0,
			0.0, 0.0, z,   0.0,
			0.0, 0.0, 0.0, 1.0,
		])
	}

	/// Multiply `self` by a translation matrix.
	pub fn translate(self, x: f32, y: f32, z: f32) -> Transform {
		self.matrix([
			1.0, 0.0, 0.0, 0.0,
			0.0, 1.0, 0.0, 0.0,
			0.0, 0.0, 1.0, 0.0,
			x,   y,   z,   1.0,
		])
	}

	/// Multiply `self` by a rotation matrix.  `x`, `y` and `z` are in PI
	/// Radians.
	pub fn rotate(self, x: f32, y: f32, z: f32) -> Transform {
		let num9 = z * ::std::f32::consts::PI;
		let num6 = num9.sin();
		let num5 = num9.cos();
		let num8 = x * ::std::f32::consts::PI;
		let num4 = num8.sin();
		let num3 = num8.cos();
		let num7 = y * ::std::f32::consts::PI;
		let num2 = num7.sin();
		let num = num7.cos();

		let qx = ((num * num4) * num5) + ((num2 * num3) * num6);
		let qy = ((num2 * num3) * num5) - ((num * num4) * num6);
		let qz = ((num * num3) * num6) - ((num2 * num4) * num5);
		let qw = ((num * num3) * num5) + ((num2 * num4) * num6);

		let nx = -qx;
		let ny = -qy;
		let nz = -qz;

		self.matrix([
			qw,nz,qy,nx,
			qz,qw,nx,ny,
			ny,qx,qw,nz,
			qx,qy,qz,qw
		]).matrix([
			qw,nz,qy,qx,
			qz,qw,nx,qy,
			ny,qx,qw,qz,
			nx,ny,nz,qw
		])
	}
}

impl ::std::ops::Mul<Transform> for Transform {
	type Output = Transform;

	fn mul(self, rhs: Transform) -> Self::Output {
		Transform([
			(self.0[0] * rhs.0[0]) + (self.0[1] * rhs.0[4]) +
			(self.0[2] * rhs.0[8]) + (self.0[3] * rhs.0[12]),
			(self.0[0] * rhs.0[1]) + (self.0[1] * rhs.0[5]) +
			(self.0[2] * rhs.0[9]) + (self.0[3] * rhs.0[13]),
			(self.0[0] * rhs.0[2]) + (self.0[1] * rhs.0[6]) +
			(self.0[2] * rhs.0[10]) + (self.0[3] * rhs.0[14]),
			(self.0[0] * rhs.0[3]) + (self.0[1] * rhs.0[7]) +
			(self.0[2] * rhs.0[11]) + (self.0[3] * rhs.0[15]),

			(self.0[4] * rhs.0[0]) + (self.0[5] * rhs.0[4]) +
			(self.0[6] * rhs.0[8]) + (self.0[7] * rhs.0[12]),
			(self.0[4] * rhs.0[1]) + (self.0[5] * rhs.0[5]) +
			(self.0[6] * rhs.0[9]) + (self.0[7] * rhs.0[13]),
			(self.0[4] * rhs.0[2]) + (self.0[5] * rhs.0[6]) +
			(self.0[6] * rhs.0[10]) + (self.0[7] * rhs.0[14]),
			(self.0[4] * rhs.0[3]) + (self.0[5] * rhs.0[7]) +
			(self.0[6] * rhs.0[11]) + (self.0[7] * rhs.0[15]),

			(self.0[8] * rhs.0[0]) + (self.0[9] * rhs.0[4]) +
			(self.0[10] * rhs.0[8]) + (self.0[11] * rhs.0[12]),
			(self.0[8] * rhs.0[1]) + (self.0[9] * rhs.0[5]) +
			(self.0[10] * rhs.0[9]) + (self.0[11] * rhs.0[13]),
			(self.0[8] * rhs.0[2]) + (self.0[9] * rhs.0[6]) +
			(self.0[10] * rhs.0[10]) + (self.0[11] * rhs.0[14]),
			(self.0[8] * rhs.0[3]) + (self.0[9] * rhs.0[7]) +
			(self.0[10] * rhs.0[11]) + (self.0[11] * rhs.0[15]),

			(self.0[12] * rhs.0[0]) + (self.0[13] * rhs.0[4]) +
			(self.0[14] * rhs.0[8]) + (self.0[15] * rhs.0[12]),
			(self.0[12] * rhs.0[1]) + (self.0[13] * rhs.0[5]) +
			(self.0[14] * rhs.0[9]) + (self.0[15] * rhs.0[13]),
			(self.0[12] * rhs.0[2]) + (self.0[13] * rhs.0[6]) +
			(self.0[14] * rhs.0[10]) + (self.0[15] * rhs.0[14]),
			(self.0[12] * rhs.0[3]) + (self.0[13] * rhs.0[7]) +
			(self.0[14] * rhs.0[11]) + (self.0[15] * rhs.0[15])
		])
	}
}
