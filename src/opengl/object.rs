use std::cmp::Ordering;
use std::collections::HashMap;
use crate::opengl::material::Material;
use crate::opengl::part::ObjectPart;
use crate::opengl::shader::ShaderProgram;
use crate::opengl::texture::Texture;
use crate::other::resource_manager::ResourceManager;
use crate::parser::ParsedObject;

#[derive(Default, Debug)]
pub struct Object {
    pub textures: Vec<Texture>,
    pub materials: Vec<Material>,
    pub parts: Vec<ObjectPart>,
    pub current_part: usize,
    pub current_material: usize,
}

impl Object {
    pub fn bake(&mut self) {
        for m in &self.materials {
            m.bake(&mut self.textures);
        }
        for p in &mut self.parts {
            p.bake();
        }
        self.current_material = self.materials.len(); //force the binding of the material on first draw
        self.current_part = self.parts.len(); //force the binding of the vbos on first draw
    }
    
    pub fn draw(&mut self, program: &ShaderProgram) {
        for (i, p) in self.parts.iter().enumerate() {
            if self.current_material != p.material {
                self.materials[p.material].bind(&self.textures, program);
                self.current_material = p.material;
            }
            if self.current_part != i {
                self.current_part = i;
                p.bind();
            }
            p.draw();
        }
    }
    
    pub fn new(resources: &mut ResourceManager, parsed: &ParsedObject) -> Self {
        let mut out = Self::default();
        let mut texture_map: HashMap<String, usize> = HashMap::new();
        texture_map.insert("".to_string(), 0);
        out.textures.push(Texture::default());
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
                        let pt = resources.load_texture(pt).unwrap();
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
        //sort parts so we reduce the number of material swaps
        out.parts.sort_by(|f1, f2| {
            if f1.material == f2.material {
                Ordering::Equal
            } else if f1.material < f2.material {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        out
    }
}