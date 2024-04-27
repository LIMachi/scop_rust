pub fn clear_screen() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}

pub fn set_clear_color(red: f32, green: f32, blue: f32) {
    unsafe {
        gl::ClearColor(red, green, blue, 1.0);
    }
}

pub fn set_depth_test() {
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        // gl::DepthFunc(gl::GREATER);
        gl::DepthFunc(gl::LESS);
    }
}