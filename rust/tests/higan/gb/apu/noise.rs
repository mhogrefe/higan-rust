use higan_rust::ares::emulator::types::{U15, U3, U4};
use higan_rust::ares::gb::apu::noise::Noise;
use higan_rust::ares::gb::memory::memory::Bus;
use malachite_base::comparison::traits::Max;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::WrappingFrom;

#[test]
fn test_dac_enable() {
    let mut noise = Noise::default();

    noise.power(true);
    noise.envelope_volume = U4::ZERO;
    noise.envelope_direction = false;
    assert_eq!(noise.dac_enable(), false);

    noise.envelope_volume = U4::wrapping_from(3);
    noise.envelope_direction = false;
    assert_eq!(noise.dac_enable(), true);

    noise.envelope_volume = U4::ZERO;
    noise.envelope_direction = true;
    assert_eq!(noise.dac_enable(), true);

    noise.envelope_volume = U4::wrapping_from(3);
    noise.envelope_direction = true;
    assert_eq!(noise.dac_enable(), true);
}

#[test]
fn test_get_period() {
    let mut noise = Noise::default();

    noise.power(true);
    noise.frequency = U4::wrapping_from(2);
    noise.divisor = U3::ZERO;
    assert_eq!(noise.get_period(), 16);

    noise.power(true);
    noise.frequency = U4::ONE;
    noise.divisor = U3::wrapping_from(5);
    assert_eq!(noise.get_period(), 80);
}

fn run_helper(noise: &mut Noise, cycles: u32, expected_output: &[i16]) {
    let mut output = Vec::new();
    for _ in 0..cycles {
        noise.run();
        output.push(noise.output);
    }
    assert_eq!(output, expected_output);
}

#[test]
fn test_run() {
    let mut noise = Noise::default();

    noise.power(true);
    noise.period = 0;
    noise.lfsr = U15::wrapping_from(2);
    noise.enable = false;
    noise.volume = U4::wrapping_from(10);
    noise.run();
    assert_eq!(noise.output, 0);

    noise.power(true);
    noise.period = 0;
    noise.lfsr = U15::wrapping_from(2);
    noise.enable = true;
    noise.volume = U4::wrapping_from(10);
    noise.run();
    assert_eq!(noise.output, 10);

    noise.power(true);
    noise.period = 1;
    noise.lfsr = U15::wrapping_from(0x1844e573);
    noise.enable = true;
    noise.volume = U4::wrapping_from(10);
    noise.divisor = U3::ZERO;
    noise.frequency = U4::ZERO;
    noise.narrow = false;
    run_helper(
        &mut noise,
        32,
        &[
            0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10,
            10, 10, 0, 0, 0, 0,
        ],
    );

    noise.power(true);
    noise.period = 1;
    noise.lfsr = U15::wrapping_from(0x1844e573);
    noise.enable = true;
    noise.volume = U4::wrapping_from(10);
    noise.divisor = U3::ZERO;
    noise.frequency = U4::ZERO;
    noise.narrow = true;
    run_helper(
        &mut noise,
        32,
        &[
            0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10,
            10, 10, 10, 10, 10, 10,
        ],
    );

    noise.power(true);
    noise.period = 1;
    noise.lfsr = U15::wrapping_from(0x1844e573);
    noise.enable = true;
    noise.volume = U4::wrapping_from(10);
    noise.divisor = U3::ONE;
    noise.frequency = U4::ZERO;
    noise.narrow = true;
    run_helper(
        &mut noise,
        32,
        &[
            0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            0, 0, 0, 0, 0, 0, 0, 0,
        ],
    );
}

#[test]
fn test_clock_length() {
    let mut noise = Noise::default();

    // counter is false
    noise.power(true);
    noise.counter = false;
    noise.enable = true;
    noise.length = 5;
    noise.clock_length();
    assert_eq!(noise.length, 5);
    assert!(noise.enable);

    noise.power(true);
    noise.counter = true;
    noise.enable = true;
    noise.length = 5;
    noise.clock_length();
    assert_eq!(noise.length, 4);
    assert!(noise.enable);

    // length is initially 0
    noise.power(true);
    noise.counter = true;
    noise.enable = true;
    noise.length = 0;
    noise.clock_length();
    assert_eq!(noise.length, 0);
    assert!(noise.enable);

    // length is initially 1
    noise.power(true);
    noise.counter = true;
    noise.enable = true;
    noise.length = 1;
    noise.clock_length();
    assert_eq!(noise.length, 0);
    assert!(!noise.enable);
}

