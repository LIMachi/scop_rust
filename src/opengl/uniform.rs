use gl::types::{GLchar, GLint, GLsizei};
use crate::maths::matrix::{Mat3, Mat4};
use crate::maths::vector::{Vec3, Vec4};
use crate::opengl::shader::ShaderProgram;

#[derive(Default, Debug, Copy, Clone)]
pub struct Uniform(GLint);

#[allow(dead_code)]
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

    pub fn vec3(&self, value: Vec3) {
        unsafe {
            gl::Uniform3f(self.0, value[0], value[1], value[2]);
        }
    }
    
    pub fn array3f(&self, value: &Vec<[f32; 3]>) {
        unsafe {
            gl::Uniform3fv(self.0, value.len() as GLsizei, value.as_ptr() as *const f32);
        }
    }
    
    pub fn vec4(&self, value: Vec4) {
        unsafe {
            gl::Uniform4f(self.0, value[0], value[1], value[2], value[3]);
        }
    }

    pub fn mat3(&self, value: Mat3) {
        unsafe {
            gl::UniformMatrix3fv(self.0, 1, gl::FALSE, Vec::from(value).as_ptr());
        }
    }
    
    pub fn mat4(&self, value: Mat4) {
        unsafe {
            gl::UniformMatrix4fv(self.0, 1, gl::FALSE, Vec::from(value).as_ptr());
        }
    }
    
    pub fn array_mat4(&self, value: &Vec<Mat4>) {
        unsafe {
            gl::UniformMatrix4fv(self.0, value.len() as GLsizei, gl::FALSE, value.iter().flat_map(|m| Vec::<f32>::from(*m)).collect::<Vec<f32>>().as_ptr());
        }
    }
    
    pub fn raw_array_mat4(&self, value: &[f32]) {
        unsafe {
            gl::UniformMatrix4fv(self.0, (value.len() / 16) as GLsizei, gl::FALSE, value.as_ptr());
        }
    }
    
    pub fn array_int(&self, value: &[i32]) {
        unsafe {
            gl::Uniform1iv(self.0, value.len() as GLsizei, value.as_ptr());
        }
    }
}