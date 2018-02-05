// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/renderer/mod.rs

use std::mem;

// use awi::Window;
use awi::WindowConnection;

use self::ffi::vulkan;
// use self::ffi::NativeRenderer;
// use RenderOps;

mod ffi;

use asi_vulkan;
use asi_vulkan::types::*;
use asi_vulkan::Connection;

// TODO
use asi_vulkan::TransformUniform;
use asi_vulkan::FogUniform;
use asi_vulkan::Style;
use asi_vulkan::VwInstance;

#[derive(Clone)] #[repr(C)] struct TransformFullUniform {
	mat4: [f32; 16],
	hcam: u32,
}

#[derive(Clone)] #[repr(C)] struct TransformAndFadeUniform {
	mat4: [f32; 16],
	fade: f32,
	hcam: u32,
}

#[derive(Clone)] #[repr(C)] struct TransformAndColorUniform {
	mat4: [f32; 16],
	vec4: [f32; 4],
	hcam: u32,
}

pub enum ShapeHandle {
	Alpha(u32),
	Opaque(u32),
	Gui(u32),
}

// TODO: no gcc dependency.
#[repr(C)]
pub struct Vw {
	pub instance: VkInstance, // Vulkan instance
	surface: VkSurfaceKHR, // Surface that we render to.
	present_queue_index: u32,
	present_queue: VkQueue,
	gpu: VkPhysicalDevice,
	device: VkDevice, // The logical device
	command_buffer: VkCommandBuffer,
	swapchain: VkSwapchainKHR,
	width:u32, height:u32, // Swapchain Dimensions.
	present_images: [VkImage; 2], // 2 for double-buffering
	frame_buffers: [VkFramebuffer; 2], // 2 for double-buffering
	color_format: VkFormat,
	image_count: u32, // 1 (single-buffering) or 2 (double-buffering)
	submit_fence: VkFence, // The submit fence
	present_image_views: [VkImageView; 2], // 2 for double-buffering
	depth_image: VkImage,
	depth_image_view: VkImageView,
	depth_image_memory: VkDeviceMemory,
	render_pass: VkRenderPass,
	next_image_index: u32,
	presenting_complete_sem: VkSemaphore,
	rendering_complete_sem: VkSemaphore,
	offsets: u64, // VkDeviceSize
	present_mode: VkPresentModeKHR,
	sampled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Texture {
	mappable_image: VkImage,
	mappable_memory: VkDeviceMemory,
	image: VkImage,
	memory: VkDeviceMemory,
	sampler: VkSampler,
	view: VkImageView,
	pub(super) w: u32,
	pub(super) h: u32,
	pitch: u32,
	staged: bool,
}

#[derive(Clone)]
pub struct Shape {
	num_buffers: usize,
	buffers: [VkBuffer; 3],
	vertice_count: u32,
	instance: VwInstance,
	offset: u64,
	bounds: [(f32, f32); 3], // xMinMax, yMinMax, zMinMax
	center: ::math::Vec3<f32>,
	position: ::math::Vec3<f32>,
}

pub struct Model {
	vertex_buffer: VkBuffer,
	#[allow(unused)] // TODO: Use for freeing
	vertex_memory: VkDeviceMemory,
	vertex_count: u32,
	indice_count: u32,
	offset: u64,
	bounds: [(f32, f32); 3], // xMinMax, yMinMax, zMinMax
	center: ::math::Vec3<f32>,
}

pub struct TexCoords {
	vertex_buffer: VkBuffer,
	#[allow(unused)] // TODO: Use for freeing
	vertex_memory: VkDeviceMemory,
	vertex_count: u32,
}

pub struct Gradient {
	vertex_buffer: VkBuffer,
	#[allow(unused)] // TODO: Use for freeing
	vertex_memory: VkDeviceMemory,
	vertex_count: u32,
}

impl ::math::Pos for Shape {
	fn posf(&self) -> ::math::Vec3<f32> {
		self.position
	}

