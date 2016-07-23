// Copyright(C) 2016 Chris Liebert
#![crate_type = "dylib"]
#![crate_name = "quick3d"]
#![allow(dead_code)]
#[macro_use]
extern crate glium;

pub mod common;

pub mod loader;
#[macro_use]
pub mod renderer;

extern crate nalgebra;
extern crate libc;


use std::ffi::CStr;
use std::os::raw::c_char;

use glium::glutin;
use glium::glutin::Event;
use glium::glutin::ElementState;
use glium::glutin::VirtualKeyCode;
use glium::DisplayBuild;

use glium::backend::glutin_backend::GlutinFacade;

use loader::DBLoader;
use renderer::Renderer;

#[no_mangle]
pub extern "C" fn create_window(screen_width: libc::int32_t,
                                screen_height: libc::int32_t,
                                title: *const c_char)
                                -> *mut GlutinFacade {
    let w: u32 = screen_width as u32;
    let h: u32 = screen_height as u32;
    let window_title: String;

    unsafe {
        window_title = CStr::from_ptr(title).to_string_lossy().into_owned();
        let display: GlutinFacade = glutin::WindowBuilder::new()
	        //.resizable()
	        //.with_vsync()
	        //with_gl_debug_flag(true)
	        .with_title(window_title)
	        .with_visibility(true)
	        .with_dimensions(w, h)
	        .build_glium()
	        .unwrap();
        return Box::into_raw(Box::new(display));
    }
}

#[no_mangle]
pub unsafe extern "C" fn free_window(ptr: *mut GlutinFacade) {
    Box::from_raw(ptr);
}

#[no_mangle]
pub extern "C" fn create_db_loader(filename_cstr: *const c_char) -> Box<DBLoader> {
    unsafe {
        let filename: String = CStr::from_ptr(filename_cstr).to_string_lossy().into_owned();
        let dbloader: DBLoader = DBLoader::new(&filename);
        println!("Loaded {}", &filename);
        Box::new(dbloader)
    }
}

#[no_mangle]
pub extern "C" fn free_db_loader(ptr: *mut DBLoader) {
    unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern "C" fn create_renderer_from_db_loader(dbloader: &DBLoader,
                                                 display: &GlutinFacade)
                                                 -> Box<Renderer> {
    Box::new(renderer::Renderer::new(display, dbloader.load_scene()))
}

#[no_mangle]
pub extern "C" fn free_renderer(ptr: *mut Renderer) {
    unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern "C" fn get_shader_from_db_loader(shader_name_cstr: *const c_char,
                                            dbloader: &DBLoader,
                                            renderer: &Renderer,
                                            display: &GlutinFacade)
                                            -> Box<glium::program::Program> {
    unsafe {
        let shader_name: String = CStr::from_ptr(shader_name_cstr).to_string_lossy().into_owned();
        let shader_program = renderer.create_shader_program(&shader_name, dbloader, display);
        return Box::new(shader_program);
    }
}

#[no_mangle]
pub extern "C" fn free_shader(ptr: *mut glium::program::Program) {
    unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern "C" fn poll_quit_event(display: &GlutinFacade) -> libc::int32_t {
    let mut quit: libc::int32_t = 0;
    for event in display.poll_events() {
        match event {
            Event::Closed => quit = 1,
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => {
                quit = 1
            }
            _ => (),
        }
    }
    if quit.clone() > 0 {
        println!("Polling quit event returned {}", quit.clone());
    }

    return quit;
}

#[no_mangle]
pub extern "C" fn render(renderer: &Renderer,
                         shader_program: &glium::program::Program,
                         display: &GlutinFacade) {
    renderer.render(display, shader_program);
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

    // #[test]
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
