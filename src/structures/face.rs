use crate::structures::point::Point;

#[derive(Default, Debug)]
pub struct Face {
    pub refs: Vec<[usize; 3]>,
    pub groups: Vec<String>,
    pub smoothing: usize,
    pub material: usize
}

impl Face {
    pub fn vertices(&self, points: &Vec<Point>) -> Vec<[f32; 3]> {
        match self.refs.len() {
            3 => {
                vec![points[self.refs[0][0] - 1].pos, points[self.refs[1][0] - 1].pos, points[self.refs[2][0] - 1].pos]
            }
            4 => {
                vec![points[self.refs[0][0] - 1].pos, points[self.refs[1][0] - 1].pos, points[self.refs[2][0] - 1].pos,
                     points[self.refs[0][0] - 1].pos, points[self.refs[2][0] - 1].pos, points[self.refs[3][0] - 1].pos]
            }
            _ => {
                vec![]
            }
        } 
    }
}