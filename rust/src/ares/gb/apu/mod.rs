use ares::emulator::types::{U12, U3};
use ares::gb::apu::noise::Noise;
use ares::gb::apu::sequencer::Sequencer;
use ares::gb::apu::square_1::Square1;
use ares::gb::apu::square_2::Square2;
use ares::gb::apu::wave::Wave;
use ares::gb::system::{Model, System};
use ares::node::audio::stream::Stream;
use ares::platform::Platform;
use malachite_base::num::arithmetic::traits::{WrappingAdd, WrappingAddAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::WrappingFrom;
use nall::random::{Pcg, Rng};

/// See higan-rust/cpp/ares/gb/apu/apu.hpp
#[derive(Clone, Debug, Default)]
pub struct APU {
    pub run_ahead: bool,
    pub stream: Option<Stream>,
    pub model: Model,
    pub square_1: Square1,
    pub square_2: Square2,
    pub noise: Noise,
    pub wave: Wave,
    pub sequencer: Sequencer,

    pub phase: U3,  //high 3-bits of clock counter
    pub cycle: U12, //low 12-bits of clock counter
}

impl APU {
    pub fn stream_frame(&mut self, samples: &[f64]) -> bool {
        if self.run_ahead {
            return false;
        }
        self.stream.as_mut().unwrap().write(samples)
    }

    pub fn load(&mut self) {
        self.stream.as_mut().unwrap().set_channels(2);
        self.stream
            .as_mut()
            .unwrap()
            .set_frequency(2.0 * 1024.0 * 1024.0);
        self.stream
            .as_mut()
            .unwrap()
            .add_high_pass_filter(20.0, 1, 1);
    }

    /// See higan-rust/cpp/ares/gb/apu/apu.cpp
    pub fn main(&mut self) {
        self.square_1.run();
        self.square_2.run();
        self.wave.run();
        self.noise.run();
        self.run_sequencer();
        if self.stream_frame(&[
            f64::from(self.sequencer.left) / 32768.0,
            f64::from(self.sequencer.right) / 32768.0,
        ]) {
            //TODO output audio
        }

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
        let sample = i32::from(self.square_1.output)
            + i32::from(self.square_2.output)
            + i32::from(self.wave.output)
            + i32::from(self.noise.output);
        self.sequencer.center = (i16::wrapping_from(sample) << 9).wrapping_sub(16_384);

        let mut sample = 0;
        if self.sequencer.square_1.left_enable {
            sample += i32::from(self.square_1.output);
        }
        if self.sequencer.square_2.left_enable {
            sample += i32::from(self.square_2.output);
        }
        if self.sequencer.wave.left_enable {
            sample += i32::from(self.wave.output);
        }
        if self.sequencer.noise.left_enable {
            sample += i32::from(self.noise.output);
        }
        sample = (sample << 9).wrapping_sub(16_384);
        sample =
            sample.wrapping_mul(i32::from(self.sequencer.left_volume.wrapping_add(U3::ONE))) >> 3;
        self.sequencer.left = i16::wrapping_from(sample);

        let mut sample = 0;
        if self.sequencer.square_1.right_enable {
            sample += i32::from(self.square_1.output);
        }
        if self.sequencer.square_2.right_enable {
            sample += i32::from(self.square_2.output);
        }
        if self.sequencer.wave.right_enable {
            sample += i32::from(self.wave.output);
        }
        if self.sequencer.noise.right_enable {
            sample += i32::from(self.noise.output);
        }
        sample = (sample << 9).wrapping_sub(16_384);
        sample =
            sample.wrapping_mul(i32::from(self.sequencer.right_volume.wrapping_add(U3::ONE))) >> 3;
        self.sequencer.right = i16::wrapping_from(sample);

        //reduce audio volume
        self.sequencer.center >>= 1;
        self.sequencer.left >>= 1;
        self.sequencer.right >>= 1;
    }
}

impl<P: Platform> System<P> {
    //TODO test
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

pub mod io;
pub mod noise;
pub mod sequencer;
pub mod square_1;
pub mod square_2;
pub mod wave;
