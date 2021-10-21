use higan::emulator::types::U3;
use higan::gb::apu::noise::Noise;
use higan::gb::apu::square_1::Square1;
use higan::gb::apu::square_2::Square2;
use higan::gb::apu::wave::Wave;

#[derive(Clone, Debug, Default)]
pub struct Channel {
    pub left_enable: bool,
    pub right_enable: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Sequencer {
    pub square_1: Square1,
    pub square_2: Square2,
    pub wave: Wave,
    pub noise: Noise,

    pub left_enable: bool,
    pub left_volume: U3,
    pub right_enable: bool,
    pub right_volume: U3,

    pub square_1_channel: Channel,
    pub square_2_channel: Channel,
    pub wave_channel: Channel,
    pub noise_channel: Channel,

    pub enable: bool,

    pub center: i16,
    pub left: i16,
    pub right: i16,
}

impl Sequencer {
    pub fn run(&mut self) {
        if !self.enable {
            self.center = 0;
            self.left = 0;
            self.right = 0;
            return;
        }

        let mut sample: i32 = 0;
        sample.wrapping_add_assign(i32::from(self.square_1.output));
        sample.wrapping_add_assign(i32::from(self.square_2.output));
        sample.wrapping_add_assign(i32::from(self.wave.output));
        sample.wrapping_add_assign(i32::from(self.noise.output));
        self.center = i16::wrapping_from(sample)
            .wrapping_mul(512)
            .wrapping_sub(16_384);

        sample = 0;
        if self.square_1_channel.left_enable {
            sample.wrapping_add_assign(i32::from(self.square_1.output));
        }
        if self.square_2_channel.left_enable {
            sample.wrapping_add_assign(i32::from(self.square_2.output));
        }
        if self.wave_channel.left_enable {
            sample.wrapping_add_assign(i32::from(self.wave.output));
        }
        if self.noise_channel.left_enable {
            sample.wrapping_add_assign(i32::from(self.noise.output));
        }
        sample = sample.wrapping_mul(512).wrapping_sub(16_384);
        sample = sample.wrapping_mul(i32::from(self.left_volume.wrapping_add(U3::ONE).0)) / 8;
        self.left = i16::wrapping_from(sample);

        sample = 0;
        if self.square_1_channel.right_enable {
            sample.wrapping_add_assign(i32::from(self.square_1.output));
        }
        if self.square_2_channel.right_enable {
            sample.wrapping_add_assign(i32::from(self.square_2.output));
        }
        if self.wave_channel.right_enable {
            sample.wrapping_add_assign(i32::from(self.wave.output));
        }
        if self.noise_channel.right_enable {
            sample.wrapping_add_assign(i32::from(self.noise.output));
        }
        sample = sample.wrapping_mul(512).wrapping_sub(16_384);
        sample = sample.wrapping_mul(i32::from(self.right_volume.wrapping_add(U3::ONE).0)) / 8;
        self.right = i16::wrapping_from(sample);

        //reduce audio volume
        self.center >>= 1;
        self.left >>= 1;
        self.right >>= 1;
    }

    pub fn read(&self, addr: u16) -> u8 {
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
                (if self.noise_channel.left_enable { 1 } else { 0 }) << 7
                    | (if self.wave_channel.left_enable { 1 } else { 0 }) << 6
                    | (if self.square_2_channel.left_enable {
                        1
                    } else {
                        0
                    }) << 5
                    | (if self.square_1_channel.left_enable {
                        1
                    } else {
                        0
                    }) << 4
                    | (if self.noise_channel.right_enable {
                        1
                    } else {
                        0
                    }) << 3
                    | (if self.wave_channel.right_enable { 1 } else { 0 }) << 2
                    | (if self.square_2_channel.right_enable {
                        1
                    } else {
                        0
                    }) << 1
                    | (if self.square_1_channel.right_enable {
                        1
                    } else {
                        0
                    }) << 0
            }
            //NR52
            0xff26 => {
                (if self.enable { 1 } else { 0 }) << 7
                    | 0x70
                    | (if self.noise.enable { 1 } else { 0 }) << 3
                    | (if self.wave.enable { 1 } else { 0 }) << 2
                    | (if self.square_2.enable { 1 } else { 0 }) << 1
                    | (if self.square_1.enable { 1 } else { 0 }) << 0
            }
            _ => 0xff,
        }
    }

    pub fn write(
        &mut self,
        model_is_game_boy_color: bool,
        apu_phase: &mut U3,
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
                self.noise_channel.left_enable = data.get_bit(7);
                self.wave_channel.left_enable = data.get_bit(6);
                self.square_2_channel.left_enable = data.get_bit(5);
                self.square_1_channel.left_enable = data.get_bit(4);
                self.noise_channel.right_enable = data.get_bit(3);
                self.wave_channel.right_enable = data.get_bit(2);
                self.square_1_channel.right_enable = data.get_bit(1);
                self.square_1_channel.right_enable = data.get_bit(0);
            }
            //NR52
            0xff26 => {
                if self.enable != data.get_bit(7) {
                    self.enable = data.get_bit(7);

                    if !self.enable {
                        //power(bool) resets length counters when true (eg for CGB only)
                        self.square_1.power(model_is_game_boy_color);
                        self.square_2.power(model_is_game_boy_color);
                        self.wave.power(model_is_game_boy_color);
                        self.noise.power(model_is_game_boy_color);
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
        let old_square_1 = self.square_1.clone();
        let old_square_2 = self.square_2.clone();
        let old_wave = self.wave.clone();
        let old_noise = self.noise.clone();
        *self = Sequencer::default();
        self.square_1 = old_square_1;
        self.square_2 = old_square_2;
        self.wave = old_wave;
        self.noise = old_noise;
    }
}
