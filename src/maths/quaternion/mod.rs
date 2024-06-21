pub mod conversions;
pub mod operators;
pub mod utilities;

pub struct Quaternion {
    pub(crate) r: f32,
    pub(crate) i: f32,
    pub(crate) j: f32,
    pub(crate) k: f32,
}