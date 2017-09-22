// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// examples/minimal/main.rs

extern crate adi_gpu;
extern crate aci_png;

fn main() {
	let display_icon = aci_png::decode(include_bytes!("../res/icon.png"))
		.unwrap();

	let mut display = adi_gpu::Display::new("Willow Minimal Example",
		&display_icon);

	let mut queue = adi_gpu::input::Queue::new();

	'app: loop {
		display.update(&mut queue);

		for input in queue.iter() {
			use adi_gpu::input::Input::*;
			use adi_gpu::input::Msg::*;

			match *input {
				Msg(Quit) | Msg(Back) => break 'app,
				_ => {},
			}
		}
	}
}
