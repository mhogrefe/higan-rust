use ares::emulator::types::U4;
use ares::gb::apu::apu::APU;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::{BitAccess, BitBlockAccess};

impl APU {
    /// See higan-rust/cpp/ares/gb/apu/io.cpp
    pub fn read_io(
        &self,
        cycle: u32,
        address: u16,
        mut data: u8,
        model_is_game_boy_color: bool,
    ) -> u8 {
        match (address, cycle) {
            (address, _) if address < 0xff10 || address > 0xff3f => data,

            //NR10
            (0xff10, 2) => {
                data.assign_bits(0, 3, &self.square_1.sweep_shift.x());
                data.assign_bit(3, self.square_1.sweep_direction);
                data.assign_bits(4, 7, &self.square_1.sweep_frequency.x());
                data
            }

            //NR11
            (0xff11, 2) => {
                data.assign_bits(6, 8, &self.square_1.duty.x());
                data
            }

            //NR12
            (0xff12, 2) => {
                data.assign_bits(0, 3, &self.square_1.envelope_frequency.x());
                data.assign_bit(3, self.square_1.envelope_direction);
                data.assign_bits(4, 8, &self.square_1.envelope_volume.x());
                data
            }

            //NR13
            (0xff13, 2) => data,

            //NR14
            (0xff14, 2) => {
                data.assign_bit(6, self.square_1.counter);
                data
            }

            //NR20
            (0xff15, 2) => data,

            //NR21
            (0xff16, 2) => {
                data.assign_bits(6, 8, &self.square_2.duty.x());
                data
            }

            //NR22
            (0xff17, 2) => {
                data.assign_bits(0, 3, &self.square_2.envelope_frequency.x());
                data.assign_bit(3, self.square_2.envelope_direction);
                data.assign_bits(4, 8, &self.square_2.envelope_volume.x());
                data
            }

            //NR23
            (0xff18, 2) => data,

            //NR24
            (0xff19, 2) => {
                data.assign_bit(6, self.square_2.counter);
                data
            }

            //NR30
            (0xff1a, 2) => {
                data.assign_bit(7, self.wave.dac_enable);
                data
            }

            //NR31
            (0xff1b, 2) => data,

            //NR32
            (0xff1c, 2) => {
                data.assign_bits(5, 7, &self.wave.volume.x());
                data
            }

            //NR33
            (0xff1d, 2) => data,

            //NR34
            (0xff1e, 2) => {
                data.assign_bit(6, self.wave.counter);
                data
            }

            //NR40
            (0xff1f, 2) => data,

            //NR41
            (0xff20, 2) => data,

            //NR42
            (0xff21, 2) => {
                data.assign_bits(0, 3, &self.noise.envelope_frequency.x());
                data.assign_bit(3, self.noise.envelope_direction);
                data.assign_bits(4, 8, &self.noise.envelope_volume.x());
                data
            }

            //NR43
            (0xff22, 2) => {
                data.assign_bits(0, 3, &self.noise.divisor.x());
                data.assign_bit(3, self.noise.narrow);
                data.assign_bits(4, 8, &self.noise.frequency.x());
                data
            }

            //NR44
            (0xff23, 2) => {
                data.assign_bit(6, self.noise.counter);
                data
            }

            //NR50
            (0xff24, 2) => {
                data.assign_bits(0, 3, &self.sequencer.right_volume.x());
                data.assign_bit(3, self.sequencer.right_enable);
                data.assign_bits(4, 7, &self.sequencer.left_volume.x());
                data.assign_bit(7, self.sequencer.left_enable);
                data
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
                data
            }

            //NR52
            (0xff26, _) => {
                data.assign_bit(0, self.square_1.enable);
                data.assign_bit(1, self.square_2.enable);
                data.assign_bit(2, self.wave.enable);
                data.assign_bit(3, self.noise.enable);
                data.assign_bit(7, self.sequencer.enable);
                data
            }

            (address, 2) if address >= 0xff30 && address <= 0xff3f => {
                self.wave
                    .read_ram(U4::wrapping_from(address), data, model_is_game_boy_color)
            }
            _ => data,
        }
    }
}
