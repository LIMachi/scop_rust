use crate::structures::object::Object;
use crate::structures::resource_manager::ResourceManager;
use crate::structures::texture::Texture;

impl Object {
    // pub fn load_model(resources: &mut ResourceManager, path: &str) -> Option<Object> {
    //     let model = resources.lines(path);
    //     ObjectParser::load_model(resources, &model).map(|p| p.building)
    // }

    pub fn load_textures(&mut self, resources: &mut ResourceManager, paths: impl IntoIterator<Item = impl Into<String>>) {
        for path in paths.into_iter().map(|s| s.into()) {
            if let Some(name) = resources.find_true_path(path.as_str()) {
                if !self.textures.contains_key(&name) {
                    let mut t = Texture::parse(&resources.raw_file(name.as_str()));
                    if !t.is_empty() {
                        t.bake(self.textures.len() as u32);
                    }
                    self.textures.insert(name, t);
                }
            }
        }
    }
    
    pub fn load_materials(&mut self, resources: &mut ResourceManager, path: String) {
        
    }
    
    
}