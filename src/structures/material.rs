#[derive(Default, Debug)]
pub struct Material { //name: newmtl<whitespace><name>
    pub specular_exponent: f32, //Ns
    pub ambient: [f32; 3], //Ka
    pub diffuse: [f32; 3], //Kd
    pub specular: [f32; 3], //Ks
    pub density: f32, //Ni
    pub transparency: f32, //d / inverted Tr
    pub filter: [f32; 3], //optional Tf
    pub illum: i32, //illumination model
    pub specular_exponent_map: String, //map_Ns
    pub ambient_map: String, //map_Ka
    pub diffuse_map: String, //map_Kd
    pub specular_map: String, //map_Ks
    pub transpaerncy_map: String, //map_d
    pub bump_map: String, //map_bum / bump
    pub displacement_map: String, //disp
    pub stencil_map: String, //decal
}