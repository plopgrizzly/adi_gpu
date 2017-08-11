// Willow Graphics API
//
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE
//
// examples/minimal/main.rs

#[macro_use]
extern crate willow;
extern crate aci_png;

const SQUARE_MODEL: &'static [f32] = &include!("../res/square.data");
const IMAGE_MODEL: &'static [f32] = &include!("../res/image.data");
const TRIANGLE_MODEL: &'static [f32] = &include!("../res/triangle.data");

pub fn resize(display: &mut willow::Display) {
	println!("!Resizing...");

	// square
	display.push(willow::Shape::Solid(
		translate!(0.5; Z) * SQUARE_MODEL.to_vec(),
		willow::Color(0.5, 1.0, 0.5, 0.5)));
	// image
//	display.push(willow::Shape::Textured(IMAGE_MODEL, image, texcoords));
	// logo
//	display.push(willow::Shape::Textured(IMAGE_MODEL, image, texcoords));
	// triangle
/*	display.push(willow::Shape::Gradient(TRIANGLE_MODEL,
		&[
			willow::Color(1.0, 0.0, 0.0, 1.0),
			willow::Color(0.0, 1.0, 0.0, 1.0),
			willow::Color(0.0, 0.0, 1.0, 1.0)
		]));*/
	// new!
//	display.push(willow::Shape::FadedTextured(vertices, image, texcoords));
	// new!
//	display.push(willow::Shape::ColoredTextured(vertices, image, texcoords));

	
}

pub fn update(display: &mut willow::Display) {
	
}

fn main() {
	let display_icon = aci_png::decode(include_bytes!("../res/icon.png"))
		.unwrap();

	let mut display = willow::Display::new("Willow Animated Example",
		&display_icon);

	let mut queue = willow::input::Queue::new();

	resize(&mut display);

	'app: loop {
		update(&mut display);

		display.update(&mut queue);

		for input in queue.iter() {
			use willow::input::Input::*;
			use willow::input::Msg::*;

			match *input {
				Msg(Quit) | Msg(Back) => break 'app,
				Resize => resize(&mut display),
				_ => {},
			}
		}
	}
}
