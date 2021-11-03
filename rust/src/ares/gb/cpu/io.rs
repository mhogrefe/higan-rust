use ares::emulator::types::{U13, U3, U4};
use ares::gb::cpu::{Interrupt, CPU};
use ares::gb::system::{Model, System};
use ares::platform::Platform;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, WrappingAdd};
use malachite_base::num::basic::traits::Iverson;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::{BitAccess, BitBlockAccess};

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

    pub fn cpu_read_io(&mut self, cycle: u32, address: u16, mut data: u8) -> u8 {
        if address <= 0xbfff {
            data
        } else if address >= 0xc000 && address <= 0xfdff && cycle == 2 {
            self.cpu.wram[self.cpu.wram_address(U13::wrapping_from(address)) as usize]
        } else if address >= 0xff80 && address <= 0xfffe && cycle == 2 {
            self.cpu.hram[address.mod_power_of_2(7) as usize]
        } else if address == 0xff00 && cycle == 2 {
            //JOYP
            self.cpu_joyp_poll();
            data.assign_bit(0, self.cpu.status.joyp.get_bit(0));
            data.assign_bit(1, self.cpu.status.joyp.get_bit(1));
            data.assign_bit(2, self.cpu.status.joyp.get_bit(2));
            data.assign_bit(3, self.cpu.status.joyp.get_bit(3));
            data.assign_bit(4, self.cpu.status.p14);
            data.assign_bit(5, self.cpu.status.p15);
            data
        } else if address == 0xff01 && cycle == 2 {
            //SB
            return self.cpu.status.serial_data;
        } else if address == 0xff02 && cycle == 2 {
            //SC
            data.assign_bit(0, self.cpu.status.serial_clock);
            data.assign_bit(
                1,
                self.cpu.status.serial_speed || self.model != Model::GameBoyColor,
            );
            data.assign_bit(7, self.cpu.status.serial_transfer);
            data
        } else if address == 0xff04 && cycle == 2 {
            //DIV
            u8::wrapping_from(self.cpu.status.div.get_bits(8, 16))
        } else if address == 0xff05 && cycle == 2 {
            //TIMA
            self.cpu.status.tima
        } else if address == 0xff06 && cycle == 2 {
            //TMA
            self.cpu.status.tma
        } else if address == 0xff07 && cycle == 2 {
            //TAC
            data.assign_bit(0, self.cpu.status.timer_clock.get_bit(0));
            data.assign_bit(1, self.cpu.status.timer_clock.get_bit(1));
            data.assign_bit(2, self.cpu.status.timer_enable);
            data
        } else if address == 0xff0f && cycle == 2 {
            //IF
            data.assign_bit(0, self.cpu.status.interrupt_flag.get_bit(0));
            data.assign_bit(1, self.cpu.status.interrupt_flag.get_bit(1));
            data.assign_bit(2, self.cpu.status.interrupt_flag.get_bit(2));
            data.assign_bit(3, self.cpu.status.interrupt_flag.get_bit(3));
            data.assign_bit(4, self.cpu.status.interrupt_flag.get_bit(4));
            data
        } else if self.model == Model::GameBoyColor && address == 0xff4d && cycle == 2 {
            //KEY1
            data.assign_bit(0, self.cpu.status.speed_switch);
            data.assign_bit(7, self.cpu.status.speed_double);
            data
        } else if self.model == Model::GameBoyColor && address == 0xff55 && cycle == 2 {
            //HDMA5
            data.assign_bits(0, 7, &self.cpu.status.dma_length.x());
            data.assign_bit(7, !self.cpu.status.hdma_active);
            data
        } else if self.model == Model::GameBoyColor && address == 0xff56 && cycle == 2 {
            //RP
            //unemulated
            0x02
        } else if self.model == Model::GameBoyColor && address == 0xff6c && cycle == 2 {
            //???
            data.assign_bit(0, self.cpu.status.ff6c);
            data
        } else if self.model == Model::GameBoyColor && address == 0xff70 && cycle == 2 {
            //???
            self.cpu.status.wram_bank.x()
        } else if self.model == Model::GameBoyColor && address == 0xff72 && cycle == 2 {
            //???
            self.cpu.status.ff72
        } else if self.model == Model::GameBoyColor && address == 0xff73 && cycle == 2 {
            //???
            self.cpu.status.ff73
        } else if self.model == Model::GameBoyColor && address == 0xff74 && cycle == 2 {
            //???
            self.cpu.status.ff74
        } else if self.model == Model::GameBoyColor && address == 0xff76 && cycle == 2 {
            //???
            data.assign_bits(4, 7, &self.cpu.status.ff75.x());
            data
        } else if self.model == Model::GameBoyColor && address == 0xff77 && cycle == 2 {
            //???
            0xff
        } else if self.model == Model::GameBoyColor && address == 0xffff && cycle == 2 {
            //IE
            self.cpu.status.interrupt_enable
        } else {
            data
        }
    }

    pub fn s_cpu_write_io(&mut self, _cycle: u32, _address: u16, _data: u8) {
        unimplemented!()
    }
}
