// Copyright(C) 2016 Chris Liebert
extern crate rusqlite;
extern crate time;

use self::rusqlite::Connection;

use std::path::Path;

use common::{ImageBlob, Material, Mesh, SceneNode, Scene, Shader, Vertex8f32};

fn create_vertex8f32(px: f64,
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

pub fn load_db(filename: &str) -> Scene {
    println!("Loading {}", filename);

    let path: &Path = Path::new(filename);
    let conn = Connection::open(path).unwrap();

    // Load vertices
    let mut vertex_stmt = conn.prepare("SELECT px, py, pz, nx, ny, nz, tu, tv FROM vertex")
        .unwrap();
    let vertex_iter = vertex_stmt.query_map(&[], |row| {
            create_vertex8f32(row.get(0),
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
        conn.prepare("SELECT name, diffuse_r, diffuse_g, diffuse_b, diffuse_texname FROM material")
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

        if sn.material_index >= materials.len() || sn.material_index < 0 {
            panic!("Material index {} out of bounds", sn.material_index);
        }

        let mesh = Mesh {
            name: sn.name,
            material_index: sn.material_index,
            vertices: new_vertices,
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

    // Load shaders
    let mut shader_stmt =
        conn.prepare("SELECT name, vertex_source_140, fragment_source_140 FROM shader")
            .unwrap();
    let shader_iter = shader_stmt.query_map(&[], |row| {
            Shader {
                name: row.get(0),
                vertex_source_140: row.get(1),
                fragment_source_140: row.get(2),
            }
        })
        .unwrap();

    let mut shaders: Vec<Shader> = Vec::new();

    for shader in shader_iter {
        shaders.push(shader.unwrap());
    }

    return Scene {
        materials: materials,
        // scene_nodes: scene_nodes,
        // vertices: vertices,
        meshes: meshes,
        shaders: shaders,
        images: textures,
    };
}
