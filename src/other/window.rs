use glutin::{ContextBuilder, ContextWrapper, PossiblyCurrent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub type Ctx = ContextWrapper<PossiblyCurrent, Window>;

pub fn spawn_single_window(builder: WindowBuilder) -> Option<(Ctx, EventLoop<()>)> {
    let event_loop = EventLoop::new();
    let window_context = ContextBuilder::new()
        .build_windowed(builder, &event_loop)
        .ok()?;
    let context = unsafe {
        window_context
            .make_current()
            .ok()?
    };
    gl::load_with(|s| context.get_proc_address(s) as *const _);
    Some((context, event_loop))
}