//TODO test

use higan::emulator::types::{U12, U3};
use higan::gb::apu::sequencer::Sequencer;
use higan::gb::memory::memory::{Bus, MMIOType, MMIO};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::arithmetic::traits::WrappingAddAssign;
use nall::random::{LinearFeedbackShiftRegisterGenerator, RandomNumberGenerator};

//TODO impl Thread
//TODO auto APU::Enter() -> void

#[derive(Clone, Debug, Default)]
pub struct APU {
    pub model_is_game_boy_color: bool,
    pub sequencer: Sequencer,

    pub phase: U3,  //high 3-bits of clock counter
    pub cycle: U12, //low 12-bits of clock counter
}

impl APU {
    pub fn main(&mut self) {
        self.sequencer.square_1.run();
        self.sequencer.square_2.run();
        self.sequencer.wave.run();
        self.sequencer.noise.run();
        self.sequencer.run();

        if !self.model_is_game_boy_color {
            //TODO stream->sample(sequencer.left / 32768.0, sequencer.right / 32768.0);
        } else {
            //TODO double samples[] = {sequencer.left / 32768.0, sequencer.right / 32768.0};
            //TODO superGameBoy->audioSample(samples, 2);
        }

        if self.cycle.x() == 0 {
            //512hz
            if self.phase.x() == 0 || self.phase.x() == 2 || self.phase.x() == 4 || self.phase.x() == 6 {
                //256hz
                self.sequencer.square_1.clock_length();
                self.sequencer.square_2.clock_length();
                self.sequencer.wave.clock_length();
                self.sequencer.noise.clock_length();
            }
            if self.phase.x() == 2 || self.phase.x() == 6 {
                //128hz
                self.sequencer.square_1.clock_sweep();
            }
            if self.phase.x() == 7 {
                //64hz
                self.sequencer.square_1.clock_envelope();
                self.sequencer.square_2.clock_envelope();
                self.sequencer.noise.clock_envelope();
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

        self.apu.sequencer.square_1.power(false);
        self.apu.sequencer.square_2.power(false);
        self.apu.sequencer.wave.power(false);
        self.apu.sequencer.noise.power(false);
        self.apu.sequencer.power();
        self.apu.phase = U3::ZERO;
        self.apu.cycle = U12::ZERO;

        let mut r = LinearFeedbackShiftRegisterGenerator::new();
        for n in self.apu.sequencer.wave.pattern.iter_mut() {
            *n = r.call() as u8;
        }
    }
}

impl MMIO for APU {
    fn read_io(&self, addr: u16) -> u8 {
        match addr {
            0xff10..=0xff14 => self.sequencer.square_1.read(addr),
            0xff15..=0xff19 => self.sequencer.square_2.read(addr),
            0xff1a..=0xff1e => self.sequencer.wave.read(self.model_is_game_boy_color, addr),
            0xff1f..=0xff23 => self.sequencer.noise.read(addr),
            0xff24..=0xff26 => self.sequencer.read(addr),
            0xff30..=0xff3f => self.sequencer.wave.read(self.model_is_game_boy_color, addr),
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
            0xff10..=0xff14 => self.sequencer.square_1.write(self.phase, addr, data),
            0xff15..=0xff19 => self.sequencer.square_2.write(self.phase, addr, data),
            0xff1a..=0xff1e => {
                self.sequencer
                    .wave
                    .write(self.model_is_game_boy_color, self.phase, addr, data)
            }
            0xff1f..=0xff23 => self.sequencer.noise.write(self.phase, addr, data),
            0xff24..=0xff26 => {
                self.sequencer
                    .write(self.model_is_game_boy_color, &mut self.phase, addr, data)
            }
            0xff30..=0xff3f => {
                self.sequencer
                    .wave
                    .write(self.model_is_game_boy_color, self.phase, addr, data)
            }
            _ => {}
        }
    }
}
