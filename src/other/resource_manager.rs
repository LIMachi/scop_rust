use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use crate::opengl::objectv2::MultiPartModel;
use crate::parser::{ParsedMaterialLib, ParsedObject, ParsedTexture};

#[derive(Default, Debug)]
pub struct ResourceManager {
    hints: Vec<String>,
    next_id: usize,
    map: HashMap<String, (String, usize)>,
    objects: HashMap<usize, ParsedObject>,
    materials: HashMap<usize, ParsedMaterialLib>,
    textures: HashMap<usize, ParsedTexture>,
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
            t.to_str().map(|s| s.to_string())
        }));
    }

    fn insert_map(&mut self, key: &String, mut path: PathBuf) -> Option<(String, usize)> {
        if path.is_dir() || !path.exists() {
            return None;
        }
        if path.is_relative() {
            path = env::current_dir().ok()?.join(path);
        }
        let value = path.to_str()?.to_string();
        self.map.insert(key.clone(), (value.clone(), self.next_id));
        self.next_id += 1;
        if let Some(hint) = path.parent().and_then(|p| p.to_str()).map(|s| s.to_string()) {
            if hint != "" && !self.hints.contains(&hint) {
                self.hints.insert(0, hint);
            }
        }
        Some((value, self.next_id - 1))
    }

    pub fn resolve_full_path<S: Into<String>>(&mut self, key: S, extensions: &[&str]) -> Option<(String, usize)> {
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
    
    pub fn load_object<S: Into<String>>(&mut self, key: S) -> Option<(usize, &ParsedObject)> {
        if let Some((p, id)) = self.resolve_full_path(key, &["obj"]) {
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
        if let Some((p, id)) = self.resolve_full_path(key, &["mtl"]) {
            if !self.materials.contains_key(&id) {
                if let Ok(file) = File::open(&p) {
                    if let Some(material) = ParsedMaterialLib::parse(self, file) {
                        self.materials.insert(id, material);
                    }
                }
            }
            self.materials.get(&id).map(|v| (id, v))
        } else {
            None
        }
    }

    pub fn get_material_lib(&self, id: usize) -> Option<&ParsedMaterialLib> {
        self.materials.get(&id)
    }

    pub fn get_material_lib_mut(&mut self, id: usize) -> Option<&mut ParsedMaterialLib> {
        self.materials.get_mut(&id)
    }

    pub fn load_texture<S: Into<String>>(&mut self, key: S) -> Option<(usize, &ParsedTexture)> {
        if let Some((p, id)) = self.resolve_full_path(key, &["bmp"]) {
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

    pub fn load_text<S: Into<String>>(&mut self, key: S) -> Option<(usize, &String)> {
        if let Some((p, id)) = self.resolve_full_path(key, &["txt", "frag", "vert", "geom"]) {
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
        if let Some((p, id)) = self.resolve_full_path(&key, &["obj"]) {
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
        if let Some((_, id)) = self.resolve_full_path(key, &["obj", "mtl", "bmp", "txt", "frag", "vert", "geom"]) {
            self.objects.remove(&id);
            self.materials.remove(&id);
            self.textures.remove(&id);
            self.texts.remove(&id);
            self.models.remove(&id);
        }
    }
    
    pub fn unload_id(&mut self, id: usize) {
        self.objects.remove(&id);
        self.materials.remove(&id);
        self.textures.remove(&id);
        self.texts.remove(&id);
        self.models.remove(&id);
    }
}