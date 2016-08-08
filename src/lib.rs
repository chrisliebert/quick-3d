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
use std::sync::{Arc, Mutex};

use glium::glutin;
use glium::glutin::{ElementState, Event, MouseButton, VirtualKeyCode};
use glium::DisplayBuild;
use glium::backend::glutin_backend::GlutinFacade;

use common::Camera;
use loader::DBLoader;
use renderer::Renderer;

/// The following methods annotated as #[no_mangle] provide external interfaces
/// that can be accessed from C and SWIG
///

/// Structure for querying stdin console input
///
/// ```c
/// /* C representation */
/// typedef void* ConsoleInput;
/// ```
///
pub struct ConsoleInput {
    pub thread_handle: std::thread::JoinHandle<i32>,
    pub buffer: Arc<Mutex<String>>,
    pub finished: Arc<Mutex<bool>>,
}

/// Structure for querying information about user input
///
/// ```c
/// /* C representation */
/// #include <stdbool.h>
/// typedef struct Input {
///		bool key_1, key_2, key_3, key_4, key_5, key_6, key_7, key_8, key_9, key_0;
///		bool a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z;
///		bool up, left, right, down;
///		bool space;
///		bool escape;
///		bool closed;
///		int mouse_dx, mouse_dy;
///		int mouse_x, mouse_y;
///		bool mouse_left, mouse_right;
///} Input;
/// ```
///
#[repr(C)]
pub struct Input {
	pub key_1: bool,
	pub key_2: bool,
	pub key_3: bool,
	pub key_4: bool,
	pub key_5: bool,
	pub key_6: bool,
	pub key_7: bool,
	pub key_8: bool,
	pub key_9: bool,
	pub key_0: bool,
	
	pub a: bool,
	pub b: bool,
	pub c: bool,
	pub d: bool,
	pub e: bool,
	pub f: bool,
	pub g: bool,
	pub h: bool,
	pub i: bool,
	pub j: bool,
	pub k: bool,
	pub l: bool,
	pub m: bool,
	pub n: bool,
	pub o: bool,
	pub p: bool,
	pub q: bool,
	pub r: bool,
	pub s: bool,
	pub t: bool,
	pub u: bool,
	pub v: bool,
	pub w: bool,
	pub x: bool,
	pub y: bool,
	pub z: bool,
	
	pub up: bool,
	pub left: bool,
	pub right: bool,
	pub down: bool,
	
	pub space: bool,
	pub escape: bool,
    pub closed: bool,
    
    // Mouse
    pub mouse_dx: i32,
    pub mouse_dy: i32,
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub mouse_left: bool,
    pub mouse_right: bool,
}

/// `extern void camera_aim(Camera camera, double x, double y);`
///
#[no_mangle]
pub extern "C" fn camera_aim(camera: &Camera, x: libc::c_double, y: libc::c_double) {
	camera.aim(x as f64, y as f64);
}

/// `extern void camera_move_forward(Camera camera, float amount);`
///
#[no_mangle]
pub extern "C" fn camera_move_forward(camera: &Camera, amount: libc::c_float) {
	camera.move_forward(amount as f32);
}

/// `extern void camera_move_backward(Camera camera, float amount);`
///
#[no_mangle]
pub extern "C" fn camera_move_backward(camera: &Camera, amount: libc::c_float) {
	camera.move_backward(amount as f32);
}

/// `extern void camera_move_left(Camera camera, float amount);`
///
#[no_mangle]
pub extern "C" fn camera_move_left(camera: &Camera, amount: libc::c_float) {
	camera.move_left(amount as f32);
}

/// `extern void camera_move_right(Camera camera, float amount);`
///
#[no_mangle]
pub extern "C" fn camera_move_right(camera: &Camera, amount: libc::c_float) {
	camera.move_right(amount as f32);
}

/// `extern void camera_update(Camera camera);`
///
#[no_mangle]
pub extern "C" fn camera_update(camera: &Camera) {
	camera.update();
}

/// `extern Camera create_camera(float screen_width, float screen_height);`
///
#[no_mangle]
pub extern "C" fn create_camera(screen_width: f32, screen_height: f32) -> Box<Camera> {
    Box::new(Camera::new(screen_width, screen_height))
}

