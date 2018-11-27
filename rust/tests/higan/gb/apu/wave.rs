use higan_rust::higan::emulator::types::{U4, U5};
use higan_rust::higan::gb::apu::wave::Wave;
use malachite_base::misc::WrappingFrom;

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
