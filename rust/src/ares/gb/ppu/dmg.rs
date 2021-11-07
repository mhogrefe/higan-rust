use ares::emulator::types::{U13, U15, U4};
use ares::gb::ppu::{hflip, Pixel, PPU};
use ares::gb::system::{Model, System};
use ares::platform::Platform;
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

    pub fn run_background_dmg(&mut self) {
        let scroll_y = self.status.ly.wrapping_add(self.status.scy);
        let scroll_x = self.px.wrapping_add(self.status.scx);
        let tile_x = scroll_x.mod_power_of_2(3);
        if tile_x == 0 || self.px == 0 {
            let mut td = self.background.tiledata;
            self.read_tile_dmg(
                self.status.bg_tilemap_select,
                u32::from(scroll_x),
                u32::from(scroll_y),
                &mut td,
            );
            self.background.tiledata = td;
        }
        let mut index = 0;
        index.assign_bit(0, self.background.tiledata.get_bit(u64::from(7 - tile_x)));
        index.assign_bit(1, self.background.tiledata.get_bit(u64::from(15 - tile_x)));
        self.bg.color = U15::from(self.bgp[usize::from(index)].x());
        self.bg.palette = index;
    }

    pub fn run_window_dmg(&mut self) {
        if self.status.ly < self.status.wy {
            return;
        }
        if self.px + 7 < self.status.wx {
            return;
        }
        if self.px + 7 == self.status.wx {
            self.latch.wy.wrapping_add_assign(1);
        }
        if !self.status.bg_enable {
            return;
        }
        let scroll_y = self.latch.wy.wrapping_sub(1);
        let scroll_x = (self.px + 7).wrapping_sub(self.latch.wx);
        let tile_x = scroll_x.mod_power_of_2(3);
        if tile_x == 0 || self.px == 0 {
            let mut td = self.background.tiledata;
            self.read_tile_dmg(
                self.status.window_tilemap_select,
                u32::from(scroll_x),
                u32::from(scroll_y),
                &mut td,
            );
            self.background.tiledata = td;
        }
        let mut index = 0;
        index.assign_bit(0, self.window.tiledata.get_bit(u64::from(7 - tile_x)));
        index.assign_bit(1, self.window.tiledata.get_bit(u64::from(15 - tile_x)));
        self.bg.color = U15::from(self.bgp[usize::from(index)].x());
        self.bg.palette = index;
    }

    pub fn run_objects_dmg(&mut self) {
        // render backwards, so that first sprite has priority
        for s in self.sprite[..usize::from(self.sprites.x())]
            .iter_mut()
            .rev()
        {
            let tile_x = i32::from(self.px) - i32::from(s.x);
            if tile_x < 0 || tile_x > 7 {
                continue;
            }
            let mut index = 0;
            index.assign_bit(0, s.tiledata.get_bit(u64::wrapping_from(7 - tile_x)));
            index.assign_bit(1, s.tiledata.get_bit(u64::wrapping_from(15 - tile_x)));
            if index == 0 {
                continue;
            }
            let palette = (if s.attributes.get_bit(4) { 4 } else { 0 }) | index;
            self.ob.color = U15::from(self.obp[usize::from(palette)].x());
            self.ob.palette = index;
            self.ob.priority = !s.attributes.get_bit(7);
        }
    }
}

impl<P: Platform> System<P> {
    pub fn ppu_run_dmg(&mut self) {
        self.ppu.bg = Pixel::default();
        self.ppu.ob = Pixel::default();
        if self.ppu.status.bg_enable {
            self.ppu.run_background_dmg();
        }
        if self.ppu.latch.window_display_enable {
            self.ppu.run_window_dmg();
        }
        if self.ppu.status.ob_enable {
            self.ppu.run_objects_dmg();
        }
        let _color = if self.ppu.ob.palette == 0 {
            self.ppu.bg.color
        } else if self.ppu.bg.palette == 0 {
            self.ppu.ob.color
        } else if self.ppu.ob.priority {
            self.ppu.ob.color
        } else {
            self.ppu.bg.color
        };
        if self.model == Model::GameBoy {
            // TODO auto output = screen->pixels().data() + status.ly * 160 + px++;
            // TODO //LCD is still blank during the first frame
            // TODO if(!latch.displayEnable) *output = color;
        }
    }
}
