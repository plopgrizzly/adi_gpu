// Aldaron's Device Interface / GPU
// Copyright (c) 2017-2018 Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// examples/animated/main.rs

extern crate adi_gpu;
extern crate aci_png;
extern crate awi;

use adi_gpu::DisplayTrait;

const SQUARE_INDICES: &'static [u32] = &include!("../res/square.indices");
const SQUARE_MODEL: &'static [f32] = &include!("../res/square.data");
const IMAGE_MODEL: &'static [f32] = &include!("../res/image.data");
const IMAGE_TEXCOORDS: &'static [f32] = &include!("../res/image.texc");
const TRIANGLE_MODEL: &'static [f32] = &include!("../res/triangle.data");
const TRIANGLE_INDICES: &'static [u32] = &include!("../res/triangle.indices");

pub fn resize(display: &mut adi_gpu::Display, image_tex: adi_gpu::Texture,
	logo_tex: adi_gpu::Texture) -> adi_gpu::Shape
{
	println!("!Resizing...");

	let square_model = display.model(SQUARE_MODEL, SQUARE_INDICES);
	let image_model = display.model(IMAGE_MODEL, SQUARE_INDICES);
	let triangle_model = display.model(TRIANGLE_MODEL, TRIANGLE_INDICES);

	let image_texcoords = display.texcoords(IMAGE_TEXCOORDS);

	let gradient = display.gradient(&[
			1.0, 0.0, 0.0, 1.0,
			0.0, 1.0, 0.0, 1.0,
			0.0, 0.0, 1.0, 1.0,
		]);

	let complexity = display.gradient(&[
			1.0, 0.0, 0.0, 1.0,
			0.0, 0.0, 1.0, 1.0,
			0.0, 1.0, 0.0, 1.0,
			0.0, 1.0, 0.0, 1.0 
		]);

	// square
	display.shape_solid(&square_model, adi_gpu::Mat4::new()
		.translate(0.0, 0.0, 0.5).rotate(0.25, 0.0, 0.0),
		[0.5, 1.0, 0.5, 0.5], false, false, true, true);
	// image
	display.shape_texture(&image_model, adi_gpu::Mat4::new(),
		image_tex, image_texcoords, false, false, true, true);
	// logo
	display.shape_texture(&image_model, adi_gpu::Mat4::new()
		.translate(0.5, 0.5, 0.0), logo_tex, image_texcoords,
		false, false, true, true);
	// triangle
	let tri = display.shape_gradient(&triangle_model, adi_gpu::Mat4::new(),
		gradient, false, false, true, true);
	// new!
	display.shape_faded(&image_model, adi_gpu::Mat4::new()
		.translate(1.0, 1.0, 0.0), image_tex, image_texcoords,
		0.5, false, true, true);
	// new!
	display.shape_tinted(&image_model, adi_gpu::Mat4::new()
		.translate(1.5, 1.5, 0.0), image_tex, image_texcoords,
		[1.0, 1.0, 0.0, 1.0], false, false, true, true);
	// new!
	display.shape_complex(&image_model, adi_gpu::Mat4::new()
		.translate(1.0, 1.5, 0.0), image_tex, image_texcoords,
		complexity, false, false, true, true);

	tri
}

fn main() {
	let mut window = awi::Window::new("ADI_GPU Animated Example",
		aci_png::decode(include_bytes!("../res/icon.png")).unwrap());
	let mut display = adi_gpu::Display::new(&window).unwrap();

	display.color((0.25, 0.25, 1.0));
	display.fog(Some((20.0, 10.0)));

	let logo_texture = display.texture(
		aci_png::decode(include_bytes!("../res/icon.png")).unwrap());
	let plopgrizzly_texture = display.texture(
		aci_png::decode(include_bytes!("../res/plopgrizzly.png"))
			.unwrap());

	let mut tri = resize(&mut display, logo_texture, plopgrizzly_texture);
	let mut ah = 0.0;

	'app: loop {
		// Go through this frame's input.
		while let Some(input) = window.input() {
			use awi::Input::*;
			use awi::Msg::*;

			match input {
				Msg(Quit) | Msg(Back) => break 'app,
				Resize => display.resize(window.wh()),
				_ => {},
			}
		}

		ah += 0.001;
		display.transform(&mut tri, &adi_gpu::Mat4::new().rotate(0.0, 0.0, ah));

		display.update();
		window.update();
	}
}
