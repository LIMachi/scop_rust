use std::collections::HashMap;
use crate::structures::face::Face;
use crate::structures::material::Material;
use crate::structures::point::Point;
use crate::structures::texture::Texture;

#[derive(Default, Debug)]
pub struct Object {
    pub name: String,
    pub materials: Vec<Material>,
    pub textures: HashMap<String, Texture>,
    pub groups: HashMap<String, Vec<usize>>,
    pub smoothing: Vec<Vec<usize>>,
    pub points: Vec<Point>,
    pub uvs: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub faces: Vec<Face>,
}

impl Object {
    pub fn triangles(&self) -> Vec<[f32; 3]> {
        let mut out = Vec::new();
        for face in self.faces.iter() {
            out.extend(face.vertices(&self.points));
        }
        out
    }
    
    pub fn uvs(&self) -> Vec<[f32; 3]> {
        let mut out = Vec::new();
        for face in self.faces.iter() {
            out.extend(face.uvs(&self.uvs));
        }
        out
    }
    
    pub fn normals(&self) -> Vec<[f32; 3]> {
        let mut out = Vec::new();
        for face in self.faces.iter() {
            out.extend(face.normals(&self.normals));
        }
        out
    }
}