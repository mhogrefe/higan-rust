namespace noise {

void TestDacEnable() {
  APU::Noise noise;
  noise.power(true);

  noise.envelopeVolume = 0;
  noise.envelopeDirection = false;
  EXPECT_FALSE("Noise dacEnable", noise.dacEnable());

  noise.envelopeVolume = 3;
  noise.envelopeDirection = false;
  EXPECT_TRUE("Noise dacEnable", noise.dacEnable());

  noise.envelopeVolume = 0;
  noise.envelopeDirection = true;
  EXPECT_TRUE("Noise dacEnable", noise.dacEnable());

  noise.envelopeVolume = 3;
  noise.envelopeDirection = true;
  EXPECT_TRUE("Noise dacEnable", noise.dacEnable());
}

void TestGetPeriod() {
  APU::Noise noise;
  noise.power(true);

  noise.frequency = 2;
  noise.divisor = 0;
  EXPECT_EQ("Noise getPeriod", noise.getPeriod(), 16u);

  noise.power(true);
  noise.frequency = 1;
  noise.divisor = 5;
  EXPECT_EQ("Noise getPeriod", noise.getPeriod(), 80u);
}

void RunHelper(APU::Noise *noise, uint cycles,
               const std::string &expected_output) {
  std::vector<i16> output;
  for (int i = 0; i < cycles; ++i) {
    noise->run();
    output.push_back(noise->output);
  }
  EXPECT_EQ("Noise run", ToString(output), expected_output);
}

void TestRun() {
  APU::Noise noise;

  noise.power(true);
  noise.period = 0;
  noise.lfsr = 2;
  noise.enable = false;
  noise.volume = 10;
  noise.run();
  EXPECT_EQ("Noise run", noise.output, (i16)0);

  noise.power(true);
  noise.period = 0;
  noise.lfsr = 2;
  noise.enable = true;
  noise.volume = 10;
  noise.run();
  EXPECT_EQ("Noise run", noise.output, (i16)10);

  noise.power(true);
  noise.period = 1;
  noise.lfsr = 0x1844e573;
  noise.enable = true;
  noise.volume = 10;
  noise.divisor = 0;
  noise.frequency = 0;
  noise.narrow = false;
  RunHelper(&noise, 32,
            "[0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10, 10, 0, 0, 0, "
            "0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 0, 0, 0, "
            "0]");

  noise.power(true);
  noise.period = 1;
  noise.lfsr = 0x1844e573;
  noise.enable = true;
  noise.volume = 10;
  noise.divisor = 0;
  noise.frequency = 0;
  noise.narrow = true;
  RunHelper(&noise, 32,
            "[0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10, 10, 0, 0, 0, "
            "0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 10, 10, "
            "10, 10]");

  noise.power(true);
  noise.period = 1;
  noise.lfsr = 0x1844e573;
  noise.enable = true;
  noise.volume = 10;
  noise.divisor = 1;
  noise.frequency = 0;
  noise.narrow = true;
  RunHelper(&noise, 32,
            "[0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10, "
            "10, 10, 10, 10, 10, 10, 10, 10, 10, 0, 0, 0, 0, 0, 0, "
            "0, 0]");
}

void TestClockLength() {
  APU::Noise noise;

  // counter is false
  noise.power(true);
  noise.counter = false;
  noise.enable = true;
  noise.length = 5;
  noise.clockLength();
  EXPECT_EQ("Noise clockLength", noise.length, 5u);
  EXPECT_TRUE("Noise clockLength", noise.enable);

  noise.power(true);
  noise.counter = true;
  noise.enable = true;
  noise.length = 5;
  noise.clockLength();
  EXPECT_EQ("Noise clockLength", noise.length, 4u);
  EXPECT_TRUE("Noise clockLength", noise.enable);

  // length is initially 0
  noise.power(true);
  noise.counter = true;
  noise.enable = true;
  noise.length = 0;
  noise.clockLength();
  EXPECT_EQ("Noise clockLength", noise.length, 0u);
  EXPECT_TRUE("Noise clockLength", noise.enable);

  // length is initially 1
  noise.power(true);
  noise.counter = true;
  noise.enable = true;
  noise.length = 1;
  noise.clockLength();
  EXPECT_EQ("Noise clockLength", noise.length, 0u);
  EXPECT_FALSE("Noise clockLength", noise.enable);
}

void TestClockEnvelope() {
  APU::Noise noise;

  noise.power(true);
  noise.enable = false;
  noise.envelopeFrequency = (n3)5;
  noise.envelopePeriod = (n3)1;
  noise.volume = (n4)10;
  noise.clockEnvelope();
  EXPECT_EQ("Noise clockEnvelope", noise.envelopePeriod, (n3)1);
  EXPECT_EQ("Noise clockEnvelope", noise.volume, (n4)10);

  noise.power(true);
  noise.enable = true;
  noise.envelopeFrequency = (n3)0;
  noise.envelopePeriod = (n3)1;
  noise.volume = (n4)10;
  noise.clockEnvelope();
  EXPECT_EQ("Noise clockEnvelope", noise.envelopePeriod, (n3)1);
  EXPECT_EQ("Noise clockEnvelope", noise.volume, (n4)10);

  noise.power(true);
  noise.enable = true;
  noise.envelopeFrequency = (n3)5;
  noise.envelopePeriod = (n3)5;
  noise.volume = (n4)10;
  noise.clockEnvelope();
  EXPECT_EQ("Noise clockEnvelope", noise.envelopePeriod, (n3)4);
  EXPECT_EQ("Noise clockEnvelope", noise.volume, (n4)10);

  noise.power(true);
  noise.enable = true;
  noise.envelopeDirection = false;
  noise.envelopeFrequency = (n3)5;
  noise.envelopePeriod = (n3)1;
  noise.volume = (n4)10;
  noise.clockEnvelope();
  EXPECT_EQ("Noise clockEnvelope", noise.envelopePeriod, (n3)5);
  EXPECT_EQ("Noise clockEnvelope", noise.volume, (n4)9);

  // volume already at min
  noise.power(true);
  noise.enable = true;
  noise.envelopeDirection = false;
  noise.envelopeFrequency = (n3)5;
  noise.envelopePeriod = (n3)1;
  noise.volume = (n4)0;
  noise.clockEnvelope();
  EXPECT_EQ("Noise clockEnvelope", noise.envelopePeriod, (n3)5);
  EXPECT_EQ("Noise clockEnvelope", noise.volume, (n4)0);

  noise.power(true);
  noise.enable = true;
  noise.envelopeDirection = true;
  noise.envelopeFrequency = (n3)5;
  noise.envelopePeriod = (n3)1;
  noise.volume = (n4)10;
  noise.clockEnvelope();
  EXPECT_EQ("Noise clockEnvelope", noise.envelopePeriod, (n3)5);
  EXPECT_EQ("Noise clockEnvelope", noise.volume, (n4)11);

  // volume already at max
  noise.power(true);
  noise.enable = true;
  noise.envelopeDirection = true;
  noise.envelopeFrequency = (n3)5;
  noise.envelopePeriod = (n3)1;
  noise.volume = (n4)15;
  noise.clockEnvelope();
  EXPECT_EQ("Noise clockEnvelope", noise.envelopePeriod, (n3)5);
  EXPECT_EQ("Noise clockEnvelope", noise.volume, (n4)15);
}

void TestPower() {
  APU::Noise noise;
  noise.length = 0;
  noise.power(true);
  EXPECT_EQ("Noise power", noise.length, 64u);

  noise.length = 0;
  noise.power(false);
  EXPECT_EQ("Noise power", noise.length, 0u);
}

void TestAll() {
  TestDacEnable();
  TestGetPeriod();
  TestRun();
  TestClockLength();
  TestClockEnvelope();
  TestPower();
}
} // namespace noise
