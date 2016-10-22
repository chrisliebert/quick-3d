// Copyright (C) 2016 Chris Liebert

extern crate libc;

use std;
use std::ffi::CString;
use std::sync::{Arc, Mutex};

use glium::glutin::{ElementState, Event, MouseButton, VirtualKeyCode};
use glium::backend::glutin_backend::GlutinFacade;

#[derive(Debug)]
pub struct EventBuffer(Vec<Event>);

#[repr(C)]
pub enum KeyCode {
    KEY1, KEY2, KEY3, KEY4, KEY5, KEY6, KEY7, KEY8, KEY9, KEY0,
    A, B, C, D, E, F, G, H,I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    ESCAPE,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11,F12, F13, F14, F15,
    SNAPSHOT, SCROLL, PAUSE, INSERT, HOME, DELETE, END, PAGEDOWN, PAGEUP,
	LEFT, UP, RIGHT, DOWN, BACK, RETURN, SPACE, NUMLOCK,
    NUMPAD0, NUMPAD1, NUMPAD2, NUMPAD3, NUMPAD4, NUMPAD5, NUMPAD6, NUMPAD7, NUMPAD8, NUMPAD9,
    ABNTC1, ABNTC2, ADD, APOSTROPHE, APPS, AT, AX, BACKSLASH, CALCULATOR, CAPITAL, COLON, COMMA,
    CONVERT, DECIMAL, DIVIDE, EQUALS, GRAVE, KANA, KANJI, LALT, LBRACKET, LCONTROL, LMENU, LSHIFT, LWIN,
    MAIL, MEDIASELECT, MEDIASTOP, MINUS, MULTIPLY, MUTE, MYCOMPUTER, NAVIGATEFORWARD, NAVIGATEBACKWARD, NEXTTRACK,
	NOCONVERT, NUMPADCOMMA, NUMPADENTER, NUMPADEQUALS, OEM102, PERIOD, PLAYPAUSE, POWER, PREVTRACK, RALT, RBRACKET,
	RCONTROL, RMENU, RSHIFT, RWIN, SEMICOLON, SLASH, SLEEP, STOP, SUBTRACT, SYSRQ, TAB, UNDERLINE, UNLABELED,
	VOLUMEDOWN, VOLUMEUP, WAKE, WEBBACK, WEBFAVORITES, WEBFORWARD, WEBHOME, WEBREFRESH, WEBSEARCH, WEBSTOP, YEN,
}