	fn posi(&self) -> ::math::Vec3<i32> {
		self.position.into()
	}
}

impl Shape {
// TODO
/*	pub fn animate(window: &mut Window, index: usize, i: usize,
		texture: *const NativeTexture, style: Style)
	{
		let hastx = window.sprites[index].hastx;

		// Must be same style
		if hastx {
			if (texture as *const _ as usize) == 0 {
				panic!("Can't set Style of a Sprite initialized\
					with Style::Texture to Style::Solid");
			}
		} else {
			if (texture as *const _ as usize) != 0 {
				panic!("Can't set Style of a Sprite initialized\
					with Style::Solid to Style::Texture");
			}
		}

		// Free old Style, and set new uniform buffers.
		unsafe {
			asi_vulkan::destroy_uniforms(&window.vw, &mut
				window.sprites[index].instances[i].instance);
			window.sprites[index].instances[i].instance =
				vw_vulkan_uniforms(&window.vw, style, texture,
					if hastx { 1 } else { 0 });
		}
		// TODO: Optimize when using same value from vw_vulkan_uniforms
		// Set texture
//		unsafe {
//			vw_vulkan_txuniform(&window.vw,
//				&mut window.sprites[index].shape.instances[i].instance, texture,
//				if window.sprites[index].shape.hastx { 1 } else { 0 });
//		}
		Shape::enable(window, index, i, true);
	}

	pub fn vertices(window: &Window, index: usize, v: &[f32]) {
		vulkan::copy_memory(window.vw.device,
			window.sprites[index].shape.vertex_buffer_memory, v);
	}*/
}

extern {
	fn vw_vulkan_draw_begin(v: *mut Vw, r: f32, g: f32, b: f32) -> ();
// TODO: In Rust
	fn vw_vulkan_draw_update(v: *mut Vw) -> ();
}

// TODO: Move FFI to asi_vulkan.
fn draw_begin(connection: &Connection, r: f32, g: f32, b: f32) {
	
}

// TODO: Move FFI to asi_vulkan.
fn draw_update(connection: &Connection) {
	
}

fn swapchain_resize(connection: &Connection, vw: &mut Vw) -> () {
	unsafe {
		// Link swapchain to vulkan instance.
		asi_vulkan::create_swapchain(
			connection,
			vw.surface,
			vw.gpu,
			vw.device,
			&mut vw.swapchain,
			vw.width,
			vw.height,
			&mut vw.image_count,
			vw.color_format.clone(),
			vw.present_mode.clone(),
			&mut vw.present_images[0]);

		// Link Image Views for each framebuffer
		asi_vulkan::create_image_view(
			connection,
			vw.device,
			&vw.color_format,
			&mut vw.submit_fence,
			vw.image_count,
			&mut vw.present_images,
			&mut vw.present_image_views,
			vw.command_buffer,
			vw.present_queue,
		);

		// Link Depth Buffer to swapchain
		let (img, view) = asi_vulkan::create_depth_buffer(
			connection,
			vw.device,
			vw.gpu,
			vw.command_buffer,
			vw.submit_fence,
			vw.present_queue,
			vw.width,
			vw.height,
		);

		vw.depth_image = img.image;
		vw.depth_image_view = view;
		vw.depth_image_memory = img.image_memory;

		// Link Render Pass to swapchain
		vw.render_pass = asi_vulkan::create_render_pass(
			connection,
			vw.device,
			&vw.color_format,
		);

		// Link Framebuffers to swapchain
		asi_vulkan::create_framebuffers(
			connection,
			vw.device,
			vw.image_count,
			vw.render_pass,
			&vw.present_image_views,
			vw.depth_image_view,
			vw.width,
			vw.height,
			&mut vw.frame_buffers,
		);
	}
}

fn swapchain_delete(connection: &Connection, vw: &mut Vw) {
	unsafe {
		asi_vulkan::destroy_swapchain(
			connection,
			vw.device,
			&vw.frame_buffers,
			&vw.present_image_views,
			vw.depth_image_view,
			vw.render_pass,
			vw.image_count,
			vw.depth_image,
			vw.swapchain,
			vw.depth_image_memory,
		);
	}
}

fn new_texture(connection: &Connection, vw: &mut Vw, width: u32, height: u32)
	-> Texture
{
//	let mut format_props = unsafe { mem::uninitialized() };
	let staged = !vw.sampled;

	let mappable_image = asi_vulkan::Image::new(connection, vw.device,
		vw.gpu, width, height, VkFormat::R8g8b8a8Srgb,
		VkImageTiling::Linear,
		if staged { VkImageUsage::TransferSrcBit }
		else { VkImageUsage::SampledBit },
		VkImageLayout::Preinitialized,
		0x00000006 /* visible|coherent */
	);

	let layout = unsafe {
		asi_vulkan::subres_layout(connection, vw.device,
			mappable_image.image)
	};

	let pitch = layout.row_pitch;

	let (image, image_memory) = if staged {
		let i = asi_vulkan::Image::new(connection, vw.device, vw.gpu,
			width, height, VkFormat::R8g8b8a8Srgb,
			VkImageTiling::Optimal,
			VkImageUsage::TransferDstAndUsage,
			VkImageLayout::Undefined, 0);

		(i.image, i.image_memory)
	} else {
		let i = &mappable_image;

		(i.image, i.image_memory)
	};
//
	let sampler = unsafe { asi_vulkan::new_sampler(connection, vw.device) };

	let view = unsafe {
		asi_vulkan::create_imgview(connection, vw.device, image,
			VkFormat::R8g8b8a8Srgb, true)
	};
//
	Texture {
		staged, mappable_image: mappable_image.image,
		mappable_memory: mappable_image.image_memory,
		image: image, memory: image_memory, view,
		pitch: pitch as u32, sampler, w: width, h: height,
	}
}

fn set_texture(connection: &Connection, vw: &mut Vw, texture: &mut Texture,
	rgba: &[u32])
{
	if texture.pitch != 4 {
		vulkan::copy_memory_pitched(connection, vw.device,
			texture.memory, rgba, texture.w as isize,
			texture.h as isize, texture.pitch as isize);
	} else {
		vulkan::copy_memory(connection, vw.device, texture.memory,
			rgba.as_ptr(), mem::size_of::<u32>() * rgba.len());
	}

	if texture.staged {
		// Use optimal tiled image - create from linear tiled image

		// Copy data from linear image to optimal image.
		unsafe {
			asi_vulkan::copy_image(connection,
				vw.command_buffer, texture.mappable_image,
				texture.image, texture.w, texture.h
			);
		}
	} else {
		// Use a linear tiled image for the texture, is supported
		texture.image = texture.mappable_image;
		texture.memory = texture.mappable_memory;
	}
}

/*pub fn make_styles(vw: &mut Vw, extrashaders: &[Shader], shaders: &mut Vec<Style>)
{
	let mut shadev = Vec::new();
	let default_shaders = [
//		Shader::create(vw, include_bytes!("res/texture-vert.spv"),
//			include_bytes!("res/texture-frag.spv"), 1),
	];
	shadev.extend(default_shaders.iter().cloned());
	shadev.extend(extrashaders.iter().cloned());

	*shaders = vec![Style { pipeline: 0, descsetlayout: 0,
		pipeline_layout: 0 }; shadev.len()];
	unsafe {
		vw_vulkan_pipeline(&mut shaders[0], vw, &shadev[0],
			shadev.len() as u32);
	}
}*/

impl Vw {
	pub fn new(window_name: &str, window_connection: WindowConnection) -> (Connection, Vw) {
		let connection = vulkan::vulkan::Vulkan::new(window_name).unwrap();

		let instance = connection.0.vk;
		let surface = vulkan::create_surface::create_surface(	
			instance, window_connection);
		let (gpu, pqi, sampled) = unsafe {
			asi_vulkan::get_gpu(&connection.0, instance, surface)
		};
		let device = unsafe {
			asi_vulkan::create_device(&connection.0, gpu, pqi)
		};
		let present_queue = unsafe {
			asi_vulkan::create_queue(&connection.0, device, pqi)
		};
		let command_buffer = unsafe {
			asi_vulkan::create_command_buffer(&connection.0,
				device, pqi)
		}.0;
		let color_format = unsafe {
			asi_vulkan::get_color_format(&connection.0,
				gpu, surface)
		};
		let image_count = unsafe {
			asi_vulkan::get_buffering(&connection.0, gpu, surface)
		};
		let present_mode = unsafe {
			asi_vulkan::get_present_mode(&connection.0, gpu,
				surface)
		};

		let mut vw = Vw {
			instance, surface,
			present_queue_index: pqi,
			present_queue, gpu, device, command_buffer,
			swapchain: unsafe { mem::zeroed() },
			width: 640, height: 360, // TODO
			present_images: unsafe { mem::zeroed() },
			frame_buffers: unsafe { mem::uninitialized() },
			color_format,
			image_count,
			submit_fence: unsafe { mem::zeroed() },
			present_image_views: [unsafe { mem::zeroed() }; 2],
			depth_image: unsafe { mem::zeroed() },
			depth_image_view: unsafe { mem::zeroed() },
			depth_image_memory: unsafe { mem::zeroed() },
			render_pass: unsafe { mem::uninitialized() },
			next_image_index: 0,
			presenting_complete_sem: unsafe { mem::uninitialized() },
			rendering_complete_sem: unsafe { mem::uninitialized() },
			offsets: 0,
			present_mode,
			sampled,
		};

		swapchain_resize(&connection.0, &mut vw);

		(connection.0, vw)
	}
}

fn projection(ratio: f32, fov: f32) -> ::Transform {
	let scale = (fov * 0.5 * ::std::f32::consts::PI / 180.).tan().recip();
	let yscale = scale * ratio;

	::Transform([
		scale,	0.,	0.,	0.,
		0.,	yscale,	0.,	0.,
		0.,	0.,	1.,	1.,
		0.,	0.,	0., 	1.,
	])
}

fn draw_shape(connection: &Connection, cmdbuf: VkCommandBuffer, shape: &Shape) {
	unsafe {
		asi_vulkan::cmd_bind_vb(connection,
			cmdbuf,
			&shape.buffers[..shape.num_buffers],
			shape.offset);

		asi_vulkan::cmd_bind_pipeline(&connection,
			cmdbuf,
			shape.instance.pipeline.pipeline);

		asi_vulkan::cmd_bind_descsets(&connection,
			cmdbuf,
			shape.instance.pipeline.pipeline_layout,
			shape.instance.desc_set);
	}

	vulkan::cmd_draw(&connection,
		cmdbuf,
		shape.vertice_count, 0);
//		shape.offset as i32);
}

pub struct Renderer {
	vw: Vw,
	ar: f32,
	connection: Connection,
	opaque_octree: ::math::Octree<Shape>,
	alpha_octree: ::math::Octree<Shape>,
	gui_vec: Vec<Shape>,
	opaque_sorted: Vec<u32>,
	alpha_sorted: Vec<u32>,
//	opaque_points: ::math::Points,
//	alpha_points: ::math::Points,
//	opaque_shapes: Vec<Shape>,
//	alpha_shapes: Vec<Shape>,
	models: Vec<Model>,
	texcoords: Vec<TexCoords>,
	gradients: Vec<Gradient>,
	style_solid: Style,
	style_nasolid: Style,
	style_bsolid: Style,
	style_texture: Style,
	style_natexture: Style,
	style_btexture: Style,
	style_gradient: Style,
	style_nagradient: Style,
	style_bgradient: Style,
	style_faded: Style,
	style_bfaded: Style,
	style_tinted: Style,
	style_natinted: Style,
	style_btinted: Style,
	style_complex: Style,
	style_nacomplex: Style,
	style_bcomplex: Style,
	projection: ::Transform,
	camera_memory: asi_vulkan::Memory<TransformUniform>,
	effect_memory: asi_vulkan::Memory<FogUniform>,
	clear_color: (f32, f32, f32),
	frustum: ::math::Frustum,
	xyz: (f32,f32,f32),
	rotate_xyz: (f32,f32,f32),
}

impl Renderer {
	pub fn new(window_name: &str, window_connection: WindowConnection,
		clear_color: (f32, f32, f32), fog: (f32, f32))
		-> Renderer
	{
		let (connection, vw) = Vw::new(window_name, window_connection);
		let solid_vert = asi_vulkan::ShaderModule::new(&connection,
			vw.device, include_bytes!(
			"../native_renderer/vulkan/res/solid-vert.spv"));
		let solid_frag = asi_vulkan::ShaderModule::new(&connection,
			vw.device, include_bytes!(
			"../native_renderer/vulkan/res/solid-frag.spv"));
		let solid_nafrag = asi_vulkan::ShaderModule::new(&connection,
			vw.device, include_bytes!(
			"../native_renderer/vulkan/res/solid-nafrag.spv"));
		let solid_bfrag = asi_vulkan::ShaderModule::new(&connection,
			vw.device, include_bytes!(
			"../native_renderer/vulkan/res/solid-bfrag.spv"));
		let texture_vert = asi_vulkan::ShaderModule::new(&connection,
			vw.device, include_bytes!(
			"../native_renderer/vulkan/res/texture-vert.spv"));
		let texture_frag = asi_vulkan::ShaderModule::new(&connection,
			vw.device, include_bytes!(
			"../native_renderer/vulkan/res/texture-frag.spv"));
		let texture_nafrag = asi_vulkan::ShaderModule::new(&connection,
			vw.device, include_bytes!(
			"../native_renderer/vulkan/res/texture-nafrag.spv"));
		let texture_bfrag = asi_vulkan::ShaderModule::new(&connection,
			vw.device, include_bytes!(
			"../native_renderer/vulkan/res/texture-bfrag.spv"));
		let gradient_vert = asi_vulkan::ShaderModule::new(&connection,
			vw.device, include_bytes!(
			"../native_renderer/vulkan/res/gradient-vert.spv"));
		let gradient_frag = asi_vulkan::ShaderModule::new(&connection,
			vw.device, include_bytes!(
			"../native_renderer/vulkan/res/gradient-frag.spv"));
		let gradient_nafrag = asi_vulkan::ShaderModule::new(&connection,
			vw.device, include_bytes!(
			"../native_renderer/vulkan/res/gradient-nafrag.spv"));
		let gradient_bfrag = asi_vulkan::ShaderModule::new(&connection,
			vw.device, include_bytes!(
			"../native_renderer/vulkan/res/gradient-bfrag.spv"));
		let faded_vert = asi_vulkan::ShaderModule::new(&connection,
			vw.device, include_bytes!(
			"../native_renderer/vulkan/res/faded-vert.spv"));
		let faded_frag = asi_vulkan::ShaderModule::new(&connection,
			vw.device, include_bytes!(
			"../native_renderer/vulkan/res/faded-frag.spv"));
		let faded_bfrag = asi_vulkan::ShaderModule::new(&connection,
			vw.device, include_bytes!(
			"../native_renderer/vulkan/res/faded-bfrag.spv"));
		let tinted_vert = asi_vulkan::ShaderModule::new(&connection,
			vw.device, include_bytes!(
			"../native_renderer/vulkan/res/gradient-vert.spv"));
		let tinted_frag = asi_vulkan::ShaderModule::new(&connection,
			vw.device, include_bytes!(
			"../native_renderer/vulkan/res/gradient-frag.spv"));
		let tinted_nafrag = asi_vulkan::ShaderModule::new(&connection,
			vw.device, include_bytes!(
			"../native_renderer/vulkan/res/gradient-nafrag.spv"));
		let tinted_bfrag = asi_vulkan::ShaderModule::new(&connection,
			vw.device, include_bytes!(
			"../native_renderer/vulkan/res/gradient-bfrag.spv"));
		let complex_vert = asi_vulkan::ShaderModule::new(&connection,
			vw.device, include_bytes!(
			"../native_renderer/vulkan/res/gradient-vert.spv"));
		let complex_frag = asi_vulkan::ShaderModule::new(&connection,
			vw.device, include_bytes!(
			"../native_renderer/vulkan/res/gradient-frag.spv"));
		let complex_nafrag = asi_vulkan::ShaderModule::new(&connection,
			vw.device, include_bytes!(
			"../native_renderer/vulkan/res/gradient-nafrag.spv"));
		let complex_bfrag = asi_vulkan::ShaderModule::new(&connection,
			vw.device, include_bytes!(
			"../native_renderer/vulkan/res/gradient-bfrag.spv"));
		let style_solid = asi_vulkan::new_pipeline(&connection,
			vw.device, vw.render_pass, vw.width, vw.height,
			&solid_vert, &solid_frag, 0, 1, true);
		let style_nasolid = asi_vulkan::new_pipeline(&connection,
			vw.device, vw.render_pass, vw.width, vw.height,
			&solid_vert, &solid_nafrag, 0, 1, false);
		let style_bsolid = asi_vulkan::new_pipeline(&connection,
			vw.device, vw.render_pass, vw.width, vw.height,
			&solid_vert, &solid_bfrag, 0, 1, true);
		let style_texture = asi_vulkan::new_pipeline(&connection,
			vw.device, vw.render_pass, vw.width, vw.height,
			&texture_vert, &texture_frag, 1, 2, true);
		let style_natexture = asi_vulkan::new_pipeline(&connection,
			vw.device, vw.render_pass, vw.width, vw.height,
			&texture_vert, &texture_nafrag, 1, 2, false);
		let style_btexture = asi_vulkan::new_pipeline(&connection,
			vw.device, vw.render_pass, vw.width, vw.height,
			&texture_vert, &texture_bfrag, 1, 2, true);
		let style_gradient = asi_vulkan::new_pipeline(&connection,
			vw.device, vw.render_pass, vw.width, vw.height,
			&gradient_vert, &gradient_frag, 0, 2, true);
		let style_nagradient = asi_vulkan::new_pipeline(&connection,
			vw.device, vw.render_pass, vw.width, vw.height,
			&gradient_vert, &gradient_nafrag, 0, 2, false);
		let style_bgradient = asi_vulkan::new_pipeline(&connection,
			vw.device, vw.render_pass, vw.width, vw.height,
			&gradient_vert, &gradient_bfrag, 0, 2, true);
		let style_faded = asi_vulkan::new_pipeline(&connection,
			vw.device, vw.render_pass, vw.width, vw.height,
			&faded_vert, &faded_frag, 1, 2, true);
		let style_bfaded = asi_vulkan::new_pipeline(&connection,
			vw.device, vw.render_pass, vw.width, vw.height,
			&faded_vert, &faded_bfrag, 1, 2, true);
		let style_tinted = asi_vulkan::new_pipeline(&connection,
			vw.device, vw.render_pass, vw.width, vw.height,
			&tinted_vert, &tinted_frag, 1, 2, true);
		let style_natinted = asi_vulkan::new_pipeline(&connection,
			vw.device, vw.render_pass, vw.width, vw.height,
			&tinted_vert, &tinted_nafrag, 1, 2, false);
		let style_btinted = asi_vulkan::new_pipeline(&connection,
			vw.device, vw.render_pass, vw.width, vw.height,
			&tinted_vert, &tinted_bfrag, 1, 2, true);
		let style_complex = asi_vulkan::new_pipeline(&connection,
			vw.device, vw.render_pass, vw.width, vw.height,
			&complex_vert, &complex_frag, 1, 3, true);
		let style_nacomplex = asi_vulkan::new_pipeline(&connection,
			vw.device, vw.render_pass, vw.width, vw.height,
			&complex_vert, &complex_nafrag, 1, 3, false);
		let style_bcomplex = asi_vulkan::new_pipeline(&connection,
			vw.device, vw.render_pass, vw.width, vw.height,
			&complex_vert, &complex_bfrag, 1, 3, true);

		let ar = vw.width as f32 / vw.height as f32;
		let projection = projection(ar, 90.0);
		let (camera_memory, effect_memory) = unsafe {
			asi_vulkan::vw_camera_new(&connection,vw.device,vw.gpu,
				(clear_color.0, clear_color.1, clear_color.2,
					1.0), (fog.0, fog.1))
		};

		let mut renderer = Renderer {
			vw, ar, connection, projection,
			camera_memory, effect_memory,
			alpha_octree: ::math::Octree::new(),
			opaque_octree: ::math::Octree::new(),
			gui_vec: Vec::new(),
			opaque_sorted: Vec::new(),
			alpha_sorted: Vec::new(),
//			alpha_points: ::math::Points::new(),
//			opaque_points: ::math::Points::new(),
//			alpha_shapes: Vec::new(),
//			opaque_shapes: Vec::new(),
			gradients: Vec::new(),
			models: Vec::new(),
			texcoords: Vec::new(),
			style_solid, style_nasolid, style_bsolid,
			style_texture, style_natexture, style_btexture,
			style_gradient, style_nagradient, style_bgradient,
			style_faded, style_bfaded,
			style_tinted, style_natinted, style_btinted,
			style_complex, style_nacomplex, style_bcomplex,
			clear_color,
			frustum: ::math::Frustum::new(
				::math::Vec3::new(0.0, 0.0, 0.0),
				fog.0 + fog.1, 90.0,
				2.0 * ((45.0 * ::std::f32::consts::PI / 180.0).tan() / ar).atan(),
				0.0, 0.0), // TODO: FAR CLIP PLANE
			xyz: (0.0, 0.0, 0.0),
			rotate_xyz: (0.0, 0.0, 0.0),
		};

		renderer.camera();

		renderer
	}

