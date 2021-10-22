use higan_rust::ares::emulator::types::{U11, U2, U3, U4};
use higan_rust::ares::gb::apu::square_2::Square2;
use higan_rust::ares::gb::memory::memory::Bus;
use malachite_base::comparison::traits::Max;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::WrappingFrom;

#[test]
fn test_dac_enable() {
    let mut square_2 = Square2::default();
    square_2.power(true);

    square_2.power(true);
    square_2.envelope_volume = U4::ZERO;
    square_2.envelope_direction = false;
    assert_eq!(square_2.dac_enable(), false);

    square_2.power(true);
    square_2.envelope_volume = U4::wrapping_from(3);
    square_2.envelope_direction = false;
    assert_eq!(square_2.dac_enable(), true);

    square_2.power(true);
    square_2.envelope_volume = U4::ZERO;
    square_2.envelope_direction = true;
    assert_eq!(square_2.dac_enable(), true);

    square_2.power(true);
    square_2.envelope_volume = U4::wrapping_from(3);
    square_2.envelope_direction = true;
    assert_eq!(square_2.dac_enable(), true);
}

fn run_helper(square_2: &mut Square2, cycles: u32, expected_output: &[i16]) {
    let mut output = Vec::new();
    for _ in 0..cycles {
        square_2.run();
        output.push(square_2.output);
    }
    assert_eq!(output, expected_output);
}

#[test]
fn test_run() {
    let mut square_2 = Square2::default();

    square_2.power(true);
    square_2.period = 0;
    square_2.duty_output = false;
    square_2.enable = false;
    square_2.run();
    assert_eq!(square_2.output, 0);

    square_2.power(true);
    square_2.period = 0;
    square_2.duty_output = true;
    square_2.enable = false;
    square_2.volume = U4::wrapping_from(10);
    square_2.run();
    assert_eq!(square_2.output, 0);

    square_2.power(true);
    square_2.period = 0;
    square_2.duty_output = true;
    square_2.enable = true;
    square_2.volume = U4::wrapping_from(10);
    square_2.run();
    assert_eq!(square_2.output, 10);

    square_2.power(true);
    square_2.period = 30;
    square_2.duty_output = false;
    square_2.enable = false;
    U4::wrapping_from(10);
    square_2.run();
    assert_eq!(square_2.output, 0);

    square_2.power(true);
    square_2.period = 30;
    square_2.duty_output = true;
    square_2.enable = false;
    square_2.volume = U4::wrapping_from(10);
    square_2.run();
    assert_eq!(square_2.output, 0);

    square_2.power(true);
    square_2.period = 30;
    square_2.duty_output = true;
    square_2.enable = true;
    square_2.volume = U4::wrapping_from(10);
    square_2.run();
    assert_eq!(square_2.output, 10);

    square_2.power(true);
    square_2.frequency = U11::wrapping_from(2_047);
    square_2.duty = U2::ZERO;
    square_2.enable = true;
    square_2.volume = U4::wrapping_from(10);
    square_2.period = 1;
    run_helper(
        &mut square_2,
        32,
        &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10,
            0, 0, 0, 0,
        ],
    );

    square_2.power(true);
    square_2.frequency = U11::wrapping_from(2_047);
    square_2.duty = U2::ONE;
    square_2.enable = true;
    square_2.volume = U4::wrapping_from(10);
    square_2.period = 1;
    run_helper(
        &mut square_2,
        32,
        &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10,
            10, 10, 10, 0, 0,
        ],
    );

    square_2.power(true);
    square_2.frequency = U11::wrapping_from(2_047);
    square_2.duty = U2::wrapping_from(2);
    square_2.enable = true;
    square_2.volume = U4::wrapping_from(10);
    square_2.period = 1;
    run_helper(
        &mut square_2,
        32,
        &[
            0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10, 10, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10,
            10, 10, 10, 10, 10, 0, 0,
        ],
    );

    square_2.power(true);
    square_2.frequency = U11::wrapping_from(2_047);
    square_2.duty = U2::wrapping_from(3);
    square_2.enable = true;
    square_2.volume = U4::wrapping_from(10);
    square_2.period = 1;
    run_helper(
        &mut square_2,
        32,
        &[
            10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 0, 0, 0, 0, 10, 10,
        ],
    );

    square_2.power(true);
    square_2.frequency = U11::wrapping_from(2_046);
    square_2.duty = U2::ZERO;
    square_2.enable = true;
    square_2.volume = U4::wrapping_from(13);
    square_2.period = 1;
    run_helper(
        &mut square_2,
        32,
        &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 13, 13, 13, 0, 0, 0, 0,
            0, 0, 0, 0,
        ],
    );
}

