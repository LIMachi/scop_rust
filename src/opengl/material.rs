use crate::opengl::shader::ShaderProgram;
use crate::opengl::texture::Texture;

#[derive(Default, Debug)]
pub struct Material {
    pub specular_exponent: f32,
    pub density: f32,
    pub transparency: f32,
    pub filter: [f32; 3],
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    pub emissive: [f32; 3],
    pub illum: i32,
    pub specular_exponent_map: usize,
    pub transparency_map: usize,
    pub ambient_map: usize,
    pub diffuse_map: usize,
    pub specular_map: usize,
    pub emissive_map: usize,
    pub bump_map: usize,
    pub displacement_map: usize,
    pub stencil_map: usize,
}

impl Material {
    pub fn maps(&self) -> [usize; 9] {
        [
            self.specular_exponent_map,
            self.transparency_map,
            self.ambient_map,
            self.diffuse_map,
            self.specular_map,
            self.emissive_map,
            self.bump_map,
            self.displacement_map,
            self.stencil_map
        ]
    }

    pub fn bake(&self, textures: &mut Vec<Texture>) {
        for t in self.maps() {
            textures[t].bake();
        }
    }

    pub fn bind(&self, textures: &Vec<Texture>, shader: &ShaderProgram) {
        for (i, t) in self.maps().iter().enumerate() {
            textures[*t].bind(i, shader);
        }
    }
}