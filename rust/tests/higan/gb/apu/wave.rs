use higan_rust::higan::emulator::types::{U11, U2, U4, U5};
use higan_rust::higan::gb::apu::wave::Wave;
use malachite_base::misc::WrappingFrom;
use malachite_base::num::{One, Zero};

#[test]
fn test_get_pattern() {
    let mut wave = Wave::default();

    wave.power(true);
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

    wave.power(true);
    wave.pattern_hold = 5;
    wave.period = 1;
    wave.frequency = U11::wrapping_from(2_047);
    wave.pattern[0] = 0x12;
    wave.pattern[1] = 0xab;
    wave.enable = false;
    wave.run();
    assert_eq!(wave.output, 0);
    assert_eq!(wave.pattern_hold, 1);

    wave.power(true);
    wave.pattern_hold = 5;
    wave.period = 5;
    wave.frequency = U11::wrapping_from(2_047);
    wave.pattern[0] = 0x12;
    wave.pattern[1] = 0xab;
    wave.enable = false;
    wave.run();
    assert_eq!(wave.output, 0);
    assert_eq!(wave.pattern_hold, 4);

    wave.power(true);
    wave.pattern_hold = 0;
    wave.period = 5;
    wave.frequency = U11::wrapping_from(2_047);
    wave.pattern[0] = 0x12;
    wave.pattern[1] = 0xab;
    wave.enable = false;
    wave.run();
    assert_eq!(wave.output, 0);
    assert_eq!(wave.pattern_hold, 0);

    wave.power(true);
    wave.period = 1;
    wave.frequency = U11::wrapping_from(2_047);
    wave.pattern[0] = 0x12;
    wave.pattern[1] = 0xab;
    wave.volume = U2::ZERO;
    wave.enable = true;
    wave.run();
    assert_eq!(wave.output, 0);

    wave.power(true);
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

    wave.power(true);
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

    wave.power(true);
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
    wave.power(true);
    wave.counter = false;
    wave.enable = true;
    wave.length = 5;
    wave.clock_length();
    assert_eq!(wave.length, 5);
    assert!(wave.enable);

    wave.power(true);
    wave.counter = true;
    wave.enable = true;
    wave.length = 5;
    wave.clock_length();
    assert_eq!(wave.length, 4);
    assert!(wave.enable);

    // length is initially 0
    wave.power(true);
    wave.counter = true;
    wave.enable = true;
    wave.length = 0;
    wave.clock_length();
    assert_eq!(wave.length, 0);
    assert!(wave.enable);

    // length is initially 1
    wave.power(true);
    wave.counter = true;
    wave.enable = true;
    wave.length = 1;
    wave.clock_length();
    assert_eq!(wave.length, 0);
    assert!(!wave.enable);
}
