use higan_rust::higan::emulator::types::U4;
use higan_rust::higan::gb::apu::noise::Noise;
use malachite_base::misc::WrappingFrom;
use malachite_base::num::Zero;

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
