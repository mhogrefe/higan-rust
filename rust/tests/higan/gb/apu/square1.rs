use higan_rust::higan::emulator::types::{U11, U2, U3, U4};
use higan_rust::higan::gb::apu::square_1::Square1;
use malachite_base::misc::{Max, WrappingFrom};
use malachite_base::num::{One, Zero};

#[test]
fn test_dac_enable() {
    let mut square_1 = Square1::default();
    square_1.power(true);

    square_1.envelope_volume = U4::ZERO;
    square_1.envelope_direction = false;
    assert_eq!(square_1.dac_enable(), false);

    square_1.envelope_volume = U4::wrapping_from(3);
    square_1.envelope_direction = false;
    assert_eq!(square_1.dac_enable(), true);

    square_1.envelope_volume = U4::ZERO;
    square_1.envelope_direction = true;
    assert_eq!(square_1.dac_enable(), true);

    square_1.envelope_volume = U4::wrapping_from(3);
    square_1.envelope_direction = true;
    assert_eq!(square_1.dac_enable(), true);
}

fn run_helper(square_1: &mut Square1, cycles: u32, expected_output: &[i16]) {
    let mut output = Vec::new();
    for _ in 0..cycles {
        square_1.run();
        output.push(square_1.output);
    }
    assert_eq!(output, expected_output);
}

#[test]
fn test_run() {
    let mut square_1 = Square1::default();

    square_1.power(true);
    square_1.period = 0;
    square_1.duty_output = false;
    square_1.enable = false;
    square_1.run();
    assert_eq!(square_1.output, 0);

    square_1.power(true);
    square_1.period = 0;
    square_1.duty_output = true;
    square_1.enable = false;
    square_1.volume = U4::wrapping_from(10);
    square_1.run();
    assert_eq!(square_1.output, 0);

    square_1.power(true);
    square_1.period = 0;
    square_1.duty_output = true;
    square_1.enable = true;
    square_1.volume = U4::wrapping_from(10);
    square_1.run();
    assert_eq!(square_1.output, 10);

    square_1.power(true);
    square_1.period = 30;
    square_1.duty_output = false;
    square_1.enable = false;
    U4::wrapping_from(10);
    square_1.run();
    assert_eq!(square_1.output, 0);

    square_1.power(true);
    square_1.period = 30;
    square_1.duty_output = true;
    square_1.enable = false;
    square_1.volume = U4::wrapping_from(10);
    square_1.run();
    assert_eq!(square_1.output, 0);

    square_1.power(true);
    square_1.period = 30;
    square_1.duty_output = true;
    square_1.enable = true;
    square_1.volume = U4::wrapping_from(10);
    square_1.run();
    assert_eq!(square_1.output, 10);

    square_1.power(true);
    square_1.frequency = U11::wrapping_from(2_047);
    square_1.duty = U2::ZERO;
    square_1.enable = true;
    square_1.volume = U4::wrapping_from(10);
    square_1.period = 1;
    run_helper(
        &mut square_1,
        32,
        &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10,
            0, 0, 0, 0,
        ],
    );

    square_1.power(true);
    square_1.frequency = U11::wrapping_from(2_047);
    square_1.duty = U2::ONE;
    square_1.enable = true;
    square_1.volume = U4::wrapping_from(10);
    square_1.period = 1;
    run_helper(
        &mut square_1,
        32,
        &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10,
            10, 10, 10, 0, 0,
        ],
    );

    square_1.power(true);
    square_1.frequency = U11::wrapping_from(2_047);
    square_1.duty = U2::wrapping_from(2);
    square_1.enable = true;
    square_1.volume = U4::wrapping_from(10);
    square_1.period = 1;
    run_helper(
        &mut square_1,
        32,
        &[
            0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10, 10, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10,
            10, 10, 10, 10, 10, 0, 0,
        ],
    );

    square_1.power(true);
    square_1.frequency = U11::wrapping_from(2_047);
    square_1.duty = U2::wrapping_from(3);
    square_1.enable = true;
    square_1.volume = U4::wrapping_from(10);
    square_1.period = 1;
    run_helper(
        &mut square_1,
        32,
        &[
            10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 0, 0, 0, 0, 10, 10,
        ],
    );

    square_1.power(true);
    square_1.frequency = U11::wrapping_from(2_046);
    square_1.duty = U2::ZERO;
    square_1.enable = true;
    square_1.volume = U4::wrapping_from(13);
    square_1.period = 1;
    run_helper(
        &mut square_1,
        32,
        &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 13, 13, 13, 0, 0, 0, 0,
            0, 0, 0, 0,
        ],
    );
}

