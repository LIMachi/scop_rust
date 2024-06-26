use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::maths::matrix::Mat3;
use crate::maths::quaternion::Quaternion;
use crate::maths::vector::{Vec3, Vector};
use super::{ParsedMaterialLib, ParsedObject, Point};
use crate::other::resource_manager::ResourceManager;

impl ParsedObject {
    //center the object to 0, 0, 0, scale the object so it's edges are [-50, -50, -50] [50, 50, 50] (without deformation)
    //TODO add automatic uvs and normals if missing
    pub fn normalize(&mut self) {
        if self.normalized {
            return;
        }
        let (min, max) = self.vertexes.iter().fold((Vector::splat(f32::MAX), Vector::splat(f32::MIN)), |acc, v| {
            let v = Vector::from(v.pos);
            (acc.0.min(v), acc.1.max(v))
        });
        let size = max - min;
        let center = size * 0.5 + min;
        let max_size = size[0].max(size[1]).max(size[2]);
        for p in &mut self.vertexes {
            p.pos = ((Vector::from(p.pos) - center) * (100. / max_size)).into();
        }
        
        self.normals.clear(); //FIXME, trick to test if dragon works with automatic normals
        
        // dbg!(self.normals.len(), self.triangles.len());
        
        if self.normals.len() == 0 && self.faces.len() > 0 {
            //create default normals
            //default algorithm: for each face, calculate normal (using normalized cross product of f0f1 X f0f2, then copy it for each vertex of the face)
            for f in &mut self.faces {
                let f0 = Vector::from(self.vertexes[f[0][0] - 1].pos);
                let f1 = Vector::from(self.vertexes[f[1][0] - 1].pos);
                let f2 = Vector::from(self.vertexes[f[2][0] - 1].pos);
                let n  = (f1 - f0).cross_product(&(f2 - f0)).normalize();
                let i = self.normals.len();
                self.normals.push(n.into());
                f.iter_mut().for_each(|fv| fv[2] = i + 1);
            }
        } else {
            // let rot = Mat3::from(Quaternion::from((Vec3::X, 90f32.to_radians()))/* * Quaternion::from((Vec3::Z, -90f32.to_radians()))*/);
            // //FIXME: test with rotated normals for dragon
            // for normal in &mut self.normals {
            //     *normal = (rot * Vec3::from(*normal)).into();
            // }
            // dbg!(self.normals[self.triangles[0][0][2]]); //0.5373, -0.20229992, -0.8188001
        }
        
        // dbg!(self.normals.len());
        
        if self.uvs.len() == 0 && self.faces.len() > 0 {
            //create default uvs
            //default algorithm: take the full size of model, take the position of vertices inside and scale them to be in the range 0-1, this will give the relative size of the triangle, now to orientate and scale it inside the texture, we use the direction of the face (normal)
        }
        
        self.normalized = true;
    }
    
    pub fn parse(resources: &mut ResourceManager, file: File) -> Option<Self> {
        let mut out = Self::default();
        out.libs = ParsedMaterialLib::with_default_material();
        for line in BufReader::new(file).lines().filter_map(|l| l.ok()) {
            let columns = line.split_whitespace().collect::<Vec<&str>>();
            if columns.len() >= 2 {
                match columns[0] {
                    "mtllib" => {
                        for lib in columns[1..].iter().filter_map(|f| resources.load_material_lib(*f).map(|(_, v)| v.clone())) {
                            out.libs.merge(&lib);
                            for name in lib.0.keys() {
                                if !out.materials.contains(name) {
                                    out.material_index.insert(name.clone(), out.materials.len());
                                    out.materials.push(name.clone());
                                }
                            }
                        }
                    }
                    "usemtl" if columns.len() == 2 => {
                        if let Some(id) = out.material_index.get(columns[1]) {
                            let l = out.groups.len();
                            if l > 0 && out.faces.len() > 0 {
                                out.groups[l - 1][2] = out.faces.len() - 1;
                            }
                            out.groups.push([*id, out.faces.len(), out.faces.len()]);
                        } else {
                            //invalid material reference error
                        }
                    }
                    "v" if columns.len() >= 4 && columns.len() <= 8 => {
                        if let Some(point) = Point::parse(columns) {
                            out.vertexes.push(point);
                        } else {
                            //invalid vertex definition error
                        }
                    }
                    v @ "vt" | v @ "vn" if columns.len() <= 4 => {
                        let mut t = [0.; 3];
                        let mut valid = false;
                        for i in 0..3 {
                            if i + 1 < columns.len() {
                                if let Ok(v) = columns[i + 1].parse() {
                                    valid = true;
                                    t[i] = v;
                                } else {
                                    break;
                                }
                            }
                        }
                        if valid {
                            if v == "vt" {
                                out.uvs.push(t);
                            } else {
                                out.normals.push(t);
                            }
                        } else {
                            return None; //invalid uv or normal definition error
                        }
                    }
                    "f" => {
                        let mut f = Vec::new();
                        let mut sm = 0;
                        for tc in columns[1..].iter().map(|c| c.split('/')) {
                            let mut r = [0usize; 3];
                            let mut i = 0;
                            let mut mask = 0;
                            for s in tc {
                                if i < 3 && s.len() > 0 {
                                    let mut t: isize = s.parse().ok()?;
                                    if t < 0 {
                                        t = match i {
                                            0 => out.vertexes.len(),
                                            1 => out.uvs.len(),
                                            2 => out.normals.len(),
                                            _ => 0,
                                        } as isize + t;
                                    }
                                    if t < 0 {
                                        return None; //error: looping back reference
                                    }
                                    r[i] = t as usize;
                                    mask |= 1 << i;
                                }
                                i += 1;
                            }
                            if sm == 0 {
                                sm = mask;
                            } else if sm != mask {
                                return None; //mismatched face format
                            }
                            f.push(r);
                        }
                        if f.len() >= 3 {
                            let l = out.groups.len();
                            if l > 0 {
                                out.groups[l - 1][2] = out.faces.len();
                            }
                            out.faces.push(f);
                        } else {
                            return None; //face too short (require at least 3 references)
                        }
                    }
                    _ => {}
                }
            }
        }
        if out.faces.len() > 0 && out.vertexes.len() > 0 {
            if out.materials.len() == 0 {
                out.materials.push("default".to_string());
            }
            if out.groups.len() == 0 { //fix missing / undeclared groups
                out.groups.push([0, 0, out.faces.len() - 1]);
            }
            Some(out)
        } else {
            None
        }
    }
}