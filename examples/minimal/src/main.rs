// "adi_gpu" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

extern crate adi_gpu;
extern crate aci_png;
extern crate awi;

use adi_gpu::DisplayTrait;

fn main() {
	let mut window = awi::Window::new("ADI_GPU Minimal Example",
		aci_png::decode(include_bytes!("../res/icon.png")).unwrap());
	let mut display = adi_gpu::Display::new(&window).unwrap();

	display.color((0.25, 0.25, 1.0));
	display.fog(Some((20.0, 10.0)));

	'app: loop {
		// Go through this frame's input.
		while let Some(input) = window.input() {
			use awi::Input::*;
			use awi::Msg::*;

			match input {
				Msg(Quit) | Msg(Back) => break 'app,
				_ => {},
			}
		}

		display.update();
	}
}
