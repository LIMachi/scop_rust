#[derive(Debug, Clone)]
pub struct Material { //name: newmtl<whitespace><name>
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

impl Default for Material {
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