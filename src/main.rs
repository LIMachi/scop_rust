use std::env;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::window::WindowBuilder;
use crate::maths::matrix::Matrix;
use crate::maths::transform::Transform;
use crate::maths::vector::Vector;
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
    if env::args().len() < 2 {
        println!("expected at least 1 argument");
        return;
    }
    let mut resources = ResourceManager::default();
    resources.register_hints(&["resources", "resources/objs", "resources/materials", "resources/textures", "resources/shaders"]);
    
    let parsed: Vec<String> = env::args().enumerate().filter_map(|(i, a)| {
        if i == 0 {
            return None;
        }
        resources.load_object(&a)?.normalize();
        Some(a)
    }).collect();
    
    if parsed.len() > 0 {
        if let Some((ctx, event_loop)) = window::spawn_single_window(WindowBuilder::new()
            .with_title("Scop")
            .with_visible(true)
        ) {
            let mut program = ShaderProgram::from_resources(&mut resources, "triangle").unwrap();
            let mut objects: Vec<Object> = parsed.iter().enumerate().map(|(i, s)| {
                let t = resources.load_object(s).unwrap().clone();
                let mut out = Object::new(&mut resources, &t);
                out.transform = Transform::from_pos(Vector::Z * -100. * i as f32);
                out
            } ).collect();

            let mut inputs = Inputs::default_handler();

            safe_calls::set_clear_color(0., 0.5, 0.2);
            safe_calls::set_depth_test(true);

            for object in objects.iter_mut() {
                object.bake(&program);
            }
            // object.render_flags = 1;

            let size = ctx.window().inner_size();
            let uniform_proj = program.uniform("proj");
            uniform_proj.mat(Matrix::projection(size.height as f32 / size.width as f32, 90f32.to_radians(), 0.1, 10000.));
            
            let mut camera = Transform::from_look_at(Vector::X * 50., Vector::default());
            let uniform_camera = program.uniform("camera");
            uniform_camera.mat(camera.as_view_matrix());

            let mut timer = std::time::Instant::now();
            
            let mut rotate = false;
            let mut speed_up = false;
            let mut fade_in = false;
            
            let mut fade = 0f32;
            let uniform_fade = program.uniform("fade");
            
            event_loop.run(move |event, _target, control_flow| {
                match event {
                    Event::WindowEvent {
                        window_id, event,
                    } if ctx.window().id() == window_id => {
                        if let WindowEvent::KeyboardInput { input, .. } = event {
                            inputs.key_event(input);
                        }
                        if let WindowEvent::CursorMoved { position, .. } = event {
                        }
                        if let WindowEvent::Resized(size) = event {
                            safe_calls::resize(size.width, size.height);
                            uniform_proj.mat(Matrix::projection(size.height as f32 / size.width as f32, 90f32.to_radians(), 0.1, 10000.));
                        }
                        if event == WindowEvent::CloseRequested {
                            *control_flow = ControlFlow::Exit
                        }
                        if let WindowEvent::Focused(focused) = event {
                            ctx.window().set_cursor_visible(!focused);
                        }
                    }
                    Event::MainEventsCleared => {
                        let elapsed = timer.elapsed();
                        if elapsed.as_secs_f64() >= 1. / 60. {
                            timer = std::time::Instant::now();
                            if Inputs::apply_to_camera(&mut camera, &inputs, speed_up) {
                                uniform_camera.mat(camera.as_view_matrix());
                            }
                            if inputs.status(Inputs::ToggleRotation).just_pressed() {
                                rotate = !rotate;
                            }
                            if inputs.status(Inputs::ToggleSpeedUp).just_pressed() {
                                speed_up = !speed_up;
                            }
                            if inputs.status(Inputs::ToggleFade).just_pressed() {
                                fade_in = !fade_in;
                            }
                            safe_calls::clear_screen();
                            fade = (fade + if fade_in { 0.02 } else { -0.02 }).clamp(0., 1.);
                            uniform_fade.float(fade);
                            for object in objects.iter_mut() {
                                if rotate {
                                    object.transform.rotate_local(Vector::Y, 1f32.to_radians());
                                }
                                object.bind();
                                object.draw(&program);
                            }
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