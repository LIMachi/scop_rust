use crate::structures::point::Point;

impl Point {
    pub fn parse(columns: Vec<&str>) -> Option<Self> {
        let mut out = Self::default();
        match columns.len() - 1 {
            3 => {
                out.pos[0] = columns[1].parse().ok()?;
                out.pos[1] = columns[2].parse().ok()?;
                out.pos[2] = columns[3].parse().ok()?;
            }
            4 => {
                out.pos[0] = columns[1].parse().ok()?;
                out.pos[1] = columns[2].parse().ok()?;
                out.pos[2] = columns[3].parse().ok()?;
                out.w = columns[4].parse().ok()?;
            }
            6 => {
                out.pos[0] = columns[1].parse().ok()?;
                out.pos[1] = columns[2].parse().ok()?;
                out.pos[2] = columns[3].parse().ok()?;
                out.color[0] = columns[4].parse().ok()?;
                out.color[1] = columns[5].parse().ok()?;
                out.color[2] = columns[6].parse().ok()?;
            }
            7 => {
                out.pos[0] = columns[1].parse().ok()?;
                out.pos[1] = columns[2].parse().ok()?;
                out.pos[2] = columns[3].parse().ok()?;
                out.color[0] = columns[4].parse().ok()?;
                out.color[1] = columns[5].parse().ok()?;
                out.color[2] = columns[6].parse().ok()?;
                out.w = columns[7].parse().ok()?;
            }
            _ => {}
        }
        Some(out)
    }
}