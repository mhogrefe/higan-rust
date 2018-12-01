//TODO test

use higan::emulator::types::{U12, U3};
use higan::gb::apu::noise::Noise;
use higan::gb::apu::sequencer::Sequencer;
use higan::gb::apu::square_1::Square1;
use higan::gb::apu::square_2::Square2;
use higan::gb::apu::wave::Wave;
use higan::gb::memory::memory::{Bus, MMIOType, MMIO};
use higan::gb::system::system::System;
use malachite_base::num::{One, WrappingAddAssign, Zero};
use nall::random::{LinearFeedbackShiftRegisterGenerator, RandomNumberGenerator};

//TODO impl Thread
//TODO auto APU::Enter() -> void

#[derive(Clone, Debug, Default)]
pub struct APU {
    pub square1: Square1,
    pub square2: Square2,
    pub wave: Wave,
    pub noise: Noise,
    pub sequencer: Sequencer,

    pub phase: U3,  //high 3-bits of clock counter
    pub cycle: U12, //low 12-bits of clock counter
}

impl APU {
    pub fn main(&mut self, system: &System) {
        self.square1.run();
        self.square2.run();
        self.wave.run();
        self.noise.run();
        self.sequencer
            .run(&self.square1, &self.square2, &self.wave, &self.noise);

        if !system.model_is_game_boy_color() {
            //TODO stream->sample(sequencer.left / 32768.0, sequencer.right / 32768.0);
        } else {
            //TODO double samples[] = {sequencer.left / 32768.0, sequencer.right / 32768.0};
            //TODO superGameBoy->audioSample(samples, 2);
        }

        if self.cycle.0 == 0 {
            //512hz
            if self.phase.0 == 0 || self.phase.0 == 2 || self.phase.0 == 4 || self.phase.0 == 6 {
                //256hz
                self.square1.clock_length();
                self.square2.clock_length();
                self.wave.clock_length();
                self.noise.clock_length();
            }
            if self.phase.0 == 2 || self.phase.0 == 6 {
                //128hz
                self.square1.clock_sweep();
            }
            if self.phase.0 == 7 {
                //64hz
                self.square1.clock_envelope();
                self.square2.clock_envelope();
                self.noise.clock_envelope();
            }
            self.phase.wrapping_add_assign(U3::ONE);
        }
        self.cycle.wrapping_add_assign(U12::ONE);

        //TODO Thread::step(1);
        //TODO synchronize(cpu);
    }
}

impl Bus {
    pub fn power_apu(&mut self) {
        //TODO create(Enter, 2 * 1024 * 1024);
        //TODO if(!Model::SuperGameBoy()) {
        //TODO   stream = Emulator::audio.createStream(2, frequency());
        //TODO   stream->addFilter(Emulator::Filter::Order::First, Emulator::Filter::Type::HighPass, 20.0);
        //TODO   stream->addFilter(Emulator::Filter::Order::Second, Emulator::Filter::Type::LowPass, 20000.0, 3);
        //TODO }
        for n in 0xff10..=0xff3f {
            self.mmio[n] = MMIOType::APU;
        }

        self.apu.square1.power(false);
        self.apu.square2.power(false);
        self.apu.wave.power(false);
        self.apu.noise.power(false);
        self.apu.sequencer.power();
        self.apu.phase = U3::ZERO;
        self.apu.cycle = U12::ZERO;

        let mut r = LinearFeedbackShiftRegisterGenerator::new();
        for n in self.apu.wave.pattern.iter_mut() {
            *n = r.call() as u8;
        }
    }
}

impl MMIO for APU {
    fn read_io(&self, system: &System, addr: u16) -> u8 {
        match addr {
            0xff10...0xff14 => self.square1.read(addr),
            0xff15...0xff19 => self.square2.read(addr),
            0xff1a...0xff1e => self.wave.read(system, addr),
            0xff1f...0xff23 => self.noise.read(addr),
            0xff24...0xff26 => self.sequencer.read(self, addr),
            0xff30...0xff3f => self.wave.read(system, addr),
            _ => 0xff,
        }
    }

    fn write_io(&mut self, system: &System, addr: u16, mut data: u8) {
        if !self.sequencer.enable {
            let mut valid = addr == 0xff26; //NR52
            if !system.model_is_game_boy_color() {
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
            0xff10...0xff14 => self.square1.write(self.phase, addr, data),
            0xff15...0xff19 => self.square2.write(self.phase, addr, data),
            0xff1a...0xff1e => self.wave.write(system, self.phase, addr, data),
            0xff1f...0xff23 => self.noise.write(self.phase, addr, data),
            0xff24...0xff26 => self.sequencer.write(
                system,
                &mut self.phase,
                &mut self.square1,
                &mut self.square2,
                &mut self.wave,
                &mut self.noise,
                addr,
                data,
            ),
            0xff30...0xff3f => self.wave.write(system, self.phase, addr, data),
            _ => {}
        }
    }
}
