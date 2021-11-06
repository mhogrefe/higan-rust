use ares::emulator::types::{U13, U4};
use ares::gb::ppu::{hflip, PPU};
use malachite_base::num::arithmetic::traits::{ModPowerOf2, WrappingAddAssign, WrappingSubAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::{BitAccess, BitBlockAccess};

impl PPU {
    pub fn read_tile_dmg(&self, select: bool, x: u32, y: u32, tiledata: &mut u16) {
        let mut tilemap_address = U13::new(if select { 0x1b00 } else { 0x1800 });
        tilemap_address += U13::wrapping_from(((y >> 3 << 5) + (x >> 3)).mod_power_of_2(10));
        let tilemap_address = usize::from(tilemap_address.x());
        let tile = self.vram[tilemap_address];
        let mut tiledata_address;
        if !self.status.bg_tiledata_select {
            tiledata_address = U13::new(0x1000);
            let tile = i8::wrapping_from(tile);
            let tile_abs = tile.unsigned_abs();
            if tile >= 0 {
                tiledata_address.wrapping_add_assign(U13::from(tile_abs));
            } else {
                tiledata_address.wrapping_sub_assign(U13::from(tile_abs));
            }
        } else {
            tiledata_address = U13::from(tile) << 4;
        }
        tiledata_address.wrapping_add_assign(U13::wrapping_from((y & 7) << 1));
        let tiledata_address = usize::from(tiledata_address.x());
        tiledata.assign_bits(0, 8, &u16::from(self.vram[tiledata_address]));
        tiledata.assign_bits(8, 16, &u16::from(self.vram[tiledata_address + 1]));
    }

    pub fn scanline_dmg(&mut self) {
        self.px = 0;
        let height = if !self.status.ob_size { 8i32 } else { 16 };
        self.sprites = U4::ZERO;
        //find first ten sprites on this scanline
        let mut n = 0;
        while n < 40 * 4 {
            let s = &mut self.sprite[usize::from(self.sprites.x())];
            s.y = i16::from(self.oam[n]) - 16;
            s.x = i16::from(self.oam[n + 1]) - 8;
            s.tile = self.oam[n + 2] & if self.status.ob_size { 0xfe } else { 0xff };
            s.attributes = self.oam[n + 3];
            if i32::wrapping_from(self.status.ly) < i32::from(s.y) {
                continue;
            }
            if i32::wrapping_from(self.status.ly) >= i32::from(s.y) + height {
                continue;
            }
            s.y = i16::wrapping_from(i32::from(self.status.ly) - i32::from(s.y));
            if s.attributes.get_bit(6) {
                s.y ^= i16::wrapping_from(height) - 1
            };
            let tiledata_address = U13::wrapping_from(
                (u16::from(s.tile) << 4).wrapping_add(u16::wrapping_from(s.y << 1)),
            );
            let tiledata_address = usize::from(tiledata_address.x());
            s.tiledata
                .assign_bits(0, 8, &u16::from(self.vram[tiledata_address]));
            s.tiledata
                .assign_bits(8, 16, &u16::from(self.vram[tiledata_address + 1]));
            if s.attributes.get_bit(5) {
                s.tiledata = hflip(s.tiledata)
            };
            self.sprites += U4::ONE;
            if self.sprites.x() == 10 {
                break;
            }
            n += 4;
        }
        //sort by X-coordinate
        self.sprite[..usize::from(self.sprites.x())].sort_by(|l, r| l.x.cmp(&r.x));
    }
}
