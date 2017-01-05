// Copyright (C) 2016 Chris Liebert

extern crate libc;

use std::ffi::CStr;
use std::fs::File;
use std::io::Error;
use std::io::prelude::*;
use std::num::ParseIntError;

use glium;
use glium::backend::glutin_backend::GlutinFacade;
use glium::Program;

#[cfg(feature = "sqlite")]
use glium::Version;

#[cfg(feature = "sqlite")]
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
    ProgramCreationError(glium::ProgramCreationError),
    InvalidGLSLVersionStringError(ParseIntError),
    UnsupportedGLSLVersionError(Error),
}

impl Shader {
    /// Create a default shader with hardcoded source
    ///
    pub fn default(display: &GlutinFacade) -> Result<glium::program::Program, ShaderError> {
        Shader::from_source(r#"
#version 110

// Input data
vec3 position;
vec3 normal;
vec2 texcoord;

// Uniforms
uniform mat4 projection;
uniform mat4 modelview;
uniform mat4 model;
uniform vec3 light_position_worldspace;

// Output data
vec3 out_position;
vec3 out_normal;
vec2 out_texcoord;
vec3 camera_direction;
vec3 light_direction;
vec3 light_position;

 void main() {
	mat4 mvp = projection * modelview * model;
	out_position = (mvp * vec4(position, 1.0)).xyz;
	camera_direction = normalize(vec3(0, 0, 0) - out_position);
	light_direction = vec3(0,-1, 0);
	light_position = light_position_worldspace;
	gl_Position = mvp * vec4(position, 1.0);
	out_normal = ( mvp * vec4(normal, 0)).xyz;
	out_texcoord = texcoord;
 }
     "#, r#"
#version 110

// Interpolated values from the vertex shaders
vec3 out_position;
vec3 out_normal;
vec2 out_texcoord;
vec3 camera_direction;
vec3 light_direction;

// Ouput data
vec3 color;

// Values that stay constant for the whole mesh.
uniform sampler2D diffuse_texture;

uniform vec3 ambient;
uniform vec3 diffuse;
uniform vec3 specular;
uniform vec3 light_position;

void main(){
	vec3 light_color = vec3(1,1,1);
	float light_power = 3.8;

	// Material properties
	vec4 diffuseColor4 = vec4(0.3, 0.3, 0.3, 1.0) * ( vec4(diffuse, 1.0) + texture2D(diffuse_texture, out_texcoord));
	vec3 diffuseColor =  vec3( diffuseColor4[0], diffuseColor4[1], diffuseColor4[2]);
	
	vec3 ambientColor = ambient + vec3(0.3, 0.3, 0.3) * diffuseColor;
	vec3 specularColor = specular;

	// Distance to the light
	float distance = length(light_position - out_position);
	// Normal of the computed fragment, in camera space
	vec3 n = normalize(out_normal);
	// Direction of the light (from the fragment to the light)
	vec3 l = normalize(light_direction);
	float cosTheta = clamp(dot(n, l), 0.0, 1.0);
	vec3 E = normalize(camera_direction);
	vec3 R = reflect(-l, n);
	float cosAlpha = clamp(dot(E, R), 0.0, 1.0);

	color = 
		// Ambient : simulates indirect lighting
		ambientColor +
		// Diffuse : "color" of the object
		diffuseColor * light_color * light_power * cosTheta / (distance * distance) +
		// Specular : reflective highlight, like a mirror
		specularColor * light_color * light_power * pow(cosAlpha, 5.0) / (distance * distance);
}
"#,
		&display)
    }
    
