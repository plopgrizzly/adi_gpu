// Aldaron's Device Interface / GPU
// Copyright (c) 2018 Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
// src/lib.rs

//! Aldaron's Device Interface / GPU is a library developed by Plop Grizzly for
//! interfacing with the GPU to render graphics or do fast calculations.

extern crate adi_gpu_vulkan;
extern crate adi_gpu_base;
extern crate afi;
extern crate ami;
extern crate awi;

pub use base::Display as DisplayTrait;
pub use base::Texture as TextureTrait;
pub use ami::Mat4;

use adi_gpu_base as base;

/// To render anything with adi_gpu, you have to make a `Display`
pub enum Display {
	Vulkan(adi_gpu_vulkan::Display),
}

impl DisplayTrait for Display {
	type Model = Model;
	type Texture = Texture;
	type Gradient = Gradient;
	type TexCoords = TexCoords;
	type Shape = Shape;

	fn new(window: &awi::Window) -> Option<Self> {
		if let Some(vulkan) = adi_gpu_vulkan::Display::new(window) {
			Some(Display::Vulkan(vulkan))
		} else {
			None
		}
	}

	fn color(&mut self, color: (f32, f32, f32)) {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.color(color)
			}
		}
	}

	fn update(&mut self) {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.update()
			}
		}
	}

	fn camera(&mut self, xyz: (f32,f32,f32), rotate_xyz: (f32,f32,f32)) {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.camera(xyz, rotate_xyz)
			}
		}
	}

	fn model(&mut self, vertices: &[f32], indices: &[u32]) -> Model {
		match *self {
			Display::Vulkan(ref mut display) => {
				Model::Vulkan(display.model(vertices, indices))
			}
		}
	}

	fn fog(&mut self, fog: Option<(f32, f32)>) -> () {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.fog(fog)
			}
		}
	}

	fn texture(&mut self, graphic: afi::Graphic) -> Texture {
		match *self {
			Display::Vulkan(ref mut display) => {
				Texture::Vulkan(display.texture(graphic))
			}
		}
	}

	fn gradient(&mut self, colors: &[f32]) -> Gradient {
		match *self {
			Display::Vulkan(ref mut display) => {
				Gradient::Vulkan(display.gradient(colors))
			}
		}
	}

	fn texcoords(&mut self, texcoords: &[f32]) -> TexCoords {
		match *self {
			Display::Vulkan(ref mut display) => {
				TexCoords::Vulkan(display.texcoords(texcoords))
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
				Shape::Vulkan(display.shape_solid(
					match *model {
						Model::Vulkan(ref model) => {
							model
						}
					},
					transform, color,
					blending, fancy, fog, camera)
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
				Shape::Vulkan(display.shape_gradient(
					match *model {
						Model::Vulkan(ref model) => {
							model
						}
					},
					transform,
					match colors {
						Gradient::Vulkan(gradient) => {
							gradient
						}
					},
					blending, fancy, fog, camera)
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
				Shape::Vulkan(display.shape_texture(
					match *model {
						Model::Vulkan(ref model) => {
							model
						}
					}, transform,
					match texture {
						Texture::Vulkan(texture) => {
							texture
						}
					},
					match tc {
						TexCoords::Vulkan(texcoords) => {
							texcoords
						}
					}, blending, fancy, fog, camera)
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
				Shape::Vulkan(display.shape_faded(
					match *model {
						Model::Vulkan(ref model) => {
							model
						}
					},
					transform,
					match texture {
						Texture::Vulkan(texture) => {
							texture
						}
					},
					match tc {
						TexCoords::Vulkan(texcoords) => {
							texcoords
						}
					}, alpha, fancy, fog, camera)
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
				Shape::Vulkan(display.shape_tinted(
					match *model {
						Model::Vulkan(ref model) => {
							model
						}
					},
					transform,
					match texture {
						Texture::Vulkan(texture) => {
							texture
						}
					},
					match tc {
						TexCoords::Vulkan(texcoords) => {
							texcoords
						}
					}, tint, blending,
					fancy, fog, camera)
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
				Shape::Vulkan(display.shape_complex(
					match *model {
						Model::Vulkan(ref model) => {
							model
						}
					}, transform,
					match texture {
						Texture::Vulkan(texture) => {
							texture
						}
					},
					match tc {
						TexCoords::Vulkan(texcoords) => {
							texcoords
						}
					},
					match tints {
						Gradient::Vulkan(gradient) => {
							gradient
						}
					},
					blending, fancy, fog, camera)
				)
			}
		}
	}

	fn transform(&mut self, shape: &mut Self::Shape, transform: &Mat4) {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.transform(match *shape {
					Shape::Vulkan(ref mut shape) => {
						shape
					}
				}, transform)
			}
		}
	}

	fn resize(&mut self, wh: (u32, u32)) -> () {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.resize(wh)
			}
		}
	}
}

/// A list of vertices that make a shape.
#[derive(Copy, Clone)]
pub enum Model {
	Vulkan(adi_gpu_vulkan::Model)
}

impl base::Model for Model { }

/// A list of colors to be paired with vertices.
#[derive(Copy, Clone)]
pub enum Gradient {
	Vulkan(adi_gpu_vulkan::Gradient)
}

impl base::Gradient for Gradient {
}

/// A list of texture coordinates to be paired with vertices.
#[derive(Copy, Clone)]
pub enum TexCoords {
	Vulkan(adi_gpu_vulkan::TexCoords)
}

impl base::TexCoords for TexCoords {
}

#[derive(Copy, Clone)]
pub enum Texture {
	Vulkan(adi_gpu_vulkan::Texture)
}

impl base::Texture for Texture {
	/// Get the width and height.
	fn wh(&self) -> (u32, u32) {
		let this: &base::Texture = match *self {
			Texture::Vulkan(ref texture) => {
				texture
			}
		};
		this.wh()
	}
}

/// A renderable object that exists on the `Display`.
pub enum Shape {
	Vulkan(adi_gpu_vulkan::Shape)
}

impl base::Shape for Shape {
}
