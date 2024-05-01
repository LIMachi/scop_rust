use std::collections::HashMap;

mod object;
mod material;
mod texture;
mod point;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub pos: [f32; 3],
    pub color: [f32; 3],
    pub w: f32
}

impl Default for Point {
    fn default() -> Self {
        Self {
            pos: [0.; 3],
            color: [1.; 3],
            w: 1.
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ParsedObject {
    pub libs: ParsedMaterialLib, //mtllibs
    pub vertexes: Vec<Point>, //v
    pub uvs: Vec<[f32; 3]>, //vt
    pub normals: Vec<[f32; 3]>, //vn
    pub materials: Vec<String>, //usemtl / mapping from index to name
    pub material_index: HashMap<String, usize>, //usemtl / mapping from name to index
    pub groups: Vec<[usize; 3]>, //usemtl / mapping material -> range inclusive of faces
    pub faces: Vec<Vec<[usize; 3]>>, //f
    pub normalized: bool, //is this object already normalized
}

#[derive(Debug, Clone)]
pub struct ParsedMaterial {
    pub specular_exponent: f32, //Ns
    pub density: f32, //Ni
    pub transparency: f32, //d / inverted Tr
    pub filter: [f32; 3], //optional Tf
    pub ambient: [f32; 3], //Ka
    pub diffuse: [f32; 3], //Kd
    pub specular: [f32; 3], //Ks
    pub emissive: [f32; 3], //Ke
    pub illum: i32, //illumination model
    pub specular_exponent_map: String, //map_Ns
    pub transparency_map: String, //map_d
    pub ambient_map: String, //map_Ka
    pub diffuse_map: String, //map_Kd
    pub specular_map: String, //map_Ks
    pub emissive_map: String, //map_Ke
    pub bump_map: String, //map_bum / bump
    pub displacement_map: String, //disp
    pub stencil_map: String, //decal
}

impl Default for ParsedMaterial {
    fn default() -> Self {
        Self {
            specular_exponent: 0.,
            density: 1.,
            transparency: 0.,
            filter: [1., 1., 1.],
            ambient: [0., 0., 0.],
            diffuse: [0., 0., 0.],
            specular: [0., 0., 0.],
            emissive: [0., 0., 0.],
            illum: 2,
            specular_exponent_map: "".to_string(),
            ambient_map: "".to_string(),
            diffuse_map: "".to_string(),
            specular_map: "".to_string(),
            transparency_map: "".to_string(),
            bump_map: "".to_string(),
            displacement_map: "".to_string(),
            stencil_map: "".to_string(),
            emissive_map: "".to_string(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ParsedMaterialLib(pub HashMap<String, ParsedMaterial>);

#[derive(Debug, Default, Clone)]
pub struct ParsedTexture {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}