// Copyright(C) 2016 Chris Liebert
extern crate glium;
extern crate image;
extern crate nalgebra;

use std::collections::HashMap;
use std::io::Error;
use std::io::ErrorKind;

use camera::Camera;
use common;
use common::{Mesh, Scene, Shader, Vertex8f32};
use loader::DBLoader;

use glium::backend::glutin_backend::GlutinFacade;
use glium::{Program, Surface, Version};

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
    
    /// Create a `glium::program::Program` object from a `DBLoader` that contains the shader and shader_version tables in SQLite
    ///
    #[allow(unused_assignments)]
    pub fn create_shader_program(&self, shader_name: &str, dbloader: &DBLoader, display: &GlutinFacade) -> glium::program::Program {
        let supported_glsl_version: Version = display.get_supported_glsl_version();
        let api: glium::Api = supported_glsl_version.0;
        let major_version : u8 = supported_glsl_version.1;
        let minor_version : u8 = supported_glsl_version.2;
        let mut glsl_version_string: String = major_version.to_string();
        glsl_version_string.push_str(&minor_version.to_string());
        glsl_version_string.push('0');
        let mut use_gles: bool = false;// Use OpenGLES instead of OpenGL (for mobile devices)
        
        match api {
                glium::Api::Gl => {
                    use_gles = false;
                },
                glium::Api::GlEs => {
                    use_gles = true;
                    glsl_version_string.push_str(" es");
                },
        };
        
        let glsl_version_number: u32 = match glsl_version_string.parse() {
            Ok(s) => s,
            Err(e) =>  {
                // This is not likely to happen
                panic!("Unable to parse supported glsl version string: {}", e);
            },
        };
        
        println!("Using glsl version {}", &glsl_version_string);
    
        let shader: Shader = dbloader.load_shader(shader_name, &glsl_version_string);
    
        let program: Program;
        
        if use_gles {
            program = program!(display,
                glsl_version_number es => {
                    vertex: &shader.vertex_source,
                    fragment: &shader.fragment_source,
                },
            )
            .unwrap();
        } else {
            program = program!(display,
                glsl_version_number => {
                    vertex: &shader.vertex_source,
                    fragment: &shader.fragment_source,
                },
            )
            .unwrap();
        }
            
        // Shader compilation no longer required
        display.release_shader_compiler();
        return program;
    }
    
    /// Create a `glium::program::Program` object from a `DBLoader` that has a specific shader version
    ///
    pub fn create_shader_program_with_version(&self, shader_name: &str, dbloader: &DBLoader, glsl_version: &Version, display: &GlutinFacade) -> Result<glium::program::Program, Error> {
        if !display.is_glsl_version_supported(&glsl_version) {
            return Err(Error::new(ErrorKind::InvalidData, "Unsupported GLSL version"));
        }
        
        let api: glium::Api = glsl_version.0;
        let major_version : u8 = glsl_version.1;
        let minor_version : u8 = glsl_version.2;
        let mut glsl_version_string: String = major_version.to_string();
        glsl_version_string.push_str(&minor_version.to_string());
        glsl_version_string.push('0');
        let mut _use_gles: bool = false;// Use OpenGLES instead of OpenGL (for mobile devices)
        
        match api {
                glium::Api::Gl => {
                    _use_gles = false;
                },
                glium::Api::GlEs => {
                    _use_gles = true;
                    glsl_version_string.push_str(" es");
                },
        };
        
        let glsl_version_number: u32 = match glsl_version_string.parse() {
            Ok(s) => s,
            Err(e) =>  {
                // This is not likely to happen
                panic!("Unable to parse supported glsl version string: {}", e);
            },
        };
        
        println!("Using glsl version {}", &glsl_version_string);
    
        let shader: Shader = dbloader.load_shader(shader_name, &glsl_version_string);
    
        let program;
        
        if _use_gles {
            program = program!(display,
                glsl_version_number es => {
                    vertex: &shader.vertex_source,
                    fragment: &shader.fragment_source,
                },
            )
        } else {
            program = program!(display,
                glsl_version_number => {
                    vertex: &shader.vertex_source,
                    fragment: &shader.fragment_source,
                },
            )
        }
        
        match program {
            Ok(p) => Ok(p),
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
        }
    }
    
    /// Draw the `Scene` data consumed by self to the display
    ///
    pub fn render(&self, display: &GlutinFacade, program: &glium::program::Program, camera: &Camera) {
        let mut target = display.draw();
        // TODO: generate this texture instead of loading from sqlite
        let default_blank_texture = &self.textures["DEFAULT_BLANK_TEXTURE.png"];
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        for i in 0..self.vertex_buffers.len() as usize {
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
                        println!("Unable to load {}, using blank texture instead.", diffuse_texname);
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
                model: *(*self.scene.meshes[i].matrix.borrow()).as_ref(),
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
        target.finish().unwrap();
    }
}
