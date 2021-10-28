namespace wave {

void PowerAndZeroPattern(APU::Wave *wave) {
  wave->power(true);
  for (int i = 0; i < 16; ++i) {
    wave->pattern[i] = 0;
  }
}

void SetPattern(APU::Wave *wave, const n8 pattern[16]) {
  for (int i = 0; i < 16; ++i) {
    wave->pattern[i] = pattern[i];
  }
}

void ExpectPatternEq(const std::string cause, APU::Wave *wave,
                     const n8 pattern[16]) {
  for (int i = 0; i < 16; ++i) {
    EXPECT_EQ(cause, wave->pattern[i], pattern[i]);
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

static const n8 INCREASING_PATTERN[16] = {1, 2,  3,  4,  5,  6,  7, 8,
                                          9, 10, 11, 12, 13, 14, 15};

static const n8 CORRUPTED_PATTERN_1[16] = {3, 2,  3,  4,  5,  6,  7, 8,
                                           9, 10, 11, 12, 13, 14, 15};

static const n8 CORRUPTED_PATTERN_2[16] = {9, 10, 11, 12, 5,  6,  7, 8,
                                           9, 10, 11, 12, 13, 14, 15};

void TestTrigger() {
  APU::Wave wave;

  PowerAndZeroPattern(&wave);
  SetPattern(&wave, INCREASING_PATTERN);
  wave.patternOffset = (n5)5;
  wave.length = 5;
  ::ares::GameBoy::apu.phase = 3;
  auto old_model = ::ares::GameBoy::system.information.model;
  ::ares::GameBoy::system.information.model =
      ::ares::GameBoy::System::Model::GameBoy;
  wave.trigger();
  ExpectPatternEq("Wave trigger", &wave, INCREASING_PATTERN);
  EXPECT_FALSE("Wave trigger", wave.enable);
  EXPECT_EQ("Wave trigger", wave.period, 2050u);
  EXPECT_EQ("Wave trigger", wave.patternOffset, (n5)0);
  EXPECT_EQ("Wave trigger", wave.patternSample, (n4)0);
  EXPECT_EQ("Wave trigger", wave.patternHold, 0u);
  EXPECT_EQ("Wave trigger", wave.length, 5u);

  // length is 0, so it gets set to 256
  PowerAndZeroPattern(&wave);
  SetPattern(&wave, INCREASING_PATTERN);
  wave.patternOffset = (n5)5;
  wave.length = 0;
  ::ares::GameBoy::apu.phase = 3;
  ::ares::GameBoy::system.information.model =
      ::ares::GameBoy::System::Model::GameBoy;
  wave.trigger();
  ExpectPatternEq("Wave trigger", &wave, INCREASING_PATTERN);
  EXPECT_FALSE("Wave trigger", wave.enable);
  EXPECT_EQ("Wave trigger", wave.period, 2050u);
  EXPECT_EQ("Wave trigger", wave.patternOffset, (n5)0);
  EXPECT_EQ("Wave trigger", wave.patternSample, (n4)0);
  EXPECT_EQ("Wave trigger", wave.patternHold, 0u);
  EXPECT_EQ("Wave trigger", wave.length, 256u);

  // length is 0, so it gets set to 256
  // counter is true, so length gets decremented to 255
  PowerAndZeroPattern(&wave);
  SetPattern(&wave, INCREASING_PATTERN);
  wave.patternOffset = (n5)5;
  wave.length = 0;
  wave.counter = true;
  ::ares::GameBoy::apu.phase = 3;
  ::ares::GameBoy::system.information.model =
      ::ares::GameBoy::System::Model::GameBoy;
  wave.trigger();
  ExpectPatternEq("Wave trigger", &wave, INCREASING_PATTERN);
  EXPECT_FALSE("Wave trigger", wave.enable);
  EXPECT_EQ("Wave trigger", wave.period, 2050u);
  EXPECT_EQ("Wave trigger", wave.patternOffset, (n5)0);
  EXPECT_EQ("Wave trigger", wave.patternSample, (n4)0);
  EXPECT_EQ("Wave trigger", wave.patternHold, 0u);
  EXPECT_EQ("Wave trigger", wave.length, 255u);

  // length is 0, so it gets set to 256
  // counter is true but apu phase is even, so length does not get decremented
  // to 255
  PowerAndZeroPattern(&wave);
  SetPattern(&wave, INCREASING_PATTERN);
  wave.patternOffset = (n5)5;
  wave.length = 0;
  wave.counter = true;
  ::ares::GameBoy::apu.phase = 2;
  ::ares::GameBoy::system.information.model =
      ::ares::GameBoy::System::Model::GameBoy;
  wave.trigger();
  ExpectPatternEq("Wave trigger", &wave, INCREASING_PATTERN);
  EXPECT_FALSE("Wave trigger", wave.enable);
  EXPECT_EQ("Wave trigger", wave.period, 2050u);
  EXPECT_EQ("Wave trigger", wave.patternOffset, (n5)0);
  EXPECT_EQ("Wave trigger", wave.patternSample, (n4)0);
  EXPECT_EQ("Wave trigger", wave.patternHold, 0u);
  EXPECT_EQ("Wave trigger", wave.length, 256u);

  // Pattern corruption, case 1
  PowerAndZeroPattern(&wave);
  SetPattern(&wave, INCREASING_PATTERN);
  wave.patternOffset = (n5)5;
  wave.length = 5;
  wave.patternHold = 5;
  ::ares::GameBoy::apu.phase = 3;
  ::ares::GameBoy::system.information.model =
      ::ares::GameBoy::System::Model::GameBoy;
  wave.trigger();
  ExpectPatternEq("Wave trigger", &wave, CORRUPTED_PATTERN_1);
  EXPECT_FALSE("Wave trigger", wave.enable);
  EXPECT_EQ("Wave trigger", wave.period, 2050u);
  EXPECT_EQ("Wave trigger", wave.patternOffset, (n5)0);
  EXPECT_EQ("Wave trigger", wave.patternSample, (n4)0);
  EXPECT_EQ("Wave trigger", wave.patternHold, 0u);
  EXPECT_EQ("Wave trigger", wave.length, 5u);

  // Pattern corruption, case 2
  PowerAndZeroPattern(&wave);
  SetPattern(&wave, INCREASING_PATTERN);
  wave.patternOffset = (n5)20;
  wave.length = 5;
  wave.patternHold = 5;
  ::ares::GameBoy::apu.phase = 3;
  ::ares::GameBoy::system.information.model =
      ::ares::GameBoy::System::Model::GameBoy;
  wave.trigger();
  ExpectPatternEq("Wave trigger", &wave, CORRUPTED_PATTERN_2);
  EXPECT_FALSE("Wave trigger", wave.enable);
  EXPECT_EQ("Wave trigger", wave.period, 2050u);
  EXPECT_EQ("Wave trigger", wave.patternOffset, (n5)0);
  EXPECT_EQ("Wave trigger", wave.patternSample, (n4)0);
  EXPECT_EQ("Wave trigger", wave.patternHold, 0u);
  EXPECT_EQ("Wave trigger", wave.length, 5u);

  ::ares::GameBoy::system.information.model = old_model;
}

void TestReadRam() {
  APU::Wave wave;

  // data is returned
  PowerAndZeroPattern(&wave);
  SetPattern(&wave, INCREASING_PATTERN);
  wave.enable = true;
  wave.patternHold = 0;
  EXPECT_EQ("Wave readRAM", wave.readRAM(0x03, 100), (n8)100);

  // data read using pattern offset
  PowerAndZeroPattern(&wave);
  SetPattern(&wave, INCREASING_PATTERN);
  wave.enable = true;
  wave.patternHold = 5;
  wave.patternOffset = 10;
  EXPECT_EQ("Wave readRAM", wave.readRAM(0x03, 100), (n8)6);

  // data read using address
  PowerAndZeroPattern(&wave);
  SetPattern(&wave, INCREASING_PATTERN);
  wave.enable = false;
  wave.patternHold = 5;
  wave.patternOffset = 10;
  EXPECT_EQ("Wave readRAM", wave.readRAM(0x03, 100), (n8)4);
}

static const n8 OUTPUT_PATTERN_1[16] = {1, 2,  3,  4,  5,  100, 7, 8,
                                        9, 10, 11, 12, 13, 14,  15};
static const n8 OUTPUT_PATTERN_2[16] = {1, 2,  3,  100, 5,  6,  7, 8,
                                        9, 10, 11, 12,  13, 14, 15};

void TestWriteRam() {
  APU::Wave wave;

  // nothing happems
  PowerAndZeroPattern(&wave);
  SetPattern(&wave, INCREASING_PATTERN);
  wave.enable = true;
  wave.patternHold = 0;
  wave.writeRAM(0x03, 100);
  ExpectPatternEq("Wave writeRam", &wave, INCREASING_PATTERN);

  // data written using pattern offset
  PowerAndZeroPattern(&wave);
  SetPattern(&wave, INCREASING_PATTERN);
  wave.enable = true;
  wave.patternHold = 5;
  wave.patternOffset = 10;
  wave.writeRAM(0x03, 100);
  ExpectPatternEq("Wave writeRam", &wave, OUTPUT_PATTERN_1);

  // data written using address
  PowerAndZeroPattern(&wave);
  SetPattern(&wave, INCREASING_PATTERN);
  wave.enable = false;
  wave.patternHold = 5;
  wave.patternOffset = 10;
  wave.writeRAM(0x03, 100);
  ExpectPatternEq("Wave writeRam", &wave, OUTPUT_PATTERN_2);
}

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
  TestTrigger();
  TestReadRam();
  TestWriteRam();
  TestPower();
}
} // namespace wave
