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

use self::ffi::vulkan::ffi::types::*;
use self::ffi::vulkan::ffi::Connection;

#[repr(C)] struct TransformAndFadeUniform {
	mat4: [f32; 16],
	fade: f32,
}

#[repr(C)] struct TransformAndColorUniform {
	mat4: [f32; 16],
	vec4: [f32; 4],
}

#[repr(C)] struct TransformUniform {
	mat4: [f32; 16],
}

#[repr(C)]
// #[derive(Copy, Clone)] // TODO: don't copy this.
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

#[derive(Copy, Clone)]
pub struct Shader {
	vertex: VkShaderModule,
	fragment: VkShaderModule,
	textures: u32,
	vertex_buffers: u32,
	has_data: u8,
}

impl Shader {
	pub fn new(connection: &Connection, device: VkDevice,
		vert: &'static [u8], frag: &'static [u8], textures: u32,
		vertex_buffers: u32) -> Shader
	{
		let (vertex, fragment) = unsafe {
			vulkan::ffi::new_shader(connection, device, vert, frag)
		};

		// TODO: has_data
		Shader {
			vertex, fragment, textures, vertex_buffers, has_data: 0 
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Style {
	pipeline: VkPipeline,
	pub descsetlayout: VkDescriptorSetLayout,
	pipeline_layout: VkPipelineLayout,
}

#[repr(C)]
#[derive(Copy,Clone)]
pub struct Texture {
	mappable_image: VkImage,
	mappable_memory: VkDeviceMemory,
	image: VkImage,
	memory: VkDeviceMemory,
	sampler: VkSampler,
	view: VkImageView,
	w: u32,
	h: u32,
	pitch: u32,
	staged: bool,
}

pub struct Shape {
	vertex_buffers: Vec<VkBuffer>,
	vertex_memory: Vec<VkDeviceMemory>,
	vertice_count: u32,
	num_vbs: u32,
	instance: VwInstance,
}

// TODO
/*impl Shape {
	pub fn create(window: &mut Window, v: &[f32], style: style::Style) -> Shape {
		let size = v.len() as u32;
		let hastx = {
			match style {
				style::Style::Solid(_) => false,
				style::Style::Texture(_, _) => true,
				style::Style::Invisible => {
					panic!("Can't create a Sprite with \
						invisible style.")
				}
			}
		};
		let mut shape = VwShape {
			vertex_buffer_memory: 0,
			vertex_input_buffer: 0,
			vertice_count: size / 8,
		};
		unsafe { vulkan::ffi::new_shape(&mut shape, window.vw, &v[0], size); }
		Shape {
			shape: shape,
			hastx: hastx,
			instances: Vec::new(),
		}
	}

	pub fn animate(window: &mut Window, index: usize, i: usize,
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
			vulkan::ffi::destroy_uniforms(&window.vw, &mut
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

	pub fn add(window: &mut Window, index: usize, tx: *const NativeTexture,
		style: Style)
	{
		let shape = &mut window.sprites[index];
		let mem = VwLinkedInstance {
			instance: unsafe {
				vw_vulkan_uniforms(&window.vw, style, tx,
					if shape.hastx { 1 } else { 0 })
			},
			enabled: true,
		};
		vulkan::copy_memory(window.vw.device,
			mem.instance.uniform_memory, &mem.matrix);
		shape.instances.push(mem);
	}

	pub fn draw(window: &mut Window, index: usize) {
		let shape = &window.sprites[index];
		for i in 0..shape.instances.len() {
			if !window.sprites[index].instances[i].enabled {
				continue;
			}
			unsafe {
				vw_vulkan_draw_shape(&mut window.vw,
					&shape.shape,
					&shape.instances[i].matrix[0],
					shape.instances[i].instance);
			}
			vulkan::cmd_draw(window.vw.command_buffer,
				shape.shape.vertice_count);
		}
	}

	pub fn matrix(window: &mut Window, index: usize, i: usize,
		matrix: [f32; 16])
	{
		window.sprites[index].instances[i].matrix = matrix;
		vulkan::copy_memory(window.vw.device,
			window.sprites[index].instances[i].instance.uniform_memory,
			&window.sprites[index].instances[i].matrix);
	}

	pub fn vertices(window: &Window, index: usize, v: &[f32]) {
		vulkan::copy_memory(window.vw.device,
			window.sprites[index].shape.vertex_buffer_memory, v);
	}
}*/

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VwInstance {
	pub matrix_buffer: VkBuffer,
	pub uniform_memory: VkDeviceMemory,
	pub desc_set: VkDescriptorSet,
	pub desc_pool: VkDescriptorPool,
	pub pipeline: Style,
}

extern {
	fn vw_vulkan_shader(a: *mut Shader, b: *const Vw, c: *const u8, d: u32,
		e: *const u8, f: u32) -> ();
	fn vw_vulkan_pipeline(z: *mut Style, a: *mut Vw, b: *const Shader,
		c: u32);
	fn vw_vulkan_draw_begin(v: *mut Vw, r: f32, g: f32, b: f32) -> ();
// TODO: In Rust
//	fn vw_vulkan_uniforms(a: *const Vw, b: Style,
//		c: *const NativeTexture, d: u8) -> VwInstance;
	fn vw_vulkan_draw_shape(v: *mut Vw, f: VwInstance)
		-> ();
	fn vw_vulkan_draw_update(v: *mut Vw) -> ();
	
	fn vw_vulkan_swapchain_delete(v: *mut Vw) -> ();
}

fn swapchain_resize(connection: &Connection, vw: &mut Vw) -> () {
	unsafe {
		// Link swapchain to vulkan instance.
		vulkan::ffi::create_swapchain(
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
		vulkan::ffi::create_image_view(
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
		let (img, view, mem) = vulkan::ffi::create_depth_buffer(
			connection,
			vw.device,
			vw.gpu,
			vw.command_buffer,
			vw.submit_fence,
			vw.present_queue,
			vw.width,
			vw.height,
		);

		vw.depth_image = img;
		vw.depth_image_view = view;
		vw.depth_image_memory = mem;

		// Link Render Pass to swapchain
		vw.render_pass = vulkan::ffi::create_render_pass(
			connection,
			vw.device,
			&vw.color_format,
		);

		// Link Framebuffers to swapchain
		vulkan::ffi::create_framebuffers(
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
		vulkan::ffi::destroy_swapchain(
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

	let (mappable_image, mappable_memory) = unsafe {
		vulkan::ffi::create_image(
			connection, vw.device, vw.gpu, width, height,
			VkFormat::B8g8r8a8Srgb, VkImageTiling::Linear,
			if staged { VkImageUsage::TransferSrcBit }
				else { VkImageUsage::SampledBit },
			VkImageLayout::Preinitialized,
			0x00000006 /* visible|coherent */
		)
	};

	let layout = unsafe {
		vulkan::ffi::subres_layout(connection, vw.device,
			mappable_image)
	};

	let pitch = layout.row_pitch;

	let (image, memory) = if staged {
		unsafe {
			vulkan::ffi::create_image(
				connection, vw.device, vw.gpu, width, height,
				VkFormat::B8g8r8a8Srgb, VkImageTiling::Optimal,
				VkImageUsage::TransferDstAndUsage,
				VkImageLayout::Undefined, 0)
		}
	} else {
		(mappable_image, mappable_memory)
	};
//
	let sampler = unsafe { vulkan::ffi::new_sampler(connection, vw.device) };

	let view = unsafe {
		vulkan::ffi::create_imgview(connection, vw.device, image,
			VkFormat::B8g8r8a8Srgb, true)
	};
//
	Texture {
		staged, mappable_image, mappable_memory, image, memory, view,
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
			vulkan::ffi::copy_image(connection,
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
			vulkan::ffi::get_gpu(&connection.0, instance, surface)
		};
		let device = unsafe {
			vulkan::ffi::create_device(&connection.0, gpu, pqi)
		};
		let present_queue = unsafe {
			vulkan::ffi::create_queue(&connection.0, device, pqi)
		};
		let command_buffer = unsafe {
			vulkan::ffi::create_command_buffer(&connection.0,
				device, pqi)
		}.0;
		let color_format = unsafe {
			vulkan::ffi::get_color_format(&connection.0,
				gpu, surface)
		};
		let image_count = unsafe {
			vulkan::ffi::get_buffering(&connection.0, gpu, surface)
		};
		let present_mode = unsafe {
			vulkan::ffi::get_present_mode(&connection.0, gpu,
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
	let xscale = scale * ratio;

	::Transform([
		xscale,	0.,	0.,	0.,
		0.,	scale,	0.,	0.,
		0.,	0.,	1.,	1.,
		0.,	0.,	0., 	1.,
	])
}

pub struct Renderer {
	vw: Vw,
	connection: Connection,
	shapes: Vec<Shape>,
	style_solid: Style,
	style_texture: Style,
	style_gradient: Style,
	style_faded: Style,
	style_tinted: Style,
	style_complex: Style,
	projection: ::Transform,
}

impl Renderer {
	pub fn new(window_name: &str, window_connection: WindowConnection)
		-> Renderer
	{
//		let native = NativeRenderer::new(window_name,
//			window_connection.clone());

		let (mut connection, mut vw) = Vw::new(window_name, window_connection);
		let shapes = Vec::new();
		let shadev = vec![
			Shader::new(&connection, vw.device,
				include_bytes!("../native_renderer/vulkan/res/solid-vert.spv"),
				include_bytes!("../native_renderer/vulkan/res/solid-frag.spv"), 0, 1),
			Shader::new(&connection, vw.device,
				include_bytes!("../native_renderer/vulkan/res/texture-vert.spv"),
				include_bytes!("../native_renderer/vulkan/res/texture-frag.spv"), 1, 2),
			Shader::new(&connection, vw.device,
				include_bytes!("../native_renderer/vulkan/res/gradient-vert.spv"),
				include_bytes!("../native_renderer/vulkan/res/gradient-frag.spv"), 0, 2),
			Shader::new(&connection, vw.device,
				include_bytes!("../native_renderer/vulkan/res/faded-vert.spv"),
				include_bytes!("../native_renderer/vulkan/res/faded-frag.spv"), 1, 2),
			Shader::new(&connection, vw.device,
				include_bytes!("../native_renderer/vulkan/res/tinted-vert.spv"),
				include_bytes!("../native_renderer/vulkan/res/tinted-frag.spv"), 1, 2),
			Shader::new(&connection, vw.device,
				include_bytes!("../native_renderer/vulkan/res/complex-vert.spv"),
				include_bytes!("../native_renderer/vulkan/res/complex-frag.spv"), 1, 3),
		];

		let mut styles = Vec::with_capacity(shadev.len());
		for i in &shadev {
			styles.push(
				unsafe {
					vulkan::ffi::new_pipeline(&connection,
						vw.device, vw.render_pass,
						vw.width, vw.height, *i)
				}
			);
		}

		let projection = projection(vw.height as f32 / vw.width as f32,
			90.0);

		Renderer {
			vw, connection, shapes, projection,
			style_solid: styles[0], style_texture: styles[1],
			style_gradient: styles[2], style_faded: styles[3],
			style_tinted: styles[4], style_complex: styles[5],
		}
	}

	pub fn update(&mut self) {
//		let color = self.color;
//		let presenting_finish_sem;
//		let rendering_finish_sem;

		unsafe {
			self.vw.presenting_complete_sem = vulkan::ffi::new_semaphore(
				&self.connection,
				self.vw.device,
			);

			self.vw.next_image_index = vulkan::ffi::get_next_image(
				&self.connection,
				self.vw.device,
				&mut self.vw.presenting_complete_sem,
				self.vw.swapchain,
			);

			self.vw.rendering_complete_sem = vulkan::ffi::new_semaphore(
				&self.connection,
				self.vw.device,
			);

			vw_vulkan_draw_begin(&mut self.vw, 0.0, 0.0, 1.0);
		}

		for shape in &self.shapes {
			unsafe {
				vulkan::ffi::cmd_bind_vb(&self.connection,
					self.vw.command_buffer,
					shape.vertex_buffers.as_slice());

				vulkan::ffi::cmd_bind_pipeline(&self.connection,
					self.vw.command_buffer,
					shape.instance.pipeline.pipeline);

				vulkan::ffi::cmd_bind_descsets(&self.connection,
					self.vw.command_buffer,
					shape.instance.pipeline.pipeline_layout,
					shape.instance.desc_set);
			}

			vulkan::cmd_draw(&self.connection,
				self.vw.command_buffer,
				shape.vertice_count);
		}

		unsafe {
			vw_vulkan_draw_update(&mut self.vw);
		}
	}

	pub fn resize(&mut self, size: (u32, u32)) {
		self.vw.width = size.0;
		self.vw.height = size.1;

		swapchain_delete(&self.connection, &mut self.vw);
		swapchain_resize(&self.connection, &mut self.vw);

		self.shapes.clear();
		self.projection = projection(size.1 as f32/size.0 as f32, 90.0);
	}

	pub fn texture(&mut self, width: u32, height: u32, rgba: &[u32])
		-> Texture
	{
		let mut texture = new_texture(&self.connection, &mut self.vw,
			width, height);

		set_texture(&self.connection, &mut self.vw, &mut texture, rgba);

		texture
	}

	pub fn textured(&mut self, vertices: &[f32], texture: Texture,
		texcoords: &[f32]) -> usize
	{
		let size = vertices.len() as u32;
		let num_vbs = 2;

		let (vertex_input_buffer, vertex_buffer_memory) = unsafe {
			vulkan::ffi::new_shape(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				vertices,
			)
		};

		let (texc_input_buffer, texc_buffer_memory) = unsafe {
			vulkan::ffi::new_shape(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				texcoords,
			)
		};

		let a = self.shapes.len();

		let uniform = TransformUniform {
			mat4: self.projection.0,
		};

		// Add an instance
		let instance = unsafe {
			vulkan::ffi::vw_instance_new(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				self.style_texture,
				mem::size_of::<TransformUniform>(),
				texture.view,
				texture.sampler,
				1, // 1 texure
			)
		};

		vulkan::copy_memory(&self.connection, self.vw.device,
			instance.uniform_memory, &uniform,
			mem::size_of::<TransformUniform>());

		println!("PUSH SHAPE");

		let vertex_buffers = [vertex_input_buffer, texc_input_buffer]
			.to_vec();
		let vertex_memory = [vertex_buffer_memory, texc_buffer_memory]
			.to_vec();

		self.shapes.push(Shape {
			num_vbs,
			instance,
			vertex_buffers,
			vertex_memory,
			vertice_count: size / 4,
		});

		a
	}

	pub fn solid(&mut self, vertices: &[f32], color: [f32; 4]) -> usize {
		let size = vertices.len() as u32;
		let num_vbs = 1;

		let (vertex_input_buffer, vertex_buffer_memory) = unsafe {
			vulkan::ffi::new_shape(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				vertices,
			)
		};

		let a = self.shapes.len();

		let matrix = TransformAndColorUniform {
			vec4: color,
			mat4: self.projection.0,
		};

		// Add an instance
		let instance = unsafe {
			vulkan::ffi::vw_instance_new(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				self.style_solid,
				mem::size_of::<TransformAndColorUniform>(),
				unsafe { mem::zeroed() },
				unsafe { mem::zeroed() },
				0, // no texure
			)
		};

		vulkan::copy_memory(&self.connection, self.vw.device,
			instance.uniform_memory, &matrix,
			mem::size_of::<TransformAndColorUniform>());

		let vertex_buffers = [vertex_input_buffer].to_vec();
		let vertex_memory = [vertex_buffer_memory].to_vec();

		self.shapes.push(Shape {
			num_vbs,
			instance,
			vertex_buffers,
			vertex_memory,
			vertice_count: size / 4,
		});

		a
	}

	pub fn gradient(&mut self, vertices: &[f32], colors: &[f32]) -> usize {
		println!("GRADIENT: {:?}", colors);

		let size = vertices.len() as u32;
		let num_vbs = 2;

		let (vertex_input_buffer, vertex_buffer_memory) = unsafe {
			vulkan::ffi::new_shape(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				vertices,
			)
		};

		let (texc_input_buffer, texc_buffer_memory) = unsafe {
			vulkan::ffi::new_shape(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				colors,
			)
		};

		let a = self.shapes.len();

		let uniform = TransformUniform {
			mat4: self.projection.0,
		};

		// Add an instance
		let instance = unsafe {
			vulkan::ffi::vw_instance_new(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				self.style_gradient,
				mem::size_of::<TransformUniform>(),
				unsafe { mem::zeroed() },
				unsafe { mem::zeroed() },
				0, // no texure
			)
		};

		vulkan::copy_memory(&self.connection, self.vw.device,
			instance.uniform_memory, &uniform,
			mem::size_of::<TransformUniform>());

		println!("PUSH GRADIENT");

		let vertex_buffers = [vertex_input_buffer, texc_input_buffer]
			.to_vec();
		let vertex_memory = [vertex_buffer_memory, texc_buffer_memory]
			.to_vec();

		self.shapes.push(Shape {
			num_vbs,
			instance,
			vertex_buffers,
			vertex_memory,
			vertice_count: size / 4,
		});

		a
	}

	pub fn faded(&mut self, vertices: &[f32], texture: Texture,
		texcoords: &[f32], fade_factor: f32) -> usize
	{
		let size = vertices.len() as u32;
		let num_vbs = 2;

		let (vertex_input_buffer, vertex_buffer_memory) = unsafe {
			vulkan::ffi::new_shape(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				vertices,
			)
		};

		let (texc_input_buffer, texc_buffer_memory) = unsafe {
			vulkan::ffi::new_shape(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				texcoords,
			)
		};

		let a = self.shapes.len();

		let uniform = TransformAndFadeUniform {
			mat4: self.projection.0,
			fade: fade_factor,
		};

		// Add an instance
		let instance = unsafe {
			vulkan::ffi::vw_instance_new(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				self.style_faded,
				mem::size_of::<TransformAndFadeUniform>(),
				texture.view,
				texture.sampler,
				1, // 1 texure
			)
		};

		vulkan::copy_memory(&self.connection, self.vw.device,
			instance.uniform_memory, &uniform,
			mem::size_of::<TransformAndFadeUniform>());

		println!("PUSH SHAPE");

		let vertex_buffers = [vertex_input_buffer, texc_input_buffer]
			.to_vec();
		let vertex_memory = [vertex_buffer_memory, texc_buffer_memory]
			.to_vec();

		self.shapes.push(Shape {
			num_vbs,
			instance,
			vertex_buffers,
			vertex_memory,
			vertice_count: size / 4,
		});

		a
	}

	pub fn tinted(&mut self, vertices: &[f32], texture: Texture,
		texcoords: &[f32], color: [f32; 4]) -> usize
	{
		let size = vertices.len() as u32;
		let num_vbs = 2;

		let (vertex_input_buffer, vertex_buffer_memory) = unsafe {
			vulkan::ffi::new_shape(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				vertices,
			)
		};

		let (texc_input_buffer, texc_buffer_memory) = unsafe {
			vulkan::ffi::new_shape(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				texcoords,
			)
		};

		let a = self.shapes.len();

		let uniform = TransformAndColorUniform {
			mat4: self.projection.0,
			vec4: color,
		};

		// Add an instance
		let instance = unsafe {
			vulkan::ffi::vw_instance_new(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				self.style_tinted,
				mem::size_of::<TransformAndColorUniform>(),
				texture.view,
				texture.sampler,
				1, // 1 texure
			)
		};

		vulkan::copy_memory(&self.connection, self.vw.device,
			instance.uniform_memory, &uniform,
			mem::size_of::<TransformAndColorUniform>());

		println!("PUSH SHAPE");

		let vertex_buffers = [vertex_input_buffer, texc_input_buffer]
			.to_vec();
		let vertex_memory = [vertex_buffer_memory, texc_buffer_memory]
			.to_vec();

		self.shapes.push(Shape {
			num_vbs,
			instance,
			vertex_buffers,
			vertex_memory,
			vertice_count: size / 4,
		});

		a
	}

	pub fn complex(&mut self, vertices: &[f32], texture: Texture,
		texcoords: &[f32], colors: &[f32]) -> usize
	{
		let size = vertices.len() as u32;
		let num_vbs = 3;

		let (vertex_input_buffer, vertex_buffer_memory) = unsafe {
			vulkan::ffi::new_shape(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				vertices,
			)
		};

		let (texc_input_buffer, texc_buffer_memory) = unsafe {
			vulkan::ffi::new_shape(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				texcoords,
			)
		};

		let (color_input_buffer, color_buffer_memory) = unsafe {
			vulkan::ffi::new_shape(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				colors,
			)
		};

		let a = self.shapes.len();

		let uniform = TransformUniform {
			mat4: self.projection.0,
		};

		// Add an instance
		let instance = unsafe {
			vulkan::ffi::vw_instance_new(
				&self.connection,
				self.vw.device,
				self.vw.gpu,
				self.style_complex,
				mem::size_of::<TransformUniform>(),
				texture.view,
				texture.sampler,
				1, // 1 texure
			)
		};

		vulkan::copy_memory(&self.connection, self.vw.device,
			instance.uniform_memory, &uniform,
			mem::size_of::<TransformUniform>());

		println!("PUSH SHAPE");

		let vertex_buffers = [vertex_input_buffer, texc_input_buffer,
			color_input_buffer].to_vec();
		let vertex_memory = [vertex_buffer_memory, texc_buffer_memory,
			color_buffer_memory].to_vec();

		self.shapes.push(Shape {
			num_vbs,
			instance,
			vertex_buffers,
			vertex_memory,
			vertice_count: size / 4,
		});

		a
	}

	pub fn get_projection(&self) -> ::Transform {
		::Transform(self.projection.0)
	}

	pub fn transform(&self, shape: usize, transform: &::Transform) -> usize {
		let uniform = TransformUniform {
			mat4: (::Transform(transform.0) * self.get_projection()).0,
		};

		vulkan::copy_memory(&self.connection, self.vw.device,
			self.shapes[shape].instance.uniform_memory, &uniform,
			mem::size_of::<TransformUniform>());

		shape
	}
}

impl Drop for Renderer {
	fn drop(&mut self) -> () {
		swapchain_delete(&self.connection, &mut self.vw);

		unsafe {
			ffi::vulkan::ffi::destroy_surface(&self.connection,
				self.vw.surface);
			ffi::vulkan::ffi::destroy_instance(&self.connection);
		}
	}
}
