// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// examples/animated/main.rs

extern crate adi_gpu;
extern crate aci_png;

const SQUARE_MODEL: &'static [f32] = &include!("../res/square.data");
const IMAGE_VERTICES: &'static [f32] = &include!("../res/image.data");
const IMAGE_TEXCOORDS: &'static [f32] = &include!("../res/image.texc");
const TRIANGLE_MODEL: &'static [f32] = &include!("../res/triangle.data");

pub fn resize(display: &mut adi_gpu::Display, image_tex: adi_gpu::Texture,
	logo_tex: adi_gpu::Texture)
{
	println!("!Resizing...");

	// square
	adi_gpu::ShapeBuilder::new(SQUARE_MODEL)
		.push_solid(display, [0.5, 1.0, 0.5, 0.5])
		.transform(display, &adi_gpu::Transform::new()
			.translate(0.0, 0.0, 0.5).rotate(0.25, 0.0, 0.0));
	// image
	adi_gpu::ShapeBuilder::new(IMAGE_VERTICES)
		.push_texture(display, image_tex, IMAGE_TEXCOORDS);
	// logo
	adi_gpu::ShapeBuilder::new(IMAGE_VERTICES)
		.push_texture(display, logo_tex, IMAGE_TEXCOORDS)
		.transform(display, &adi_gpu::Transform::new()
			.translate(0.5, 0.5, 0.0));
	// triangle
	adi_gpu::ShapeBuilder::new(TRIANGLE_MODEL)
		.push_gradient(display, &[
			1.0, 0.0, 0.0, 1.0,
			0.0, 1.0, 0.0, 1.0,
			0.0, 0.0, 1.0, 1.0,
		]);
	// new!
	adi_gpu::ShapeBuilder::new(IMAGE_VERTICES)
		.push_faded(display, image_tex, IMAGE_TEXCOORDS, 0.5)
		.transform(display, &adi_gpu::Transform::new()
			.translate(1.0, 1.0, 0.0));
	// new!
	adi_gpu::ShapeBuilder::new(IMAGE_VERTICES)
		.push_tinted(display, image_tex, IMAGE_TEXCOORDS,
			[1.0, 1.0, 0.0, 1.0])
		.transform(display, &adi_gpu::Transform::new()
			.translate(1.5, 1.5, 0.0));
	// new!
	adi_gpu::ShapeBuilder::new(IMAGE_VERTICES)
		.push_complex(display, image_tex, IMAGE_TEXCOORDS, &[
			1.0, 0.0, 0.0, 1.0,
			0.0, 0.0, 1.0, 1.0,
			0.0, 1.0, 0.0, 1.0,

			0.0, 0.0, 1.0, 1.0,
			1.0, 0.0, 0.0, 1.0,
			0.0, 1.0, 0.0, 1.0 ])
		.transform(display, &adi_gpu::Transform::new()
			.translate(1.0, 1.5, 0.0));
}

pub fn update(_: &mut adi_gpu::Display) {
	
}

fn main() {
	let display_icon = aci_png::decode(include_bytes!("../res/icon.png"))
		.unwrap();

	let mut display = adi_gpu::Display::new("adi_gpu Animated Example",
		&display_icon);

	let mut queue = adi_gpu::input::Queue::new();

	let logo_texture = adi_gpu::Texture::new(&mut display, display_icon.as_slice());
	let plopgrizzly_texture = adi_gpu::Texture::new(&mut display,
		aci_png::decode(include_bytes!("../res/plopgrizzly.png"))
			.unwrap().as_slice());
//	let test_texture = display.push_texture(aci_png::decode(
//		include_bytes!("../res/test.png")).unwrap());

	resize(&mut display, logo_texture, plopgrizzly_texture);

	'app: loop {
		update(&mut display);

		display.update(&mut queue);

		for input in queue.iter() {
			use adi_gpu::input::Input::*;
			use adi_gpu::input::Msg::*;

			match *input {
				Msg(Quit) | Msg(Back) => break 'app,
				Resize => resize(&mut display, logo_texture,
					plopgrizzly_texture),
				_ => {},
			}
		}
	}
}
