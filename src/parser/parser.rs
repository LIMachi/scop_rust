use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::parser::ObjectParser;
use crate::structures::face::Face;
use crate::structures::material::Material;
use crate::structures::point::Point;

impl ObjectParser {
    pub fn parse_face(&self, columns: Vec<&str>) -> Option<Face> {
        let mut out = Face::default();
        if self.current_groups.len() > 0 {
            out.groups = self.current_groups.clone();
        }
        out.smoothing = self.current_smoothing;
        out.material = self.current_material;
        let mut split_mask = 0;
        for column in columns[1..].iter().map(|c| c.split('/')) {
            let mut r = [0usize; 3];
            let mut i = 0;
            let mut mask = 0;
            for s in column {
                if i < 3 && s.len() > 0 {
                    let mut t: isize = s.parse().ok()?;
                    if t < 0 {
                        t = match i {
                            0 => self.building.points.len(),
                            1 => self.building.uvs.len(),
                            2 => self.building.normals.len(),
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
            if split_mask == 0 {
                split_mask = mask;
            } else if split_mask != mask {
                return None; //mismatched face format
            }
            out.refs.push(r);
        }
        Some(out)
    }
    
    pub fn load_model(file: File) -> Option<Self> {
        let lines = BufReader::new(file).lines().filter_map(|rs| rs.ok());
        let mut out = Self::default();
        for line in lines {
            let columns = line.split_whitespace().collect::<Vec<&str>>();
            if columns.len() == 0 {
                continue;
            }
            match columns[0] {
                "mtllib" => {
                    for file in columns[1..].iter().filter_map(|f| File::open(*f).ok()) {
                        out.building.load_textures(Material::extract_texture_paths(file));
                    }
                }
                "usemtl" => {
                    //check if materiel exists, set it so it affect the faces
                }
                "v" => {
                    if let Some(point) = Point::parse(columns) {
                        out.building.points.push(point);
                    }
                }
                "vt" | "vn" => {
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
                        if columns[0] == "vt" {
                            out.building.uvs.push(t);
                        } else {
                            out.building.normals.push(t);
                        }
                    } else {
                        return None; //should print an error
                    }
                }
                "f" => {
                    if let Some(face) = out.parse_face(columns) {
                        out.building.faces.push(face);
                    } else {
                        return None; //should print an error
                    }
                }
                _ => {}
            }
        }
        Some(out)
    }
}