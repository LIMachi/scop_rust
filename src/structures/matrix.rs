use std::fmt::Debug;
use std::ops::{Add, Mul, Sub};
use crate::structures::vector::Vector;

#[derive(Debug, Clone)]
pub struct Mat<const C: usize, const R: usize, T> {
    inner: [[T; R]; C]
}

impl <const C: usize, const R: usize, T: Copy> Mat<C, R, T> {
    pub fn new(array: [[T; R]; C]) -> Self {
        Self {
            inner: array.clone()
        }
    }
    
    pub fn get(&self, col: usize, row: usize) -> T {
        assert!(col < C, "Invalid column index {} for matrix of size {}", col, C);
        assert!(row < R, "Invalid row index {} for matrix of size {}", row, R);
        self.inner[col][row]
    }
    
    pub fn set(&mut self, col: usize, row: usize, value: T) -> &mut Self {
        assert!(col < C, "Invalid column index {} for matrix of size {}", col, C);
        assert!(row < R, "Invalid row index {} for matrix of size {}", row, R);
        self.inner[col][row] = value;
        self
    }
    
    pub fn array(&self) -> [[T; R]; C] {
        self.inner.clone()
    }
    
    pub fn column(&self, col: usize) -> Vector<R, T> {
        assert!(col < C, "Invalid column index {} for matrix of size {}", col, C);
        Vector::new(self.inner[col])
    }

    pub fn row(&self, row: usize) -> Vector<C, T> {
        assert!(row < R, "Invalid row index {} for matrix of size {}", row, R);
        let mut t = [self.inner[0][0]; C];
        for i in 1..C {
            t[i] = self.inner[i][row];
        }
        Vector::new(t)
    }
}

impl <const C: usize, const R: usize> Mat<C, R, f32> {
    pub fn rot_x(rad: f32) -> Self {
        let mut out = Self::default();
        if C > 2 && R > 2 {
            let cos = rad.cos();
            let sin = rad.sin();
            out.inner[1][1] = cos;
            out.inner[2][1] = -sin;
            out.inner[1][2] = sin;
            out.inner[2][2] = cos;
        }
        out
    }

    pub fn rot_y(rad: f32) -> Self {
        let mut out = Self::default();
        if C > 2 && R > 2 {
            let cos = rad.cos();
            let sin = rad.sin();
            out.inner[0][0] = cos;
            out.inner[2][0] = sin;
            out.inner[0][2] = -sin;
            out.inner[2][2] = cos;
        }
        out
    }

    pub fn rot_z(rad: f32) -> Self {
        let mut out = Self::default();
        if C > 1 && R > 1 {
            let cos = rad.cos();
            let sin = rad.sin();
            out.inner[0][0] = cos;
            out.inner[1][0] = -sin;
            out.inner[0][1] = sin;
            out.inner[1][1] = cos;
        }
        out
    }
}

impl <const C: usize, const R: usize> Default for Mat<C, R, f32> {
    fn default() -> Self {
        let mut out = Self::new([[0.; R]; C]);
        for i in 0..C.min(R) {
            out.set(i, i, 1.);
        }
        out
    }
}