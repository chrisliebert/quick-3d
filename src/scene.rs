// Copyright 2016 (C) Chris Liebert

use bincode::rustc_serialize::{encode_into, decode_from, EncodingError, DecodingError};
use bincode::SizeLimit::Infinite;
use std::fs::File;
use std::io::{BufWriter, BufReader};

use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use flate2::Compression;

use common::{ImageBlob, Material, Mesh};

/// Geometry and material information that can be rendered
///
/// A `Scene` contains geometry that will be rendered along with reference materials and
/// textures
///
#[derive(PartialEq, RustcEncodable, RustcDecodable)]
pub struct Scene {
    pub materials: Vec<Material>,
    pub meshes: Vec<Mesh>,
    pub images: Vec<ImageBlob>,
}

impl Scene {
    pub fn from_binary_file(filename: String) -> Result<Scene, DecodingError> {
        let file = match File::open(filename.clone()) {
            Ok(f) => f,
            Err(e) => { return Err(DecodingError::from(e)); },
        };
        let mut reader = BufReader::new(file);
        let scene_result: Result<Scene, _> = decode_from(&mut reader, Infinite);
        scene_result
    }

    pub fn to_binary_file(&self, filename: String) -> Result<(), EncodingError> {
        let file = match File::create(filename.clone()) {
            Ok(f) => f,
            Err(e) => { return Err(EncodingError::IoError(e)); },
        };
        let mut writer = BufWriter::new(file);
        encode_into(&self, &mut writer, Infinite)
    }

    pub fn from_compressed_binary_file(filename: String) -> Result<Scene, DecodingError> {
        let file = match File::open(filename.clone()) {
            Ok(f) => f,
            Err(e) => { return Err(DecodingError::from(e)); },
        };
        let reader = BufReader::new(file);
        let mut decoder = ZlibDecoder::new(reader);
        let scene_result: Result<Scene, _> = decode_from(&mut decoder, Infinite);
        scene_result
    }

    pub fn to_compressed_binary_file(&self, filename: String) -> Result<(), EncodingError> {
        let file = match File::create(filename.clone()) {
            Ok(f) => f,
            Err(e) => { return Err(EncodingError::IoError(e)); },
        };
        let writer = BufWriter::new(file);
        let mut encoder = ZlibEncoder::new(writer, Compression::Best);
        encode_into(&self, &mut encoder, Infinite)
    }
}
