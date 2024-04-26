use crate::structures::matrix::Matrix;
use crate::structures::quat::Quat;
use crate::structures::vector::Vector;

#[derive(Default, Debug, Copy, Clone)]
pub struct Camera {
    pub pos: Vector,
    pub rot: Quat,
}

impl Camera {
    pub fn view(&self) -> Matrix {
        self.rot * Matrix::from_pos(&-self.pos)
    }

    pub fn up(&self) -> Vector {
        self.rot * Vector::Y
    }

    pub fn right(&self) -> Vector {
        self.rot * Vector::X
    }

    pub fn forward(&self) -> Vector {
        self.rot * Vector::Z
    }
}