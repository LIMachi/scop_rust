use winit::event::VirtualKeyCode;
use crate::maths::matrix::Matrix;
use crate::maths::quat::Quat;
use crate::maths::vector::Vector;
use crate::opengl::camera::Camera;
use crate::other::input_handler::InputHandler;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum Inputs {
    Forward,
    Backward,
    Right,
    Left,
    Up,
    Down,
    RightRoll,
    LeftRoll,
    RightPitch,
    LeftPitch,
    UpYaw,
    DownYaw,
    ToggleRotation
}

impl Inputs {
    pub fn default_handler() -> InputHandler<Self> {
        let mut out = InputHandler::<Self>::default();
        out.map(Self::Forward, VirtualKeyCode::W);
        out.map(Self::Backward, VirtualKeyCode::S);
        out.map(Self::Right, VirtualKeyCode::D);
        out.map(Self::Left, VirtualKeyCode::A);
        out.map(Self::Up, VirtualKeyCode::Space);
        out.map(Self::Down, VirtualKeyCode::LShift);
        out.map(Self::RightRoll, VirtualKeyCode::E);
        out.map(Self::LeftRoll, VirtualKeyCode::Q);
        out.map(Self::RightPitch, VirtualKeyCode::Right);
        out.map(Self::LeftPitch, VirtualKeyCode::Left);
        out.map(Self::UpYaw, VirtualKeyCode::Up);
        out.map(Self::DownYaw, VirtualKeyCode::Down);
        out.map(Self::ToggleRotation, VirtualKeyCode::R);
        out
    }
    
    pub fn apply_to_camera(camera: &mut Camera, handler: &InputHandler<Self>) -> bool {
        let mut displacement = Vector::default();
        let mut rotation = Quat::default();
        let mat = Matrix::from(camera.rot);
        let up = mat * Vector::Y;
        let right = mat * Vector::X;
        let forward = mat * -Vector::Z;
        for input in handler.pressed() {
            match input {
                Inputs::Forward => displacement += forward,
                Inputs::Backward => displacement -= forward,
                Inputs::Right => displacement += right,
                Inputs::Left => displacement -= right,
                Inputs::Up => displacement += up,
                Inputs::Down => displacement -= up,
                Inputs::RightRoll => rotation *= Quat::from_axis_angle(forward, -1f32.to_radians()),
                Inputs::LeftRoll => rotation *= Quat::from_axis_angle(forward, 1f32.to_radians()),
                Inputs::RightPitch => rotation *= Quat::from_axis_angle(up, 1f32.to_radians()),
                Inputs::LeftPitch => rotation *= Quat::from_axis_angle(up, -1f32.to_radians()),
                Inputs::UpYaw => rotation *= Quat::from_axis_angle(right, -1f32.to_radians()),
                Inputs::DownYaw => rotation *= Quat::from_axis_angle(right, 1f32.to_radians()),
                _ => {}
            }
        }
        if displacement != Vector::default() || rotation != Quat::default() {
            camera.rot *= rotation;
            camera.pos += displacement;
            true
        } else {
            false
        }
    }
}