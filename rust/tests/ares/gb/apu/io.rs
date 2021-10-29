use higan_rust::ares::emulator::types::{U11, U15, U2, U3, U4, U5};
use higan_rust::ares::gb::apu::apu::APU;
use higan_rust::ares::gb::apu::wave::Wave;
use higan_rust::ares::gb::bus::Bus;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::WrappingFrom;

fn read_helper(apu: &APU, address: u16) -> u8 {
    apu.read_io(2, address, 0xff)
}

fn power_and_zero_pattern_wave(wave: &mut Wave) {
    wave.power(true);
    for p in wave.pattern.iter_mut() {
        *p = 0;
    }
}

#[test]
fn test_read_io() {
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
    apu.model_is_game_boy_color = true;
    power_and_zero_pattern_wave(&mut apu.wave);
    apu.wave.enable = true;
    apu.wave.pattern_hold = 0;
    apu.wave.pattern_offset = U5::wrapping_from(3);
    apu.wave.pattern[1] = 0xab;
    assert_eq!(read_helper(&apu, 0xff3a), 0xab);

    // enable is false
    apu.model_is_game_boy_color = false;
    power_and_zero_pattern_wave(&mut apu.wave);
    apu.wave.enable = false;
    apu.wave.pattern_hold = 0;
    apu.wave.pattern_offset = U5::wrapping_from(3);
    apu.wave.pattern[5] = 0xab;
    assert_eq!(read_helper(&apu, 0xff35), 0xab);
}

fn write_helper(apu: &mut APU, address: u16, data: u8) {
    apu.write_io(2, address, data);
}

fn write_helper_with_cycle(apu: &mut APU, cycle: u32, address: u16, data: u8) {
    apu.write_io(cycle, address, data);
}

