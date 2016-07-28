// Copyright(C) 2016 Chris Liebert
extern crate glium;
extern crate image;
extern crate nalgebra;

use std::collections::HashMap;

use common;
use common::{Mesh, Scene, Shader, Vertex8f32};
use loader::DBLoader;

use glium::backend::glutin_backend::GlutinFacade;
use glium::{Program, Surface, Version};

use self::nalgebra::{Matrix4, PerspectiveMatrix3};

implement_vertex!(Vertex8f32, position, normal, texcoord);

pub struct Renderer {
	pub index_buffer: glium::index::NoIndices,
	pub modelview_matrix_array: [[f32; 4]; 4],
	pub projection_matrix: Matrix4<f32>,
	pub scene: Scene,
	pub textures: HashMap<String, glium::texture::CompressedSrgbTexture2d>,
	pub vertex_buffers: Vec<glium::vertex::VertexBuffer<common::Vertex8f32>>,
}

impl Renderer {
	pub fn new(display: &GlutinFacade, scene: Scene) -> Renderer {
	
		let mut vertex_buffers: Vec<glium::vertex::VertexBuffer<common::Vertex8f32>> =
			Vec::with_capacity(scene.meshes.len());
	
		for i in 0..scene.meshes.len() {
			vertex_buffers.push(glium::vertex::VertexBuffer::new(display, &scene.meshes[i].vertices).unwrap());
		}
		
		let index_buffer: glium::index::NoIndices =
			glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
	
		let mut textures: HashMap<String, glium::texture::CompressedSrgbTexture2d> = HashMap::new();
	
		for i in 0..scene.images.len() {
			//TODO: determine image format by name extension if nessisary
			//let image = image::load_from_memory_with_format(&scene.images[i].image, image::PNG).unwrap().to_rgba();
			let image = image::load_from_memory(&scene.images[i].image).unwrap().to_rgba();
			
			let image_dimensions = image.dimensions();
			let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(),
			                                                               image_dimensions);
			let opengl_texture: glium::texture::CompressedSrgbTexture2d =
				glium::texture::CompressedSrgbTexture2d::new(display, image).unwrap();
			textures.insert(scene.images[i].name.clone(), opengl_texture);		
		}
		
		// Get the screen width and height in pixels
		let screen_width: u32;
		let screen_height: u32;
		match display.get_window() {
			Some(window) =>  {
				let pixel_dimensions: (u32,u32) = window.get_inner_size_pixels().unwrap();
				screen_width = pixel_dimensions.0;
				screen_height = pixel_dimensions.1;
			},
			None => {
				panic!("Error retrieving window when querying display size.");
			}
		}
	
		// Set up camera
		let projection_matrix = PerspectiveMatrix3::new(screen_width as f32 / screen_height as f32,
		                                          45.0,
		                                          0.1,
		                                          1000.0);
		let projection_matrix_array: Matrix4<f32> = projection_matrix.to_matrix();
	
	
		let modelview_matrix_array = [[1.0f32, -0.0f32, -0.0f32, 0.0f32],
		                          [0.0f32, 1.0f32, -0.0f32, 0.0f32],
		                          [0.0f32, 0.0f32, 1.0f32, 0.0f32],
		                          [-5.0f32, -3.0f32, -12.0f32, 1.0f32]];
		Renderer { 
			index_buffer: index_buffer,
			modelview_matrix_array: modelview_matrix_array,
			projection_matrix: projection_matrix_array,
			scene: scene,
			textures: textures,
			vertex_buffers: vertex_buffers,
		}
	}
	
	pub fn get_mesh(&self, name: &str) -> Option<&Mesh> {
		for i in 0..self.vertex_buffers.len() as usize {
			if self.scene.meshes[i].name.eq(name) {
				return Some(&self.scene.meshes[i]);
			}
		}
		return None;
	}
	
	#[allow(unused_assignments)]
	pub fn create_shader_program(&self, shader_name: &str, dbloader: &DBLoader, display: &GlutinFacade) -> glium::program::Program {
		let supported_glsl_version: Version = display.get_supported_glsl_version();
		let api: glium::Api = supported_glsl_version.0;
		let major_version : u8 = supported_glsl_version.1;
		let minor_version : u8 = supported_glsl_version.2;
		let mut glsl_version_string: String = major_version.to_string();
		glsl_version_string.push_str(&minor_version.to_string());
		glsl_version_string.push('0');
		let mut use_gles: bool = false;// Use OpenGLES instead of OpenGL (for mobile devices)
		
		match api {
				glium::Api::Gl => {
					use_gles = false;
				},
				glium::Api::GlEs => {
					use_gles = true;
					glsl_version_string.push_str(" es");
				},
		};
		
		let glsl_version_number: u32 = match glsl_version_string.parse() {
			Ok(s) => s,
			Err(e) =>  {
				// This is not likely to happen
				panic!("Unable to parse supported glsl version string: {}", e);
			},
		};
		
		println!("Using glsl version {}", &glsl_version_string);
	
		let shader: Shader = dbloader.load_shader(shader_name, &glsl_version_string);
	
		let program: Program;
		
		if use_gles {
			program = program!(display,
				glsl_version_number es => {
					vertex: &shader.vertex_source,
					fragment: &shader.fragment_source,
				},
			)
			.unwrap();
		} else {
			program = program!(display,
				glsl_version_number => {
					vertex: &shader.vertex_source,
					fragment: &shader.fragment_source,
				},
			)
			.unwrap();
		}
			
		// Shader compilation no longer required
		display.release_shader_compiler();
		return program;
	}
	
	pub fn render(&self, display: &GlutinFacade, program: &glium::program::Program) {
		let mut target = display.draw();
		// TODO: generate this texture instead of loading from sqlite
		let default_blank_texture = &self.textures["DEFAULT_BLANK_TEXTURE.png"];
		target.clear_color(0.0, 0.0, 0.0, 1.0);
		for i in 0..self.vertex_buffers.len() as usize {
			let material_index: usize = self.scene.meshes[i].material_index.clone();
			let diffuse = self.scene.materials[material_index].diffuse.clone();
			let diffuse_texname: String =
				self.scene.materials[material_index].diffuse_texname.clone();
			let opengl_texture: &glium::texture::CompressedSrgbTexture2d;
			if diffuse_texname.len() == 0 {
				opengl_texture = &default_blank_texture;
			} else {
				match self.textures.get(&diffuse_texname) {
					Some(t) => {
						opengl_texture = &t;
					},
					None => {
						println!("Unable to load {}, using blank texture instead.", diffuse_texname);
						opengl_texture = &default_blank_texture;
					},
				}
			}
			let uniforms: glium::uniforms::UniformsStorage<_, _> = uniform! {
				projection: *self.projection_matrix.as_ref(),
				modelview: self.modelview_matrix_array,
				light_position_worldspace: [2.0, 10.0, 1.0f32],
				diffuse: diffuse,
				diffuse_texture: opengl_texture,
				model: *self.scene.meshes[i].matrix.borrow(),
			};
			
			target.draw(&self.vertex_buffers[i],
				&self.index_buffer,
				program,
				&uniforms,
				&Default::default())
				.unwrap();
		}
		target.finish().unwrap();
	}
}
