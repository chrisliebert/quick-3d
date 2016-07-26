// Copyright(C) 2016 Chris Liebert
extern crate glium;
extern crate nalgebra;
extern crate quick3d;

use glium::glutin;
use glium::glutin::Event;
use glium::glutin::ElementState;
use glium::glutin::VirtualKeyCode;
use glium::DisplayBuild;

use quick3d::common::Scene;
use quick3d::loader::DBLoader;
use quick3d::renderer;

fn main() {
    let screen_width = 400;
    let screen_height = 300;
    let db_file: &str = "test.db";

    let dbloader: DBLoader = DBLoader::new(db_file);
    let shader_dbloader: DBLoader = DBLoader::new("shaders.db");
    let scene: Scene = dbloader.load_scene();

    let display = glutin::WindowBuilder::new()
        //.resizable()
        //.with_vsync()
        .with_gl_debug_flag(true)
        .with_visibility(false) // Window is shown when scene finishes loading.
        .with_dimensions(screen_width, screen_height)
        .build_glium()
        .unwrap();

    let renderer = renderer::Renderer::new(&display, scene);

    let shader_program = renderer.create_shader_program("default", &shader_dbloader, &display);

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
        renderer.render(&display, &shader_program);

        // Check for close events
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
