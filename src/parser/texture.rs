use std::fs::File;
use std::io::Read;
use super::ParsedTexture;

impl ParsedTexture {
    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    pub fn set(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) -> &mut Self {
        if x < self.width && y < self.height {
            self.data[x * 3 + y * self.width * 3] = r;
            self.data[x * 3 + y * self.width * 3 + 1] = g;
            self.data[x * 3 + y * self.width * 3 + 2] = b;
        }
        self
    }

    pub fn parse(mut file: File) -> Option<Self> {
        let mut out = Self::default();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).ok()?;

        if bytes.len() < 6 || bytes[0] != 'B' as u8 || bytes[1] != 'M' as u8 {
            return None; //invalid header
        }

        if bytes.len() != u32::from_ne_bytes(bytes[2..6].try_into().ok()?) as usize {
            return None; //invalid bytes size
        }

        let pixel_data = u32::from_ne_bytes(bytes[10..14].try_into().ok()?) as usize;

        let dib_size = u32::from_ne_bytes(bytes[14..18].try_into().ok()?);
        let (width, height, bpp, dib_offset, compression) = if dib_size == 12 {
            if u16::from_ne_bytes(bytes[22..24].try_into().ok()?) != 1 {
                return None; //invalid planes count
            }
            (
                u16::from_ne_bytes(bytes[18..20].try_into().ok()?) as usize,
                u16::from_ne_bytes(bytes[20..22].try_into().ok()?) as usize,
                u16::from_ne_bytes(bytes[24..26].try_into().ok()?) as usize,
                0usize,
                0u32
            )
        } else {
            if u16::from_ne_bytes(bytes[26..28].try_into().ok()?) != 1 {
                return None; //invalid planes count
            }
            (
                i32::from_ne_bytes(bytes[18..22].try_into().ok()?) as usize,
                i32::from_ne_bytes(bytes[22..26].try_into().ok()?) as usize,
                u16::from_ne_bytes(bytes[28..30].try_into().ok()?) as usize,
                4usize,
                u32::from_ne_bytes(bytes[30..34].try_into().ok()?),
            )
        };

        if compression != 0 || !(bpp == 24 || bpp == 32) {
            return None; //for now, only support standard RGB format
        }

        let row_size = ((bpp * width + 31) / 32) * 4;
        let column_size = if bpp == 24 { 3 } else { 4 };

        for y in 0..height {
            for x in 0..width {
                let p = y * row_size + x * column_size + pixel_data;
                out.set(x, y, bytes[p + 2], bytes[p + 1], bytes[p]);
            }
        }

        Some(out)
    }
}