use winit::event::VirtualKeyCode;
use crate::maths::matrix::{Mat3, Matrix};
use crate::maths::quaternion::Quaternion;
use crate::maths::transform::Transform;
use crate::maths::Unit;
use crate::maths::vector::{Vec3, Vector};
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
    ToggleRotation,
    ToggleSpeedUp,
    ToggleFade,
    ToggleMode
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
        out.map(Self::ToggleSpeedUp, VirtualKeyCode::LControl);
        out.map(Self::ToggleFade, VirtualKeyCode::F);
        out.map(Self::ToggleMode, VirtualKeyCode::M);
        out
    }
    
    pub fn apply_to_camera(camera: &mut Transform, handler: &InputHandler<Self>, speed_up: bool) -> bool {
        let mut displacement = Vector::default();
        let mut rotation = Quaternion::unit();
        let mat = Mat3::from(camera.rot.inverse());
        let up = mat * Vec3::Y;
        let right = mat * Vec3::X;
        let forward = mat * -Vec3::Z;
        let speed = if speed_up { 2f32 } else { 0.25f32 };
        let angular = if speed_up { 5f32 } else { 2f32 };
        for input in handler.pressed() {
            match input {
                Inputs::Forward => displacement += forward * speed,
                Inputs::Backward => displacement -= forward * speed,
                Inputs::Right => displacement += right * speed,
                Inputs::Left => displacement -= right * speed,
                Inputs::Up => displacement += up * speed,
                Inputs::Down => displacement -= up * speed,
                Inputs::RightRoll => rotation *= Quaternion::from((Vec3::Z, -angular.to_radians())),
                Inputs::LeftRoll => rotation *= Quaternion::from((Vec3::Z, angular.to_radians())),
                Inputs::RightPitch => rotation *= Quaternion::from((Vec3::Y,-angular.to_radians())),
                Inputs::LeftPitch => rotation *= Quaternion::from((Vec3::Y, angular.to_radians())),
                Inputs::UpYaw => rotation *= Quaternion::from((Vec3::X, angular.to_radians())),
                Inputs::DownYaw => rotation *= Quaternion::from((Vec3::X, -angular.to_radians())),
                _ => {}
            }
        }
        if displacement != Vector::default() || rotation != Quaternion::unit() {
            camera.rot *= rotation;
            camera.pos += displacement;
            true
        } else {
            false
        }
    }
}