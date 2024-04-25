mod parser;
mod structures;
mod window;
mod shader;
mod safe_calls;

extern crate gl;

use std::fs::File;
use std::io::Read;
use gl::*;

use glutin::{ContextWrapper, PossiblyCurrent};
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    window::WindowBuilder,
};
use winit::window::Window;
use crate::safe_calls::{clear_screen, set_clear_color};
use crate::shader::{ShaderProgramBuilder, VertexBuffer};
use crate::structures::matrix::Mat;
use crate::structures::object::Object;

type Ctx = ContextWrapper<PossiblyCurrent, Window>;

fn handle_event(_ctx: &Ctx, event: WindowEvent, control_flow: &mut ControlFlow) {
    println!("Got window event {event:?}");
    if event == WindowEvent::CloseRequested {
        *control_flow = ControlFlow::Exit
    }
}

fn main() {
    if std::env::args().len() != 2 {
        println!("expected exactly 1 argument");
        return;
    }
    let model = std::env::args().last().unwrap(); //we can unwrap since we know the size of args is 2
    println!("loading model: '{model}'");
    if let Some(model) = Object::load_model(model) {
        println!("debug model: {model:?}");
        if let Some((ctx, event_loop)) = window::spawn_single_window(
            WindowBuilder::new()
                .with_title("Scop")
                .with_always_on_top(true)
                .with_visible(true)
        ) {
            let mut timer = std::time::Instant::now();
            let mut acc = std::time::Duration::new(0, 0);

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

            set_clear_color(0.2, 0.5, 0.2);
            
            let mut rot: f32 = 0.;

            event_loop.run(move |event, _target, control_flow| {
                match event {
                    Event::WindowEvent {
                        window_id, event,
                    } if ctx.window().id() == window_id => {
                        handle_event(&ctx, event, control_flow);
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
                            program.set_mat("test", Mat::<4, 4, f32>::rot_y(rot.to_radians()));
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