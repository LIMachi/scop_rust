pub mod utilities;
pub mod conversions;
pub mod operators;

pub struct Vector<const S: usize, K>(pub(crate) [K; S]);

pub type Vec3 = Vector<3, f32>;
pub type Vec4 = Vector<4, f32>;

impl Vec3 {
    pub const X: Vec3 = Vec3::new(1., 0., 0.);
    pub const Y: Vec3 = Vec3::new(0., 1., 0.);
    pub const Z: Vec3 = Vec3::new(0., 0., 1.);
    
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
    }
}

impl Vec4 {
    pub const X: Vec4 = Vec4::new(1., 0., 0., 0.);
    pub const Y: Vec4 = Vec4::new(0., 1., 0., 0.);
    pub const Z: Vec4 = Vec4::new(0., 0., 1., 0.);
    pub const W: Vec4 = Vec4::new(0., 0., 0., 1.);

    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self([x, y, z, w])
    }
}