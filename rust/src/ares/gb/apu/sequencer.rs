use ares::emulator::types::U3;

/// See higan-rust/cpp/ares/gb/apu/apu.hpp
#[derive(Clone, Debug, Default)]
pub struct Channel {
    pub left_enable: bool,
    pub right_enable: bool,
}

/// See higan-rust/cpp/ares/gb/apu/apu.hpp
#[derive(Clone, Debug, Default)]
pub struct Sequencer {
    pub left_enable: bool,
    pub left_volume: U3,
    pub right_enable: bool,
    pub right_volume: U3,

    pub square_1: Channel,
    pub square_2: Channel,
    pub wave: Channel,
    pub noise: Channel,

    pub enable: bool,

    pub center: i16,
    pub left: i16,
    pub right: i16,
}

impl Sequencer {
    /*
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
                self.left_volume = U3::wrapping_from(data.get_bits(4, 7));
                self.right_enable = data.get_bit(3);
                self.right_volume = U3::wrapping_from(data.get_bits(0, 3));
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
    }*/

    /// See higan-rust/cpp/ares/gb/apu/sequencer.cpp
    pub fn power(&mut self) {
        *self = Sequencer::default();
    }
}
