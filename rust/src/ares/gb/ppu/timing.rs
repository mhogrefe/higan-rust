use ares::emulator::types::U9;
use ares::gb::ppu::PPU;
use ares::gb::system::{Model, System};
use ares::platform::Platform;
use malachite_base::num::logic::traits::BitBlockAccess;

impl PPU {
    pub fn can_access_vram(&self) -> bool {
        if !self.status.display_enable {
            true
        } else if self.history.mode.get_bits(4, 6).x() == 3 {
            false
        } else if self.history.mode.get_bits(4, 6).x() == 2 && self.status.lx.x() >> 2 == 20 {
            false
        } else {
            true
        }
    }

    pub fn can_access_oam(&self) -> bool {
        if !self.status.display_enable {
            true
        } else if self.status.dma_active && self.status.dma_clock.x() >= 8 {
            false
        } else if self.history.mode.get_bits(4, 6).x() == 2 {
            false
        } else if self.history.mode.get_bits(4, 6).x() == 3 {
            false
        } else if self.status.ly != 0 && self.status.lx.x() >> 2 == 0 {
            false
        } else {
            true
        }
    }
}

impl<P: Platform> System<P> {
    pub fn ppu_compare_lyc(&self) -> bool {
        let ly = self.ppu.status.ly;
        let lyc = self.ppu.status.lyc;
        if self.model == Model::GameBoy || self.model == Model::SuperGameBoy {
            let lx: U9 = self.ppu.status.lx >> 2;
            if ly != 0 && lx.x() == 0 {
                return false;
            } else if ly == 153 && lx.x() == 2 {
                return false;
            } else if ly == 153 && lx.x() >= 3 {
                return lyc == 0;
            }
        }
        if self.model == Model::GameBoyColor && self.cpu.low_speed() {
            let lx: U9 = self.ppu.status.lx >> 2;
            if ly == 153 && lx.x() >= 1 {
                return lyc == 0;
            }
        }
        if self.model == Model::GameBoyColor && self.cpu.high_speed() {
            let lx: U9 = self.ppu.status.lx >> 1;
            if ly != 0 && lx.x() == 0 {
                return lyc == ly - 1;
            };
            if ly == 153 && lx.x() >= 8 {
                return lyc == 0;
            };
        }
        lyc == ly
    }

    pub fn ppu_get_ly(&self) -> u8 {
        let mut ly = self.ppu.status.ly;
        if self.model == Model::GameBoy || self.model == Model::SuperGameBoy {
            let lx: U9 = self.ppu.status.lx >> 2;
            if ly == 153 && lx.x() >= 1 {
                return 0;
            }
        }
        if self.model == Model::GameBoyColor && self.cpu.low_speed() {
            let lx: U9 = self.ppu.status.lx >> 2;
            if ly != 153 && lx.x() >= 113 {
                const PATTERN: [u8; 8] = [0, 0, 2, 0, 4, 4, 6, 0];
                if ly.get_bits(0, 4) != 0xf {
                    ly.assign_bits(0, 3, &PATTERN[usize::from(ly.get_bits(0, 3))]);
                } else {
                    ly.assign_bits(0, 4, &0);
                    ly.assign_bits(4, 7, &PATTERN[usize::from(ly.get_bits(4, 7))]);
                }
                return ly;
            }
            if ly == 153 && lx.x() >= 1 {
                return 0;
            }
        }
        if self.model == Model::GameBoyColor && self.cpu.high_speed() {
            let lx: U9 = self.ppu.status.lx >> 1;
            if ly == 153 && lx.x() >= 6 {
                return 0;
            }
        }
        ly
    }

    pub fn ppu_trigger_oam(&self) -> bool {
        if self.ppu.status.mode.x() != 2 {
            return false;
        }
        let lx: U9 = self.ppu.status.lx >> if self.cpu.status.speed_double { 1 } else { 2 };
        lx.x() == (if self.ppu.status.ly == 0 { 1 } else { 0 })
    }
}
