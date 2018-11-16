use higan::emulator::types::{Bits, U11, U2, U3, U4};
use malachite_base::misc::WrappingFrom;
use malachite_base::num::{BitAccess, One, WrappingAddAssign, WrappingSubAssign, Zero};

#[derive(Debug, Default)]
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
    pub fn dac_enable(&self) -> bool {
        self.envelope_volume.0 != 0 || self.envelope_direction
    }

    pub fn run(&mut self) {
        if self.period != 0 {
            self.period -= 1;
            if self.period == 0 {
                self.period = 2 * (2_048 - u32::wrapping_from(self.frequency.0));
                self.phase.wrapping_add_assign(U3::ONE);
                self.duty_output = match self.duty.0 {
                    0 => self.phase.0 == 6, //______-_
                    1 => self.phase.0 >= 6, //______--
                    2 => self.phase.0 >= 4, //____----
                    3 => self.phase.0 <= 5, //------__
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
        self.output = i16::from(sample.0);
    }

    pub fn sweep(&mut self, update: bool) {
        if !self.sweep_enable {
            return;
        }
        self.sweep_negate = self.sweep_direction;
        let delta = u32::wrapping_from(self.frequency_shadow >> self.sweep_shift.0);
        let freq = self.frequency_shadow
            + if self.sweep_negate {
                -i32::wrapping_from(delta)
            } else {
                i32::wrapping_from(delta)
            };

        if freq > 2_047 {
            self.enable = false;
        } else if self.sweep_shift.0 != 0 && update {
            self.frequency_shadow = freq;
            self.frequency = U11::wrapping_from(freq & 2_047);
            self.period = 2 * (2_048 - u32::from(self.frequency.0));
        }
    }

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

    pub fn clock_sweep(&mut self) {
        self.sweep_period.wrapping_sub_assign(U3::ONE);
        if self.sweep_period.0 == 0 {
            self.sweep_period = if self.sweep_frequency.0 != 0 {
                self.sweep_frequency
            } else {
                U3::ZERO
            };
            if self.sweep_enable && self.sweep_frequency.0 != 0 {
                self.sweep(true);
                self.sweep(false);
            }
        }
    }

    pub fn clock_envelope(&mut self) {
        if self.enable && self.envelope_frequency.0 != 0 {
            self.envelope_period.wrapping_sub_assign(U3::ONE);
            if self.envelope_period.0 == 0 {
                self.envelope_period = self.envelope_frequency;
                if !self.envelope_direction && self.volume.0 > 0 {
                    self.volume -= U4::ONE;
                }
                if self.envelope_direction && self.volume.0 < 15 {
                    self.volume += U4::ONE;
                }
            }
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            //NR10
            0xff10 => {
                0x80 | self.sweep_frequency.0 << 4
                    | (if self.sweep_direction { 1 } else { 0 } << 3)
                    | self.sweep_shift.0
            }
            //NR11
            0xff11 => self.duty.0 << 6 | 0x3f,
            //NR12
            0xff12 => {
                self.envelope_volume.0 << 4
                    | (if self.envelope_direction { 1 } else { 0 } << 3)
                    | self.envelope_frequency.0
            }
            //NR13
            0xff13 => 0xff,
            //NR14
            0xff14 => 0x80 | (if self.counter { 1 } else { 0 } << 6) | 0x3f,
            _ => 0xff,
        }
    }

    pub fn write(&mut self, apu_phase: U3, addr: u16, data: u8) {
        match addr {
            //NR10
            0xff10 => {
                if self.sweep_enable && self.sweep_negate && !data.get_bit(3) {
                    self.enable = false;
                }
                self.sweep_frequency = U3::wrapping_from(data.get_bits(6, 4));
                self.sweep_direction = data.get_bit(3);
                self.sweep_shift = U3::wrapping_from(data.get_bits(2, 0));
            }
            //NR11
            0xff11 => {
                self.duty = U2::wrapping_from(data.get_bits(7, 6));
                self.length = 64 - u32::from(data.get_bits(5, 0));
            }
            //NR12
            0xff12 => {
                self.envelope_volume = U4::wrapping_from(data.get_bits(7, 4));
                self.envelope_direction = data.get_bit(3);
                self.envelope_frequency = U3::wrapping_from(data.get_bits(2, 0));
                if !self.dac_enable() {
                    self.enable = false;
                }
            }
            //NR13
            0xff13 => {
                self.frequency.set_bits(U11::wrapping_from(data), 7, 0);
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
                    .set_bits(U11::wrapping_from(data.get_bits(2, 0)), 10, 8);

                if data.get_bit(7) {
                    self.enable = self.dac_enable();
                    self.period = u32::from(2 * (2_048 - self.frequency.0));
                    self.envelope_period = self.envelope_frequency;
                    self.volume = self.envelope_volume;

                    if self.length == 0 {
                        self.length = 64;
                        if apu_phase.get_bit(0) && self.counter {
                            self.length -= 1;
                        }
                    }

                    self.frequency_shadow = i32::wrapping_from(self.frequency.0);
                    self.sweep_negate = false;
                    self.sweep_period = self.sweep_frequency;
                    self.sweep_enable = self.sweep_period.0 != 0 || self.sweep_shift.0 != 0;
                    if self.sweep_shift.0 != 0 {
                        self.sweep(false);
                    }
                }
            }
            _ => {}
        }
    }

    pub fn power(&mut self, initialize_length: bool) {
        *self = Square1::default();
        if initialize_length {
            self.length = 64;
        }
    }
}
