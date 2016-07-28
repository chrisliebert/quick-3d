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
    pub matrix: RefCell<[[f32; 4]; 4]>,
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


