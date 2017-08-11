// Willow Graphics API
//
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE
//
// examples/minimal/main.rs

extern crate willow;
extern crate aci_png;

fn main() {
	let display_icon = aci_png::decode(include_bytes!("../res/icon.png"))
		.unwrap();

	let mut display = willow::Display::new("Willow Minimal Example",
		&display_icon);

	let mut queue = willow::input::Queue::new();

	'app: loop {
		display.update(&mut queue);

		for input in queue.iter() {
			use willow::input::Input::*;
			use willow::input::Msg::*;

			match *input {
				Msg(Quit) | Msg(Back) => break 'app,
				_ => {},
			}
		}
	}
}
