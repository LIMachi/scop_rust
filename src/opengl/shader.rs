use gl::{*, types::*};
use crate::opengl::enums::Shaders;
use crate::opengl::uniform::Uniform;
use crate::other::resource_manager::ResourceManager;

pub trait Drawable {
    fn bake(&mut self, program: &ShaderProgram);
    fn bind(&self);
    fn draw(&mut self);
}

#[derive(Default, Debug)]
pub struct ShaderProgram {
    id: GLuint
}

impl ShaderProgram {
    pub fn from_resources(resources: &mut ResourceManager, name: &str) -> Option<Self> {
        let mut builder = ShaderProgramBuilder::default();
        builder.add_shader(Shaders::Vertex, resources.load_text(format!("{name}.vert")).map(|(_, v)| v)?.as_str());
        builder.add_shader(Shaders::Fragment, resources.load_text(format!("{name}.frag")).map(|(_, v)| v)?.as_str());
        if let Some((_, geo)) = resources.load_text(format!("{name}.geom")){
            builder.add_shader(Shaders::Geometry, geo.as_str());
        }
        builder.build()
    }

    pub fn new(id: GLuint) -> Self { Self { id } }
    
    pub fn id(&self) -> GLuint { self.id }

    pub fn uniform(&self, name: &str) -> Uniform { Uniform::new(self, name) }
    
    pub fn set_active(&self) {
        unsafe {
            UseProgram(self.id);
        }
    }
}

#[derive(Default)]
pub struct ShaderProgramBuilder {
    shaders: Vec<GLuint>,
    error: GLint
}

impl ShaderProgramBuilder {

    pub fn add_shader(&mut self, kind: Shaders, source: &str) -> &mut Self {
        unsafe {
            let shader = gl::CreateShader(kind.into());
            if shader == 0 {
                return self;
            }
            gl::ShaderSource(shader, 1, &(source.as_bytes().as_ptr() as *const GLchar), &(source.len() as GLint));
            gl::CompileShader(shader);
            gl::GetShaderiv(shader, COMPILE_STATUS, &mut self.error);
            if self.error != 0 {
                self.shaders.push(shader);
            } else {
                let mut info: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(
                    shader,
                    1024,
                    &mut log_len,
                    info.as_mut_ptr().cast(),
                );
                info.set_len(log_len.try_into().unwrap());
                println!("{:?} Compile Error: {}", kind, String::from_utf8_lossy(&info));
                DeleteShader(shader);
            }
            self
        }
    }

    pub fn build(&mut self) -> Option<ShaderProgram> {
        unsafe {
            let shader_program = CreateProgram();
            if shader_program == 0 {
                return None;
            }
            for shader in self.shaders.iter() {
                AttachShader(shader_program, *shader);
            }
            LinkProgram(shader_program);
            let mut success = 0;
            GetProgramiv(shader_program, LINK_STATUS, &mut success);
            if success != 0 {
                for shader in self.shaders.iter() {
                    DeleteShader(*shader);
                }
                UseProgram(shader_program);
                gl::Enable(PROGRAM_POINT_SIZE);
                Some(ShaderProgram::new(shader_program))
            } else {
                let mut info: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                GetProgramInfoLog(
                    shader_program,
                    1024,
                    &mut log_len,
                    info.as_mut_ptr().cast(),
                );
                info.set_len(log_len.try_into().unwrap());
                println!("Program Link Error: {}", String::from_utf8_lossy(&info));
                None
            }
        }
    }
}