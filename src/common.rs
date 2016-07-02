#[derive(Debug)]
pub struct SceneNode {
    pub name: String,
    pub material_id: i32,
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
    // pub vertex_source_110: String,
    // pub fragment_source_110: String,
    // pub vertex_source_120: String,
    // pub fragment_source_120: String,
    // pub vertex_source_130: String,
    // pub fragment_source_130: String,
    pub vertex_source_140: String,
    pub fragment_source_140: String, /* pub vertex_source_150: String,
                                      * pub fragment_source_150: String,
                                      * pub vertex_source_330: String,
                                      * pub fragment_source_330: String,
                                      * pub vertex_source_400: String,
                                      * pub fragment_source_400: String,
                                      * pub vertex_source_410: String,
                                      * pub fragment_source_401: String,
                                      * pub vertex_source_420: String,
                                      * pub fragment_source_420: String,
                                      * pub vertex_source_430: String,
                                      * pub fragment_source_430: String,
                                      * pub vertex_source_100es: String,
                                      * pub fragment_source_100es: String,
                                      * pub vertex_source_300es: String,
                                      * pub fragment_source_300es: String,
                                      * */
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Vertex8f32 {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub texcoord: [f32; 2],
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Mesh {
    pub name: String,
    pub vertices: Vec<Vertex8f32>,
    pub material_id: i32,
}

#[derive(Debug)]
pub struct Scene {
    pub materials: Vec<Material>,
    pub meshes: Vec<Mesh>,
    pub shaders: Vec<Shader>,
}
