use crate::maths::matrix::{Mat3, Mat4, Matrix};
use crate::maths::Unit;
use crate::maths::vector::{Vec3, Vec4};
use super::Quaternion;

#[derive(Debug)]
pub enum QuaternionError {
    CantCoerceQuaternionToRealDueToImaginaryParts,
    CantCoerceQuaternionToComplexDueToImaginaryParts,
}

impl From<f32> for Quaternion {
    fn from(value: f32) -> Self {
        Self {
            r: value.into(),
            i: 0.,
            j: 0.,
            k: 0.
        }
    }
}

impl TryFrom<Quaternion> for f32 {
    type Error = QuaternionError;

    fn try_from(value: Quaternion) -> Result<Self, Self::Error> {
        if value.i != 0. || value.j != 0. || value.k != 0. {
            Err(QuaternionError::CantCoerceQuaternionToRealDueToImaginaryParts)
        } else {
            Ok(value.r)
        }
    }
}

impl From<Vec3> for Quaternion {
    fn from(axis: Vec3) -> Self {
        let sqr = axis.dot(&axis);
        if sqr == 0. {
            Self::unit()
        } else {
            let angle = 0.5f32.to_radians();
            let s = if sqr != 1. {
                angle.sin() / sqr.sqrt()
            } else {
                angle.sin()
            };
            Self {
                r: angle.cos(),
                i: axis[0] * s,
                j: axis[1] * s,
                k: axis[2] * s,
            }
        }
    }
}

impl From<(Vec3, f32)> for Quaternion {
    fn from((axis, angle): (Vec3, f32)) -> Self {
        let sqr = axis.dot(&axis);
        let angle = angle / 2.;
        if sqr == 0. {
            Self::unit()
        } else {
            let s = if sqr != 1. {
                angle.sin() / sqr.sqrt()
            } else {
                angle.sin()
            };
            Self {
                r: angle.cos(),
                i: axis[0] * s,
                j: axis[1] * s,
                k: axis[2] * s,
            }
        }
    }
}

impl From<Quaternion> for Mat4 {
    fn from(mut value: Quaternion) -> Self {
        let sqr = {
            let t = Vec4::from([value.r, value.i, value.j, value.k]);
            t.dot(&t)
        };
        if sqr != 0. {
            if sqr != 1. {
                value = value / sqr.sqrt();
            }
            Mat4::from([[
                1. - 2. * (value.j * value.j + value.k * value.k),
                2. * (value.r * value.k + value.i * value.j),
                2. * (value.i * value.k - value.r * value.j),
                0.
            ], [
                2. * (value.i * value.j - value.r * value.k),
                1. - 2. * (value.i * value.i + value.k * value.k),
                2. * (value.r * value.i + value.j * value.k),
                0.
            ], [
                2. * (value.r * value.j + value.i * value.k),
                2. * (value.j * value.k - value.r * value.i),
                1. - 2. * (value.i * value.i + value.j * value.j),
                0.
            ], [0., 0., 0., 1.]])
        } else {
            Mat4::identity()
        }
    }
}

impl From<Quaternion> for Mat3 {
    fn from(mut value: Quaternion) -> Self {
        let sqr = {
            let t = Vec4::from([value.r, value.i, value.j, value.k]);
            t.dot(&t)
        };
        if sqr != 0. {
            if sqr != 1. {
                value = value / sqr.sqrt();
            }
            Mat3::from([[
                1. - 2. * (value.j * value.j + value.k * value.k),
                2. * (value.r * value.k + value.i * value.j),
                2. * (value.i * value.k - value.r * value.j),
            ], [
                2. * (value.i * value.j - value.r * value.k),
                1. - 2. * (value.i * value.i + value.k * value.k),
                2. * (value.r * value.i + value.j * value.k),
            ], [
                2. * (value.r * value.j + value.i * value.k),
                2. * (value.j * value.k - value.r * value.i),
                1. - 2. * (value.i * value.i + value.j * value.j),
            ]])
        } else {
            Mat3::identity()
        }
    }
}

impl <const C: usize, const R: usize> From<Matrix<C, R, f32>> for Quaternion {
    fn from(value: Matrix<C, R, f32>) -> Self {
        if C < 3 || R < 3 {
            Self::unit()
        } else {
            let trace = value[(0, 0)] + value[(1, 1)] + value[(2, 2)];
            if trace > 0. {
                let s = 0.5 / (trace + 1.).sqrt();
                Self {
                    r: 0.25 / s,
                    i: (value[(1, 2)] - value[(2, 1)]) * s,
                    j: (value[(2, 0)] - value[(0, 2)]) * s,
                    k: (value[(0, 1)] - value[(1, 0)]) * s,
                }
            } else if value[(0, 0)] > value[(1, 1)] && value[(0, 0)] > value[(2, 2)] {
                let s = 2. * (1. + value[(0, 0)] - value[(1, 1)] - value[(2, 2)]).sqrt();
                Self {
                    r: (value[(1, 2)] - value[(2, 1)]) / s,
                    i: 0.25 * s,
                    j: (value[(1, 0)] + value[(0, 1)]) / s,
                    k: (value[(2, 0)] + value[(0, 2)]) / s,
                }
            } else if value[(1, 1)] > value[(2, 2)] {
                let s = 2. * (1. + value[(1, 1)] - value[(0, 0)] - value[(2, 2)]).sqrt();
                Self {
                    r: (value[(2, 0)] - value[(0, 2)]) / s,
                    i: (value[(1, 0)] + value[(0, 1)]) / s,
                    j: 0.25 * s,
                    k: (value[(2, 1)] + value[(1, 2)]) / s,
                }
            } else {
                let s = 2. * (1. + value[(2, 2)] - value[(0, 0)] - value[(1, 1)]).sqrt();
                Self {
                    r: (value[(0, 1)] - value[(1, 0)]) / s,
                    i: (value[(2, 0)] + value[(0, 2)]) / s,
                    j: (value[(2, 1)] + value[(1, 2)]) / s,
                    k: 0.25 * s,
                }
            }
        }
    }
}