	pub fn bg_color(&mut self, rgb: (f32, f32, f32)) {
		self.clear_color = rgb;
	}

	pub fn update(&mut self) {
		let matrix = ::Transform::new()
			.rotate(self.rotate_xyz.0, self.rotate_xyz.1,
				self.rotate_xyz.2)
			.translate(self.xyz.0, self.xyz.1, self.xyz.2);

		unsafe {
			self.vw.presenting_complete_sem = asi_vulkan::new_semaphore(
				&self.connection,
				self.vw.device,
			);

			self.vw.next_image_index = asi_vulkan::get_next_image(
				&self.connection,
				self.vw.device,
				&mut self.vw.presenting_complete_sem,
				self.vw.swapchain,
			);

			self.vw.rendering_complete_sem = asi_vulkan::new_semaphore(
				&self.connection,
				self.vw.device,
			);

			vw_vulkan_draw_begin(&mut self.vw, self.clear_color.0,
				self.clear_color.1, self.clear_color.2);
		}

		let frustum = matrix * self.frustum;

//		self.opaque_octree.print();
//		println!("FRUSTUM {:?}", frustum);

		self.opaque_octree.nearest(&mut self.opaque_sorted, frustum);
		for id in self.opaque_sorted.iter() {
			let shape = &self.opaque_octree[*id];

			draw_shape(&self.connection, self.vw.command_buffer, shape);
		}

		self.alpha_octree.farthest(&mut self.alpha_sorted, frustum);
		for id in self.alpha_sorted.iter() {
			let shape = &self.alpha_octree[*id];

			draw_shape(&self.connection, self.vw.command_buffer, shape);
		}

		for shape in self.gui_vec.iter() {
			draw_shape(&self.connection, self.vw.command_buffer, shape);
		}

		unsafe {
			vw_vulkan_draw_update(&mut self.vw);
		}
	}

