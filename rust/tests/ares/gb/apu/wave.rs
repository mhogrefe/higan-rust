use higan_rust::ares::emulator::types::{U11, U2, U3, U4, U5};
use higan_rust::ares::gb::apu::wave::Wave;
use malachite_base::num::basic::traits::{One, Two, Zero};
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

const INCREASING_PATTERN: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

#[test]
fn test_trigger() {
    let mut wave = Wave::default();

    power_and_zero_pattern(&mut wave);
    wave.pattern = INCREASING_PATTERN;
    wave.pattern_offset = U5::new(5);
    wave.length = 5;
    wave.trigger(false, U3::new(3));
    assert_eq!(wave.pattern, INCREASING_PATTERN);
    assert!(!wave.enable);
    assert_eq!(wave.period, 2050);
    assert_eq!(wave.pattern_offset, U5::ZERO);
    assert_eq!(wave.pattern_sample, U4::ZERO);
    assert_eq!(wave.pattern_hold, 0);
    assert_eq!(wave.length, 5);

    // length is 0, so it gets set to 256
    power_and_zero_pattern(&mut wave);
    wave.pattern = INCREASING_PATTERN;
    wave.pattern_offset = U5::new(5);
    wave.length = 0;
    wave.trigger(false, U3::new(3));
    assert_eq!(wave.pattern, INCREASING_PATTERN);
    assert!(!wave.enable);
    assert_eq!(wave.period, 2050);
    assert_eq!(wave.pattern_offset, U5::ZERO);
    assert_eq!(wave.pattern_sample, U4::ZERO);
    assert_eq!(wave.pattern_hold, 0);
    assert_eq!(wave.length, 256);

    // length is 0, so it gets set to 256
    // counter is true, so length gets decremented to 255
    power_and_zero_pattern(&mut wave);
    wave.pattern = INCREASING_PATTERN;
    wave.pattern_offset = U5::new(5);
    wave.length = 0;
    wave.counter = true;
    wave.trigger(false, U3::new(3));
    assert_eq!(wave.pattern, INCREASING_PATTERN);
    assert!(!wave.enable);
    assert_eq!(wave.period, 2050);
    assert_eq!(wave.pattern_offset, U5::ZERO);
    assert_eq!(wave.pattern_sample, U4::ZERO);
    assert_eq!(wave.pattern_hold, 0);
    assert_eq!(wave.length, 255);

    // length is 0, so it gets set to 256
    // counter is true but apu phase is even, so length does not get decremented to 255
    power_and_zero_pattern(&mut wave);
    wave.pattern = INCREASING_PATTERN;
    wave.length = 0;
    wave.counter = true;
    wave.trigger(false, U3::TWO);
    assert_eq!(wave.pattern, INCREASING_PATTERN);
    assert!(!wave.enable);
    assert_eq!(wave.period, 2050);
    assert_eq!(wave.pattern_offset, U5::ZERO);
    assert_eq!(wave.pattern_sample, U4::ZERO);
    assert_eq!(wave.pattern_hold, 0);
    assert_eq!(wave.length, 256);

    // Pattern corruption, case 1
    power_and_zero_pattern(&mut wave);
    wave.pattern = INCREASING_PATTERN;
    wave.pattern_offset = U5::new(5);
    wave.length = 5;
    wave.pattern_hold = 5;
    wave.trigger(false, U3::new(3));
    assert_eq!(
        wave.pattern,
        [3, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
    );
    assert!(!wave.enable);
    assert_eq!(wave.period, 2050);
    assert_eq!(wave.pattern_offset, U5::ZERO);
    assert_eq!(wave.pattern_sample, U4::ZERO);
    assert_eq!(wave.pattern_hold, 0);
    assert_eq!(wave.length, 5);

    // Pattern corruption, case 2
    power_and_zero_pattern(&mut wave);
    wave.pattern = INCREASING_PATTERN;
    wave.pattern_offset = U5::new(20);
    wave.length = 5;
    wave.pattern_hold = 5;
    wave.trigger(false, U3::new(3));
    assert_eq!(
        wave.pattern,
        [9, 10, 11, 12, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
    );
    assert!(!wave.enable);
    assert_eq!(wave.period, 2050);
    assert_eq!(wave.pattern_offset, U5::ZERO);
    assert_eq!(wave.pattern_sample, U4::ZERO);
    assert_eq!(wave.pattern_hold, 0);
    assert_eq!(wave.length, 5);
}

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