#[test]
fn test_sweep() {
    let mut square_1 = Square1::default();

    // sweep_enable false
    square_1.power(true);
    square_1.enable = true;
    square_1.sweep_enable = false;
    square_1.frequency_shadow = 10;
    square_1.sweep_shift = U3::ONE;
    square_1.sweep(true);
    assert!(square_1.enable);
    assert_eq!(square_1.frequency_shadow, 10);
    assert_eq!(square_1.frequency, U11::ZERO);
    assert_eq!(square_1.period, 0);

    // positive delta
    square_1.power(true);
    square_1.enable = true;
    square_1.sweep_enable = true;
    square_1.frequency_shadow = 10;
    square_1.sweep_shift = U3::ONE;
    square_1.sweep(true);
    assert!(square_1.enable);
    assert_eq!(square_1.frequency_shadow, 15);
    assert_eq!(square_1.frequency, U11::wrapping_from(15));
    assert_eq!(square_1.period, 4_066);

    // freq exceeds 2,047
    square_1.power(true);
    square_1.enable = true;
    square_1.sweep_enable = true;
    square_1.frequency_shadow = 2_047;
    square_1.sweep_shift = U3::ONE;
    square_1.sweep(true);
    assert!(!square_1.enable);
    assert_eq!(square_1.frequency_shadow, 2_047);
    assert_eq!(square_1.frequency, U11::ZERO);
    assert_eq!(square_1.period, 0);

    // negative delta
    square_1.power(true);
    square_1.enable = true;
    square_1.sweep_direction = true;
    square_1.sweep_enable = true;
    square_1.frequency_shadow = 10;
    square_1.sweep_shift = U3::ONE;
    square_1.sweep(true);
    assert!(square_1.enable);
    assert_eq!(square_1.frequency_shadow, 5);
    assert_eq!(square_1.frequency, U11::wrapping_from(5));
    assert_eq!(square_1.period, 4_086);
}

#[test]
fn test_clock_length() {
    let mut square_1 = Square1::default();

    // counter is false
    square_1.power(true);
    square_1.counter = false;
    square_1.enable = true;
    square_1.length = 5;
    square_1.clock_length();
    assert_eq!(square_1.length, 5);
    assert!(square_1.enable);

    square_1.power(true);
    square_1.counter = true;
    square_1.enable = true;
    square_1.length = 5;
    square_1.clock_length();
    assert_eq!(square_1.length, 4);
    assert!(square_1.enable);

    // length is initially 0
    square_1.power(true);
    square_1.counter = true;
    square_1.enable = true;
    square_1.length = 0;
    square_1.clock_length();
    assert_eq!(square_1.length, 0);
    assert!(square_1.enable);

    // length is initially 1
    square_1.power(true);
    square_1.counter = true;
    square_1.enable = true;
    square_1.length = 1;
    square_1.clock_length();
    assert_eq!(square_1.length, 0);
    assert!(!square_1.enable);
}

#[test]
fn test_clock_sweep() {
    let mut square_1 = Square1::default();

    square_1.power(true);
    square_1.sweep_period = U3::wrapping_from(5);
    square_1.clock_sweep();
    assert_eq!(square_1.sweep_period, U3::wrapping_from(4));

    square_1.power(true);
    square_1.sweep_period = U3::ONE;
    square_1.sweep_frequency = U3::ZERO;
    square_1.clock_sweep();
    assert_eq!(square_1.sweep_period, U3::ZERO);

    square_1.power(true);
    square_1.enable = true;
    square_1.sweep_enable = true;
    square_1.sweep_period = U3::ONE;
    square_1.sweep_frequency = U3::wrapping_from(5);
    square_1.frequency_shadow = 10;
    square_1.sweep_shift = U3::ONE;
    square_1.clock_sweep();
    assert_eq!(square_1.sweep_period, U3::wrapping_from(5));
    assert!(square_1.enable);
    assert_eq!(square_1.frequency_shadow, 15);
    assert_eq!(square_1.frequency, U11::wrapping_from(15));
    assert_eq!(square_1.period, 4066);

    // sweep_enable is false
    square_1.power(true);
    square_1.enable = true;
    square_1.sweep_enable = false;
    square_1.sweep_period = U3::ONE;
    square_1.sweep_frequency = U3::wrapping_from(5);
    square_1.frequency_shadow = 10;
    square_1.sweep_shift = U3::ONE;
    square_1.clock_sweep();
    assert_eq!(square_1.sweep_period, U3::wrapping_from(5));
    assert!(square_1.enable);
    assert_eq!(square_1.frequency_shadow, 10);
    assert_eq!(square_1.frequency, U11::ZERO);
    assert_eq!(square_1.period, 0);
}

