use ares::emulator::types::{U13, U3, U4};
use ares::gb::cpu::{Interrupt, CPU};
use ares::gb::system::{Model, System};
use ares::platform::Platform;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, WrappingAdd};
use malachite_base::num::basic::traits::Iverson;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::BitAccess;

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

impl<P: Platform> System<P> {
    pub fn cpu_joyp_poll(&mut self) {
        if self.model != Model::SuperGameBoy {
            self.controls_poll();
            let mut dpad = U4::ZERO;
            dpad.assign_bit(0, !self.controls.right_latch);
            dpad.assign_bit(1, !self.controls.left_latch);
            dpad.assign_bit(2, !self.controls.up_latch);
            dpad.assign_bit(3, !self.controls.down_latch);

            let mut button = U4::ZERO;
            button.assign_bit(0, !self.controls.a.as_button_ref().value);
            button.assign_bit(1, !self.controls.b.as_button_ref().value);
            button.assign_bit(2, !self.controls.select.as_button_ref().value);
            button.assign_bit(3, !self.controls.start.as_button_ref().value);

            self.cpu.status.joyp = U4::new(0xf);
            if !self.cpu.status.p14 {
                self.cpu.status.joyp &= dpad;
            }
            if !self.cpu.status.p15 {
                self.cpu.status.joyp &= button;
            }
        }

        if self.cpu.status.joyp.x() != 0xf {
            self.cpu.raise(Interrupt::Joypad.value());
        }
    }
}
