use higan_rust::ares::emulator::types::{U15, U3, U4};
use higan_rust::ares::gb::apu::noise::Noise;
use malachite_base::comparison::traits::Max;
use malachite_base::num::basic::traits::{One, Two, Zero};
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
fn test_trigger() {
    let mut noise = Noise::default();

    noise.power(true);
    noise.envelope_frequency = U3::TWO;
    noise.envelope_volume = U4::new(3);
    noise.length = 5;
    noise.trigger(U3::new(3));
    assert!(noise.enable);
    assert_eq!(noise.lfsr, U15::MAX);
    assert_eq!(noise.envelope_period, U3::TWO);
    assert_eq!(noise.volume, U4::new(3));
    assert_eq!(noise.length, 5);

    // length is 0, so gets set to 64
    noise.power(true);
    noise.envelope_frequency = U3::TWO;
    noise.envelope_volume = U4::new(3);
    noise.length = 0;
    noise.trigger(U3::new(3));
    assert!(noise.enable);
    assert_eq!(noise.lfsr, U15::MAX);
    assert_eq!(noise.envelope_period, U3::TWO);
    assert_eq!(noise.volume, U4::new(3));
    assert_eq!(noise.length, 64);

    // length is 0, so gets set to 64
    // counter is true, so length gets decremented to 63
    noise.power(true);
    noise.envelope_frequency = U3::TWO;
    noise.envelope_volume = U4::new(3);
    noise.length = 0;
    noise.counter = true;
    noise.trigger(U3::new(3));
    assert!(noise.enable);
    assert_eq!(noise.lfsr, U15::MAX);
    assert_eq!(noise.envelope_period, U3::TWO);
    assert_eq!(noise.volume, U4::new(3));
    assert_eq!(noise.length, 63);

    // length is 0, so gets set to 64
    // counter is true but phase is even, so length doesn't get decremented
    noise.power(true);
    noise.envelope_frequency = U3::TWO;
    noise.envelope_volume = U4::new(3);
    noise.length = 0;
    noise.counter = true;
    noise.trigger(U3::TWO);
    assert!(noise.enable);
    assert_eq!(noise.lfsr, U15::MAX);
    assert_eq!(noise.envelope_period, U3::TWO);
    assert_eq!(noise.volume, U4::new(3));
    assert_eq!(noise.length, 64);
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
