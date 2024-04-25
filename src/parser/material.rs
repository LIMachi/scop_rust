use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::structures::material::Material;

impl Material {
    pub fn extract_texture_paths(file: File) -> HashSet<String> {
        let mut out = HashSet::new();
        let lines = BufReader::new(file).lines().filter_map(|rs| rs.ok());
        for line in lines {
            let mut it = line.split_whitespace();
            if let Some(fc) = it.next() {
                match fc {
                    "map_Kd" => {
                        if let Some(tex) = it.next() {
                            out.insert(tex.to_string());
                        }
                    },
                    _ => {}
                }
            }
        }
        out
    }
    
    pub fn parse_file(file: String) -> HashMap<String, Material> {
        let lib = File::open(file);
        let mut map = HashMap::new();
        if let Ok(lib) = lib {
            let mut name = "Default".to_string();
            let mut material = Material::default();
            let lines = BufReader::new(lib).lines().filter_map(|rs| rs.ok());
            let mut first_material = true;
            for line in lines {
                let columns = line.split_whitespace().collect::<Vec<&str>>();
                if columns.len() == 0 {
                    continue;
                }
                match columns[0] {
                    "newmtl" if columns.len() == 2 => {
                        if first_material {
                            first_material = false;
                        } else {
                            map.insert(name, material);
                            material = Material::default();
                        }
                        name = columns[1].to_string();
                    },
                    _ => {}
                }
            }
        }
        map
    }
}