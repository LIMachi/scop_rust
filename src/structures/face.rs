#[derive(Default, Debug)]
pub struct Face {
    pub refs: Vec<[usize; 3]>,
    pub groups: Vec<String>,
    pub smoothing: usize,
    pub material: usize
}