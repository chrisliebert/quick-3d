// Copyright(C) 2016 Chris Liebert

/// A representation of a binary image and it's name
///
/// An `ImageBlob` represents a row in the `texture` table of an SQL database.
///
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
#[derive(Clone)]
pub struct Mesh {
    pub name: String,
    pub vertices: Vec<Vertex8f32>,
    pub material_index: usize,
    pub matrix: RefCell<Matrix4<f32>>,
}

/// `Material`
///
/// Material properties from that can be passed as uniforms
/// to `Shader` programs.
///
#[derive(Debug)]
#[derive(Clone)]
pub struct Material {
    pub name: String,
    pub diffuse: [f32; 3],
    pub diffuse_texname: String,
}

/// Geometry and material information that can be rendered
///
/// A `Scene` contains geometry that will be rendered along with reference materials and
/// textures
///
pub struct Scene {
    pub materials: Vec<Material>,
    pub meshes: Vec<Mesh>,
    pub images: Vec<ImageBlob>,
}

/// A representation for a GPU program
///
/// Shader programs contain the source for a vertex and fragment shader
/// in addition to a name
///
#[derive(Debug)]
pub struct Shader {
    pub name: String,
    pub vertex_source: String,
    pub fragment_source: String,
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
#[derive(Copy, Clone)]
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

use std::cell::RefCell;

use nalgebra;
use nalgebra::{Eye, Isometry3, Matrix4, PerspectiveMatrix3, ToHomogeneous, Vector3};

/// `Camera`
///
/// Data for camera manipulation. Only modelview_matrix and projection_matrix are passed to the
/// shader as uniform values.
///
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

use std::f64::consts::FRAC_PI_2;

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

    /// Rotate a `Camera` in a relative direction perpendicular to the focal point.
    ///
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

    /// Move a `Camera` backward by a specified amount.
    ///
    pub fn move_backward(&self, amount: f32) {
        self.move_forward(-1.0 * amount);
    }

    /// Move a `Camera` forward by a specified amount.
    ///
    pub fn move_forward(&self, amount: f32) {
        let direction: Vector3<f32> = *self.direction.borrow();
        let position: Vector3<f32> = *self.position.borrow();
        let scaled_direction: Vector3<f32> = amount * direction;
        *self.position.borrow_mut() = position + scaled_direction;
    }

    /// Move a `Camera` left by a specified amount.
    ///
    pub fn move_left(&self, amount: f32) {
        self.move_right(-1.0 * amount);
    }

    /// Move a `Camera` right by a specified amount.
    ///
    pub fn move_right(&self, amount: f32) {
        let right: Vector3<f32> = *self.right.borrow();
        let position: Vector3<f32> = *self.position.borrow();
        let scaled_right: Vector3<f32> = amount * right;
        *self.position.borrow_mut() = position + scaled_right;
    }

    /// Once a camera is moved or aimed, the modelview matrix must be re-calculated.
    ///
    pub fn update(&self) {
        let position: Vector3<f32> = *self.position.borrow();
        let direction: Vector3<f32> = *self.direction.borrow();
        let up: Vector3<f32> = *self.up.borrow();

        let iso3 = Isometry3::look_at_rh(&position.to_point(),
                                         &(position + direction).to_point(),
                                         &up);
        let matrix: Matrix4<f32> = iso3.to_homogeneous();
        let arr: [[f32; 4]; 4] = *matrix.as_ref();
        *self.modelview_matrix.borrow_mut() = arr;
    }
}
