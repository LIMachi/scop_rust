use std::env;
use gl::types::GLsizei;
use gl::Viewport;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::window::WindowBuilder;
use crate::maths::matrix::Matrix;
use crate::maths::quat::Quat;
use crate::maths::vector::Vector;
use crate::opengl::camera::Camera;
use crate::opengl::object::Object;
use crate::opengl::safe_calls;
use crate::opengl::shader::ShaderProgram;
use crate::other::input_handler::InputHandler;
use crate::other::resource_manager::ResourceManager;
use crate::other::window;

mod structures;
mod parser;
mod opengl;
mod maths;
mod other;

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


fn main() {
    if env::args().len() != 2 {
        println!("expected exactly 1 argument");
        return;
    }
    let model = env::args().last().unwrap();
    let mut resources = ResourceManager::default();
    resources.register_hints(&["resources", "resources/objs", "resources/textures", "resources/shaders"]);
    if let Some(object) = resources.load_object(model).cloned() {
        if let Some((ctx, event_loop)) = window::spawn_single_window(WindowBuilder::new()
            .with_title("Scop")
            .with_visible(true)
        ) {
            let mut program = ShaderProgram::from_resources(&mut resources, "triangle").unwrap();
            let mut object = Object::new(&mut resources, &object);
            //load object in graphics
            //load vbos
            //load textures
            //load uniforms

            let mut inputs = InputHandler::<Inputs>::default();

            inputs.map(Inputs::Forward, VirtualKeyCode::W);
            inputs.map(Inputs::Backward, VirtualKeyCode::S);
            inputs.map(Inputs::Right, VirtualKeyCode::D);
            inputs.map(Inputs::Left, VirtualKeyCode::A);
            inputs.map(Inputs::Up, VirtualKeyCode::Space);
            inputs.map(Inputs::Down, VirtualKeyCode::LShift);
            inputs.map(Inputs::RightRoll, VirtualKeyCode::E);
            inputs.map(Inputs::LeftRoll, VirtualKeyCode::Q);
            inputs.map(Inputs::RightPitch, VirtualKeyCode::Right);
            inputs.map(Inputs::LeftPitch, VirtualKeyCode::Left);
            inputs.map(Inputs::UpYaw, VirtualKeyCode::Up);
            inputs.map(Inputs::DownYaw, VirtualKeyCode::Down);
            inputs.map(Inputs::ToggleRotation, VirtualKeyCode::R);

            safe_calls::set_clear_color(0., 0.5, 0.2);
            safe_calls::set_depth_test(true);

            object.bind();

            let size = ctx.window().inner_size();
            let proj = Matrix::projection(size.height as f32 / size.width as f32, 90f32.to_radians(), 0.1, 10000.);
            program.set_mat("proj", proj);
            let mut camera = Camera::default();
            program.set_mat("camera", camera.view());
            program.set_mat("object", Matrix::identity());

            let mut timer = std::time::Instant::now();

            event_loop.run(move |event, _target, control_flow| {
                match event {
                    Event::WindowEvent {
                        window_id, event,
                    } if ctx.window().id() == window_id => {
                        if let WindowEvent::KeyboardInput { input, .. } = event {
                            inputs.key_event(input);
                        }
                        if let WindowEvent::Resized(size) = event {
                            safe_calls::resize(size.width, size.height);
                            let proj = Matrix::projection(size.height as f32 / size.width as f32, 90f32.to_radians(), 0.1, 10000.);
                            program.set_mat("proj", proj);
                        }
                        if event == WindowEvent::CloseRequested {
                            *control_flow = ControlFlow::Exit
                        }
                    }
                    Event::MainEventsCleared => {
                        let elapsed = timer.elapsed();
                        if elapsed.as_secs_f64() >= 1. / 60. {
                            timer = std::time::Instant::now();
                            for key in inputs.pressed() {
                                match key {
                                    Inputs::Forward => { camera.pos -= camera.forward(); }
                                    Inputs::Backward => { camera.pos += camera.forward(); }
                                    Inputs::Right => { camera.pos += camera.right(); }
                                    Inputs::Left => { camera.pos -= camera.right(); }
                                    Inputs::Up => { camera.pos += camera.up(); }
                                    Inputs::Down => { camera.pos -= camera.up(); }
                                    Inputs::RightRoll => { camera.rot *= Quat::from_axis_angle(camera.forward(), 1f32.to_radians()); }
                                    Inputs::LeftRoll => { camera.rot *= Quat::from_axis_angle(camera.forward(), -1f32.to_radians()); }
                                    Inputs::RightPitch => { camera.rot *= Quat::from_axis_angle(camera.up(), 1f32.to_radians()); }
                                    Inputs::LeftPitch => { camera.rot *= Quat::from_axis_angle(camera.up(), -1f32.to_radians()); }
                                    Inputs::UpYaw => { camera.rot *= Quat::from_axis_angle(camera.right(), -1f32.to_radians()); }
                                    Inputs::DownYaw => { camera.rot *= Quat::from_axis_angle(camera.right(), 1f32.to_radians()); }
                                    // Inputs::ToggleRotation => { if inputs.status(Inputs::ToggleRotation).just_pressed() { do_rotate = !do_rotate; }}
                                    _ => {}
                                }
                            }
                            program.set_mat("camera", camera.view());
                            safe_calls::clear_screen();
                            object.draw();
                            ctx.swap_buffers().unwrap();
                            inputs.tick();
                        }
                    }
                    _ => {}
                }
            });
        }
    }
}