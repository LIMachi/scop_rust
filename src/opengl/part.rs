use std::ffi::c_void;
use std::mem::size_of;
use gl::types::{GLsizei, GLsizeiptr, GLuint};

#[derive(Default, Debug)]
pub struct ObjectPart {
    pub material: usize,
    pub vertices: Vec<[f32; 3]>,
    pub colors: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub vbos: [GLuint; 4]
}

impl ObjectPart {
    const VEC3_SIZE: usize = size_of::<[f32; 3]>();

    pub fn bake(&mut self) {
        unsafe {
            if self.vbos[0] == 0 {
                gl::GenBuffers(4, &mut self.vbos[0]); //allocate the buffers
                if self.vbos[0] == 0 {
                    return;
                }

                //set this buffer active for all subsequent functions
                gl::BindBuffer(gl::ARRAY_BUFFER, self.vbos[0]);
                //allocation of the buffer (total size, data, access kind) access kind STATIC_DRAW = one or few changes, frequent access
                gl::BufferData(gl::ARRAY_BUFFER, (ObjectPart::VEC3_SIZE * self.vertices.len()) as GLsizeiptr, self.vertices.as_ptr() as *const c_void, gl::STATIC_DRAW);

                if self.colors.len() > 0 {
                    gl::BindBuffer(gl::ARRAY_BUFFER, self.vbos[1]);
                    gl::BufferData(gl::ARRAY_BUFFER, (ObjectPart::VEC3_SIZE * self.colors.len()) as GLsizeiptr, self.colors.as_ptr() as *const c_void, gl::STATIC_DRAW);
                }
                
                if self.uvs.len() > 0 {
                    gl::BindBuffer(gl::ARRAY_BUFFER, self.vbos[2]);
                    gl::BufferData(gl::ARRAY_BUFFER, (ObjectPart::VEC3_SIZE * self.uvs.len()) as GLsizeiptr, self.uvs.as_ptr() as *const c_void, gl::STATIC_DRAW);
                }

                if self.normals.len() > 0 {
                    gl::BindBuffer(gl::ARRAY_BUFFER, self.vbos[3]);
                    gl::BufferData(gl::ARRAY_BUFFER, (ObjectPart::VEC3_SIZE * self.normals.len()) as GLsizeiptr, self.normals.as_ptr() as *const c_void, gl::STATIC_DRAW);
                }
            }
        }
    }

    pub fn bind(&self) {
        unsafe {
            if self.vbos[0] != 0 {
                //set this buffer active for all subsequent functions
                gl::BindBuffer(gl::ARRAY_BUFFER, self.vbos[0]);
                //description of the array in the shader (location = 0, each element is a group of 3 floats, unormalized, with a pointer offset of 0 because we already have an array defined above)
                gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, ObjectPart::VEC3_SIZE as GLsizei, 0 as *const _);
                //enable shader access to location 0
                gl::EnableVertexAttribArray(0);

                if self.colors.len() > 0 {
                    gl::BindBuffer(gl::ARRAY_BUFFER, self.vbos[1]);
                    gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, ObjectPart::VEC3_SIZE as GLsizei, 0 as *const _);
                    gl::EnableVertexAttribArray(1);
                }
                
                if self.uvs.len() > 0 {
                    gl::BindBuffer(gl::ARRAY_BUFFER, self.vbos[2]);
                    gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, ObjectPart::VEC3_SIZE as GLsizei, 0 as *const _);
                    gl::EnableVertexAttribArray(2);
                }

                if self.normals.len() > 0 {
                    gl::BindBuffer(gl::ARRAY_BUFFER, self.vbos[3]);
                    gl::VertexAttribPointer(3, 3, gl::FLOAT, gl::FALSE, ObjectPart::VEC3_SIZE as GLsizei, 0 as *const _);
                    gl::EnableVertexAttribArray(3);
                }
            }
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as GLsizei);
        }
    }
}