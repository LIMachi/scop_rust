use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Index, Mul, Sub};
use crate::maths::{Root2, Unit};
use super::Vector;

impl <const S: usize, K: Default + Copy> Default for Vector<S, K> {
    fn default() -> Self {
        Self([K::default(); S])
    }
}

impl <const S: usize, K: Debug> Debug for Vector<S, K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl <const S: usize, K: Display> Display for Vector<S, K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        for i in 0..S - 1 {
            f.write_fmt(format_args!("{}, ", self.0[i]))?;
        }
        f.write_fmt(format_args!("{}]", self.0[S - 1]))
    }
}

impl <const S: usize, K> Vector<S, K> {
    pub fn size() -> usize { S }
}

impl <const S: usize, K: Clone> Clone for Vector<S, K>{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl <const S: usize, K: Copy> Copy for Vector<S, K>{}

impl <const S: usize, K: Copy + Default> Vector<S, K> {
    pub fn swizzle<const L: usize, I: Copy>(&self, indexes: [I; L]) -> Vector<L, K> where Vector<S, K>: Index<I, Output = K> {
        let mut out = Vector::<L, K>::default();
        for l in 0..L {
            out[l] = self[indexes[l]];
        }
        out
    }
}

impl <const S: usize, K: Root2 + Copy + Default + Unit + PartialEq + Add<Output = K> + Mul<Output = K> + Div<Output = K>> Vector<S, K> {
    pub fn normalize(&self) -> Self {
        let l = self.dot(&self);
        if l != K::unit() && l != K::default() {
            *self / l.root2()
        } else {
            *self
        }
    }
}

impl <const S: usize, K: Copy + PartialOrd> Vector<S, K> {
    pub fn min(&self, mut rhs: Self) -> Self {
        for i in 0..S {
            if self.0[i] < rhs.0[i] {
                rhs.0[i] = self.0[i];
            } 
        }
        rhs
    }

    pub fn max(&self, mut rhs: Self) -> Self {
        for i in 0..S {
            if self.0[i] > rhs.0[i] {
                rhs.0[i] = self.0[i];
            }
        }
        rhs
    }
}

impl <const S: usize, K: Copy> Vector<S, K> {
    pub fn splat(value: K) -> Self {
        Self([value; S])
    }
}

impl <K: Mul<Output = K> + Sub<Output = K> + Copy> Vector<3, K> {
    pub fn cross_product(&self, v: &Self) -> Self {
        Vector::from([
            self.0[1] * v.0[2] - self.0[2] * v.0[1],
            self.0[2] * v.0[0] - self.0[0] * v.0[2],
            self.0[0] * v.0[1] - self.0[1] * v.0[0],
        ])
    }
}

impl <const S: usize, K: Add<Output = K> + Mul<Output = K> + Copy + Default> Vector<S, K> {
    pub fn dot(&self, other: &Self) -> K {
        let mut acc = K::default();
        for i in 0..S {
            acc = acc + self[i] * other[i];
        }
        acc
    }
}