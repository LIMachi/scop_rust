use std::env;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::window::WindowBuilder;
use crate::maths::matrix::Matrix;
use crate::opengl::camera::Camera;
use crate::opengl::object::Object;
use crate::opengl::safe_calls;
use crate::opengl::shader::ShaderProgram;
use crate::other::inputs::Inputs;
use crate::other::resource_manager::ResourceManager;
use crate::other::window;

mod parser;
mod opengl;
mod maths;
mod other;

fn main() {
    if env::args().len() != 2 {
        println!("expected exactly 1 argument");
        return;
    }
    let model = env::args().last().unwrap();
    let mut resources = ResourceManager::default();
    resources.register_hints(&["resources", "resources/objs", "resources/materials", "resources/textures", "resources/shaders"]);
    if let Some(object) = resources.load_object(model).cloned() {
        if let Some((ctx, event_loop)) = window::spawn_single_window(WindowBuilder::new()
            .with_title("Scop")
            .with_visible(true)
        ) {
            let mut program = ShaderProgram::from_resources(&mut resources, "triangle").unwrap();
            let mut object = Object::new(&mut resources, &object);

            let mut inputs = Inputs::default_handler();

            safe_calls::set_clear_color(0., 0.5, 0.2);
            safe_calls::set_depth_test(true);

            object.bake();
            // object.render_flags = 1;

            let size = ctx.window().inner_size();
            let proj = Matrix::projection(size.height as f32 / size.width as f32, 90f32.to_radians(), 0.1, 10000.);
            program.set_mat("proj", proj);
            let mut camera = Camera::default();
            program.set_mat("camera", camera.view());

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
                            if Inputs::apply_to_camera(&mut camera, &inputs) {
                                program.set_mat("camera", camera.view());
                            }
                            safe_calls::clear_screen();
                            object.draw(&program);
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