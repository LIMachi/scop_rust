use std::fmt::Debug;
use std::ops::Mul;
use crate::structures::vector::Vector;

#[derive(Default, Debug, Copy, Clone)]
pub struct Matrix {
    inner: [f32; 16]
}

impl From<[f32; 16]> for Matrix {
    fn from(value: [f32; 16]) -> Self {
        Self {
            inner: value
        }
    }
}

impl From<Matrix> for [f32; 16] {
    fn from(value: Matrix) -> Self {
        value.inner
    }
}

impl Matrix {
    pub fn from_pos(pos: &Vector) -> Self {
        Self {
            inner: [
                1., 0., 0., pos.x(),
                0., 1., 0., pos.y(),
                0., 0., 1., pos.z(),
                0., 0., 0., 1.,
            ]
        }
    }
    
    pub fn as_array(&self) -> [f32; 16] {
        self.inner
    }
    
    pub fn identity() -> Self {
        Self {
            inner: [
                1., 0., 0., 0.,
                0., 1., 0., 0.,
                0., 0., 1., 0.,
                0., 0., 0., 1.,
            ]
        }
    }
    
    pub fn set(&mut self, col: usize, row: usize, value: f32) -> &mut Self {
        assert!(col < 4, "Invalid column access {col}, matrix is 4*4");
        assert!(row < 4, "Invalid row access {row}, matrix is 4*4");
        self.inner[col + row * 4] = value;
        self
    }
    
    pub fn get(&self, col: usize, row: usize) -> f32 {
        assert!(col < 4, "Invalid column access {col}, matrix is 4*4");
        assert!(row < 4, "Invalid row access {row}, matrix is 4*4");
        self.inner[col + row * 4]
    }

    pub fn projection(ratio: f32, fov: f32, near: f32, far: f32) -> Self {
        let mut inner = [0.; 16];
        let s = 1. / (fov / 2.).tan();
        let l = near - far;
        inner[0] = s;
        inner[5] = s / ratio;
        inner[10] = (far + near) / l;
        inner[11] = 2. * near * far / l;
        inner[14] = 1.;
        inner[15] = 1.;
        Self {
            inner
        }
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut out = Self::default();
        for i in 0..4 {
            for j in 0..4 {
                let mut acc = 0.;
                for n in 0..4 {
                    acc += self.get(j, n) * rhs.get(n, i);
                }
                out.set(j, i, acc);
            }
        }
        out
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        let mut t = [0.; 4];
        for i in 0..4 {
            for j in 0..4 {
                t[i] += rhs.get(j) * self.get(i, j);
            }
        }
        Vector::from(t)
    }
}

impl Mul<Matrix> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Matrix) -> Self::Output {
        rhs * self
    }
}