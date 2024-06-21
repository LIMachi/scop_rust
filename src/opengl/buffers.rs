use std::collections::HashMap;
use std::mem::size_of;
use std::os::raw::c_void;
use gl::types::{GLenum, GLint, GLsizei, GLsizeiptr, GLuint};
use crate::opengl::safe_calls;

//abstraction of VAO, VBO, EBO, etc...
//reminder: vbo and ebo usually refers to buffers that store
//vertex data and index data respectively,
//and vao refers to a buffer containing the access data of the previous buffers
//ex: we declare a vao, then declare a vbo with a triangle inside, plus information for the shader
//on how to access the vertice data, this information is now in vao, and the raw triangle data in vbo

//https://www.khronos.org/opengl/wiki/Vertex_Specification#Vertex_Array_Object

//this struct is then a glorified vao that also handles vbo/ebo usage
pub struct GPUBuffers {
    vao: GLuint,
    vbos: HashMap<usize, GLuint>, //mapping layout -> vbo
    ebo: GLuint,
}

struct VertexTypeLayout {
    count: GLint,
    kind: GLenum,
    size: GLsizei
}

pub enum VertexType {
    Float,
    Int,
    Vec2,
    Vec3,
    Vec4
}

impl VertexType {
    pub fn layout(&self) -> VertexTypeLayout {
        match self {
            VertexType::Float => VertexTypeLayout {
                count: 1,
                kind: gl::FLOAT,
                size: size_of::<f32>() as GLsizei,
            },
            VertexType::Int => VertexTypeLayout {
                count: 1,
                kind: gl::INT,
                size: size_of::<i32>() as GLsizei,
            },
            VertexType::Vec2 => VertexTypeLayout {
                count: 2,
                kind: gl::FLOAT,
                size: size_of::<[f32; 2]>() as GLsizei,
            },
            VertexType::Vec3 => VertexTypeLayout {
                count: 3,
                kind: gl::FLOAT,
                size: size_of::<[f32; 3]>() as GLsizei,
            },
            VertexType::Vec4 => VertexTypeLayout {
                count: 4,
                kind: gl::FLOAT,
                size: size_of::<[f32; 4]>() as GLsizei,
            },
        }
    }
}

impl GPUBuffers {
    ///try to create a new instance of vao (might fail if we overflown the gpu)
    pub fn new() -> Option<Self> {
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        if vao == 0 {
            None
        } else {
            Some(Self {
                vao,
                vbos: HashMap::new(),
                ebo: 0
            })
        }
    }

    ///bind this object (vao) to be in use by the shader/gpu or for manipulation/declaration cpu side
    pub fn bind(&self) {
        if self.vao != 0 && safe_calls::get_vao() != self.vao {
            unsafe {
                gl::BindVertexArray(self.vao);
            }
        }
    }