#[test]
fn test_clock_length() {
    let mut square_2 = Square2::default();

    // counter is false
    square_2.power(true);
    square_2.counter = false;
    square_2.enable = true;
    square_2.length = 5;
    square_2.clock_length();
    assert_eq!(square_2.length, 5);
    assert!(square_2.enable);

    square_2.power(true);
    square_2.counter = true;
    square_2.enable = true;
    square_2.length = 5;
    square_2.clock_length();
    assert_eq!(square_2.length, 4);
    assert!(square_2.enable);

    // length is initially 0
    square_2.power(true);
    square_2.counter = true;
    square_2.enable = true;
    square_2.length = 0;
    square_2.clock_length();
    assert_eq!(square_2.length, 0);
    assert!(square_2.enable);

    // length is initially 1
    square_2.power(true);
    square_2.counter = true;
    square_2.enable = true;
    square_2.length = 1;
    square_2.clock_length();
    assert_eq!(square_2.length, 0);
    assert!(!square_2.enable);
}

#[test]
fn test_clock_envelope() {
    let mut square_2 = Square2::default();

    square_2.power(true);
    square_2.enable = false;
    square_2.envelope_frequency = U3::wrapping_from(5);
    square_2.envelope_period = U3::ONE;
    square_2.volume = U4::wrapping_from(10);
    square_2.clock_envelope();
    assert_eq!(square_2.envelope_period, U3::ONE);
    assert_eq!(square_2.volume, U4::wrapping_from(10));

    square_2.power(true);
    square_2.enable = true;
    square_2.envelope_frequency = U3::ZERO;
    square_2.envelope_period = U3::ONE;
    square_2.volume = U4::wrapping_from(10);
    square_2.clock_envelope();
    assert_eq!(square_2.envelope_period, U3::ONE);
    assert_eq!(square_2.volume, U4::wrapping_from(10));

    square_2.power(true);
    square_2.enable = true;
    square_2.envelope_frequency = U3::wrapping_from(5);
    square_2.envelope_period = U3::wrapping_from(5);
    square_2.volume = U4::wrapping_from(10);
    square_2.clock_envelope();
    assert_eq!(square_2.envelope_period, U3::wrapping_from(4));
    assert_eq!(square_2.volume, U4::wrapping_from(10));

    square_2.power(true);
    square_2.enable = true;
    square_2.envelope_direction = false;
    square_2.envelope_frequency = U3::wrapping_from(5);
    square_2.envelope_period = U3::ONE;
    square_2.volume = U4::wrapping_from(10);
    square_2.clock_envelope();
    assert_eq!(square_2.envelope_period, U3::wrapping_from(5));
    assert_eq!(square_2.volume, U4::wrapping_from(9));

    // volume already at min
    square_2.power(true);
    square_2.enable = true;
    square_2.envelope_direction = false;
    square_2.envelope_frequency = U3::wrapping_from(5);
    square_2.envelope_period = U3::ONE;
    square_2.volume = U4::ZERO;
    square_2.clock_envelope();
    assert_eq!(square_2.envelope_period, U3::wrapping_from(5));
    assert_eq!(square_2.volume, U4::ZERO);

    square_2.power(true);
    square_2.enable = true;
    square_2.envelope_direction = true;
    square_2.envelope_frequency = U3::wrapping_from(5);
    square_2.envelope_period = U3::ONE;
    square_2.volume = U4::wrapping_from(10);
    square_2.clock_envelope();
    assert_eq!(square_2.envelope_period, U3::wrapping_from(5));
    assert_eq!(square_2.volume, U4::wrapping_from(11));

    // volume already at max
    square_2.power(true);
    square_2.enable = true;
    square_2.envelope_direction = true;
    square_2.envelope_frequency = U3::wrapping_from(5);
    square_2.envelope_period = U3::ONE;
    square_2.volume = U4::MAX;
    square_2.clock_envelope();
    assert_eq!(square_2.envelope_period, U3::wrapping_from(5));
    assert_eq!(square_2.volume, U4::MAX);
}

#[test]
fn test_read() {
    let mut square_2 = Square2::default();

    square_2.power(true);
    assert_eq!(square_2.read(0), 0xff);

    square_2.power(true);
    assert_eq!(square_2.read(0xff15), 0b11111111);

    square_2.power(true);
    square_2.duty = U2::wrapping_from(0b01);
    assert_eq!(square_2.read(0xff16), 0b01111111);

    square_2.power(true);
    square_2.envelope_volume = U4::wrapping_from(0b1011);
    square_2.envelope_direction = true;
    square_2.envelope_frequency = U3::wrapping_from(0b010);
    assert_eq!(square_2.read(0xff17), 0b10111010);

    square_2.power(true);
    assert_eq!(square_2.read(0xff18), 0b11111111);

    square_2.power(true);
    square_2.counter = false;
    assert_eq!(square_2.read(0xff19), 0b10111111);
}

