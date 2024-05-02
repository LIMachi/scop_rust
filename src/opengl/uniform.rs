use gl::TRUE;
use gl::types::{GLchar, GLint};
use crate::maths::matrix::Matrix;
use crate::maths::vector::Vector;
use crate::opengl::shader::ShaderProgram;

#[derive(Default, Debug, Copy, Clone)]
pub struct Uniform(GLint);

impl Uniform {
    pub fn new(shader: &ShaderProgram, name: &str) -> Self {
        unsafe {
            Self (
                gl::GetUniformLocation(shader.id(), format!("{name}\0").as_ptr() as * const GLchar)
            )
        }
    }
    
    pub fn int(&self, value: i32) {
        unsafe {
            gl::Uniform1i(self.0, value);
        }
    }
    
    pub fn float(&self, value: f32) {
        unsafe {
            gl::Uniform1f(self.0, value);
        }
    }
    
    pub fn vec(&self, value: Vector) {
        unsafe {
            gl::Uniform4f(self.0, value.x(), value.y(), value.z(), value.w());
        }
    }
    
    pub fn mat(&self, value: Matrix) {
        unsafe {
            gl::UniformMatrix4fv(self.0, 1, TRUE, value.as_array().as_mut_ptr() as *const f32);
        }
    }
}