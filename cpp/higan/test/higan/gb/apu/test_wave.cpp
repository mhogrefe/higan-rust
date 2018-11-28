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

void TestAll() {
  TestGetPattern();
  TestRun();
}
}
