use higan_rust::higan::emulator::types::{U15, U3, U4};
use higan_rust::higan::gb::apu::noise::Noise;
use malachite_base::misc::WrappingFrom;
use malachite_base::num::{One, Zero};

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
