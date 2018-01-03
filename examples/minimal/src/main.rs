// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// examples/minimal/main.rs

extern crate adi_gpu;
extern crate aci_png;

fn main() {
	let mut display = adi_gpu::Display::new("Willow Minimal Example",
		aci_png::decode(include_bytes!("../res/icon.png")).unwrap(),
		(0.25, 0.25, 1.0), (20.0, 10.0));

	'app: loop {
		// Go through this frame's input.
		while let Some(input) = display.input() {
			use adi_gpu::input::Input::*;
			use adi_gpu::input::Msg::*;

			match input {
				Msg(Quit) | Msg(Back) => break 'app,
				_ => {},
			}
		}

		display.update();
	}
}
