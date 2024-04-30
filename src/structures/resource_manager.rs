use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};

pub struct ResourceManager {
    pub paths: Vec<String>
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            paths: vec![]
        }
    }
    
    pub fn register_path(&mut self, path: &str) -> &mut Self {
        let mut path = PathBuf::from(Path::new(&OsString::from(path)));
        if path.is_relative() {
            if let Ok(dir) = env::current_dir() {
                path = dir.join(path);
            }
        }
        if let Some(str) = if path.is_dir() {
            path.to_str()
        } else {
            path.parent().and_then(|p| p.to_str())
        } {
            let str = str.to_string();
            if !self.paths.contains(&str) {
                self.paths.insert(0, str);
            }
        }
        self
    }
    
    pub fn find_true_path(&self, path: &str) -> Option<String> {
        let p = Path::new(path);
        if p.is_file() {
            Some(p.canonicalize().ok()?.to_str()?.to_string())
        } else if p.is_absolute() {
            None
        } else {
            for tp in self.paths.iter() {
                let rp = Path::new(tp).join(p);
                if rp.is_file() {
                    return Some(p.to_str()?.to_string());
                }
            }
            None
        }
    }
    
    pub fn open_file(&mut self, path: &str) -> Option<File> {
        let p = Path::new(path);
        let t = File::open(p).ok();
        if t.is_some() {
            self.register_path(path);
            t
        } else if p.is_absolute() {
            None
        } else {
            for tp in self.paths.iter() {
                let rp = Path::new(tp).join(p);
                let t = File::open(&rp).ok();
                if t.is_some() {
                    self.register_path(rp.to_str().unwrap());
                    return t;
                }
            }
            None
        }
    }
    
    pub fn raw_file(&mut self, path: &str) -> Vec<u8> {
        if let Some(mut file) = self.open_file(path) {
            let mut v = Vec::new();
            file.read_to_end(&mut v).map_or_else(|_| Vec::new(), |_| v)
        } else {
            Vec::new()
        }
    }
    
    pub fn lines(&mut self, path: &str) -> Vec<String> {
        if let Some(file) = self.open_file(path) {
            BufReader::new(file).lines().filter_map(|r| r.ok()).collect()
        } else {
            Vec::new()
        }
    }
}