	pub fn resize(&mut self, size: (u32, u32)) {
		self.vw.width = size.0;
		self.vw.height = size.1;
		self.ar = size.0 as f32 / size.1 as f32;
		self.frustum = ::math::Frustum::new(
			self.frustum.center,
			self.frustum.radius,
			90.0, 2.0 * ((45.0 * ::std::f32::consts::PI / 180.0)
				.tan() / self.ar).atan(),
			self.frustum.xrot, self.frustum.yrot);

		swapchain_delete(&self.connection, &mut self.vw);
		swapchain_resize(&self.connection, &mut self.vw);

		self.projection = projection(self.ar, 90.0);
		self.camera();
	}

	pub fn texture(&mut self, width: u32, height: u32, rgba: &[u32])
		-> Texture
	{
		let mut texture = new_texture(&self.connection, &mut self.vw,
			width, height);

		set_texture(&self.connection, &mut self.vw, &mut texture, rgba);

		texture
	}

	pub fn set_texture(&mut self, texture: &mut Texture, rgba: &[u32]) {
		set_texture(&self.connection, &mut self.vw, texture, rgba);
	}

	/// Push a model (collection of vertices) into graphics memory.
	pub fn model(&mut self, vertices: &[f32], indices: &[u32]) -> usize {
		let (vertex_buffer, vertex_memory, offset) = unsafe {
			asi_vulkan::new_shape(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				vertices,
				indices,
			)
		};

		let a = self.models.len();

		let mut xtot = vertices[0];
		let mut ytot = vertices[1];
		let mut ztot = vertices[2];
		let mut xmin = vertices[0];
		let mut ymin = vertices[1];
		let mut zmin = vertices[2];
		let mut xmax = vertices[0];
		let mut ymax = vertices[1];
		let mut zmax = vertices[2];

		for i in 4..vertices.len() {
			match i % 4 {
				0 => {
					let x = vertices[i];
					xtot += x;
					if x < xmin {
						xmin = x;
					} else if x > xmax {
						xmax = x;
					}
				},
				1 => {
					let y = vertices[i];
					ytot += y;
					if y < ymin {
						ymin = y;
					} else if y > ymax {
						ymax = y;
					}
				},
				2 => {
					let z = vertices[i];
					ztot += z;
					if z < zmin {
						zmin = z;
					} else if z > zmax {
						zmax = z;
					}
				},
				_ => { },
			}
		}

		let n = (vertices.len() / 4) as f32;

		self.models.push(Model {
			vertex_buffer,
			vertex_memory,
			vertex_count: vertices.len() as u32 / 4,
			indice_count: indices.len() as u32,
			offset,
			bounds: [(xmin, xmax), (ymin, ymax), (zmin, zmax)],
			center: ::math::Vec3::new(xtot / n, ytot / n, ztot / n),
		});

		a
	}

