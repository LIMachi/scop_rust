use glutin::{ContextBuilder, ContextWrapper, PossiblyCurrent};
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder, WindowId};

pub type Ctx = ContextWrapper<PossiblyCurrent, Window>;

pub struct GlWindow {
    context: ContextWrapper<PossiblyCurrent, Window>,
    focused: bool,
}

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

impl GlWindow {
    pub fn new(builder: WindowBuilder) -> Option<(Self, EventLoop<()>)> {
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
        context.window().set_cursor_grab(true).unwrap();
        context.window().set_cursor_visible(false);
        Some((Self {
            context,
            focused: true,
        }, event_loop))
    }
    
    pub fn size(&self) -> PhysicalSize<u32> {
        self.context.window().inner_size()
    }
    
    pub fn aspect_ratio(&self) -> f32 {
        let t = self.size();
        t.height as f32 / t.width as f32
    }
    
    pub fn id(&self) -> WindowId {
        self.context.window().id()
    }
    
    pub fn focused(&self) -> bool {
        self.focused
    }
    
    pub fn focus(&mut self, state: bool) {
        if self.focused != state {
            self.context.window().set_cursor_visible(!state);
            self.context.window().set_cursor_grab(state).unwrap();
            self.focused = state;
        }
    }
    
    pub fn refresh(&self) {
        self.context.swap_buffers().unwrap();
    }
}