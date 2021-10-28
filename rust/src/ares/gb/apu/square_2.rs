use ares::emulator::types::{U11, U2, U3, U4};
use malachite_base::num::arithmetic::traits::{
    Parity, SaturatingAddAssign, SaturatingSubAssign, WrappingAddAssign, WrappingSubAssign,
};
use malachite_base::num::basic::traits::One;

/// See higan-rust/cpp/ares/gb/apu/apu.hpp
#[derive(Clone, Debug, Default)]
pub struct Square2 {
    pub enable: bool,

    pub duty: U2,
    pub length: u32,
    pub envelope_volume: U4,
    pub envelope_direction: bool,
    pub envelope_frequency: U3,
    pub frequency: U11,
    pub counter: bool,

    pub output: i16,
    pub duty_output: bool,
    pub phase: U3,
    pub period: u32,
    pub envelope_period: U3,
    pub volume: U4,
}

impl Square2 {
    /// See cpp/ares/gb/apu/square2.cpp
    pub fn dac_enable(&self) -> bool {
        self.envelope_volume.x() != 0 || self.envelope_direction
    }

    /// See cpp/ares/gb/apu/square2.cpp
    pub fn run(&mut self) {
        if self.period != 0 {
            self.period -= 1;
            if self.period == 0 {
                self.period = (2048 - u32::from(self.frequency)) << 1;
                self.phase.wrapping_add_assign(U3::ONE);
                let x = self.phase.x();
                self.duty_output = match self.duty.x() {
                    0 => x == 6, //______-_
                    1 => x >= 6, //______--
                    2 => x >= 4, //____----
                    3 => x <= 5, //------__
                    _ => unreachable!(),
                };
            }
        }

        self.output = if self.enable && self.duty_output {
            i16::from(self.volume)
        } else {
            0
        };
    }

    /// See cpp/ares/gb/apu/square2.cpp
    pub fn clock_length(&mut self) {
        if self.counter && self.length != 0 {
            self.length -= 1;
            if self.length == 0 {
                self.enable = false;
            }
        }
    }

    /// See cpp/ares/gb/apu/square2.cpp
    pub fn clock_envelope(&mut self) {
        if self.enable && self.envelope_frequency.x() != 0 {
            self.envelope_period.wrapping_sub_assign(U3::ONE);
            if self.envelope_period.x() == 0 {
                self.envelope_period = self.envelope_frequency;
                if self.envelope_direction {
                    self.volume.saturating_add_assign(U4::ONE);
                } else {
                    self.volume.saturating_sub_assign(U4::ONE);
                }
            }
        }
    }

    //TODO test
    /// See cpp/ares/gb/apu/square2.cpp
    pub fn trigger(&mut self, apu_phase: U3) {
        self.enable = self.dac_enable();
        self.period = (2048 - u32::from(self.frequency)) << 1;
        self.envelope_period = self.envelope_frequency;
        self.volume = self.envelope_volume;
        if self.length == 0 {
            self.length = 64;
            if apu_phase.odd() && self.counter {
                self.length -= 1;
            };
        }
    }

    /// See cpp/ares/gb/apu/square2.cpp
    pub fn power(&mut self, initialize_length: bool) {
        let old_length = self.length;
        *self = Square2::default();
        if initialize_length {
            self.length = 64;
        } else {
            self.length = old_length;
        }
    }
}