    ///internal function used to allocate vbos when needed and mapping them to a location
    fn ensure_vbo(&mut self, index: usize) -> bool {
        if !self.vbos.contains_key(&index) {
            let mut t = 0;
            unsafe {
                gl::GenBuffers(1, &mut t);
            }
            if t == 0 {
                return false;
            }
            self.vbos.insert(index, t);
        }
        self.bind();
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, *self.vbos.get(&index).unwrap());
        }
        true
    }

    ///create a simple vertex buffer without mingle (meaning: all the data uploaded will be used only by this location)
    pub fn new_vbo(&mut self, index: usize, kind: VertexType) {
        if self.ensure_vbo(index) {
            unsafe {
                let VertexTypeLayout { count, kind, size } = kind.layout();
                gl::VertexAttribPointer(index as GLuint, count, kind, gl::FALSE, size, 0 as *const _);
                gl::EnableVertexAttribArray(index as GLuint);
            }
        }
    }

    ///create a mingled vertex buffer (meaning: multiple locations will be bound to this single buffer using offsets, as if the data of this buffer was a vector of structs)
    ///ex: we want to use a single buffer to send position (vec3) and uv (vec2), the mingle size will be 20 (5 floats)
    ///(position): new_mingled_vbo(0, 0, Vec3, 20, 0);
    ///(uv): new_mingled_vbo(0, 1, Vec2, 20, 12); (at index 1, we use the same buffer as index 0, and since we already pushed vec3, the offset will be 12 or 3 floats)
    pub fn new_mingled_vbo(&mut self, first_index: usize, index: usize, kind: VertexType, mingle_size: usize, offset: usize) {
        if self.ensure_vbo(first_index) {
            if first_index != index {
                self.vbos.insert(index, *self.vbos.get(&first_index).unwrap());
            }
            unsafe {
                let VertexTypeLayout { count, kind, .. } = kind.layout();
                gl::VertexAttribPointer(index as GLuint, count, kind, gl::FALSE, mingle_size as GLsizei, offset as *const _);
                gl::EnableVertexAttribArray(index as GLuint);
            }
        }
    }

    ///set the contend of a vbo by copying a vec as raw data to the gpu
    pub fn set_vbo<T>(&self, index: usize, data: Vec<T>) {
        if !self.vbos.contains_key(&index) { return; }
        self.bind();
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, *self.vbos.get(&index).unwrap());
            gl::BufferData(gl::ARRAY_BUFFER, (size_of::<T>() * data.len()) as GLsizeiptr, data.as_ptr() as *const c_void, gl::STATIC_DRAW);
        }
    }
    
    ///remove the buffer at this location for this vao
    pub fn discard_vbo(&mut self, index: usize) {
        if !self.vbos.contains_key(&index) { return; }
        self.bind();
        let vbo = self.vbos.remove(&index).unwrap();
        unsafe {
            gl::DeleteBuffers(1, &vbo);
            gl::DisableVertexAttribArray(index as GLuint);
        }
    }

    ///get access to a registered vbo if available
    pub fn vbo(&self, index: usize) -> GLuint {
        self.vbos.get(&index).copied().unwrap_or(0)
    }
    
    ///set an ebo for this vao, meaning the vbos will be read in the order of the given indices instead of all sequentially (useful to reuse multiple vertices)
    pub fn set_ebo(&mut self, indices: Vec<u32>) {
        if self.ebo == 0 {
            self.bind();
            unsafe {
                gl::GenBuffers(1, &mut self.ebo);
            }
            if self.ebo == 0 {
                return;
            }
        }
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (size_of::<u32>() * indices.len()) as GLsizeiptr, indices.as_ptr() as *const c_void, gl::STATIC_DRAW);
        }
    }
    
    ///disable the ebo for this vao (the vertices will be used sequentially instead of in the order of the ebo)
    pub fn discard_ebo(&mut self) {
        if self.ebo == 0 { return; }
        self.bind();
        unsafe {
            gl::DeleteBuffers(1, &self.ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
        self.ebo = 0;
    }
    
    ///try to draw this vao/vbos using either array mode or element mode based on if the ebo is in use or not
    ///note: count is the number of vertices/indexes, not the count of triangles/lines/etc...
    pub fn draw(&self, mode: GLenum, from: usize, count: usize) {
        self.bind();
        unsafe {
            if self.ebo != 0 {
                gl::DrawElements(mode, count as GLsizei, gl::UNSIGNED_INT, (from * size_of::<u32>()) as *const c_void);
            } else {
                gl::DrawArrays(mode, from as GLint, count as GLsizei);
            }
        }
    }

    ///try to draw this vao/vbos using either array mode or element mode based on if the ebo is in use or not
    ///note: count is the number of vertices/indexes, not the count of triangles/lines/etc...
    ///this variant will draw multiple versions of the same part either updating gl_InstanceID between each instance and updating a an instance buffer (see: glVertexAttribDivisor)
    pub fn draw_instances(&self, mode: GLenum, from: usize, count: usize, instances: usize) {
        self.bind();
        unsafe {
            if self.ebo != 0 {
                gl::DrawElementsInstanced(mode, count as GLsizei, gl::UNSIGNED_INT, (from * size_of::<u32>()) as *const c_void, instances as GLsizei);
            } else {
                gl::DrawArraysInstanced(mode, from as GLint, count as GLsizei, instances as GLsizei);
            }
        }
    }
}