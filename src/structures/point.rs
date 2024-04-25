#[derive(Debug)]
pub struct Point {
    pub pos: [f32; 3],
    pub color: [f32; 3],
    pub w: f32
}

impl Default for Point {
    fn default() -> Self {
        Self {
            pos: [0.; 3],
            color: [1.; 3],
            w: 1.
        }
    }
}