	/// Push texture coordinates (collection of vertices) into graphics
	/// memory.
	pub fn texcoords(&mut self, texcoords: &[f32]) -> usize {
		let (vertex_buffer, vertex_memory) = unsafe {
			asi_vulkan::new_buffer(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				texcoords,
			)
		};

		let a = self.texcoords.len();

		self.texcoords.push(TexCoords {
			vertex_buffer,
			vertex_memory,
			vertex_count: texcoords.len() as u32 / 4,
		});

		a
	}

	/// Push colors per vertex into graphics memory.
	pub fn colors(&mut self, colors: &[f32]) -> usize {
		let (vertex_buffer, vertex_memory) = unsafe {
			asi_vulkan::new_buffer(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				colors,
			)
		};

		let a = self.gradients.len();

		self.gradients.push(Gradient {
			vertex_buffer,
			vertex_memory,
			vertex_count: colors.len() as u32 / 4,
		});

		a
	}

	pub fn textured(&mut self, model: usize, mat4: [f32; 16],
		texture: Texture, texcoords: usize, alpha: bool, blend: bool,
		fog: bool, camera: bool) -> ShapeHandle
	{
		if self.models[model].vertex_count
			!= self.texcoords[texcoords].vertex_count
		{
			panic!("TexCoord length doesn't match vertex length");
		}

		// Add an instance
		let instance = unsafe {
			asi_vulkan::vw_instance_new(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				if blend {
					self.style_btexture
				} else {
					if alpha {
						self.style_texture
					} else {
						self.style_natexture
					}
				},
				TransformFullUniform {
					mat4,
					hcam: fog as u32 + camera as u32,
				},
				&self.camera_memory, // TODO: at shader creation, not shape creation
				&self.effect_memory,
				texture.view,
				texture.sampler,
				true, // 1 texure
			)
		};

		let shape = Shape {
			instance,
			num_buffers: 2,
			buffers: [
				self.models[model].vertex_buffer,
				self.texcoords[texcoords].vertex_buffer,
				unsafe { mem::uninitialized() }
			],
			vertice_count: self.models[model].indice_count,
			offset: self.models[model].offset,
			bounds: self.models[model].bounds,
			center: self.models[model].center,
			position: ::Transform(mat4) * self.models[model].center,
		};

		println!("DBGU {}", fog as u32);

		if !camera && !fog {
			self.gui_vec.push(shape);
			ShapeHandle::Gui(self.gui_vec.len() as u32 - 1)
		} else if alpha {
			ShapeHandle::Alpha(self.alpha_octree.add(shape))
		} else {
			ShapeHandle::Opaque(self.opaque_octree.add(shape))
		}
	}

