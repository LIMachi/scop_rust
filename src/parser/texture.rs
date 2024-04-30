use crate::structures::texture::Texture;

impl Texture {
    
    pub fn parse(file: &Vec<u8>) -> Self {
        if file.len() < 6 || file[0] != 'B' as u8 || file[1] != 'M' as u8 {
            return Self::EMPTY; //invalid header
        }

        if file.len() != u32::from_ne_bytes(file[2..6].try_into().unwrap()) as usize {
            return Self::EMPTY; //invalid file size
        }

        let pixel_data = u32::from_ne_bytes(file[10..14].try_into().unwrap()) as usize;

        let dib_size = u32::from_ne_bytes(file[14..18].try_into().unwrap());
        let (width, height, bpp, dib_offset, compression) = if dib_size == 12 {
            if u16::from_ne_bytes(file[22..24].try_into().unwrap()) != 1 {
                return Self::EMPTY; //invalid planes count
            }
            (
                u16::from_ne_bytes(file[18..20].try_into().unwrap()) as usize,
                u16::from_ne_bytes(file[20..22].try_into().unwrap()) as usize,
                u16::from_ne_bytes(file[24..26].try_into().unwrap()) as usize,
                0usize,
                0u32
            )
        } else {
            if u16::from_ne_bytes(file[26..28].try_into().unwrap()) != 1 {
                return Self::EMPTY; //invalid planes count
            }
            (
                i32::from_ne_bytes(file[18..22].try_into().unwrap()) as usize,
                i32::from_ne_bytes(file[22..26].try_into().unwrap()) as usize,
                u16::from_ne_bytes(file[28..30].try_into().unwrap()) as usize,
                4usize,
                u32::from_ne_bytes(file[30..34].try_into().unwrap()),
            )
        };

        if compression != 0 || !(bpp == 24 || bpp == 32) {
            return Self::EMPTY; //for now, only support standard RGB format
        }

        let mut out = Self::new(width, height);

        let row_size = ((bpp * width + 31) / 32) * 4;
        let column_size = if bpp == 24 { 3 } else { 4 };

        for y in 0..height {
            for x in 0..width {
                let p = y * row_size + x * column_size + pixel_data;
                out.set(x, y, file[p + 2], file[p + 1], file[p]);
            }
        }

        out
    }
}