    /// Create a `glium::program::Program` object from a `DBLoader` that contains the shader and shader_version tables in SQLite
    ///
	#[cfg(feature = "sqlite")]
    #[allow(unused_assignments)]
    pub fn from_dbloader(shader_name: &str, dbloader: &DBLoader, display: &GlutinFacade) -> Result<glium::program::Program, ShaderError> {
        use std::io::ErrorKind;
        use glium::Version;
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
	#[cfg(feature = "sqlite")]
    pub fn from_dbloader_with_version(shader_name: &str, dbloader: &DBLoader, glsl_version: &Version, display: &GlutinFacade) -> Result<glium::program::Program, ShaderError> {
        use std::io::ErrorKind;
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
    
    /// Create a `glium::program::Program` object from vertex and fragment file sources
    ///
    pub fn from_file(vertex: &str, fragment: &str, display: &GlutinFacade) -> Result<Program, ShaderError> {
        let mut vf = try!(
        	File::open(vertex).map_err(ShaderError::IoError)
        );
        let mut vertex_source = String::new();
        try!(
        	vf.read_to_string(&mut vertex_source).map_err(ShaderError::IoError)
        );
        let mut ff = try!(
        	File::open(fragment).map_err(ShaderError::IoError)
        );
        let mut fragment_source = String::new();
        try!(
        	ff.read_to_string(&mut fragment_source).map_err(ShaderError::IoError)
        );
        let program = try!(
            Program::from_source(display, &vertex_source, &fragment_source, None)
                .map_err(ShaderError::ProgramCreationError)
        );
        Ok(program)
    }
    
    /// Create a `glium::program::Program` object from vertex and fragment sources
    ///
    pub fn from_source(vertex_source: &str, fragment_source: &str, display: &GlutinFacade) -> Result<Program, ShaderError> {
        let program = try!(
            Program::from_source(display, &vertex_source, &fragment_source, None)
                .map_err(ShaderError::ProgramCreationError)
        );
        Ok(program)
    }
}

/// `extern Shader shader_default(Display display);`
///
#[no_mangle]
pub extern "C" fn shader_default(display: &GlutinFacade)
                                        -> Box<glium::program::Program> {
    match Shader::default(display) {
        Ok(s) => Box::new(s),
        Err(e) => panic!("Unable to load default shader: {:?}", e),
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
#[cfg(feature = "sqlite")]
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
                                            
/// `extern Shader get_shader_from_db_loader(const char* name, DBLoader dbloader, Renderer renderer, Display display);`
///
#[cfg(not(feature = "sqlite"))]
#[no_mangle]
pub extern "C" fn get_shader_from_dbloader(_shader_name_cstr: *const libc::c_char,
                                            _dbloader: *const libc::c_void,
                                            _display: &GlutinFacade)
                                            -> Box<glium::program::Program> {
    panic!("The SQLite feature is not enabled");
}
                                      
/// `extern Shader get_shader_from_source(const char* vertex, const char* fragment, Display display);`
///
#[no_mangle]
pub extern "C" fn get_shader_from_source(vertex_cstr: *const libc::c_char,
                                         fragment_cstr: *const libc::c_char,
                                         display: &GlutinFacade) -> Box<glium::program::Program> {
    unsafe {
        let vertex: String = CStr::from_ptr(vertex_cstr).to_string_lossy().into_owned();
        let fragment: String = CStr::from_ptr(fragment_cstr).to_string_lossy().into_owned();
        let shader_program = match Shader::from_source(&vertex, &fragment, display) {
	        Ok(s) => s,
	        Err(e) => panic!("Unable to create shader program: {:?}", e),
        };
        return Box::new(shader_program);
    }
}
                                         
/// `extern Shader is_shader_source_valid(const char* vertex, const char* fragment, Display display);`
///
#[no_mangle]
pub extern "C" fn shader_source_is_valid(vertex_cstr: *const libc::c_char,
                                         fragment_cstr: *const libc::c_char,
                                         display: &GlutinFacade) -> bool {
    unsafe {
        let vertex: String = CStr::from_ptr(vertex_cstr).to_string_lossy().into_owned();
        let fragment: String = CStr::from_ptr(fragment_cstr).to_string_lossy().into_owned();
        match Shader::from_source(&vertex, &fragment, display) {
	        Ok(s) =>  {
	            drop(s);
	            true
	        },
	        Err(_e) => {
	        	false
	        },
        }
    }
}