use higan_rust::ares::gb::apu::APU;

#[test]
fn test_run_sequencer() {
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
