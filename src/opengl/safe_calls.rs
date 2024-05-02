use gl::types::GLsizei;
use crate::opengl::enums::{RenderMode, Side};

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

pub fn set_depth_test(state: bool) {
    unsafe {
        if state {
            gl::Enable(gl::DEPTH_TEST);
            // gl::DepthFunc(gl::GREATER);
            gl::DepthFunc(gl::LESS);
        } else {
            gl::Disable(gl::DEPTH_TEST);
        }
    }
}

pub fn set_cull_face(state: bool) {
    unsafe {
        if state {
            gl::Enable(gl::CULL_FACE);
        } else {
            gl::Disable(gl::CULL_FACE);
        }
    }
}

pub fn set_draw_mode(side: Side, mode: RenderMode) {
    unsafe {
        gl::PolygonMode(side.into(), mode.into());
    }
}

pub fn resize(width: u32, height: u32) {
    unsafe {
        gl::Viewport(0, 0, width as GLsizei, height as GLsizei);
    }
}