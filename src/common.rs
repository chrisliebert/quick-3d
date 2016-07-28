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

#[derive(Debug)]
pub struct Camera {
    pub modelview_matrix: RefCell<[[f32; 4]; 4]>,
    pub projection_matrix: RefCell<[[f32; 4]; 4]>,
}

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

use nalgebra::{Matrix4, PerspectiveMatrix3};

impl Camera {
    pub fn new(screen_width: f32, screen_height: f32) -> Camera {
        // Set up camera
        let projection_matrix =
            PerspectiveMatrix3::new(screen_width / screen_height, 45.0, 0.1, 1000.0);
        let projection_matrix: Matrix4<f32> = projection_matrix.to_matrix();

        let modelview_matrix_array = [[1.0f32, -0.0f32, -0.0f32, 0.0f32],
                                      [0.0f32, 1.0f32, -0.0f32, 0.0f32],
                                      [0.0f32, 0.0f32, 1.0f32, 0.0f32],
                                      [-5.0f32, -3.0f32, -12.0f32, 1.0f32]];

        Camera {
            modelview_matrix: RefCell::new(modelview_matrix_array),
            projection_matrix: RefCell::new(*projection_matrix.as_ref()),
        }
    }
}
