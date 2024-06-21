use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use super::Quaternion;

impl PartialEq for Quaternion {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.i == other.i && self.j == other.j && self.k == other.k
    }
}

impl PartialEq<f32> for Quaternion {
    fn eq(&self, other: &f32) -> bool {
        self.i == 0. && self.j == 0. && self.k == 0. && &self.r == other
    }
}

impl Add for Quaternion {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            i: self.i + rhs.i,
            j: self.j + rhs.j,
            k: self.k + rhs.k
        }
    }
}

impl AddAssign for Quaternion {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Quaternion {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r - rhs.r,
            i: self.i - rhs.i,
            j: self.j - rhs.j,
            k: self.k - rhs.k
        }
    }
}

impl SubAssign for Quaternion {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Neg for Quaternion {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            r: -self.r,
            i: -self.i,
            j: -self.j,
            k: -self.k
        }
    }
}

impl Mul for Quaternion {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r * rhs.r - self.i * rhs.i - self.j * rhs.j - self.k * rhs.k,
            i: self.r * rhs.i + self.i * rhs.r + self.j * rhs.k - self.k * rhs.j,
            j: self.r * rhs.j + self.j * rhs.r - self.i * rhs.k + self.k * rhs.i,
            k: self.r * rhs.k + self.k * rhs.r + self.i * rhs.j - self.j * rhs.i,
        }
    }
}

impl MulAssign for Quaternion {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul<f32> for Quaternion {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r * rhs,
            i: self.i * rhs,
            j: self.j * rhs,
            k: self.k * rhs,
        }
    }
}

impl MulAssign<f32> for Quaternion {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div for Quaternion {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let d = rhs.dot(&rhs);
        Self {
            r: (self.r * rhs.r + self.i * rhs.i + self.j * rhs.j + self.k * rhs.k) / d,
            i: (self.r * rhs.i - self.i * rhs.r - self.j * rhs.k + self.k * rhs.j) / d,
            j: (self.r * rhs.j + self.i * rhs.k - self.j * rhs.r - self.k * rhs.i) / d,
            k: (self.r * rhs.k - self.i * rhs.j + self.j * rhs.i - self.k * rhs.r) / d
        }
    }
}

impl DivAssign for Quaternion {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Div<f32> for Quaternion {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r / rhs,
            i: self.i / rhs,
            j: self.j / rhs,
            k: self.k / rhs,
        }
    }
}

impl DivAssign<f32> for Quaternion {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}