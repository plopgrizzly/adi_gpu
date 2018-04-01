// Aldaron's Device Interface / GPU
// Copyright (c) 2018 Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
// src/lib.rs

//! Aldaron's Device Interface / GPU is a library developed by Plop Grizzly for
//! interfacing with the GPU to render graphics or do fast calculations.

extern crate adi_gpu_opengl;
extern crate adi_gpu_vulkan;
extern crate adi_gpu_base;
extern crate afi;
extern crate ami;
extern crate awi;

pub use base::Model;
pub use base::TexCoords;
pub use base::Gradient;
pub use base::Shape;
pub use base::Display as DisplayTrait;
pub use base::Texture as TextureTrait;
pub use ami::Mat4;

use adi_gpu_base as base;

/// To render anything with adi_gpu, you have to make a `Display`
pub enum Display {
	Vulkan(adi_gpu_vulkan::Display),
	OpenGL(adi_gpu_opengl::Display),
}

impl DisplayTrait for Display {
	type Texture = Texture;

	fn new(title: &str, icon: &afi::Graphic) -> Option<Self> {
		/* if let Some(vulkan) = adi_gpu_vulkan::Display::new(title, icon) {
			Some(Display::Vulkan(vulkan))
		} else */if let Some(opengl) = adi_gpu_opengl::Display::new(title, icon) {
			Some(Display::OpenGL(opengl))
		} else {
			None
		}
	}

