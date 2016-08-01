// Copyright(C) 2016 Chris Liebert
extern crate glium;
extern crate nalgebra;
extern crate quick3d;

use glium::glutin;
use glium::glutin::{ElementState,Event,MouseButton,VirtualKeyCode};
use glium::DisplayBuild;

use nalgebra::Matrix4;
use std::io::Error;

use quick3d::common::{Camera, Mesh, Scene};
use quick3d::loader::DBLoader;
use quick3d::renderer;

fn main() {
    let screen_width = 1200;
    let screen_height = 800;
    let db_file: &str = "test.db";

    let dbloader: DBLoader = DBLoader::new(db_file);
    let shader_dbloader: DBLoader = DBLoader::new("shaders.db");
    let scene: Scene = dbloader.load_scene();

    let display = glutin::WindowBuilder::new()
        //.resizable()
        //.with_vsync()
        .with_depth_buffer(24)
        .with_title("Rust Window")
        .with_gl_debug_flag(true)
        .with_visibility(false) // Window is shown when scene finishes loading.
        .with_dimensions(screen_width, screen_height)
        .build_glium()
        .unwrap();

    let camera: Camera = Camera::new(screen_width as f32, screen_height as f32);
    let renderer = renderer::Renderer::new(&display, scene);

    let shader_program = renderer.create_shader_program("default", &shader_dbloader, &display);

    let mut running: bool = true;

    // Show the window once the data is loaded
    match display.get_window() {
        Some(x) => x.show(),
        None => {
            running = false;
            panic!("Error retrieving window");
        }
    }


    // The torus will be movable in the scene if it is found
    let torus: Result<&Mesh, Error> = renderer.get_mesh("Torus");
    
    let mut torus_x = 0.0f32;
    let mut torus_y = 0.0f32;
    let mut torus_vertical_speed = 0.0f32;
    let mut torus_horizontal_speed = 0.0f32;
    let mut left_button_pressed = false;
    
    let mut mouse_last_x: i32 = 0;
    let mut mouse_last_y: i32 = 0;
    let mut mouse_dx: i32 = 0;
    let mut mouse_dy: i32 = 0;
    
    let mut camera_forward_speed = 0.0f32;
    let mut camera_left_speed = 0.0f32;
    
    let mut w_state = false;
    let mut camera_forward_speed = 0.0;
    
    while running {
    	camera.update();
        renderer.render(&display, &shader_program, &camera);

        // Check for close events
        for event in display.poll_events() {
            match event {
                Event::Closed => running = false,
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => {
                    running = false
                }
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::W)) => {
			        camera_forward_speed = 1.0;
		        }
		        Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::W)) => {
		            camera_forward_speed = 0.0;
		        }
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::A)) => {
                    camera.move_left(1.0);
                }
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::S)) => {
                    camera.move_backward(1.0);
                }
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::D)) => {
                    camera.move_right(1.0);
                }
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::I)) => {
                    torus_vertical_speed = 0.001f32;
                }
                Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::I)) => {
                    torus_vertical_speed = 0.0f32;
                }
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::J)) => {
                    torus_horizontal_speed = -0.001f32;
                }
                Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::J)) => {
                    torus_horizontal_speed = 0.0f32;
                }
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::K)) => {
                    torus_vertical_speed = -0.001f32;
                }
                Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::K)) => {
                    torus_vertical_speed = 0.0f32;
                }
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::L)) => {
                    torus_horizontal_speed = 0.001f32;
                }
                Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::L)) => {
                    torus_horizontal_speed = 0.0f32;
                }
                Event::MouseInput(ElementState::Pressed, MouseButton::Left) => {
                    left_button_pressed = true;
                }
                Event::MouseInput(ElementState::Released, MouseButton::Left) => {
                    left_button_pressed = false;
                }
                Event::MouseMoved(x, y) => {
                	mouse_dx = mouse_last_x - x;
                	mouse_dy = mouse_last_y - y;
                	if left_button_pressed {
                		// Rotate the camera if the left mouse button is pressed
                		camera.aim(mouse_dx as f64, mouse_dy as f64);
                		
                		//todo: store a reference to window
		                let cx: i32 = 600;
		            	let cy: i32 = 400;
                		if x + 10 >= screen_width as i32 || x <= 10 {
		                	display.get_window().unwrap().set_cursor_position(cx, y);
                		} else if y + 10 >= screen_height as i32 || y <= 10 {
		                	display.get_window().unwrap().set_cursor_position(x, cy);
                		}
                	}
                	mouse_last_x = x;
                	mouse_last_y = y;
                }
                
	            _ => (),
            }
        }
        
		camera.move_forward(camera_forward_speed * 0.01);

		// Move the torus (if found) based on changes from keyboard input
        match torus {
        	Ok(torus) => {
		        // Get existing matrix
        		let mut matrix: Matrix4<f32> = *torus.matrix.borrow();
		        torus_x += torus_horizontal_speed;
		        torus_y += torus_vertical_speed;
		        matrix.m14 = torus_x;
		        matrix.m24 = torus_y;
		        // Mutate the matrix
		        *torus.matrix.borrow_mut() = matrix;
        	},
        	Err(_) => {}
        }
    }
}