	pub fn solid(&mut self, model: usize, mat4: [f32; 16], color: [f32; 4],
		alpha: bool, blend: bool, fog: bool, camera: bool)
		-> ShapeHandle
	{
		// Add an instance
		let instance = unsafe {
			asi_vulkan::vw_instance_new(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				if blend {
					self.style_bsolid
				} else {
					if alpha {
						self.style_solid
					} else {
						self.style_nasolid
					}
				},
				TransformAndColorUniform {
					vec4: color,
					hcam: fog as u32 + camera as u32,
					mat4,
				},
				&self.camera_memory,
				&self.effect_memory,
				mem::zeroed(),
				mem::zeroed(),
				false, // no texure
			)
		};

		let shape = Shape {
			instance,
			num_buffers: 1,
			buffers: [
				self.models[model].vertex_buffer,
				unsafe { mem::uninitialized() },
				unsafe { mem::uninitialized() }
			],
			vertice_count: self.models[model].indice_count,
			offset: self.models[model].offset,
			bounds: self.models[model].bounds,
			center: self.models[model].center,
			position: ::Transform(mat4) * self.models[model].center,
		};

		if !camera && !fog {
			self.gui_vec.push(shape);
			ShapeHandle::Gui(self.gui_vec.len() as u32 - 1)
		} else if alpha {
			ShapeHandle::Alpha(self.alpha_octree.add(shape))
		} else {
			ShapeHandle::Opaque(self.opaque_octree.add(shape))
		}
	}

