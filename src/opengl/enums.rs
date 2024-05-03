use gl::types::GLenum;

#[allow(dead_code)]
#[derive(Default, Debug, Copy, Clone)]
pub enum Side {
    #[default]
    FrontAndBack = gl::FRONT_AND_BACK as isize,
    FrontOnly = gl::FRONT as isize,
    BackOnly = gl::BACK as isize,
}

#[derive(Default, Debug, Copy, Clone)]
pub enum RenderMode {
    #[default]
    Full = gl::FILL as isize,
    Lines = gl::LINE as isize,
    Points = gl::POINT as isize
}

#[derive(Debug, Copy, Clone)]
pub enum Shaders {
    Vertex = gl::VERTEX_SHADER as isize,
    Fragment = gl::FRAGMENT_SHADER as isize,
}

impl Into<GLenum> for Side {
    fn into(self) -> GLenum {
        self as GLenum
    }
}

impl Into<GLenum> for RenderMode {
    fn into(self) -> GLenum {
        self as GLenum
    }
}

impl Into<GLenum> for Shaders {
    fn into(self) -> GLenum {
        self as GLenum
    }
}