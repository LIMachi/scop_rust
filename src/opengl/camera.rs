use crate::maths::matrix::Matrix;
use crate::maths::quat::Quat;
use crate::maths::vector::Vector;

#[derive(Default, Debug, Copy, Clone)]
pub struct Camera {
    pub pos: Vector,
    pub rot: Quat,
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
        Matrix::from(self.rot) * Vector::Z
    }
}