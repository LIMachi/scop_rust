use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use crate::maths::matrix::Matrix;
use super::Vector;

impl <const S: usize, K> Index<usize> for Vector<S, K> {
    type Output = K;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < S, "Invalid index {index} for vector of size {S}");
        &self.0[index]
    }
}

impl <const S: usize, K> IndexMut<usize> for Vector<S, K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < S, "Invalid index {index} for vector of size {S}");
        &mut self.0[index]
    }
}

impl <const S: usize, K> Index<char> for Vector<S, K> {
    type Output = K;

    fn index(&self, index: char) -> &Self::Output {
        match index {
            'x' if S > 0 => &self.0[0],
            'y' if S > 1 => &self.0[1],
            'z' if S > 2 => &self.0[2],
            'w' if S > 3 => &self.0[3],
            _ => panic!("Invalid label access {index} for vector of size {S}"),
        }
    }
}

impl <const S: usize, K> IndexMut<char> for Vector<S, K> {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        match index {
            'x' if S > 0 => &mut self.0[0],
            'y' if S > 1 => &mut self.0[1],
            'z' if S > 2 => &mut self.0[2],
            'w' if S > 3 => &mut self.0[3],
            _ => panic!("Invalid label access {index} for vector of size {S}"),
        }
    }
}

impl <const S: usize, K: Add<Output = K> + Copy> Add<K> for Vector<S, K> {
    type Output = Self;

    fn add(mut self, rhs: K) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] + rhs;
        }
        self
    }
}

impl <const S: usize, K: Add<Output = K> + Copy> AddAssign<K> for Vector<S, K> {
    fn add_assign(&mut self, rhs: K) {
        *self = *self + rhs;
    }
}

impl <const S: usize, K: Sub<Output = K> + Copy> Sub<K> for Vector<S, K> {
    type Output = Self;

    fn sub(mut self, rhs: K) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] - rhs;
        }
        self
    }
}

impl <const S: usize, K: Sub<Output = K> + Copy> SubAssign<K> for Vector<S, K> {
    fn sub_assign(&mut self, rhs: K) {
        *self = *self - rhs;
    }
}

impl <const S: usize, K: Neg<Output=K> + Copy> Neg for Vector<S, K> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for i in 0..S {
            self[i] = -self[i];
        }
        self
    }
}

impl <const S: usize, K: Mul<Output = K> + Copy> Mul for Vector<S, K> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] * rhs[i];
        }
        self
    }
}

impl <const S: usize, K: Mul<Output = K> + Copy> MulAssign for Vector<S, K> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl <const S: usize, K: Mul<Output = K> + Copy> Mul<K> for Vector<S, K> {
    type Output = Self;

    fn mul(mut self, rhs: K) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] * rhs;
        }
        self
    }
}

impl <const S: usize, K: Mul<Output = K> + Copy> MulAssign<K> for Vector<S, K> {
    fn mul_assign(&mut self, rhs: K) {
        *self = *self * rhs;
    }
}

impl <const S: usize, K: Div<Output = K> + Copy> Div for Vector<S, K> {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] / rhs[i];
        }
        self
    }
}

impl <const S: usize, K: Div<Output = K> + Copy> DivAssign for Vector<S, K> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl <const S: usize, K: Div<Output = K> + Copy> Div<K> for Vector<S, K> {
    type Output = Self;

    fn div(mut self, rhs: K) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] / rhs;
        }
        self
    }
}

impl <const S: usize, K: Div<Output = K> + Copy> DivAssign<K> for Vector<S, K> {
    fn div_assign(&mut self, rhs: K) {
        *self = *self / rhs;
    }
}

impl <const S: usize, K: PartialEq> PartialEq for Vector<S, K> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..S {
            if self.0[i] != other.0[i] {
                return false;
            }
        }
        true
    }
}

impl <const S: usize, K: Add<Output = K> + Copy> Add for Vector<S, K> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] + rhs[i];
        }
        self
    }
}

impl <const S: usize, K: Add<Output = K> + Copy> AddAssign for Vector<S, K> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl <const S: usize, K: Sub<Output = K> + Copy> Sub for Vector<S, K> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] - rhs[i];
        }
        self
    }
}

impl <const S: usize, K: Sub<Output = K> + Copy> SubAssign for Vector<S, K> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}