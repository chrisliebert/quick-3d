// Copyright (C) 2016 Chris Liebert

extern crate libc;

use std;
use std::ffi::CString;
use std::sync::{Arc, Mutex};

use glium::glutin::{ElementState, Event, MouseButton, VirtualKeyCode};
use glium::backend::glutin_backend::GlutinFacade;

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
///        bool key_1, key_2, key_3, key_4, key_5, key_6, key_7, key_8, key_9, key_0;
///        bool a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z;
///        bool up, left, right, down;
///        bool space;
///        bool escape;
///        bool closed;
///        int mouse_dx, mouse_dy;
///        int mouse_x, mouse_y;
///        bool mouse_left, mouse_right;
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

/// `extern bool console_is_closed(ConsoleInput console);`
///
#[no_mangle]
pub extern "C" fn console_is_closed(console: &ConsoleInput) -> bool {
    let arc = console.finished.clone();
    let mutex = arc.lock().unwrap();
    mutex.clone()
}

/// Free an event
///
/// `extern void free_event(Input* input);`
///
#[no_mangle]
pub extern "C" fn free_event(ptr: *mut Input) {
    let box_ptr: Box<Input> = unsafe { Box::from_raw(ptr) };
    drop(box_ptr)
}


/// Check for user input events
///
/// `extern Input poll_event(Display display);`
///
#[no_mangle]
pub unsafe extern "C" fn poll_event(display: &GlutinFacade) -> Box<Input> {
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
    Box::new(Input {
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
    })
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