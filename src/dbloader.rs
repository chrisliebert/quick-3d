// Copyright(C) 2016 Chris Liebert

extern crate time;
extern crate libc;

#[cfg(feature = "sqlite")]
extern crate rusqlite;

#[cfg(feature = "sqlite")]
use std::ffi::CStr;
#[cfg(feature = "sqlite")]
use std::io::Error;
#[cfg(feature = "sqlite")]
use scene::Scene;
#[cfg(feature = "sqlite")]use shader::Shader;
#[cfg(feature = "sqlite")]use std::path::Path;
#[cfg(feature = "sqlite")]
use self::rusqlite::Connection;/// A struct representing an SQLite database loader
pub struct DBLoader {
    filename: String,
}

#[cfg(feature = "sqlite")]
#[derive(Debug)]
pub enum DBLoaderError {
    IoError(Error),
    DBError(self::rusqlite::Error),
}

/// A node to index geometric data loaded from SQLite
///
/// A scene node is used to index data that is loaded from the vertex table
/// of an SQLite database.
///
pub struct SceneNodeTableRow {
    pub name: String,
    pub material_index: usize,
    pub start_position: i32,
    pub end_position: i32,
    pub radius: f64,
    pub center_x: f64,
    pub center_y: f64,
    pub center_z: f64,
}

#[cfg(feature = "sqlite")]
impl DBLoader {
    /// Create a new DBLoader object
    pub fn new(filename: &str) -> Result<DBLoader, DBLoaderError> {
        use std::io::Error;
        use std::io::ErrorKind;
        use std::path::Path;
        if !Path::new(filename.clone()).exists() {
            return Err(DBLoaderError::IoError(Error::new(ErrorKind::NotFound, format!("Unable to load {}", filename) )));
        }
        Ok(DBLoader { filename: String::from(filename) })
    }

    /// Load the contents of an SQLite datbase into a `Scene` data structure.
    pub fn load_scene(&self) -> Result<Scene, DBLoaderError> {
        use self::rusqlite::Connection;
        use nalgebra::{Eye, Matrix4};
        use std::cell::RefCell;
        use std::path::Path;
        use common::{ImageBlob, Material, Mesh, Vertex8f32};
        use scene::Scene;
        let conn = try!(
            Connection::open(Path::new(&self.filename))
                .map_err(DBLoaderError::DBError)
        );

        // Load vertices
        let mut vertex_stmt = try!(
            conn.prepare("SELECT px, py, pz, nx, ny, nz, tu, tv FROM vertex")
               .map_err(DBLoaderError::DBError)
        );
        let vertex_iter = try!(
            vertex_stmt.query_map(&[], |row| {
                Vertex8f32::from_f64(row.get(0),
                                     row.get(1),
                                     row.get(2),
                                     row.get(3),
                                     row.get(4),
                                     row.get(5),
                                     row.get(6),
                                     row.get(7))
            }).map_err(DBLoaderError::DBError)
        );

        let mut vertices: Vec<Vertex8f32> = Vec::new();

        for vertex in vertex_iter {
            vertices.push(try!(vertex.map_err(DBLoaderError::DBError)));
        }
        
        if vertices.len() == 0 {
            panic!("No vertices defined in database");
        }

        // Load materials
        let mut material_stmt = try!(
            conn.prepare("SELECT name, diffuse_r, diffuse_g, diffuse_b, diffuse_texname FROM \
                          material").map_err(DBLoaderError::DBError)
        );
        let material_iter = try!(
            material_stmt.query_map(&[], |row| {
                let diffuse_r: f64 = row.get(1);
                let diffuse_g: f64 = row.get(2);
                let diffuse_b: f64 = row.get(3);
                Material {
                    name: row.get(0),
                    diffuse: [diffuse_r as f32, diffuse_g as f32, diffuse_b as f32],
                    diffuse_texname: row.get(4),
                }
            }).map_err(DBLoaderError::DBError)
        );

        let mut materials: Vec<Material> = Vec::new();

        for material in material_iter {
            materials.push(try!(material.map_err(DBLoaderError::DBError)));
        }

        let mut meshes: Vec<Mesh> = Vec::new();

        // Load scene nodes
        let mut scene_node_stmt = try!(
            conn.prepare("SELECT name, material_id, start_position, end_position, radius, center_x, center_y, center_z FROM scene_node")
                .map_err(DBLoaderError::DBError)
        );
        let scene_node_iter = try!(
            scene_node_stmt.query_map(&[], |row| {
                let material_id: i32 = row.get(1);
                let material_index: usize = material_id as usize - 1 as usize;
                SceneNodeTableRow {
                    name: row.get(0),
                    material_index: material_index, // index starts at 1 in database, not 0
                    start_position: row.get(2),
                    end_position: row.get(3),
                    radius: row.get(4),
                    center_x: row.get(5),
                    center_y: row.get(6),
                    center_z: row.get(7),
                }
            })
            .map_err(DBLoaderError::DBError)
        );

        for scene_node in scene_node_iter {
            let sn = try!(scene_node.map_err(DBLoaderError::DBError));
            let mut new_vertices: Vec<Vertex8f32> = Vec::new();

            for i in sn.start_position as usize..(sn.end_position) as usize {
                new_vertices.push(vertices[i]);
            }

            if sn.material_index >= materials.len() {
                panic!("Material index {} out of bounds", sn.material_index);
            }

            let identity: Matrix4<f32> = Eye::new_identity(4);

            let mesh = Mesh {
                name: sn.name,
                material_index: sn.material_index,
                vertices: new_vertices,
                radius: sn.radius as f32,
                center: [sn.center_x as f32, sn.center_y as f32, sn.center_z as f32],
                matrix: RefCell::new(identity),
            };
            meshes.push(mesh);
        }

        println!("Loaded {} vertices in {} meshes",
                 vertices.len(),
                 meshes.len());

        // Free up some memory
        vertices.clear();

        // Load textures
        let mut texture_stmt = try!(
            conn.prepare("SELECT name, image FROM texture")
                .map_err(DBLoaderError::DBError)
        );
        let texture_iter = try!(
            texture_stmt.query_map(&[], |row| {
                ImageBlob {
                    name: row.get(0),
                    image: row.get(1),
                }
            }).map_err(DBLoaderError::DBError)
        );
        let mut textures: Vec<ImageBlob> = Vec::new();

        for texture in texture_iter {
            textures.push(try!(texture.map_err(DBLoaderError::DBError)));
        }

        Ok(Scene {
            materials: materials,
            meshes: meshes,
            images: textures,
        })
    }

