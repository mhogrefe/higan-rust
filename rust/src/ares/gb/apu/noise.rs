use ares::emulator::types::{U15, U3, U4};
use malachite_base::num::arithmetic::traits::WrappingSubAssign;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::BitAccess;

/// See higan-rust/cpp/ares/gb/apu/apu.hpp
#[derive(Clone, Debug, Default)]
pub struct Noise {
    pub enable: bool,

    pub envelope_volume: U4,
    pub envelope_direction: bool,
    pub envelope_frequency: U3,
    pub frequency: U4,
    pub narrow: bool,
    pub divisor: U3,
    pub counter: bool,

    pub output: i16,
    pub length: u32,
    pub envelope_period: U3,
    pub volume: U4,
    pub period: u32,
    pub lfsr: U15,
}

impl Noise {
    /// See higan-rust/cpp/ares/gb/apu/noise.cpp
    pub fn dac_enable(&self) -> bool {
        self.envelope_volume.x() != 0 || self.envelope_direction
    }

    /// See higan-rust/cpp/ares/gb/apu/noise.cpp
    pub fn get_period(&self) -> u32 {
        const TABLE: [u32; 8] = [4, 8, 16, 24, 32, 40, 48, 56];
        TABLE[self.divisor.x() as usize] << self.frequency.x()
    }

    /// See higan-rust/cpp/ares/gb/apu/noise.cpp
    pub fn run(&mut self) {
        if self.period != 0 {
            self.period -= 1;
            if self.period == 0 {
                self.period = self.get_period();
                if self.frequency.x() < 14 {
                    let bit = (self.lfsr ^ (self.lfsr >> 1)) & U15::ONE;
                    self.lfsr = (self.lfsr >> 1) ^ (bit << (if self.narrow { 6 } else { 14 }));
                }
            }
        }

        let mut sample: U4 = if self.lfsr.get_bit(0) {
            U4::ZERO
        } else {
            self.volume
        };
        if !self.enable {
            sample = U4::ZERO;
        }
        self.output = i16::from(sample.x());
    }

    /// See higan-rust/cpp/ares/gb/apu/noise.cpp
    pub fn clock_length(&mut self) {
        if self.counter {
            if self.length != 0 {
                self.length -= 1;
                if self.length == 0 {
                    self.enable = false;
                }
            }
        }
    }

    /// See higan-rust/cpp/ares/gb/apu/noise.cpp
    pub fn clock_envelope(&mut self) {
        if self.enable && self.envelope_frequency.x() != 0 {
            self.envelope_period.wrapping_sub_assign(U3::ONE);
            if self.envelope_period.x() == 0 {
                self.envelope_period = self.envelope_frequency;
                if !self.envelope_direction && self.volume.x() > 0 {
                    self.volume -= U4::ONE;
                }
                if self.envelope_direction && self.volume.x() < 15 {
                    self.volume += U4::ONE;
                }
            }
        }
    }

    /// See higan-rust/cpp/ares/gb/apu/noise.cpp
    pub fn trigger(&mut self, apu_phase: U3) {
        self.enable = self.dac_enable();
        self.lfsr = U15::wrapping_from(-1);
        self.envelope_period = self.envelope_frequency;
        self.volume = self.envelope_volume;
        if self.length == 0 {
            self.length = 64;
            if apu_phase.get_bit(0) && self.counter {
                self.length -= 1;
            };
        }
    }

    /*
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            //NR40
            0xff1f => 0xff,

            //NR41
            0xff20 => 0xff,
            //NR42
            0xff21 => {
                self.envelope_volume.x() << 4
                    | if self.envelope_direction { 1 } else { 0 } << 3
                    | self.envelope_frequency.x()
            }
            //NR43
            0xff22 => {
                self.frequency.x() << 4 | if self.narrow { 1 } else { 0 } << 3 | self.divisor.x()
            }
            //NR44
            0xff23 => 0x80 | if self.counter { 1 } else { 0 } << 6 | 0x3f,
            _ => 0xff,
        }
    }

    pub fn write(&mut self, apu_phase: U3, addr: u16, data: u8) {
        match addr {
            //NR41
            0xff20 => {
                self.length = u32::from(64 - (data & 0x3f));
            }
            //NR42
            0xff21 => {
                self.envelope_volume = U4::wrapping_from(data.get_bits(4, 8));
                self.envelope_direction = data.get_bit(3);
                self.envelope_frequency = U3::wrapping_from(data.get_bits(0, 3));
                if !self.dac_enable() {
                    self.enable = false;
                }
            }
            //NR43
            0xff22 => {
                self.frequency = U4::wrapping_from(data.get_bits(4, 8));
                self.narrow = data.get_bit(3);
                self.divisor = U3::wrapping_from(data.get_bits(0, 3));
                self.period = self.get_period();
            }
            //NR44
            0xff23 => {
                if apu_phase.get_bit(0) && !self.counter && (data & 0x40) != 0 {
                    if self.length != 0 {
                        self.length -= 1;
                        if self.length == 0 {
                            self.enable = false;
                        }
                    }
                }

                self.counter = data.get_bit(6);

                if data.get_bit(7) {
                    self.enable = self.dac_enable();
                    self.lfsr = U15::MAX;
                    self.envelope_period = self.envelope_frequency;
                    self.volume = self.envelope_volume;

                    if self.length == 0 {
                        self.length = 64;
                        if apu_phase.get_bit(0) && self.counter {
                            self.length -= 1;
                        }
                    }
                }
            }
            _ => {}
        }
    }*/

    /// See higan-rust/cpp/ares/gb/apu/noise.cpp
    pub fn power(&mut self, initialize_length: bool) {
        let old_length = self.length;
        *self = Noise::default();
        if initialize_length {
            self.length = 64;
        } else {
            self.length = old_length;
        }
    }
}
