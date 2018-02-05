// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// examples/animated/main.rs

extern crate adi_gpu;
extern crate aci_png;

const SQUARE_INDICES: &'static [u32] = &include!("../res/square.indices");
const SQUARE_MODEL: &'static [f32] = &include!("../res/square.data");
const IMAGE_MODEL: &'static [f32] = &include!("../res/image.data");
const IMAGE_TEXCOORDS: &'static [f32] = &include!("../res/image.texc");
const TRIANGLE_MODEL: &'static [f32] = &include!("../res/triangle.data");
const TRIANGLE_INDICES: &'static [u32] = &include!("../res/triangle.indices");

pub fn resize(display: &mut adi_gpu::Display, image_tex: adi_gpu::Texture,
	logo_tex: adi_gpu::Texture)
{
	println!("!Resizing...");

	let square_model = adi_gpu::Model::new(display, SQUARE_MODEL, SQUARE_INDICES);
	let image_model = adi_gpu::Model::new(display, IMAGE_MODEL, SQUARE_INDICES);
	let triangle_model = adi_gpu::Model::new(display, TRIANGLE_MODEL, TRIANGLE_INDICES);

	let image_texcoords = adi_gpu::TexCoords::new(display, IMAGE_TEXCOORDS);

	let gradient = adi_gpu::Gradient::new(display, &[
			1.0, 0.0, 0.0, 1.0,
			0.0, 1.0, 0.0, 1.0,
			0.0, 0.0, 1.0, 1.0,
		]);

	let complexity = adi_gpu::Gradient::new(display, &[
			1.0, 0.0, 0.0, 1.0,
			0.0, 0.0, 1.0, 1.0,
			0.0, 1.0, 0.0, 1.0,
			0.0, 1.0, 0.0, 1.0 
		]);

	// square
	adi_gpu::ShapeBuilder::new(square_model)
		.push_solid(display, adi_gpu::Transform::new()
			.translate(0.0, 0.0, 0.5).rotate(0.25, 0.0, 0.0),
			[0.5, 1.0, 0.5, 0.5], false, false);
	// image
	adi_gpu::ShapeBuilder::new(image_model)
		.push_texture(display, adi_gpu::Transform::new(), image_tex,
		image_texcoords, false, false);
	// logo
	adi_gpu::ShapeBuilder::new(image_model)
		.push_texture(display, adi_gpu::Transform::new()
			.translate(0.5, 0.5, 0.0), logo_tex, image_texcoords,
			false, false);
	// triangle
	adi_gpu::ShapeBuilder::new(triangle_model)
		.push_gradient(display, adi_gpu::Transform::new(), gradient, false, false);
	// new!
	adi_gpu::ShapeBuilder::new(image_model)
		.push_faded(display, adi_gpu::Transform::new()
			.translate(1.0, 1.0, 0.0), image_tex, image_texcoords,
			0.5, false);
	// new!
	adi_gpu::ShapeBuilder::new(image_model)
		.push_tinted(display, adi_gpu::Transform::new()
			.translate(1.5, 1.5, 0.0), image_tex, image_texcoords,
			[1.0, 1.0, 0.0, 1.0], false, false);
	// new!
	adi_gpu::ShapeBuilder::new(image_model)
		.push_complex(display, adi_gpu::Transform::new()
			.translate(1.0, 1.5, 0.0), image_tex, image_texcoords,
			complexity, false, false);
}

fn main() {
//	let mut display = adi_gpu::Display::new("adi_gpu Animated Example",
//		aci_png::decode(include_bytes!("../res/test.png")).unwrap(),
//		(0.25, 0.25, 1.0), (20.0, 10.0));
	let mut display = adi_gpu::Display::new("Willow Minimal Example",
		aci_png::decode(include_bytes!("../res/icon.png")).unwrap(),
		(0.25, 0.25, 1.0), (20.0, 10.0));

	let logo_texture = adi_gpu::Texture::new(&mut display,
		aci_png::decode(include_bytes!("../res/icon.png")).unwrap());
	let plopgrizzly_texture = adi_gpu::Texture::new(&mut display,
		aci_png::decode(include_bytes!("../res/plopgrizzly.png"))
			.unwrap());
//	let test_texture = display.push_texture(aci_png::decode(
//		include_bytes!("../res/test.png")).unwrap());

	resize(&mut display, logo_texture, plopgrizzly_texture);

	'app: loop {
		// Go through this frame's input.
		while let Some(input) = display.input() {
			use adi_gpu::window::Input::*;
			use adi_gpu::window::Msg::*;

			match input {
				Msg(Quit) | Msg(Back) => break 'app,
				_ => {},
			}
		}

		display.update();
	}
}
