use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;
use crate::structures::material::Material;
use crate::structures::point::Point;

mod structures;
mod parser;
mod window;
mod shader;
mod safe_calls;

//o/g/s ignored
#[derive(Debug, Default)]
pub struct ParsedObject {
    libs: ParsedMaterialLib, //mtllibs
    vertexes: Vec<Point>, //v
    uvs: Vec<[f32; 3]>, //vt
    normals: Vec<[f32; 3]>, //vn
    materials: Vec<String>, //usemtl / mapping from index to name
    material_index: HashMap<String, usize>, //usemtl / mapping from name to index
    groups: Vec<[usize; 3]>, //usemtl / mapping material -> range inclusive of faces
    faces: Vec<Vec<[usize; 3]>> //f
}

#[derive(Debug, Default, Clone)]
pub struct ParsedMaterialLib(pub HashMap<String, Material>);

#[derive(Debug, Default)]
pub struct ParsedTexture {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl ParsedObject {
    fn parse(resources: &mut ResourceManager, file: File) -> Option<Self> {
        let mut out = Self::default();
        for line in BufReader::new(file).lines().filter_map(|l| l.ok()) {
            let columns = line.split_whitespace().collect::<Vec<&str>>();
            if columns.len() >= 2 {
                match columns[0] {
                    "mtllib" => {
                        for lib in columns[1..].iter().filter_map(|f| resources.load_mat(*f).cloned()) {
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

impl ParsedMaterialLib {
    
    fn merge(&mut self, other: &Self) {
        self.0.extend(other.0.iter().map(|(k, v)| (k.clone(), v.clone())));
    }
    
    fn parse(resources: &mut ResourceManager, file: File) -> Option<Self> {
        let mut out = Self::default();
        let mut name = "default".to_string();
        let mut material = Material::default();
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
                            material = Material::default();
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
                        resources.load_tex(&tex)?; //file not found
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

impl ParsedTexture {
    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }
    
    pub fn set(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) -> &mut Self {
        if x < self.width && y < self.height {
            self.data[x * 3 + y * self.width * 3] = r;
            self.data[x * 3 + y * self.width * 3 + 1] = g;
            self.data[x * 3 + y * self.width * 3 + 2] = b;
        }
        self
    }
    
    pub fn parse(mut file: File) -> Option<Self> {
        let mut out = Self::default();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).ok()?;

        if bytes.len() < 6 || bytes[0] != 'B' as u8 || bytes[1] != 'M' as u8 {
            return None; //invalid header
        }

        if bytes.len() != u32::from_ne_bytes(bytes[2..6].try_into().ok()?) as usize {
            return None; //invalid bytes size
        }

        let pixel_data = u32::from_ne_bytes(bytes[10..14].try_into().ok()?) as usize;

        let dib_size = u32::from_ne_bytes(bytes[14..18].try_into().ok()?);
        let (width, height, bpp, dib_offset, compression) = if dib_size == 12 {
            if u16::from_ne_bytes(bytes[22..24].try_into().ok()?) != 1 {
                return None; //invalid planes count
            }
            (
                u16::from_ne_bytes(bytes[18..20].try_into().ok()?) as usize,
                u16::from_ne_bytes(bytes[20..22].try_into().ok()?) as usize,
                u16::from_ne_bytes(bytes[24..26].try_into().ok()?) as usize,
                0usize,
                0u32
            )
        } else {
            if u16::from_ne_bytes(bytes[26..28].try_into().ok()?) != 1 {
                return None; //invalid planes count
            }
            (
                i32::from_ne_bytes(bytes[18..22].try_into().ok()?) as usize,
                i32::from_ne_bytes(bytes[22..26].try_into().ok()?) as usize,
                u16::from_ne_bytes(bytes[28..30].try_into().ok()?) as usize,
                4usize,
                u32::from_ne_bytes(bytes[30..34].try_into().ok()?),
            )
        };

        if compression != 0 || !(bpp == 24 || bpp == 32) {
            return None; //for now, only support standard RGB format
        }

        let row_size = ((bpp * width + 31) / 32) * 4;
        let column_size = if bpp == 24 { 3 } else { 4 };

        for y in 0..height {
            for x in 0..width {
                let p = y * row_size + x * column_size + pixel_data;
                out.set(x, y, bytes[p + 2], bytes[p + 1], bytes[p]);
            }
        }
        
        Some(out)
    }
}

#[derive(Default, Debug)]
pub struct ResourceManager {
    hints: Vec<String>,
    map: HashMap<String, String>,
    objects: HashMap<String, ParsedObject>,
    materials: HashMap<String, ParsedMaterialLib>,
    textures: HashMap<String, ParsedTexture>,
}

impl ResourceManager {
    pub fn register_hints<S: AsRef<OsStr>>(&mut self, hints: &[S]) {
        self.hints.extend(hints.iter().filter_map(|p| {
            let mut t = PathBuf::from(p);
            if t.is_relative() {
                t = env::current_dir().ok()?.join(t);
            }
            t.to_str().map(|s| s.to_string())
        }));
    }

    fn insert_map(&mut self, key: &String, mut path: PathBuf) -> Option<String> {
        if path.is_dir() || !path.exists() {
            return None;
        }
        if path.is_relative() {
            path = env::current_dir().ok()?.join(path);
        }
        let value = path.to_str()?.to_string();
        self.map.insert(key.clone(), value.clone());
        if let Some(hint) = path.parent().and_then(|p| p.to_str()).map(|s| s.to_string()) {
            if hint != "" && !self.hints.contains(&hint) {
                self.hints.insert(0, hint);
            }
        }
        Some(value)
    }

    fn resolve_full_path<S: Into<String>>(&mut self, key: S, extension: &str) -> Option<String> {
        let name = key.into();
        if !self.map.contains_key(&name) {
            if !name.ends_with(extension) {
                if let Some(ext) = self.resolve_full_path(format!("{name}.{extension}"), extension) {
                    return Some(ext);
                }
            }
            let path = PathBuf::from(&name);
            if !path.is_dir() && path.exists() {
                return self.insert_map(&name, path);
            }
            if !path.is_relative() {
                return None;
            }
            for t in self.hints.clone().iter() {
                if let Some(v) = self.insert_map(&name, PathBuf::from(t).join(&path)) {
                    return Some(v);
                }
            }
        }
        self.map.get(&name).cloned()
    }

    pub fn load_obj<S: Into<String>>(&mut self, key: S) -> Option<&ParsedObject> {
        if let Some(p) = self.resolve_full_path(key, "obj") {
            if !self.objects.contains_key(&p) {
                if let Ok(file) = File::open(&p) {
                    if let Some(object) = ParsedObject::parse(self, file) {
                        self.objects.insert(p.clone(), object);
                    }
                }
            }
            self.objects.get(&p)
        } else {
            None
        }
    }

    pub fn load_mat<S: Into<String>>(&mut self, key: S) -> Option<&ParsedMaterialLib> {
        if let Some(p) = self.resolve_full_path(key, "mtl") {
            if !self.materials.contains_key(&p) {
                if let Ok(file) = File::open(&p) {
                    if let Some(material) = ParsedMaterialLib::parse(self, file) {
                        self.materials.insert(p.clone(), material);
                    }
                }
            }
            self.materials.get(&p)
        } else {
            None
        }
    }

    pub fn load_tex<S: Into<String>>(&mut self, key: S) -> Option<&ParsedTexture> {
        if let Some(p) = self.resolve_full_path(key, "bmp") {
            if !self.textures.contains_key(&p) {
                if let Ok(file) = File::open(&p) {
                    if let Some(texture) = ParsedTexture::parse(file) {
                        self.textures.insert(p.clone(), texture);
                    }
                }
            }
            self.textures.get(&p)
        } else {
            None
        }
    }
}

fn main() {
    if env::args().len() != 2 {
        println!("expected exactly 1 argument");
        return;
    }
    let model = env::args().last().unwrap();
    let mut resources = ResourceManager::default();
    resources.register_hints(&["resources", "resources/objs", "resources/textures", "resources/shaders"]);
    resources.load_obj(model);
}