// Copyright(C) 2016 Chris Liebert
extern crate rusqlite;
extern crate time;

use self::rusqlite::Connection;
use nalgebra::{Eye, Matrix4};

use std::cell::RefCell;
use std::path::Path;

use common::{ImageBlob, Material, Mesh, Scene, Shader, Vertex8f32};


/// A struct representing an SQLite database loader
pub struct DBLoader {
    filename: String,
}

/// A node to index geometric data loaded from SQLite
///
/// A scene node is used to index data that is loaded from the vertex table
/// of an SQLite database.
///
pub struct SceneNode {
    pub name: String,
    pub material_index: usize,
    pub start_position: i32,
    pub end_position: i32,
}

impl DBLoader {
    /// Create a new DBLoader object
    pub fn new(filename: &str) -> DBLoader {
        DBLoader { filename: String::from(filename) }
    }

    /// Load the contents of an SQLite datbase into a `Scene` data structure.
    pub fn load_scene(&self) -> Scene {
        let conn = Connection::open(Path::new(&self.filename)).unwrap();

        // Load vertices
        let mut vertex_stmt = conn.prepare("SELECT px, py, pz, nx, ny, nz, tu, tv FROM vertex")
            .unwrap();
        let vertex_iter = vertex_stmt.query_map(&[], |row| {
                Vertex8f32::from_f64(row.get(0),
                                     row.get(1),
                                     row.get(2),
                                     row.get(3),
                                     row.get(4),
                                     row.get(5),
                                     row.get(6),
                                     row.get(7))
            })
            .unwrap();

        let mut vertices: Vec<Vertex8f32> = Vec::new();

        for vertex in vertex_iter {
            vertices.push(vertex.unwrap());
        }

        // Load materials
        let mut material_stmt =
            conn.prepare("SELECT name, diffuse_r, diffuse_g, diffuse_b, diffuse_texname FROM \
                          material")
                .unwrap();
        let material_iter = material_stmt.query_map(&[], |row| {
                let diffuse_r: f64 = row.get(1);
                let diffuse_g: f64 = row.get(2);
                let diffuse_b: f64 = row.get(3);
                Material {
                    name: row.get(0),
                    diffuse: [diffuse_r as f32, diffuse_g as f32, diffuse_b as f32],
                    diffuse_texname: row.get(4),
                }
            })
            .unwrap();

        let mut materials: Vec<Material> = Vec::new();

        for material in material_iter {
            materials.push(material.unwrap());
        }

        let mut meshes: Vec<Mesh> = Vec::new();

        // Load scene nodes
        let mut scene_node_stmt =
            conn.prepare("SELECT name, material_id, start_position, end_position FROM scene_node")
                .unwrap();
        let scene_node_iter = scene_node_stmt.query_map(&[], |row| {
                let material_id: i32 = row.get(1);
                let material_index: usize = material_id as usize - 1 as usize;
                SceneNode {
                    name: row.get(0),
                    material_index: material_index, // index starts at 1 in database, not 0
                    start_position: row.get(2),
                    end_position: row.get(3),
                }
            })
            .unwrap();

        for scene_node in scene_node_iter {
            let sn = scene_node.unwrap();
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
        let mut texture_stmt = conn.prepare("SELECT name, image FROM texture")
            .unwrap();
        let texture_iter = texture_stmt.query_map(&[], |row| {
                ImageBlob {
                    name: row.get(0),
                    image: row.get(1),
                }
            })
            .unwrap();

        let mut textures: Vec<ImageBlob> = Vec::new();

        for texture in texture_iter {
            textures.push(texture.unwrap());
        }

        // conn.close();
        return Scene {
            materials: materials,
            // scene_nodes: scene_nodes,
            // vertices: vertices,
            meshes: meshes,
            // shaders: shaders,
            images: textures,
        };
    }

    /// Load a shader from an SQLite database
    pub fn load_shader(&self, name: &str, glsl_version_string: &str) -> Shader {
        let conn = Connection::open(Path::new(&self.filename)).unwrap();
        let mut id_sql: String = "SELECT id FROM shader WHERE name = '".to_owned();
        id_sql.push_str(name);
        id_sql.push('\'');

        let shader_id: i32 = conn.query_row(&id_sql, &[], |row| row.get(0))
            .unwrap();

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

        let vertex_source: String = conn.query_row(&vertex_source_sql, &[], |row| row.get(0))
            .unwrap();

        let fragment_source: String = conn.query_row(&fragment_source_sql, &[], |row| row.get(0))
            .unwrap();

        Shader {
            name: String::from(name),
            vertex_source: vertex_source,
            fragment_source: fragment_source,
        }
    }
}
