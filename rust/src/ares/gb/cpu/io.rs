use ares::emulator::types::{U13, U3, U4};
use ares::gb::cpu::CPU;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, WrappingAdd};
use malachite_base::num::basic::traits::Iverson;

// See higan-rust/cpp/ares/gb/cpu/io.cpp
impl CPU {
    pub fn wram_address(&self, address: U13) -> u16 {
        if address.x() < 0x1000 {
            return address.x();
        }
        let bank = self
            .status
            .wram_bank
            .wrapping_add(U3::iverson(self.status.wram_bank.x() == 0));
        u16::from(bank.x()) << 12 | address.x().mod_power_of_2(12)
    }

    pub fn input(&mut self, data: U4) {
        self.status.joyp = data;
    }
}
