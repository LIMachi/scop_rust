use crate::parser::Point;

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
    
    pub fn uvs(&self, uvs: &Vec<[f32; 3]>) -> Vec<[f32; 3]> {
        match self.refs.len() {
            3 => {
                vec![uvs[self.refs[0][1] - 1], uvs[self.refs[1][1] - 1], uvs[self.refs[2][1] - 1]]
            }
            4 => {
                vec![uvs[self.refs[0][1] - 1], uvs[self.refs[1][1] - 1], uvs[self.refs[2][1] - 1],
                     uvs[self.refs[0][1] - 1], uvs[self.refs[2][1] - 1], uvs[self.refs[3][1] - 1]]
            }
            _ => {
                vec![]
            }
        }
    }

    pub fn normals(&self, normals: &Vec<[f32; 3]>) -> Vec<[f32; 3]> {
        match self.refs.len() {
            3 => {
                vec![normals[self.refs[0][2] - 1], normals[self.refs[1][2] - 1], normals[self.refs[2][2] - 1]]
            }
            4 => {
                vec![normals[self.refs[0][2] - 1], normals[self.refs[1][2] - 1], normals[self.refs[2][2] - 1],
                     normals[self.refs[0][2] - 1], normals[self.refs[2][2] - 1], normals[self.refs[3][2] - 1]]
            }
            _ => {
                vec![]
            }
        }
    }
}