#[test]
fn test_write() {
    let mut bus = Bus::default();
    let mut square_2 = Square2::default();

    square_2.power(true);
    square_2.write(bus.apu.phase, 0xff16, 0b01110010);
    assert_eq!(square_2.duty, U2::wrapping_from(0b01));
    assert_eq!(square_2.length, 14);

    square_2.power(true);
    square_2.enable = true;
    square_2.write(bus.apu.phase, 0xff17, 0b10111010);
    assert_eq!(square_2.envelope_volume, U4::wrapping_from(0b1011));
    assert!(square_2.envelope_direction);
    assert_eq!(square_2.envelope_frequency, U3::wrapping_from(0b010));
    assert!(square_2.enable);

    square_2.power(true);
    square_2.enable = true;
    square_2.write(bus.apu.phase, 0xff17, 0);
    assert_eq!(square_2.envelope_volume, U4::ZERO);
    assert!(!square_2.envelope_direction);
    assert_eq!(square_2.envelope_frequency, U3::ZERO);
    assert!(!square_2.enable);

    square_2.power(true);
    square_2.write(bus.apu.phase, 0xff18, 0b10110100);
    assert_eq!(square_2.frequency, U11::wrapping_from(0b10110100));

    // data.bit(6) is false, data.bit(7) is true
    square_2.power(true);
    square_2.write(bus.apu.phase, 0xff19, 0b10110011);
    assert!(!square_2.enable);
    assert!(!square_2.counter);
    assert_eq!(square_2.frequency, U11::wrapping_from(0b01100000000));
    assert_eq!(square_2.period, 2560);
    assert_eq!(square_2.envelope_period, U3::ZERO);
    assert_eq!(square_2.volume, U4::ZERO);
    assert_eq!(square_2.length, 64);

    // data.bit(6) is false, data.bit(7) is false. Length stays 0
    square_2.power(true);
    square_2.enable = true;
    square_2.length = 0;
    square_2.write(bus.apu.phase, 0xff19, 0b00110011);
    assert!(square_2.enable);
    assert!(!square_2.counter);
    assert_eq!(square_2.frequency, U11::wrapping_from(0b01100000000));
    assert_eq!(square_2.length, 0);

    // data.bit(6) is true, data.bit(7) is true, enable stays true
    square_2.power(true);
    square_2.length = 1;
    square_2.enable = true;
    square_2.envelope_volume = U4::wrapping_from(5);
    square_2.envelope_direction = true;
    square_2.write(bus.apu.phase, 0xff19, 0b11110011);
    assert!(square_2.enable);
    assert!(square_2.counter);
    assert_eq!(square_2.frequency, U11::wrapping_from(0b01100000000));
    assert_eq!(square_2.period, 2560);
    assert_eq!(square_2.envelope_period, U3::ZERO);
    assert_eq!(square_2.volume, U4::wrapping_from(5));
    assert_eq!(square_2.length, 1);

    // same as previous, but length is initially 0 and becomes 64
    square_2.power(true);
    square_2.enable = true;
    square_2.envelope_volume = U4::wrapping_from(5);
    square_2.length = 0;
    square_2.envelope_direction = true;
    square_2.write(bus.apu.phase, 0xff19, 0b11110011);
    assert!(square_2.enable);
    assert!(square_2.counter);
    assert_eq!(square_2.frequency, U11::wrapping_from(0b01100000000));
    assert_eq!(square_2.period, 2560);
    assert_eq!(square_2.envelope_period, U3::ZERO);
    assert_eq!(square_2.volume, U4::wrapping_from(5));
    assert_eq!(square_2.length, 64);

    // same as previous, but length is initially 0 and becomes 63 because of
    // apu.phase
    bus.power_apu();
    square_2.power(true);
    bus.apu.phase = U3::ONE;
    square_2.enable = true;
    square_2.envelope_volume = U4::wrapping_from(5);
    square_2.length = 0;
    square_2.envelope_direction = true;
    square_2.write(bus.apu.phase, 0xff19, 0b11110011);
    assert!(square_2.enable);
    assert!(square_2.counter);
    assert_eq!(square_2.frequency, U11::wrapping_from(0b01100000000));
    assert_eq!(square_2.period, 2560);
    assert_eq!(square_2.envelope_period, U3::ZERO);
    assert_eq!(square_2.volume, U4::wrapping_from(5));
    assert_eq!(square_2.length, 63);
    // clear phase
    bus.power_apu();

    // data.bit(6) is true, data.bit(7) is false, enable stays true
    square_2.power(true);
    square_2.length = 1;
    square_2.enable = true;
    square_2.write(bus.apu.phase, 0xff19, 0b01110011);
    assert!(square_2.enable);
    assert!(square_2.counter);
    assert_eq!(square_2.frequency, U11::wrapping_from(0b01100000000));
    assert_eq!(square_2.length, 1);

    // same as previous, but apu.phase = 1, so enable becomes false
    bus.power_apu();
    square_2.power(true);
    bus.apu.phase = U3::ONE;
    square_2.length = 1;
    square_2.enable = true;
    square_2.write(bus.apu.phase, 0xff19, 0b01110011);

    assert!(!square_2.enable);
    assert!(square_2.counter);
    assert_eq!(square_2.frequency, U11::wrapping_from(0b01100000000));
    assert_eq!(square_2.length, 0);
    // clear phase
    bus.power_apu();
}

#[test]
fn test_power() {
    let mut square_2 = Square2::default();
    square_2.length = 0;
    square_2.power(true);
    assert_eq!(square_2.length, 64);

    square_2.length = 0;
    square_2.power(false);
    assert_eq!(square_2.length, 0);
}
