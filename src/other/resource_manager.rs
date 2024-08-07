use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use crate::opengl::material::Material;
use crate::opengl::object::MultiPartModel;
use crate::opengl::texture::Texture;
use crate::parser::{ParsedMaterialLib, ParsedObject, ParsedTexture};

#[derive(Default, Debug)]
pub struct ResourceManager {
    hints: HashSet<String>,
    next_id: usize,
    map: HashMap<String, String>,
    ids: HashMap<String, usize>,
    objects: HashMap<usize, ParsedObject>,
    mat_libs: HashMap<usize, ParsedMaterialLib>,
    material: HashMap<usize, Material>,
    textures: HashMap<usize, ParsedTexture>,
    maps: HashMap<usize, Texture>,
    texts: HashMap<usize, String>,
    models: HashMap<usize, MultiPartModel>,
}

impl ResourceManager {
    pub fn register_hints<S: AsRef<OsStr>>(&mut self, hints: &[S]) {
        self.hints.extend(hints.iter().filter_map(|p| {
            let mut t = PathBuf::from(p);
            if t.is_relative() {
                t = env::current_dir().ok()?.join(t);
            }
            t.to_str().map(|s| s.to_string().replace('\\', "/"))
        }));
    }

    fn insert_map(&mut self, key: &String, mut path: PathBuf) -> Option<String> {
        if path.is_dir() || !path.exists() {
            return None;
        }
        if path.is_relative() {
            path = env::current_dir().ok()?.join(path);
        }
        let value = path.to_str()?.to_string().replace('\\', "/");
        self.map.insert(key.clone(), value.clone());
        if let Some(hint) = path.parent().and_then(|p| p.to_str()).map(|s| s.to_string()) {
            if hint != "" && !self.hints.contains(&hint) {
                self.hints.insert(hint.replace('\\', "/"));
            }
        }
        Some(value)
    }

