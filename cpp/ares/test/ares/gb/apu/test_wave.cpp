namespace wave {

void PowerAndZeroPattern(APU::Wave *wave) {
  wave->power(true);
  for (int i = 0; i < 16; ++i) {
    wave->pattern[i] = 0;
  }
}

void TestGetPattern() {
  APU::Wave wave;

  PowerAndZeroPattern(&wave);
  wave.pattern[0] = 0x12;
  wave.pattern[1] = 0xab;
  EXPECT_EQ("Wave getPattern", wave.getPattern(0), (n4)0x1);
  EXPECT_EQ("Wave getPattern", wave.getPattern(1), (n4)0x2);
  EXPECT_EQ("Wave getPattern", wave.getPattern(2), (n4)0xa);
  EXPECT_EQ("Wave getPattern", wave.getPattern(3), (n4)0xb);
  EXPECT_EQ("Wave getPattern", wave.getPattern(4), (n4)0);
}

void RunHelper(APU::Wave *wave, uint cycles,
               const std::string &expected_output) {
  std::vector<i16> output;
  for (int i = 0; i < cycles; ++i) {
    wave->run();
    output.push_back(wave->output);
  }
  EXPECT_EQ("Wave run", ToString(output), expected_output);
}

void TestRun() {
  APU::Wave wave;

  PowerAndZeroPattern(&wave);
  wave.patternHold = 5;
  wave.period = 1;
  wave.frequency = 2047;
  wave.pattern[0] = 0x12;
  wave.pattern[1] = 0xab;
  wave.enable = false;
  wave.run();
  EXPECT_EQ("Wave run", wave.output, (i16)0);
  EXPECT_EQ("Wave run", wave.patternHold, 1u);

  PowerAndZeroPattern(&wave);
  wave.patternHold = 5;
  wave.period = 5;
  wave.frequency = 2047;
  wave.pattern[0] = 0x12;
  wave.pattern[1] = 0xab;
  wave.enable = false;
  wave.run();
  EXPECT_EQ("Wave run", wave.output, (i16)0);
  EXPECT_EQ("Wave run", wave.patternHold, 4u);

  PowerAndZeroPattern(&wave);
  wave.patternHold = 0;
  wave.period = 5;
  wave.frequency = 2047;
  wave.pattern[0] = 0x12;
  wave.pattern[1] = 0xab;
  wave.enable = false;
  wave.run();
  EXPECT_EQ("Wave run", wave.output, (i16)0);
  EXPECT_EQ("Wave run", wave.patternHold, 0u);

  PowerAndZeroPattern(&wave);
  wave.period = 1;
  wave.frequency = 2047;
  wave.pattern[0] = 0x12;
  wave.pattern[1] = 0xab;
  wave.volume = 0;
  wave.enable = true;
  wave.run();
  EXPECT_EQ("Wave run", wave.output, (i16)0);

  PowerAndZeroPattern(&wave);
  wave.period = 1;
  wave.frequency = 2047;
  wave.pattern[0] = 0x12;
  wave.pattern[1] = 0xab;
  wave.volume = 1;
  wave.enable = true;
  RunHelper(&wave, 64,
            "[2, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "
            "0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 10, "
            "11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "
            "0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]");

  PowerAndZeroPattern(&wave);
  wave.period = 1;
  wave.frequency = 2047;
  wave.pattern[0] = 0x12;
  wave.pattern[1] = 0xab;
  wave.volume = 2;
  wave.enable = true;
  RunHelper(&wave, 64,
            "[1, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "
            "0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 5, 5, 0, "
            "0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "
            "0, 0, 0, 0, 0, 0, 0, 0, 0, 0]");

  PowerAndZeroPattern(&wave);
  wave.period = 1;
  wave.frequency = 2046;
  wave.pattern[0] = 0x12;
  wave.pattern[1] = 0xab;
  wave.volume = 1;
  wave.enable = true;
  RunHelper(&wave, 64,
            "[2, 2, 10, 10, 11, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "
            "0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "
            "0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "
            "0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1]");
}

void TestClockLength() {
  APU::Wave wave;

  // counter is false
  PowerAndZeroPattern(&wave);
  wave.counter = false;
  wave.enable = true;
  wave.length = 5;
  wave.clockLength();
  EXPECT_EQ("Wave clockLength", wave.length, 5u);
  EXPECT_TRUE("Wave clockLength", wave.enable);

  PowerAndZeroPattern(&wave);
  wave.counter = true;
  wave.enable = true;
  wave.length = 5;
  wave.clockLength();
  EXPECT_EQ("Wave clockLength", wave.length, 4u);
  EXPECT_TRUE("Wave clockLength", wave.enable);

  // length is initially 0
  PowerAndZeroPattern(&wave);
  wave.counter = true;
  wave.enable = true;
  wave.length = 0;
  wave.clockLength();
  EXPECT_EQ("Wave clockLength", wave.length, 0u);
  EXPECT_TRUE("Wave clockLength", wave.enable);

  // length is initially 1
  PowerAndZeroPattern(&wave);
  wave.counter = true;
  wave.enable = true;
  wave.length = 1;
  wave.clockLength();
  EXPECT_EQ("Wave clockLength", wave.length, 0u);
  EXPECT_FALSE("Wave clockLength", wave.enable);
}

/*
void TestRead() {
  APU::Wave wave;

  PowerAndZeroPattern(&wave);
  EXPECT_EQ("Wave read", wave.read(0), (n8)0xff);

  PowerAndZeroPattern(&wave);
  wave.dacEnable = false;
  EXPECT_EQ("Wave read", wave.read(0xff1a), (n8)0b01111111);

  PowerAndZeroPattern(&wave);
  EXPECT_EQ("Wave read", wave.read(0xff1b), (n8)0b11111111);

  PowerAndZeroPattern(&wave);
  wave.volume = 0b10;
  EXPECT_EQ("Wave read", wave.read(0xff1c), (n8)0b11011111);

  PowerAndZeroPattern(&wave);
  EXPECT_EQ("Wave read", wave.read(0xff1d), (n8)0b11111111);

  PowerAndZeroPattern(&wave);
  wave.counter = false;
  EXPECT_EQ("Wave read", wave.read(0xff1e), (n8)0b10111111);

  // Model::GameBoyColor() is false, patternHold is zero
  PowerAndZeroPattern(&wave);
  wave.enable = true;
  wave.patternHold = 0;
  wave.patternOffset = 3;
  wave.pattern[1] = 0xab;
  EXPECT_EQ("Wave read", wave.read(0xff3a), (n8)0xff);

  // Model::GameBoyColor() is false, patternHold is nonzero
  PowerAndZeroPattern(&wave);
  wave.enable = true;
  wave.patternHold = 5;
  wave.patternOffset = 3;
  wave.pattern[1] = 0xab;
  EXPECT_EQ("Wave read", wave.read(0xff3a), (n8)0xab);

  // Model::GameBoyColor() is true, patternHold is zero
  PowerAndZeroPattern(&wave);
  auto old_model = GameBoy::system._model;
  GameBoy::system._model = GameBoy::System::Model::GameBoyColor;
  wave.enable = true;
  wave.patternHold = 0;
  wave.patternOffset = 3;
  wave.pattern[1] = 0xab;
  EXPECT_EQ("Wave read", wave.read(0xff3a), (n8)0xab);
  GameBoy::system._model = old_model;

  // enable is false
  PowerAndZeroPattern(&wave);
  wave.enable = false;
  wave.patternHold = 0;
  wave.patternOffset = 3;
  wave.pattern[5] = 0xab;
  EXPECT_EQ("Wave read", wave.read(0xff35), (n8)0xab);
}

void TestWrite() {
  APU::Wave wave;

  PowerAndZeroPattern(&wave);
  wave.dacEnable = false;
  wave.write(0xff1a, 0b10000000);
  EXPECT_TRUE("Wave write", wave.dacEnable);

  PowerAndZeroPattern(&wave);
  wave.dacEnable = true;
  wave.enable = true;
  wave.write(0xff1a, 0);
  EXPECT_FALSE("Wave write", wave.dacEnable);
  EXPECT_FALSE("Wave write", wave.enable);

  PowerAndZeroPattern(&wave);
  wave.write(0xff1b, 100);
  EXPECT_EQ("Wave write", wave.length, 156u);

  PowerAndZeroPattern(&wave);
  wave.write(0xff1c, 0b01000000);
  EXPECT_EQ("Wave write", wave.volume, (n2)0b10);

  PowerAndZeroPattern(&wave);
  wave.write(0xff1d, 0b10101010);
  EXPECT_EQ("Wave write", wave.frequency, (n11)0b00010101010);

  // apu.phase.bit(0) is false so enable stays true
  PowerAndZeroPattern(&wave);
  wave.enable = true;
  wave.length = 1;
  wave.write(0xff1e, 0b01000101);
  EXPECT_TRUE("Wave write", wave.enable);
  EXPECT_EQ("Wave write", wave.length, 1u);
  EXPECT_TRUE("Wave write", wave.counter);
  EXPECT_EQ("Wave write", wave.frequency, (n11)0b10100000000);

  // apu.phase.bit(0) is true so enable becomes false
  GameBoy::apu.power();
  PowerAndZeroPattern(&wave);
  GameBoy::apu.phase = 1;
  PowerAndZeroPattern(&wave);
  wave.enable = true;
  wave.length = 1;
  wave.write(0xff1e, 0b01000000);
  EXPECT_FALSE("Wave write", wave.enable);
  EXPECT_EQ("Wave write", wave.length, 0u);
  // clear phase
  GameBoy::apu.power();

  // pattern[0] corrupted
  PowerAndZeroPattern(&wave);
  for (int i = 0; i < 16; ++i) {
    wave.pattern[i] = i;
  }
  wave.patternHold = 5;
  wave.patternOffset = 2;
  wave.write(0xff1e, 0b11000101);
  EXPECT_EQ("Wave write", wave.pattern[0], (n8)1);
  EXPECT_EQ("Wave write", wave.pattern[1], (n8)1);
  EXPECT_EQ("Wave write", wave.pattern[2], (n8)2);
  EXPECT_EQ("Wave write", wave.pattern[3], (n8)3);
  EXPECT_EQ("Wave write", wave.pattern[4], (n8)4);

  // pattern[0-3] corrupted
  PowerAndZeroPattern(&wave);
  for (int i = 0; i < 16; ++i) {
    wave.pattern[i] = i;
  }
  wave.patternHold = 5;
  wave.patternOffset = 9;
  wave.write(0xff1e, 0b11000101);
  EXPECT_EQ("Wave write", wave.pattern[0], (n8)4);
  EXPECT_EQ("Wave write", wave.pattern[1], (n8)5);
  EXPECT_EQ("Wave write", wave.pattern[2], (n8)6);
  EXPECT_EQ("Wave write", wave.pattern[3], (n8)7);
  EXPECT_EQ("Wave write", wave.pattern[4], (n8)4);

  // no corruption when system is Game Boy Color
  PowerAndZeroPattern(&wave);
  auto old_model = GameBoy::system._model;
  GameBoy::system._model = GameBoy::System::Model::GameBoyColor;
  for (int i = 0; i < 16; ++i) {
    wave.pattern[i] = i;
  }
  wave.patternHold = 5;
  wave.patternOffset = 9;
  wave.write(0xff1e, 0b11000101);
  EXPECT_EQ("Wave write", wave.pattern[0], (n8)0);
  EXPECT_EQ("Wave write", wave.pattern[1], (n8)1);
  EXPECT_EQ("Wave write", wave.pattern[2], (n8)2);
  EXPECT_EQ("Wave write", wave.pattern[3], (n8)3);
  EXPECT_EQ("Wave write", wave.pattern[4], (n8)4);
  GameBoy::system._model = old_model;

  // no corruption when data.bit(7) is false
  PowerAndZeroPattern(&wave);
  for (int i = 0; i < 16; ++i) {
    wave.pattern[i] = i;
  }
  wave.patternHold = 5;
  wave.patternOffset = 9;
  wave.write(0xff1e, 0b01000101);
  EXPECT_EQ("Wave write", wave.pattern[0], (n8)0);
  EXPECT_EQ("Wave write", wave.pattern[1], (n8)1);
  EXPECT_EQ("Wave write", wave.pattern[2], (n8)2);
  EXPECT_EQ("Wave write", wave.pattern[3], (n8)3);
  EXPECT_EQ("Wave write", wave.pattern[4], (n8)4);

  PowerAndZeroPattern(&wave);
  wave.patternOffset = 9;
  wave.frequency = 1;
  wave.patternSample = 1;
  wave.patternHold = 5;
  wave.dacEnable = true;
  wave.write(0xff1e, 0b11000000);
  EXPECT_TRUE("Wave write", wave.enable);
  EXPECT_EQ("Wave write", wave.period, 2047u);
  EXPECT_EQ("Wave write", wave.patternOffset, (n5)0);
  EXPECT_EQ("Wave write", wave.patternSample, (n4)0);
  EXPECT_EQ("Wave write", wave.patternHold, 0u);

  PowerAndZeroPattern(&wave);
  wave.write(0xff1e, 0b11000000);
  EXPECT_EQ("Wave write", wave.length, 256u);

  PowerAndZeroPattern(&wave);
  wave.length = 100;
  wave.write(0xff1e, 0b11000000);
  EXPECT_EQ("Wave write", wave.length, 100u);

  GameBoy::apu.power();
  PowerAndZeroPattern(&wave);
  GameBoy::apu.phase = 1;
  wave.length = 100;
  wave.write(0xff1e, 0b11000000);
  EXPECT_EQ("Wave write", wave.length, 99u);
  // clear phase
  GameBoy::apu.power();

  PowerAndZeroPattern(&wave);
  wave.write(0xff3a, 123);
  EXPECT_EQ("Wave write", wave.pattern[0xa], (n8)123);

  PowerAndZeroPattern(&wave);
  wave.patternOffset = 5;
  wave.enable = true;
  wave.patternHold = 5;
  wave.write(0xff3a, 123);
  EXPECT_EQ("Wave write", wave.pattern[2], (n8)123);

  GameBoy::apu.power();
  PowerAndZeroPattern(&wave);
  GameBoy::apu.phase = 1;
  wave.patternOffset = 5;
  wave.enable = true;
  wave.write(0xff3a, 123);
  EXPECT_EQ("Wave write", wave.pattern[2], (n8)0);
  // clear phase
  GameBoy::apu.power();
}*/

void TestPower() {
  APU::Wave wave;
  wave.length = 0;
  wave.power(true);
  EXPECT_EQ("Wave power", wave.length, 256u);

  wave.length = 0;
  wave.power(false);
  EXPECT_EQ("Wave power", wave.length, 0u);
}

void TestAll() {
  TestGetPattern();
  TestRun();
  TestClockLength();
  // TestRead();
  // TestWrite();
  TestPower();
}
} // namespace wave
