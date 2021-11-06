use ares::emulator::types::{U13, U2, U3, U4, U5, U7};
use ares::gb::cpu::{Interrupt, CPU};
use ares::gb::system::{Model, System, ThreadState};
use ares::platform::Platform;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, WrappingAdd, WrappingAddAssign, WrappingSubAssign,
};
use malachite_base::num::basic::traits::Iverson;
use malachite_base::num::basic::traits::{One, Zero};
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
        match (address, cycle, self.model) {
            (address, _, _) if address <= 0xbfff => data,
            (address, 2, _) if (0xc000..=0xfdff).contains(&address) => {
                self.cpu.wram[self.cpu.wram_address(U13::wrapping_from(address)) as usize]
            }
            (address, 2, _) if (0xff80..=0xfffe).contains(&address) => {
                self.cpu.hram[address.mod_power_of_2(7) as usize]
            }
            (0xff00, 2, _) => {
                self.cpu_joyp_poll();
                data.assign_bit(0, self.cpu.status.joyp.get_bit(0));
                data.assign_bit(1, self.cpu.status.joyp.get_bit(1));
                data.assign_bit(2, self.cpu.status.joyp.get_bit(2));
                data.assign_bit(3, self.cpu.status.joyp.get_bit(3));
                data.assign_bit(4, self.cpu.status.p14);
                data.assign_bit(5, self.cpu.status.p15);
                data
            }
            (0xff01, 2, _) => self.cpu.status.serial_data,
            (0xff02, 2, _) => {
                data.assign_bit(0, self.cpu.status.serial_clock);
                data.assign_bit(
                    1,
                    self.cpu.status.serial_speed || self.model != Model::GameBoyColor,
                );
                data.assign_bit(7, self.cpu.status.serial_transfer);
                data
            }
            (0xff04, 2, _) => u8::wrapping_from(self.cpu.status.div.get_bits(8, 16)),
            (0xff05, 2, _) => self.cpu.status.tima,
            (0xff06, 2, _) => self.cpu.status.tma,
            (0xff07, 2, _) => {
                data.assign_bit(0, self.cpu.status.timer_clock.get_bit(0));
                data.assign_bit(1, self.cpu.status.timer_clock.get_bit(1));
                data.assign_bit(2, self.cpu.status.timer_enable);
                data
            }
            (0xff0f, 2, _) => {
                data.assign_bit(0, self.cpu.status.interrupt_flag.get_bit(0));
                data.assign_bit(1, self.cpu.status.interrupt_flag.get_bit(1));
                data.assign_bit(2, self.cpu.status.interrupt_flag.get_bit(2));
                data.assign_bit(3, self.cpu.status.interrupt_flag.get_bit(3));
                data.assign_bit(4, self.cpu.status.interrupt_flag.get_bit(4));
                data
            }
            (0xff4d, 2, Model::GameBoyColor) => {
                data.assign_bit(0, self.cpu.status.speed_switch);
                data.assign_bit(7, self.cpu.status.speed_double);
                data
            }
            (0xff55, 2, Model::GameBoyColor) => {
                data.assign_bits(0, 7, &self.cpu.status.dma_length.x());
                data.assign_bit(7, !self.cpu.status.hdma_active);
                data
            }
            (0xff56, 2, Model::GameBoyColor) => 0x02,
            (0xff6c, 2, Model::GameBoyColor) => {
                //???
                data.assign_bit(0, self.cpu.status.ff6c);
                data
            }
            (0xff70, 2, Model::GameBoyColor) => self.cpu.status.wram_bank.x(),
            (0xff72, 2, Model::GameBoyColor) => self.cpu.status.ff72,
            (0xff73, 2, Model::GameBoyColor) => self.cpu.status.ff73,
            (0xff74, 2, Model::GameBoyColor) => self.cpu.status.ff74,
            (0xff76, 2, Model::GameBoyColor) => {
                data.assign_bits(4, 7, &self.cpu.status.ff75.x());
                data
            }
            (0xff77, 2, Model::GameBoyColor) => 0xff,
            (0xffff, 2, Model::GameBoyColor) => self.cpu.status.interrupt_enable,
            _ => data,
        }
    }

    // synchronized
    pub fn s_cpu_write_io(&mut self, cycle: u32, address: u16, data: u8) {
        let sync_point = if self.cpu_thread_state == ThreadState::Resuming {
            self.cpu_sync_points.pop()
        } else {
            0
        };
        match sync_point {
            0 => self.s_cpu_write_io_fresh(cycle, address, data),
            1 => self.s_cpu_write_io_resume_at_1(),
            2 => self.s_cpu_write_io_resume_at_2(),
            _ => panic!(),
        }
    }

    fn s_cpu_write_io_fresh(&mut self, cycle: u32, address: u16, data: u8) {
        match (address, cycle, self.model) {
            (address, _, _) if address <= 0xbfff => {}
            (address, 2, _) if address >= 0xc000 && address <= 0xfdff => {
                let a = self.cpu.wram_address(U13::wrapping_from(address));
                self.cpu.wram[a as usize] = data;
            }
            (address, 2, _) if address >= 0xff80 && address <= 0xfffe => {
                self.cpu.hram[address.mod_power_of_2(7) as usize] = data
            }
            (0xff00, 2, model) => {
                self.cpu.status.p14 = data.get_bit(4);
                self.cpu.status.p15 = data.get_bit(5);
                if model == Model::SuperGameBoy {
                    // TODO superGameBoy->joypWrite(self.cpu.status.p14, self.cpu.status.p15);
                }
            }
            (0xff01, 2, _) => self.cpu.status.serial_data = data,
            (0xff02, 2, _) => {
                self.cpu.status.serial_clock = data.get_bit(0);
                self.cpu.status.serial_speed = data.get_bit(1) && self.model == Model::GameBoyColor;
                self.cpu.status.serial_transfer = data.get_bit(7);
                if self.cpu.status.serial_transfer {
                    self.cpu.status.serial_bits = U4::new(8)
                };
            }
            (0xff04, 2, _) => self.cpu.status.div = 0,
            (0xff05, 2, _) => self.cpu.status.tima = data,
            (0xff06, 2, _) => self.cpu.status.tma = data,
            (0xff07, 2, _) => {
                self.cpu.status.timer_clock = U2::wrapping_from(data);
                self.cpu.status.timer_enable = data.get_bit(2);
            }
            (0xff0f, 2, _) => self.cpu.status.interrupt_flag = U5::wrapping_from(data),
            (0xff4d, 2, Model::GameBoyColor) => self.cpu.status.speed_switch = data.get_bit(0),
            (0xff51, 2, Model::GameBoyColor) => {
                self.cpu
                    .status
                    .dma_source
                    .assign_bits(8, 16, &u16::from(data))
            }
            (0xff52, 2, Model::GameBoyColor) => {
                self.cpu
                    .status
                    .dma_source
                    .assign_bits(4, 8, &u16::from(data.get_bits(4, 8)))
            }
            (0xff53, 2, Model::GameBoyColor) => {
                self.cpu
                    .status
                    .dma_target
                    .assign_bits(8, 16, &u16::from(data))
            }
            (0xff54, 2, Model::GameBoyColor) => {
                self.cpu
                    .status
                    .dma_target
                    .assign_bits(4, 8, &u16::from(data.get_bits(4, 8)))
            }
            (0xff55, 2, Model::GameBoyColor) => {
                //1->0 transition stops an active HDMA (and does not trigger GDMA)
                if self.cpu.status.hdma_active && !data.get_bit(7) {
                    self.cpu.status.dma_length = U7::wrapping_from(data);
                    self.cpu.status.hdma_active = false;
                    return;
                }
                self.cpu.status.dma_length = U7::wrapping_from(data);
                self.cpu.status.hdma_active = data.get_bit(7);
                //GDMA
                if !data.get_bit(7) {
                    loop {
                        for i in 0..16 {
                            let r = self.cpu_read_dma(self.cpu.status.dma_source, 0xff);
                            // ** S1
                            self.s_cpu_write_dma(U13::wrapping_from(self.cpu.status.dma_target), r);
                            if self.cpu_thread_state == ThreadState::Pausing {
                                self.cpu_sync_points.push(1);
                                self.cpu_local_u8s.push(r);
                                self.cpu_local_u8s.push(i);
                                return;
                            }

                            self.cpu.status.dma_target.wrapping_add_assign(1);
                            self.cpu.status.dma_source.wrapping_add_assign(1);
                        }
                        // ** S2
                        self.s_cpu_step(if self.cpu.status.speed_double { 16 } else { 8 });
                        if self.cpu_thread_state == ThreadState::Pausing {
                            self.cpu_sync_points.push(2);
                            return;
                        }

                        let b = self.cpu.status.dma_length.x() == 0;
                        self.cpu.status.dma_length.wrapping_sub_assign(U7::ONE);
                        if b {
                            break;
                        }
                    }
                }
            }
            (0xff56, 2, Model::GameBoyColor) => {}
            (0xff6c, 2, Model::GameBoyColor) => self.cpu.status.ff6c = data.get_bit(0),
            (0xff72, 2, Model::GameBoyColor) => self.cpu.status.ff72 = data,
            (0xff73, 2, Model::GameBoyColor) => self.cpu.status.ff73 = data,
            (0xff74, 2, Model::GameBoyColor) => self.cpu.status.ff74 = data,
            (0xff75, 2, Model::GameBoyColor) => {
                self.cpu.status.ff75 = U3::wrapping_from(data.get_bits(4, 7))
            }
            (0xff70, 2, Model::GameBoyColor) => self.cpu.status.wram_bank = U3::wrapping_from(data),
            (0xffff, 2, _) => self.cpu.status.interrupt_enable = data,
            _ => {}
        }
    }

    fn s_cpu_write_io_resume_at_1(&mut self) {
        loop {
            let initial_i = if self.cpu_thread_state == ThreadState::Resuming {
                self.cpu_local_u8s.pop()
            } else {
                0
            };
            for i in initial_i..16 {
                let r = if self.cpu_thread_state == ThreadState::Resuming {
                    self.cpu_local_u8s.pop()
                } else {
                    self.cpu_read_dma(self.cpu.status.dma_source, 0xff)
                };
                // ** S1
                self.s_cpu_write_dma(U13::wrapping_from(self.cpu.status.dma_target), r);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(1);
                    self.cpu_local_u8s.push(r);
                    self.cpu_local_u8s.push(i);
                    return;
                }

                self.cpu.status.dma_target.wrapping_add_assign(1);
                self.cpu.status.dma_source.wrapping_add_assign(1);
            }
            // ** S2
            self.s_cpu_step(if self.cpu.status.speed_double { 16 } else { 8 });
            if self.cpu_thread_state == ThreadState::Pausing {
                self.cpu_sync_points.push(2);
                return;
            }

            let b = self.cpu.status.dma_length.x() == 0;
            self.cpu.status.dma_length.wrapping_sub_assign(U7::ONE);
            if b {
                break;
            }
        }
    }

    fn s_cpu_write_io_resume_at_2(&mut self) {
        loop {
            if self.cpu_thread_state != ThreadState::Resuming {
                for i in 0..16 {
                    let r = self.cpu_read_dma(self.cpu.status.dma_source, 0xff);
                    // ** S1
                    self.s_cpu_write_dma(U13::wrapping_from(self.cpu.status.dma_target), r);
                    if self.cpu_thread_state == ThreadState::Pausing {
                        self.cpu_sync_points.push(1);
                        self.cpu_local_u8s.push(r);
                        self.cpu_local_u8s.push(i);
                        return;
                    }

                    self.cpu.status.dma_target.wrapping_add_assign(1);
                    self.cpu.status.dma_source.wrapping_add_assign(1);
                }
            }
            // ** S2
            self.s_cpu_step(if self.cpu.status.speed_double { 16 } else { 8 });
            if self.cpu_thread_state == ThreadState::Pausing {
                self.cpu_sync_points.push(2);
                return;
            }

            let b = self.cpu.status.dma_length.x() == 0;
            self.cpu.status.dma_length.wrapping_sub_assign(U7::ONE);
            if b {
                break;
            }
        }
    }
}
