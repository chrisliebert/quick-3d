// Copyright(C) 2016 Chris Liebert

use std::cell::RefCell;
use nalgebra::Matrix4;

/// A representation of a binary image and it's name
///
/// An `ImageBlob` represents a row in the `texture` table of an SQL database.
///
#[derive(PartialEq, RustcEncodable, RustcDecodable)]
pub struct ImageBlob {
    pub name: String,
    pub image: Vec<u8>,
}

/// A representation a piece geometry and it's material
///
/// A `Mesh` contains vertices that share the same material and a mutable matrix
/// is used to track the position and orientation each `Mesh`. The matrix is passed
/// to the shader program as a uniform.
///
#[derive(Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Mesh {
    pub name: String,
    pub vertices: Vec<Vertex8f32>,
    pub material_index: usize,
    pub radius: f32,
    pub center: [f32; 3],
    pub matrix: RefCell<Matrix4<f32>>,
}

/// `Material`
///
/// Material properties from that can be passed as uniforms
/// to `Shader` programs.
///
#[derive(Clone, Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Material {
    pub name: String,
    pub diffuse: [f32; 3],
    pub diffuse_texname: String,
}
	
/// `Vertex8f32` - The default implementation of a vertex which is buffered to
/// the graphics processing unit.
///
/// `Vertex8f32` consists of a `position` attribute
/// which contains x, y and z coordinates, a `normal` attribute and a `texcoord`
/// attribute. The `normal` contains the coordinates for a unit vector that is
/// perpendicular to the surface of the vertex. The `texcoord` attribute
/// represents the texture coordinates which describe how a 2D texture is mapped
/// onto the 3D geometry.
///
#[derive(Copy, Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Vertex8f32 {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub texcoord: [f32; 2],
}

impl Vertex8f32 {
    /// Create a `Vertex3f32` from f64 values
    ///
    pub fn from_f64(px: f64,
                    py: f64,
                    pz: f64,
                    nx: f64,
                    ny: f64,
                    nz: f64,
                    tu: f64,
                    tv: f64)
                    -> Vertex8f32 {
        let position: [f32; 3] = [px as f32, py as f32, pz as f32];
        let normal: [f32; 3] = [nx as f32, ny as f32, nz as f32];
        let texcoord: [f32; 2] = [tu as f32, tv as f32];
        Vertex8f32 {
            position: position,
            normal: normal,
            texcoord: texcoord,
        }
    }
}

