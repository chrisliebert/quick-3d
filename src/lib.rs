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

    use glium::glutin::WindowBuilder;
    use glium::backend::glutin_backend::GlutinFacade;
    use glium::DisplayBuild;
    use loader;
    use common::{Scene, Vertex8f32};

    fn create_test_display(scene: &Scene) -> GlutinFacade {
        let display = glutin::WindowBuilder::new()
            .with_visibility(false)
            //.with_dimensions(screen_width, screen_height)
            .build_glium()
            .unwrap();
        return display;
    }

    fn load_test_scene() -> Scene {
        loader::load_db("test.db")
    }

    #[test]
    fn loader_load_db_not_empty() {
        println!("Running load_db test");

        let scene: Scene = load_test_scene();
        println!("loaded {} vertices", scene.vertices.len());
        assert!(scene.vertices.len() > 0);
        assert!(scene.scene_nodes.len() > 0);
        assert!(scene.materials.len() > 0);
        assert!(scene.shaders.len() > 0);
    }
}
