use crate::maths::matrix::Matrix;
use crate::maths::quat::Quat;
use crate::maths::vector::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub pos: Vector,
    pub rot: Quat,
    pub scale: Vector
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            pos: Vector::default(),
            rot: Quat::identity(),
            scale: Vector::splat(1.)
        }
    }
}

impl Transform {
    pub fn from_look_at(pos: Vector, target: Vector) -> Self {
        let rot = Quat::from_look_at((target - pos).normalize(), Vector::Y);
        Self {
            pos,
            rot,
            scale: Vector::splat(1.)
        }
    }
    
    pub fn from_pos(pos: Vector) -> Self {
        Self {
            pos,
            ..Self::default()
        }
    }
    
    pub fn as_view_matrix(&self) -> Matrix {
        Matrix::from_pos(&-self.pos) * Matrix::from(self.rot) * Matrix::from_scale(&self.scale)
    }

    pub fn up(&self) -> Vector {
        Matrix::from(self.rot) * Vector::Y
    }

    pub fn right(&self) -> Vector {
        Matrix::from(self.rot) * Vector::X
    }

    pub fn forward(&self) -> Vector {
        Matrix::from(self.rot) * -Vector::Z
    }

    pub fn move_local(&mut self, vec: Vector) {
        self.pos += Matrix::from(self.rot) * vec;
    }

    pub fn move_absolute(&mut self, vec: Vector) {
        self.pos += vec;
    }

    pub fn rotate_local(&mut self, axis: Vector, angle: f32) {
        self.rot *= Quat::from_axis_angle(Matrix::from(self.rot) * axis, angle);
    }

    pub fn rotate_absolute(&mut self, axis: Vector, angle: f32) {
        self.rot *= Quat::from_axis_angle(axis, angle);
    }
}

impl From<Transform> for Matrix {
    fn from(value: Transform) -> Self {
        Self::from_pos_rot_scale(&value.pos, &value.rot, &value.scale)
    }
}