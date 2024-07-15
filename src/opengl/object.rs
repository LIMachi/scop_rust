use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::maths::transform::Transform;
use crate::maths::vector::Vec3;
use crate::opengl::buffers::{GPUBuffers, VertexType};
use crate::opengl::frustrum::{Frustrum, Volume};
use crate::opengl::main_shader::MainShader;
use crate::opengl::material::Material;
use crate::opengl::texture::Texture;
use crate::other::resource_manager::ResourceManager;
use crate::parser::ParsedObject;

#[derive(Debug)]
struct Part {
    material: usize,
    len: usize,
    buffers: GPUBuffers,
    volume: Volume
}

#[derive(Default, Debug)]
pub struct MultiPartModel {
    textures: Vec<Texture>,
    materials: Vec<Material>,
    parts: Vec<Part>
}

impl Eq for MultiPartModel {}

impl PartialEq for MultiPartModel {
    fn eq(&self, other: &Self) -> bool {
        self.textures.as_ptr() == other.textures.as_ptr()
        && self.materials.as_ptr() == other.materials.as_ptr()
        && self.parts.as_ptr() == other.parts.as_ptr()
    }
}

impl Hash for MultiPartModel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.textures.as_ptr() as usize);
        state.write_usize(self.materials.as_ptr() as usize);
        state.write_usize(self.parts.as_ptr() as usize);
    }
}

impl MultiPartModel {
    pub fn from_raw(vertices: Vec<[f32; 3]>, indices: Vec<u32>) -> Self {
        let mut parts = Vec::new();
        let mut buffers = GPUBuffers::new().unwrap();
        buffers.new_vbo(0, VertexType::Vec3);
        let len = indices.len();
        buffers.set_ebo(indices);
        let mut volume = Volume::default();
        for v in &vertices {
            volume.expand(&Vec3::from(*v));
        }
        buffers.set_vbo(0, vertices);
        parts.push(Part {
            material: 0,
            len,
            buffers,
            volume,
        });
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
            let mut colors: Vec<f32> = Vec::new();
            let mut uvs: Vec<f32> = Vec::new();
            let mut normals: Vec<f32> = Vec::new();
            let mut volume = Volume::default();
            // buffers.new_mingled_vbo(0, 0, VertexType::Vec3, 48, 0);
            // buffers.new_mingled_vbo(0, 1, VertexType::Vec3, 48, 12);
            // buffers.new_mingled_vbo(0, 2, VertexType::Vec3, 48, 24);
            // buffers.new_mingled_vbo(0, 3, VertexType::Vec3, 48, 36);
            buffers.new_vbo(0, VertexType::Vec3);
            buffers.new_vbo(1, VertexType::Vec3);
            buffers.new_vbo(2, VertexType::Vec3);
            buffers.new_vbo(3, VertexType::Vec3);
            for f in start..=end {
                let face = &parsed.faces[f];
                let pf = vertices.len() /* / 12 */ / 3;
                for vf in face {
                    let (v, c) = if vf[0] > 0 && vf[0] <= parsed.vertexes.len() {
                        (parsed.vertexes[vf[0] - 1].pos, parsed.vertexes[vf[0] - 1].color)
                    } else {
                        ([0., 0., 0.], [0., 0., 0.])
                    };
                    for i in 0..3 {
                        vertices.push(v[i]);
                    }
                    volume.expand(&Vec3::from(v));
                    for i in 0..3 {
                        // vertices.push(c[i]);
                        colors.push(c[i]);
                    }
                    let u = if vf[1] > 0 && vf[1] <= parsed.uvs.len() {
                        parsed.uvs[vf[1] - 1]
                    } else {
                        [0., 0., 0.]
                    };
                    for i in 0..3 {
                        // vertices.push(u[i]);
                        uvs.push(u[i]);
                    }
                    let n = if vf[2] > 0 && vf[2] <= parsed.normals.len() {
                        parsed.normals[vf[2] - 1]
                    } else {
                        [0., 0., 0.]
                    };
                    for i in 0..3 {
                        // vertices.push(n[i]);
                        normals.push(n[i]);
                    }
                }
                for step in 0..(face.len() - 2) {
                    for i in 0..3 {
                        indexes.push((pf + if i == 0 { 0 } else { i + step }) as u32);
                    }
                }
            }
            buffers.set_vbo(0, vertices);
            buffers.set_vbo(1, colors);
            buffers.set_vbo(2, uvs);
            buffers.set_vbo(3, normals);
            let len = indexes.len();
            buffers.set_ebo(indexes);
            out.parts.push(Part {
                material,
                len,
                buffers,
                volume,
            });
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
                        let (_, pt) = resource_manager.load_texture(pt).unwrap();
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
    
    pub fn visible(&self, transform: &Transform, frustrum: &Frustrum) -> bool {
        for Part { volume, .. } in &self.parts {
            if frustrum.has_volume(transform, volume) {
                return true;
            }
        }
        return false;
    }

    pub fn draw_instances(&self, count: usize, shader: Option<&MainShader>) {
        for Part { material, len, buffers, .. } in &self.parts {
            if let Some(shader) = shader {
                if *material < self.materials.len() {
                    self.materials[*material].bind(&self.textures, shader);
                }
            }
            buffers.draw_instances(gl::TRIANGLES, 0, *len, count);
        }
    }
}