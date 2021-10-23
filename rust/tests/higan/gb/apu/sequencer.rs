use higan_rust::ares::emulator::types::U3;
use higan_rust::ares::gb::apu::sequencer::Sequencer;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::WrappingFrom;

#[test]
fn test_power() {
    let mut sequencer = Sequencer::default();

    sequencer.left_volume = U3::wrapping_from(2);
    sequencer.power();
    assert_eq!(sequencer.left_volume, U3::ZERO);
}
