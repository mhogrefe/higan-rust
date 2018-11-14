use higan_rust::higan::emulator::types::{U11, U2, U4};
use higan_rust::higan::gb::apu::square_1::Square1;
use malachite_base::misc::WrappingFrom;
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