#[test]
fn test_clock_envelope() {
    let mut square_1 = Square1::default();

    square_1.power(true);
    square_1.enable = false;
    square_1.envelope_frequency = U3::wrapping_from(5);
    square_1.envelope_period = U3::ONE;
    square_1.volume = U4::wrapping_from(10);
    square_1.clock_envelope();
    assert_eq!(square_1.envelope_period, U3::ONE);
    assert_eq!(square_1.volume, U4::wrapping_from(10));

    square_1.power(true);
    square_1.enable = true;
    square_1.envelope_frequency = U3::ZERO;
    square_1.envelope_period = U3::ONE;
    square_1.volume = U4::wrapping_from(10);
    square_1.clock_envelope();
    assert_eq!(square_1.envelope_period, U3::ONE);
    assert_eq!(square_1.volume, U4::wrapping_from(10));

    square_1.power(true);
    square_1.enable = true;
    square_1.envelope_frequency = U3::wrapping_from(5);
    square_1.envelope_period = U3::wrapping_from(5);
    square_1.volume = U4::wrapping_from(10);
    square_1.clock_envelope();
    assert_eq!(square_1.envelope_period, U3::wrapping_from(4));
    assert_eq!(square_1.volume, U4::wrapping_from(10));

    square_1.power(true);
    square_1.enable = true;
    square_1.envelope_direction = false;
    square_1.envelope_frequency = U3::wrapping_from(5);
    square_1.envelope_period = U3::ONE;
    square_1.volume = U4::wrapping_from(10);
    square_1.clock_envelope();
    assert_eq!(square_1.envelope_period, U3::wrapping_from(5));
    assert_eq!(square_1.volume, U4::wrapping_from(9));

    // volume already at min
    square_1.power(true);
    square_1.enable = true;
    square_1.envelope_direction = false;
    square_1.envelope_frequency = U3::wrapping_from(5);
    square_1.envelope_period = U3::ONE;
    square_1.volume = U4::ZERO;
    square_1.clock_envelope();
    assert_eq!(square_1.envelope_period, U3::wrapping_from(5));
    assert_eq!(square_1.volume, U4::ZERO);

    square_1.power(true);
    square_1.enable = true;
    square_1.envelope_direction = true;
    square_1.envelope_frequency = U3::wrapping_from(5);
    square_1.envelope_period = U3::ONE;
    square_1.volume = U4::wrapping_from(10);
    square_1.clock_envelope();
    assert_eq!(square_1.envelope_period, U3::wrapping_from(5));
    assert_eq!(square_1.volume, U4::wrapping_from(11));

    // volume already at max
    square_1.power(true);
    square_1.enable = true;
    square_1.envelope_direction = true;
    square_1.envelope_frequency = U3::wrapping_from(5);
    square_1.envelope_period = U3::ONE;
    square_1.volume = U4::MAX;
    square_1.clock_envelope();
    assert_eq!(square_1.envelope_period, U3::wrapping_from(5));
    assert_eq!(square_1.volume, U4::MAX);
}

#[test]
fn test_read() {
    let mut square_1 = Square1::default();

    square_1.power(true);
    square_1.sweep_frequency = U3::wrapping_from(0b101);
    square_1.sweep_direction = true;
    square_1.sweep_shift = U3::wrapping_from(0b010);
    assert_eq!(square_1.read(0xff10), 0b11011010);

    square_1.power(true);
    square_1.duty = U2::wrapping_from(0b01);
    assert_eq!(square_1.read(0xff11), 0b01111111);

    square_1.power(true);
    square_1.envelope_volume = U4::wrapping_from(0b1011);
    square_1.envelope_direction = true;
    square_1.envelope_frequency = U3::wrapping_from(0b010);
    assert_eq!(square_1.read(0xff12), 0b10111010);

    square_1.power(true);
    assert_eq!(square_1.read(0xff13), 0b11111111);

    square_1.power(true);
    square_1.counter = false;
    assert_eq!(square_1.read(0xff14), 0b10111111);
}

