use crate::maths::matrix::{Mat4, Matrix};
use crate::maths::quaternion::Quaternion;
use crate::maths::vector::Vec3;

impl Mat4 {
    pub fn from_pos_rot_scale(pos: &Vec3, rot: &Quaternion, scale: &Vec3) -> Self {
        Self::from_pos(pos) * Self::from(*rot) * Self::from_scale(scale)
    }

    pub fn from_scale(scale: &Vec3) -> Self {
        Self::from([
            [scale[0], 0., 0., 0.],
            [0., scale[1], 0., 0.],
            [0., 0., scale[2], 0.],
            [0., 0., 0., 1.],
        ])
    }

    pub fn from_pos(pos: &Vec3) -> Self {
        Self::from([
            [1., 0., 0., pos[0]],
            [0., 1., 0., pos[1]],
            [0., 0., 1., pos[2]],
            [0., 0., 0., 1.],
        ])
    }

    pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Self {
        let s = 1. / (fov / 2.).tan();
        let l = near - far;
        Self::from([
            [s / ratio, 0., 0., 0.],
            [0., s, 0., 0.],
            [0., 0., (far + near) / l, 2. * near * far / l],
            [0., 0., -1., 1.],
        ])
    }

    pub fn orthographic(width: f32, ratio: f32, near: f32, far: f32) -> Self {
        let height = width / ratio;
        Self::from([
            [2. / width, 0., 0., 0.],
            [0., 2. / height, 0., 0.],
            [0., 0., 2. / (far - near), 0.],
            [0., 0., 0., 1.],
        ])
    }

    pub fn raw_array(&self) -> [f32; 16] {
        let mut out = [0f32; 16];
        for c in 0..4 {
            for r in 0..4 {
                out[r + c * 4] = self.0[r][c];
            }
        }
        out
    }
}