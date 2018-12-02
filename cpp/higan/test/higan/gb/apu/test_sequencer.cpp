namespace sequencer {

void TestRun() {
  APU::Sequencer sequencer;

  sequencer.power();
  sequencer.center = 1;
  sequencer.left = 2;
  sequencer.right = 3;
  sequencer.run();
  EXPECT_EQ("Sequencer run", sequencer.center, (int16)0);
  EXPECT_EQ("Sequencer run", sequencer.left, (int16)0);
  EXPECT_EQ("Sequencer run", sequencer.right, (int16)0);

  sequencer.power();
  GameBoy::apu.square1.power(true);
  GameBoy::apu.square2.power(true);
  GameBoy::apu.wave.power(true);
  GameBoy::apu.noise.power(true);
  sequencer.enable = true;
  GameBoy::apu.square1.output = 15;
  GameBoy::apu.square2.output = 15;
  GameBoy::apu.wave.output = 15;
  GameBoy::apu.noise.output = 15;
  sequencer.run();
  EXPECT_EQ("Sequencer run", sequencer.center, (int16)7168);
  EXPECT_EQ("Sequencer run", sequencer.left, (int16)-1024);
  EXPECT_EQ("Sequencer run", sequencer.right, (int16)-1024);

  sequencer.power();
  GameBoy::apu.square1.power(true);
  GameBoy::apu.square2.power(true);
  GameBoy::apu.wave.power(true);
  GameBoy::apu.noise.power(true);
  sequencer.enable = true;
  GameBoy::apu.square1.output = 15;
  GameBoy::apu.square2.output = 15;
  GameBoy::apu.wave.output = 15;
  GameBoy::apu.noise.output = 15;
  sequencer.square1.leftEnable = true;
  sequencer.square2.leftEnable = true;
  sequencer.wave.leftEnable = true;
  sequencer.noise.leftEnable = true;
  sequencer.run();
  EXPECT_EQ("Sequencer run", sequencer.center, (int16)7168);
  EXPECT_EQ("Sequencer run", sequencer.left, (int16)896);
  EXPECT_EQ("Sequencer run", sequencer.right, (int16)-1024);

  sequencer.power();
  GameBoy::apu.square1.power(true);
  GameBoy::apu.square2.power(true);
  GameBoy::apu.wave.power(true);
  GameBoy::apu.noise.power(true);
  sequencer.enable = true;
  GameBoy::apu.square1.output = 1;
  GameBoy::apu.square2.output = 1;
  GameBoy::apu.wave.output = 1;
  GameBoy::apu.noise.output = 1;
  sequencer.square1.rightEnable = true;
  sequencer.square2.rightEnable = true;
  sequencer.wave.rightEnable = true;
  sequencer.noise.rightEnable = true;
  sequencer.run();
  EXPECT_EQ("Sequencer run", sequencer.center, (int16)-7168);
  EXPECT_EQ("Sequencer run", sequencer.left, (int16)-1024);
  EXPECT_EQ("Sequencer run", sequencer.right, (int16)-896);
}

void TestAll() { TestRun(); }
}
