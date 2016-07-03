// Copyright(C) 2016 Chris Liebert
extern crate glium;
extern crate image;
extern crate nalgebra;

use std::collections::HashMap;

use common;
use glium::backend::glutin_backend::GlutinFacade;
use glium::Program;
use glium::Surface;
use self::nalgebra::{Matrix4, PerspectiveMatrix3};

use common::{Scene, Vertex8f32};
implement_vertex!(Vertex8f32, position, normal, texcoord);

pub struct Renderer {
	pub index_buffer: glium::index::NoIndices,
	pub modelview_matrix_array: [[f32; 4]; 4],
	pub program: Program,
	pub projection_matrix: Matrix4<f32>,
    pub scene: Scene,
    pub textures: HashMap<String, glium::texture::CompressedSrgbTexture2d>,
    pub vertex_buffers: Vec<glium::vertex::VertexBuffer<common::Vertex8f32>>,
}

impl Renderer {
    pub fn new(display: &GlutinFacade, scene: Scene) -> Renderer {

    let mut vertex_buffers: Vec<glium::vertex::VertexBuffer<common::Vertex8f32>> = Vec::with_capacity(scene.meshes.len());

    for i in 0..scene.meshes.len() {
        vertex_buffers.push(glium::vertex::VertexBuffer::new(display, &scene.meshes[i].vertices).unwrap());
    }
	
    let index_buffer: glium::index::NoIndices =
        glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    if scene.shaders.len() == 0 {
        panic!("No shaders loaded");
    }

	let mut textures: HashMap<String, glium::texture::CompressedSrgbTexture2d> = HashMap::new();

	for i in 0..scene.images.len() {
		//TODO: determine image format by name extension if nessisary
		//let image = image::load_from_memory_with_format(&scene.images[i].image, image::PNG).unwrap().to_rgba();
		let image = image::load_from_memory(&scene.images[i].image).unwrap().to_rgba();

	    let image_dimensions = image.dimensions();
	    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(),
	                                                                   image_dimensions);
	    let opengl_texture: glium::texture::CompressedSrgbTexture2d = glium::texture::CompressedSrgbTexture2d::new(display, image).unwrap();
		textures.insert(scene.images[i].name.clone(), opengl_texture);		
	}

    // Load the first shader in the database by default
    // TODO: load from config settings table?
    let shader_index = 0;

    // println!("Using glsl version {}", glsl_version);

    let program: Program = program!(display,
    	/*
			110 => {
				vertex: &scene.shaders[shader_index].vertex_source_110,
				fragment: &scene.shaders[shader_index].fragment_source_110,
			},
			120 => {
				vertex: &scene.shaders[shader_index].vertex_source_120,
				fragment: &scene.shaders[shader_index].fragment_source_120,
			},
			130 => {
				vertex: &scene.shaders[shader_index].vertex_source_130,
				fragment: &scene.shaders[shader_index].fragment_source_130,
			},
			*/
			140 => {
				vertex: &scene.shaders[shader_index].vertex_source_140,
				fragment: &scene.shaders[shader_index].fragment_source_140,
			},
			/*
			150 => {
				vertex: &scene.shaders[shader_index].vertex_source_150,
				fragment: &scene.shaders[shader_index].fragment_source_150,
			},
			300 => {
				vertex: &scene.shaders[shader_index].vertex_source_300,
				fragment: &scene.shaders[shader_index].fragment_source_300,
			},
			330 => {
				vertex: &scene.shaders[shader_index].vertex_source_330,
				fragment: &scene.shaders[shader_index].fragment_source_330,
			},
			400 => {
				vertex: &scene.shaders[shader_index].vertex_source_400,
				fragment: &scene.shaders[shader_index].fragment_source_400,
			},
			410 => {
				vertex: &scene.shaders[shader_index].vertex_source_410,
				fragment: &scene.shaders[shader_index].fragment_source_410,
			},
			420 => {
				vertex: &scene.shaders[shader_index].vertex_source_420,
				fragment: &scene.shaders[shader_index].fragment_source_420,
			},
			430 => {
				vertex: &scene.shaders[shader_index].vertex_source_430,
				fragment: &scene.shaders[shader_index].fragment_source_430,
			},
			100 es => {
				vertex: &scene.shaders[shader_index].vertex_source_100es,
				fragment: &scene.shaders[shader_index].fragment_source_100es,
			},
			300 es => {
				vertex: &scene.shaders[shader_index].vertex_source_300es,
				fragment: &scene.shaders[shader_index].fragment_source_300es,
			}, */
		)
        .unwrap();

    // Shader compilation no longer required
    display.release_shader_compiler();
    
    //TODO get width and height from display
    let screen_width = 400;
    let screen_height = 300;

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
        	program: program,
        	projection_matrix: projection_matrix_array,
        	scene: scene,
        	textures: textures,
        	vertex_buffers: vertex_buffers,
        }
    }

    pub fn render(&self, display: &GlutinFacade) {
		 let mut target = display.draw();
	 	// TODO: generate this texture instead of loading from sqlite
		let default_blank_texture = &self.textures["DEFAULT_BLANK_TEXTURE.png"];
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        for i in 0..self.vertex_buffers.len() {
            let material_id: i32 = self.scene.meshes[i as usize].material_id.clone();
            let diffuse = self.scene.materials[material_id as usize].diffuse.clone();
            let diffuse_texname: String =
                self.scene.materials[material_id as usize].diffuse_texname.clone();
            let opengl_texture: &glium::texture::CompressedSrgbTexture2d;
            if diffuse_texname.len() == 0 {
                opengl_texture = &default_blank_texture;
            } else {
                opengl_texture = &self.textures[&diffuse_texname];
            }
            let uniforms: glium::uniforms::UniformsStorage<_, _> = uniform! {
		        projection: *self.projection_matrix.as_ref(),
		        modelview: self.modelview_matrix_array,
		        light_position_worldspace: [2.0, 10.0, 1.0f32],
		        diffuse: diffuse,
		        diffuse_texture: opengl_texture,
		    };

            target.draw(&self.vertex_buffers[i],
                      &self.index_buffer,
                      &self.program,
                      &uniforms,
                      &Default::default())
                .unwrap();
        }
        target.finish().unwrap();
    }
}
