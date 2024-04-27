mod parser;
mod structures;
mod window;
mod shader;
mod safe_calls;

extern crate gl;

use std::fs::File;
use std::io::Read;
use gl::*;
use gl::types::GLsizei;

use glutin::{ContextWrapper, PossiblyCurrent};
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    window::WindowBuilder,
};
use winit::event::VirtualKeyCode;
use winit::window::Window;
use crate::safe_calls::{clear_screen, set_clear_color, set_depth_test};
use crate::shader::{ShaderProgramBuilder, VertexBuffer};
use crate::structures::camera::Camera;
use crate::structures::input_handler::InputHandler;
use crate::structures::matrix::Matrix;
use crate::structures::object::Object;
use crate::structures::quat::Quat;
use crate::structures::texture::Texture;
use crate::structures::vector::Vector;

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

type Ctx = ContextWrapper<PossiblyCurrent, Window>;

fn main() {
    if std::env::args().len() != 2 {
        println!("expected exactly 1 argument");
        return;
    }
    let model = std::env::args().last().unwrap(); //we can unwrap since we know the size of args is 2
    if let Some(model) = Object::load_model(model) {
        if let Some((ctx, event_loop)) = window::spawn_single_window(
            WindowBuilder::new()
                .with_title("Scop")
                .with_visible(true)
        ) {
            let mut timer = std::time::Instant::now();
            let mut acc = std::time::Duration::new(0, 0);

            let mut vao = 0;
            unsafe {
                GenVertexArrays(1, &mut vao);
                BindVertexArray(vao);
            }

            let mut vbo1 = VertexBuffer::<f32, 3>::gen().unwrap();
            let mut vbo2 = VertexBuffer::<f32, 3>::gen().unwrap();
            let mut vbo3 = VertexBuffer::<f32, 3>::gen().unwrap();

            let vertices = model.triangles();

            vbo1.load(vertices, false).enable(0, FLOAT, TRIANGLES);

            let uvs = model.uvs();

            vbo2.load(uvs, false).enable(1, FLOAT, TRIANGLES);

            let normals = model.normals();

            vbo3.load(normals, false).enable(2, FLOAT, TRIANGLES);

            let mut program = ShaderProgramBuilder::default();

            if let Ok(mut file) = File::open("resources/shaders/triangle.vert") {
                let mut t = String::new();
                file.read_to_string(&mut t);
                program.add_shader(VERTEX_SHADER, t.as_str());
            }

            if let Ok(mut file) = File::open("resources/shaders/triangle.frag") {
                let mut t = String::new();
                file.read_to_string(&mut t);
                program.add_shader(FRAGMENT_SHADER, t.as_str());
            }

            let mut program = program.build().unwrap();

            let size = ctx.window().inner_size();
            let proj = Matrix::projection(size.height as f32 / size.width as f32, 90f32.to_radians(), 0.1, 10000.);
            program.set_mat("proj", proj);
            let mut camera = Camera::default();
            program.set_mat("camera", camera.view());

            let mut tex = Texture::parse_file("resources/objs/dragon.bmp".to_string()).unwrap();
            tex.bake(0);
            program.set_tex("tex", tex);

            set_clear_color(0.2, 0.5, 0.2);
            set_depth_test();

            let mut rot: f32 = 90.;
            let mut do_rotate = true;
            
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
                
            event_loop.run(move |event, _target, control_flow| {
                match event {
                    Event::WindowEvent {
                        window_id, event,
                    } if ctx.window().id() == window_id => {
                        if let WindowEvent::KeyboardInput { input, .. } = event {
                            inputs.key_event(input);
                        }
                        if let WindowEvent::Resized(size) = event {
                            unsafe {
                                Viewport(0, 0, size.width as GLsizei, size.height as GLsizei);
                                let proj = Matrix::projection(size.height as f32 / size.width as f32, 90f32.to_radians(), 0.1, 10000.);
                                program.set_mat("proj", proj);
                            }
                        }
                        if event == WindowEvent::CloseRequested {
                            *control_flow = ControlFlow::Exit
                        }
                    }
                    Event::MainEventsCleared => { //aka main loop
                        let elapsed = timer.elapsed();
                        if elapsed.as_secs_f64() >= 1. / 60. {
                            for key in inputs.pressed() {
                                match key {
                                    Inputs::Forward => { camera.pos -= Vector::Z; }
                                    Inputs::Backward => { camera.pos += Vector::Z; }
                                    Inputs::Right => { camera.pos += Vector::X; }
                                    Inputs::Left => { camera.pos -= Vector::X; }
                                    Inputs::Up => { camera.pos += Vector::Y; }
                                    Inputs::Down => { camera.pos -= Vector::Y; }
                                    Inputs::RightRoll => { camera.rot *= Quat::from_axis_angle(Vector::Z, 1f32.to_radians()); }
                                    Inputs::LeftRoll => { camera.rot *= Quat::from_axis_angle(Vector::Z, -1f32.to_radians()); }
                                    Inputs::RightPitch => { camera.rot *= Quat::from_axis_angle(Vector::Y, -1f32.to_radians()); }
                                    Inputs::LeftPitch => { camera.rot *= Quat::from_axis_angle(Vector::Y, 1f32.to_radians()); }
                                    Inputs::UpYaw => { camera.rot *= Quat::from_axis_angle(Vector::X, -1f32.to_radians()); }
                                    Inputs::DownYaw => { camera.rot *= Quat::from_axis_angle(Vector::X, 1f32.to_radians()); }
                                    Inputs::ToggleRotation => { if inputs.status(Inputs::ToggleRotation).just_pressed() { do_rotate = !do_rotate; }}
                                }
                            }
                            program.set_mat("camera", camera.view());
                            acc += elapsed;
                            timer = std::time::Instant::now(); //extremely simplified fps limiter, each frame that takes more than 1/60s will slow the accumulator (currently results in an offset of almost 0.05s per minute, or about a full frame)
                            clear_screen();
                            rot += 1.;
                            if rot >= 360. {
                                rot = 0.;
                            }
                            if do_rotate {
                                program.set_mat("object", Quat::from_axis_angle(Vector::Y, rot.to_radians()).into());
                            }
                            vbo1.draw();
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