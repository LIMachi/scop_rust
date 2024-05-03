use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    inner: [f32; 4]
}

impl <const S: usize> Into<[f32; S]> for Vector {
    fn into(self) -> [f32; S] {
        let mut arr = [0.; S];
        for i in 0..S.min(4) {
            arr[i] = self.inner[i];
        }
        arr
    }
}

impl <const S: usize> From<[f32; S]> for Vector {
    fn from(value: [f32; S]) -> Self {
        let mut out = Self::default();
        for i in 0..S.min(4) {
            out.inner[i] = value[i];
        }
        out
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        self.scale(rhs)
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs.scale(self)
    }
}

impl MulAssign<f32> for Vector {
    fn mul_assign(&mut self, rhs: f32) {
        self.scale(rhs);
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..4 {
            self.inner[i] += rhs.inner[i];
        }
        self
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..4 {
            self.inner[i] += rhs.inner[i];
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for i in 0..4 {
            self.inner[i] -= rhs.inner[i];
        }
        self
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..4 {
            self.inner[i] -= rhs.inner[i];
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(mut self) -> Self::Output {
        for i in 0..4 {
            self.inner[i] = -self.inner[i];
        }
        self
    }
}

impl Mul for Vector {
    type Output = Vector;

    fn mul(mut self, rhs: Self) -> Self::Output {
        for i in 0..4 {
            self.inner[i] *= rhs.inner[i];
        }
        self
    }
}

impl Div for Vector {
    type Output = Vector;

    fn div(mut self, rhs: Self) -> Self::Output {
        for i in 0..4 {
            self.inner[i] /= rhs.inner[i];
        }
        self
    }
}

impl Vector {
    pub const X: Vector = Vector {
        inner: [1., 0., 0., 0.]
    };

    pub const Y: Vector = Vector {
        inner: [0., 1., 0., 0.]
    };

    pub const Z: Vector = Vector {
        inner: [0., 0., 1., 0.]
    };

    pub const W: Vector = Vector {
        inner: [0., 0., 0., 1.]
    };
    
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            inner: [x, y, z, w]
        }
    }
    
    pub fn splat(value: f32) -> Self {
        Self {
            inner: [value, value, value, value]
        }
    }
    
    pub fn dot(&self, other: &Self) -> f32 {
        let mut acc = 0.;
        for i in 0..4 {
            acc += self.inner[i] * other.inner[i];
        }
        acc
    }
    
    ///implemented as if vector of size 3, not 4 (w will be set to 0)
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            inner: [
                self.inner[1] * other.inner[2] - self.inner[2] * other.inner[1],
                self.inner[2] * other.inner[0] - self.inner[0] * other.inner[2],
                self.inner[0] * other.inner[1] - self.inner[1] * other.inner[0],
                0.
            ]
        }
    }
    
    pub fn len_sqr(&self) -> f32 {
        self.dot(self)
    }
    
    pub fn len(&self) -> f32 {
        self.len_sqr().sqrt()
    }
    
    pub fn scale(mut self, scale: f32) -> Self {
        for i in 0..4 {
            self.inner[i] *= scale;
        }
        self
    }
    
    pub fn normalize(self) -> Self {
        let sqr = self.len_sqr();
        if sqr != 1. && sqr != 0. {
            self.scale(1. / sqr.sqrt())
        } else {
            self
        }
    }
    
    pub fn x(&self) -> f32 {
        self.inner[0]
    }

    pub fn y(&self) -> f32 {
        self.inner[1]
    }

    pub fn z(&self) -> f32 {
        self.inner[2]
    }

    pub fn w(&self) -> f32 {
        self.inner[3]
    }
    
    pub fn get(&self, row: usize) -> f32 {
        assert!(row < 4, "Invalid row access {row}, vector is 4");
        self.inner[row]
    }

    pub fn set_x(&mut self, value: f32) -> &mut Self {
        self.inner[0] = value;
        self
    }

    pub fn set_y(&mut self, value: f32) -> &mut Self {
        self.inner[1] = value;
        self
    }

    pub fn set_z(&mut self, value: f32) -> &mut Self {
        self.inner[2] = value;
        self
    }

    pub fn set_w(&mut self, value: f32) -> &mut Self {
        self.inner[3] = value;
        self
    }

    pub fn set(&mut self, row: usize, value: f32) -> &mut Self {
        assert!(row < 4, "Invalid row access {row}, vector is 4");
        self.inner[row] = value;
        self
    }
    
    pub fn array(&self) -> [f32; 4] {
        self.inner
    }
    
    pub fn min(&self, other: &Self) -> Self {
        Self {
            inner: [
                self.inner[0].min(other.inner[0]),
                self.inner[1].min(other.inner[1]),
                self.inner[2].min(other.inner[2]),
                self.inner[3].min(other.inner[3])
            ]
        }
    }

    pub fn max(&self, other: &Self) -> Self {
        Self {
            inner: [
                self.inner[0].max(other.inner[0]),
                self.inner[1].max(other.inner[1]),
                self.inner[2].max(other.inner[2]),
                self.inner[3].max(other.inner[3])
            ]
        }
    }
}