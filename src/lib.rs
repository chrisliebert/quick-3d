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
use glium::glutin::{ElementState, Event, MouseButton, VirtualKeyCode};
use glium::DisplayBuild;
use glium::backend::glutin_backend::GlutinFacade;

use common::Camera;
use loader::DBLoader;
use renderer::Renderer;

#[no_mangle]
pub extern "C" fn camera_aim(camera: &Camera, x: libc::c_double, y: libc::c_double) {
	camera.aim(x as f64, y as f64);
}

#[no_mangle]
pub extern "C" fn camera_move_forward(camera: &Camera, amount: libc::c_float) {
	camera.move_forward(amount as f32);
}

#[no_mangle]
pub extern "C" fn camera_move_backward(camera: &Camera, amount: libc::c_float) {
	camera.move_backward(amount as f32);
}

#[no_mangle]
pub extern "C" fn camera_move_left(camera: &Camera, amount: libc::c_float) {
	camera.move_left(amount as f32);
}

#[no_mangle]
pub extern "C" fn camera_move_right(camera: &Camera, amount: libc::c_float) {
	camera.move_right(amount as f32);
}

#[no_mangle]
pub extern "C" fn camera_update(camera: &Camera) {
	camera.update();
}

#[no_mangle]
pub extern "C" fn create_camera(screen_width: f32, screen_height: f32) -> Box<Camera> {
    Box::new(Camera::new(screen_width, screen_height))
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
pub extern "C" fn create_display(screen_width: libc::int32_t,
                                 screen_height: libc::int32_t,
                                 title: *const c_char)
                                 -> Box<GlutinFacade> {
    let w: u32 = screen_width as u32;
    let h: u32 = screen_height as u32;
    let window_title: String;

    unsafe {
        window_title = CStr::from_ptr(title).to_string_lossy().into_owned();
        let display: GlutinFacade = glutin::WindowBuilder::new()
	        //.resizable()
	        //.with_vsync()
	        .with_gl_debug_flag(true)
	        .with_title(window_title)
	        .with_visibility(true)
	        .with_dimensions(w, h)
	        .build_glium()
	        .unwrap();
        return Box::new(display);
    }
}

#[no_mangle]
pub extern "C" fn create_renderer_from_db_loader(dbloader: &DBLoader,
                                                 display: &GlutinFacade)
                                                 -> Box<Renderer> {
    Box::new(renderer::Renderer::new(display, dbloader.load_scene()))
}

#[no_mangle]
pub extern "C" fn free_camera(ptr: *mut Camera) {
    let box_ptr: Box<Camera> = unsafe { Box::from_raw(ptr) };
    Box::into_raw(box_ptr);
}

#[no_mangle]
pub extern "C" fn free_db_loader(ptr: *mut DBLoader) {
    let box_ptr: Box<DBLoader> = unsafe { Box::from_raw(ptr) };
    Box::into_raw(box_ptr);
}

#[no_mangle]
pub extern "C" fn free_display(ptr: *mut GlutinFacade) {
    let box_ptr: Box<GlutinFacade> = unsafe { Box::from_raw(ptr) };
    Box::into_raw(box_ptr);
}

#[no_mangle]
pub extern "C" fn free_renderer(ptr: *mut Renderer) {
    let box_ptr: Box<Renderer> = unsafe { Box::from_raw(ptr) };
    Box::into_raw(box_ptr);
}

#[no_mangle]
pub extern "C" fn free_shader(ptr: *mut glium::program::Program) {
    let box_ptr: Box<glium::program::Program> = unsafe { Box::from_raw(ptr) };
    Box::into_raw(box_ptr);
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

#[repr(C)]
pub struct Mouse {
    pub dx: i32,
    pub dy: i32,
    pub last_x: i32,
    pub last_y: i32,
    pub left_button_pressed: bool,
    pub right_button_pressed: bool,
}

#[repr(C)]
pub struct Input {
    pub mouse: Mouse,
    pub closed: bool,
}

#[no_mangle]
pub unsafe extern "C" fn poll_event(display: &GlutinFacade) -> Input {
	// The margin from the edge of the screen to allow before
	// mouse cursor is moved to center
	
	static mut mouse_last_x: i32 = 0;
	static mut mouse_last_y: i32 = 0;
	static mut _mouse_dx: i32 = 0;
	static mut _mouse_dy: i32 = 0;
	static mut left_button_pressed: bool = false;
	static mut right_button_pressed: bool = false;
 
    // Get the screen width and height in pixels
    let pixel_dimensions: (u32, u32) = match display.get_window() {
        Some(window) => window.get_inner_size_pixels().unwrap(),
        None => {
            panic!("Error retrieving window when querying display size.");
        }
    };
    let screen_width: u32 = pixel_dimensions.0;
    let screen_height: u32 = pixel_dimensions.1;
    
    let screen_center_x: i32 = (screen_width / 2) as i32;
    let screen_center_y: i32 = (screen_height / 2) as i32;
    
    let mouse_grab_margin: i32 = screen_center_y / 2;

    let mut closed = false;
    
    for event in display.poll_events() {
        match event {
        	Event::Closed => closed = true,
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => {
                closed = true
            }
            Event::MouseInput(ElementState::Pressed, MouseButton::Left) => {
                left_button_pressed = true;
            }
            Event::MouseInput(ElementState::Released, MouseButton::Left) => {
                left_button_pressed = false;
            }
            Event::MouseInput(ElementState::Pressed, MouseButton::Right) => {
                right_button_pressed = true;
            }
            Event::MouseInput(ElementState::Released, MouseButton::Right) => {
                right_button_pressed = false;
            }
            Event::MouseMoved(x, y) => {
                _mouse_dx = mouse_last_x - x;
                _mouse_dy = mouse_last_y - y;
                mouse_last_x = x;
		        mouse_last_y = y;
                if left_button_pressed {
                    if x + mouse_grab_margin >= screen_width as i32 || x <= mouse_grab_margin {
                        let _ =
                            display.get_window().unwrap().set_cursor_position(screen_center_x, y);
                            _mouse_dx = 0;
                            mouse_last_x = screen_center_x;
                    } else if y + mouse_grab_margin >= screen_height as i32 || y <= mouse_grab_margin {
                        let _ =
                            display.get_window().unwrap().set_cursor_position(x, screen_center_y);
                            _mouse_dy = 0;
                            mouse_last_y = screen_center_y;
                    }
                } 
            }
            _ => ()
        }
    }
    Input {
	    mouse: Mouse {
	        dx: _mouse_dx,
	        dy: _mouse_dy,
	        last_x: mouse_last_x,
	        last_y: mouse_last_y,
	        left_button_pressed: left_button_pressed,
	        right_button_pressed: right_button_pressed,
	    },
	    closed: closed,
    }
}

#[no_mangle]
pub extern "C" fn render(renderer: &Renderer,
                         shader_program: &glium::program::Program,
                         camera: &Camera,
                         display: &GlutinFacade) {
    renderer.render(display, shader_program, camera);
}

#[cfg(test)]
mod tests {
    use glium::glutin;
    use glium::backend::glutin_backend::GlutinFacade;
    use glium::DisplayBuild;
    use loader;
    use common::Scene;

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
    }

}
