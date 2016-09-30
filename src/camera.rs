// Copyright (C) 2016 Chris Liebert

extern crate libc;

use nalgebra;
use nalgebra::{Eye, Isometry3, Matrix4, PerspectiveMatrix3, ToHomogeneous, Vector3};

/// `Camera`
///
/// Data for camera manipulation. Only modelview_matrix and projection_matrix are passed to the
/// shader as uniform values.
///
#[derive(Debug)]
#[repr(C)]
pub struct Camera {
    pub modelview_matrix: [[f32; 4]; 4],
    pub projection_matrix: [[f32; 4]; 4],
    pub position: Vector3<f32>,
    pub direction: Vector3<f32>,
    pub right: Vector3<f32>,
    pub up: Vector3<f32>,
    pub pitch: f64,
    pub yaw: f64,
    pub roll: f64,
}

use std::f64::consts::{FRAC_PI_2, PI};

impl Camera {
    /// Create a new perspective `Camera` using screen_width and screen_height to calculate the
    /// aspect ratio.
    ///
    pub fn new(screen_width: f32, screen_height: f32) -> Camera {
        // Set up camera
        let projection_matrix =
            PerspectiveMatrix3::new(screen_width / screen_height, 45.0, 0.1, 1000.0);
        let projection_matrix: Matrix4<f32> = projection_matrix.to_matrix();
        let matrix: Matrix4<f32> = Eye::new_identity(4);
        let modelview_matrix_array = *matrix.as_ref();

        let camera: Camera = Camera {
            modelview_matrix: modelview_matrix_array,
            projection_matrix: *projection_matrix.as_ref(),
            position: Vector3::new(0.0f32, 1.0f32, 0.0f32),
            direction: Vector3::new(0.0f32, 0.0f32, 1.0f32),
            right: Vector3::new(1.0f32, 0.0f32, 0.0f32),
            up: Vector3::new(0.0f32, 1.0f32, 0.0f32),
            pitch: PI,
            yaw: 0.0,
            roll: 0.0,
        };
        camera.aim(0.0, 0.0)
    }

    /// Rotate a `Camera` in a relative direction perpendicular to the focal point.
    ///
    pub fn aim(&self, x: f64, y: f64) -> Camera {
        
        
        let factor: f64 = 0.01;
        let horizontal: f64 = self.pitch + x * factor;
        let vertical: f64 = self.yaw + y * factor;

        let direction: Vector3<f32> = Vector3::new(
            (vertical.cos() * horizontal.sin()) as f32,
            vertical.sin() as f32,
            (vertical.cos() * horizontal.cos()) as f32);

        let right: Vector3<f32> = Vector3::new(
            (horizontal - FRAC_PI_2).sin() as f32,
            0.0f32,
            (horizontal - FRAC_PI_2).cos() as f32);

        let up: Vector3<f32> = nalgebra::cross(&right, &direction);
        let position: Vector3<f32> = Vector3::new(self.position[0], self.position[1], self.position[2]);
        let iso3 = Isometry3::look_at_rh(&position.to_point(),
                                         &(position + direction).to_point(),
                                         &up);
        let matrix: Matrix4<f32> = iso3.to_homogeneous();
        
        Camera {
            modelview_matrix: *matrix.as_ref(),
            projection_matrix: self.projection_matrix,
            position: self.position,
            direction: direction,
            right: right,
            up: up,
            pitch: horizontal,
            yaw: vertical,
            roll: 0.0f64,
        }
    }

    /// Move a `Camera` backward by a specified amount.
    ///
    pub fn move_backward(&self, amount: f32) -> Camera {
        self.move_forward(-1.0 * amount)
    }

