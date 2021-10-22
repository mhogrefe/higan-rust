use higan_rust::ares::emulator::types::{U11, U2, U4, U5};
use higan_rust::ares::gb::apu::wave::Wave;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::WrappingFrom;

fn power_and_zero_pattern(wave: &mut Wave) {
    wave.power(true);
    for p in wave.pattern.iter_mut() {
        *p = 0;
    }
}

#[test]
fn test_get_pattern() {
    let mut wave = Wave::default();

    power_and_zero_pattern(&mut wave);
    wave.pattern[0] = 0x12;
    wave.pattern[1] = 0xab;
    assert_eq!(
        wave.get_pattern(U5::wrapping_from(0)),
        U4::wrapping_from(0x1)
    );
    assert_eq!(
        wave.get_pattern(U5::wrapping_from(1)),
        U4::wrapping_from(0x2)
    );
    assert_eq!(
        wave.get_pattern(U5::wrapping_from(2)),
        U4::wrapping_from(0xa)
    );
    assert_eq!(
        wave.get_pattern(U5::wrapping_from(3)),
        U4::wrapping_from(0xb)
    );
    assert_eq!(wave.get_pattern(U5::wrapping_from(4)), U4::wrapping_from(0));
}

fn run_helper(wave: &mut Wave, cycles: u32, expected_output: &[i16]) {
    let mut output = Vec::new();
    for _ in 0..cycles {
        wave.run();
        output.push(wave.output);
    }
    assert_eq!(output, expected_output);
}

#[test]
fn test_run() {
    let mut wave = Wave::default();

    power_and_zero_pattern(&mut wave);
    wave.pattern_hold = 5;
    wave.period = 1;
    wave.frequency = U11::wrapping_from(2_047);
    wave.pattern[0] = 0x12;
    wave.pattern[1] = 0xab;
    wave.enable = false;
    wave.run();
    assert_eq!(wave.output, 0);
    assert_eq!(wave.pattern_hold, 1);

    power_and_zero_pattern(&mut wave);
    wave.pattern_hold = 5;
    wave.period = 5;
    wave.frequency = U11::wrapping_from(2_047);
    wave.pattern[0] = 0x12;
    wave.pattern[1] = 0xab;
    wave.enable = false;
    wave.run();
    assert_eq!(wave.output, 0);
    assert_eq!(wave.pattern_hold, 4);

    power_and_zero_pattern(&mut wave);
    wave.pattern_hold = 0;
    wave.period = 5;
    wave.frequency = U11::wrapping_from(2_047);
    wave.pattern[0] = 0x12;
    wave.pattern[1] = 0xab;
    wave.enable = false;
    wave.run();
    assert_eq!(wave.output, 0);
    assert_eq!(wave.pattern_hold, 0);

    power_and_zero_pattern(&mut wave);
    wave.period = 1;
    wave.frequency = U11::wrapping_from(2_047);
    wave.pattern[0] = 0x12;
    wave.pattern[1] = 0xab;
    wave.volume = U2::ZERO;
    wave.enable = true;
    wave.run();
    assert_eq!(wave.output, 0);

    power_and_zero_pattern(&mut wave);
    wave.period = 1;
    wave.frequency = U11::wrapping_from(2_047);
    wave.pattern[0] = 0x12;
    wave.pattern[1] = 0xab;
    wave.volume = U2::ONE;
    wave.enable = true;
    run_helper(
        &mut wave,
        64,
        &[
            2, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 2, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 1,
        ],
    );

    power_and_zero_pattern(&mut wave);
    wave.period = 1;
    wave.frequency = U11::wrapping_from(2_047);
    wave.pattern[0] = 0x12;
    wave.pattern[1] = 0xab;
    wave.volume = U2::wrapping_from(2);
    wave.enable = true;
    run_helper(
        &mut wave,
        64,
        &[
            1, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ],
    );

    power_and_zero_pattern(&mut wave);
    wave.period = 1;
    wave.frequency = U11::wrapping_from(2_046);
    wave.pattern[0] = 0x12;
    wave.pattern[1] = 0xab;
    wave.volume = U2::ONE;
    wave.enable = true;
    run_helper(
        &mut wave,
        64,
        &[
            2, 2, 10, 10, 11, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 1, 1,
        ],
    );
}

