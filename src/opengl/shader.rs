use gl::{*, types::*};
use crate::maths::matrix::Matrix;
use crate::maths::vector::Vector;
use crate::other::resource_manager::ResourceManager;

#[derive(Default)]
pub struct ShaderProgram {
    id: GLuint
}

impl ShaderProgram {
    pub fn from_resources(resources: &mut ResourceManager, name: &str) -> Option<Self> {
        let mut builder = ShaderProgramBuilder::default();
        builder.add_shader(VERTEX_SHADER, resources.load_text(format!("{name}.vert"))?.as_str());
        builder.add_shader(FRAGMENT_SHADER, resources.load_text(format!("{name}.frag"))?.as_str());
        builder.build()
    }

    pub fn new(id: GLuint) -> Self {
        Self { id }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        unsafe {
            gl::Uniform1f(gl::GetUniformLocation(self.id, format!("{name}\0").as_ptr() as *const GLchar), value);
        }
    }

    pub fn set_vec(&self, name: &str, value: Vector) {
        unsafe {
            gl::Uniform4f(gl::GetUniformLocation(self.id, format!("{name}\0").as_ptr() as *const GLchar), value.x(), value.y(), value.z(), value.w());
        }
    }

    pub fn set_mat(&self, name: &str, value: Matrix) {
        unsafe {
            gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, format!("{name}\0").as_ptr() as *const GLchar), 1, TRUE, value.as_array().as_mut_ptr() as *const f32);
        }
    }

    pub fn set_u32(&self, name: &str, value: u32) {
        unsafe {
            gl::Uniform1i(gl::GetUniformLocation(self.id, format!("{name}\0").as_ptr() as *const GLchar), value as GLint);
        }
    }
}

#[derive(Default)]
pub struct ShaderProgramBuilder {
    shaders: Vec<GLuint>,
    error: GLint
}

impl ShaderProgramBuilder {

    pub fn add_shader(&mut self, kind: GLenum, source: &str) -> &mut Self {
        unsafe {
            let shader = gl::CreateShader(kind);
            if shader == 0 {
                return self;
            }
            gl::ShaderSource(shader, 1, &(source.as_bytes().as_ptr() as *const GLchar), &(source.len() as GLint));
            gl::CompileShader(shader);
            gl::GetShaderiv(shader, COMPILE_STATUS, &mut self.error);
            if self.error != 0 {
                self.shaders.push(shader);
            } else {
                let mut info: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(
                    shader,
                    1024,
                    &mut log_len,
                    info.as_mut_ptr().cast(),
                );
                info.set_len(log_len.try_into().unwrap());
                println!("Fragment Compile Error: {}", String::from_utf8_lossy(&info));
                DeleteShader(shader);
            }
            self
        }
    }

    pub fn build(&mut self) -> Option<ShaderProgram> {
        unsafe {
            let shader_program = CreateProgram();
            if shader_program == 0 {
                return None;
            }
            for shader in self.shaders.iter() {
                AttachShader(shader_program, *shader);
            }
            LinkProgram(shader_program);
            let mut success = 0;
            GetProgramiv(shader_program, LINK_STATUS, &mut success);
            if success != 0 {
                for shader in self.shaders.iter() {
                    DeleteShader(*shader);
                }
                UseProgram(shader_program);
                Some(ShaderProgram::new(shader_program))
            } else {
                let mut info: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                GetProgramInfoLog(
                    shader_program,
                    1024,
                    &mut log_len,
                    info.as_mut_ptr().cast(),
                );
                info.set_len(log_len.try_into().unwrap());
                println!("Program Link Error: {}", String::from_utf8_lossy(&info));
                None
            }
        }
    }
}