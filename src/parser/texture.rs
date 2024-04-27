use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use crate::structures::texture::Texture;

impl Texture {
    pub fn parse_file(file: String) -> Option<Self> {
        let mut file = File::open(file).ok()?;

        let mut head = [0u8; 6];
        file.read_exact(&mut head).ok()?;
        if head[0] != 'B' as u8 || head[1] != 'M' as u8 {
            return None; //invalid header
        }
        let file_size = u32::from_ne_bytes(head[2..].try_into().ok()?) as usize;

        file.seek(SeekFrom::Start(0)).ok()?;
        let mut full = Vec::with_capacity(file_size);
        file.read_to_end(&mut full).ok()?;

        if full.len() != file_size {
            return None; //invalid file size
        }

        let pixel_data = u32::from_ne_bytes(full[10..14].try_into().ok()?) as usize;

        let dib_size = u32::from_ne_bytes(full[14..18].try_into().ok()?);
        let (width, height, bpp, dib_offset, compression) = if dib_size == 12 {
            if u16::from_ne_bytes(full[22..24].try_into().ok()?) != 1 {
                return None; //invalid planes count
            }
            (
                u16::from_ne_bytes(full[18..20].try_into().ok()?) as usize,
                u16::from_ne_bytes(full[20..22].try_into().ok()?) as usize,
                u16::from_ne_bytes(full[24..26].try_into().ok()?) as usize,
                0usize,
                0u32
            )
        } else {
            if u16::from_ne_bytes(full[26..28].try_into().ok()?) != 1 {
                return None; //invalid planes count
            }
            (
                i32::from_ne_bytes(full[18..22].try_into().ok()?) as usize,
                i32::from_ne_bytes(full[22..26].try_into().ok()?) as usize,
                u16::from_ne_bytes(full[28..30].try_into().ok()?) as usize,
                4usize,
                u32::from_ne_bytes(full[30..34].try_into().ok()?),
            )
        };

        if compression != 0 || !(bpp == 24 || bpp == 32) {
            return None; //for now, only support standard RGB format
        }

        let mut out = Self::new(width, height);

        let row_size = ((bpp * width + 31) / 32) * 4;
        let column_size = if bpp == 24 { 3 } else { 4 };

        for y in 0..height {
            for x in 0..width {
                let p = y * row_size + x * column_size + pixel_data;
                out.set(x, height - y - 1, full[p + 2], full[p + 1], full[p]);
            }
        }

        Some(out)
    }
}