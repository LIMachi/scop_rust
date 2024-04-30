use std::collections::HashMap;
use std::ffi::c_void;
use std::mem::size_of;
use gl::types::{GLsizei, GLsizeiptr, GLuint};
use crate::other::resource_manager::ResourceManager;
use crate::parser::{ParsedObject, ParsedTexture};

#[derive(Default, Debug)]
pub struct ObjectPart {
    pub material: usize,
    pub vertices: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub vbos: [GLuint; 3]
}

impl ObjectPart {
    pub fn bind(&mut self) {
        unsafe {
            if self.vbos[0] == 0 {
                gl::GenBuffers(3, &mut self.vbos[0]); //allocate the buffers
                if self.vbos[0] == 0 {
                    return;
                }
            }
            const SIZE: usize = size_of::<[f32; 3]>();
            
            //set this buffer active for all subsequent functions
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbos[0]);
            //allocation of the buffer (total size, data, access kind) access kind STATIC_DRAW = one or few changes, frequent access
            gl::BufferData(gl::ARRAY_BUFFER, (SIZE * self.vertices.len()) as GLsizeiptr, self.vertices.as_ptr() as *const c_void, gl::STATIC_DRAW);
            //description of the array in the shader (location = 0, each element is a group of 3 floats, unormalized, with a pointer offset of 0 because we already have an array defined above)
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, SIZE as GLsizei, 0 as *const _);
            //enable shader access to location 0
            gl::EnableVertexAttribArray(0);
            
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbos[1]);
            gl::BufferData(gl::ARRAY_BUFFER, (SIZE * self.uvs.len()) as GLsizeiptr, self.uvs.as_ptr() as *const c_void, gl::STATIC_DRAW);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, SIZE as GLsizei, 0 as *const _);
            gl::EnableVertexAttribArray(1);
            
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbos[2]);
            gl::BufferData(gl::ARRAY_BUFFER, (SIZE * self.normals.len()) as GLsizeiptr, self.normals.as_ptr() as *const c_void, gl::STATIC_DRAW);
            gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, SIZE as GLsizei, 0 as *const _);
            gl::EnableVertexAttribArray(2);
        }
    }
    
    pub fn draw(&self) {
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as GLsizei);
        }
    }
}

#[derive(Default, Debug)]
pub struct Material {
    pub specular_exponent: f32,
    pub density: f32,
    pub transparency: f32,
    pub filter: [f32; 3],
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    pub emissive: [f32; 3],
    pub illum: i32,
    pub specular_exponent_map: usize,
    pub transparency_map: usize,
    pub ambient_map: usize,
    pub diffuse_map: usize,
    pub specular_map: usize,
    pub emissive_map: usize,
    pub bump_map: usize,
    pub displacement_map: usize,
    pub stencil_map: usize,
}

#[derive(Default, Debug)]
pub struct Object {
    pub textures: Vec<ParsedTexture>,
    pub materials: Vec<Material>,
    pub parts: Vec<ObjectPart>,
}

impl Object {
    pub fn bind(&mut self) {
        for p in &mut self.parts {
            p.bind();
        }
    }
    
    pub fn draw(&self) {
        for p in &self.parts {
            //use correct textures/materials there
            p.draw();
        }
    }
    
    pub fn new(resources: &mut ResourceManager, parsed: &ParsedObject) -> Self {
        let mut out = Self::default();
        let mut texture_map: HashMap<String, usize> = HashMap::new();
        texture_map.insert("".to_string(), 0);
        out.textures.push(ParsedTexture::default());
        for group in parsed.groups.iter() {
            let mut part = ObjectPart::default();
            part.material = group[0];
            for f in group[1]..=group[2] {
                let face = &parsed.faces[f];
                for step in 0..(face.len() - 2) {
                    for i in 0..3 {
                        let r = if i == 0 { 0 } else { i + step };
                        let ti = face[r][0];
                        if ti > 0 && ti - 1 < parsed.vertexes.len() {
                            part.vertices.push(parsed.vertexes[face[r][0] - 1].pos);
                        }
                        let ti = face[r][1];
                        if ti > 0 && ti - 1 < parsed.uvs.len() {
                            part.uvs.push(parsed.uvs[face[r][1] - 1]);
                        }
                        let ti = face[r][2];
                        if ti > 0 && ti - 1 < parsed.normals.len() {
                            part.normals.push(parsed.normals[face[r][2] - 1]);
                        }
                    }
                }
            }
            out.parts.push(part);
        }
        for material in parsed.materials.iter() {
            let p = &parsed.libs.0[material];
            let mut mat = Material {
                specular_exponent: p.specular_exponent,
                density: p.density,
                transparency: p.transparency,
                filter: p.filter,
                ambient: p.ambient,
                diffuse: p.diffuse,
                specular: p.specular,
                emissive: p.emissive,
                illum: p.illum,
                ..Default::default()
            };
            for (pt, mt) in [
                    (&p.specular_exponent_map, &mut mat.specular_exponent_map),
                    (&p.ambient_map, &mut mat.ambient_map),
                    (&p.diffuse_map, &mut mat.diffuse_map),
                    (&p.specular_map, &mut mat.specular_map),
                    (&p.transparency_map, &mut mat.transparency_map), 
                    (&p.bump_map, &mut mat.bump_map),
                    (&p.displacement_map, &mut mat.displacement_map),
                    (&p.stencil_map, &mut mat.stencil_map),
                    (&p.emissive_map, &mut mat.emissive_map)
            ] {
                if pt != "" {
                    *mt = if let Some(texture) = texture_map.get(pt) {
                        *texture
                    } else {
                        let t = out.textures.len();
                        texture_map.insert(pt.clone(), t);
                        out.textures.push(resources.load_texture(pt).unwrap().clone());
                        t
                    };
                }
            }
            out.materials.push(mat);
        }
        out
    }
}