#[test]
fn test_clock_envelope() {
    let mut noise = Noise::default();

    noise.power(true);
    noise.enable = false;
    noise.envelope_frequency = U3::wrapping_from(5);
    noise.envelope_period = U3::ONE;
    noise.volume = U4::wrapping_from(10);
    noise.clock_envelope();
    assert_eq!(noise.envelope_period, U3::ONE);
    assert_eq!(noise.volume, U4::wrapping_from(10));

    noise.power(true);
    noise.enable = true;
    noise.envelope_frequency = U3::ZERO;
    noise.envelope_period = U3::ONE;
    noise.volume = U4::wrapping_from(10);
    noise.clock_envelope();
    assert_eq!(noise.envelope_period, U3::ONE);
    assert_eq!(noise.volume, U4::wrapping_from(10));

    noise.power(true);
    noise.enable = true;
    noise.envelope_frequency = U3::wrapping_from(5);
    noise.envelope_period = U3::wrapping_from(5);
    noise.volume = U4::wrapping_from(10);
    noise.clock_envelope();
    assert_eq!(noise.envelope_period, U3::wrapping_from(4));
    assert_eq!(noise.volume, U4::wrapping_from(10));

    noise.power(true);
    noise.enable = true;
    noise.envelope_direction = false;
    noise.envelope_frequency = U3::wrapping_from(5);
    noise.envelope_period = U3::ONE;
    noise.volume = U4::wrapping_from(10);
    noise.clock_envelope();
    assert_eq!(noise.envelope_period, U3::wrapping_from(5));
    assert_eq!(noise.volume, U4::wrapping_from(9));

    // volume already at min
    noise.power(true);
    noise.enable = true;
    noise.envelope_direction = false;
    noise.envelope_frequency = U3::wrapping_from(5);
    noise.envelope_period = U3::ONE;
    noise.volume = U4::ZERO;
    noise.clock_envelope();
    assert_eq!(noise.envelope_period, U3::wrapping_from(5));
    assert_eq!(noise.volume, U4::ZERO);

    noise.power(true);
    noise.enable = true;
    noise.envelope_direction = true;
    noise.envelope_frequency = U3::wrapping_from(5);
    noise.envelope_period = U3::ONE;
    noise.volume = U4::wrapping_from(10);
    noise.clock_envelope();
    assert_eq!(noise.envelope_period, U3::wrapping_from(5));
    assert_eq!(noise.volume, U4::wrapping_from(11));

    // volume already at max
    noise.power(true);
    noise.enable = true;
    noise.envelope_direction = true;
    noise.envelope_frequency = U3::wrapping_from(5);
    noise.envelope_period = U3::ONE;
    noise.volume = U4::MAX;
    noise.clock_envelope();
    assert_eq!(noise.envelope_period, U3::wrapping_from(5));
    assert_eq!(noise.volume, U4::MAX);
}

#[test]
fn test_read() {
    let mut noise = Noise::default();

    noise.power(true);
    assert_eq!(noise.read(0), 0xff);

    noise.power(true);
    assert_eq!(noise.read(0xff1f), 0b11111111);

    noise.power(true);
    assert_eq!(noise.read(0xff20), 0b11111111);

    noise.power(true);
    noise.envelope_volume = U4::wrapping_from(0b1011);
    noise.envelope_direction = true;
    noise.envelope_frequency = U3::wrapping_from(0b010);
    assert_eq!(noise.read(0xff21), 0b10111010);

    noise.power(true);
    noise.frequency = U4::wrapping_from(0b1011);
    noise.narrow = true;
    noise.divisor = U3::wrapping_from(0b010);
    assert_eq!(noise.read(0xff22), 0b10111010);

    noise.power(true);
    noise.counter = false;
    assert_eq!(noise.read(0xff23), 0b10111111);
}

