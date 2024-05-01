use std::ops::{Mul, MulAssign};
use super::matrix::Matrix;
use super::vector::Vector;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Quat {
    pub r: f32,
    pub i: f32,
    pub j: f32,
    pub k: f32,
}

impl Quat {
    pub fn identity() -> Self {
        Self {
            r: 1.,
            i: 0.,
            j: 0.,
            k: 0.,
        }
    }
    
    pub fn from_axis_angle(axis: Vector, angle: f32) -> Self {
        let sqr = axis.len_sqr();
        if sqr == 0. {
            Self::identity()
        } else {
            let angle = angle / 2.;
            let r = angle.cos();
            let s = if sqr != 1. {
                angle.sin() / sqr.sqrt()
            } else {
                angle.sin()
            };
            Self {
                r,
                i: axis.x() * s,
                j: axis.y() * s,
                k: axis.z() * s,
            }
        }
    }

    pub fn conjugate(&mut self) -> &mut Self {
        self.i = -self.i;
        self.j = -self.j;
        self.k = -self.k;
        self
    }

    pub fn scale(&mut self, scale: f32) -> &mut Self {
        self.r = self.r * scale;
        self.i = self.i * scale;
        self.j = self.j * scale;
        self.k = self.k * scale;
        self
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.r * rhs.r + self.i * rhs.i + self.j * rhs.j + self.k * rhs.k
    }

    pub fn len_sqr(&self) -> f32 {
        self.dot(self)
    }

    pub fn inverse(&mut self) -> &mut Self {
        let l = self.len_sqr();
        if l == 0. {
            self
        } else {
            if l != 1. {
                self.scale(1. / l);
            }
            self.conjugate()
        }
    }

    pub fn len(&self) -> f32 {
        self.len_sqr().sqrt()
    }

    pub fn normalize(&mut self) -> &mut Self {
        let sqr = self.len_sqr();
        if sqr != 1. && sqr != 0. {
            self.scale(1. / sqr.sqrt())
        } else {
            self
        }
    }
}

impl Mul for Quat {
    type Output = Quat;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r * rhs.r - self.i * rhs.i - self.j * rhs.j - self.k * rhs.k,
            i: self.r * rhs.i + self.i * rhs.r + self.j * rhs.k - self.k * rhs.j,
            j: self.r * rhs.j + self.j * rhs.r - self.i * rhs.k + self.k * rhs.i,
            k: self.r * rhs.k + self.k * rhs.r + self.i * rhs.j - self.j * rhs.i,
        }
    }
}

impl MulAssign for Quat {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl From<Quat> for Matrix {
    fn from(mut value: Quat) -> Self {
        let sqr = value.len_sqr();
        let mut out = Matrix::identity();
        if sqr != 0. {
            if sqr != 1. {
                value.scale(1. / sqr.sqrt());
            }
            out.set(0, 0, 1. - 2. * (value.j * value.j + value.k * value.k));
            out.set(1, 0, 2. * (value.i * value.j - value.r * value.k));
            out.set(2, 0, 2. * (value.r * value.j + value.i * value.k));
            out.set(0, 1, 2. * (value.r * value.k + value.i * value.j));
            out.set(1, 1, 1. - 2. * (value.i * value.i + value.k * value.k));
            out.set(2, 1, 2. * (value.j * value.k - value.r * value.i));
            out.set(0, 2, 2. * (value.i * value.k - value.r * value.j));
            out.set(1, 2, 2. * (value.r * value.i + value.j * value.k));
            out.set(2, 2, 1. - 2. * (value.i * value.i + value.j * value.j));
        }
        out
    }
}