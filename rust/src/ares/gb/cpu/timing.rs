use ares::emulator::types::{U13, U4, U7};
use ares::gb::cpu::Interrupt;
use ares::gb::cpu::CPU;
use ares::gb::system::{Model, System};
use ares::platform::Platform;
use malachite_base::num::arithmetic::traits::{
    DivisibleByPowerOf2, Parity, WrappingAddAssign, WrappingSubAssign,
};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::WrappingFrom;

impl<P: Platform> System<P> {
    // synchronized
    pub fn s_cpu_step(&mut self, clocks: u32) {
        let start = if self.cpu_resuming_execution {
            self.cpu_resuming_execution = false;
            self.cpu_local_u32s.pop()
        } else {
            0
        };
        for i in start..clocks {
            self.cpu.status.div.wrapping_add_assign(1);
            if self.cpu.status.div.divisible_by_power_of_2(4) {
                self.cpu.timer_262144_hz();
            }
            if self.cpu.status.div.divisible_by_power_of_2(6) {
                self.cpu.timer_65536_hz();
            }
            if self.cpu.status.div.divisible_by_power_of_2(8) {
                self.cpu.timer_16384_hz();
            }
            if self.cpu.status.div.divisible_by_power_of_2(9) {
                self.cpu.timer_8192_hz();
            }
            if self.cpu.status.div.divisible_by_power_of_2(10) {
                self.cpu.timer_4096_hz();
            }
            if self.cpu.status.div.divisible_by_power_of_2(12) {
                self.cpu_timer_1024_hz();
            }
            self.cpu_thread.step(1);
            if self.cpu_is_sync_needed() {
                self.cpu_pausing_execution = true;
                self.cpu_local_u32s.push(i + 1);
                return;
            }
        }
        if self.model == Model::SuperGameBoy {
            self.information.clocks_executed += clocks;
        }
    }
}

impl CPU {
    pub fn timer_262144_hz(&mut self) {
        if self.status.timer_enable && self.status.timer_clock.x() == 1 {
            self.status.tima.wrapping_add_assign(1);
            if self.status.tima == 0 {
                self.status.tima = self.status.tma;
                self.raise(Interrupt::Timer.value());
            }
        }
    }

    pub fn timer_65536_hz(&mut self) {
        if self.status.timer_enable && self.status.timer_clock.x() == 2 {
            self.status.tima.wrapping_add_assign(1);
            if self.status.tima == 0 {
                self.status.tima = self.status.tma;
                self.raise(Interrupt::Timer.value());
            }
        }
    }

    pub fn timer_16384_hz(&mut self) {
        if self.status.timer_enable && self.status.timer_clock.x() == 3 {
            self.status.tima.wrapping_add_assign(1);
            if self.status.tima == 0 {
                self.status.tima = self.status.tma;
                self.raise(Interrupt::Timer.value());
            }
        }
    }

    pub fn timer_8192_hz(&mut self) {
        if self.status.serial_transfer && self.status.serial_clock {
            self.status.serial_data <<= 1;
            self.status.serial_data |= 1;
            self.status.serial_bits.wrapping_sub_assign(U4::ONE);
            if self.status.serial_bits.x() == 0 {
                self.status.serial_transfer = false;
                self.raise(Interrupt::Serial.value());
            }
        }
    }

    pub fn timer_4096_hz(&mut self) {
        if self.status.timer_enable && self.status.timer_clock.x() == 0 {
            self.status.tima.wrapping_add_assign(1);
            if self.status.tima == 0 {
                self.status.tima = self.status.tma;
                self.raise(Interrupt::Timer.value());
            }
        }
    }

    pub fn h_blank(&mut self) {
        self.status.h_blank_pending = true;
    }
}

impl<P: Platform> System<P> {
    pub fn cpu_timer_1024_hz(&mut self) {
        self.cpu_joyp_poll();
    }

    pub fn s_cpu_h_blank_trigger(&mut self) {
        if self.cpu.status.hdma_active && self.ppu.status.ly < 144 {
            for i in 0..16 {
                let r = self.cpu_read_dma(self.cpu.status.dma_source, 0xff);
                self.s_cpu_write_dma(U13::wrapping_from(self.cpu.status.dma_target), r);
                self.cpu.status.dma_target.wrapping_add_assign(1);
                self.cpu.status.dma_source.wrapping_add_assign(1);
                if i.odd() {
                    self.s_cpu_step(if self.cpu.status.speed_double { 2 } else { 1 });
                }
            }
            let b = self.cpu.status.dma_length.x() == 0;
            self.cpu.status.dma_length.wrapping_sub_assign(U7::ONE);
            if b {
                self.cpu.status.hdma_active = false;
            }
        }
    }
}
