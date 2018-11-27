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

void TestAll() { TestGetPattern(); }
}
