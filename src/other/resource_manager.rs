use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use crate::other::handles::Handle;
use crate::parser::{ParsedMaterialLib, ParsedObject, ParsedTexture};

#[derive(Default, Debug)]
pub struct ResourceManager {
    hints: Vec<String>,
    map: HashMap<String, String>,
    objects: HashMap<String, Handle<ParsedObject>>,
    materials: HashMap<String, Handle<ParsedMaterialLib>>,
    textures: HashMap<String, Handle<ParsedTexture>>,
    texts: HashMap<String, Handle<String>>
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

    pub fn load_object<S: Into<String>>(&mut self, key: S) -> Handle<ParsedObject> {
        if let Some(p) = self.resolve_full_path(key, &["obj"]) {
            if !self.objects.contains_key(&p) {
                if let Ok(file) = File::open(&p) {
                    if let Some(object) = ParsedObject::parse(self, file) {
                        self.objects.insert(p.clone(), Handle::new_strong(object));
                    }
                }
            }
            self.objects.get(&p).unwrap_or(&Handle::EMPTY).clone_weak()
        } else {
            Handle::EMPTY
        }
    }

    pub fn load_material_lib<S: Into<String>>(&mut self, key: S) -> Handle<ParsedMaterialLib> {
        if let Some(p) = self.resolve_full_path(key, &["mtl"]) {
            if !self.materials.contains_key(&p) {
                if let Ok(file) = File::open(&p) {
                    if let Some(material) = ParsedMaterialLib::parse(self, file) {
                        self.materials.insert(p.clone(), Handle::new_strong(material));
                    }
                }
            }
            self.materials.get(&p).unwrap_or(&Handle::EMPTY).clone_weak()
        } else {
            Handle::EMPTY
        }
    }

    pub fn load_texture<S: Into<String>>(&mut self, key: S) -> Handle<ParsedTexture> {
        if let Some(p) = self.resolve_full_path(key, &["bmp"]) {
            if !self.textures.contains_key(&p) {
                if let Ok(file) = File::open(&p) {
                    if let Some(texture) = ParsedTexture::parse(file) {
                        self.textures.insert(p.clone(), Handle::new_strong(texture));
                    }
                }
            }
            self.textures.get(&p).unwrap_or(&Handle::EMPTY).clone_weak()
        } else {
            Handle::EMPTY
        }
    }

    pub fn load_text<S: Into<String>>(&mut self, key: S) -> Handle<String> {
        if let Some(p) = self.resolve_full_path(key, &["txt", "frag", "vert"]) {
            if !self.texts.contains_key(&p) {
                if let Ok(mut file) = File::open(&p) {
                    let mut text = String::new();
                    if file.read_to_string(&mut text).is_err() {
                        return Handle::EMPTY;
                    }
                    self.texts.insert(p.clone(), Handle::new_strong(text));
                }
            }
            self.texts.get(&p).unwrap_or(&Handle::EMPTY).clone_weak()
        } else {
            Handle::EMPTY
        }
    }
}