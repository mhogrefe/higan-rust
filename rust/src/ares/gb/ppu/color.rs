use ares::gb::ppu::PPU;
use malachite_base::num::logic::traits::BitBlockAccess;
use nall::image::image_static::normalize;
use std::cmp::min;

impl PPU {
    pub fn color_game_boy(&self, color: u32) -> u64 {
        match self.color_emulation_dmg {
            "Game Boy" => {
                const MONOCHROME: [[u8; 3]; 4] = [
                    [0xae, 0xd9, 0x27],
                    [0x58, 0xa0, 0x28],
                    [0x20, 0x62, 0x29],
                    [0x1a, 0x45, 0x2a],
                ];
                let color = color as usize;
                let r = u64::from(MONOCHROME[color][0]) * 0x0101;
                let g = u64::from(MONOCHROME[color][1]) * 0x0101;
                let b = u64::from(MONOCHROME[color][2]) * 0x0101;
                r << 32 | g << 16 | b
            }

            "Game Boy Pocket" => {
                const MONOCHROME: [[u8; 3]; 4] = [
                    [0xe0, 0xdb, 0xcd],
                    [0xa8, 0x9f, 0x94],
                    [0x70, 0x6b, 0x66],
                    [0x2b, 0x2b, 0x26],
                ];
                let color = color as usize;
                let r = u64::from(MONOCHROME[color][0]) * 0x0101;
                let g = u64::from(MONOCHROME[color][1]) * 0x0101;
                let b = u64::from(MONOCHROME[color][2]) * 0x0101;
                r << 32 | g << 16 | b
            }

            "RGB" => {
                const MONOCHROME: [[u8; 3]; 4] = [
                    [0xff, 0xff, 0xff],
                    [0xaa, 0xaa, 0xaa],
                    [0x55, 0x55, 0x55],
                    [0x00, 0x00, 0x00],
                ];
                let color = color as usize;
                let r = u64::from(MONOCHROME[color][0]) * 0x0101;
                let g = u64::from(MONOCHROME[color][1]) * 0x0101;
                let b = u64::from(MONOCHROME[color][2]) * 0x0101;
                r << 32 | g << 16 | b
            }
            _ => 0,
        }
    }

    pub fn color_game_boy_color(&self, color: u32) -> u64 {
        let r = color.get_bits(0, 5);
        let g = color.get_bits(5, 10);
        let b = color.get_bits(10, 15);

        let mut r = normalize(u64::from(r), 5, 16);
        let mut g = normalize(u64::from(g), 5, 16);
        let mut b = normalize(u64::from(b), 5, 16);

        if self.color_emulation_cgb {
            let temp_r = r * 26 + g * 4 + b * 2;
            let temp_g = g * 24 + b * 8;
            let temp_b = r * 6 + g * 4 + b * 22;
            r = normalize(min(960, temp_r), 10, 16);
            g = normalize(min(960, temp_g), 10, 16);
            b = normalize(min(960, temp_b), 10, 16);
        }
        return r << 32 | g << 16 | b;
    }
}
