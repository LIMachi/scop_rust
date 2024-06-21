use crate::maths::vector::Vector;

pub mod conversions;
pub mod utilities;
pub mod operators;
pub mod mat4;

pub struct Matrix<const C: usize, const R: usize, K>(pub(crate) Vector<R, Vector<C, K>>);

pub type Mat3 = Matrix<3, 3, f32>;
pub type Mat4 = Matrix<4, 4, f32>;