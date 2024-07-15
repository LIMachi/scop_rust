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
    let modes = get_int_array::<2>(gl::POLYGON_MODE);
    (RenderMode::try_from(modes[0] as GLenum).unwrap(), RenderMode::try_from(modes[1] as GLenum).unwrap())
}

pub fn resize(width: u32, height: u32) {
    unsafe {
        gl::Viewport(0, 0, width as GLsizei, height as GLsizei);
    }
}

pub fn get_size() -> (u32, u32) {
    let view = get_int_array::<4>(gl::VIEWPORT);
    (view[2] as u32, view[3] as u32)
}

pub fn set_point_size(size: f32) {
    unsafe {
        gl::PointSize(size);
    }
}

pub fn get_vao() -> GLuint { get_int(gl::ARRAY_BUFFER_BINDING) as GLuint }

pub fn get_int(query: GLenum) -> GLint {
    let mut v = 0;
    unsafe {
        gl::GetIntegerv(query, &mut v);
    }
    v
}

pub fn get_int_array<const S: usize>(query: GLenum) -> [GLint; S] {
    let mut v = [0; S];
    unsafe {
        gl::GetIntegerv(query, &mut v[0]);
    }
    v
}