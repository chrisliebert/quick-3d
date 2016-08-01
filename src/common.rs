// Copyright(C) 2016 Chris Liebert
pub struct SceneNode {
    pub name: String,
    pub material_index: usize,
    pub start_position: i32,
    pub end_position: i32,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Material {
    pub name: String,
    pub diffuse: [f32; 3],
    pub diffuse_texname: String,
}

#[derive(Debug)]
pub struct Shader {
    pub name: String,
    pub vertex_source: String,
    pub fragment_source: String,
}

#[derive(Copy, Clone)]
pub struct Vertex8f32 {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub texcoord: [f32; 2],
}

use std::cell::RefCell;

#[derive(Clone)]
pub struct Mesh {
    pub name: String,
    pub vertices: Vec<Vertex8f32>,
    pub material_index: usize,
    pub matrix: RefCell<Matrix4<f32>>,
}

pub struct ImageBlob {
    pub name: String,
    pub image: Vec<u8>,
}

pub struct Scene {
    pub materials: Vec<Material>,
    pub meshes: Vec<Mesh>,
    pub images: Vec<ImageBlob>,
}

use nalgebra;
use nalgebra::{Eye, Isometry3, Matrix4, PerspectiveMatrix3, ToHomogeneous, Vector3};

#[derive(Debug)]
pub struct Camera {
    pub modelview_matrix: RefCell<[[f32; 4]; 4]>,
    pub projection_matrix: RefCell<[[f32; 4]; 4]>,
    pub position: RefCell<Vector3<f32>>,
    pub direction: RefCell<Vector3<f32>>,
    pub right: RefCell<Vector3<f32>>,
    pub up: RefCell<Vector3<f32>>,
    pub horizontal_angle: RefCell<f64>,
    pub vertical_angle: RefCell<f64>,
}

use std::f64::consts::{FRAC_PI_2};

impl Camera {
    pub fn new(screen_width: f32, screen_height: f32) -> Camera {
        // Set up camera
        let projection_matrix =
            PerspectiveMatrix3::new(screen_width / screen_height, 45.0, 0.1, 1000.0);
        let projection_matrix: Matrix4<f32> = projection_matrix.to_matrix();

		let matrix: Matrix4<f32> = Eye::new_identity(4);
        let modelview_matrix_array = *matrix.as_ref();

        let camera: Camera = Camera {
            modelview_matrix: RefCell::new(modelview_matrix_array),
            projection_matrix: RefCell::new(*projection_matrix.as_ref()),
            position: RefCell::new(Vector3::new(0.0f32, 1.0f32, 0.0f32)),
            direction: RefCell::new(Vector3::new(0.0f32, 0.0f32, 1.0f32)),
            right: RefCell::new(Vector3::new(1.0f32, 0.0f32, 0.0f32)),
            up: RefCell::new(Vector3::new(0.0f32, 1.0f32, 0.0f32)),
            horizontal_angle: RefCell::new(0.0),
            vertical_angle: RefCell::new(0.0),
        };
        camera.aim(0.0, 0.0);
        camera.update();
        return camera;
    }
    
    pub fn aim(&self, x: f64, y: f64) {
    	let factor = 0.01;
    	let mut horizontal = *self.horizontal_angle.borrow();
    	let mut vertical = *self.vertical_angle.borrow();
    	horizontal += x * factor;
    	vertical += y * factor;
    	
    	let mut direction: Vector3<f32> = *self.direction.borrow();
    	direction.x = (vertical.cos() * horizontal.sin()) as f32;
    	direction.y = vertical.sin() as f32;
    	direction.z = (vertical.cos() * horizontal.cos()) as f32;
    	
    	let mut right: Vector3<f32> = *self.right.borrow();
    	right.x = (horizontal - FRAC_PI_2).sin() as f32;
    	right.y = 0.0f32;
    	right.z = (horizontal - FRAC_PI_2).cos() as f32;
    	
    	*self.horizontal_angle.borrow_mut() = horizontal;
    	*self.vertical_angle.borrow_mut() = vertical;
    	*self.direction.borrow_mut() = direction;
    	*self.right.borrow_mut() = right;
    	*self.up.borrow_mut() = nalgebra::cross(&right, &direction);
    }
    
    pub fn move_backward(&self, amount: f32) {
		self.move_forward(-1.0 * amount);
    }
    
    pub fn move_forward(&self, amount: f32) {
    	let direction: Vector3<f32> = *self.direction.borrow();
    	let position: Vector3<f32> = *self.position.borrow();
    	let scaled_direction: Vector3<f32> = amount * direction;
    	*self.position.borrow_mut() = position + scaled_direction;
    }
    
    pub fn move_left(&self, amount: f32) {
    	self.move_right(-1.0 * amount);
    }
    
    pub fn move_right(&self , amount: f32) {
    	let right: Vector3<f32> = *self.right.borrow();
    	let position: Vector3<f32> = *self.position.borrow();
    	let scaled_right: Vector3<f32> = amount * right;
    	*self.position.borrow_mut() = position + scaled_right;
    }
    
    pub fn update(&self) {
    	let position: Vector3<f32> = *self.position.borrow();
    	let direction: Vector3<f32> = *self.direction.borrow();
    	let up: Vector3<f32> = *self.up.borrow();
    	
    	let iso3 = Isometry3::look_at_rh(
    		&position.to_point(), &(position + direction).to_point(), &up
    	);
    	let matrix: Matrix4<f32> = iso3.to_homogeneous();
    	let arr: [[f32; 4]; 4] = *matrix.as_ref();
    	*self.modelview_matrix.borrow_mut() = arr;
    }
}
