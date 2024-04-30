use std::collections::HashMap;
use crate::structures::material::Material;

impl Material {
    /// returns the set of textures to load and the named materials found in the lib
    pub fn parse(lines: &Vec<String>) -> HashMap<String, Material> {
        let mut map = HashMap::new();
        let mut name = "Default".to_string();
        let mut material = Material::default();
        let mut first_material = true;
        for line in lines {
            let columns = line.split_whitespace().collect::<Vec<&str>>();
            if columns.len() >= 2 {
                match columns[0] {
                    "newmtl" => {
                        if first_material {
                            first_material = false;
                        } else {
                            map.insert(name, material);
                            material = Material::default();
                        }
                        name = columns[1].to_string();
                    },
                    "Ns" => {
                        material.specular_exponent = columns[1].parse().unwrap_or(0.);
                    }
                    "Ni" => {
                        material.density = columns[1].parse().unwrap_or(1.);
                    }
                    v @ "d" | v @ "Tr" => {
                        let d: f32 = columns[1].parse().unwrap_or(0.);
                        material.transparency = if v == "d" { d } else { 1. - d }.clamp(0., 1.);
                    }
                    "Tf" if columns.len() >= 4 => {
                        material.filter = [columns[1].parse().unwrap_or(1.), columns[2].parse().unwrap_or(1.), columns[3].parse().unwrap_or(1.)];
                    }
                    "illum" => {
                        material.illum = columns[1].parse().unwrap_or(2).clamp(0, 10);
                    }
                    "Ka" if columns.len() >= 4 => {
                        material.ambient = [columns[1].parse().unwrap_or(0.), columns[2].parse().unwrap_or(0.), columns[3].parse().unwrap_or(0.)];
                    }
                    "Kd" if columns.len() >= 4 => {
                        material.diffuse = [columns[1].parse().unwrap_or(0.), columns[2].parse().unwrap_or(0.), columns[3].parse().unwrap_or(0.)];
                    }
                    "Ks" if columns.len() >= 4 => {
                        material.specular = [columns[1].parse().unwrap_or(0.), columns[2].parse().unwrap_or(0.), columns[3].parse().unwrap_or(0.)];
                    }
                    "Ke" if columns.len() >= 4 => {
                        material.emissive = [columns[1].parse().unwrap_or(0.), columns[2].parse().unwrap_or(0.), columns[3].parse().unwrap_or(0.)];
                    }
                    "map_Ns" => {
                        material.specular_exponent_map = columns[1].to_string();
                    }
                    "map_d" => {
                        material.transparency_map = columns[1].to_string();
                    }
                    "map_Ka" => {
                        material.ambient_map = columns[1].to_string();
                    }
                    "map_Kd" => {
                        material.diffuse_map = columns[1].to_string();
                    }
                    "map_Ks" => {
                        material.specular_map = columns[1].to_string();
                    }
                    "map_Ke" => {
                        material.emissive_map = columns[1].to_string();
                    }
                    "map_bump" | "bump" => {
                        material.bump_map = columns[1].to_string();
                    }
                    "disp" => {
                        material.displacement_map = columns[1].to_string();
                    }
                    "decal" => {
                        material.stencil_map = columns[1].to_string();
                    }
                    _ => {}
                }
            }
        }
        if first_material == false {
            map.insert(name, material);
        }
        map
    }
}