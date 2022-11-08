use ares::emulator::types::U13;
use ares::gb::ppu::PPU;
use ares::gb::system::{Model, System};
use ares::platform::Platform;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::traits::Iverson;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::{BitAccess, BitBlockAccess};

impl PPU {
    pub fn vram_address(&self, address: U13) -> u16 {
        (u16::iverson(self.status.vram_bank) << 13) | address.x()
    }
}

impl<P: Platform> System<P> {
    pub fn ppu_read_io(&self, cycle: u32, address: u16, mut data: u8) -> u8 {
        match (address, cycle, self.model) {
            (address, 2, _) if (0x8000..=0x9fff).contains(&address) => {
                if !self.ppu.can_access_vram() {
                    data
                } else {
                    self.ppu.vram[usize::from(self.ppu.vram_address(U13::wrapping_from(address)))]
                }
            }
            (address, 2, _) if (0xfe00..=0xfe9f).contains(&address) => {
                if !self.ppu.can_access_oam() {
                    data
                } else {
                    self.ppu.oam[usize::from(u8::wrapping_from(address))]
                }
            }
            (address, 2, _) if !(0xff40..=0xff7f).contains(&address) => data,
            (0xff40, 2, _) => {
                data.assign_bit(0, self.ppu.status.bg_enable);
                data.assign_bit(1, self.ppu.status.ob_enable);
                data.assign_bit(2, self.ppu.status.ob_size);
                data.assign_bit(3, self.ppu.status.bg_tilemap_select);
                data.assign_bit(4, self.ppu.status.bg_tiledata_select);
                data.assign_bit(5, self.ppu.status.window_display_enable);
                data.assign_bit(6, self.ppu.status.window_tilemap_select);
                data.assign_bit(7, self.ppu.status.display_enable);
                data
            }
            (0xff41, 2, _) => {
                data.assign_bit(0, self.ppu.history.mode.get_bit(8));
                data.assign_bit(1, self.ppu.history.mode.get_bit(9));
                data.assign_bit(2, self.ppu_compare_lyc());
                data.assign_bit(3, self.ppu.status.interrupt_h_blank);
                data.assign_bit(4, self.ppu.status.interrupt_v_blank);
                data.assign_bit(5, self.ppu.status.interrupt_oam);
                data.assign_bit(6, self.ppu.status.interrupt_lyc);
                data.assign_bit(7, true);
                data
            }
            (0xff42, 2, _) => self.ppu.status.scy,
            (0xff43, 2, _) => self.ppu.status.scx,
            (0xff44, 2, _) => self.ppu_get_ly(),
            (0xff45, 2, _) => self.ppu.status.lyc,
            (0xff46, 2, _) => self.ppu.status.dma_bank,
            (0xff47, 2, _) => {
                data.assign_bits(0, 2, &self.ppu.bgp[0].x());
                data.assign_bits(2, 4, &self.ppu.bgp[1].x());
                data.assign_bits(4, 6, &self.ppu.bgp[2].x());
                data.assign_bits(6, 8, &self.ppu.bgp[3].x());
                data
            }
            (0xff48, 2, _) => {
                data.assign_bits(0, 2, &self.ppu.obp[0].x());
                data.assign_bits(2, 4, &self.ppu.obp[1].x());
                data.assign_bits(4, 6, &self.ppu.obp[2].x());
                data.assign_bits(6, 8, &self.ppu.obp[3].x());
                data
            }
            (0xff49, 2, _) => {
                data.assign_bits(0, 2, &self.ppu.obp[4].x());
                data.assign_bits(2, 4, &self.ppu.obp[5].x());
                data.assign_bits(4, 6, &self.ppu.obp[6].x());
                data.assign_bits(6, 8, &self.ppu.obp[7].x());
                data
            }
            (0xff4a, 2, _) => self.ppu.status.wy,
            (0xff4b, 2, _) => self.ppu.status.wx,
            (0xff4f, 2, Model::GameBoyColor) => u8::iverson(self.ppu.status.vram_bank),
            (0xff68, 2, Model::GameBoyColor) => {
                data.assign_bits(0, 6, &self.ppu.status.bgpi.x());
                data.assign_bit(7, self.ppu.status.bgpi_increment);
                data
            }
            (0xff69, 2, Model::GameBoyColor) => {
                let b = if self.ppu.status.bgpi.even() { 0 } else { 8 };
                u8::wrapping_from(
                    self.ppu.bgpd[usize::from(self.ppu.status.bgpi.x() >> 1)].get_bits(b, b + 8),
                )
            }
            (0xff6a, 2, Model::GameBoyColor) => {
                data.assign_bits(0, 6, &self.ppu.status.obpi);
                data.assign_bit(7, self.ppu.status.obpi_increment);
                data
            }
            (0xff6b, 2, Model::GameBoyColor) => {
                let b = if self.ppu.status.obpi.even() { 0 } else { 8 };
                u8::wrapping_from(
                    self.ppu.obpd[usize::from(self.ppu.status.obpi >> 1)].get_bits(b, b + 8),
                )
            }
            _ => data,
        }
    }
}
