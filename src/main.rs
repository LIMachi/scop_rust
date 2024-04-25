mod parser;
mod structures;
mod window;

extern crate gl;

use glutin::{ContextWrapper, PossiblyCurrent};
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    window::WindowBuilder,
};
use winit::window::Window;
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
                            // println!("tick: {acc:?}");
                            timer = std::time::Instant::now(); //extremely simplified fps limiter, each frame that takes more than 1/60s will slow the accumulator (currently results in an offset of almost 0.05s per minute, or about a full frame)
                            unsafe {
                                gl::ClearColor(0.2, 0.5, 0.2, 1.0);
                                gl::Clear(gl::COLOR_BUFFER_BIT);
                            }
                            ctx.swap_buffers().unwrap();
                        }
                    }
                    _ => {}
                }
            });
        }
    }
}