/// `extern ConsoleInput create_console_reader();`
///
#[no_mangle]
pub extern "C" fn create_console_reader() -> Box<ConsoleInput> {
	use std::thread;
	let buffer_arc: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
	let buffer_arc_copy = buffer_arc.clone();
	let finished_arc: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
	let finished_arc_copy = finished_arc.clone();
	{
		// Initialize the finished value
		let finished = finished_arc.clone();
		let mut finished_lock = finished.lock().unwrap();
		*finished_lock = false;
	}
	
	let child = thread::spawn(move || {
		println!("Enter command, or return to close console");
		'console: loop {
			let mut buffer = String::new();
			match std::io::stdin().read_line(&mut buffer) {
				Ok(_) => {
					buffer = buffer
						.replace("\r", "")
						.replace("\n", " ");
					if 1 == buffer.len() { break 'console };
					let arc = buffer_arc.clone();
					let mut writer = arc.lock().unwrap();
					let mut new_string: String = (*writer).clone();
					new_string.push_str(&buffer);
					*writer = new_string;
					std::thread::yield_now();
				},
				Err(e) => println!("Error: {:?}", e),
			}
		}
		println!("Console closed");
		let finished = finished_arc.clone();
		let mut finished_lock = finished.lock().unwrap();
		*finished_lock = true;
	    return 0;
	});
	
	std::thread::yield_now();
	Box::new(ConsoleInput{thread_handle: child, buffer: buffer_arc_copy, finished: finished_arc_copy})
}

/// `extern DBLoader create_db_loader(const char* filename);`
///
#[no_mangle]
pub extern "C" fn create_db_loader(filename_cstr: *const c_char) -> Box<DBLoader> {
    unsafe {
        let filename: String = CStr::from_ptr(filename_cstr).to_string_lossy().into_owned();
        let dbloader: DBLoader = DBLoader::new(&filename);
        println!("Loaded {}", &filename);
        Box::new(dbloader)
    }
}

/// `extern Display create_display(int screen_width, int screen_height, const char* title);`
///
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

/// `extern Renderer create_renderer_from_db_loader(DBLoader loader, Display display);`
///
#[no_mangle]
pub extern "C" fn create_renderer_from_db_loader(dbloader: &DBLoader,
                                                 display: &GlutinFacade)
                                                 -> Box<Renderer> {
    Box::new(renderer::Renderer::new(display, dbloader.load_scene()))
}
                                                 
/// `extern bool console_is_closed(ConsoleInput console);`
///
#[no_mangle]
pub extern "C" fn console_is_closed(console: &ConsoleInput) -> bool {
	let arc = console.finished.clone();
	let mutex = arc.lock().unwrap();
	mutex.clone()
}

/// `extern void free_camera(Camera camera);`
///
#[no_mangle]
pub extern "C" fn free_camera(ptr: *mut Camera) {
    let box_ptr: Box<Camera> = unsafe { Box::from_raw(ptr) };
    drop(box_ptr)
}

/// `extern void free_db_loader(DBLoader dbloader);`
///
#[no_mangle]
pub extern "C" fn free_db_loader(ptr: *mut DBLoader) {
    let box_ptr: Box<DBLoader> = unsafe { Box::from_raw(ptr) };
    drop(box_ptr)
}

/// `extern void free_display(Display memory);`
///
#[no_mangle]
pub extern "C" fn free_display(ptr: *mut GlutinFacade) {
    let box_ptr: Box<GlutinFacade> = unsafe { Box::from_raw(ptr) };
    drop(box_ptr)
}

/// `extern void free_renderer(Renderer renderer);`
///
#[no_mangle]
pub extern "C" fn free_renderer(ptr: *mut Renderer) {
    let box_ptr: Box<Renderer> = unsafe { Box::from_raw(ptr) };
    drop(box_ptr)
}

/// `extern void free_shader(Shader shader);`
///
#[no_mangle]
pub extern "C" fn free_shader(ptr: *mut glium::program::Program) {
    let box_ptr: Box<glium::program::Program> = unsafe { Box::from_raw(ptr) };
    drop(box_ptr)
}