impl KeyCode {
   pub fn get_enum(&self) -> VirtualKeyCode {
       match self {
           &KeyCode::KEY1 => VirtualKeyCode::Key1,
           &KeyCode::KEY2 => VirtualKeyCode::Key2,
           &KeyCode::KEY3 => VirtualKeyCode::Key3,
           &KeyCode::KEY4 => VirtualKeyCode::Key4,
           &KeyCode::KEY5 => VirtualKeyCode::Key5,
           &KeyCode::KEY6 => VirtualKeyCode::Key6,
           &KeyCode::KEY7 => VirtualKeyCode::Key7,
           &KeyCode::KEY8 => VirtualKeyCode::Key8,
           &KeyCode::KEY9 => VirtualKeyCode::Key9,
           &KeyCode::KEY0 => VirtualKeyCode::Key0,
           &KeyCode::A => VirtualKeyCode::A,
           &KeyCode::B => VirtualKeyCode::B,
           &KeyCode::C => VirtualKeyCode::C,
           &KeyCode::D => VirtualKeyCode::D,
           &KeyCode::E => VirtualKeyCode::E,
           &KeyCode::F => VirtualKeyCode::F,
           &KeyCode::G => VirtualKeyCode::G,
           &KeyCode::H => VirtualKeyCode::H,
           &KeyCode::I => VirtualKeyCode::I,
           &KeyCode::J => VirtualKeyCode::J,
           &KeyCode::K => VirtualKeyCode::K,
           &KeyCode::L => VirtualKeyCode::L,
           &KeyCode::M => VirtualKeyCode::M,
           &KeyCode::N => VirtualKeyCode::N,
           &KeyCode::O => VirtualKeyCode::O,
           &KeyCode::P => VirtualKeyCode::P,
           &KeyCode::Q => VirtualKeyCode::Q,
           &KeyCode::R => VirtualKeyCode::R,
           &KeyCode::S => VirtualKeyCode::S,
           &KeyCode::T => VirtualKeyCode::T,
           &KeyCode::U => VirtualKeyCode::U,
           &KeyCode::V => VirtualKeyCode::V,
           &KeyCode::W => VirtualKeyCode::W,
           &KeyCode::X => VirtualKeyCode::X,
           &KeyCode::Y => VirtualKeyCode::Y,
           &KeyCode::Z => VirtualKeyCode::Z,
           &KeyCode::ESCAPE => VirtualKeyCode::Escape,
           &KeyCode::F1 => VirtualKeyCode::F1,
           &KeyCode::F2 => VirtualKeyCode::F2,
           &KeyCode::F3 => VirtualKeyCode::F3,
           &KeyCode::F4 => VirtualKeyCode::F4,
           &KeyCode::F5 => VirtualKeyCode::F5,
           &KeyCode::F6 => VirtualKeyCode::F6,
           &KeyCode::F7 => VirtualKeyCode::F7,
           &KeyCode::F8 => VirtualKeyCode::F8,
           &KeyCode::F9 => VirtualKeyCode::F9,
           &KeyCode::F10 => VirtualKeyCode::F10,
           &KeyCode::F11 => VirtualKeyCode::F11,
           &KeyCode::F12 => VirtualKeyCode::F12,
           &KeyCode::F13 => VirtualKeyCode::F13,
           &KeyCode::F14 => VirtualKeyCode::F14,
           &KeyCode::F15 => VirtualKeyCode::F15,
           &KeyCode::SNAPSHOT => VirtualKeyCode::Snapshot,
           &KeyCode::SCROLL => VirtualKeyCode::Scroll,
           &KeyCode::PAUSE => VirtualKeyCode::Pause,
           &KeyCode::INSERT => VirtualKeyCode::Insert,
           &KeyCode::HOME => VirtualKeyCode::Home,
           &KeyCode::DELETE => VirtualKeyCode::Delete,
           &KeyCode::END => VirtualKeyCode::End,
           &KeyCode::PAGEDOWN => VirtualKeyCode::PageDown,
           &KeyCode::PAGEUP => VirtualKeyCode::PageUp,
           &KeyCode::LEFT => VirtualKeyCode::Left,
           &KeyCode::UP => VirtualKeyCode::Up,
           &KeyCode::RIGHT => VirtualKeyCode::Right,
           &KeyCode::DOWN => VirtualKeyCode::Down,
           &KeyCode::BACK => VirtualKeyCode::Back,
           &KeyCode::RETURN => VirtualKeyCode::Return,
           &KeyCode::SPACE => VirtualKeyCode::Space,
           &KeyCode::NUMLOCK => VirtualKeyCode::Numlock,
           &KeyCode::NUMPAD0 => VirtualKeyCode::Numpad0,
           &KeyCode::NUMPAD1 => VirtualKeyCode::Numpad1,
           &KeyCode::NUMPAD2 => VirtualKeyCode::Numpad2,
           &KeyCode::NUMPAD3 => VirtualKeyCode::Numpad3,
           &KeyCode::NUMPAD4 => VirtualKeyCode::Numpad4,
           &KeyCode::NUMPAD5 => VirtualKeyCode::Numpad5,
           &KeyCode::NUMPAD6 => VirtualKeyCode::Numpad6,
           &KeyCode::NUMPAD7 => VirtualKeyCode::Numpad7,
           &KeyCode::NUMPAD8 => VirtualKeyCode::Numpad8,
           &KeyCode::NUMPAD9 => VirtualKeyCode::Numpad9,
           &KeyCode::ABNTC1 => VirtualKeyCode::AbntC1,
           &KeyCode::ABNTC2 => VirtualKeyCode::AbntC2,
           &KeyCode::ADD => VirtualKeyCode::Add,
           &KeyCode::APOSTROPHE => VirtualKeyCode::Apostrophe,
           &KeyCode::APPS => VirtualKeyCode::Apps,
           &KeyCode::AT => VirtualKeyCode::At,
           &KeyCode::AX => VirtualKeyCode::Ax,
           &KeyCode::BACKSLASH => VirtualKeyCode::Backslash,
           &KeyCode::CALCULATOR => VirtualKeyCode::Calculator,
           &KeyCode::CAPITAL => VirtualKeyCode::Capital,
           &KeyCode::COLON => VirtualKeyCode::Colon,
           &KeyCode::COMMA => VirtualKeyCode::Comma,
           &KeyCode::CONVERT => VirtualKeyCode::Convert,
           &KeyCode::DECIMAL => VirtualKeyCode::Decimal,
           &KeyCode::DIVIDE => VirtualKeyCode::Divide,
           &KeyCode::EQUALS => VirtualKeyCode::Equals,
           &KeyCode::GRAVE => VirtualKeyCode::Grave,
           &KeyCode::KANA => VirtualKeyCode::Kana,
           &KeyCode::KANJI => VirtualKeyCode::Kanji,
           &KeyCode::LALT => VirtualKeyCode::LAlt,
           &KeyCode::LBRACKET => VirtualKeyCode::LBracket,
           &KeyCode::LCONTROL => VirtualKeyCode::LControl,
           &KeyCode::LMENU => VirtualKeyCode::LMenu,
           &KeyCode::LSHIFT => VirtualKeyCode::LShift,
           &KeyCode::LWIN => VirtualKeyCode::LWin,
           &KeyCode::MAIL => VirtualKeyCode::Mail,
           &KeyCode::MEDIASELECT => VirtualKeyCode::MediaSelect,
           &KeyCode::MEDIASTOP => VirtualKeyCode::MediaStop,
           &KeyCode::MINUS => VirtualKeyCode::Minus,
           &KeyCode::MULTIPLY => VirtualKeyCode::Multiply,
           &KeyCode::MUTE => VirtualKeyCode::Mute,
           &KeyCode::MYCOMPUTER => VirtualKeyCode::MyComputer,
           &KeyCode::NAVIGATEFORWARD => VirtualKeyCode::NavigateForward,
           &KeyCode::NAVIGATEBACKWARD => VirtualKeyCode::NavigateBackward,
           &KeyCode::NEXTTRACK => VirtualKeyCode::NextTrack,
           &KeyCode::NOCONVERT => VirtualKeyCode::NoConvert,
           &KeyCode::NUMPADCOMMA => VirtualKeyCode::NumpadComma,
           &KeyCode::NUMPADENTER => VirtualKeyCode::NumpadEnter,
           &KeyCode::NUMPADEQUALS => VirtualKeyCode::NumpadEquals,
           &KeyCode::OEM102 => VirtualKeyCode::OEM102,
           &KeyCode::PERIOD => VirtualKeyCode::Period,
           &KeyCode::PLAYPAUSE => VirtualKeyCode::PlayPause,
           &KeyCode::POWER => VirtualKeyCode::Power,
           &KeyCode::PREVTRACK => VirtualKeyCode::PrevTrack,
           &KeyCode::RALT => VirtualKeyCode::RAlt,
           &KeyCode::RBRACKET => VirtualKeyCode::RBracket,
           &KeyCode::RCONTROL => VirtualKeyCode::RControl,
           &KeyCode::RMENU => VirtualKeyCode::RMenu,
           &KeyCode::RSHIFT => VirtualKeyCode::RShift,
           &KeyCode::RWIN => VirtualKeyCode::RWin,
           &KeyCode::SEMICOLON => VirtualKeyCode::Semicolon,
           &KeyCode::SLASH => VirtualKeyCode::Slash,
           &KeyCode::SLEEP => VirtualKeyCode::Sleep,
           &KeyCode::STOP => VirtualKeyCode::Stop,
           &KeyCode::SUBTRACT => VirtualKeyCode::Subtract,
           &KeyCode::SYSRQ => VirtualKeyCode::Sysrq,
           &KeyCode::TAB => VirtualKeyCode::Tab,
           &KeyCode::UNDERLINE => VirtualKeyCode::Underline,
           &KeyCode::UNLABELED => VirtualKeyCode::Unlabeled,
           &KeyCode::VOLUMEDOWN => VirtualKeyCode::VolumeDown,
           &KeyCode::VOLUMEUP => VirtualKeyCode::VolumeUp,
           &KeyCode::WAKE => VirtualKeyCode::Wake,
           &KeyCode::WEBBACK => VirtualKeyCode::WebBack,
           &KeyCode::WEBFAVORITES => VirtualKeyCode::WebFavorites,
           &KeyCode::WEBFORWARD => VirtualKeyCode::WebForward,
           &KeyCode::WEBHOME => VirtualKeyCode::WebHome,
           &KeyCode::WEBREFRESH => VirtualKeyCode::WebRefresh,
           &KeyCode::WEBSEARCH => VirtualKeyCode::WebSearch,
           &KeyCode::WEBSTOP => VirtualKeyCode::WebStop,
           &KeyCode::YEN => VirtualKeyCode::Yen, 
       }
   }
}

