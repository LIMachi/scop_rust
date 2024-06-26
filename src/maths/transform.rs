use std::ops::{Add, Div, Mul, Sub};
use crate::maths::matrix::{Mat3, Mat4, Matrix};
use crate::maths::quaternion::Quaternion;
use crate::maths::Unit;
use crate::maths::vector::{Vec3, Vector};

#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub pos: Vec3,
    pub rot: Quaternion,
    pub scale: Vec3
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            pos: Vector::default(),
            rot: Quaternion::unit(),
            scale: Vector::splat(1.)
        }
    }
}

impl Transform {
    pub fn from_look_at(pos: Vec3, target: Vec3) -> Self {
        let rot = Quaternion::from_look_at(target - pos, Vec3::Y);
        Self {
            pos,
            rot,
            scale: Vector::splat(1.)
        }
    }
    
    pub fn from_look_towards(pos: Vec3, dir: Vec3) -> Self {
        let rot = Quaternion::from_look_at(dir, Vec3::Y);
        Self {
            pos,
            rot,
            scale: Vector::splat(1.)
        }
    }
    
    pub fn from_pos(pos: Vec3) -> Self {
        Self {
            pos,
            ..Self::default()
        }
    }
    
    pub fn as_view_matrix(&self) -> Mat4 {
        Matrix::from_scale(&self.scale) * Matrix::from(self.rot) * Matrix::from_pos(&-self.pos)
    }

    pub fn up(&self) -> Vec3 { Mat3::from(self.rot) * Vec3::Y }

    pub fn right(&self) -> Vec3 { Mat3::from(self.rot) * Vec3::X }

    pub fn forward(&self) -> Vec3 { Mat3::from(self.rot) * -Vec3::Z }

    pub fn move_local(&mut self, vec: Vec3) { self.pos += Mat3::from(self.rot) * vec; }

    pub fn move_absolute(&mut self, vec: Vec3) { self.pos += vec; }

    pub fn rotate_local(&mut self, axis: Vec3, angle: f32) {
        self.rot *= Quaternion::from((Mat3::from(self.rot) * axis, angle));
    }

    pub fn rotate_absolute(&mut self, axis: Vec3, angle: f32) {
        self.rot *= Quaternion::from((axis, angle));
    }
}

impl From<Transform> for Mat4 {
    fn from(value: Transform) -> Self {
        Self::from_pos_rot_scale(&value.pos, &value.rot, &value.scale)
    }
}

impl From<&Transform> for Mat4 {
    fn from(value: &Transform) -> Self {
        Self::from_pos_rot_scale(&value.pos, &value.rot, &value.scale)
    }
}

impl Add<Vec3> for Transform {
    type Output = Transform;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            pos: self.pos + rhs,
            rot: self.rot,
            scale: self.scale
        }
    }
}

impl Sub<Vec3> for Transform {
    type Output = Transform;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {
            pos: self.pos - rhs,
            rot: self.rot,
            scale: self.scale
        }
    }
}

impl Mul<Vec3> for Transform {
    type Output = Transform;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self {
            pos: self.pos,
            rot: self.rot,
            scale: self.scale * rhs
        }
    }
}

impl Mul<f32> for Transform {
    type Output = Transform;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            pos: self.pos,
            rot: self.rot,
            scale: self.scale * rhs
        }
    }
}

impl Div<Vec3> for Transform {
    type Output = Transform;

    fn div(self, rhs: Vec3) -> Self::Output {
        Self {
            pos: self.pos,
            rot: self.rot,
            scale: self.scale / rhs
        }
    }
}

impl Div<f32> for Transform {
    type Output = Transform;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            pos: self.pos,
            rot: self.rot,
            scale: self.scale / rhs
        }
    }
}

impl Mul<Quaternion> for Transform {
    type Output = Transform;

    fn mul(self, rhs: Quaternion) -> Self::Output {
        Self {
            pos: self.pos,
            rot: self.rot * rhs,
            scale: self.scale
        }
    }
}

impl Div<Quaternion> for Transform {
    type Output = Transform;

    fn div(self, rhs: Quaternion) -> Self::Output {
        Self {
            pos: self.pos,
            rot: self.rot / rhs,
            scale: self.scale
        }
    }
}