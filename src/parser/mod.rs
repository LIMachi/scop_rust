use crate::structures::object::Object;

pub mod texture;
pub mod material;
pub mod object;
pub mod point;
mod parser;

#[derive(Default, Debug)]
pub struct ObjectParser {
    pub current_groups: Vec<String>,
    pub current_smoothing: usize,
    pub current_material: usize,
    pub building: Object,
}