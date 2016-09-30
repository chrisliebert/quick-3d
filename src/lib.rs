// Copyright(C) 2016 Chris Liebert

#![crate_type = "lib"]
#![crate_name = "quick3d"]
#![allow(dead_code)]
#[macro_use]
extern crate glium;
extern crate rustc_serialize;
extern crate bincode;
extern crate flate2;

pub mod frustum;
pub mod common;
pub mod dbloader;
pub mod input;
pub mod camera;
pub mod scene;
pub mod shader;
#[macro_use]
pub mod renderer;

extern crate nalgebra;
extern crate libc;

use std::ffi::CStr;
use std::ffi::CString;

use glium::glutin;
use glium::DisplayBuild;
use glium::backend::glutin_backend::GlutinFacade;

use dbloader::DBLoader;
use scene::Scene;

#[cfg(feature = "sqlite")]
#[link(name = "tinyobjloader", kind="static")]
#[link(name = "scenebuilder", kind="static")]
#[link(name = "stdc++")]
extern "C" {
    fn wavefrontToSQLite(wavefront_file: *const libc::c_char, database_file: *const libc::c_char);
}

/// The following methods annotated as #[no_mangle] provide external interfaces
/// that can be accessed from C and SWIG
///

/// `extern void obj2sqlite(const char* wavefront, const char* database);`
///
/// If the obj2sqlite feature is enabled, a bundled c++ library is used.
///
#[cfg(feature = "sqlite")]
#[no_mangle]
pub fn obj2sqlite(wavefront_file: *const libc::c_char, database_file: *const libc::c_char) {
    unsafe { wavefrontToSQLite(wavefront_file, database_file) };
}

/// `extern void obj2sqlite(const char* wavefront, const char* database);`
///
/// When the sqlite feature is disabled, the method is still availible 
/// so that the FFI wrappers will still build without modification the user
/// is notified that the feature is disabled.
#[cfg(not(feature = "sqlite"))]
#[no_mangle]
pub fn obj2sqlite(wavefront_file: *const libc::c_char, database_file: *const libc::c_char) {
    let filename1: String = unsafe{ CStr::from_ptr(wavefront_file).to_string_lossy().into_owned() };
    let filename2: String = unsafe{ CStr::from_ptr(database_file).to_string_lossy().into_owned() };
    panic!("Unable to convert {} to {}, the obj2sqlite feature is not enabled.", filename1, filename2);
}

/// `extern void obj2bin(const char* wavefront, const char* database);`
///
/// This will print an error if the sqlite feature is disabled
///

#[no_mangle]
pub fn obj2bin(wavefront_file: *const libc::c_char, binfile: *const libc::c_char) {
    let filename: String = unsafe{ CStr::from_ptr(wavefront_file.clone()).to_string_lossy().into_owned() };
    let binfile_str: String = unsafe{ CStr::from_ptr(binfile.clone()).to_string_lossy().into_owned() };
    let mut database_file: String = filename.clone();
    database_file.push_str(&String::from(".db"));
    let sqlite_file = CString::new(database_file.clone()).unwrap();
    obj2sqlite(wavefront_file, sqlite_file.clone().into_raw());
    let scene: Scene = match DBLoader::new(&(sqlite_file.into_string().unwrap())).unwrap().load_scene() {
        Ok(s) => s,
        Err(e) => panic!("Unable to load scene from SQLite: {:?}", e),
    };
    match scene.to_binary_file(binfile_str.clone()) {
        Ok(()) => println!("Saved {}", binfile_str),
        Err(e) => panic!("Unable to save binary file {}: {:?}", binfile_str, e),
    };
}

/// `extern void obj2compressed(const char* wavefront, const char* database);`
///
/// This will print an error if the obj2sqlite feature is disabled
///

#[no_mangle]
pub fn obj2compressed(wavefront_file: *const libc::c_char, binfile: *const libc::c_char) {
    let filename: String = unsafe{ CStr::from_ptr(wavefront_file.clone()).to_string_lossy().into_owned() };
    let binfile_str: String = unsafe{ CStr::from_ptr(binfile.clone()).to_string_lossy().into_owned() };
    let mut database_file: String = filename.clone();
    database_file.push_str(&String::from(".db"));
    let sqlite_file = CString::new(database_file.clone()).unwrap();
    obj2sqlite(wavefront_file, sqlite_file.clone().into_raw());
    let scene: Scene = match DBLoader::new(&(sqlite_file.into_string().unwrap())).unwrap().load_scene() {
        Ok(s) => s,
        Err(e) => panic!("Unable to load scene from SQLite: {:?}", e),
    };
    match scene.to_compressed_binary_file(binfile_str.clone()) {
        Ok(()) => println!("Saved {}", binfile_str),
        Err(e) => panic!("Unable to save compressed binary file {}: {:?}", binfile_str, e),
    };
}

/// `extern Display create_display(int screen_width, int screen_height, const char* title);`
///
#[no_mangle]
pub extern "C" fn create_display(screen_width: libc::int32_t,
                                 screen_height: libc::int32_t,
                                 title: *const libc::c_char)
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

