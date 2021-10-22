//TODO test

use ares::emulator::types::{U12, U3};
use ares::gb::apu::noise::Noise;
use ares::gb::apu::sequencer::Sequencer;
use ares::gb::apu::square_1::Square1;
use ares::gb::apu::square_2::Square2;
use ares::gb::apu::wave::Wave;
use ares::gb::memory::memory::Bus;
use malachite_base::num::arithmetic::traits::{WrappingAdd, WrappingAddAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::WrappingFrom;
use nall::random::{Pcg, Rng};

//TODO impl Thread
//TODO auto APU::Enter() -> void

/// See higan-rust/cpp/ares/gb/apu/apu.hpp
#[derive(Clone, Debug, Default)]
pub struct APU {
    pub model_is_game_boy_color: bool,
    pub square_1: Square1,
    pub square_2: Square2,
    pub noise: Noise,
    pub wave: Wave,
    pub sequencer: Sequencer,

    pub phase: U3,  //high 3-bits of clock counter
    pub cycle: U12, //low 12-bits of clock counter
}

impl APU {
    /// See higan-rust/cpp/ares/gb/apu/apu.cpp
    pub fn main(&mut self) {
        self.square_1.run();
        self.square_2.run();
        self.wave.run();
        self.noise.run();
        self.run_sequencer();
        //TODO stream->frame(sequencer.left / 32768.0, sequencer.right / 32768.0);

        if self.cycle.x() == 0 {
            //512hz
            if self.phase.x() == 0
                || self.phase.x() == 2
                || self.phase.x() == 4
                || self.phase.x() == 6
            {
                //256hz
                self.square_1.clock_length();
                self.square_2.clock_length();
                self.wave.clock_length();
                self.noise.clock_length();
            }
            if self.phase.x() == 2 || self.phase.x() == 6 {
                //128hz
                self.square_1.clock_sweep();
            }
            if self.phase.x() == 7 {
                //64hz
                self.square_1.clock_envelope();
                self.square_2.clock_envelope();
                self.noise.clock_envelope();
            }
            self.phase.wrapping_add_assign(U3::ONE);
        }
        self.cycle.wrapping_add_assign(U12::ONE);

        //TODO Thread::step(1);
        //TODO synchronize(cpu);
    }

    /// See higan-rust/cpp/ares/gb/apu/sequencer.cpp
    pub fn run_sequencer(&mut self) {
        if !self.sequencer.enable {
            self.sequencer.center = 0;
            self.sequencer.left = 0;
            self.sequencer.right = 0;
            return;
        }

        let mut sample: i32 = 0;
        sample.wrapping_add_assign(i32::from(self.square_1.output));
        sample.wrapping_add_assign(i32::from(self.square_2.output));
        sample.wrapping_add_assign(i32::from(self.wave.output));
        sample.wrapping_add_assign(i32::from(self.noise.output));
        self.sequencer.center = i16::wrapping_from(sample)
            .wrapping_mul(512)
            .wrapping_sub(16_384);

        sample = 0;
        if self.sequencer.square_1.left_enable {
            sample.wrapping_add_assign(i32::from(self.square_1.output));
        }
        if self.sequencer.square_2.left_enable {
            sample.wrapping_add_assign(i32::from(self.square_2.output));
        }
        if self.sequencer.wave.left_enable {
            sample.wrapping_add_assign(i32::from(self.wave.output));
        }
        if self.sequencer.noise.left_enable {
            sample.wrapping_add_assign(i32::from(self.noise.output));
        }
        sample = sample.wrapping_mul(512).wrapping_sub(16_384);
        sample = sample.wrapping_mul(i32::from(
            self.sequencer.left_volume.wrapping_add(U3::ONE).x(),
        )) / 8;
        self.sequencer.left = i16::wrapping_from(sample);

        sample = 0;
        if self.sequencer.square_1.right_enable {
            sample.wrapping_add_assign(i32::from(self.square_1.output));
        }
        if self.sequencer.square_2.right_enable {
            sample.wrapping_add_assign(i32::from(self.square_2.output));
        }
        if self.sequencer.wave.right_enable {
            sample.wrapping_add_assign(i32::from(self.wave.output));
        }
        if self.sequencer.noise.right_enable {
            sample.wrapping_add_assign(i32::from(self.noise.output));
        }
        sample = sample.wrapping_mul(512).wrapping_sub(16_384);
        sample = sample.wrapping_mul(i32::from(
            self.sequencer.right_volume.wrapping_add(U3::ONE).x(),
        )) / 8;
        self.sequencer.right = i16::wrapping_from(sample);

        //reduce audio volume
        self.sequencer.center >>= 1;
        self.sequencer.left >>= 1;
        self.sequencer.right >>= 1;
    }
}

impl Bus {
    pub fn power_apu(&mut self) {
        //TODO Thread::create(2 * 1024 * 1024, {&APU::main, this});

        self.apu.square_1.power(false);
        self.apu.square_2.power(false);
        self.apu.wave.power(false);
        self.apu.noise.power(false);
        self.apu.sequencer.power();
        self.apu.phase = U3::ZERO;
        self.apu.cycle = U12::ZERO;

        let mut prng = Pcg::new();
        for n in self.apu.wave.pattern.iter_mut() {
            *n = prng.random() as u8;
        }
    }
}

/*
impl MMIO for APU {
    fn read_io(&self, addr: u16) -> u8 {
        match addr {
            0xff10..=0xff14 => self.square_1.read(addr),
            0xff15..=0xff19 => self.square_2.read(addr),
            0xff1a..=0xff1e => self.wave.read(self.model_is_game_boy_color, addr),
            0xff1f..=0xff23 => self.noise.read(addr),
            0xff24..=0xff26 => self.sequencer.read(addr),
            0xff30..=0xff3f => self.wave.read(self.model_is_game_boy_color, addr),
            _ => 0xff,
        }
    }

    fn write_io(&mut self, addr: u16, mut data: u8) {
        if !self.sequencer.enable {
            let mut valid = addr == 0xff26; //NR52
            if !self.model_is_game_boy_color {
                //NRx1 length is writable only on DMG,SGB; not on CGB
                //NR11; duty is not writable (remains 0)
                match addr {
                    0xff11 => {
                        valid = true;
                        data &= 0x3f;
                    }
                    //NR21; duty is not writable (remains 0)
                    0xff16 => {
                        valid = true;
                        data &= 0x3f;
                    }
                    0xff1b => {
                        valid = true; //NR31
                    }
                    0xff20 => {
                        valid = true; //NR41
                    }
                    _ => {}
                }
            }
            if !valid {
                return;
            }
        }
        match addr {
            0xff10..=0xff14 => self.square_1.write(self.phase, addr, data),
            0xff15..=0xff19 => self.square_2.write(self.phase, addr, data),
            0xff1a..=0xff1e => {
                self.wave
                    .write(self.model_is_game_boy_color, self.phase, addr, data)
            }
            0xff1f..=0xff23 => self.noise.write(self.phase, addr, data),
            0xff24..=0xff26 => {
                self.sequencer
                    .write(self.model_is_game_boy_color, &mut self.phase, addr, data)
            }
            0xff30..=0xff3f => {
                self.wave
                    .write(self.model_is_game_boy_color, self.phase, addr, data)
            }
            _ => {}
        }
    }
}*/
