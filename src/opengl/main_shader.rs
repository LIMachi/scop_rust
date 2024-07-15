use crate::opengl::shader::ShaderProgram;
use crate::opengl::uniform::Uniform;

#[derive(Debug)]
pub struct MainShader {
    pub program: ShaderProgram,
    
    pub projection: Uniform,
    pub camera: Uniform,
    pub flags: Uniform,
    pub object: Uniform,
    
    pub fade: Uniform,
    
    pub ambient: Uniform,
    pub diffuse: Uniform,
    pub transparency: Uniform,
    pub specular_exponent: Uniform,
    pub specular: Uniform,
    pub emissive: Uniform,
    pub bump: Uniform,
    pub displacement: Uniform,
    pub stencil: Uniform,
}

impl MainShader {
    pub fn new(program: ShaderProgram) -> Self {
        Self {
            projection: program.uniform("projection"),
            camera: program.uniform("camera"),
            flags: program.uniform("flags"),
            object: program.uniform("object"),
            fade: program.uniform("fade"),
            ambient: program.uniform("ambient"),
            diffuse: program.uniform("diffuse"),
            transparency: program.uniform("transparency"),
            specular_exponent: program.uniform("specular_exponent"),
            specular: program.uniform("specular"),
            emissive: program.uniform("emissive"),
            bump: program.uniform("bump"),
            displacement: program.uniform("displacement"),
            stencil: program.uniform("stencil"),
            program
        }
    }
    
    pub fn material_uniforms(&self) -> [Uniform; 9] {
        [
            self.ambient,
            self.diffuse,
            self.transparency,
            self.specular_exponent,
            self.specular,
            self.emissive,
            self.bump,
            self.displacement,
            self.stencil
        ]
    }
}