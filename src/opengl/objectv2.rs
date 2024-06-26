use std::collections::HashMap;
use crate::opengl::buffers::{GPUBuffers, VertexType};
use crate::opengl::material::Material;
use crate::opengl::texture::Texture;
use crate::other::resource_manager::ResourceManager;
use crate::parser::ParsedObject;

#[derive(Default)]
pub struct Object {
    pub textures: Vec<Texture>,
    pub materials: Vec<Material>,
    pub parts: Vec<(usize, usize, GPUBuffers)>
}

impl Object {
    pub fn from_raw(vertices: Vec<[f32; 3]>, indices: Vec<u32>) -> Self {
        let mut parts = Vec::new();
        let mut t = GPUBuffers::new().unwrap();
        t.new_vbo(0, VertexType::Vec3);
        t.set_vbo(0, vertices);
        let len = indices.len();
        t.set_ebo(indices);
        parts.push((0, len, t));
        Self {
            textures: Vec::new(),
            materials: Vec::new(),
            parts
        }
    }

    pub fn new(resource_manager: &mut ResourceManager, parsed: &ParsedObject) -> Self {
        let mut out  = Self::default();
        let mut texture_map: HashMap<String, usize> = HashMap::new();
        texture_map.insert("".to_string(), 0);
        out.textures.push(Texture::palette());
        for &[material, start, end] in &parsed.groups {
            let mut buffers = GPUBuffers::new().unwrap();
            let mut indexes = Vec::new();
            let mut vertices: Vec<f32> = Vec::new();
            buffers.new_mingled_vbo(0, 0, VertexType::Vec3, 48, 0);
            buffers.new_mingled_vbo(0, 1, VertexType::Vec3, 48, 12);
            buffers.new_mingled_vbo(0, 2, VertexType::Vec3, 48, 24);
            buffers.new_mingled_vbo(0, 3, VertexType::Vec3, 48, 36);
            for f in start..=end {
                let face = &parsed.faces[f];
                let pf = vertices.len() / 12;
                for vf in face {
                    let (v, c) = if vf[0] > 0 && vf[0] <= parsed.vertexes.len() {
                        (parsed.vertexes[vf[0] - 1].pos, parsed.vertexes[vf[0] - 1].color)
                    } else {
                        ([0., 0., 0.], [0., 0., 0.])
                    };
                    for i in 0..3 {
                        vertices.push(v[i]);
                    }
                    for i in 0..3 {
                        vertices.push(c[i]);
                    }
                    let u = if vf[1] > 0 && vf[1] <= parsed.uvs.len() {
                        parsed.uvs[vf[1] - 1]
                    } else {
                        [0., 0., 0.]
                    };
                    for i in 0..3 {
                        vertices.push(u[i]);
                    }
                    let n = if vf[2] > 0 && vf[2] <= parsed.normals.len() {
                        parsed.normals[vf[1] - 1]
                    } else {
                        [0., 0., 0.]
                    };
                    for i in 0..3 {
                        vertices.push(n[i]);
                    }
                }
                for step in 0..(face.len() - 2) {
                    for i in 0..3 {
                        indexes.push((pf + if i == 0 { 0 } else { i + step }) as u32);
                    }
                }
            }
            buffers.set_vbo(0, vertices);
            let len = indexes.len();
            buffers.set_ebo(indexes);
            out.parts.push((material, len, buffers));
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
                        let pt = resource_manager.load_texture(pt).unwrap();
                        out.textures.push(Texture {
                            name: 0,
                            width: pt.width,
                            height: pt.height,
                            data: pt.data.clone(),
                        });
                        t
                    };
                }
            }
            out.materials.push(mat);
        }
        out
    }

    pub fn draw(&self) {
        for (mat, len, part) in &self.parts {
            if *mat < self.materials.len() {
                self.materials[*mat].bind(&self.textures);
            }
            part.draw(gl::TRIANGLES, 0, *len);
        }
    }
    
    pub fn draw_instances(&self, count: usize) {
        for (mat, len, part) in &self.parts {
            if *mat < self.materials.len() {
                self.materials[*mat].bind(&self.textures);
            }
            part.draw_instances(gl::TRIANGLES, 0, *len, count);
        }
    }
}