#[repr(C)]
pub struct Mouse {
    pub x: i32,
    pub y: i32,
}

impl EventBuffer {
    pub fn closed(&self) -> bool {
       self.0.iter().any(|e| match e {
           &Event::Closed => true,
           _ => false,
       })
    }
    
    pub fn empty(&self) -> bool {
        0 == self.0.len()
    }
    
    pub fn new(display: &GlutinFacade) -> EventBuffer {
        let events: Vec<Event> = display.poll_events().collect();
        EventBuffer(events)
    }
    
    pub fn key_pressed(&self, keycode: u8) -> bool {
       self.0.iter().any(|e| match e {
           &Event::KeyboardInput(ElementState::Pressed, _keycode, _) => (_keycode == keycode),
            _ => false,
       })
    }
    
    pub fn key_released(&self, keycode: u8) -> bool {
       self.0.iter().any(|e| match e {
           &Event::KeyboardInput(ElementState::Released, _keycode, _) => (_keycode == keycode),
            _ => false,
       })
    }
    
    pub fn pressed(&self, keycode: VirtualKeyCode) -> bool {
       self.0.iter().any(|e| match e {
           &Event::KeyboardInput(ElementState::Pressed, _, virtual_keycode) => (virtual_keycode == Some(keycode)),
            _ => false,
       })
    }
    
