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

	/*/// Add a `Shape` onto the `Display`.
	pub fn push(&mut self, shape: Shape) -> usize {
		let matrix = self.renderer.get_projection();

		match shape {
			Shape::Solid(vertices, color) => {
				self.renderer.solid(vertices, color)
			},
			Shape::Texture(vertices, image, txcoords) => {
				self.renderer.textured(vertices, image, txcoords)
			},
			Shape::Gradient(vertices, colors) => {
				0
			},
			Shape::FadeTexture(vertices, image, txcoords, a) => {
				0
			},
			Shape::TintTexture(vertices, image, txcoords, colr) => {
				0
			},
		}
	}*/

	// Push a texture into GPU memory.
/*	pub fn push_texture(&mut self, image_data: Vec<u32>) -> Texture {
		self.renderer.texture(image_data[0], image_data[1],
			&image_data[2..])
	}*/

	/// Update the display / window.
	pub fn update(&mut self, input_queue: &mut input::Queue) {
		self.renderer.update();
		self.window.update(input_queue);

		if input_queue.get_resized() {
			self.renderer.resize(self.window.get_dimensions());
		}
	}
}

/// An RGBA color.
#[repr(C)]
pub struct Color(pub f32, pub f32, pub f32, pub f32);

/*/// Macro to create a `Color`, from rgba.  Each compenents range: 0.0 to 1.0.
#[macro_export] macro_rules! color {
	($rgba:expr) => ({
		let rgba : [u8;4] = unsafe { ::std::mem::transmute($rgba) };
		let transform = ::std::u16::MAX as f64 / ::std::u8::MAX as f64;

		let r = (rgba[0] as f64 * transform) as u16;
		let g = (rgba[1] as f64 * transform) as u16;
		let b = (rgba[2] as f64 * transform) as u16;
		let a = (rgba[3] as f64 * transform) as u16;

		let bgra = [b, g, r, a];

		willow::Color(unsafe { ::std::mem::transmute(bgra) })
	});

	(r: f64, g: f64, b: f64, a: f64) => ({
		let r: u16 = r * ::std::u16::MAX;
		let g: u16 = g * ::std::u16::MAX;
		let b: u16 = b * ::std::u16::MAX;
		let a: u16 = a * ::std::u16::MAX;

		let bgra = [b, g, r, a];

		willow::Color(unsafe { ::std::mem::transmute(bgra) })
	});
}*/

/// A drawable shape.
/*pub enum Shape<'a> {
	/// A Single-Color Shape `(vertices, color)`
	Solid(&'a [f32], Color),
	/// A Textured Shape `(vertices, image, texture coordinates)`
	Texture(&'a [f32], Texture, &'a [f32]),
	/// A Multi-Color Shape - One color per vertex `(vertices, colors)`
	Gradient(&'a [f32], &'a [Color]),
	/// A Fading Texture Shape
	/// `(vertices, image, texture coordinates, alpha)`
	FadeTexture(&'a [f32], Texture, &'a [f32], f32),
	/// A Tinted Texture Shape
	/// `(vertices, image, texture coordinates, color)`
	TintTexture(&'a [f32], Texture, &'a [f32], Color),
}*/

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
	pub fn push_solid(&self, display: &mut Display, color: Color) -> Shape {
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
	#[inline(always)]
	pub fn push_texture(&self, display: &mut Display, texture: Texture,
		tc: &[f32]) -> Shape
	{
		Shape(display.renderer.textured(self.vertices, texture, tc))
	}

	/// Push a shape with a texture, texture coordinates and alpha.
	#[inline(always)]
	pub fn push_faded(&self, display: &mut Display, texture: Texture,
		tc: &[f32], alpha: f32) -> Shape
	{
//		display.renderer.textured(self.vertices, image, tc)
		Shape(0)
	}

	/// Push a shape with a texture and texture coordinates and tint.
	#[inline(always)]
	pub fn push_tinted(&self, display: &mut Display, texture: Texture,
		tc: &[f32], tint: Color) -> Shape
	{
//		display.renderer.textured(self.vertices, image, tc)
		Shape(0)
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