    pub fn resolve_full_path<S: Into<String>>(&mut self, key: S, extensions: &[&str]) -> Option<String> {
        let name = key.into();
        if !self.map.contains_key(&name) {
            for ext in extensions {
                if !name.ends_with(ext) {
                    if let Some(f) = self.resolve_full_path(format!("{name}.{ext}"), &[]) {
                        return Some(f);
                    }
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
    
    fn resolve_id(&mut self, path: &String) -> usize {
        if !self.ids.contains_key(path) {
            self.ids.insert(path.clone(), self.next_id);
            self.next_id += 1;
        }
        *self.ids.get(path).unwrap()
    }
    
    pub fn load_object<S: Into<String>>(&mut self, key: S) -> Option<(usize, &ParsedObject)> {
        if let Some(p) = self.resolve_full_path(key, &["obj"]) {
            let id = self.resolve_id(&p);
            if !self.objects.contains_key(&id) {
                if let Ok(file) = File::open(&p) {
                    if let Some(object) = ParsedObject::parse(self, file) {
                        self.objects.insert(id, object);
                    }
                }
            }
            self.objects.get(&id).map(|v| (id, v))
        } else {
            None
        }
    }
    
    pub fn get_object(&self, id: usize) -> Option<&ParsedObject> {
        self.objects.get(&id)
    }
    
    pub fn get_object_mut(&mut self, id: usize) -> Option<&mut ParsedObject> {
        self.objects.get_mut(&id)
    }

    pub fn load_material_lib<S: Into<String>>(&mut self, key: S) -> Option<(usize, &ParsedMaterialLib)> {
        if let Some(p) = self.resolve_full_path(key, &["mtl"]) {
            let id = self.resolve_id(&p);
            if !self.mat_libs.contains_key(&id) {
                if let Ok(file) = File::open(&p) {
                    if let Some(material) = ParsedMaterialLib::parse(self, file) {
                        self.mat_libs.insert(id, material);
                    }
                }
            }
            self.mat_libs.get(&id).map(|v| (id, v))
        } else {
            None
        }
    }

    pub fn get_material_lib(&self, id: usize) -> Option<&ParsedMaterialLib> {
        self.mat_libs.get(&id)
    }

    pub fn get_material_lib_mut(&mut self, id: usize) -> Option<&mut ParsedMaterialLib> {
        self.mat_libs.get_mut(&id)
    }

    pub fn load_material<S: Into<String>, M: Into<String>>(&mut self, key: S, mat: M) -> Option<(usize, &Material)> {
        let mat = mat.into();
        let key = key.into();
        let mat_path = format!("{key}:{mat}");
        if let Some(id) = self.ids.get(&mat_path) {
            return self.material.get(id).map(|m| (*id, m));
        }
        let id = self.next_id;
        if let Some(m) = self.load_material_lib(key).iter().find_map(|(_, l)| l.0.get(&mat)).cloned() {
            let ambient_map = self.load_map(m.ambient_map).map(|(id, _)| id).unwrap_or(0);
            let diffuse_map = self.load_map(m.diffuse_map).map(|(id, _)| id).unwrap_or(0);
            let transparency_map = self.load_map(m.transparency_map).map(|(id, _)| id).unwrap_or(0);
            let specular_exponent_map = self.load_map(m.specular_exponent_map).map(|(id, _)| id).unwrap_or(0);
            let specular_map = self.load_map(m.specular_map).map(|(id, _)| id).unwrap_or(0);
            let emissive_map = self.load_map(m.emissive_map).map(|(id, _)| id).unwrap_or(0);
            let bump_map = self.load_map(m.bump_map).map(|(id, _)| id).unwrap_or(0);
            let displacement_map = self.load_map(m.displacement_map).map(|(id, _)| id).unwrap_or(0);
            let stencil_map = self.load_map(m.stencil_map).map(|(id, _)| id).unwrap_or(0);
            self.material.insert(id, Material {
                specular_exponent: m.specular_exponent,
                density: m.density,
                transparency: m.transparency,
                filter: m.filter,
                ambient: m.ambient,
                diffuse: m.diffuse,
                specular: m.specular,
                emissive: m.emissive,
                illum: m.illum,
                ambient_map,
                diffuse_map,
                transparency_map,
                specular_exponent_map,
                specular_map,
                emissive_map,
                bump_map,
                displacement_map,
                stencil_map,
            });
            self.next_id += 1;
        }
        self.material.get(&id).map(|m| (id, m))
    }

    pub fn get_material(&self, id: usize) -> Option<&Material> {
        self.material.get(&id)
    }

    pub fn get_material_mut(&mut self, id: usize) -> Option<&mut Material> {
        self.material.get_mut(&id)
    }

    pub fn load_texture<S: Into<String>>(&mut self, key: S) -> Option<(usize, &ParsedTexture)> {
        if let Some(p) = self.resolve_full_path(key, &["bmp"]) {
            let id = self.resolve_id(&p);
            if !self.textures.contains_key(&id) {
                if let Ok(file) = File::open(&p) {
                    if let Some(texture) = ParsedTexture::parse(file) {
                        self.textures.insert(id, texture);
                    }
                }
            }
            self.textures.get(&id).map(|v| (id, v))
        } else {
            None
        }
    }

    pub fn get_texture(&self, id: usize) -> Option<&ParsedTexture> {
        self.textures.get(&id)
    }

    pub fn get_texture_mut(&mut self, id: usize) -> Option<&mut ParsedTexture> {
        self.textures.get_mut(&id)
    }
    
    pub fn load_map<S: Into<String>>(&mut self, key: S) -> Option<(usize, &Texture)> {
        let key = key.into();
        if let Some(p) = self.resolve_full_path(&key, &["bmp"]) {
            let id = self.resolve_id(&p);
            if !self.maps.contains_key(&id) {
                if let Some(map) = self.load_texture(key).map(|(id, v)| Texture {
                    name: 0,
                    width: v.width,
                    height: v.height,
                    data: v.data.clone()
                }) {
                    self.maps.insert(id, map);
                }
            }
            self.maps.get(&id).map(|v| (id, v))
        } else {
            None
        }
    }

    pub fn get_map(&self, id: usize) -> Option<&Texture> {
        self.maps.get(&id)
    }

    pub fn get_map_mut(&mut self, id: usize) -> Option<&mut Texture> {
        self.maps.get_mut(&id)
    }

    pub fn load_text<S: Into<String>>(&mut self, key: S) -> Option<(usize, &String)> {
        if let Some(p) = self.resolve_full_path(key, &["txt", "frag", "vert", "geom"]) {
            let id = self.resolve_id(&p);
            if !self.texts.contains_key(&id) {
                if let Ok(mut file) = File::open(&p) {
                    let mut text = String::new();
                    file.read_to_string(&mut text).ok()?;
                    self.texts.insert(id, text);
                }
            }
            self.texts.get(&id).map(|v| (id, v))
        } else {
            None
        }
    }

    pub fn get_text(&self, id: usize) -> Option<&String> {
        self.texts.get(&id)
    }

    pub fn get_text_mut(&mut self, id: usize) -> Option<&mut String> {
        self.texts.get_mut(&id)
    }
    
    //instead of returning a parsed object file, it will actively try to create an instance of model (loading materials and textures and creating gpu buffers if needed)
    pub fn load_multipart_model<S: Into<String>>(&mut self, key: S) -> Option<(usize, &MultiPartModel)> {
        let key = key.into();
        if let Some(p) = self.resolve_full_path(&key, &["obj"]) {
            let id = self.resolve_id(&p);
            if !self.models.contains_key(&id) {
                if let Some(obj) = self.load_object(key).map(|(id, v)| v.clone()) {
                    let model = MultiPartModel::new(self, &obj);
                    self.models.insert(id, model);
                }
            }
            self.models.get(&id).map(|v| (id, v))
        } else {
            None
        }
    }

    pub fn get_multipart_model(&self, id: usize) -> Option<&MultiPartModel> {
        self.models.get(&id)
    }

    pub fn get_multipart_model_mut(&mut self, id: usize) -> Option<&mut MultiPartModel> {
        self.models.get_mut(&id)
    }
    
    pub fn unload_key<S: Into<String>>(&mut self, key: S) {
        if let Some(p) = self.resolve_full_path(key, &["obj", "mtl", "bmp", "txt", "frag", "vert", "geom"]) {
            let id = self.resolve_id(&p);
            self.objects.remove(&id);
            self.mat_libs.remove(&id);
            self.material.remove(&id);
            self.textures.remove(&id);
            self.maps.remove(&id);
            self.texts.remove(&id);
            self.models.remove(&id);
            self.ids.remove(&p);
            self.map.retain(|_, v| *v != p);
        }
    }
    
    pub fn unload_id(&mut self, id: usize) {
        self.objects.remove(&id);
        self.mat_libs.remove(&id);
        self.material.remove(&id);
        self.textures.remove(&id);
        self.maps.remove(&id);
        self.texts.remove(&id);
        self.models.remove(&id);
        let mut p = "".to_string();
        self.ids.retain(|k, v| if *v == id {
            p = k.clone();
            false
        } else {
            true
        });
        self.map.retain(|_, v| *v != p);
    }
    
    pub fn debug(&self) {
        dbg!(&self.hints);
        dbg!(&self.map);
        dbg!(&self.ids);
        dbg!(&self.objects.keys());
        dbg!(&self.models.keys());
    }
}