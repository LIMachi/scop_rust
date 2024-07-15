use crate::maths::vector::Vec3;
use crate::opengl::buffers::{GPUBuffers, VertexType};
use crate::opengl::frustrum::Volume;

pub struct VaoObject {
    len: usize,
    buffers: GPUBuffers,
    volume: Volume
}

impl VaoObject {
    pub fn from_raw(vertices: Vec<[f32; 3]>, indices: Vec<u32>) -> Self {
        let mut buffers = GPUBuffers::new().unwrap();
        buffers.new_vbo(0, VertexType::Vec3);
        let len = indices.len();
        buffers.set_ebo(indices);
        let mut volume = Volume::default();
        for v in &vertices {
            volume.expand(&Vec3::from(*v));
        }
        buffers.set_vbo(0, vertices);
        Self {
            len,
            buffers,
            volume,
        }
    }
}