use higan_rust::ares::emulator::types::U3;
use higan_rust::ares::gb::apu::sequencer::Sequencer;
use higan_rust::ares::gb::memory::memory::Bus;
use higan_rust::ares::gb::system::system::{Model, System};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::WrappingFrom;

#[test]
fn test_run() {
    let mut sequencer = Sequencer::default();

    sequencer.power();
    sequencer.center = 1;
    sequencer.left = 2;
    sequencer.right = 3;
    sequencer.run();
    assert_eq!(sequencer.center, 0);
    assert_eq!(sequencer.left, 0);
    assert_eq!(sequencer.right, 0);

    sequencer.power();
    sequencer.square_1.power(true);
    sequencer.square_2.power(true);
    sequencer.wave.power(true);
    sequencer.noise.power(true);
    sequencer.enable = true;
    sequencer.square_1.output = 15;
    sequencer.square_2.output = 15;
    sequencer.wave.output = 15;
    sequencer.noise.output = 15;
    sequencer.run();
    assert_eq!(sequencer.center, 7168);
    assert_eq!(sequencer.left, -1024);
    assert_eq!(sequencer.right, -1024);

    sequencer.power();
    sequencer.square_1.power(true);
    sequencer.square_2.power(true);
    sequencer.wave.power(true);
    sequencer.noise.power(true);
    sequencer.enable = true;
    sequencer.square_1.output = 15;
    sequencer.square_2.output = 15;
    sequencer.wave.output = 15;
    sequencer.noise.output = 15;
    sequencer.square_1_channel.left_enable = true;
    sequencer.square_2_channel.left_enable = true;
    sequencer.wave_channel.left_enable = true;
    sequencer.noise_channel.left_enable = true;
    sequencer.run();
    assert_eq!(sequencer.center, 7168);
    assert_eq!(sequencer.left, 896);
    assert_eq!(sequencer.right, -1024);

    sequencer.power();
    sequencer.square_1.power(true);
    sequencer.square_2.power(true);
    sequencer.wave.power(true);
    sequencer.noise.power(true);
    sequencer.enable = true;
    sequencer.square_1.output = 1;
    sequencer.square_2.output = 1;
    sequencer.wave.output = 1;
    sequencer.noise.output = 1;
    sequencer.square_1_channel.right_enable = true;
    sequencer.square_2_channel.right_enable = true;
    sequencer.wave_channel.right_enable = true;
    sequencer.noise_channel.right_enable = true;
    sequencer.run();
    assert_eq!(sequencer.center, -7168);
    assert_eq!(sequencer.left, -1024);
    assert_eq!(sequencer.right, -896);
}

#[test]
fn test_read() {
    let mut sequencer = Sequencer::default();

    sequencer.power();
    assert_eq!(sequencer.read(0), 0xff);

    sequencer.power();
    sequencer.left_enable = true;
    sequencer.left_volume = U3::wrapping_from(0b010);
    sequencer.right_enable = false;
    sequencer.right_volume = U3::wrapping_from(0b101);
    assert_eq!(sequencer.read(0xff24), 0b10100101);

    sequencer.power();
    sequencer.noise_channel.left_enable = true;
    sequencer.wave_channel.left_enable = false;
    sequencer.square_2_channel.left_enable = true;
    sequencer.square_1_channel.left_enable = false;
    sequencer.noise_channel.right_enable = false;
    sequencer.wave_channel.right_enable = true;
    sequencer.square_2_channel.right_enable = false;
    sequencer.square_1_channel.right_enable = true;
    assert_eq!(sequencer.read(0xff25), 0b10100101);

    sequencer.power();
    sequencer.enable = true;
    sequencer.noise.enable = false;
    sequencer.wave.enable = true;
    sequencer.square_2.enable = false;
    sequencer.square_1.enable = true;
    assert_eq!(sequencer.read(0xff26), 0b11110101);
}

#[test]
fn test_write() {
    let mut sequencer = Sequencer::default();
    let mut bus = Bus::default();
    let mut system = System::default();

    sequencer.power();
    sequencer.write(
        system.model_is_game_boy_color(),
        &mut bus.apu.phase,
        0xff24,
        0b10100101,
    );
    assert!(sequencer.left_enable);
    assert_eq!(sequencer.left_volume, U3::wrapping_from(0b010));
    assert!(!sequencer.right_enable);
    assert_eq!(sequencer.right_volume, U3::wrapping_from(0b101));

    sequencer.power();
    sequencer.write(
        system.model_is_game_boy_color(),
        &mut bus.apu.phase,
        0xff25,
        0b10100101,
    );
    assert!(sequencer.noise_channel.left_enable);
    assert!(!sequencer.wave_channel.left_enable);
    assert!(sequencer.square_2_channel.left_enable);
    assert!(!sequencer.square_1_channel.left_enable);
    assert!(!sequencer.noise_channel.right_enable);
    assert!(sequencer.wave_channel.right_enable);
    assert!(!sequencer.square_2_channel.right_enable);
    assert!(sequencer.square_1_channel.right_enable);

    // enable and data.bit(7) both false, so nothing happens
    sequencer.power();
    sequencer.square_1.power(true);
    sequencer.square_1.period = 5;
    sequencer.write(
        system.model_is_game_boy_color(),
        &mut bus.apu.phase,
        0xff26,
        0,
    );
    assert_eq!(sequencer.square_1.period, 5);
    sequencer.square_1.power(true);

    // enable and data.bit(7) both true, so nothing happens
    sequencer.power();
    sequencer.square_1.power(true);
    sequencer.square_1.period = 5;
    sequencer.enable = true;
    sequencer.write(
        system.model_is_game_boy_color(),
        &mut bus.apu.phase,
        0xff26,
        0b10000000,
    );
    assert_eq!(sequencer.square_1.period, 5);
    sequencer.square_1.power(true);

    // enable is false and data.bit(7) is true, so apu phase is set to 0
    bus.power_apu();
    sequencer.power();
    bus.apu.phase = U3::wrapping_from(5);
    sequencer.write(
        system.model_is_game_boy_color(),
        &mut bus.apu.phase,
        0xff26,
        0b10000000,
    );
    assert_eq!(bus.apu.phase, U3::ZERO);
    // clear phase
    bus.power_apu();

    // enable is true, data.bit(7) is false, and model is not GBC, so APU
    // components are powered without initializing length
    sequencer.power();
    sequencer.square_1.power(true);
    sequencer.square_1.period = 5;
    sequencer.square_1.length = 5;
    sequencer.enable = true;
    sequencer.write(
        system.model_is_game_boy_color(),
        &mut bus.apu.phase,
        0xff26,
        0,
    );
    assert_eq!(sequencer.square_1.period, 0);
    assert_eq!(sequencer.square_1.length, 5);
    sequencer.square_1.power(true);

    // enable is true, data.bit(7) is false, and model is GBC, so APU components
    // are powered, initializing length
    sequencer.power();
    system.model = Model::GameBoyColor;
    sequencer.square_1.power(true);
    sequencer.square_1.period = 5;
    sequencer.square_1.length = 5;
    sequencer.enable = true;
    sequencer.write(
        system.model_is_game_boy_color(),
        &mut bus.apu.phase,
        0xff26,
        0,
    );
    assert_eq!(sequencer.square_1.period, 0);
    assert_eq!(sequencer.square_1.length, 64);
    sequencer.square_1.power(true);
}

#[test]
fn test_power() {
    let mut sequencer = Sequencer::default();

    sequencer.left_volume = U3::wrapping_from(2);
    sequencer.power();
    assert_eq!(sequencer.left_volume, U3::ZERO);
}
