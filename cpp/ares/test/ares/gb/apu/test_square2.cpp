namespace square2 {

void TestDacEnable() {
  APU::Square2 square2;

  square2.power(true);
  square2.envelopeVolume = 0;
  square2.envelopeDirection = false;
  EXPECT_FALSE("Square2 dacEnable", square2.dacEnable());

  square2.power(true);
  square2.envelopeVolume = 3;
  square2.envelopeDirection = false;
  EXPECT_TRUE("Square2 dacEnable", square2.dacEnable());

  square2.power(true);
  square2.envelopeVolume = 0;
  square2.envelopeDirection = true;
  EXPECT_TRUE("Square2 dacEnable", square2.dacEnable());

  square2.power(true);
  square2.envelopeVolume = 3;
  square2.envelopeDirection = true;
  EXPECT_TRUE("Square2 dacEnable", square2.dacEnable());
}

void RunHelper(APU::Square2 *square2, uint cycles,
               const std::string &expected_output) {
  std::vector<i16> output;
  for (int i = 0; i < cycles; ++i) {
    square2->run();
    output.push_back(square2->output);
  }
  EXPECT_EQ("Square2 run", ToString(output), expected_output);
}

void TestRun() {
  APU::Square2 square2;

  square2.power(true);
  square2.period = 0;
  square2.dutyOutput = false;
  square2.enable = false;
  square2.run();
  EXPECT_EQ("Square2 run", square2.output, (i16)0);

  square2.power(true);
  square2.period = 0;
  square2.dutyOutput = true;
  square2.enable = false;
  square2.volume = 10;
  square2.run();
  EXPECT_EQ("Square2 run", square2.output, (i16)0);

  square2.power(true);
  square2.period = 0;
  square2.dutyOutput = true;
  square2.enable = true;
  square2.volume = 10;
  square2.run();
  EXPECT_EQ("Square2 run", square2.output, (i16)10);

  square2.power(true);
  square2.period = 30;
  square2.dutyOutput = false;
  square2.enable = false;
  square2.volume = 10;
  square2.run();
  EXPECT_EQ("Square2 run", square2.output, (i16)0);

  square2.power(true);
  square2.period = 30;
  square2.dutyOutput = true;
  square2.enable = false;
  square2.volume = 10;
  square2.run();
  EXPECT_EQ("Square2 run", square2.output, (i16)0);

  square2.power(true);
  square2.period = 30;
  square2.dutyOutput = true;
  square2.enable = true;
  square2.volume = 10;
  square2.run();
  EXPECT_EQ("Square2 run", square2.output, (i16)10);

  square2.power(true);
  square2.frequency = 2047;
  square2.duty = 0;
  square2.enable = true;
  square2.volume = 10;
  square2.period = 1;
  RunHelper(&square2, 32,
            "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 0, 0, 0, 0, "
            "0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 0, 0, 0, 0]");

  square2.power(true);
  square2.frequency = 2047;
  square2.duty = 1;
  square2.enable = true;
  square2.volume = 10;
  square2.period = 1;
  RunHelper(&square2, 32,
            "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 0, "
            "0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 0, "
            "0]");

  square2.power(true);
  square2.frequency = 2047;
  square2.duty = 2;
  square2.enable = true;
  square2.volume = 10;
  square2.period = 1;
  RunHelper(&square2, 32,
            "[0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10, 10, "
            "0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10, "
            "10, 0, 0]");

  square2.power(true);
  square2.frequency = 2047;
  square2.duty = 3;
  square2.enable = true;
  square2.volume = 10;
  square2.period = 1;
  RunHelper(&square2, 32,
            "[10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 0, 0, 0, "
            "0, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, "
            "0, 0, 0, 0, 10, 10]");

  square2.power(true);
  square2.frequency = 2046;
  square2.duty = 0;
  square2.enable = true;
  square2.volume = 13;
  square2.period = 1;
  RunHelper(&square2, 32,
            "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "
            "0, 0, 0, 13, 13, 13, 13, 0, 0, 0, 0, 0, 0, 0, 0]");
}

void TestClockLength() {
  APU::Square2 square2;

  // counter is false
  square2.power(true);
  square2.counter = false;
  square2.enable = true;
  square2.length = 5;
  square2.clockLength();
  EXPECT_EQ("Square2 clockLength", square2.length, 5u);
  EXPECT_TRUE("Square2 clockLength", square2.enable);

  square2.power(true);
  square2.counter = true;
  square2.enable = true;
  square2.length = 5;
  square2.clockLength();
  EXPECT_EQ("Square2 clockLength", square2.length, 4u);
  EXPECT_TRUE("Square2 clockLength", square2.enable);

  // length is initially 0
  square2.power(true);
  square2.counter = true;
  square2.enable = true;
  square2.length = 0;
  square2.clockLength();
  EXPECT_EQ("Square2 clockLength", square2.length, 0u);
  EXPECT_TRUE("Square2 clockLength", square2.enable);

  // length is initially 1
  square2.power(true);
  square2.counter = true;
  square2.enable = true;
  square2.length = 1;
  square2.clockLength();
  EXPECT_EQ("Square2 clockLength", square2.length, 0u);
  EXPECT_FALSE("Square2 clockLength", square2.enable);
}

void TestClockEnvelope() {
  APU::Square2 square2;

  square2.power(true);
  square2.enable = false;
  square2.envelopeFrequency = (n3)5;
  square2.envelopePeriod = (n3)1;
  square2.volume = (n4)10;
  square2.clockEnvelope();
  EXPECT_EQ("Square2 clockEnvelope", square2.envelopePeriod, (n3)1);
  EXPECT_EQ("Square2 clockEnvelope", square2.volume, (n4)10);

  square2.power(true);
  square2.enable = true;
  square2.envelopeFrequency = (n3)0;
  square2.envelopePeriod = (n3)1;
  square2.volume = (n4)10;
  square2.clockEnvelope();
  EXPECT_EQ("Square2 clockEnvelope", square2.envelopePeriod, (n3)1);
  EXPECT_EQ("Square2 clockEnvelope", square2.volume, (n4)10);

  square2.power(true);
  square2.enable = true;
  square2.envelopeFrequency = (n3)5;
  square2.envelopePeriod = (n3)5;
  square2.volume = (n4)10;
  square2.clockEnvelope();
  EXPECT_EQ("Square2 clockEnvelope", square2.envelopePeriod, (n3)4);
  EXPECT_EQ("Square2 clockEnvelope", square2.volume, (n4)10);

  square2.power(true);
  square2.enable = true;
  square2.envelopeDirection = false;
  square2.envelopeFrequency = (n3)5;
  square2.envelopePeriod = (n3)1;
  square2.volume = (n4)10;
  square2.clockEnvelope();
  EXPECT_EQ("Square2 clockEnvelope", square2.envelopePeriod, (n3)5);
  EXPECT_EQ("Square2 clockEnvelope", square2.volume, (n4)9);

  // volume already at min
  square2.power(true);
  square2.enable = true;
  square2.envelopeDirection = false;
  square2.envelopeFrequency = (n3)5;
  square2.envelopePeriod = (n3)1;
  square2.volume = (n4)0;
  square2.clockEnvelope();
  EXPECT_EQ("Square2 clockEnvelope", square2.envelopePeriod, (n3)5);
  EXPECT_EQ("Square2 clockEnvelope", square2.volume, (n4)0);

  square2.power(true);
  square2.enable = true;
  square2.envelopeDirection = true;
  square2.envelopeFrequency = (n3)5;
  square2.envelopePeriod = (n3)1;
  square2.volume = (n4)10;
  square2.clockEnvelope();
  EXPECT_EQ("Square2 clockEnvelope", square2.envelopePeriod, (n3)5);
  EXPECT_EQ("Square2 clockEnvelope", square2.volume, (n4)11);

  // volume already at max
  square2.power(true);
  square2.enable = true;
  square2.envelopeDirection = true;
  square2.envelopeFrequency = (n3)5;
  square2.envelopePeriod = (n3)1;
  square2.volume = (n4)15;
  square2.clockEnvelope();
  EXPECT_EQ("Square2 clockEnvelope", square2.envelopePeriod, (n3)5);
  EXPECT_EQ("Square2 clockEnvelope", square2.volume, (n4)15);
}

void TestPower() {
  APU::Square2 square2;
  square2.length = 0;
  square2.power(true);
  EXPECT_EQ("Square2 power", square2.length, 64u);

  square2.length = 0;
  square2.power(false);
  EXPECT_EQ("Square2 power", square2.length, 0u);
}

void TestAll() {
  TestDacEnable();
  TestRun();
  TestClockLength();
  TestClockEnvelope();
  TestPower();
}
} // namespace square2
