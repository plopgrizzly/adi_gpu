// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// examples/animated/main.rs

#[macro_use]
extern crate adi_gpu;
extern crate aci_png;

const SQUARE_MODEL: &'static [f32] = &include!("../res/square.data");
// const IMAGE_MODEL: &'static [f32] = &include!("../res/image.data");
const TRIANGLE_MODEL: &'static [f32] = &include!("../res/triangle.data");

pub fn resize(display: &mut adi_gpu::Display) {
	println!("!Resizing...");

	// square
	display.push(adi_gpu::Shape::Solid(
		translate!(0.5; Z) * SQUARE_MODEL.to_vec(),
		adi_gpu::Color(0.5, 1.0, 0.5, 0.5)));
	// image
//	display.push(adi_gpu::Shape::Texture(IMAGE_MODEL, image, &[
			
//		]));
	// logo
//	display.push(adi_gpu::Shape::Texture(IMAGE_MODEL, image, &[
			
//		]));
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

	resize(&mut display);

	'app: loop {
		update(&mut display);

		display.update(&mut queue);

		for input in queue.iter() {
			use adi_gpu::input::Input::*;
			use adi_gpu::input::Msg::*;

			match *input {
				Msg(Quit) | Msg(Back) => break 'app,
				Resize => resize(&mut display),
				_ => {},
			}
		}
	}
}
