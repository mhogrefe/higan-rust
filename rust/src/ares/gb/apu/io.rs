use ares::emulator::types::{U11, U2, U3, U4};
use ares::gb::apu::APU;
use ares::gb::system::Model;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::{BitAccess, BitBlockAccess};

impl APU {
    /// See higan-rust/cpp/ares/gb/apu/io.cpp
    pub fn read_io(&self, cycle: u32, address: u16, mut data: u8) -> u8 {
        match (address, cycle) {
            (address, _) if !(0xff10..=0xff3f).contains(&address) => {}

            //NR10
            (0xff10, 2) => {
                data.assign_bits(0, 3, &self.square_1.sweep_shift.x());
                data.assign_bit(3, self.square_1.sweep_direction);
                data.assign_bits(4, 7, &self.square_1.sweep_frequency.x());
            }

            //NR11
            (0xff11, 2) => {
                data.assign_bits(6, 8, &self.square_1.duty.x());
            }

            //NR12
            (0xff12, 2) => {
                data.assign_bits(0, 3, &self.square_1.envelope_frequency.x());
                data.assign_bit(3, self.square_1.envelope_direction);
                data.assign_bits(4, 8, &self.square_1.envelope_volume.x());
            }

            //NR13
            (0xff13, 2) => {}

            //NR14
            (0xff14, 2) => {
                data.assign_bit(6, self.square_1.counter);
            }

            //NR20
            (0xff15, 2) => {}

            //NR21
            (0xff16, 2) => {
                data.assign_bits(6, 8, &self.square_2.duty.x());
            }

            //NR22
            (0xff17, 2) => {
                data.assign_bits(0, 3, &self.square_2.envelope_frequency.x());
                data.assign_bit(3, self.square_2.envelope_direction);
                data.assign_bits(4, 8, &self.square_2.envelope_volume.x());
            }

            //NR23
            (0xff18, 2) => {}

            //NR24
            (0xff19, 2) => {
                data.assign_bit(6, self.square_2.counter);
            }

            //NR30
            (0xff1a, 2) => {
                data.assign_bit(7, self.wave.dac_enable);
            }

            //NR31
            (0xff1b, 2) => {}

            //NR32
            (0xff1c, 2) => {
                data.assign_bits(5, 7, &self.wave.volume.x());
            }

            //NR33
            (0xff1d, 2) => {}

            //NR34
            (0xff1e, 2) => {
                data.assign_bit(6, self.wave.counter);
            }

            //NR40
            (0xff1f, 2) => {}

            //NR41
            (0xff20, 2) => {}

            //NR42
            (0xff21, 2) => {
                data.assign_bits(0, 3, &self.noise.envelope_frequency.x());
                data.assign_bit(3, self.noise.envelope_direction);
                data.assign_bits(4, 8, &self.noise.envelope_volume.x());
            }

            //NR43
            (0xff22, 2) => {
                data.assign_bits(0, 3, &self.noise.divisor.x());
                data.assign_bit(3, self.noise.narrow);
                data.assign_bits(4, 8, &self.noise.frequency.x());
            }

            //NR44
            (0xff23, 2) => {
                data.assign_bit(6, self.noise.counter);
            }

            //NR50
            (0xff24, 2) => {
                data.assign_bits(0, 3, &self.sequencer.right_volume.x());
                data.assign_bit(3, self.sequencer.right_enable);
                data.assign_bits(4, 7, &self.sequencer.left_volume.x());
                data.assign_bit(7, self.sequencer.left_enable);
            }

            //NR51
            (0xff25, 2) => {
                data.assign_bit(0, self.sequencer.square_1.right_enable);
                data.assign_bit(1, self.sequencer.square_2.right_enable);
                data.assign_bit(2, self.sequencer.wave.right_enable);
                data.assign_bit(3, self.sequencer.noise.right_enable);
                data.assign_bit(4, self.sequencer.square_1.left_enable);
                data.assign_bit(5, self.sequencer.square_2.left_enable);
                data.assign_bit(6, self.sequencer.wave.left_enable);
                data.assign_bit(7, self.sequencer.noise.left_enable);
            }

            //NR52
            (0xff26, _) => {
                data.assign_bit(0, self.square_1.enable);
                data.assign_bit(1, self.square_2.enable);
                data.assign_bit(2, self.wave.enable);
                data.assign_bit(3, self.noise.enable);
                data.assign_bit(7, self.sequencer.enable);
            }

            (address, 2) if (0xff30..=0xff3f).contains(&address) => {
                return self.wave.read_ram(
                    U4::wrapping_from(address),
                    data,
                    self.model == Model::GameBoyColor,
                );
            }
            _ => {}
        }
        data
    }