    /// Move a `Camera` forward by a specified amount.
    ///
    pub fn move_forward(&self, amount: f32) -> Camera {
        
        let scaled_direction: Vector3<f32> = amount * self.direction;
        let position: Vector3<f32> = self.position + scaled_direction;
        let iso3 = Isometry3::look_at_rh(&position.to_point(),
                                         &(position + self.direction).to_point(),
                                         &self.up);
        let matrix: Matrix4<f32> = iso3.to_homogeneous();
        Camera {
            modelview_matrix: *matrix.as_ref(),
            projection_matrix: self.projection_matrix,
            position: position,
            direction: self.direction,
            right: self.right,
            up: self.up,
            pitch: self.pitch,
            yaw: self.yaw,
            roll: self.roll,
        }
    }

    /// Move a `Camera` left by a specified amount.
    ///
    pub fn move_left(&self, amount: f32) -> Camera {
        self.move_right(-1.0 * amount)
    }

    /// Move a `Camera` right by a specified amount.
    ///
    pub fn move_right(&self, amount: f32) -> Camera {
        let scaled_right: Vector3<f32> = amount * self.right;
        let position: Vector3<f32> = self.position + scaled_right;
        let iso3 = Isometry3::look_at_rh(&position.to_point(),
                                         &(position + self.direction).to_point(),
                                         &self.up);
        let matrix: Matrix4<f32> = iso3.to_homogeneous();
        Camera {
            modelview_matrix: *matrix.as_ref(),
            projection_matrix: self.projection_matrix,
            position: position,
            direction: self.direction,
            right: self.right,
            up: self.up,
            pitch: self.pitch,
            yaw: self.yaw,
            roll: self.roll,
        }
    }
}

/// `extern void camera_aim(Camera camera, double x, double y);`
///
#[no_mangle]
pub extern "C" fn camera_aim(camera: *mut Camera, x: libc::c_double, y: libc::c_double) -> Box<Camera> {
    let camera: Box<Camera> = unsafe { Box::from_raw(camera) };
    let new_camera = Box::new(camera.aim(x as f64, y as f64));
    drop(camera);
    new_camera
}

/// `extern void camera_move_forward(Camera camera, float amount);`
///
#[no_mangle]
pub extern "C" fn camera_move_forward(camera: *mut Camera, amount: libc::c_float) -> Box<Camera> {
    let camera: Box<Camera> = unsafe { Box::from_raw(camera) };
    let new_camera = Box::new(camera.move_forward(amount as f32));
    drop(camera);
    new_camera
}

/// `extern void camera_move_backward(Camera camera, float amount);`
///
#[no_mangle]
pub extern "C" fn camera_move_backward(camera: *mut Camera, amount: libc::c_float) -> Box<Camera> {
    let camera: Box<Camera> = unsafe { Box::from_raw(camera) };
    let new_camera = Box::new(camera.move_backward(amount as f32));
    drop(camera);
    new_camera
}

/// `extern void camera_move_left(Camera camera, float amount);`
///
#[no_mangle]
pub extern "C" fn camera_move_left(camera: *mut Camera, amount: libc::c_float) -> Box<Camera> {
    let camera: Box<Camera> = unsafe { Box::from_raw(camera) };    
    let new_camera = Box::new(camera.move_left(amount as f32));
    drop(camera);
    new_camera
}

/// `extern void camera_move_right(Camera camera, float amount);`
///
#[no_mangle]
pub extern "C" fn camera_move_right(camera: *mut Camera, amount: libc::c_float) -> Box<Camera> {
    let camera: Box<Camera> = unsafe { Box::from_raw(camera) };   
    let new_camera = Box::new(camera.move_right(amount as f32));
    drop(camera);
    new_camera
}

/// `extern Camera create_camera(float screen_width, float screen_height);`
///
#[no_mangle]
pub extern "C" fn create_camera(screen_width: f32, screen_height: f32) -> Box<Camera> {
    Box::new(Camera::new(screen_width, screen_height))
}

/// `extern void free_camera(Camera memory);`
///
#[no_mangle]
pub extern "C" fn free_camera(ptr: *mut Camera) {
    let box_ptr: Box<Camera> = unsafe { Box::from_raw(ptr) };
    drop(box_ptr)
}