    pub fn released(&self, keycode: VirtualKeyCode) -> bool {
       self.0.iter().any(|e| match e {
           &Event::KeyboardInput(ElementState::Pressed, _, virtual_keycode) => (virtual_keycode == Some(keycode)),
            _ => false,
       })
    }
    
    pub fn mouse_moved(&self) -> Mouse {
        let mut mouse_x: i32 = 0;
        let mut mouse_y: i32 = 0;
        for e in self.0.as_slice() {
            match e {
                &Event::MouseMoved(_x, _y) => { mouse_x = _x; mouse_y = _y; },
                _ => (),
            }
        }
        Mouse { x: mouse_x, y: mouse_y }
    }
    
    pub fn mouse_pressed_left(&self) -> bool {
        self.0.iter().any(|e| match e {
           &Event::MouseInput(ElementState::Pressed, MouseButton::Left) => true,
            _ => false,
       })
    }
    
    pub fn mouse_pressed_right(&self) -> bool {
        self.0.iter().any(|e| match e {
           &Event::MouseInput(ElementState::Pressed, MouseButton::Left) => true,
            _ => false,
       })
    }
    
    pub fn mouse_released_left(&self) -> bool {
        self.0.iter().any(|e| match e {
           &Event::MouseInput(ElementState::Released, MouseButton::Left) => true,
            _ => false,
       })
    }
    
    pub fn mouse_released_right(&self) -> bool {
        self.0.iter().any(|e| match e {
           &Event::MouseInput(ElementState::Released, MouseButton::Left) => true,
            _ => false,
       })
    }
}

