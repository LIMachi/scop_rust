use crate::maths::matrix::Mat4;
use crate::maths::transform::Transform;
use crate::maths::vector::Vec3;

//should be rebuilt when the mvp is changed (camera or light moved)
#[derive(Debug)]
pub struct Frustrum {
    normals: [Vec3; 6] //order: left, right, bottom, top, near, far
}

//composite volume: sphere + aabb
//aabb is tested only if the sphere is intersecting partially
#[derive(Debug, Default)]
pub struct Volume {
    radius: f32,
    aabb_min: Vec3,
    aabb_max: Vec3
}

impl Volume {
    pub fn expand(&mut self, vertex: &Vec3) {
        let s = vertex.dot(vertex);
        if self.radius * self.radius < s {
            self.radius = s.sqrt();
        }
        self.aabb_min = self.aabb_min.min(*vertex);
        self.aabb_max = self.aabb_max.max(*vertex);
    }
}

impl Frustrum {
    pub fn from_mvp(mvp: &Mat4) -> Self {
        let v = mvp.row(3); //position
        let mut out = Self { normals: [Vec3::default(); 6] };
        for i in 0..3 {
            let r = mvp.row(i);
            out.normals[i * 2] = (v + r).resize().normalize();
            out.normals[i * 2 + 1] = (v - r).resize().normalize();
        }
        out
    }
    
    pub fn has_volume(&self, transform: &Transform, volume: &Volume) -> bool {
        let mut sub = true;
        for norm in &self.normals {
            if norm.dot(&transform.pos) + volume.radius <= 0. {
                return false;
            }
            if !sub && norm.dot(&transform.pos) - volume.radius >= 0. {
                sub = false;
            }
        }
        if sub {
            //do aabb check
        }
        true
    }
}