#[test]
fn test_write() {
    let mut square_1 = Square1::default();

    square_1.power(true);
    square_1.enable = true;
    square_1.sweep_enable = true;
    square_1.sweep_negate = true;
    square_1.write(U3::ZERO, 0xff10, 0b11011010);
    assert_eq!(square_1.sweep_frequency, U3::wrapping_from(0b101));
    assert!(square_1.sweep_direction);
    assert_eq!(square_1.sweep_shift, U3::wrapping_from(0b010));
    assert!(square_1.enable);

    square_1.power(true);
    square_1.enable = true;
    square_1.sweep_enable = true;
    square_1.sweep_negate = true;
    square_1.write(U3::ZERO, 0xff10, 0b11010010);
    assert_eq!(square_1.sweep_frequency, U3::wrapping_from(0b101));
    assert!(!square_1.sweep_direction);
    assert_eq!(square_1.sweep_shift, U3::wrapping_from(0b010));
    assert!(!square_1.enable);

    square_1.power(true);
    square_1.write(U3::ZERO, 0xff11, 0b01110010);
    assert_eq!(square_1.duty, U2::wrapping_from(0b01));
    assert_eq!(square_1.length, 14);

    square_1.power(true);
    square_1.enable = true;
    square_1.write(U3::ZERO, 0xff12, 0b10111010);
    assert_eq!(square_1.envelope_volume, U4::wrapping_from(0b1011));
    assert!(square_1.envelope_direction);
    assert_eq!(square_1.envelope_frequency, U3::wrapping_from(0b010));
    assert!(square_1.enable);

    square_1.power(true);
    square_1.enable = true;
    square_1.write(U3::ZERO, 0xff12, 0);
    assert_eq!(square_1.envelope_volume, U4::ZERO);
    assert!(!square_1.envelope_direction);
    assert_eq!(square_1.envelope_frequency, U3::ZERO);
    assert!(!square_1.enable);

    square_1.power(true);
    square_1.write(U3::ZERO, 0xff13, 0b10110100);
    assert_eq!(square_1.frequency, U11::wrapping_from(0b10110100));

    square_1.power(true);
    square_1.write(U3::ZERO, 0xff14, 0b10110011);
    assert!(!square_1.enable);
    assert!(!square_1.counter);
    assert_eq!(square_1.frequency, U11::wrapping_from(0b01100000000));
    assert_eq!(square_1.period, 2560);
    assert_eq!(square_1.envelope_period, U3::ZERO);
    assert_eq!(square_1.volume, U4::ZERO);
    assert_eq!(square_1.length, 64);
    assert_eq!(square_1.frequency_shadow, 768);
    assert!(!square_1.sweep_negate);
    assert_eq!(square_1.sweep_period, U3::ZERO);
    assert!(!square_1.sweep_enable);

    square_1.power(true);
    square_1.enable = true;
    square_1.write(U3::ZERO, 0xff14, 0b00110011);
    assert!(square_1.enable);
    assert!(!square_1.counter);
    assert_eq!(square_1.frequency, U11::wrapping_from(0b01100000000));

    square_1.power(true);
    square_1.length = 1;
    square_1.enable = true;
    square_1.write(U3::ZERO, 0xff14, 0b11110011);
    assert!(!square_1.enable);
    assert!(square_1.counter);
    assert_eq!(square_1.frequency, U11::wrapping_from(0b01100000000));
    assert_eq!(square_1.period, 2560);
    assert_eq!(square_1.envelope_period, U3::ZERO);
    assert_eq!(square_1.volume, U4::ZERO);
    assert_eq!(square_1.length, 1);
    assert_eq!(square_1.frequency_shadow, 768);
    assert!(!square_1.sweep_negate);
    assert_eq!(square_1.sweep_period, U3::ZERO);
    assert!(!square_1.sweep_enable);
}

#[test]
fn test_power() {
    let mut square_1 = Square1::default();
    square_1.length = 0;
    square_1.power(true);
    assert_eq!(square_1.length, 64);

    square_1.length = 0;
    square_1.power(false);
    assert_eq!(square_1.length, 0);
}
