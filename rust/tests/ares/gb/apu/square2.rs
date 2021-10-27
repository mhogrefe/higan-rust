use higan_rust::ares::emulator::types::{U11, U2, U3, U4};
use higan_rust::ares::gb::apu::square_2::Square2;
use malachite_base::comparison::traits::Max;
use malachite_base::num::basic::traits::{One, Two, Zero};
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
fn test_trigger() {
    let mut square_2 = Square2::default();

    square_2.power(true);
    square_2.envelope_frequency = U3::new(3);
    square_2.envelope_volume = U4::TWO;
    square_2.length = 5;
    square_2.frequency = U11::new(100);
    square_2.trigger(U3::new(3));
    assert!(square_2.enable);
    assert_eq!(square_2.period, 3896);
    assert_eq!(square_2.envelope_period, U3::new(3));
    assert_eq!(square_2.volume, U4::TWO);
    assert_eq!(square_2.length, 5);

    // length is 0, so it gets set to 64
    square_2.power(true);
    square_2.envelope_frequency = U3::new(3);
    square_2.envelope_volume = U4::TWO;
    square_2.length = 0;
    square_2.frequency = U11::new(100);
    square_2.trigger(U3::new(3));
    assert!(square_2.enable);
    assert_eq!(square_2.period, 3896);
    assert_eq!(square_2.envelope_period, U3::new(3));
    assert_eq!(square_2.volume, U4::TWO);
    assert_eq!(square_2.length, 64);

    // length is 0, so it gets set to 64
    // counter is true, so length gets decremented to 63
    square_2.power(true);
    square_2.envelope_frequency = U3::new(3);
    square_2.envelope_volume = U4::TWO;
    square_2.length = 0;
    square_2.frequency = U11::new(100);
    square_2.counter = true;
    square_2.trigger(U3::new(3));
    assert!(square_2.enable);
    assert_eq!(square_2.period, 3896);
    assert_eq!(square_2.envelope_period, U3::new(3));
    assert_eq!(square_2.volume, U4::TWO);
    assert_eq!(square_2.length, 63);

    // length is 0, so it gets set to 64
    // counter is true but apu phase is even, so length doesn't get decremented to 63
    square_2.power(true);
    square_2.envelope_frequency = U3::new(3);
    square_2.envelope_volume = U4::TWO;
    square_2.length = 0;
    square_2.frequency = U11::new(100);
    square_2.counter = true;
    square_2.trigger(U3::TWO);
    assert!(square_2.enable);
    assert_eq!(square_2.period, 3896);
    assert_eq!(square_2.envelope_period, U3::new(3));
    assert_eq!(square_2.volume, U4::TWO);
    assert_eq!(square_2.length, 64);
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
