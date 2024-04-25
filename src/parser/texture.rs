use std::fs::File;
use crate::structures::texture::RawTexture;

impl RawTexture {
    pub fn parse_file(file: String) -> Option<Self> {
        let file = File::open(file).ok()?;
        Some(Self::default()) //FIXME!
    }
}