// Copyright(C) 2016 Chris Liebert
#![crate_type = "dylib"]
#![crate_name = "quick3d"]
#[macro_use]
extern crate glium;

pub mod common;

pub mod loader;
#[macro_use]
pub mod renderer;

#[no_mangle]
pub extern fn hello() {
    extern crate glium;
    extern crate nalgebra;

    use glium::glutin;
    use glium::glutin::Event;
    use glium::glutin::ElementState;
    use glium::glutin::VirtualKeyCode;
    use glium::DisplayBuild;

    use common::Scene;
    use loader::DBLoader;
    use renderer;
    let screen_width = 400;
    let screen_height = 300;
    let db_file: &str = "test.db";

    let dbloader: DBLoader = DBLoader::new(db_file);
    let scene: Scene = dbloader.load_scene();

    let display = glutin::WindowBuilder::new()
        //.resizable()
        //.with_vsync()
        //with_gl_debug_flag(true)
        .with_visibility(false) // Window is shown when scene finishes loading.
        .with_dimensions(screen_width, screen_height)
        .build_glium()
        .unwrap();

    let renderer = renderer::Renderer::new(&display, scene);
    let shader_program = renderer.create_shader_program("default", &dbloader, &display);

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


#[cfg(test)]
mod tests {
    use glium::glutin;
    use glium::backend::glutin_backend::GlutinFacade;
    use glium::DisplayBuild;
    use loader;
    use common::Scene;
    use renderer;

    fn create_test_display() -> GlutinFacade {
        let display = glutin::WindowBuilder::new()
            .with_visibility(false)
            .with_gl_debug_flag(true)
            .build_glium()
            .unwrap();
        return display;
    }

    fn load_test_scene() -> Scene {
        let dbloader: loader::DBLoader = loader::DBLoader::new("test.db");
        return dbloader.load_scene();
    }

    #[test]
    fn loader_load_db_not_empty() {
        println!("Running loader_load_db_not_empty test");
        let scene: Scene = load_test_scene();
        assert!(scene.meshes.len() > 0);
        assert!(scene.materials.len() > 0);
        // assert!(scene.shaders.len() > 0);
    }

    #[test]
    fn load_and_render_db() {
        println!("Running load_and_render_db test");
        let display = create_test_display();
        let scene: Scene = load_test_scene();
        assert!(scene.meshes.len() > 0);
        assert!(scene.materials.len() > 0);
        // assert!(scene.shaders.len() > 0);
        // let renderer: renderer::Renderer = renderer::Renderer::new(&display, scene);
        // renderer.render(&display);
    }
}
