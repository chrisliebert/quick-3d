// Copyright(C) 2016 Chris Liebert

extern crate glium;
extern crate image;
extern crate libc;
extern crate nalgebra;

use std::collections::HashMap;
use std::ffi::CStr;
use std::io::Error;
use std::io::ErrorKind;

use camera::Camera;
use common;
use common::{Mesh,Vertex8f32};
use dbloader::DBLoader;
use frustum::Frustum;
use scene::Scene;

use glium::backend::glutin_backend::GlutinFacade;
use glium::Surface;

use nalgebra::Vector4;

implement_vertex!(Vertex8f32, position, normal, texcoord);

/// A representation of the Glium data needed for rendering
///
pub struct Renderer {
    pub index_buffer: glium::index::NoIndices,
    pub scene: Scene,
    pub textures: HashMap<String, glium::texture::CompressedSrgbTexture2d>,
    pub vertex_buffers: Vec<glium::vertex::VertexBuffer<common::Vertex8f32>>,
}

impl Renderer {
    /// Create a new renderer from a `Scene` struct
    ///
    pub fn new(display: &GlutinFacade, scene: Scene) -> Renderer {
    
        let mut vertex_buffers: Vec<glium::vertex::VertexBuffer<common::Vertex8f32>> =
            Vec::with_capacity(scene.meshes.len());
        
        for i in 0..scene.meshes.len() {
            vertex_buffers.push(glium::vertex::VertexBuffer::new(display, &scene.meshes[i].vertices).unwrap());
        }
        
        let index_buffer: glium::index::NoIndices =
            glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        
        let mut textures: HashMap<String, glium::texture::CompressedSrgbTexture2d> = HashMap::new();
        
        for i in 0..scene.images.len() {
            //TODO: determine image format by name extension if nessisary
            //let image = image::load_from_memory_with_format(&scene.images[i].image, image::PNG).unwrap().to_rgba();
            let image = image::load_from_memory(&scene.images[i].image).unwrap().to_rgba();
            
            let image_dimensions = image.dimensions();
            let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(),
                                                                           image_dimensions);
            let opengl_texture: glium::texture::CompressedSrgbTexture2d =
                glium::texture::CompressedSrgbTexture2d::new(display, image).unwrap();
            textures.insert(scene.images[i].name.clone(), opengl_texture);        
        }
        
        Renderer { 
            index_buffer: index_buffer,
            scene: scene,
            textures: textures,
            vertex_buffers: vertex_buffers,
        }
    }
    
    /// Try to find the reference to a `Mesh` by name
    /// 
    pub fn get_mesh(&self, name: &str) -> Result<&Mesh, Error> {
        for i in 0..self.vertex_buffers.len() as usize {
            if self.scene.meshes[i].name.eq(name) {
                return Ok(&self.scene.meshes[i]);
            }
        }
        return Err(Error::new(ErrorKind::NotFound, "Unable to load mesh"));
    }
    
    /// Draw the `Scene` data consumed by self to the display
    ///
    pub fn render(&self, display: &GlutinFacade, program: &glium::program::Program, camera: &Camera) {
        
        let frustum: Frustum = Frustum::create_from_2d_array(
        	&camera.modelview_matrix,
        	&camera.projection_matrix,
        );
        
        let mut target = display.draw();
        // TODO: generate this texture instead of loading from sqlite
        let default_blank_texture = &self.textures["DEFAULT_BLANK_TEXTURE.png"];
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        for i in 0..self.vertex_buffers.len() as usize {
        	
        	let matrix = *self.scene.meshes[i].matrix.borrow();
        	let local_center: Vector4<f32> = Vector4::new(self.scene.meshes[i].center[0].clone(), self.scene.meshes[i].center[1].clone(), self.scene.meshes[i].center[2].clone(), 1.0f32);
	        let center: Vector4<f32> = matrix * local_center;
            if frustum.sphere_intersecting(&center.x, &center.y, &center.z, &self.scene.meshes[i].radius) {
	            let material_index: usize = self.scene.meshes[i].material_index.clone();
	            let diffuse = self.scene.materials[material_index].diffuse.clone();
	            let diffuse_texname: String =
	                self.scene.materials[material_index].diffuse_texname.clone();
	            let opengl_texture: &glium::texture::CompressedSrgbTexture2d;
	            if diffuse_texname.len() == 0 {
	                opengl_texture = &default_blank_texture;
	            } else {
	                match self.textures.get(&diffuse_texname) {
	                    Some(t) => {
	                        opengl_texture = &t;
	                    },
	                    None => {
	                        opengl_texture = &default_blank_texture;
	                    },
	                }
	            }
	            let uniforms: glium::uniforms::UniformsStorage<_, _> = uniform! {
	                projection: camera.projection_matrix,
	                modelview: camera.modelview_matrix,
	                light_position_worldspace: [2.0, 10.0, 1.0f32],
	                diffuse: diffuse,
	                diffuse_texture: opengl_texture,
	                model: *(matrix).as_ref(),
	            };
	            
	            let params = glium::DrawParameters {
	                depth: glium::Depth {
	                    test: glium::DepthTest::IfLess,
	                    write: true,
	                    .. Default::default()
	                },
	                .. Default::default()
	            };
	            
	            target.draw(&self.vertex_buffers[i],
	                &self.index_buffer,
	                program,
	                &uniforms,
	                &params)
	                .unwrap();
            }
        }
        target.finish().unwrap();
    }
}

/// `extern Renderer create_renderer_from_db_loader(DBLoader loader, Display display);`
///
#[no_mangle]
pub extern "C" fn create_renderer_from_db_loader(dbloader: &DBLoader,
                                                 display: &GlutinFacade)
                                                 -> Box<Renderer> {
    Box::new(Renderer::new(display, dbloader.load_scene()))
}

                                                
/// `extern Renderer create_renderer_from_compressed_binary(const char* filename, Display display);`
///
#[no_mangle]
pub extern "C" fn create_renderer_from_compressed_binary(file: *const libc::c_char,
                                                 display: &GlutinFacade)
                                                 -> Box<Renderer> {
    let filename: String = unsafe{ CStr::from_ptr(file).to_string_lossy().into_owned() };
    match Scene::from_compressed_binary_file(filename.clone()) {
        Ok(s) => {
            println!("Loaded compressed binary: {}", filename);
            return Box::new(Renderer::new(display, s));
        },
        Err(e) => panic!("Unable to load compressed binary file {}: {:?}", filename, e),
    };
}
                                                 
/// `extern Renderer create_renderer_from_binary(const char* filename, Display display);`
///
#[no_mangle]
pub extern "C" fn create_renderer_from_binary(file: *const libc::c_char,
                                                 display: &GlutinFacade)
                                                 -> Box<Renderer> {
    let filename: String = unsafe{ CStr::from_ptr(file).to_string_lossy().into_owned() };
    match Scene::from_binary_file(filename.clone()) {
        Ok(s) => {
            println!("Loaded binary: {}", filename);
            return Box::new(Renderer::new(display, s));
        },
        Err(e) => panic!("Unable to load binary file {}: {:?}", filename, e),
    };
}
                                                 
/// `extern void free_renderer(Renderer renderer);`
///
#[no_mangle]
pub extern "C" fn free_renderer(ptr: *mut Renderer) {
    let box_ptr: Box<Renderer> = unsafe { Box::from_raw(ptr) };
    drop(box_ptr)
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