#[test]
fn test_clock_length() {
    let mut wave = Wave::default();

    // counter is false
    power_and_zero_pattern(&mut wave);
    wave.counter = false;
    wave.enable = true;
    wave.length = 5;
    wave.clock_length();
    assert_eq!(wave.length, 5);
    assert!(wave.enable);

    power_and_zero_pattern(&mut wave);
    wave.counter = true;
    wave.enable = true;
    wave.length = 5;
    wave.clock_length();
    assert_eq!(wave.length, 4);
    assert!(wave.enable);

    // length is initially 0
    power_and_zero_pattern(&mut wave);
    wave.counter = true;
    wave.enable = true;
    wave.length = 0;
    wave.clock_length();
    assert_eq!(wave.length, 0);
    assert!(wave.enable);

    // length is initially 1
    power_and_zero_pattern(&mut wave);
    wave.counter = true;
    wave.enable = true;
    wave.length = 1;
    wave.clock_length();
    assert_eq!(wave.length, 0);
    assert!(!wave.enable);
}

/*
#[test]
fn test_read() {
    let mut wave = Wave::default();
    let mut system = System::default();

    power_and_zero_pattern(&mut wave);
    assert_eq!(wave.read(system.model_is_game_boy_color(), 0), 0xff);

    power_and_zero_pattern(&mut wave);
    wave.dac_enable = false;
    assert_eq!(
        wave.read(system.model_is_game_boy_color(), 0xff1a),
        0b01111111
    );

    power_and_zero_pattern(&mut wave);
    assert_eq!(
        wave.read(system.model_is_game_boy_color(), 0xff1b),
        0b11111111
    );

    power_and_zero_pattern(&mut wave);
    wave.volume = U2::wrapping_from(0b10);
    assert_eq!(
        wave.read(system.model_is_game_boy_color(), 0xff1c),
        0b11011111
    );

    power_and_zero_pattern(&mut wave);
    assert_eq!(
        wave.read(system.model_is_game_boy_color(), 0xff1d),
        0b11111111
    );

    power_and_zero_pattern(&mut wave);
    wave.counter = false;
    assert_eq!(
        wave.read(system.model_is_game_boy_color(), 0xff1e),
        0b10111111
    );

    // Model::GameBoyColor() is false, pattern_hold is zero
    power_and_zero_pattern(&mut wave);
    wave.enable = true;
    wave.pattern_hold = 0;
    wave.pattern_offset = U5::wrapping_from(3);
    wave.pattern[1] = 0xab;
    assert_eq!(wave.read(system.model_is_game_boy_color(), 0xff3a), 0xff);

    // Model::GameBoyColor() is false, pattern_hold is nonzero
    power_and_zero_pattern(&mut wave);
    wave.enable = true;
    wave.pattern_hold = 5;
    wave.pattern_offset = U5::wrapping_from(3);
    wave.pattern[1] = 0xab;
    assert_eq!(wave.read(system.model_is_game_boy_color(), 0xff3a), 0xab);

    // Model::GameBoyColor() is true, pattern_hold is zero
    power_and_zero_pattern(&mut wave);
    let old_system = system.clone();
    system.model = Model::GameBoyColor;
    wave.enable = true;
    wave.pattern_hold = 0;
    wave.pattern_offset = U5::wrapping_from(3);
    wave.pattern[1] = 0xab;
    assert_eq!(wave.read(system.model_is_game_boy_color(), 0xff3a), 0xab);
    system = old_system;

    // enable is false
    power_and_zero_pattern(&mut wave);
    wave.enable = false;
    wave.pattern_hold = 0;
    wave.pattern_offset = U5::wrapping_from(3);
    wave.pattern[5] = 0xab;
    assert_eq!(wave.read(system.model_is_game_boy_color(), 0xff35), 0xab);
}

#[test]
fn test_write() {
    let mut wave = Wave::default();
    let mut bus = Bus::default();
    let mut system = System::default();

    power_and_zero_pattern(&mut wave);
    wave.dac_enable = false;
    wave.write(
        system.model_is_game_boy_color(),
        bus.apu.phase,
        0xff1a,
        0b10000000,
    );
    assert!(wave.dac_enable);

    power_and_zero_pattern(&mut wave);
    wave.dac_enable = true;
    wave.enable = true;
    wave.write(system.model_is_game_boy_color(), bus.apu.phase, 0xff1a, 0);
    assert!(!wave.dac_enable);
    assert!(!wave.enable);

    power_and_zero_pattern(&mut wave);
    wave.write(system.model_is_game_boy_color(), bus.apu.phase, 0xff1b, 100);
    assert_eq!(wave.length, 156);

    power_and_zero_pattern(&mut wave);
    wave.write(
        system.model_is_game_boy_color(),
        bus.apu.phase,
        0xff1c,
        0b01000000,
    );
    assert_eq!(wave.volume, U2::wrapping_from(0b10));

    power_and_zero_pattern(&mut wave);
    wave.write(
        system.model_is_game_boy_color(),
        bus.apu.phase,
        0xff1d,
        0b10101010,
    );
    assert_eq!(wave.frequency, U11::wrapping_from(0b00010101010));

    // apu.phase.bit(0) is false so enable stays true
    power_and_zero_pattern(&mut wave);
    wave.enable = true;
    wave.length = 1;
    wave.write(
        system.model_is_game_boy_color(),
        bus.apu.phase,
        0xff1e,
        0b01000101,
    );
    assert!(wave.enable);
    assert_eq!(wave.length, 1);
    assert!(wave.counter);
    assert_eq!(wave.frequency, U11::wrapping_from(0b10100000000));

    // apu.phase.bit(0) is true so enable becomes false
    bus.power_apu();
    power_and_zero_pattern(&mut wave);
    bus.apu.phase = U3::ONE;
    power_and_zero_pattern(&mut wave);
    wave.enable = true;
    wave.length = 1;
    wave.write(
        system.model_is_game_boy_color(),
        bus.apu.phase,
        0xff1e,
        0b01000000,
    );
    assert!(!wave.enable);
    assert_eq!(wave.length, 0);
    // clear phase
    bus.power_apu();

    // pattern[0] corrupted
    power_and_zero_pattern(&mut wave);
    for i in 0..16 {
        wave.pattern[i] = i as u8;
    }
    wave.pattern_hold = 5;
    wave.pattern_offset = U5::wrapping_from(2);
    wave.write(
        system.model_is_game_boy_color(),
        bus.apu.phase,
        0xff1e,
        0b11000101,
    );
    assert_eq!(wave.pattern[0], 1);
    assert_eq!(wave.pattern[1], 1);
    assert_eq!(wave.pattern[2], 2);
    assert_eq!(wave.pattern[3], 3);
    assert_eq!(wave.pattern[4], 4);

    // pattern[0-3] corrupted
    power_and_zero_pattern(&mut wave);
    for i in 0..16 {
        wave.pattern[i] = i as u8;
    }
    wave.pattern_hold = 5;
    wave.pattern_offset = U5::wrapping_from(9);
    wave.write(
        system.model_is_game_boy_color(),
        bus.apu.phase,
        0xff1e,
        0b11000101,
    );
    assert_eq!(wave.pattern[0], 4);
    assert_eq!(wave.pattern[1], 5);
    assert_eq!(wave.pattern[2], 6);
    assert_eq!(wave.pattern[3], 7);
    assert_eq!(wave.pattern[4], 4);

    // no corruption when system is Game Boy Color
    power_and_zero_pattern(&mut wave);
    let old_system = system.clone();
    system.model = Model::GameBoyColor;
    for i in 0..16 {
        wave.pattern[i] = i as u8;
    }
    wave.pattern_hold = 5;
    wave.pattern_offset = U5::wrapping_from(9);
    wave.write(
        system.model_is_game_boy_color(),
        bus.apu.phase,
        0xff1e,
        0b11000101,
    );
    assert_eq!(wave.pattern[0], 0);
    assert_eq!(wave.pattern[1], 1);
    assert_eq!(wave.pattern[2], 2);
    assert_eq!(wave.pattern[3], 3);
    assert_eq!(wave.pattern[4], 4);
    system = old_system;

    // no corruption when data.bit(7) is false
    power_and_zero_pattern(&mut wave);
    for i in 0..16 {
        wave.pattern[i] = i as u8;
    }
    wave.pattern_hold = 5;
    wave.pattern_offset = U5::wrapping_from(9);
    wave.write(
        system.model_is_game_boy_color(),
        bus.apu.phase,
        0xff1e,
        0b01000101,
    );
    assert_eq!(wave.pattern[0], 0);
    assert_eq!(wave.pattern[1], 1);
    assert_eq!(wave.pattern[2], 2);
    assert_eq!(wave.pattern[3], 3);
    assert_eq!(wave.pattern[4], 4);

    power_and_zero_pattern(&mut wave);
    wave.pattern_offset = U5::wrapping_from(9);
    wave.frequency = U11::ONE;
    wave.pattern_sample = U4::ONE;
    wave.pattern_hold = 5;
    wave.dac_enable = true;
    wave.write(
        system.model_is_game_boy_color(),
        bus.apu.phase,
        0xff1e,
        0b11000000,
    );
    assert!(wave.enable);
    assert_eq!(wave.period, 2047);
    assert_eq!(wave.pattern_offset, U5::ZERO);
    assert_eq!(wave.pattern_sample, U4::ZERO);
    assert_eq!(wave.pattern_hold, 0);

    power_and_zero_pattern(&mut wave);
    wave.write(
        system.model_is_game_boy_color(),
        bus.apu.phase,
        0xff1e,
        0b11000000,
    );
    assert_eq!(wave.length, 256);

    power_and_zero_pattern(&mut wave);
    wave.length = 100;
    wave.write(
        system.model_is_game_boy_color(),
        bus.apu.phase,
        0xff1e,
        0b11000000,
    );
    assert_eq!(wave.length, 100);

    bus.power_apu();
    power_and_zero_pattern(&mut wave);
    bus.apu.phase = U3::ONE;
    wave.length = 100;
    wave.write(
        system.model_is_game_boy_color(),
        bus.apu.phase,
        0xff1e,
        0b11000000,
    );
    assert_eq!(wave.length, 99);
    // clear phase
    bus.power_apu();

    power_and_zero_pattern(&mut wave);
    wave.write(system.model_is_game_boy_color(), bus.apu.phase, 0xff3a, 123);
    assert_eq!(wave.pattern[0xa], 123);

    power_and_zero_pattern(&mut wave);
    wave.pattern_offset = U5::wrapping_from(5);
    wave.enable = true;
    wave.pattern_hold = 5;
    wave.write(system.model_is_game_boy_color(), bus.apu.phase, 0xff3a, 123);
    assert_eq!(wave.pattern[2], 123);

    bus.power_apu();
    power_and_zero_pattern(&mut wave);
    bus.apu.phase = U3::ONE;
    wave.pattern_offset = U5::wrapping_from(5);
    wave.enable = true;
    wave.write(system.model_is_game_boy_color(), bus.apu.phase, 0xff3a, 123);
    assert_eq!(wave.pattern[2], 0);
    // clear phase
    bus.power_apu();
}*/

#[test]
fn test_power() {
    let mut wave = Wave::default();
    wave.length = 0;
    power_and_zero_pattern(&mut wave);
    assert_eq!(wave.length, 256);

    wave.length = 0;
    wave.power(false);
    assert_eq!(wave.length, 0);
}