	pub fn gradient(&mut self, model: usize, mat4: [f32; 16], colors: usize,
		alpha: bool, blend: bool, fog: bool, camera: bool)
		-> ShapeHandle
	{
		if self.models[model].vertex_count
			!= self.gradients[colors].vertex_count
		{
			panic!("TexCoord length doesn't match gradient length");
		}

		// Add an instance
		let instance = unsafe {
			asi_vulkan::vw_instance_new(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				if blend {
					self.style_bgradient
				} else {
					if alpha {
						self.style_gradient
					} else {
						self.style_nagradient
					}
				},
				TransformFullUniform {
					mat4,
					hcam: fog as u32 + camera as u32,
				},
				&self.camera_memory,
				&self.effect_memory,
				mem::zeroed(),
				mem::zeroed(),
				false, // no texure
			)
		};

		let shape = Shape {
			instance,
			num_buffers: 2,
			buffers: [
				self.models[model].vertex_buffer,
				self.gradients[colors].vertex_buffer,
				unsafe { mem::uninitialized() }
			],
			vertice_count: self.models[model].indice_count,
			offset: self.models[model].offset,
			bounds: self.models[model].bounds,
			center: self.models[model].center,
			position: ::Transform(mat4) * self.models[model].center,
		};

		if !camera && !fog {
			self.gui_vec.push(shape);
			ShapeHandle::Gui(self.gui_vec.len() as u32 - 1)
		} else if alpha {
			ShapeHandle::Alpha(self.alpha_octree.add(shape))
		} else {
			ShapeHandle::Opaque(self.opaque_octree.add(shape))
		}
	}

	pub fn faded(&mut self, model: usize, mat4: [f32; 16], texture: Texture,
		texcoords: usize, fade_factor: f32, blend: bool, fog: bool,
		camera: bool) -> ShapeHandle
	{
		if self.models[model].vertex_count
			!= self.texcoords[texcoords].vertex_count
		{
			panic!("TexCoord length doesn't match vertex length");
		}

		// Add an instance
		let instance = unsafe {
			asi_vulkan::vw_instance_new(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				if blend {
					self.style_bfaded
				} else {
					self.style_faded
				},
				TransformAndFadeUniform {
					mat4,
					hcam: fog as u32 + camera as u32,
					fade: fade_factor,
				},
				&self.camera_memory,
				&self.effect_memory,
				texture.view,
				texture.sampler,
				true, // 1 texure
			)
		};

		let shape = Shape {
			instance,
			num_buffers: 2,
			buffers: [
				self.models[model].vertex_buffer,
				self.texcoords[texcoords].vertex_buffer,
				unsafe { mem::uninitialized() }
			],
			vertice_count: self.models[model].indice_count,
			offset: self.models[model].offset,
			bounds: self.models[model].bounds,
			center: self.models[model].center,
			position: ::Transform(mat4) * self.models[model].center,
		};

		if !camera && !fog {
			self.gui_vec.push(shape);
			ShapeHandle::Gui(self.gui_vec.len() as u32 - 1)
		} else {
			ShapeHandle::Alpha(self.alpha_octree.add(shape))
		}
	}

