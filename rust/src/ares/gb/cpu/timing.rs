//TODO test

use ares::emulator::types::U22;
use ares::gb::cpu::cpu::{Interrupt, CPU};
use malachite_base::num::arithmetic::traits::{WrappingAddAssign, WrappingSubAssign};
use malachite_base::num::basic::traits::One;

//70224 clocks/frame
//  456 clocks/scanline
//  154 scanlines/frame

impl CPU {
    pub fn step(&mut self, clocks: u32) {
        for _ in 0..clocks {
            self.bus.cpu_io.status.clock.wrapping_add_assign(U22::ONE);
            if self.bus.cpu_io.status.clock.x() == 0 {
                //TODO cartridge.second();
            }

            //4MHz / N(hz) - 1 = mask
            self.bus.cpu_io.status.div.wrapping_add_assign(1);
            if (self.bus.cpu_io.status.div & 15) == 0 {
                self.timer_262144_hz();
            }
            if (self.bus.cpu_io.status.div & 63) == 0 {
                self.timer_65536_hz();
            }
            if (self.bus.cpu_io.status.div & 255) == 0 {
                self.timer_16384_hz();
            }
            if (self.bus.cpu_io.status.div & 511) == 0 {
                self.timer_8192_hz();
            }
            if (self.bus.cpu_io.status.div & 1023) == 0 {
                self.timer_4096_hz();
            }

            //TODO Thread::step(1);
            //TODO synchronize(ppu);
            //TODO synchronize(apu);
        }

        //TODO if system.model_is_game_boy_color() {
        //TODO     system._clocksExecuted += clocks;
        //TODO     scheduler.exit(Scheduler::Event::Step);
        //TODO }
    }

    pub fn timer_262144_hz(&mut self) {
        if self.bus.cpu_io.status.timer_enable && self.bus.cpu_io.status.timer_clock == 1 {
            self.bus.cpu_io.status.tima.wrapping_add_assign(1);
            if self.bus.cpu_io.status.tima == 0 {
                self.bus.cpu_io.status.tima = self.bus.cpu_io.status.tma;
                self.raise(Interrupt::Timer);
            }
        }
    }

    pub fn timer_65536_hz(&mut self) {
        if self.bus.cpu_io.status.timer_enable && self.bus.cpu_io.status.timer_clock == 2 {
            self.bus.cpu_io.status.tima.wrapping_add_assign(1);
            if self.bus.cpu_io.status.tima == 0 {
                self.bus.cpu_io.status.tima = self.bus.cpu_io.status.tma;
                self.raise(Interrupt::Timer);
            }
        }
    }

    pub fn timer_16384_hz(&mut self) {
        if self.bus.cpu_io.status.timer_enable && self.bus.cpu_io.status.timer_clock == 3 {
            self.bus.cpu_io.status.tima.wrapping_add_assign(1);
            if self.bus.cpu_io.status.tima == 0 {
                self.bus.cpu_io.status.tima = self.bus.cpu_io.status.tma;
                self.raise(Interrupt::Timer);
            }
        }
    }

    pub fn timer_8192_hz(&mut self) {
        if self.bus.cpu_io.status.serial_transfer && self.bus.cpu_io.status.serial_clock {
            self.bus.cpu_io.status.serial_bits.wrapping_sub_assign(1);
            if self.bus.cpu_io.status.serial_bits == 0 {
                self.bus.cpu_io.status.serial_transfer = false;
                self.raise(Interrupt::Serial);
            }
        }
    }

    pub fn timer_4096_hz(&mut self) {
        if self.bus.cpu_io.status.timer_enable && self.bus.cpu_io.status.timer_clock == 0 {
            self.bus.cpu_io.status.tima.wrapping_add_assign(1);
            if self.bus.cpu_io.status.tima == 0 {
                self.bus.cpu_io.status.tima = self.bus.cpu_io.status.tma;
                self.raise(Interrupt::Timer);
            }
        }
    }

    /*
    pub fn hblank(&mut self) {
        if self.bus.cpu_io.status.dma_mode && self.bus.cpu_io.status.dma_length != 0
        /* TODO && ppu.status.ly < 144*/
        {
            for n in 0..16 {
                let dma_target = self.bus.cpu_io.status.dma_target;
                let dma_source = self.bus.cpu_io.status.dma_source;
                let read_result = self.bus.read_dma(dma_source);
                self.bus_write_dma(dma_target, read_result);
                self.bus.cpu_io.status.dma_target.wrapping_add_assign(1);
                self.bus.cpu_io.status.dma_source.wrapping_add_assign(1);
                self.bus.cpu_io.status.dma_length.wrapping_sub_assign(1);
                if (n & 1) != 0 {
                    let clocks = 1
                        << if self.bus.cpu_io.status.speed_double {
                            1
                        } else {
                            0
                        };
                    self.step(clocks);
                }
            }
        }
    }*/
}
