use higan_rust::higan::emulator::types::U3;
use higan_rust::higan::gb::apu::sequencer::Sequencer;
use malachite_base::misc::WrappingFrom;

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
