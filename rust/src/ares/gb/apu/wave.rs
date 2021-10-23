use ares::emulator::types::{U11, U2, U3, U4, U5};
use malachite_base::num::arithmetic::traits::{Parity, WrappingAddAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::BitAccess;

/// See higan-rust/cpp/ares/gb/apu/apu.hpp
#[derive(Clone, Debug, Default)]
pub struct Wave {
    pub enable: bool,

    pub dac_enable: bool,
    pub volume: U2,
    pub frequency: U11,
    pub counter: bool,
    pub pattern: [u8; 16],

    pub output: i16,
    pub length: u32,
    pub period: u32,
    pub pattern_offset: U5,
    pub pattern_sample: U4,
    pub pattern_hold: u32,
}

impl Wave {
    /// See cpp/ares/gb/apu/wave.cpp
    pub fn get_pattern(&self, offset: U5) -> U4 {
        let mut p = self.pattern[(offset.x() >> 1) as usize];
        if offset.even() {
            p >>= 4;
        }
        U4::wrapping_from(p)
    }

    /// See cpp/ares/gb/apu/wave.cpp
    pub fn run(&mut self) {
        if self.pattern_hold != 0 {
            self.pattern_hold -= 1;
        }
        if self.period != 0 {
            self.period -= 1;
            if self.period == 0 {
                self.period = 2_048 - u32::from(self.frequency);
                self.pattern_offset.wrapping_add_assign(U5::ONE);
                self.pattern_sample = self.get_pattern(self.pattern_offset);
                self.pattern_hold = 1;
            }
        }
        const SHIFT: [u32; 4] = [4, 0, 1, 2]; // 0%, 100%, 50%, 25%
        self.output = if self.enable {
            i16::from(self.pattern_sample >> SHIFT[self.volume.x() as usize])
        } else {
            0
        };
    }

    /// See cpp/ares/gb/apu/wave.cpp
    pub fn clock_length(&mut self) {
        if self.counter && self.length != 0 {
            self.length -= 1;
            if self.length == 0 {
                self.enable = false;
            }
        }
    }

    //TODO test
    /// See cpp/ares/gb/apu/wave.cpp
    pub fn trigger(&mut self, model_is_game_boy_color: bool, apu_phase: U3) {
        if !model_is_game_boy_color && self.pattern_hold != 0 {
            //DMG,SGB trigger while channel is being read corrupts wave RAM
            if (self.pattern_offset >> 1u32).x() <= 3 {
                //if current pattern is with 0-3; only byte 0 is corrupted
                let index = usize::from(u16::from(self.pattern_offset >> 1u32));
                self.pattern[0] = self.pattern[index];
            } else {
                //if current pattern is within 4-15; pattern&~3 is copied to pattern[0-3]
                let index = usize::from((self.pattern_offset >> 1u32).x() & !3);
                self.pattern[0] = self.pattern[index];
                self.pattern[1] = self.pattern[index + 1];
                self.pattern[2] = self.pattern[index + 2];
                self.pattern[3] = self.pattern[index + 3];
            }
        }

        self.enable = self.dac_enable;
        self.period = 2048 - u32::from(self.frequency) + 2;
        self.pattern_offset = U5::ZERO;
        self.pattern_sample = U4::ZERO;
        self.pattern_hold = 0;

        if self.length != 0 {
            self.length = 256;
            if apu_phase.get_bit(0) && self.counter {
                self.length -= 1;
            };
        }
    }

    //TODO test
    /// See cpp/ares/gb/apu/wave.cpp
    pub fn read_ram(&self, address: U4, data: u8, model_is_game_boy_color: bool) -> u8 {
        if self.enable {
            if !model_is_game_boy_color && self.pattern_hold == 0 {
                data
            } else {
                self.pattern[usize::from(u8::from(self.pattern_offset >> 1))]
            }
        } else {
            self.pattern[usize::from(u8::from(address))]
        }
    }

    //TODO test
    /// See cpp/ares/gb/apu/wave.cpp
    pub fn write_ram(&mut self, address: U4, data: u8, model_is_game_boy_color: bool) {
        if self.enable {
            if !model_is_game_boy_color && self.pattern_hold == 0 {
            } else {
                self.pattern[usize::from(u8::from(self.pattern_offset >> 1))] = data;
            }
        } else {
            self.pattern[usize::from(u8::from(address))] = data;
        }
    }

    /// See cpp/ares/gb/apu/wave.cpp
    pub fn power(&mut self, initialize_length: bool) {
        let old_length = self.length;
        let old_pattern = self.pattern;
        *self = Wave::default();
        if initialize_length {
            self.length = 256;
        } else {
            self.length = old_length;
        }
        self.pattern = old_pattern;
    }
}
