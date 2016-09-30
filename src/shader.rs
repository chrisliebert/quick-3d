// Copyright (C) 2016 Chris Liebert

extern crate libc;

use std::ffi::CStr;
use std::io::Error;
use std::io::ErrorKind;
use std::num::ParseIntError;

use glium;
use glium::backend::glutin_backend::GlutinFacade;
use glium::{Program, Version};

use dbloader::{DBLoader, DBLoaderError};

/// A representation for a GPU program
///
/// Shader programs contain the source for a vertex and fragment shader
/// in addition to a name
///
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Shader {
    pub name: String,
    pub vertex_source: String,
    pub fragment_source: String,
}

#[derive(Debug)]
pub enum ShaderError {
    IoError(Error),
    ProgramChooserCreationError(glium::program::ProgramChooserCreationError),
    InvalidGLSLVersionStringError(ParseIntError),
    UnsupportedGLSLVersionError(Error),
}

impl Shader {
    /// Create a `glium::program::Program` object from a `DBLoader` that contains the shader and shader_version tables in SQLite
    ///
    #[allow(unused_assignments)]
    pub fn from_dbloader(shader_name: &str, dbloader: &DBLoader, display: &GlutinFacade) -> Result<glium::program::Program, ShaderError> {
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
            Err(e) => return Err(ShaderError::InvalidGLSLVersionStringError(e)),
        };
        
        println!("Using glsl version {}", &glsl_version_string);
    
        let shader: Shader = match dbloader.load_shader(shader_name, &glsl_version_string) {
            Ok(s) => s,
            Err(DBLoaderError::DBError(e)) => {
                let description = format!("{:?}", e);
                return Err(ShaderError::IoError(Error::new(ErrorKind::InvalidData, description)));
            },
            Err(e) => {
                let description = format!("{:?}", e);
                return Err(ShaderError::IoError(Error::new(ErrorKind::InvalidData, description)));
            },
        };
    
        let program: Program = match use_gles {
            true => try!(
                program!(display,
                    glsl_version_number es => {
                        vertex: &shader.vertex_source,
                        fragment: &shader.fragment_source,
                    }
                ).map_err(ShaderError::ProgramChooserCreationError)
            ),
            false => try!(
                program!(display,
                    glsl_version_number => {
                        vertex: &shader.vertex_source,
                        fragment: &shader.fragment_source,
                    }
                ).map_err(ShaderError::ProgramChooserCreationError)
            ),
        };
            
        // Shader compilation no longer required
        display.release_shader_compiler();
        return Ok(program);
    }
    
    /// Create a `glium::program::Program` object from a `DBLoader` that has a specific shader version
    ///
    pub fn from_dbloader_with_version(shader_name: &str, dbloader: &DBLoader, glsl_version: &Version, display: &GlutinFacade) -> Result<glium::program::Program, ShaderError> {
        if !display.is_glsl_version_supported(&glsl_version) {
            return Err(ShaderError::UnsupportedGLSLVersionError(Error::new(ErrorKind::InvalidData, "Unsupported GLSL version")));
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
    
        let shader: Shader = match dbloader.load_shader(shader_name, &glsl_version_string) {
            Ok(s) => s,
            Err(DBLoaderError::DBError(e)) => {
                let description = format!("{:?}", e);
                return Err(ShaderError::IoError(Error::new(ErrorKind::InvalidData, description)));
            },
            Err(e) => {
                let description = format!("{:?}", e);
                return Err(ShaderError::IoError(Error::new(ErrorKind::InvalidData, description)));
            },
        };
    
        Ok(match _use_gles {
            true => try!(
                program!(display,
                    glsl_version_number es => {
                        vertex: &shader.vertex_source,
                        fragment: &shader.fragment_source,
                    }
                ).map_err(ShaderError::ProgramChooserCreationError)
            ),
            false => try!(
                program!(display,
                    glsl_version_number => {
                        vertex: &shader.vertex_source,
                        fragment: &shader.fragment_source,
                    }
                ).map_err(ShaderError::ProgramChooserCreationError)
            ),
        })
    }
}

/// `extern void free_shader(Shader shader);`
///
#[no_mangle]
pub extern "C" fn free_shader(ptr: *mut glium::program::Program) {
    let box_ptr: Box<glium::program::Program> = unsafe { Box::from_raw(ptr) };
    drop(box_ptr)
}

/// `extern Shader get_shader_from_db_loader(const char* name, DBLoader dbloader, Renderer renderer, Display display);`
///
#[no_mangle]
pub extern "C" fn get_shader_from_dbloader(shader_name_cstr: *const libc::c_char,
                                            dbloader: &DBLoader,
                                            display: &GlutinFacade)
                                            -> Box<glium::program::Program> {
    unsafe {
        let shader_name: String = CStr::from_ptr(shader_name_cstr).to_string_lossy().into_owned();
        let shader_program = Shader::from_dbloader(&shader_name, dbloader, display).unwrap();
        return Box::new(shader_program);
    }
}