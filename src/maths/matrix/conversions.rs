#![cfg_attr(nightly, feature(generic_const_exprs))]

use crate::maths::vector::Vector;
use super::Matrix;

impl <const C: usize, const R: usize, K: Default + Copy> From<[[K; C]; R]> for Matrix<C, R, K> {
    fn from(value: [[K; C]; R]) -> Self {
        let mut t = Vector::default();
        for i in 0..R {
            t[i] = Vector::from(value[i]);
        }
        Self(t)
    }
}

//TODO check which order is used by OpenGL/Vulkan to make this operation usable in the future
#[cfg(feature = "nightly")]
impl <const C: usize, const R: usize, K: Default + Copy> From<Matrix<C, R, K>> for [K; C * R] {
    fn from(value: Matrix<C, R, K>) -> Self {
        let mut out = [K::default(); C * R];
        for c in 0..C {
            for r in 0..R {
                out[r + c * R] = value[(c, r)];
            }
        }
        out
    }
}

impl <const C: usize, const R: usize, K: Default + Copy> From<Matrix<C, R, K>> for Vec<K> {
    fn from(value: Matrix<C, R, K>) -> Self {
        let mut out = Vec::with_capacity(C * R);
        for c in 0..C {
            for r in 0..R {
                out.push(value[(c, r)]);
            }
        }
        out
    }
}