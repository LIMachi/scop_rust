use std::fs::File;
use crate::parser::ObjectParser;
use crate::structures::object::Object;

impl Object {
    pub fn load_model(path: String) -> Option<Object> {
        let model = File::open(path).ok()?;
        ObjectParser::load_model(model).map(|p| p.building)
    }

    pub fn load_textures(&mut self, paths: impl IntoIterator<Item = impl Into<String>>) {
        for path in paths.into_iter().map(|s| s.into()) {
            if !self.textures.contains_key(&path) {

            }
        }
    }
}