	fn color(&mut self, color: (f32, f32, f32)) {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.color(color)
			}
			Display::OpenGL(ref mut display) => {
				display.color(color)
			}
		}
	}

	fn update(&mut self) {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.update()
			}
			Display::OpenGL(ref mut display) => {
				display.update()
			}
		}
	}

	fn camera(&mut self, xyz: (f32,f32,f32), rotate_xyz: (f32,f32,f32)) {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.camera(xyz, rotate_xyz)
			}
			Display::OpenGL(ref mut display) => {
				display.camera(xyz, rotate_xyz)
			}
		}
	}

	fn model(&mut self, vertices: &[f32], indices: &[u32]) -> Model {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.model(vertices, indices)
			}
			Display::OpenGL(ref mut display) => {
				display.model(vertices, indices)
			}
		}
	}

	fn fog(&mut self, fog: Option<(f32, f32)>) -> () {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.fog(fog)
			}
			Display::OpenGL(ref mut display) => {
				display.fog(fog)
			}
		}
	}

	fn texture(&mut self, graphic: afi::Graphic) -> Texture {
		match *self {
			Display::Vulkan(ref mut display) => {
				Texture::Vulkan(display.texture(graphic))
			}
			Display::OpenGL(ref mut display) => {
				Texture::OpenGL(display.texture(graphic))
			}
		}
	}

	fn gradient(&mut self, colors: &[f32]) -> Gradient {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.gradient(colors)
			}
			Display::OpenGL(ref mut display) => {
				display.gradient(colors)
			}
		}
	}

	fn texcoords(&mut self, texcoords: &[f32]) -> TexCoords {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.texcoords(texcoords)
			}
			Display::OpenGL(ref mut display) => {
				display.texcoords(texcoords)
			}
		}
	}

	fn set_texture(&mut self, texture: &mut Self::Texture, pixels: &[u32]) {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.set_texture(
					match *texture {
						Texture::Vulkan(ref mut texture) => {
							texture
						}
						_ => panic!("mismatch"),
					}, pixels)
			}
			Display::OpenGL(ref mut display) => {
				display.set_texture(
					match *texture {
						Texture::OpenGL(ref mut texture) => {
							texture
						}
						_ => panic!("mismatch"),
					}, pixels)
			}
		}
	}

	#[inline(always)]
	fn shape_solid(&mut self, model: &Model, transform: Mat4,
		color: [f32; 4], blending: bool, fancy: bool, fog: bool,
		camera: bool) -> Shape
	{
		match *self {
			Display::Vulkan(ref mut display) => {
				display.shape_solid(
					model, transform, color,
					blending, fancy, fog, camera
				)
			}
			Display::OpenGL(ref mut display) => {
				display.shape_solid(
					model, transform, color,
					blending, fancy, fog, camera
				)
			}
		}
	}

	#[inline(always)]
	fn shape_gradient(&mut self, model: &Model, transform: Mat4,
		colors: Gradient, blending: bool, fancy: bool, fog: bool,
		camera: bool) -> Shape
	{
		match *self {
			Display::Vulkan(ref mut display) => {
				display.shape_gradient(
					model, transform, colors,
					blending, fancy, fog, camera
				)
			}
			Display::OpenGL(ref mut display) => {
				display.shape_gradient(
					model, transform, colors,
					blending, fancy, fog, camera
				)
			}
		}
	}

	#[inline(always)]
	fn shape_texture(&mut self, model: &Model, transform: Mat4,
		texture: Texture, tc: TexCoords, blending: bool, fancy: bool,
		fog: bool, camera: bool) -> Shape
	{
		match *self {
			Display::Vulkan(ref mut display) => {
				display.shape_texture(
					model, transform,
					match texture {
						Texture::Vulkan(texture) => {
							texture
						}
						_ => panic!("mismatch"),
					},
					tc, blending, fancy, fog, camera
				)
			}
			Display::OpenGL(ref mut display) => {
				display.shape_texture(
					model, transform,
					match texture {
						Texture::OpenGL(texture) => {
							texture
						}
						_ => panic!("mismatch"),
					},
					tc, blending, fancy, fog, camera
				)
			}
		}
	}

	#[inline(always)]
	fn shape_faded(&mut self, model: &Model, transform: Mat4,
		texture: Texture, tc: TexCoords, alpha: f32, fancy: bool,
		fog: bool, camera: bool) -> Shape
	{
		match *self {
			Display::Vulkan(ref mut display) => {
				display.shape_faded(
					model, transform,
					match texture {
						Texture::Vulkan(texture) => {
							texture
						}
						_ => panic!("mismatch"),
					},
					tc, alpha, fancy, fog, camera
				)
			}
			Display::OpenGL(ref mut display) => {
				display.shape_faded(
					model, transform,
					match texture {
						Texture::OpenGL(texture) => {
							texture
						}
						_ => panic!("mismatch"),
					},
					tc, alpha, fancy, fog, camera
				)
			}
		}
	}

	#[inline(always)]
	fn shape_tinted(&mut self, model: &Model, transform: Mat4,
		texture: Texture, tc: TexCoords, tint: [f32; 4], blending: bool,
		fancy: bool, fog: bool, camera: bool) -> Shape
	{
		match *self {
			Display::Vulkan(ref mut display) => {
				display.shape_tinted(
					model, transform,
					match texture {
						Texture::Vulkan(texture) => {
							texture
						}
						_ => panic!("mismatch"),
					},
					tc, tint, blending,
					fancy, fog, camera
				)
			}
			Display::OpenGL(ref mut display) => {
				display.shape_tinted(
					model, transform,
					match texture {
						Texture::OpenGL(texture) => {
							texture
						}
						_ => panic!("mismatch"),
					},
					tc, tint, blending,
					fancy, fog, camera
				)
			}
		}
	}

	#[inline(always)]
	fn shape_complex(&mut self, model: &Model, transform: Mat4,
		texture: Texture, tc: TexCoords, tints: Gradient,
		blending: bool, fancy: bool, fog: bool, camera: bool) -> Shape
	{
		match *self {
			Display::Vulkan(ref mut display) => {
				display.shape_complex(
					model, transform,
					match texture {
						Texture::Vulkan(texture) => {
							texture
						}
						_ => panic!("mismatch"),
					},
					tc, tints,
					blending, fancy, fog, camera
				)
			}
			Display::OpenGL(ref mut display) => {
				display.shape_complex(
					model, transform,
					match texture {
						Texture::OpenGL(texture) => {
							texture
						}
						_ => panic!("mismatch"),
					},
					tc, tints,
					blending, fancy, fog, camera
				)
			}
		}
	}

	fn transform(&mut self, shape: &mut Shape, transform: &Mat4) {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.transform(shape, transform)
			}
			Display::OpenGL(ref mut display) => {
				display.transform(shape, transform)
			}
		}
	}

	fn resize(&mut self, wh: (u32, u32)) -> () {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.resize(wh)
			}
			Display::OpenGL(ref mut display) => {
				display.resize(wh)
			}
		}
	}

	fn wh(&self) -> (u32, u32) {
		match *self {
			Display::Vulkan(ref display) => {
				display.wh()
			}
			Display::OpenGL(ref display) => {
				display.wh()
			}
		}
	}

	fn input(&mut self) -> Option<awi::Input> {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.input()
			}
			Display::OpenGL(ref mut display) => {
				display.input()
			}
		}
	}
}

#[derive(Copy, Clone)]
pub enum Texture {
	Vulkan(adi_gpu_vulkan::Texture),
	OpenGL(adi_gpu_opengl::Texture)
}

impl base::Texture for Texture {
	/// Get the width and height.
	fn wh(&self) -> (u32, u32) {
		let this: &base::Texture = match *self {
			Texture::Vulkan(ref texture) => {
				texture
			}
			Texture::OpenGL(ref texture) => {
				texture
			}
		};
		this.wh()
	}
}
