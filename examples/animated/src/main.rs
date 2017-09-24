// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// examples/animated/main.rs

#[macro_use]
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
	display.push(adi_gpu::Shape::Solid(
		translate!(0.5; Z) * SQUARE_MODEL.to_vec(),
		adi_gpu::Color(0.5, 1.0, 0.5, 0.5)));
	// image
	display.push(adi_gpu::Shape::Texture(IMAGE_VERTICES.to_vec(), image_tex,
		IMAGE_TEXCOORDS.to_vec()));
	// logo
	display.push(adi_gpu::Shape::Texture(IMAGE_VERTICES.to_vec(), logo_tex,
		IMAGE_TEXCOORDS.to_vec()));
	// triangle
	display.push(adi_gpu::Shape::Gradient(TRIANGLE_MODEL,
		&[
			adi_gpu::Color(1.0, 0.0, 0.0, 1.0),
			adi_gpu::Color(0.0, 1.0, 0.0, 1.0),
			adi_gpu::Color(0.0, 0.0, 1.0, 1.0)
		]));
	// new!
//	display.push(adi_gpu::Shape::FadeTexture(vertices, image, texcoords,
//		0.5));
	// new!
//	display.push(adi_gpu::Shape::TintTexture(vertices, image, texcoords,
//		adi_gpu::Color(1.0, 1.0, 0.0, 0.5)));
}

pub fn update(_: &mut adi_gpu::Display) {
	
}

fn main() {
	let display_icon = aci_png::decode(include_bytes!("../res/icon.png"))
		.unwrap();

	let mut display = adi_gpu::Display::new("adi_gpu Animated Example",
		&display_icon);

	let mut queue = adi_gpu::input::Queue::new();

	let logo_texture = display.push_texture(display_icon);
	let plopgrizzly_texture = display.push_texture(aci_png::decode(
		include_bytes!("../res/plopgrizzly.png")).unwrap());
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
