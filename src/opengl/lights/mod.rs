use std::collections::HashMap;
use std::mem::size_of;
use gl::types::{GLsizei, GLuint};
use crate::maths::vector::Vec3;
use crate::opengl::enums::Shaders;
use crate::opengl::part::ObjectPart;
use crate::opengl::shader::{ShaderProgram, ShaderProgramBuilder};

mod directional;

#[derive(Debug, Copy, Clone)]
pub struct Light {
    pub kind: LightKind,
    pub falloff: f32,
    pub color: Vec3
}

#[derive(Debug, Copy, Clone)]
pub enum LightKind {
    Directional {
        direction: Vec3, //defaults to -Z
    },
    Spot {
        position: Vec3, //defaults to 0
        direction: Vec3, //defaults to -Z
        aperture: f32, //defaults to 2 pi
    },
    Point {
        position: Vec3 //defaults to 0
    }
}

//calculate the shadow maps and light intensities of directional lights, spot lights and point lights in a scene
pub struct Lights {
    pub program: ShaderProgram,
    pub lights: HashMap<usize, Light>, //kind, falloff, color
    pub next_id: usize,
    pub kinds: Vec<i32>, //0 directional, 1 spot, 2 point
    pub apertures: Vec<f32>,
    pub falloffs: Vec<f32>,
    pub positions: Vec<Vec3>,
    pub directions: Vec<Vec3>,
    pub colors: Vec<Vec3>,
    pub vao: GLuint,
    pub vbos: [GLuint; 6],
    pub maps: [GLuint; 16],
    pub cube_maps: [GLuint; 8]
}

impl Lights {
    pub fn new() -> Self {
        let program = ShaderProgramBuilder::default()
            .add_shader(Shaders::Vertex, include_str!("lights.vert"))
            // .add_shader(Shaders::Geometry, include_str!("lights.geom"))
            .add_shader(Shaders::Fragment, include_str!("lights.frag"))
            .build().unwrap();
        let mut vao = 0;
        let mut vbos = [0; 6];
        let mut maps = [0; 16];
        let mut cube_maps = [0; 8];
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::GenBuffers(6, &mut vbos[0]);
            for i in 0..6 {
                gl::BindBuffer(gl::ARRAY_BUFFER, vbos[i]);
                if i < 3 {
                    gl::VertexAttribPointer(i as GLuint, 1, gl::FLOAT, gl::FALSE, size_of::<f32>() as GLsizei, 0 as *const _);
                } else {
                    gl::VertexAttribPointer(i as GLuint, 3, gl::FLOAT, gl::FALSE, ObjectPart::VEC3_SIZE as GLsizei, 0 as *const _);
                }
            }
            gl::GenTextures(16, &mut maps[0]);
            gl::GenTextures(8, &mut cube_maps[0]);
            for i in 0..8 {
                
            }
        }
        Self {
            program,
            lights: HashMap::new(),
            next_id: 0,
            kinds: Vec::new(),
            positions: Vec::new(),
            directions: Vec::new(),
            apertures: Vec::new(),
            falloffs: Vec::new(),
            colors: Vec::new(),
            vao,
            vbos,
            maps,
            cube_maps
        }
    }
}