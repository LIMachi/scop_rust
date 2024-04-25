use std::ops::{Add, Div, Mul, Sub};
#[derive(Debug, Clone)]
pub struct Vector<const S: usize, T: Copy> {
    inner: [T; S]
}

impl <const S: usize, T: Copy> Vector<S, T> {
    pub fn new(array: [T; S]) -> Self {
        Self {
            inner: array.clone()
        }
    }
    
    pub fn array(&self) -> [T; S] {
        self.inner.clone()
    }
}

impl <const S: usize, T: Copy + Add<T, Output = T> + Mul<T, Output = T>> Vector<S, T> {
    pub fn dot(self, rhs: Self) -> T {
        let mut acc = self.inner[0] * rhs.inner[0];
        for i in 1..S {
            acc = acc + self.inner[i] * rhs.inner[i];
        }
        acc
    }
    
    pub fn len_sqr(&self) -> T {
        let mut acc = self.inner[0] * self.inner[0];
        for i in 1..S {
            acc = acc + self.inner[i] * self.inner[i];
        }
        acc
    }
    
    pub fn scale(mut self, mul: T) -> Self {
        for i in 0..S {
            self.inner[i] = self.inner[i] * mul;
        }
        self
    }
}

impl <const S: usize> Vector<S, f32> {
    pub fn len(&self) -> f32 {
        self.len_sqr().sqrt()
    }
    
    pub fn normalize(self) -> Self {
        let t = self.len_sqr();
        if t != 1. && t != 0. {
            self.scale(1. / t.sqrt())
        } else {
            self
        }
    }
}

impl <const S: usize, T: Copy + Add<T, Output = T>> Add<Vector<S, T>> for Vector<S, T> {
    type Output = Vector<S, T>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = [self.inner[0]; S];
        for i in 0..S {
            out[i] = self.inner[i] + rhs.inner[i];
        }
        Self::new(out)
    }
}

impl <const S: usize, T: Copy + Sub<T, Output = T>> Sub<Vector<S, T>> for Vector<S, T> {
    type Output = Vector<S, T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut out = [self.inner[0]; S];
        for i in 0..S {
            out[i] = self.inner[i] - rhs.inner[i];
        }
        Self::new(out)
    }
}



impl <const S: usize, T: Copy + Default> Vector<S, T> {
    pub fn x(&self) -> T {
        if S > 0 {
            self.inner[0]
        } else {
            T::default()
        }
    }

    pub fn y(&self) -> T {
        if S > 1 {
            self.inner[1]
        } else {
            T::default()
        }
    }

    pub fn z(&self) -> T {
        if S > 2 {
            self.inner[2]
        } else {
            T::default()
        }
    }

    pub fn w(&self) -> T {
        if S > 3 {
            self.inner[3]
        } else {
            T::default()
        }
    }

    pub fn set_x(&mut self, value: T) {
        if S > 0 {
            self.inner[0] = value;
        }
    }

    pub fn set_y(&mut self, value: T) {
        if S > 1 {
            self.inner[1] = value;
        }
    }

    pub fn set_z(&mut self, value: T) {
        if S > 2 {
            self.inner[2] = value;
        }
    }

    pub fn set_w(&mut self, value: T) {
        if S > 3 {
            self.inner[3] = value;
        }
    }
}

impl <const S: usize, T: Copy + Default> Default for Vector<S, T> {
    fn default() -> Self {
        Self {
            inner: [T::default(); S]
        }
    }
}