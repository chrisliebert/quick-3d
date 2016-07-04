// Copyright(C) 2016 Chris Liebert
#![crate_type = "lib"]
#![crate_name = "quick_3d"]
#[macro_use]
extern crate glium;

pub mod common;

pub mod loader;
#[macro_use]
pub mod renderer;

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
