// Copyright(C) 2016 Chris Liebert
#[macro_use]
extern crate glium;
extern crate quick_3d;
extern crate nalgebra;

use glium::glutin;
use glium::glutin::Event;
use glium::glutin::ElementState;
use glium::glutin::VirtualKeyCode;
use glium::Surface;
use glium::DisplayBuild;
use quick_3d::loader;

use nalgebra::{Matrix4, PerspectiveMatrix3};

fn main() {
    let screen_width = 800;
    let screen_height = 600;

    let db_file: &str = "test.db";

    let scene = loader::load_db(db_file);
    println!("Loaded {} vertices", scene.vertices.len());

    let display = glutin::WindowBuilder::new()
        //.resizable()
        //.with_vsync()
        //with_gl_debug_flag(true)
        .with_visibility(false)
        .with_dimensions(screen_width, screen_height)
        .build_glium()
        .unwrap();


    let vertex_buffer = glium::vertex::VertexBuffer::new(&display, &scene.vertices).unwrap();
    let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    if scene.shaders.len() == 0 {
        println!("No shaders loaded");
        return;
    }

    // Load the first shader in the database by default
    // TODO: load from config settings table?
    let shader_index = 0;

    // println!("Using glsl version {}", glsl_version);

    // TODO: wrap program! macro in another macros

    let program = program!(&display,
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

    // Set up camera
    let perspective = PerspectiveMatrix3::new(screen_width as f32 / screen_height as f32,
                                              45.0,
                                              0.1,
                                              1000.0);
    let perspective_matrix: Matrix4<f32> = perspective.to_matrix();


    let modelview_matrix_array = [[1.0f32, -0.0f32, -0.0f32, 0.0f32],
                                  [0.0f32, 1.0f32, -0.0f32, 0.0f32],
                                  [0.0f32, 0.0f32, 1.0f32, 0.0f32],
                                  [-5.0f32, -3.0f32, -12.0f32, 1.0f32]];

    let uniforms = uniform! {
	        projection: *perspective_matrix.as_ref(),
	        modelview: modelview_matrix_array,
	        light_position_worldspace: [2.0, 10.0, 1.0f32],
	        diffuse: [0.5f32, 0.5, 0.5]
	    };

    let mut running = true;

    // Show the window once the data is loaded
    match display.get_window() {
        Some(x) => x.show(),
        None => {
            running = false;
            println!("Error retrieving window");
        }
    }

    while running {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vertex_buffer,
                  &index_buffer,
                  &program,
                  &uniforms,
                  &Default::default())
            .unwrap();
        target.finish().unwrap();

        for event in display.poll_events() {
            match event {
                Event::Closed => running = false,
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => {
                    running = false
                }
                _ => (),
            }
        }
    }

}
