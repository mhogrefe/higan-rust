use higan_rust::higan::gb::apu::sequencer::Sequencer;

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
