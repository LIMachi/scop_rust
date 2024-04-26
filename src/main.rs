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
use crate::safe_calls::{clear_screen, set_clear_color};
use crate::shader::{ShaderProgramBuilder, VertexBuffer};
use crate::structures::camera::Camera;
use crate::structures::matrix::Matrix;
use crate::structures::object::Object;
use crate::structures::quat::Quat;
use crate::structures::vector::Vector;

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
            
            let mut vbo = VertexBuffer::<f32, 3>::gen().unwrap();

            let vertices = model.triangles();

            vbo.load(vertices, false).enable(0, FLOAT, TRIANGLES);

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
            let proj = Matrix::projection(size.height as f32 / size.width as f32, 90f32.to_radians(), 0.1, 100.);
            program.set_mat("proj", proj);
            let mut camera = Camera::default();
            program.set_mat("camera", camera.view());

            set_clear_color(0.2, 0.5, 0.2);
            
            let mut rot: f32 = 90.;

            event_loop.run(move |event, _target, control_flow| {
                match event {
                    Event::WindowEvent {
                        window_id, event,
                    } if ctx.window().id() == window_id => {
                        if let WindowEvent::KeyboardInput { device_id, input, is_synthetic } = event {
                            if let Some(key) = input.virtual_keycode {
                                match key {
                                    VirtualKeyCode::S => {
                                        camera.pos -= Vector::Z * 0.1;
                                    }
                                    VirtualKeyCode::W => {
                                        camera.pos += Vector::Z * 0.1;
                                    }
                                    VirtualKeyCode::A => {
                                        camera.pos -= Vector::X * 0.1;
                                    }
                                    VirtualKeyCode::D => {
                                        camera.pos += Vector::X * 0.1;
                                    }
                                    VirtualKeyCode::LShift => {
                                        camera.pos -= Vector::Y * 0.1;
                                    }
                                    VirtualKeyCode::Space => {
                                        camera.pos += Vector::Y * 0.1;
                                    }
                                    VirtualKeyCode::Q => {
                                        camera.rot *= Quat::from_axis_angle(Vector::Z, -10f32.to_radians());
                                    }
                                    VirtualKeyCode::E => {
                                        camera.rot *= Quat::from_axis_angle(Vector::Z, 10f32.to_radians());
                                    }
                                    _ => {}
                                }
                                program.set_mat("camera", camera.view());
                                println!("camera: {camera:?}");
                            }
                        }
                        if let WindowEvent::Resized(size) = event {
                            unsafe {
                                Viewport(0, 0, size.width as GLsizei, size.height as GLsizei);
                                let proj = Matrix::projection(size.height as f32 / size.width as f32, 90f32.to_radians(), 0.1, 100.);
                                program.set_mat("proj", proj);
                            }
                        }
                        if event == WindowEvent::CloseRequested {
                            *control_flow = ControlFlow::Exit
                        }
                    }
                    Event::MainEventsCleared => {
                        let elapsed = timer.elapsed();
                        if elapsed.as_secs_f64() >= 1. / 60. {
                            acc += elapsed;
                            timer = std::time::Instant::now(); //extremely simplified fps limiter, each frame that takes more than 1/60s will slow the accumulator (currently results in an offset of almost 0.05s per minute, or about a full frame)
                            clear_screen();
                            rot += 1.;
                            if rot >= 360. {
                                rot = 0.;
                            }
                            program.set_mat("object", Quat::from_axis_angle(Vector::Y, rot.to_radians()).into());
                            vbo.draw();
                            ctx.swap_buffers().unwrap();
                        }
                    }
                    _ => {}
                }
            });
        }
    }
}