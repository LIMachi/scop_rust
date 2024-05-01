use crate::maths::matrix::Matrix;
use crate::maths::quat::Quat;
use crate::maths::vector::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub pos: Vector,
    pub rot: Quat,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: Vector::default(),
            rot: Quat::identity()
        }
    }
}

impl Camera {
    pub fn view(&self) -> Matrix {
        Matrix::from_pos(&-self.pos) * Matrix::from(self.rot)
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