#[test]
fn test_write() {
    let mut bus = Bus::default();
    let mut noise = Noise::default();

    noise.power(true);
    noise.write(bus.apu.phase, 0xff20, 0b10110100);
    assert_eq!(noise.length, 12);

    noise.power(true);
    noise.enable = true;
    noise.write(bus.apu.phase, 0xff21, 0b10111010);
    assert_eq!(noise.envelope_volume, U4::wrapping_from(0b1011));
    assert!(noise.envelope_direction);
    assert_eq!(noise.envelope_frequency, U3::wrapping_from(0b010));
    assert!(noise.enable);

    noise.power(true);
    noise.enable = true;
    noise.write(bus.apu.phase, 0xff21, 0);
    assert_eq!(noise.envelope_volume, U4::ZERO);
    assert!(!noise.envelope_direction);
    assert_eq!(noise.envelope_frequency, U3::ZERO);
    assert!(!noise.enable);

    noise.power(true);
    noise.enable = true;
    noise.write(bus.apu.phase, 0xff22, 0b10111010);
    assert_eq!(noise.frequency, U4::wrapping_from(0b1011));
    assert!(noise.narrow);
    assert_eq!(noise.divisor, U3::wrapping_from(0b010));

    // data.bit(6) is false, data.bit(7) is true
    noise.power(true);
    noise.write(bus.apu.phase, 0xff23, 0b10110011);
    assert!(!noise.enable);
    assert!(!noise.counter);
    assert_eq!(noise.envelope_period, U3::ZERO);
    assert_eq!(noise.lfsr, U15::wrapping_from(0x7fff));
    assert_eq!(noise.volume, U4::ZERO);
    assert_eq!(noise.length, 64);

    // data.bit(6) is false, data.bit(7) is false. Length stays 0
    noise.power(true);
    noise.enable = true;
    noise.length = 0;
    noise.write(bus.apu.phase, 0xff23, 0b00110011);
    assert!(noise.enable);
    assert!(!noise.counter);
    assert_eq!(noise.length, 0);

    // data.bit(6) is true, data.bit(7) is true, enable stays true
    noise.power(true);
    noise.length = 1;
    noise.enable = true;
    noise.envelope_volume = U4::wrapping_from(5);
    noise.envelope_direction = true;
    noise.write(bus.apu.phase, 0xff23, 0b11110011);
    assert!(noise.enable);
    assert!(noise.counter);
    assert_eq!(noise.envelope_period, U3::ZERO);
    assert_eq!(noise.lfsr, U15::wrapping_from(0x7fff));
    assert_eq!(noise.volume, U4::wrapping_from(5));
    assert_eq!(noise.length, 1);

    // same as previous, but length is initially 0 and becomes 64
    noise.power(true);
    noise.enable = true;
    noise.envelope_volume = U4::wrapping_from(5);
    noise.length = 0;
    noise.envelope_direction = true;
    noise.write(bus.apu.phase, 0xff23, 0b11110011);
    assert!(noise.enable);
    assert!(noise.counter);
    assert_eq!(noise.envelope_period, U3::ZERO);
    assert_eq!(noise.lfsr, U15::wrapping_from(0x7fff));
    assert_eq!(noise.volume, U4::wrapping_from(5));
    assert_eq!(noise.length, 64);

    // same as previous, but length is initially 0 and becomes 63 because of
    // apu.phase
    bus.power_apu();
    noise.power(true);
    bus.apu.phase = U3::ONE;
    noise.enable = true;
    noise.envelope_volume = U4::wrapping_from(5);
    noise.length = 0;
    noise.envelope_direction = true;
    noise.write(bus.apu.phase, 0xff23, 0b11110011);
    assert!(noise.enable);
    assert!(noise.counter);
    assert_eq!(noise.envelope_period, U3::ZERO);
    assert_eq!(noise.lfsr, U15::wrapping_from(0x7fff));
    assert_eq!(noise.volume, U4::wrapping_from(5));
    assert_eq!(noise.length, 63);
    // clear phase
    bus.power_apu();

    // data.bit(6) is true, data.bit(7) is false, enable stays true
    noise.power(true);
    noise.length = 1;
    noise.enable = true;
    noise.write(bus.apu.phase, 0xff23, 0b01110011);
    assert!(noise.enable);
    assert!(noise.counter);
    assert_eq!(noise.length, 1);

    // same as previous, but apu.phase = 1, so enable becomes false
    bus.power_apu();
    noise.power(true);
    bus.apu.phase = U3::ONE;
    noise.length = 1;
    noise.enable = true;
    noise.write(bus.apu.phase, 0xff23, 0b01110011);

    assert!(!noise.enable);
    assert!(noise.counter);
    assert_eq!(noise.length, 0);
    // clear phase
    bus.power_apu();
}

#[test]
fn test_power() {
    let mut noise = Noise::default();
    noise.length = 0;
    noise.power(true);
    assert_eq!(noise.length, 64);

    noise.length = 0;
    noise.power(false);
    assert_eq!(noise.length, 0);
}
