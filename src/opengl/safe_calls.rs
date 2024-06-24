use gl::types::{GLenum, GLint, GLsizei, GLuint};
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

pub fn get_draw_mode() -> (RenderMode, RenderMode) {
    unsafe {
        let mut modes = [0 as GLint; 2];
        gl::GetIntegerv(gl::POLYGON_MODE, &mut modes as *mut GLint);
        (RenderMode::try_from(modes[0] as GLenum).unwrap(), RenderMode::try_from(modes[1] as GLenum).unwrap())
    }
}

pub fn resize(width: u32, height: u32) {
    unsafe {
        gl::Viewport(0, 0, width as GLsizei, height as GLsizei);
    }
}

pub fn get_size() -> (u32, u32) {
    let mut v = [0; 4];
    unsafe {
        gl::GetIntegerv(gl::VIEWPORT, &mut v[0]);
    }
    (v[2] as u32, v[3] as u32)
}

pub fn set_point_size(size: f32) {
    unsafe {
        gl::PointSize(size);
    }
}

pub fn get_vao() -> GLuint {
    let mut v = 0;
    unsafe {
        gl::GetIntegerv(gl::ARRAY_BUFFER_BINDING, &mut v);
    }
    v as GLuint
}