/// `extern void free_display(Display memory);`
///
#[no_mangle]
pub extern "C" fn free_display(ptr: *mut GlutinFacade) {
    let box_ptr: Box<GlutinFacade> = unsafe { Box::from_raw(ptr) };
    drop(box_ptr)
}

use std::time::Duration;

/// `extern void thread_sleep(int ms);`
///
#[no_mangle]
pub extern "C" fn thread_sleep(ms: libc::int32_t) {
    std::thread::sleep(Duration::from_millis(ms as u64));
}

/// `extern void thread_yield();`
///
#[no_mangle]
pub extern "C" fn thread_yield() {
    std::thread::yield_now();
}


/// `extern void window_hide(Display display);`
///
#[no_mangle]
pub extern "C" fn window_hide(display: &GlutinFacade) {
    match display.get_window() {
        Some(w) => {
            w.hide();
        }
        None => {
            panic!("Error retrieving window");
        }
    };
}

/// `extern void window_show(Display display);`
///
#[no_mangle]
pub extern "C" fn window_show(display: &GlutinFacade) {
    match display.get_window() {
        Some(w) => {
            w.show();
        }
        None => {
            panic!("Error retrieving window");
        }
    };
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::thread;
    use glium::glutin;
    use glium::backend::glutin_backend::GlutinFacade;
    use glium::DisplayBuild;
    use dbloader::DBLoader;
    use camera::Camera;
    use renderer::Renderer;
    use shader::Shader;
    use scene::Scene;
    
    fn create_test_display() -> GlutinFacade {
        let display = glutin::WindowBuilder::new()
            .with_visibility(false)
            .with_gl_debug_flag(true)
            .build_glium()
            .unwrap();
        return display;
    }

    fn load_test_scene() -> Scene {
        Scene::from_compressed_binary_file(String::from("test.bin.gz")).unwrap()
    }

    #[test]
    fn test_scene_not_empty() {
        let scene: Scene = load_test_scene();
        assert!(scene.meshes.len() > 0);
        assert!(scene.materials.len() > 0);
    }
    
    #[test]
    fn display_creation() {
        // Opens a window for 100 miliseconds
        let display = create_test_display();
        let window = match display.get_window() {
            Some(w) => {
                w
            }
            None => {
                panic!("Error retrieving window");
            }
        };
        window.show();
        thread::sleep(Duration::from_millis(100));
    }
    
    #[test]
    fn renderer() {
        // Opens a window for 100 miliseconds and draws the contents of test.db
        let display = create_test_display();
        let window = match display.get_window() {
            Some(w) => {
                w
            }
            None => {
                panic!("Error retrieving window");
            }
        };
        window.show();
        let window_size = window.get_inner_size().unwrap();
        let screen_width = window_size.0;
        let screen_height = window_size.1;
        let mut camera = Camera::new(screen_width as f32, screen_height as f32);
        camera = camera.move_backward(6.0);        
        
        let scene = load_test_scene();
        let renderer = Renderer::new(&display, scene).unwrap();
        
        let shader_dbloader = DBLoader::new("shaders.db").unwrap();
        let shader_name = "default";
        let shader_program = Shader::from_dbloader(&shader_name, &shader_dbloader, &display).unwrap();
        
        renderer.render(&display, &shader_program, &camera).unwrap();
        
        thread::sleep(Duration::from_millis(100));
    }

    #[test]
    fn test_compressed_binary_scene() {
        // Opens a window for 100 miliseconds and draws the contents of test.db
        let display = create_test_display();
        let window = match display.get_window() {
            Some(w) => {
                w
            }
            None => {
                panic!("Error retrieving window");
            }
        };
        window.show();
        let window_size = window.get_inner_size().unwrap();
        let screen_width = window_size.0;
        let screen_height = window_size.1;
        let mut camera = Camera::new(screen_width as f32, screen_height as f32);
        camera = camera.move_backward(6.0);        
        
        let scene = load_test_scene();
        
        let binfile_name = String::from("test.bin");
        match scene.to_compressed_binary_file(binfile_name.clone()) {
            Ok(_) => (),
            Err(e) => panic!("Unable to save binary file {}: {:?}", binfile_name.clone(), e),
        };
        
        let scene_binary: Scene =  match Scene::from_compressed_binary_file(binfile_name.clone()) {
            Ok(s) => s,
            Err(e) => panic!("Unable to load binary scene from {}: {:?}", binfile_name.clone(), e),
        };
        
        assert!(scene == scene_binary);
        drop(scene);
        
        let renderer: Renderer = Renderer::new(&display, scene_binary).unwrap();
        
        let shader_dbloader = DBLoader::new("shaders.db").unwrap();
        let shader_name = "default";
        let shader_program = Shader::from_dbloader(&shader_name, &shader_dbloader, &display).unwrap();
        
        renderer.render(&display, &shader_program, &camera).unwrap();

        thread::sleep(Duration::from_millis(100));
    }
}
