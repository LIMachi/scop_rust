use std::ffi::c_void;
use gl::types::{GLint, GLsizei, GLuint};
use gl::*;

#[derive(Default, Debug)]
pub struct Texture {
    name: GLuint,
    index: u32,
    width: usize,
    height: usize,
    pixel_data: Vec<u8>
}

impl Texture {
    pub const EMPTY: Self = Self {
        name: 0,
        index: 0,
        width: 0,
        height: 0,
        pixel_data: vec![],
    };
    
    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }
    
    pub fn new(width: usize, height: usize) -> Self {
        let mut name = 0;
        unsafe {
            GenTextures(1, &mut name);
        }
        Self {
            name,
            index: 0,
            width,
            height,
            pixel_data: vec![0; width * height * 3]
        }
    }

    pub fn name(&self) -> GLuint {
        self.name
    }
    
    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn set(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) -> &mut Self {
        if x < self.width && y < self.height {
            self.pixel_data[x * 3 + y * self.width * 3] = r;
            self.pixel_data[x * 3 + y * self.width * 3 + 1] = g;
            self.pixel_data[x * 3 + y * self.width * 3 + 2] = b;
        }
        self
    }

    pub fn bake(&mut self, index: u32) -> &mut Self {
        unsafe {
            BindTexture(TEXTURE_2D, self.name);
            TexParameteri(TEXTURE_2D, TEXTURE_WRAP_S, CLAMP_TO_BORDER as i32);
            TexParameteri(TEXTURE_2D, TEXTURE_WRAP_T, CLAMP_TO_BORDER as i32);
            TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
            TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
            TexImage2D(
                TEXTURE_2D,
                0,
                RGB as GLint,
                self.width as GLsizei,
                self.height as GLsizei,
                0,
                RGB,
                UNSIGNED_BYTE,
                self.pixel_data.as_ptr() as *const c_void,
            );
            GenerateMipmap(TEXTURE_2D);
            self.index = index;
            ActiveTexture(TEXTURE0 + index);
        }
        self
    }
}