/// Check for user close event
///
/// `extern bool display_closed(void* events);`
///
#[no_mangle]
pub extern "C" fn display_closed(buffer: &EventBuffer) -> bool {
    buffer.closed()
}

/// Poll the next batch of events
///
/// `extern EventBuffer get_events(Display display);`
///
#[no_mangle]
pub extern "C" fn get_events(display: &GlutinFacade) -> Box<EventBuffer> {
    Box::new(EventBuffer::new(display))
}

/// Checks if the event buffer is empty
///
/// `extern EventBuffer events_empty(EventBuffer display);`
///
#[no_mangle]
pub extern "C" fn events_empty(buffer: &EventBuffer) -> bool {
    buffer.empty()
}

/// Free an event buffer
///
/// `extern void free_events(EventBuffer queue);`
///
#[no_mangle]
pub extern "C" fn free_events(ptr: *mut EventBuffer) {
    let box_ptr: Box<EventBuffer> = unsafe { Box::from_raw(ptr) };
    drop(box_ptr);
}

/// Free a mouse struct
///
/// `extern void free_mouse(Mouse* mouse);`
///
#[no_mangle]
pub extern "C" fn free_mouse(ptr: *mut Mouse) {
    let box_ptr: Box<Mouse> = unsafe { Box::from_raw(ptr) };
    drop(box_ptr);
}

/// Print an event queue
///
/// `extern void print_events(EventBuffer queue);`
///
#[no_mangle]
pub extern "C" fn print_events(ptr: &EventBuffer) {
    println!("{:?}", ptr);
}

/// Check for user key press input events
///
/// `extern bool key_pressed(void* events, int keycode);`
///
#[no_mangle]
pub extern "C" fn key_pressed(buffer: &EventBuffer, keycode: KeyCode) -> bool {
    buffer.pressed(keycode.get_enum())
}

/// Check for user key release input events
///
/// `extern bool key_released(void* events, int keycode);`
///
#[no_mangle]
pub extern "C" fn key_released(buffer: &EventBuffer, keycode: KeyCode) -> bool {
   buffer.released(keycode.get_enum())
}

/// Check if the mouse has moved
///
/// `extern Mouse* mouse_moved(void* events);`
///
#[no_mangle]
pub extern "C" fn mouse_moved(buffer: &EventBuffer) -> Box<Mouse> {
   Box::new(buffer.mouse_moved())
}

/// Check for mouse left button was pressed
///
/// `extern bool mouse_pressed_left(void* events, int keycode);`
///
#[no_mangle]
pub extern "C" fn mouse_pressed_left(buffer: &EventBuffer) -> bool {
    buffer.mouse_pressed_left()
}

/// Check for mouse right button was pressed
///
/// `extern bool mouse_pressed_right(void* events, int keycode);`
///
#[no_mangle]
pub extern "C" fn mouse_pressed_right(buffer: &EventBuffer) -> bool {
    buffer.mouse_pressed_right()
}

/// Check for mouse left button was released
///
/// `extern bool mouse_released_left(void* events, int keycode);`
///
#[no_mangle]
pub extern "C" fn mouse_released_left(buffer: &EventBuffer) -> bool {
    buffer.mouse_released_left()
}

/// Check for mouse right button was released
///
/// `extern bool mouse_released_right(void* events, int keycode);`
///
#[no_mangle]
pub extern "C" fn mouse_released_right(buffer: &EventBuffer) -> bool {
    buffer.mouse_released_right()
}

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

/// `extern char* read_console_buffer(ConsoleInput console);`
///
#[no_mangle]
pub extern "C" fn read_console_buffer(console: &ConsoleInput) -> *mut libc::c_char {
    let retval: String;
    let arc = console.buffer.clone();
    let mut mutex = arc.lock().unwrap();
    retval = (*mutex).clone();
    *mutex = String::new();
    match CString::new(retval) {
        Ok(s) => s.into_raw(),
        Err(e) => panic!("Unable to read console buffer: {:?}", e),
    }
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
