use std::fs::File;
use std::io::{BufRead, BufReader};
use super::{ParsedObject, Point};
use crate::other::resource_manager::ResourceManager;

impl ParsedObject {
    pub fn parse(resources: &mut ResourceManager, file: File) -> Option<Self> {
        let mut out = Self::default();
        for line in BufReader::new(file).lines().filter_map(|l| l.ok()) {
            let columns = line.split_whitespace().collect::<Vec<&str>>();
            if columns.len() >= 2 {
                match columns[0] {
                    "mtllib" => {
                        for lib in columns[1..].iter().filter_map(|f| resources.load_material_lib(*f).cloned()) {
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
        Some(out)
    }
}