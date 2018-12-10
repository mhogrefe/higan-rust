use higan::emulator::types::{Bits, U11, U2, U3, U4, U5};
use higan::gb::system::system::System;
use malachite_base::misc::WrappingFrom;
use malachite_base::num::{BitAccess, One, WrappingAddAssign, Zero};

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
    pub fn get_pattern(&self, offset: U5) -> U4 {
        U4::wrapping_from(
            self.pattern[(offset.0 >> 1) as usize] >> (if offset.get_bit(0) { 0 } else { 4 }),
        )
    }

    pub fn run(&mut self) {
        if self.pattern_hold != 0 {
            self.pattern_hold -= 1;
        }

        if self.period != 0 {
            self.period -= 1;
            if self.period == 0 {
                self.period = u32::from(2_048 - self.frequency.0);
                self.pattern_offset.wrapping_add_assign(U5::ONE);
                self.pattern_sample = self.get_pattern(self.pattern_offset);
                self.pattern_hold = 1;
            }
        }

        const SHIFT: [u32; 4] = [4, 0, 1, 2]; //0%, 100%, 50%, 25%
        let mut sample: U4 = self.pattern_sample >> SHIFT[self.volume.0 as usize];
        if !self.enable {
            sample = U4::ZERO;
        }
        self.output = i16::from(sample.0);
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

    pub fn read(&self, system: &System, addr: u16) -> u8 {
        match addr {
            //NR30
            0xff1a => (if self.dac_enable { 1 } else { 0 }) << 7 | 0x7f,
            //NR31
            0xff1b => 0xff,
            //NR32
            0xff1c => 0x80 | self.volume.0 << 5 | 0x1f,
            //NR33
            0xff1d => 0xff,
            //NR34
            0xff1e => 0x80 | (if self.counter { 1 } else { 0 }) << 6 | 0x3f,
            0xff30...0xff3f => {
                if self.enable {
                    if !system.model_is_game_boy_color() && self.pattern_hold == 0 {
                        0xff
                    } else {
                        self.pattern[(self.pattern_offset.0 >> 1) as usize]
                    }
                } else {
                    self.pattern[(addr & 15) as usize]
                }
            }

            _ => return 0xff,
        }
    }

    pub fn write(&mut self, system: &System, apu_phase: U3, addr: u16, data: u8) {
        match addr {
            //NR30
            0xff1a => {
                self.dac_enable = data.get_bit(7);
                if !self.dac_enable {
                    self.enable = false;
                }
            }
            //NR31
            0xff1b => {
                self.length = 256 - u32::from(data);
            }
            //NR32
            0xff1c => {
                self.volume = U2::wrapping_from(data.get_bits(6, 5));
            }
            //NR33
            0xff1d => {
                self.frequency.set_bits(U11::wrapping_from(data), 7, 0);
            }
            //NR34
            0xff1e => {
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
                    if !system.model_is_game_boy_color() && self.pattern_hold != 0 {
                        //DMG,SGB trigger while channel is being read corrupts wave RAM
                        if (self.pattern_offset.0 >> 1) <= 3 {
                            //if current pattern is with 0-3; only byte 0 is corrupted
                            self.pattern[0] = self.pattern[(self.pattern_offset.0 >> 1) as usize];
                        } else {
                            //if current pattern is within 4-15; pattern&~3 is copied to
                            // pattern[0-3]
                            self.pattern[0] =
                                self.pattern[((self.pattern_offset.0 >> 1) & !3) as usize + 0];
                            self.pattern[1] =
                                self.pattern[((self.pattern_offset.0 >> 1) & !3) as usize + 1];
                            self.pattern[2] =
                                self.pattern[((self.pattern_offset.0 >> 1) & !3) as usize + 2];
                            self.pattern[3] =
                                self.pattern[((self.pattern_offset.0 >> 1) & !3) as usize + 3];
                        }
                    }

                    self.enable = self.dac_enable;
                    self.period = u32::from(1 * (2_048 - self.frequency.0));
                    self.pattern_offset = U5::ZERO;
                    self.pattern_sample = U4::ZERO;
                    self.pattern_hold = 0;

                    if self.length == 0 {
                        self.length = 256;
                        if apu_phase.get_bit(0) && self.counter {
                            self.length -= 1;
                        }
                    }
                }
            }

            0xff30...0xff3f => {
                if self.enable {
                    if !system.model_is_game_boy_color() && self.pattern_hold == 0 {
                        return;
                    }
                    self.pattern[(self.pattern_offset.0 >> 1) as usize] = data;
                } else {
                    self.pattern[(addr & 15) as usize] = data;
                }
            }
            _ => {}
        }
    }

    pub fn power(&mut self, initialize_length: bool) {
        let old_length = self.length;
        let old_pattern = self.pattern.clone();
        *self = Wave::default();
        if initialize_length {
            self.length = 256;
        } else {
            self.length = old_length;
        }
        self.pattern = old_pattern;
    }
}
