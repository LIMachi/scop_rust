use crate::shader::VertexBuffer;
use crate::structures::object::Object;
use crate::structures::quat::Quat;
use crate::structures::vector::Vector;

pub struct SpawnedObject {
    pub object: Object,
    pub vertices: VertexBuffer<f32, 3>,
    pub uvs: VertexBuffer<f32, 3>,
    pub normals: VertexBuffer<f32, 3>,
    pub vtextures: VertexBuffer<u32, 1>,
    pub pos: Vector,
    pub rot: Quat,
    pub scale: Vector,
}