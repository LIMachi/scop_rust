use std::ffi::c_void;
use gl::{ActiveTexture, BindTexture, CLAMP_TO_BORDER, GenerateMipmap, GenTextures, LINEAR, RGB, TexImage2D, TexParameteri, TEXTURE0, TEXTURE_2D, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, TEXTURE_WRAP_S, TEXTURE_WRAP_T, UNSIGNED_BYTE};
use gl::types::{GLenum, GLint, GLsizei, GLuint};
use crate::opengl::shader::ShaderProgram;

#[derive(Default, Debug)]
pub struct Texture {
    pub name: GLuint,
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>
}

impl Texture {
    pub fn bake(&mut self) {
        unsafe {
            if self.name == 0 {
                GenTextures(1, &mut self.name);
                if self.name == 0 {
                    return;
                }
                //set this texture active for all subsequent functions
                BindTexture(TEXTURE_2D, self.name);
                //set repeating texture mode to none (will be stretched to fit)
                TexParameteri(TEXTURE_2D, TEXTURE_WRAP_S, CLAMP_TO_BORDER as i32);
                TexParameteri(TEXTURE_2D, TEXTURE_WRAP_T, CLAMP_TO_BORDER as i32);
                //set filtering for lower lods to linear (aka weighted sum of nearby pixels)
                TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
                TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
                //load the data in this texture
                TexImage2D(
                    TEXTURE_2D,
                    0,
                    RGB as GLint,
                    self.width as GLsizei,
                    self.height as GLsizei,
                    0,
                    RGB,
                    UNSIGNED_BYTE,
                    self.data.as_ptr() as *const c_void,
                );
                //prepare the mip map (will generate different scales of this texture for lods)
                GenerateMipmap(TEXTURE_2D);
            }
        }
    }

    pub fn bind(&self, tex_offset: usize, shader: &ShaderProgram) {
        unsafe {
            if self.name != 0 {
                //set this texture active for all subsequent functions
                BindTexture(TEXTURE_2D, self.name);
                //take the texture and bind it to the texture indexed by the given offset
                ActiveTexture(TEXTURE0 + tex_offset as GLenum);
                //bind the texture sampler to the uniform location 'tex<offset>'
                shader.set_u32(format!("tex{tex_offset}").as_str(), tex_offset as u32);
                // Uniform1i(tex_offset as GLint, tex_offset as GLint);
            }
        }
    }
}