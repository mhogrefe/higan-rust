namespace sequencer {

/*
void TestRead() {
  APU::Sequencer sequencer;

  sequencer.power();
  EXPECT_EQ("Sequencer read", sequencer.read(0), (n8)0xff);

  sequencer.power();
  sequencer.leftEnable = true;
  sequencer.leftVolume = (n3)0b010;
  sequencer.rightEnable = false;
  sequencer.rightVolume = (n3)0b101;
  EXPECT_EQ("Sequencer read", sequencer.read(0xff24), (n8)0b10100101);

  sequencer.power();
  sequencer.noise.leftEnable = true;
  sequencer.wave.leftEnable = false;
  sequencer.square2.leftEnable = true;
  sequencer.square1.leftEnable = false;
  sequencer.noise.rightEnable = false;
  sequencer.wave.rightEnable = true;
  sequencer.square2.rightEnable = false;
  sequencer.square1.rightEnable = true;
  EXPECT_EQ("Sequencer read", sequencer.read(0xff25), (n8)0b10100101);

  sequencer.power();
  sequencer.enable = true;
  GameBoy::apu.noise.enable = false;
  GameBoy::apu.wave.enable = true;
  GameBoy::apu.square2.enable = false;
  GameBoy::apu.square1.enable = true;
  EXPECT_EQ("Sequencer read", sequencer.read(0xff26), (n8)0b11110101);

  GameBoy::apu.square1.power(true);
  GameBoy::apu.square2.power(true);
  GameBoy::apu.wave.power(true);
  GameBoy::apu.noise.power(true);
}

void TestWrite() {
  APU::Sequencer sequencer;

  sequencer.power();
  sequencer.write(0xff24, 0b10100101);
  EXPECT_TRUE("Sequencer write", sequencer.leftEnable);
  EXPECT_EQ("Sequencer write", sequencer.leftVolume, (n3)0b010);
  EXPECT_FALSE("Sequencer write", sequencer.rightEnable);
  EXPECT_EQ("Sequencer write", sequencer.rightVolume, (n3)0b101);

  sequencer.power();
  sequencer.write(0xff25, 0b10100101);
  EXPECT_TRUE("Sequencer write", sequencer.noise.leftEnable);
  EXPECT_FALSE("Sequencer write", sequencer.wave.leftEnable);
  EXPECT_TRUE("Sequencer write", sequencer.square2.leftEnable);
  EXPECT_FALSE("Sequencer write", sequencer.square1.leftEnable);
  EXPECT_FALSE("Sequencer write", sequencer.noise.rightEnable);
  EXPECT_TRUE("Sequencer write", sequencer.wave.rightEnable);
  EXPECT_FALSE("Sequencer write", sequencer.square2.rightEnable);
  EXPECT_TRUE("Sequencer write", sequencer.square1.rightEnable);

  // enable and data.bit(7) both false, so nothing happens
  sequencer.power();
  GameBoy::apu.square1.power(true);
  GameBoy::apu.square1.period = 5;
  sequencer.write(0xff26, 0);
  EXPECT_EQ("Sequencer write", GameBoy::apu.square1.period, 5u);
  GameBoy::apu.square1.power(true);

  // enable and data.bit(7) both true, so nothing happens
  sequencer.power();
  GameBoy::apu.square1.power(true);
  GameBoy::apu.square1.period = 5;
  sequencer.enable = true;
  sequencer.write(0xff26, 0b10000000);
  EXPECT_EQ("Sequencer write", GameBoy::apu.square1.period, 5u);
  GameBoy::apu.square1.power(true);

  // enable is false and data.bit(7) is true, so apu phase is set to 0
  sequencer.power();
  GameBoy::apu.power();
  GameBoy::apu.phase = 5;
  sequencer.write(0xff26, 0b10000000);
  EXPECT_EQ("Sequencer write", GameBoy::apu.phase, (n3)0);
  GameBoy::apu.power();

  // enable is true, data.bit(7) is false, and model is not GBC, so APU
  // components are powered without initializing length
  sequencer.power();
  GameBoy::apu.square1.power(true);
  GameBoy::apu.square1.period = 5;
  GameBoy::apu.square1.length = 5;
  sequencer.enable = true;
  sequencer.write(0xff26, 0);
  EXPECT_EQ("Sequencer write", GameBoy::apu.square1.period, 0u);
  EXPECT_EQ("Sequencer write", GameBoy::apu.square1.length, 5u);
  GameBoy::apu.square1.power(true);

  // enable is true, data.bit(7) is false, and model is GBC, so APU components
  // are powered, initializing length
  sequencer.power();
  auto old_model = GameBoy::system._model;
  GameBoy::system._model = GameBoy::System::Model::GameBoyColor;
  GameBoy::apu.square1.power(true);
  GameBoy::apu.square1.period = 5;
  GameBoy::apu.square1.length = 5;
  sequencer.enable = true;
  sequencer.write(0xff26, 0);
  EXPECT_EQ("Sequencer write", GameBoy::apu.square1.period, 0u);
  EXPECT_EQ("Sequencer write", GameBoy::apu.square1.length, 64u);
  GameBoy::system._model = old_model;
  GameBoy::apu.square1.power(true);
}*/

void TestPower() {
  APU::Sequencer sequencer;

  sequencer.leftVolume = 2;
  sequencer.power();
  EXPECT_EQ("Sequencer power", sequencer.leftVolume, (n3)0);
}

void TestAll() {
  // TestRead();
  // TestWrite();
  TestPower();
}
} // namespace sequencer