    /// Load a shader from an SQLite database
    pub fn load_shader(&self, name: &str, glsl_version_string: &str) -> Result<Shader, DBLoaderError> {
        let conn = try!(
            Connection::open(Path::new(&self.filename))
                .map_err(DBLoaderError::DBError)
        );
        let mut id_sql: String = "SELECT id FROM shader WHERE name = '".to_owned();
        id_sql.push_str(name);
        id_sql.push('\'');

        let shader_id: i32 = try!(
            conn.query_row(&id_sql, &[], |row| row.get(0))
                .map_err(DBLoaderError::DBError)
        );

        let mut base_query: String = "SELECT source FROM shader_version WHERE shader_id="
            .to_owned();
        let shader_id_str: String = shader_id.to_string();
        base_query.push_str(&shader_id_str);
        base_query.push_str(" AND version = ");
        base_query.push_str(&glsl_version_string);
        base_query.push_str(" AND type = '");

        let mut vertex_source_sql: String = base_query.clone();
        let mut fragment_source_sql: String = base_query;

        vertex_source_sql.push_str("vertex';");
        fragment_source_sql.push_str("fragment';");

        let vertex_source: String = try!(
            conn.query_row(&vertex_source_sql, &[], |row| row.get(0))
                .map_err(DBLoaderError::DBError)
        );
        let fragment_source: String = try!(
            conn.query_row(&fragment_source_sql, &[], |row| row.get(0))
                .map_err(DBLoaderError::DBError)
        );
        
        Ok(Shader {
            name: String::from(name),
            vertex_source: vertex_source,
            fragment_source: fragment_source,
        })
    }
}

/// `extern void free_db_loader(DBLoader dbloader);`
///
#[cfg(feature = "sqlite")]
#[no_mangle]
pub extern "C" fn free_db_loader(ptr: *mut DBLoader) {
    let box_ptr: Box<DBLoader> = unsafe { Box::from_raw(ptr) };
    drop(box_ptr)
}

/// `extern void free_db_loader(DBLoader dbloader);`
///
#[cfg(not(feature = "sqlite"))]
#[no_mangle]
pub extern "C" fn free_db_loader(_ptr: *mut libc::c_void) {
	panic!("The SQLite feature is not enabled");
}

/// `extern DBLoader create_db_loader(const char* filename);`
///
#[cfg(feature = "sqlite")]
#[no_mangle]
pub extern "C" fn create_db_loader(filename_cstr: *const libc::c_char) -> Box<DBLoader> {
    unsafe {
        let filename: String = CStr::from_ptr(filename_cstr).to_string_lossy().into_owned();
        let dbloader: DBLoader = DBLoader::new(&filename).expect("Unable to create database loader");
        println!("Loaded {}", &filename);
        Box::new(dbloader)
    }
}

/// `extern DBLoader create_db_loader(const char* filename);`
///
#[cfg(not(feature = "sqlite"))]
#[no_mangle]
pub extern "C" fn create_db_loader(_filename_cstr: *const libc::c_char) -> Box<libc::c_void> {
    panic!("The SQLite feature is not enabled");
}