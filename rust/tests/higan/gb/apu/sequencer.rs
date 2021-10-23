use higan_rust::ares::emulator::types::U3;
use higan_rust::ares::gb::apu::apu::APU;
use higan_rust::ares::gb::apu::sequencer::Sequencer;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::WrappingFrom;

#[test]
fn test_run() {
    let mut apu = APU::default();

    apu.sequencer.power();
    apu.sequencer.center = 1;
    apu.sequencer.left = 2;
    apu.sequencer.right = 3;
    apu.run_sequencer();
    assert_eq!(apu.sequencer.center, 0);
    assert_eq!(apu.sequencer.left, 0);
    assert_eq!(apu.sequencer.right, 0);

    apu.sequencer.power();
    apu.square_1.power(true);
    apu.square_2.power(true);
    apu.wave.power(true);
    apu.noise.power(true);
    apu.sequencer.enable = true;
    apu.square_1.output = 15;
    apu.square_2.output = 15;
    apu.wave.output = 15;
    apu.noise.output = 15;
    apu.run_sequencer();
    assert_eq!(apu.sequencer.center, 7168);
    assert_eq!(apu.sequencer.left, -1024);
    assert_eq!(apu.sequencer.right, -1024);

    apu.sequencer.power();
    apu.square_1.power(true);
    apu.square_2.power(true);
    apu.wave.power(true);
    apu.noise.power(true);
    apu.sequencer.enable = true;
    apu.square_1.output = 15;
    apu.square_2.output = 15;
    apu.wave.output = 15;
    apu.noise.output = 15;
    apu.sequencer.square_1.left_enable = true;
    apu.sequencer.square_2.left_enable = true;
    apu.sequencer.wave.left_enable = true;
    apu.sequencer.noise.left_enable = true;
    apu.run_sequencer();
    assert_eq!(apu.sequencer.center, 7168);
    assert_eq!(apu.sequencer.left, 896);
    assert_eq!(apu.sequencer.right, -1024);

    apu.sequencer.power();
    apu.square_1.power(true);
    apu.square_2.power(true);
    apu.wave.power(true);
    apu.noise.power(true);
    apu.sequencer.enable = true;
    apu.square_1.output = 1;
    apu.square_2.output = 1;
    apu.wave.output = 1;
    apu.noise.output = 1;
    apu.sequencer.square_1.right_enable = true;
    apu.sequencer.square_2.right_enable = true;
    apu.sequencer.wave.right_enable = true;
    apu.sequencer.noise.right_enable = true;
    apu.run_sequencer();
    assert_eq!(apu.sequencer.center, -7168);
    assert_eq!(apu.sequencer.left, -1024);
    assert_eq!(apu.sequencer.right, -896);
}

/*
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
}*/

#[test]
fn test_power() {
    let mut sequencer = Sequencer::default();

    sequencer.left_volume = U3::wrapping_from(2);
    sequencer.power();
    assert_eq!(sequencer.left_volume, U3::ZERO);
}
