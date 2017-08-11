// Willow Graphics API
//
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE
//
// src/lib.rs

// #![no_std]

extern crate ami;
extern crate window;

/// Transform represents a transformation matrix.
#[must_use]
pub struct Transform(pub [f32; 16]);

/// A Matrix Transform
#[macro_export] macro_rules! matrix {
	[$ ( $ x : expr ), *] => ( $crate::Transform([ $( $x ), *]) );
	[$ ( $ x : expr , ) *] => ( matrix![ $( $x ), *] );
}

/// A No-Op Transform - An Identity Matrix.
#[macro_export] macro_rules! identity {
	() => ( matrix![1.,0.,0.,0.,0.,1.,0.,0.,0.,0.,1.,0.,0.,0.,0.,1.] );
}

/// Scale Transformation Matrix
#[macro_export] macro_rules! scale {
	($x: expr, $y: expr, $z: expr) =>
		( matrix![$x,0.,0.,0.,0.,$y,0.,0.,0.,0.,$z,0.,0.,0.,0.,1.] );
	($x: expr, $y: expr) => ( scale!($x, $y, 1.0) );
	($s: expr; X) => ( scale!($s, 1.0, 1.0) );
	($s: expr; Y) => ( scale!(1.0, $s, 1.0) );
	($s: expr; Z) => ( scale!(1.0, 1.0, $s) );
	($s: expr; 2D) => ( scale!($s, $s) );
	($s: expr; 3D) => ( scale!($s, $s, $s) );
}

/// Translation Matrix
#[macro_export] macro_rules! translate {
	($x: expr, $y: expr, $z: expr) =>
		( matrix![1.,0.,0.,$x,0.,1.,0.,$y,0.,0.,1.,$z,0.,0.,0.,1.] );
	($x: expr, $y: expr) => ( translate!($x, $y, 0.0) );
	($s: expr; X) => ( translate!($s, 0.0, 0.0) );
	($s: expr; Y) => ( translate!(0.0, $s, 0.0) );
	($s: expr; Z) => ( translate!(0.0, 0.0, $s) );
}

/// Rotation Matrix
#[macro_export] macro_rules! rotate {
	($x: expr, $y: expr, $z: expr) => ({
		let num9 = $z * ::std::f32::consts::PI;
		let num6 = num9.sin();
		let num5 = num9.cos();
		let num8 = $x * ::std::f32::consts::PI;
		let num4 = num8.sin();
		let num3 = num8.cos();
		let num7 = $y * ::std::f32::consts::PI;
		let num2 = num7.sin();
		let num = num7.cos();

		let qx = ((num * num4) * num5) + ((num2 * num3) * num6);
		let qy = ((num2 * num3) * num5) - ((num * num4) * num6);
		let qz = ((num * num3) * num6) - ((num2 * num4) * num5);
		let qw = ((num * num3) * num5) + ((num2 * num4) * num6);

		let nx = -qx;
		let ny = -qy;
		let nz = -qz;

		matrix![qw,nz,qy,nx,qz,qw,nx,ny,ny,qx,qw,nz,qx,qy,qz,qw] *
			matrix![qw,nz,qy,qx,qz,qw,nx,qy,ny,qx,qw,qz,nx,ny,nz,qw]
	});
}

mod renderer;

pub mod input {
	pub use window::InputQueue as Queue;
	pub use window::Msg;
	pub use window::Input;
}

/// To render anything with The Willow Graphics API, you have to make a
/// `Display`
pub struct Display {
	window: window::Window,
	renderer: renderer::Renderer,
}

impl Display {
	/// Connect to the display as a window with a name and icon.  Icon is in
	/// aci image format: `[ width, height, bgra pixels ]`.
	pub fn new(name: &str, icon: &Vec<u32>) -> Display {
		let window = window::Window::new(name, icon);
		let renderer = renderer::Renderer::new(name,
			window.get_connection());

		Display { window, renderer }
	}

	/// Add a `Shape` onto the `Display`.
	pub fn push(&mut self, shape: Shape) -> usize {
		let matrix = self.renderer.get_projection();

		match shape {
			Shape::Solid(vertices, color) => {
				let vertices = matrix * vertices;

				self.renderer.solid(vertices, color)
			},
			Shape::Texture(vertices, image, txcoords) => {
				0
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
pub enum Shape<'a> {
	/// A Single-Color Shape `(vertices, color)`
	Solid(Vec<f32>, Color),
	/// A Textured Shape `(vertices, image, texture coordinates)`
	Texture(&'a [f32], u32, &'a [f32]),
	/// A Multi-Color Shape - One color per vertex `(vertices, colors)`
	Gradient(&'a [f32], &'a [Color]),
	/// A Fading Texture Shape
	/// `(vertices, image, texture coordinates, alpha)`
	FadeTexture(&'a [f32], u32, &'a [f32], f32),
	/// A Tinted Texture Shape
	/// `(vertices, image, texture coordinates, color)`
	TintTexture(&'a [f32], u32, &'a [f32], Color),
}

impl ::std::ops::Mul<(f32, f32, f32)> for Transform {
	type Output = Transform;

	fn mul(mut self, rhs: (f32, f32, f32)) -> Self::Output {
		self.0[0] *= rhs.0;
		self.0[5] *= rhs.1;
		self.0[15] *= rhs.2;

		self
	}
}

// 11 0
// 12 1 
// 13 2
// 14 3 
// 21 4
// 22 5
// 23 6
// 24 7
// 31 8
// 32 9
// 33 10
// 34 11
// 41 12
// 42 13
// 43 14
// 44 15

impl ::std::ops::Mul<Transform> for Transform {
	type Output = Transform;

	fn mul(self, rhs: Transform) -> Self::Output {
		matrix![
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
		]
	}
}

impl ::std::ops::Mul<Vec<f32>> for Transform {
	type Output = Vec<f32>;

	fn mul(self, rhs: Vec<f32>) -> Self::Output {
		let mut model = rhs;

		for i in 0..(model.len() / 4) {
			let i = i * 4;

			let x = model[i + 0];
			let y = model[i + 1];
			let z = model[i + 2];
			let w = model[i + 3];

			model[i + 0] = self.0[0] * x + self.0[1] * y
				+ self.0[2] * z + self.0[3] * w;
			model[i + 1] = self.0[4] * x + self.0[5] * y
				+ self.0[6] * z + self.0[7] * w;
			model[i + 2] = self.0[8] * x + self.0[9] * y
				+ self.0[10] * z + self.0[11] * w;
			model[i + 3] = self.0[12] * x + self.0[13] * y
				+ self.0[14] * z + self.0[15] * w;
		}

		model
	}
}
