use higan_rust::ares::emulator::types::{U2, U3, U4, U5};
use higan_rust::ares::gb::apu::apu::APU;
use higan_rust::ares::gb::apu::wave::Wave;
use malachite_base::num::conversion::traits::WrappingFrom;

fn read_helper(apu: &APU, address: u16) -> u8 {
    apu.read_io(2, address, 0xff, false)
}

fn read_helper_gbc(apu: &APU, address: u16) -> u8 {
    apu.read_io(2, address, 0xff, true)
}

fn power_and_zero_pattern_wave(wave: &mut Wave) {
    wave.power(true);
    for p in wave.pattern.iter_mut() {
        *p = 0;
    }
}

#[test]
fn test_read() {
    let mut apu = APU::default();

    // Noise
    apu.noise.power(true);
    assert_eq!(read_helper(&apu, 0), 0xff);

    apu.noise.power(true);
    assert_eq!(read_helper(&apu, 0xff1f), 0xff);

    apu.noise.power(true);
    assert_eq!(read_helper(&apu, 0xff20), 0xff);

    apu.noise.power(true);
    apu.noise.envelope_volume = U4::wrapping_from(0b1011);
    apu.noise.envelope_direction = true;
    apu.noise.envelope_frequency = U3::wrapping_from(0b010);
    assert_eq!(read_helper(&apu, 0xff21), 0b10111010);

    apu.noise.power(true);
    apu.noise.frequency = U4::wrapping_from(0b1011);
    apu.noise.narrow = true;
    apu.noise.divisor = U3::wrapping_from(0b010);
    assert_eq!(read_helper(&apu, 0xff22), 0b10111010);

    apu.noise.power(true);
    apu.noise.counter = false;
    assert_eq!(read_helper(&apu, 0xff23), 0b10111111);

    // Sequencer

    apu.sequencer.power();
    assert_eq!(read_helper(&apu, 0), 0xff);

    apu.sequencer.power();
    apu.sequencer.left_enable = true;
    apu.sequencer.left_volume = U3::wrapping_from(0b010);
    apu.sequencer.right_enable = false;
    apu.sequencer.right_volume = U3::wrapping_from(0b101);
    assert_eq!(read_helper(&apu, 0xff24), 0b10100101);

    apu.sequencer.power();
    apu.sequencer.noise.left_enable = true;
    apu.sequencer.wave.left_enable = false;
    apu.sequencer.square_2.left_enable = true;
    apu.sequencer.square_1.left_enable = false;
    apu.sequencer.noise.right_enable = false;
    apu.sequencer.wave.right_enable = true;
    apu.sequencer.square_2.right_enable = false;
    apu.sequencer.square_1.right_enable = true;
    assert_eq!(read_helper(&apu, 0xff25), 0b10100101);

    apu.sequencer.power();
    apu.sequencer.enable = true;
    apu.noise.enable = false;
    apu.wave.enable = true;
    apu.square_2.enable = false;
    apu.square_1.enable = true;
    assert_eq!(read_helper(&apu, 0xff26), 0b11110101);

    // Square 1
    apu.square_1.power(true);
    assert_eq!(read_helper(&apu, 0), 0xff);

    apu.square_1.power(true);
    apu.square_1.sweep_frequency = U3::wrapping_from(0b101);
    apu.square_1.sweep_direction = true;
    apu.square_1.sweep_shift = U3::wrapping_from(0b010);
    assert_eq!(read_helper(&apu, 0xff10), 0b11011010);

    apu.square_1.power(true);
    apu.square_1.duty = U2::wrapping_from(0b01);
    assert_eq!(read_helper(&apu, 0xff11), 0b01111111);

    apu.square_1.power(true);
    apu.square_1.envelope_volume = U4::wrapping_from(0b1011);
    apu.square_1.envelope_direction = true;
    apu.square_1.envelope_frequency = U3::wrapping_from(0b010);
    assert_eq!(read_helper(&apu, 0xff12), 0b10111010);

    apu.square_1.power(true);
    assert_eq!(read_helper(&apu, 0xff13), 0b11111111);

    apu.square_1.power(true);
    apu.square_1.counter = false;
    assert_eq!(read_helper(&apu, 0xff14), 0b10111111);

    // Square 2

    apu.square_2.power(true);
    assert_eq!(read_helper(&apu, 0), 0xff);

    apu.square_2.power(true);
    assert_eq!(read_helper(&apu, 0xff15), 0b11111111);

    apu.square_2.power(true);
    apu.square_2.duty = U2::wrapping_from(0b01);
    assert_eq!(read_helper(&apu, 0xff16), 0b01111111);

    apu.square_2.power(true);
    apu.square_2.envelope_volume = U4::wrapping_from(0b1011);
    apu.square_2.envelope_direction = true;
    apu.square_2.envelope_frequency = U3::wrapping_from(0b010);
    assert_eq!(read_helper(&apu, 0xff17), 0b10111010);

    apu.square_2.power(true);
    assert_eq!(read_helper(&apu, 0xff18), 0b11111111);

    apu.square_2.power(true);
    apu.square_2.counter = false;
    assert_eq!(read_helper(&apu, 0xff19), 0b10111111);

    // apu.wave
    power_and_zero_pattern_wave(&mut apu.wave);
    assert_eq!(read_helper(&apu, 0), 0xff);

    power_and_zero_pattern_wave(&mut apu.wave);
    apu.wave.dac_enable = false;
    assert_eq!(read_helper(&apu, 0xff1a), 0b01111111);

    power_and_zero_pattern_wave(&mut apu.wave);
    assert_eq!(read_helper(&apu, 0xff1b), 0b11111111);

    power_and_zero_pattern_wave(&mut apu.wave);
    apu.wave.volume = U2::wrapping_from(0b10);
    assert_eq!(read_helper(&apu, 0xff1c), 0b11011111);

    power_and_zero_pattern_wave(&mut apu.wave);
    assert_eq!(read_helper(&apu, 0xff1d), 0b11111111);

    power_and_zero_pattern_wave(&mut apu.wave);
    apu.wave.counter = false;
    assert_eq!(read_helper(&apu, 0xff1e), 0b10111111);

    // Model::GameBoyColor() is false, pattern_hold is zero
    power_and_zero_pattern_wave(&mut apu.wave);
    apu.wave.enable = true;
    apu.wave.pattern_hold = 0;
    apu.wave.pattern_offset = U5::wrapping_from(3);
    apu.wave.pattern[1] = 0xab;
    assert_eq!(read_helper(&apu, 0xff3a), 0xff);

    // Model::GameBoyColor() is false, pattern_hold is nonzero
    power_and_zero_pattern_wave(&mut apu.wave);
    apu.wave.enable = true;
    apu.wave.pattern_hold = 5;
    apu.wave.pattern_offset = U5::wrapping_from(3);
    apu.wave.pattern[1] = 0xab;
    assert_eq!(read_helper(&apu, 0xff3a), 0xab);

    // Model::GameBoyColor() is true, pattern_hold is zero
    power_and_zero_pattern_wave(&mut apu.wave);
    apu.wave.enable = true;
    apu.wave.pattern_hold = 0;
    apu.wave.pattern_offset = U5::wrapping_from(3);
    apu.wave.pattern[1] = 0xab;
    assert_eq!(read_helper_gbc(&apu, 0xff3a), 0xab);

    // enable is false
    power_and_zero_pattern_wave(&mut apu.wave);
    apu.wave.enable = false;
    apu.wave.pattern_hold = 0;
    apu.wave.pattern_offset = U5::wrapping_from(3);
    apu.wave.pattern[5] = 0xab;
    assert_eq!(read_helper(&apu, 0xff35), 0xab);
}
