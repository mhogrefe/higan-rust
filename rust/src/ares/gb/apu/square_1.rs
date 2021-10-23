use ares::emulator::types::{U11, U2, U3, U4};
use malachite_base::num::arithmetic::traits::{
    NegAssign, SaturatingAddAssign, SaturatingSubAssign, WrappingAddAssign, WrappingSubAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::BitAccess;

/// See higan-rust/cpp/ares/gb/apu/apu.hpp
#[derive(Clone, Debug, Default)]
pub struct Square1 {
    pub enable: bool,

    pub sweep_frequency: U3,
    pub sweep_direction: bool,
    pub sweep_shift: U3,
    pub sweep_negate: bool,
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
    pub sweep_period: U3,
    pub frequency_shadow: i32,
    pub sweep_enable: bool,
    pub volume: U4,
}

impl Square1 {
    /// See cpp/ares/gb/apu/square1.cpp
    pub fn dac_enable(&self) -> bool {
        self.envelope_volume.x() != 0 || self.envelope_direction
    }

    /// See cpp/ares/gb/apu/square1.cpp
    pub fn run(&mut self) {
        if self.period != 0 {
            self.period -= 1;
            if self.period == 0 {
                self.period = (2_048 - u32::wrapping_from(self.frequency)) << 1;
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
        let mut sample = if self.duty_output {
            self.volume
        } else {
            U4::ZERO
        };
        if !self.enable {
            sample = U4::ZERO;
        }
        self.output = i16::from(sample);
    }

    /// See cpp/ares/gb/apu/square1.cpp
    pub fn sweep(&mut self, update: bool) {
        if !self.sweep_enable {
            return;
        }
        self.sweep_negate = self.sweep_direction;
        let mut delta = i32::wrapping_from(self.frequency_shadow >> self.sweep_shift.x());
        if self.sweep_negate {
            delta.neg_assign();
        }
        let freq = self.frequency_shadow + delta;
        if freq > 2_047 {
            self.enable = false;
        } else if self.sweep_shift.x() != 0 && update {
            self.frequency_shadow = freq;
            self.frequency = U11::wrapping_from(freq);
            self.period = (2_048 - u32::from(self.frequency)) << 1;
        }
    }

    /// See cpp/ares/gb/apu/square1.cpp
    pub fn clock_length(&mut self) {
        if self.counter && self.length != 0 {
            self.length -= 1;
            if self.length == 0 {
                self.enable = false;
            }
        }
    }

    /// See cpp/ares/gb/apu/square1.cpp
    pub fn clock_sweep(&mut self) {
        self.sweep_period.wrapping_sub_assign(U3::ONE);
        if self.sweep_period.x() == 0 {
            self.sweep_period = self.sweep_frequency;
            if self.sweep_enable && self.sweep_frequency.x() != 0 {
                self.sweep(true);
                self.sweep(false);
            }
        }
    }

    /// See cpp/ares/gb/apu/square1.cpp
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
    /// See cpp/ares/gb/apu/square1.cpp
    pub fn trigger(&mut self, apu_phase: U3) {
        self.enable = self.dac_enable();
        self.period = 2 * (2048 - u32::from(self.frequency));
        self.envelope_period = self.envelope_frequency;
        self.volume = self.envelope_volume;

        if self.length != 0 {
            self.length = 64;
            if apu_phase.get_bit(0) && self.counter {
                self.length -= 1;
            }
        }

        self.frequency_shadow = i32::from(self.frequency);
        self.sweep_negate = false;
        self.sweep_period = self.sweep_frequency;
        self.sweep_enable = self.sweep_period.x() != 0 || self.sweep_shift.x() != 0;
        if self.sweep_shift.x() != 0 {
            self.sweep(false)
        };
    }

    /*
    pub fn write(&mut self, apu_phase: U3, addr: u16, data: u8) {
        match addr {
            //NR10
            0xff10 => {
                if self.sweep_enable && self.sweep_negate && !data.get_bit(3) {
                    self.enable = false;
                }
                self.sweep_frequency = U3::wrapping_from(data.get_bits(4, 7));
                self.sweep_direction = data.get_bit(3);
                self.sweep_shift = U3::wrapping_from(data.get_bits(0, 3));
            }
            //NR11
            0xff11 => {
                self.duty = U2::wrapping_from(data.get_bits(6, 8));
                self.length = 64 - u32::from(data.get_bits(0, 6));
            }
            //NR12
            0xff12 => {
                self.envelope_volume = U4::wrapping_from(data.get_bits(4, 8));
                self.envelope_direction = data.get_bit(3);
                self.envelope_frequency = U3::wrapping_from(data.get_bits(0, 3));
                if !self.dac_enable() {
                    self.enable = false;
                }
            }
            //NR13
            0xff13 => {
                self.frequency.assign_bits(0, 8, &U11::wrapping_from(data));
            }
            //NR14
            0xff14 => {
                if apu_phase.get_bit(0) && !self.counter && data.get_bit(6) {
                    if self.length != 0 {
                        self.length -= 1;
                        if self.length == 0 {
                            self.enable = false;
                        }
                    }
                }

                self.counter = data.get_bit(6);
                self.frequency
                    .assign_bits(8, 11, &U11::wrapping_from(data.get_bits(0, 3)));

                if data.get_bit(7) {
                    self.enable = self.dac_enable();
                    self.period = u32::from(2 * (2_048 - self.frequency.x()));
                    self.envelope_period = self.envelope_frequency;
                    self.volume = self.envelope_volume;

                    if self.length == 0 {
                        self.length = 64;
                        if apu_phase.get_bit(0) && self.counter {
                            self.length -= 1;
                        }
                    }

                    self.frequency_shadow = i32::wrapping_from(self.frequency.x());
                    self.sweep_negate = false;
                    self.sweep_period = self.sweep_frequency;
                    self.sweep_enable = self.sweep_period.x() != 0 || self.sweep_shift.x() != 0;
                    if self.sweep_shift.x() != 0 {
                        self.sweep(false);
                    }
                }
            }
            _ => {}
        }
    }*/

    /// See cpp/ares/gb/apu/square1.cpp
    pub fn power(&mut self, initialize_length: bool) {
        let old_length = self.length;
        *self = Square1::default();
        if initialize_length {
            self.length = 64;
        } else {
            self.length = old_length;
        }
    }
}
