//TODO test

use higan::emulator::types::{Bits, U3};
use higan::gb::apu::apu::APU;
use higan::gb::apu::noise::Noise;
use higan::gb::apu::square_1::Square1;
use higan::gb::apu::square_2::Square2;
use higan::gb::apu::wave::Wave;
use higan::gb::system::system::System;
use malachite_base::misc::WrappingFrom;
use malachite_base::num::{BitAccess, One, WrappingAdd, WrappingAddAssign, Zero};

#[derive(Clone, Debug, Default)]
pub struct Channel {
    pub left_enable: bool,
    pub right_enable: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Sequencer {
    pub left_enable: bool,
    pub left_volume: U3,
    pub right_enable: bool,
    pub right_volume: U3,

    pub square1: Channel,
    pub square2: Channel,
    pub wave: Channel,
    pub noise: Channel,

    pub enable: bool,

    pub center: i16,
    pub left: i16,
    pub right: i16,
}

impl Sequencer {
    pub fn run(
        &mut self,
        apu_square1: &Square1,
        apu_square2: &Square2,
        apu_wave: &Wave,
        apu_noise: &Noise,
    ) {
        if !self.enable {
            self.center = 0;
            self.left = 0;
            self.right = 0;
            return;
        }

        let mut sample: i32 = 0;
        sample.wrapping_add_assign(i32::from(apu_square1.output));
        sample.wrapping_add_assign(i32::from(apu_square2.output));
        sample.wrapping_add_assign(i32::from(apu_wave.output));
        sample.wrapping_add_assign(i32::from(apu_noise.output));
        self.center = i16::wrapping_from(sample)
            .wrapping_mul(512)
            .wrapping_sub(16_384);

        sample = 0;
        if self.square1.left_enable {
            sample.wrapping_add_assign(i32::from(apu_square1.output));
        }
        if self.square2.left_enable {
            sample.wrapping_add_assign(i32::from(apu_square2.output));
        }
        if self.wave.left_enable {
            sample.wrapping_add_assign(i32::from(apu_wave.output));
        }
        if self.noise.left_enable {
            sample.wrapping_add_assign(i32::from(apu_noise.output));
        }
        sample = sample.wrapping_mul(512).wrapping_sub(16_384);
        sample = sample.wrapping_mul(i32::from(self.left_volume.wrapping_add(U3::ONE).0)) / 8;
        self.left = i16::wrapping_from(sample);

        sample = 0;
        if self.square1.right_enable {
            sample.wrapping_add_assign(i32::from(apu_square1.output));
        }
        if self.square2.right_enable {
            sample.wrapping_add_assign(i32::from(apu_square2.output));
        }
        if self.wave.right_enable {
            sample.wrapping_add_assign(i32::from(apu_wave.output));
        }
        if self.noise.right_enable {
            sample.wrapping_add_assign(i32::from(apu_noise.output));
        }
        sample = sample.wrapping_mul(512).wrapping_sub(16_384);
        sample = sample.wrapping_mul(i32::from(self.right_volume.wrapping_add(U3::ONE).0)) / 8;
        self.right = i16::wrapping_from(sample);

        //reduce audio volume
        self.center >>= 1;
        self.left >>= 1;
        self.right >>= 1;
    }

    pub fn read(&self, apu: &APU, addr: u16) -> u8 {
        match addr {
            //NR50
            0xff24 => {
                (if self.left_enable { 1 } else { 0 }) << 7
                    | self.left_volume.0 << 4
                    | (if self.right_enable { 1 } else { 0 }) << 3
                    | self.right_volume.0
            }
            //NR51
            0xff25 => {
                (if self.noise.left_enable { 1 } else { 0 }) << 7
                    | (if self.wave.left_enable { 1 } else { 0 }) << 6
                    | (if self.square2.left_enable { 1 } else { 0 }) << 5
                    | (if self.square1.left_enable { 1 } else { 0 }) << 4
                    | (if self.noise.right_enable { 1 } else { 0 }) << 3
                    | (if self.wave.right_enable { 1 } else { 0 }) << 2
                    | (if self.square2.right_enable { 1 } else { 0 }) << 1
                    | (if self.square1.right_enable { 1 } else { 0 }) << 0
            }
            //NR52
            0xff26 => {
                (if self.enable { 1 } else { 0 }) << 7
                    | 0x70
                    | (if apu.noise.enable { 1 } else { 0 }) << 3
                    | (if apu.wave.enable { 1 } else { 0 }) << 2
                    | (if apu.square2.enable { 1 } else { 0 }) << 1
                    | (if apu.square1.enable { 1 } else { 0 }) << 0
            }
            _ => 0xff,
        }
    }

    pub fn write(
        &mut self,
        system: &System,
        apu_phase: &mut U3,
        apu_square1: &mut Square1,
        apu_square2: &mut Square2,
        apu_wave: &mut Wave,
        apu_noise: &mut Noise,
        addr: u16,
        data: u8,
    ) {
        match addr {
            //NR50
            0xff24 => {
                self.left_enable = data.get_bit(7);
                self.left_volume = U3::wrapping_from(data.get_bits(6, 4));
                self.right_enable = data.get_bit(3);
                self.right_volume = U3::wrapping_from(data.get_bits(2, 0));
            }
            //NR51
            0xff25 => {
                self.noise.left_enable = data.get_bit(7);
                self.wave.left_enable = data.get_bit(6);
                self.square2.left_enable = data.get_bit(5);
                self.square1.left_enable = data.get_bit(4);
                self.noise.right_enable = data.get_bit(3);
                self.wave.right_enable = data.get_bit(2);
                self.square2.right_enable = data.get_bit(1);
                self.square1.right_enable = data.get_bit(0);
            }
            //NR52
            0xff26 => {
                if self.enable != data.get_bit(7) {
                    self.enable = data.get_bit(7);

                    if !self.enable {
                        //power(bool) resets length counters when true (eg for CGB only)
                        apu_square1.power(system.model_is_game_boy_color());
                        apu_square2.power(system.model_is_game_boy_color());
                        apu_wave.power(system.model_is_game_boy_color());
                        apu_noise.power(system.model_is_game_boy_color());
                        self.power();
                    } else {
                        *apu_phase = U3::ZERO;
                    }
                }
            }
            _ => {}
        }
    }

    pub fn power(&mut self) {
        *self = Sequencer::default();
    }
}
