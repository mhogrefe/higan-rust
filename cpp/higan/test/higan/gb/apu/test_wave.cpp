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
  EXPECT_EQ("Wave getPattern", wave.getPattern(0), (uint4)0x1);
  EXPECT_EQ("Wave getPattern", wave.getPattern(1), (uint4)0x2);
  EXPECT_EQ("Wave getPattern", wave.getPattern(2), (uint4)0xa);
  EXPECT_EQ("Wave getPattern", wave.getPattern(3), (uint4)0xb);
  EXPECT_EQ("Wave getPattern", wave.getPattern(4), (uint4)0);
}

void RunHelper(APU::Wave *wave, uint cycles,
               const std::string &expected_output) {
  std::vector<int16> output;
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
  EXPECT_EQ("Wave run", wave.output, (int16)0);
  EXPECT_EQ("Wave run", wave.patternHold, 1u);

  PowerAndZeroPattern(&wave);
  wave.patternHold = 5;
  wave.period = 5;
  wave.frequency = 2047;
  wave.pattern[0] = 0x12;
  wave.pattern[1] = 0xab;
  wave.enable = false;
  wave.run();
  EXPECT_EQ("Wave run", wave.output, (int16)0);
  EXPECT_EQ("Wave run", wave.patternHold, 4u);

  PowerAndZeroPattern(&wave);
  wave.patternHold = 0;
  wave.period = 5;
  wave.frequency = 2047;
  wave.pattern[0] = 0x12;
  wave.pattern[1] = 0xab;
  wave.enable = false;
  wave.run();
  EXPECT_EQ("Wave run", wave.output, (int16)0);
  EXPECT_EQ("Wave run", wave.patternHold, 0u);

  PowerAndZeroPattern(&wave);
  wave.period = 1;
  wave.frequency = 2047;
  wave.pattern[0] = 0x12;
  wave.pattern[1] = 0xab;
  wave.volume = 0;
  wave.enable = true;
  wave.run();
  EXPECT_EQ("Wave run", wave.output, (int16)0);

  PowerAndZeroPattern(&wave);
  wave.period = 1;
  wave.frequency = 2047;
  wave.pattern[0] = 0x12;
  wave.pattern[1] = 0xab;
  wave.volume = 1;
  wave.enable = true;
  RunHelper(&wave, 64, "[2, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "
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
  RunHelper(&wave, 64, "[1, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "
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
  RunHelper(&wave, 64, "[2, 2, 10, 10, 11, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "
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

void TestRead() {
  APU::Wave wave;

  PowerAndZeroPattern(&wave);
  wave.dacEnable = false;
  EXPECT_EQ("Wave read", wave.read(0xff1a), (uint8)0b01111111);

  PowerAndZeroPattern(&wave);
  EXPECT_EQ("Wave read", wave.read(0xff1b), (uint8)0b11111111);

  PowerAndZeroPattern(&wave);
  wave.volume = 0b10;
  EXPECT_EQ("Wave read", wave.read(0xff1c), (uint8)0b11011111);

  PowerAndZeroPattern(&wave);
  EXPECT_EQ("Wave read", wave.read(0xff1d), (uint8)0b11111111);

  PowerAndZeroPattern(&wave);
  wave.counter = false;
  EXPECT_EQ("Wave read", wave.read(0xff1e), (uint8)0b10111111);

  // Model::GameBoyColor() is false, patternHold is zero
  PowerAndZeroPattern(&wave);
  wave.enable = true;
  wave.patternHold = 0;
  wave.patternOffset = 3;
  wave.pattern[1] = 0xab;
  EXPECT_EQ("Wave read", wave.read(0xff3a), (uint8)0xff);

  // Model::GameBoyColor() is false, patternHold is nonzero
  PowerAndZeroPattern(&wave);
  wave.enable = true;
  wave.patternHold = 5;
  wave.patternOffset = 3;
  wave.pattern[1] = 0xab;
  EXPECT_EQ("Wave read", wave.read(0xff3a), (uint8)0xab);

  // Model::GameBoyColor() is true, patternHold is zero
  PowerAndZeroPattern(&wave);
  auto old_model = GameBoy::system._model;
  GameBoy::system._model = GameBoy::System::Model::GameBoyColor;
  wave.enable = true;
  wave.patternHold = 0;
  wave.patternOffset = 3;
  wave.pattern[1] = 0xab;
  EXPECT_EQ("Wave read", wave.read(0xff3a), (uint8)0xab);
  GameBoy::system._model = old_model;

  // enable is false
  PowerAndZeroPattern(&wave);
  wave.enable = false;
  wave.patternHold = 0;
  wave.patternOffset = 3;
  wave.pattern[5] = 0xab;
  EXPECT_EQ("Wave read", wave.read(0xff35), (uint8)0xab);
}

void TestAll() {
  TestGetPattern();
  TestRun();
  TestClockLength();
  TestRead();
}
}