#[test]
fn test_write_io() {
    // Noise
    let mut bus = Bus::default();
    bus.power_apu();
    bus.apu.sequencer.enable = true;

    bus.apu.noise.power(true);
    write_helper(&mut bus.apu, 0xff20, 0b10110100);
    assert_eq!(bus.apu.noise.length, 12);

    bus.apu.noise.power(true);
    bus.apu.noise.enable = true;
    write_helper(&mut bus.apu, 0xff21, 0b10111010);
    assert_eq!(bus.apu.noise.envelope_volume, U4::wrapping_from(0b1011));
    assert!(bus.apu.noise.envelope_direction);
    assert_eq!(bus.apu.noise.envelope_frequency, U3::wrapping_from(0b010));
    assert!(bus.apu.noise.enable);

    bus.apu.noise.power(true);
    bus.apu.noise.enable = true;
    write_helper(&mut bus.apu, 0xff21, 0);
    assert_eq!(bus.apu.noise.envelope_volume, U4::ZERO);
    assert!(!bus.apu.noise.envelope_direction);
    assert_eq!(bus.apu.noise.envelope_frequency, U3::ZERO);
    assert!(!bus.apu.noise.enable);

    bus.apu.noise.power(true);
    bus.apu.noise.enable = true;
    write_helper(&mut bus.apu, 0xff22, 0b10111010);
    assert_eq!(bus.apu.noise.frequency, U4::wrapping_from(0b1011));
    assert!(bus.apu.noise.narrow);
    assert_eq!(bus.apu.noise.divisor, U3::wrapping_from(0b010));

    // data.bit(6) is false, data.bit(7) is true
    bus.apu.noise.power(true);
    write_helper_with_cycle(&mut bus.apu, 4, 0xff23, 0b10110011);
    assert!(!bus.apu.noise.enable);
    assert!(!bus.apu.noise.counter);
    assert_eq!(bus.apu.noise.envelope_period, U3::ZERO);
    assert_eq!(bus.apu.noise.lfsr, U15::wrapping_from(0x7fff));
    assert_eq!(bus.apu.noise.volume, U4::ZERO);
    assert_eq!(bus.apu.noise.length, 64);

    // data.bit(6) is false, data.bit(7) is false. Length stays 0
    bus.apu.noise.power(true);
    bus.apu.noise.enable = true;
    bus.apu.noise.length = 0;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff23, 0b00110011);
    assert!(bus.apu.noise.enable);
    assert!(!bus.apu.noise.counter);
    assert_eq!(bus.apu.noise.length, 0);

    // data.bit(6) is true, data.bit(7) is true, enable stays true
    bus.apu.noise.power(true);
    bus.apu.noise.length = 1;
    bus.apu.noise.enable = true;
    bus.apu.noise.envelope_volume = U4::wrapping_from(5);
    bus.apu.noise.envelope_direction = true;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff23, 0b11110011);
    assert!(bus.apu.noise.enable);
    assert!(bus.apu.noise.counter);
    assert_eq!(bus.apu.noise.envelope_period, U3::ZERO);
    assert_eq!(bus.apu.noise.lfsr, U15::wrapping_from(0x7fff));
    assert_eq!(bus.apu.noise.volume, U4::wrapping_from(5));
    assert_eq!(bus.apu.noise.length, 1);

    // same as previous, but length is initially 0 and becomes 64
    bus.apu.noise.power(true);
    bus.apu.noise.enable = true;
    bus.apu.noise.envelope_volume = U4::wrapping_from(5);
    bus.apu.noise.length = 0;
    bus.apu.noise.envelope_direction = true;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff23, 0b11110011);
    assert!(bus.apu.noise.enable);
    assert!(bus.apu.noise.counter);
    assert_eq!(bus.apu.noise.envelope_period, U3::ZERO);
    assert_eq!(bus.apu.noise.lfsr, U15::wrapping_from(0x7fff));
    assert_eq!(bus.apu.noise.volume, U4::wrapping_from(5));
    assert_eq!(bus.apu.noise.length, 64);

    // same as previous, but length is initially 0 and becomes 63 because of
    // apu.phase
    bus.power_apu();
    bus.apu.sequencer.enable = true;
    bus.apu.noise.power(true);
    bus.apu.phase = U3::ONE;
    bus.apu.noise.enable = true;
    bus.apu.noise.envelope_volume = U4::wrapping_from(5);
    bus.apu.noise.length = 0;
    bus.apu.noise.envelope_direction = true;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff23, 0b11110011);
    assert!(bus.apu.noise.enable);
    assert!(bus.apu.noise.counter);
    assert_eq!(bus.apu.noise.envelope_period, U3::ZERO);
    assert_eq!(bus.apu.noise.lfsr, U15::wrapping_from(0x7fff));
    assert_eq!(bus.apu.noise.volume, U4::wrapping_from(5));
    assert_eq!(bus.apu.noise.length, 63);
    // clear phase
    bus.power_apu();

    // data.bit(6) is true, data.bit(7) is false, enable stays true
    bus.apu.noise.power(true);
    bus.apu.sequencer.enable = true;
    bus.apu.noise.length = 1;
    bus.apu.noise.enable = true;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff23, 0b01110011);
    assert!(bus.apu.noise.enable);
    assert!(bus.apu.noise.counter);
    assert_eq!(bus.apu.noise.length, 1);

    // same as previous, but apu.phase = 1
    bus.power_apu();
    bus.apu.noise.power(true);
    bus.apu.sequencer.enable = true;
    bus.apu.phase = U3::ONE;
    bus.apu.noise.length = 1;
    bus.apu.noise.enable = true;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff23, 0b01110011);

    assert!(!bus.apu.noise.enable);
    assert!(bus.apu.noise.counter);
    assert_eq!(bus.apu.noise.length, 0);
    // clear phase
    bus.power_apu();

    // Sequencer
    bus.apu.sequencer.power();
    bus.apu.sequencer.enable = true;
    write_helper(&mut bus.apu, 0xff24, 0b10100101);
    assert!(bus.apu.sequencer.left_enable);
    assert_eq!(bus.apu.sequencer.left_volume, U3::wrapping_from(0b010));
    assert!(!bus.apu.sequencer.right_enable);
    assert_eq!(bus.apu.sequencer.right_volume, U3::wrapping_from(0b101));

    bus.apu.sequencer.power();
    bus.apu.sequencer.enable = true;
    write_helper(&mut bus.apu, 0xff25, 0b10100101);
    assert!(bus.apu.sequencer.noise.left_enable);
    assert!(!bus.apu.sequencer.wave.left_enable);
    assert!(bus.apu.sequencer.square_2.left_enable);
    assert!(!bus.apu.sequencer.square_1.left_enable);
    assert!(!bus.apu.sequencer.noise.right_enable);
    assert!(bus.apu.sequencer.wave.right_enable);
    assert!(!bus.apu.sequencer.square_2.right_enable);
    assert!(bus.apu.sequencer.square_1.right_enable);

    // enable and data.bit(7) both false, so nothing happens
    bus.apu.sequencer.power();
    bus.apu.square_1.power(true);
    bus.apu.square_1.period = 5;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff26, 0);
    assert_eq!(bus.apu.square_1.period, 5);
    bus.apu.square_1.power(true);

    // enable and data.bit(7) both true, so nothing happens
    bus.apu.sequencer.power();
    bus.apu.square_1.power(true);
    bus.apu.square_1.period = 5;
    bus.apu.sequencer.enable = true;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff26, 0b10000000);
    assert_eq!(bus.apu.square_1.period, 5);
    bus.apu.square_1.power(true);

    // enable is false and data.bit(7) is true, so bus.apu phase is set to 0
    bus.power_apu();
    bus.apu.sequencer.power();
    bus.apu.phase = U3::wrapping_from(5);
    write_helper_with_cycle(&mut bus.apu, 4, 0xff26, 0b10000000);
    assert_eq!(bus.apu.phase, U3::ZERO);
    // clear phase
    bus.power_apu();

    // enable is true, data.bit(7) is false, and model is not GBC, so bus.apu
    // components are powered without initializing length
    bus.apu.sequencer.power();
    bus.apu.square_1.power(true);
    bus.apu.square_1.period = 5;
    bus.apu.square_1.length = 5;
    bus.apu.sequencer.enable = true;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff26, 0);
    assert_eq!(bus.apu.square_1.period, 0);
    assert_eq!(bus.apu.square_1.length, 5);
    bus.apu.square_1.power(true);

    // enable is true, data.bit(7) is false, and model is GBC, so bus.apu components
    // are powered, initializing length
    bus.apu.sequencer.power();
    bus.apu.model_is_game_boy_color = true;
    bus.apu.square_1.power(true);
    bus.apu.square_1.period = 5;
    bus.apu.square_1.length = 5;
    bus.apu.sequencer.enable = true;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff26, 0);
    assert_eq!(bus.apu.square_1.period, 0);
    assert_eq!(bus.apu.square_1.length, 64);
    bus.apu.square_1.power(true);

    // Square 1
    bus.apu.square_1.power(true);
    bus.apu.square_1.enable = true;
    bus.apu.square_1.sweep_enable = true;
    bus.apu.square_1.sweep_negate = true;
    bus.apu.sequencer.enable = true;
    write_helper(&mut bus.apu, 0xff10, 0b11011010);
    assert_eq!(bus.apu.square_1.sweep_frequency, U3::wrapping_from(0b101));
    assert!(bus.apu.square_1.sweep_direction);
    assert_eq!(bus.apu.square_1.sweep_shift, U3::wrapping_from(0b010));
    assert!(bus.apu.square_1.enable);

    bus.apu.square_1.power(true);
    bus.apu.square_1.enable = true;
    bus.apu.square_1.sweep_enable = true;
    bus.apu.square_1.sweep_negate = true;
    write_helper(&mut bus.apu, 0xff10, 0b11010010);
    assert_eq!(bus.apu.square_1.sweep_frequency, U3::wrapping_from(0b101));
    assert!(!bus.apu.square_1.sweep_direction);
    assert_eq!(bus.apu.square_1.sweep_shift, U3::wrapping_from(0b010));
    assert!(!bus.apu.square_1.enable);

    bus.apu.square_1.power(true);
    write_helper(&mut bus.apu, 0xff11, 0b01110010);
    assert_eq!(bus.apu.square_1.duty, U2::wrapping_from(0b01));
    assert_eq!(bus.apu.square_1.length, 14);

    bus.apu.square_1.power(true);
    bus.apu.square_1.enable = true;
    write_helper(&mut bus.apu, 0xff12, 0b10111010);
    assert_eq!(bus.apu.square_1.envelope_volume, U4::wrapping_from(0b1011));
    assert!(bus.apu.square_1.envelope_direction);
    assert_eq!(
        bus.apu.square_1.envelope_frequency,
        U3::wrapping_from(0b010)
    );
    assert!(bus.apu.square_1.enable);

    bus.apu.square_1.power(true);
    bus.apu.square_1.enable = true;
    write_helper(&mut bus.apu, 0xff12, 0);
    assert_eq!(bus.apu.square_1.envelope_volume, U4::ZERO);
    assert!(!bus.apu.square_1.envelope_direction);
    assert_eq!(bus.apu.square_1.envelope_frequency, U3::ZERO);
    assert!(!bus.apu.square_1.enable);

    bus.apu.square_1.power(true);
    write_helper(&mut bus.apu, 0xff13, 0b10110100);
    assert_eq!(bus.apu.square_1.frequency, U11::wrapping_from(0b10110100));

    // data.bit(6) is false, data.bit(7) is true
    bus.apu.square_1.power(true);
    write_helper_with_cycle(&mut bus.apu, 4, 0xff14, 0b10110011);
    assert!(!bus.apu.square_1.enable);
    assert!(!bus.apu.square_1.counter);
    assert_eq!(
        bus.apu.square_1.frequency,
        U11::wrapping_from(0b01100000000)
    );
    assert_eq!(bus.apu.square_1.period, 2560);
    assert_eq!(bus.apu.square_1.envelope_period, U3::ZERO);
    assert_eq!(bus.apu.square_1.volume, U4::ZERO);
    assert_eq!(bus.apu.square_1.length, 64);
    assert_eq!(bus.apu.square_1.frequency_shadow, 768);
    assert!(!bus.apu.square_1.sweep_negate);
    assert_eq!(bus.apu.square_1.sweep_period, U3::ZERO);
    assert!(!bus.apu.square_1.sweep_enable);

    // data.bit(6) is false, data.bit(7) is false. Length stays 0
    bus.apu.square_1.power(true);
    bus.apu.square_1.enable = true;
    bus.apu.square_1.length = 0;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff14, 0b00110011);
    assert!(bus.apu.square_1.enable);
    assert!(!bus.apu.square_1.counter);
    assert_eq!(
        bus.apu.square_1.frequency,
        U11::wrapping_from(0b01100000000)
    );
    assert_eq!(bus.apu.square_1.length, 0);

    // data.bit(6) is true, data.bit(7) is true, enable stays true
    bus.apu.square_1.power(true);
    bus.apu.square_1.length = 1;
    bus.apu.square_1.enable = true;
    bus.apu.square_1.envelope_volume = U4::wrapping_from(5);
    bus.apu.square_1.envelope_direction = true;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff14, 0b11110011);
    assert!(bus.apu.square_1.enable);
    assert!(bus.apu.square_1.counter);
    assert_eq!(
        bus.apu.square_1.frequency,
        U11::wrapping_from(0b01100000000)
    );
    assert_eq!(bus.apu.square_1.period, 2560);
    assert_eq!(bus.apu.square_1.envelope_period, U3::ZERO);
    assert_eq!(bus.apu.square_1.volume, U4::wrapping_from(5));
    assert_eq!(bus.apu.square_1.length, 1);
    assert_eq!(bus.apu.square_1.frequency_shadow, 768);
    assert!(!bus.apu.square_1.sweep_negate);
    assert_eq!(bus.apu.square_1.sweep_period, U3::ZERO);
    assert!(!bus.apu.square_1.sweep_enable);

    bus.apu.square_1.power(true);
    bus.apu.square_1.enable = true;
    bus.apu.square_1.envelope_volume = U4::wrapping_from(5);
    bus.apu.square_1.length = 0;
    bus.apu.square_1.envelope_direction = true;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff14, 0b11110011);
    assert!(bus.apu.square_1.enable);
    assert!(bus.apu.square_1.counter);
    assert_eq!(
        bus.apu.square_1.frequency,
        U11::wrapping_from(0b01100000000)
    );
    assert_eq!(bus.apu.square_1.period, 2560);
    assert_eq!(bus.apu.square_1.envelope_period, U3::ZERO);
    assert_eq!(bus.apu.square_1.volume, U4::wrapping_from(5));
    assert_eq!(bus.apu.square_1.length, 64);
    assert_eq!(bus.apu.square_1.frequency_shadow, 768);
    assert!(!bus.apu.square_1.sweep_negate);
    assert_eq!(bus.apu.square_1.sweep_period, U3::ZERO);
    assert!(!bus.apu.square_1.sweep_enable);

    bus.power_apu();
    bus.apu.square_1.power(true);
    bus.apu.phase = U3::ONE;
    bus.apu.square_1.enable = true;
    bus.apu.square_1.envelope_volume = U4::wrapping_from(5);
    bus.apu.square_1.length = 0;
    bus.apu.square_1.envelope_direction = true;
    bus.apu.sequencer.enable = true;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff14, 0b11110011);
    assert!(bus.apu.square_1.enable);
    assert!(bus.apu.square_1.counter);
    assert_eq!(
        bus.apu.square_1.frequency,
        U11::wrapping_from(0b01100000000)
    );
    assert_eq!(bus.apu.square_1.period, 2560);
    assert_eq!(bus.apu.square_1.envelope_period, U3::ZERO);
    assert_eq!(bus.apu.square_1.volume, U4::wrapping_from(5));
    assert_eq!(bus.apu.square_1.length, 63);
    assert_eq!(bus.apu.square_1.frequency_shadow, 768);
    assert!(!bus.apu.square_1.sweep_negate);
    assert_eq!(bus.apu.square_1.sweep_period, U3::ZERO);
    assert!(!bus.apu.square_1.sweep_enable);
    // clear phase
    bus.power_apu();

    // data.bit(6) is true, data.bit(7) is false, enable stays true
    bus.apu.square_1.power(true);
    bus.apu.square_1.length = 1;
    bus.apu.square_1.enable = true;
    bus.apu.sequencer.enable = true;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff14, 0b01110011);
    assert!(bus.apu.square_1.enable);
    assert!(bus.apu.square_1.counter);
    assert_eq!(
        bus.apu.square_1.frequency,
        U11::wrapping_from(0b01100000000)
    );
    assert_eq!(bus.apu.square_1.length, 1);

    bus.power_apu();
    bus.apu.square_1.power(true);
    bus.apu.phase = U3::ONE;
    bus.apu.square_1.length = 1;
    bus.apu.square_1.enable = true;
    bus.apu.sequencer.enable = true;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff14, 0b01110011);

    assert!(!bus.apu.square_1.enable);
    assert!(bus.apu.square_1.counter);
    assert_eq!(
        bus.apu.square_1.frequency,
        U11::wrapping_from(0b01100000000)
    );
    assert_eq!(bus.apu.square_1.length, 0);
    // clear phase
    bus.power_apu();

    // Square 2
    bus.apu.square_2.power(true);
    bus.apu.sequencer.enable = true;
    write_helper(&mut bus.apu, 0xff16, 0b01110010);
    assert_eq!(bus.apu.square_2.duty, U2::wrapping_from(0b01));
    assert_eq!(bus.apu.square_2.length, 14);

    bus.apu.square_2.power(true);
    bus.apu.square_2.enable = true;
    write_helper(&mut bus.apu, 0xff17, 0b10111010);
    assert_eq!(bus.apu.square_2.envelope_volume, U4::wrapping_from(0b1011));
    assert!(bus.apu.square_2.envelope_direction);
    assert_eq!(
        bus.apu.square_2.envelope_frequency,
        U3::wrapping_from(0b010)
    );
    assert!(bus.apu.square_2.enable);

    bus.apu.square_2.power(true);
    bus.apu.square_2.enable = true;
    write_helper(&mut bus.apu, 0xff17, 0);
    assert_eq!(bus.apu.square_2.envelope_volume, U4::ZERO);
    assert!(!bus.apu.square_2.envelope_direction);
    assert_eq!(bus.apu.square_2.envelope_frequency, U3::ZERO);
    assert!(!bus.apu.square_2.enable);

    bus.apu.square_2.power(true);
    write_helper(&mut bus.apu, 0xff18, 0b10110100);
    assert_eq!(bus.apu.square_2.frequency, U11::wrapping_from(0b10110100));

    // data.bit(6) is false, data.bit(7) is true
    bus.apu.square_2.power(true);
    write_helper_with_cycle(&mut bus.apu, 4, 0xff19, 0b10110011);
    assert!(!bus.apu.square_2.enable);
    assert!(!bus.apu.square_2.counter);
    assert_eq!(
        bus.apu.square_2.frequency,
        U11::wrapping_from(0b01100000000)
    );
    assert_eq!(bus.apu.square_2.period, 2560);
    assert_eq!(bus.apu.square_2.envelope_period, U3::ZERO);
    assert_eq!(bus.apu.square_2.volume, U4::ZERO);
    assert_eq!(bus.apu.square_2.length, 64);

    // data.bit(6) is false, data.bit(7) is false. Length stays 0
    bus.apu.square_2.power(true);
    bus.apu.square_2.enable = true;
    bus.apu.square_2.length = 0;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff19, 0b00110011);
    assert!(bus.apu.square_2.enable);
    assert!(!bus.apu.square_2.counter);
    assert_eq!(
        bus.apu.square_2.frequency,
        U11::wrapping_from(0b01100000000)
    );
    assert_eq!(bus.apu.square_2.length, 0);

    // data.bit(6) is true, data.bit(7) is true, enable stays true
    bus.apu.square_2.power(true);
    bus.apu.square_2.length = 1;
    bus.apu.square_2.enable = true;
    bus.apu.square_2.envelope_volume = U4::wrapping_from(5);
    bus.apu.square_2.envelope_direction = true;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff19, 0b11110011);
    assert!(bus.apu.square_2.enable);
    assert!(bus.apu.square_2.counter);
    assert_eq!(
        bus.apu.square_2.frequency,
        U11::wrapping_from(0b01100000000)
    );
    assert_eq!(bus.apu.square_2.period, 2560);
    assert_eq!(bus.apu.square_2.envelope_period, U3::ZERO);
    assert_eq!(bus.apu.square_2.volume, U4::wrapping_from(5));
    assert_eq!(bus.apu.square_2.length, 1);

    bus.apu.square_2.power(true);
    bus.apu.square_2.enable = true;
    bus.apu.square_2.envelope_volume = U4::wrapping_from(5);
    bus.apu.square_2.length = 0;
    bus.apu.square_2.envelope_direction = true;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff19, 0b11110011);
    assert!(bus.apu.square_2.enable);
    assert!(bus.apu.square_2.counter);
    assert_eq!(
        bus.apu.square_2.frequency,
        U11::wrapping_from(0b01100000000)
    );
    assert_eq!(bus.apu.square_2.period, 2560);
    assert_eq!(bus.apu.square_2.envelope_period, U3::ZERO);
    assert_eq!(bus.apu.square_2.volume, U4::wrapping_from(5));
    assert_eq!(bus.apu.square_2.length, 64);

    bus.power_apu();
    bus.apu.square_2.power(true);
    bus.apu.phase = U3::ONE;
    bus.apu.square_2.enable = true;
    bus.apu.square_2.envelope_volume = U4::wrapping_from(5);
    bus.apu.square_2.length = 0;
    bus.apu.square_2.envelope_direction = true;
    bus.apu.sequencer.enable = true;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff19, 0b11110011);
    assert!(bus.apu.square_2.enable);
    assert!(bus.apu.square_2.counter);
    assert_eq!(
        bus.apu.square_2.frequency,
        U11::wrapping_from(0b01100000000)
    );
    assert_eq!(bus.apu.square_2.period, 2560);
    assert_eq!(bus.apu.square_2.envelope_period, U3::ZERO);
    assert_eq!(bus.apu.square_2.volume, U4::wrapping_from(5));
    assert_eq!(bus.apu.square_2.length, 63);
    // clear phase
    bus.power_apu();

    // data.bit(6) is true, data.bit(7) is false, enable stays true
    bus.apu.square_2.power(true);
    bus.apu.square_2.length = 1;
    bus.apu.square_2.enable = true;
    bus.apu.sequencer.enable = true;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff19, 0b01110011);
    assert!(bus.apu.square_2.enable);
    assert!(bus.apu.square_2.counter);
    assert_eq!(
        bus.apu.square_2.frequency,
        U11::wrapping_from(0b01100000000)
    );
    assert_eq!(bus.apu.square_2.length, 1);

    // same as previous, but apu.phase = 1, so enable becomes false
    bus.power_apu();
    bus.apu.square_2.power(true);
    bus.apu.phase = U3::ONE;
    bus.apu.square_2.length = 1;
    bus.apu.square_2.enable = true;
    bus.apu.sequencer.enable = true;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff19, 0b01110011);

    assert!(!bus.apu.square_2.enable);
    assert!(bus.apu.square_2.counter);
    assert_eq!(
        bus.apu.square_2.frequency,
        U11::wrapping_from(0b01100000000)
    );
    assert_eq!(bus.apu.square_2.length, 0);
    // clear phase
    bus.power_apu();

    // Wave
    power_and_zero_pattern_wave(&mut bus.apu.wave);
    bus.apu.wave.dac_enable = false;
    bus.apu.model_is_game_boy_color = false;
    bus.apu.sequencer.enable = true;
    write_helper(&mut bus.apu, 0xff1a, 0b10000000);
    assert!(bus.apu.wave.dac_enable);

    power_and_zero_pattern_wave(&mut bus.apu.wave);
    bus.apu.wave.dac_enable = true;
    bus.apu.wave.enable = true;
    write_helper(&mut bus.apu, 0xff1a, 0);
    assert!(!bus.apu.wave.dac_enable);
    assert!(!bus.apu.wave.enable);

    power_and_zero_pattern_wave(&mut bus.apu.wave);
    write_helper(&mut bus.apu, 0xff1b, 100);
    assert_eq!(bus.apu.wave.length, 156);

    power_and_zero_pattern_wave(&mut bus.apu.wave);
    write_helper(&mut bus.apu, 0xff1c, 0b01000000);
    assert_eq!(bus.apu.wave.volume, U2::wrapping_from(0b10));

    power_and_zero_pattern_wave(&mut bus.apu.wave);
    write_helper(&mut bus.apu, 0xff1d, 0b10101010);
    assert_eq!(bus.apu.wave.frequency, U11::wrapping_from(0b00010101010));

    // apu.phase.bit(0) is false so enable stays true
    power_and_zero_pattern_wave(&mut bus.apu.wave);
    bus.apu.wave.enable = true;
    bus.apu.wave.length = 1;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff1e, 0b01000101);
    assert!(bus.apu.wave.enable);
    assert_eq!(bus.apu.wave.length, 1);
    assert!(bus.apu.wave.counter);
    assert_eq!(bus.apu.wave.frequency, U11::wrapping_from(0b10100000000));

    // apu.phase.bit(0) is true so enable becomes false
    bus.power_apu();
    power_and_zero_pattern_wave(&mut bus.apu.wave);
    bus.apu.phase = U3::ONE;
    power_and_zero_pattern_wave(&mut bus.apu.wave);
    bus.apu.wave.enable = true;
    bus.apu.wave.length = 1;
    bus.apu.sequencer.enable = true;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff1e, 0b01000000);
    assert!(!bus.apu.wave.enable);
    assert_eq!(bus.apu.wave.length, 0);
    // clear phase
    bus.power_apu();

    // pattern[0] corrupted
    bus.apu.phase = U3::ONE;
    bus.apu.sequencer.enable = true;
    power_and_zero_pattern_wave(&mut bus.apu.wave);
    for i in 0..16 {
        bus.apu.wave.pattern[i] = i as u8;
    }
    bus.apu.wave.pattern_hold = 5;
    bus.apu.wave.pattern_offset = U5::wrapping_from(2);
    write_helper_with_cycle(&mut bus.apu, 4, 0xff1e, 0b11000101);
    assert_eq!(bus.apu.wave.pattern[0], 1);
    assert_eq!(bus.apu.wave.pattern[1], 1);
    assert_eq!(bus.apu.wave.pattern[2], 2);
    assert_eq!(bus.apu.wave.pattern[3], 3);
    assert_eq!(bus.apu.wave.pattern[4], 4);

    // pattern[0-3] corrupted
    power_and_zero_pattern_wave(&mut bus.apu.wave);
    for i in 0..16 {
        bus.apu.wave.pattern[i] = i as u8;
    }
    bus.apu.wave.pattern_hold = 5;
    bus.apu.wave.pattern_offset = U5::wrapping_from(9);
    write_helper_with_cycle(&mut bus.apu, 4, 0xff1e, 0b11000101);
    assert_eq!(bus.apu.wave.pattern[0], 4);
    assert_eq!(bus.apu.wave.pattern[1], 5);
    assert_eq!(bus.apu.wave.pattern[2], 6);
    assert_eq!(bus.apu.wave.pattern[3], 7);
    assert_eq!(bus.apu.wave.pattern[4], 4);

    // no corruption when system is Game Boy Color
    power_and_zero_pattern_wave(&mut bus.apu.wave);
    bus.apu.model_is_game_boy_color = true;
    for i in 0..16 {
        bus.apu.wave.pattern[i] = i as u8;
    }
    bus.apu.wave.pattern_hold = 5;
    bus.apu.wave.pattern_offset = U5::wrapping_from(9);
    write_helper_with_cycle(&mut bus.apu, 4, 0xff1e, 0b11000101);
    assert_eq!(bus.apu.wave.pattern[0], 0);
    assert_eq!(bus.apu.wave.pattern[1], 1);
    assert_eq!(bus.apu.wave.pattern[2], 2);
    assert_eq!(bus.apu.wave.pattern[3], 3);
    assert_eq!(bus.apu.wave.pattern[4], 4);

    // no corruption when data.bit(7) is false
    power_and_zero_pattern_wave(&mut bus.apu.wave);
    for i in 0..16 {
        bus.apu.wave.pattern[i] = i as u8;
    }
    bus.apu.wave.pattern_hold = 5;
    bus.apu.wave.pattern_offset = U5::wrapping_from(9);
    write_helper_with_cycle(&mut bus.apu, 4, 0xff1e, 0b01000101);
    assert_eq!(bus.apu.wave.pattern[0], 0);
    assert_eq!(bus.apu.wave.pattern[1], 1);
    assert_eq!(bus.apu.wave.pattern[2], 2);
    assert_eq!(bus.apu.wave.pattern[3], 3);
    assert_eq!(bus.apu.wave.pattern[4], 4);

    power_and_zero_pattern_wave(&mut bus.apu.wave);
    bus.apu.wave.pattern_offset = U5::wrapping_from(9);
    bus.apu.wave.frequency = U11::ONE;
    bus.apu.wave.pattern_sample = U4::ONE;
    bus.apu.wave.pattern_hold = 5;
    bus.apu.wave.dac_enable = true;
    bus.apu.sequencer.enable = true;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff1e, 0b11000000);
    assert!(bus.apu.wave.enable);
    assert_eq!(bus.apu.wave.period, 2049);
    assert_eq!(bus.apu.wave.pattern_offset, U5::ZERO);
    assert_eq!(bus.apu.wave.pattern_sample, U4::ZERO);
    assert_eq!(bus.apu.wave.pattern_hold, 0);

    power_and_zero_pattern_wave(&mut bus.apu.wave);
    write_helper_with_cycle(&mut bus.apu, 4, 0xff1e, 0b11000000);
    assert_eq!(bus.apu.wave.length, 255);

    bus.apu.phase = U3::ZERO;
    power_and_zero_pattern_wave(&mut bus.apu.wave);
    bus.apu.wave.length = 100;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff1e, 0b11000000);
    assert_eq!(bus.apu.wave.length, 100);

    bus.power_apu();
    bus.apu.sequencer.enable = true;
    power_and_zero_pattern_wave(&mut bus.apu.wave);
    bus.apu.phase = U3::ONE;
    bus.apu.wave.length = 100;
    write_helper_with_cycle(&mut bus.apu, 4, 0xff1e, 0b11000000);
    assert_eq!(bus.apu.wave.length, 99);
    // clear phase
    bus.power_apu();

    bus.apu.sequencer.enable = true;
    bus.apu.model_is_game_boy_color = false;
    power_and_zero_pattern_wave(&mut bus.apu.wave);
    write_helper(&mut bus.apu, 0xff3a, 123);
    assert_eq!(bus.apu.wave.pattern[0xa], 123);

    power_and_zero_pattern_wave(&mut bus.apu.wave);
    bus.apu.wave.pattern_offset = U5::wrapping_from(5);
    bus.apu.wave.enable = true;
    bus.apu.wave.pattern_hold = 5;
    write_helper(&mut bus.apu, 0xff3a, 123);
    assert_eq!(bus.apu.wave.pattern[2], 123);

    bus.power_apu();
    bus.apu.sequencer.enable = true;
    power_and_zero_pattern_wave(&mut bus.apu.wave);
    bus.apu.phase = U3::ONE;
    bus.apu.wave.pattern_offset = U5::wrapping_from(5);
    bus.apu.wave.enable = true;
    write_helper(&mut bus.apu, 0xff3a, 123);
    assert_eq!(bus.apu.wave.pattern[2], 0);
    // clear phase
    bus.power_apu();
}