/// `extern Shader get_shader_from_db_loader(const char* name, DBLoader dbloader, Renderer renderer, Display display);`
///
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

/// Check for user input events
///
/// `extern Input poll_event(Display display);`
///
#[no_mangle]
pub unsafe extern "C" fn poll_event(display: &GlutinFacade) -> Input {
	static mut mouse_last_x: i32 = 0;
	static mut mouse_last_y: i32 = 0;
	static mut _mouse_dx: i32 = 0;
	static mut _mouse_dy: i32 = 0;
	static mut left_button_pressed: bool = false;
	static mut right_button_pressed: bool = false;
	
	// Top numeric key states
	static mut key_1: bool = false;
	static mut key_2: bool = false;
	static mut key_3: bool = false;
	static mut key_4: bool = false;
	static mut key_5: bool = false;
	static mut key_6: bool = false;
	static mut key_7: bool = false;
	static mut key_8: bool = false;
	static mut key_9: bool = false;
	static mut key_0: bool = false;
	
	// Letter key states
	static mut a: bool = false;
	static mut b: bool = false;
	static mut c: bool = false;
	static mut d: bool = false;
	static mut e: bool = false;
	static mut f: bool = false;
	static mut g: bool = false;
	static mut h: bool = false;
	static mut i: bool = false;
	static mut j: bool = false;
	static mut k: bool = false;
	static mut l: bool = false;
	static mut m: bool = false;
	static mut n: bool = false;
	static mut o: bool = false;
	static mut p: bool = false;
	static mut q: bool = false;
	static mut r: bool = false;
	static mut s: bool = false;
	static mut t: bool = false;
	static mut u: bool = false;
	static mut v: bool = false;
	static mut w: bool = false;
	static mut x: bool = false;
	static mut y: bool = false;
	static mut z: bool = false;

	static mut up: bool = false;
	static mut left: bool = false;
	static mut right: bool = false;
	static mut down: bool = false;
	
	static mut space: bool = false;
	static mut escape: bool = false;
	
	
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
    
    // The margin from the edge of the screen to allow before
	// mouse cursor is moved to center
	
    let mouse_grab_margin: i32 = screen_center_y / 2;

	let mut closed = false;
    
    for event in display.poll_events() {
        match event {
        	Event::Closed => closed = true,
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Key1)) => {
                key_1 = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Key1)) => {
                key_1 = false
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Key2)) => {
                key_2 = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Key2)) => {
                key_2 = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Key3)) => {
                key_3 = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Key3)) => {
                key_3 = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Key4)) => {
                key_4 = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Key4)) => {
                key_4 = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Key5)) => {
                key_5 = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Key5)) => {
                key_5 = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Key6)) => {
                key_6 = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Key6)) => {
                key_6 = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Key7)) => {
                key_7 = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Key7)) => {
                key_7 = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Key8)) => {
                key_8 = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Key8)) => {
                key_8 = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Key9)) => {
                key_9 = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Key9)) => {
                key_9 = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Key0)) => {
                key_0 = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Key0)) => {
                key_0 = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::A)) => {
                a = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::A)) => {
                a = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::B)) => {
                b = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::B)) => {
                b = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::C)) => {
                c = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::C)) => {
                c = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::D)) => {
                d = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::D)) => {
                d = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::E)) => {
                e = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::E)) => {
                e = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::F)) => {
                f = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::F)) => {
                f = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::G)) => {
                g = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::G)) => {
                g = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::H)) => {
                h = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::H)) => {
                h = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::I)) => {
                i = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::I)) => {
                i = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::J)) => {
                j = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::J)) => {
                j = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::K)) => {
                k = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::K)) => {
                k = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::L)) => {
                l = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::L)) => {
                l = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::M)) => {
                m = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::M)) => {
                m = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::N)) => {
                n = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::N)) => {
                n = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::O)) => {
                o = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::O)) => {
                o = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::P)) => {
                p = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::P)) => {
                p = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Q)) => {
                q = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Q)) => {
                q = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::R)) => {
                r = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::R)) => {
                r = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::S)) => {
                s = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::S)) => {
                s = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::T)) => {
                t = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::T)) => {
                t = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::U)) => {
                u = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::U)) => {
                u = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::V)) => {
                v = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::V)) => {
                v = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::W)) => {
                w = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::W)) => {
                w = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::X)) => {
                x = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::X)) => {
                x = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Y)) => {
                y = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Y)) => {
                y = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Z)) => {
                z = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Z)) => {
                z = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Space)) => {
                space = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Space)) => {
                space = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => {
                escape = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Escape)) => {
                escape = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Up)) => {
                up = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Up)) => {
                up = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Left)) => {
                left = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Left)) => {
                left = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Right)) => {
                right = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Right)) => {
                right = false;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Down)) => {
                down = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Down)) => {
                down = false;
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
            Event::MouseMoved(mx, my) => {
                _mouse_dx = mouse_last_x - mx;
                _mouse_dy = mouse_last_y - my;
                mouse_last_x = mx;
		        mouse_last_y = my;
                if left_button_pressed {
                    if mx + mouse_grab_margin >= screen_width as i32 || mx <= mouse_grab_margin {
                        let _ =
                            display.get_window().unwrap().set_cursor_position(screen_center_x, my);
                            _mouse_dx = 0;
                            mouse_last_x = screen_center_x;
                    } else if my + mouse_grab_margin >= screen_height as i32 || my <= mouse_grab_margin {
                        let _ =
                            display.get_window().unwrap().set_cursor_position(mx, screen_center_y);
                            _mouse_dy = 0;
                            mouse_last_y = screen_center_y;
                    }
                } 
            }
            _ => () // All events that arent matched are consumed here
        }
    }
    Input {
    	key_1: key_1,
    	key_2: key_2,
    	key_3: key_3,
    	key_4: key_4,
    	key_5: key_5,
    	key_6: key_6,
    	key_7: key_7,
    	key_8: key_8,
    	key_9: key_9,
    	key_0: key_0,
    	a: a,
    	b: b,
    	c: c,
    	d: d,
    	e: e,
    	f: f,
    	g: g,
    	h: h,
    	i: i,
    	j: j,
    	k: k,
    	l: l,
    	m: m,
    	n: n,
    	o: o,
    	p: p,
    	q: q,
    	r: r,
    	s: s,
    	t: t,
    	u: u,
    	v: v,
    	w: w,
    	x: x,
    	y: y,
    	z: z,
    	up: up,
    	left: left,
    	right: right,
    	down: down,
    	space: space,
    	escape: escape,
		closed: closed,
	    
		// Mouse
        mouse_dx: _mouse_dx,
        mouse_dy: _mouse_dy,
        mouse_x: mouse_last_x,
        mouse_y: mouse_last_y,
        mouse_left: left_button_pressed,
        mouse_right: right_button_pressed,
    }
}

/// `extern char* read_console_buffer(ConsoleInput console);`
///
#[no_mangle]
pub extern "C" fn read_console_buffer(console: &ConsoleInput) -> *mut libc::c_char {
	let retval: String;
	let arc = console.buffer.clone();
	let mut mutex = arc.lock().unwrap();
	retval = (*mutex).clone();
	*mutex = String::new();
	CString::new(retval).unwrap().into_raw()
}

/// `extern void render(Renderer renderer, Shader shader, Camera camera, Display display);`
///
#[no_mangle]
pub extern "C" fn render(renderer: &Renderer,
                         shader_program: &glium::program::Program,
                         camera: &Camera,
                         display: &GlutinFacade) {
    renderer.render(display, shader_program, camera);
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

use std::ffi::CString;

/// `extern void wait_console_quit(ConsoleInput console);`
///
#[no_mangle]
pub extern "C" fn wait_console_quit(handle: *mut ConsoleInput) {
	let child: Box<ConsoleInput> = unsafe { Box::from_raw(handle) };
	match child.thread_handle.join() {
		Ok(c) => drop(c),
		Err(e) => println!("Console thread did not return: {:?}", e),
	}
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