    pub fn write_io(&mut self, cycle: u32, address: u16, mut data: u8) {
        if !(0xff10..=0xff3f).contains(&address) {
            return;
        }
        if !self.sequencer.enable {
            let mut valid = address == 0xff26; //NR52
            if self.model != Model::GameBoyColor {
                //NRx1 length is writable only on DMG,SGB; not on CGB
                if address == 0xff11 {
                    //NR11; duty is not writable (remains 0)
                    valid = true;
                    data &= 0x3f;
                }
                if address == 0xff16 {
                    //NR21; duty is not writable (remains 0)
                    valid = true;
                    data &= 0x3f;
                }
                if address == 0xff1b {
                    //NR31valid = true;
                }
                if address == 0xff20 {
                    //NR41
                    valid = true;
                }
            }
            if !valid {
                return;
            };
        }
        match (address, cycle) {
            //NR10
            (0xff10, 2) => {
                if self.square_1.sweep_enable && self.square_1.sweep_negate && !data.get_bit(3) {
                    self.square_1.enable = false
                };
                self.square_1.sweep_shift = U3::wrapping_from(data.get_bits(0, 3));
                self.square_1.sweep_direction = data.get_bit(3);
                self.square_1.sweep_frequency = U3::wrapping_from(data.get_bits(4, 7));
            }

            //NR11
            (0xff11, 2) => {
                self.square_1.length = 64 - u32::from(data.get_bits(0, 6));
                self.square_1.duty = U2::wrapping_from(data.get_bits(6, 8));
            }

            //NR12
            (0xff12, 2) => {
                self.square_1.envelope_frequency = U3::wrapping_from(data.get_bits(0, 3));
                self.square_1.envelope_direction = data.get_bit(3);
                self.square_1.envelope_volume = U4::wrapping_from(data.get_bits(4, 8));
                if !self.square_1.dac_enable() {
                    self.square_1.enable = false
                };
            }

            //NR13
            (0xff13, 2) => {
                self.square_1
                    .frequency
                    .assign_bits(0, 8, &U11::wrapping_from(data));
            }

            //NR14
            (0xff14, 4) => {
                if self.phase.get_bit(0)
                    && !self.square_1.counter
                    && data.get_bit(6)
                    && self.square_1.length != 0
                {
                    self.square_1.length -= 1;
                    if self.square_1.length == 0 {
                        self.square_1.enable = false;
                    }
                }
                self.square_1.frequency.assign_bits(
                    8,
                    11,
                    &U11::wrapping_from(data.get_bits(0, 3)),
                );
                self.square_1.counter = data.get_bit(6);
                if data.get_bit(7) {
                    self.square_1.trigger(self.phase)
                };
            }

            //NR20
            (0xff15, 2) => {}

            //NR21
            (0xff16, 2) => {
                self.square_2.length = 64 - u32::from(data.get_bits(0, 6));
                self.square_2.duty = U2::wrapping_from(data.get_bits(6, 8));
            }

            //NR22
            (0xff17, 2) => {
                self.square_2.envelope_frequency = U3::wrapping_from(data.get_bits(0, 3));
                self.square_2.envelope_direction = data.get_bit(3);
                self.square_2.envelope_volume = U4::wrapping_from(data.get_bits(4, 8));
                if !self.square_2.dac_enable() {
                    self.square_2.enable = false
                };
            }

            //NR23
            (0xff18, 2) => {
                self.square_2
                    .frequency
                    .assign_bits(0, 8, &U11::wrapping_from(data));
            }

            //NR24
            (0xff19, 4) => {
                if self.phase.get_bit(0)
                    && !self.square_2.counter
                    && data.get_bit(6)
                    && self.square_2.length != 0
                {
                    self.square_2.length -= 1;
                    if self.square_2.length == 0 {
                        self.square_2.enable = false
                    };
                }
                self.square_2.frequency.assign_bits(
                    8,
                    11,
                    &U11::wrapping_from(data.get_bits(0, 3)),
                );
                self.square_2.counter = data.get_bit(6);
                if data.get_bit(7) {
                    self.square_2.trigger(self.phase)
                };
            }

            //NR30
            (0xff1a, 2) => {
                self.wave.dac_enable = data.get_bit(7);
                if !self.wave.dac_enable {
                    self.wave.enable = false;
                }
            }

            //NR31
            (0xff1b, 2) => {
                self.wave.length = 256 - u32::from(data);
            }

            //NR32
            (0xff1c, 2) => {
                self.wave.volume = U2::wrapping_from(data.get_bits(5, 7));
            }

            //NR33
            (0xff1d, 2) => {
                self.wave
                    .frequency
                    .assign_bits(0, 8, &U11::wrapping_from(data));
            }

            //NR34
            (0xff1e, 4) => {
                if self.phase.get_bit(0)
                    && !self.wave.counter
                    && data.get_bit(6)
                    && self.wave.length != 0
                {
                    self.wave.length -= 1;
                    if self.wave.length == 0 {
                        self.wave.enable = false
                    };
                }
                self.wave
                    .frequency
                    .assign_bits(8, 11, &U11::wrapping_from(data.get_bits(0, 3)));
                self.wave.counter = data.get_bit(6);
                if data.get_bit(7) {
                    self.wave
                        .trigger(self.model == Model::GameBoyColor, self.phase)
                };
            }

            //NR40
            (0xff1f, 2) => {}

            //NR41
            (0xff20, 2) => {
                self.noise.length = 64 - u32::from(data.get_bits(0, 6));
            }

            //NR42
            (0xff21, 2) => {
                self.noise.envelope_frequency = U3::wrapping_from(data.get_bits(0, 3));
                self.noise.envelope_direction = data.get_bit(3);
                self.noise.envelope_volume = U4::wrapping_from(data.get_bits(4, 8));
                if !self.noise.dac_enable() {
                    self.noise.enable = false
                };
            }

            //NR43
            (0xff22, 2) => {
                self.noise.divisor = U3::wrapping_from(data.get_bits(0, 3));
                self.noise.narrow = data.get_bit(3);
                self.noise.frequency = U4::wrapping_from(data.get_bits(4, 8));
                self.noise.period = self.noise.get_period();
            }

            //NR44
            (0xff23, 4) => {
                if self.phase.get_bit(0)
                    && !self.noise.counter
                    && data.get_bit(6)
                    && self.noise.length != 0
                {
                    self.noise.length -= 1;
                    if self.noise.length == 0 {
                        self.noise.enable = false
                    };
                }
                self.noise.counter = data.get_bit(6);
                if data.get_bit(7) {
                    self.noise.trigger(self.phase)
                };
            }

            //NR50
            (0xff24, 2) => {
                self.sequencer.right_volume = U3::wrapping_from(data.get_bits(0, 3));
                self.sequencer.right_enable = data.get_bit(3);
                self.sequencer.left_volume = U3::wrapping_from(data.get_bits(4, 7));
                self.sequencer.left_enable = data.get_bit(7);
            }

            //NR51
            (0xff25, 2) => {
                self.sequencer.square_1.right_enable = data.get_bit(0);
                self.sequencer.square_2.right_enable = data.get_bit(1);
                self.sequencer.wave.right_enable = data.get_bit(2);
                self.sequencer.noise.right_enable = data.get_bit(3);
                self.sequencer.square_1.left_enable = data.get_bit(4);
                self.sequencer.square_2.left_enable = data.get_bit(5);
                self.sequencer.wave.left_enable = data.get_bit(6);
                self.sequencer.noise.left_enable = data.get_bit(7);
            }

            //NR52
            (0xff26, 4) => {
                if self.sequencer.enable != data.get_bit(7) {
                    self.sequencer.enable = data.get_bit(7);
                    if !self.sequencer.enable {
                        let reset_length_counters = self.model == Model::GameBoyColor;
                        self.square_1.power(reset_length_counters);
                        self.square_2.power(reset_length_counters);
                        self.wave.power(reset_length_counters);
                        self.noise.power(reset_length_counters);
                        self.sequencer.power();
                    } else {
                        self.phase = U3::ZERO;
                    }
                }
            }

            (address, 2) if (0xff30..=0xff3f).contains(&address) => {
                self.wave.write_ram(
                    U4::wrapping_from(address),
                    data,
                    self.model == Model::GameBoyColor,
                );
            }
            _ => {}
        }
    }
}
