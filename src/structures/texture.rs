#[derive(Default, Debug)]
pub struct RawTexture {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<[f32; 3]>
}