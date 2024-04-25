use std::ffi::{c_void, CStr};
use std::mem::size_of;
use gl::*;
use gl::types::{GLchar, GLenum, GLint, GLsizei, GLsizeiptr, GLuint};
use crate::structures::matrix::Mat;

#[derive(Default)]
pub struct ShaderProgram {
    id: GLuint
}

impl ShaderProgram {
    pub fn new(id: GLuint) -> Self {
        Self { id }
    }
    
    pub fn set_float(&self, name: &str, value: f32) {
        unsafe {
            gl::Uniform1f(gl::GetUniformLocation(self.id, format!("{name}\0").as_ptr() as *const GLchar), value);
        }
    }

    pub fn set_vecf<const S: usize>(&self, name: &str, value: [f32; S]) {
        unsafe {
            match S {
                1 => {
                    gl::Uniform1f(gl::GetUniformLocation(self.id, format!("{name}\0").as_ptr() as *const GLchar), value[0]);
                }
                2 => {
                    gl::Uniform2f(gl::GetUniformLocation(self.id, format!("{name}\0").as_ptr() as *const GLchar), value[0], value[1]);
                }
                3 => {
                    gl::Uniform3f(gl::GetUniformLocation(self.id, format!("{name}\0").as_ptr() as *const GLchar), value[0], value[1], value[2]);
                }
                4 => {
                    gl::Uniform4f(gl::GetUniformLocation(self.id, format!("{name}\0").as_ptr() as *const GLchar), value[0], value[1], value[2], value[3]);
                }
                e @ _ => {
                    //should print an error
                }
            }
        }
    }
    
    pub fn set_mat<const S: usize>(&self, name: &str, value: Mat<S, S, f32>) {
        unsafe {
            match S {
                2 => {
                    gl::UniformMatrix2fv(gl::GetUniformLocation(self.id, format!("{name}\0").as_ptr() as *const GLchar), 1, TRUE, value.array().as_mut_ptr() as *const f32);
                }
                3 => {
                    gl::UniformMatrix3fv(gl::GetUniformLocation(self.id, format!("{name}\0").as_ptr() as *const GLchar), 1, TRUE, value.array().as_mut_ptr() as *const f32);
                }
                4 => {
                    gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, format!("{name}\0").as_ptr() as *const GLchar), 1, TRUE, value.array().as_mut_ptr() as *const f32);
                }
                e @ _ => {
                    //should print an error
                }
            }
        }
    }
}

#[derive(Default)]
pub struct ShaderProgramBuilder {
    shaders: Vec<GLuint>,
    error: GLint
}

pub struct VertexBuffer<T: Sized, const S: usize> {
    binding: GLuint,
    element_size: usize,
    element_count: usize,
    index: GLuint,
    var_kind: GLenum,
    draw_kind: GLenum,
    data: Vec<[T; S]>
}

impl <T: Sized, const S: usize> Default for VertexBuffer<T, S> {
    fn default() -> Self {
        Self {
            binding: 0,
            element_size: 0,
            element_count: 0,
            index: 0,
            var_kind: 0,
            draw_kind: 0,
            data: vec![],
        }
    }
}

impl <T: Sized, const S: usize> VertexBuffer<T, S> {
    pub fn gen() -> Option<VertexBuffer<T, S>> {
        let mut out = Self::default();
        unsafe {
            GenBuffers(1, &mut out.binding);
            if out.binding != 0 {
                Some(out)
            } else {
                None
            }
        }
    }

    pub fn load(&mut self, content: Vec<[T; S]>, reload: bool) -> &mut Self {
        self.element_size = size_of::<[T; S]>();
        self.element_count = content.len();
        println!("size: {}, count: {}", self.element_size, self.element_count);
        self.data = content;
        unsafe {
            BindBuffer(ARRAY_BUFFER, self.binding);
            BufferData(ARRAY_BUFFER, (self.element_size * self.element_count) as GLsizeiptr, self.data.as_ptr() as *const c_void, STATIC_DRAW);
        }
        if reload {
            self.enable(self.index, self.var_kind, self.draw_kind);
        }
        self
    }

    pub fn enable(&mut self, index: GLuint, var_kind: GLenum, draw_kind: GLenum) -> &mut Self {
        self.index = index;
        self.var_kind = var_kind;
        self.draw_kind = draw_kind;
        unsafe {
            VertexAttribPointer(index, S as GLint, var_kind, FALSE, self.element_size as GLsizei, 0 as *const _);
            EnableVertexAttribArray(index);
        }
        self
    }
    
    pub fn draw(&self) {
        unsafe {
            EnableVertexAttribArray(self.index);
            DrawArrays(self.draw_kind, 0, self.element_count as GLsizei);
        }
    }
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