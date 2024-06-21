use crate::opengl::shader::ShaderProgram;
use crate::opengl::texture::Texture;
use crate::opengl::uniform::Uniform;

#[derive(Default, Debug, Clone)]
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
    pub ambient_map: usize,
    pub diffuse_map: usize,
    pub transparency_map: usize,
    pub specular_exponent_map: usize,
    pub specular_map: usize,
    pub emissive_map: usize,
    pub bump_map: usize,
    pub displacement_map: usize,
    pub stencil_map: usize,
    pub ambient_uniform: Uniform,
    pub diffuse_uniform: Uniform,
    pub transparency_uniform: Uniform,
    pub specular_exponent_uniform: Uniform,
    pub specular_uniform: Uniform,
    pub emissive_uniform: Uniform,
    pub bump_uniform: Uniform,
    pub displacement_uniform: Uniform,
    pub stencil_uniform: Uniform,
}

impl Material {
    pub fn maps(&self) -> [usize; 9] {
        [
            self.ambient_map,
            self.diffuse_map,
            self.transparency_map,
            self.specular_exponent_map,
            self.specular_map,
            self.emissive_map,
            self.bump_map,
            self.displacement_map,
            self.stencil_map
        ]
    }
    
    pub fn uniforms(&self) -> [Uniform; 9] {
        [
            self.ambient_uniform,
            self.diffuse_uniform,
            self.transparency_uniform,
            self.specular_exponent_uniform,
            self.specular_uniform,
            self.emissive_uniform,
            self.bump_uniform,
            self.displacement_uniform,
            self.stencil_uniform
        ]
    }

    pub fn bake(&mut self, textures: &mut Vec<Texture>, program: &ShaderProgram) {
        for t in self.maps().iter() {
            textures[*t].bake();
        }
        self.ambient_uniform = program.uniform("ambient");
        self.diffuse_uniform = program.uniform("diffuse");
        self.transparency_uniform = program.uniform("transparency");
        self.specular_exponent_uniform = program.uniform("specular_exponent");
        self.specular_uniform = program.uniform("specular");
        self.emissive_uniform = program.uniform("emissive");
        self.bump_uniform = program.uniform("bump");
        self.displacement_uniform = program.uniform("displacement");
        self.stencil_uniform = program.uniform("stencil");
    }

    pub fn bind(&self, textures: &Vec<Texture>) {
        for (i, (t, u)) in self.maps().iter().zip(self.uniforms()).enumerate() {
            textures[*t].bind(i, u);
        }
    }
}