	pub fn tinted(&mut self, model: usize, mat4: [f32; 16],
		texture: Texture, texcoords: usize, color: [f32; 4],
		alpha: bool, blend: bool, fog: bool, camera: bool)
		-> ShapeHandle
	{
		if self.models[model].vertex_count
			!= self.texcoords[texcoords].vertex_count
		{
			panic!("TexCoord length doesn't match vertex length");
		}

		// Add an instance
		let instance = unsafe {
			asi_vulkan::vw_instance_new(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				if blend {
					self.style_btinted
				} else {
					if alpha {
						self.style_tinted
					} else {
						self.style_natinted
					}
				},
				TransformAndColorUniform {
					mat4,
					hcam: fog as u32 + camera as u32,
					vec4: color,
				},
				&self.camera_memory,
				&self.effect_memory,
				texture.view,
				texture.sampler,
				true, // 1 texure
			)
		};

		let shape = Shape {
			instance,
			num_buffers: 2,
			buffers: [
				self.models[model].vertex_buffer,
				self.texcoords[texcoords].vertex_buffer,
				unsafe { mem::uninitialized() }
			],
			vertice_count: self.models[model].indice_count,
			offset: self.models[model].offset,
			bounds: self.models[model].bounds,
			center: self.models[model].center,
			position: ::Transform(mat4) * self.models[model].center,
		};

		if !camera && !fog {
			self.gui_vec.push(shape);
			ShapeHandle::Gui(self.gui_vec.len() as u32 - 1)
		} else if alpha {
			ShapeHandle::Alpha(self.alpha_octree.add(shape))
		} else {
			ShapeHandle::Opaque(self.opaque_octree.add(shape))
		}
	}

	pub fn complex(&mut self, model: usize, mat4: [f32; 16],
		texture: Texture, texcoords: usize, colors: usize, alpha: bool,
		blend: bool, fog: bool, camera: bool) -> ShapeHandle
	{
		if self.models[model].vertex_count
			!= self.texcoords[texcoords].vertex_count ||
			self.models[model].vertex_count
			!= self.gradients[colors].vertex_count
		{
			panic!("TexCoord length doesn't match vertex length");
		}

		// Add an instance
		let instance = unsafe {
			asi_vulkan::vw_instance_new(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				if blend {
					self.style_bcomplex
				} else {
					if alpha {
						self.style_complex
					} else {
						self.style_nacomplex
					}
				},
				TransformFullUniform {
					mat4,
					hcam: fog as u32 + camera as u32,
				},
				&self.camera_memory,
				&self.effect_memory,
				texture.view,
				texture.sampler,
				true, // 1 texure
			)
		};

		let shape = Shape {
			instance,
			num_buffers: 3,
			buffers: [
				self.models[model].vertex_buffer,
				self.texcoords[texcoords].vertex_buffer,
				self.gradients[colors].vertex_buffer
			],
			vertice_count: self.models[model].indice_count,
			offset: self.models[model].offset,
			bounds: self.models[model].bounds,
			center: self.models[model].center,
			position: ::Transform(mat4) * self.models[model].center,
		};

		if !camera && !fog {
			self.gui_vec.push(shape);
			ShapeHandle::Gui(self.gui_vec.len() as u32 - 1)
		} else if alpha {
			ShapeHandle::Alpha(self.alpha_octree.add(shape))
		} else {
			ShapeHandle::Opaque(self.opaque_octree.add(shape))
		}
	}

	pub fn transform(&mut self, shape: &mut ShapeHandle,
		transform: &::Transform)
	{
		let uniform = TransformUniform {
			mat4: transform.0,
		};

		match *shape {
			ShapeHandle::Opaque(ref mut x) => {
				let mut shape = self.opaque_octree[*x].clone();

				shape.position = ::Transform(transform.0) *
					self.opaque_octree[*x].center;
				self.opaque_octree.modify(x, shape);

				vulkan::copy_memory(&self.connection, self.vw.device,
					self.opaque_octree[*x].instance.uniform_memory,
					&uniform, mem::size_of::<TransformUniform>());
			},
			ShapeHandle::Alpha(ref mut x) => {
				let mut shape = self.alpha_octree[*x].clone();

				shape.position = ::Transform(transform.0) *
					self.alpha_octree[*x].center;
				self.alpha_octree.modify(x, shape);

				vulkan::copy_memory(&self.connection, self.vw.device,
					self.alpha_octree[*x].instance.uniform_memory,
					&uniform, mem::size_of::<TransformUniform>());
			},
			ShapeHandle::Gui(x) => {
				let x = x as usize; // for indexing
				let mut shape = self.gui_vec[x].clone();

				shape.position = ::Transform(transform.0) *
					self.gui_vec[x].center;

				vulkan::copy_memory(&self.connection, self.vw.device,
					self.gui_vec[x].instance.uniform_memory,
					&uniform, mem::size_of::<TransformUniform>());
			},
		}
	}

	pub fn set_camera(&mut self, xyz: (f32,f32,f32),
		rotate_xyz: (f32,f32,f32))
	{
		self.xyz = xyz;
		self.rotate_xyz = rotate_xyz;
	}

	pub fn camera(&mut self) {
		self.camera_memory.data.mat4 = ::Transform::new()
			.translate(-self.xyz.0, -self.xyz.1, -self.xyz.2)
			.rotate(-self.rotate_xyz.0, -self.rotate_xyz.1,
				-self.rotate_xyz.2)
			.matrix(self.projection.0).0;

		self.camera_memory.update(&self.connection);
	}

	#[allow(unused)]
	pub fn fog(&mut self, color: ::math::Vec4<f32>, min: f32, max: f32) -> ()
	{
		self.effect_memory.data.fogc = [color.x, color.y, color.z, color.w];
		self.effect_memory.data.fogr = [min, max];

		self.effect_memory.update(&self.connection);
	}
}

impl Drop for Renderer {
	fn drop(&mut self) -> () {
		swapchain_delete(&self.connection, &mut self.vw);

		unsafe {
			asi_vulkan::destroy_surface(&self.connection,
				self.vw.surface);
			asi_vulkan::destroy_instance(&self.connection);
		}
	}
}
