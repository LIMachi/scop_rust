use std::fs::File;
use std::io::{BufRead, BufReader};
use super::{ParsedMaterial, ParsedMaterialLib};
use crate::other::resource_manager::ResourceManager;

impl ParsedMaterialLib {

    pub fn merge(&mut self, other: &Self) {
        self.0.extend(other.0.iter().map(|(k, v)| (k.clone(), v.clone())));
    }

    pub fn parse(resources: &mut ResourceManager, file: File) -> Option<Self> {
        let mut out = Self::default();
        let mut name = "default".to_string();
        let mut material = ParsedMaterial::default();
        let mut first_material = true;
        for line in BufReader::new(file).lines().filter_map(|l| l.ok()) {
            let columns = line.split_whitespace().collect::<Vec<&str>>();
            if columns.len() >= 2 {
                match columns[0] {
                    "newmtl" if columns.len() == 2 => {
                        if first_material {
                            first_material = false;
                        } else {
                            out.0.insert(name, material);
                            material = ParsedMaterial::default();
                        }
                        name = columns[1].to_string();
                    },
                    "Ns" if columns.len() == 2 => material.specular_exponent = columns[1].parse().unwrap_or(0.),
                    "Ni" if columns.len() == 2 => material.density = columns[1].parse().unwrap_or(1.),
                    v @ "d" | v @ "Tr" if columns.len() == 2 => {
                        let d: f32 = columns[1].parse().unwrap_or(0.);
                        material.transparency = if v == "d" { d } else { 1. - d }.clamp(0., 1.);
                    }
                    "Tf" if columns.len() >= 4 => {
                        material.filter = [columns[1].parse().unwrap_or(1.), columns[2].parse().unwrap_or(1.), columns[3].parse().unwrap_or(1.)];
                    }
                    "illum" if columns.len() == 2 => material.illum = columns[1].parse().unwrap_or(2).clamp(0, 10),
                    "Ka" | "Kd" | "Ks" | "Ke" if columns.len() >= 4 => {
                        let v = [columns[1].parse().unwrap_or(0.), columns[2].parse().unwrap_or(0.), columns[3].parse().unwrap_or(0.)];
                        match columns[0] {
                            "Ka" if columns.len() >= 4 => material.ambient = v,
                            "Kd" if columns.len() >= 4 => material.diffuse = v,
                            "Ks" if columns.len() >= 4 => material.specular = v,
                            "Ke" if columns.len() >= 4 => material.emissive = v,
                            _ => {}
                        }
                    }
                    "map_Ns" | "map_d" | "map_Ka" | "map_Kd" | "map_Ks" | "map_Ke" | "map_bump" | "bump" | "disp" | "decal" if columns.len() == 2 => {
                        let tex = columns[1].to_string();
                        resources.load_texture(&tex)?; //file not found
                        match columns[0] {
                            "map_Ns" => material.specular_exponent_map = tex,
                            "map_d" => material.transparency_map = tex,
                            "map_Ka" => material.ambient_map = tex,
                            "map_Kd" => material.diffuse_map = tex,
                            "map_Ks" => material.specular_map = tex,
                            "map_Ke" => material.emissive_map = tex,
                            "map_bump" | "bump" => material.bump_map = tex,
                            "disp" => material.displacement_map = tex,
                            "decal" => material.stencil_map = tex,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
        if first_material == false {
            out.0.insert(name, material);
        }
        if out.0.is_empty() {
            None
        } else {
            Some(out)
        }
    }
}