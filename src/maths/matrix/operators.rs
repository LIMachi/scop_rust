use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};
use crate::maths::vector::Vector;
use super::Matrix;

impl <const C: usize, const R: usize, K> Index<usize> for Matrix<C, R, K> {
    type Output = Vector<C, K>;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < R, "Invalid row index {index} for matrix of size {C} (columns) * {R} (rows)");
        &self.0[index]
    }
}

impl <const C: usize, const R: usize, K> IndexMut<usize> for Matrix<C, R, K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < R, "Invalid column index {index} for matrix of size {C} (columns) * {R} (rows)");
        &mut self.0[index]
    }
}

impl <const C: usize, const R: usize, K> Index<(usize, usize)> for Matrix<C, R, K> {
    type Output = K;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(index.0 < C, "Invalid column index {} for matrix of size {C} (columns) * {R} (rows)", index.0);
        assert!(index.1 < R, "Invalid row index {} for matrix of size {C} (columns) * {R} (rows)", index.1);
        &self.0[index.1][index.0]
    }
}

impl <const C: usize, const R: usize, K> IndexMut<(usize, usize)> for Matrix<C, R, K> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert!(index.0 < C, "Invalid column index {} for matrix of size {C} (columns) * {R} (rows)", index.0);
        assert!(index.1 < R, "Invalid row index {} for matrix of size {C} (columns) * {R} (rows)", index.1);
        &mut self.0[index.1][index.0]
    }
}

impl <const C: usize, const R: usize, K: Add<Output = K> + Copy> Add<K> for Matrix<C, R, K> {
    type Output = Self;

    fn add(mut self, rhs: K) -> Self::Output {
        for r in 0..R {
            for c in 0..C {
                self[(c, r)] = self[(c, r)] + rhs;
            }
        }
        self
    }
}

impl <const C: usize, const R: usize, K: Add<Output = K> + Copy> Add<Vector<C, K>> for Matrix<C, R, K> {
    type Output = Self;

    fn add(mut self, rhs: Vector<C, K>) -> Self::Output {
        for r in 0..R {
            self[r] = self[r] + rhs;
        }
        self
    }
}

impl <const C: usize, const R: usize, K: Sub<Output = K> + Copy> Sub<K> for Matrix<C, R, K> {
    type Output = Self;

    fn sub(mut self, rhs: K) -> Self::Output {
        for r in 0..R {
            for c in 0..C {
                self[(c, r)] = self[(c, r)] - rhs;
            }
        }
        self
    }
}

impl <const C: usize, const R: usize, K: Sub<Output = K> + Copy> Sub<Vector<C, K>> for Matrix<C, R, K> {
    type Output = Self;

    fn sub(mut self, rhs: Vector<C, K>) -> Self::Output {
        for r in 0..R {
            self[r] = self[r] - rhs;
        }
        self
    }
}

impl <const C: usize, const R: usize, K: Mul<Output = K> + Copy> Mul<K> for Matrix<C, R, K> {
    type Output = Self;

    fn mul(mut self, rhs: K) -> Self::Output {
        for r in 0..R {
            for c in 0..C {
                self[(c, r)] = self[(c, r)] * rhs;
            }
        }
        self
    }
}

impl <const C: usize, const R: usize, K: Div<Output = K> + Copy> Div<K> for Matrix<C, R, K> {
    type Output = Self;

    fn div(mut self, rhs: K) -> Self::Output {
        for r in 0..R {
            for c in 0..C {
                self[(c, r)] = self[(c, r)] / rhs;
            }
        }
        self
    }
}

impl <const C: usize, const R: usize, K: PartialEq> PartialEq for Matrix<C, R, K> {
    fn eq(&self, other: &Self) -> bool {
        for r in 0..R {
            for c in 0..C {
                if self[(c, r)] != other[(c, r)] {
                    return false;
                }
            }
        }
        true
    }
}

impl <const C: usize, const R: usize, K: Default + Copy + Mul<Output = K> + Add<Output = K>> Mul<Vector<R, K>> for Matrix<C, R, K> {
    type Output = Vector<C, K>;

    fn mul(self, rhs: Vector<R, K>) -> Self::Output {
        let mut out = Vector::<C, K>::default();
        for i in 0..R {
            for n in 0..C {
                out[i] = out[i] + rhs[n] * self[(n, i)];
            }
        }
        out
    }
}

impl <const C: usize, const R: usize, const P: usize, K: Default + Copy + Mul<Output = K> + Add<Output = K>> Mul<Matrix<P, C, K>> for Matrix<C, R, K> {
    type Output = Matrix<P, R, K>;

    fn mul(self, rhs: Matrix<P, C, K>) -> Self::Output {
        let mut out = Matrix::<P, R, K>::default();
        for p in 0..P {
            for c in 0..C {
                for r in 0..R {
                    out[(p, r)] = out[(p, r)] + self[(c, r)] * rhs[(p, c)];
                }
            }
        }
        out
    }
}