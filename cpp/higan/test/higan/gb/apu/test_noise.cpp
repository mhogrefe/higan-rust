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
  std::vector<int16> output;
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
  EXPECT_EQ("Noise run", noise.output, (int16)0);

  noise.power(true);
  noise.period = 0;
  noise.lfsr = 2;
  noise.enable = true;
  noise.volume = 10;
  noise.run();
  EXPECT_EQ("Noise run", noise.output, (int16)10);

  noise.power(true);
  noise.period = 1;
  noise.lfsr = 0x1844e573;
  noise.enable = true;
  noise.volume = 10;
  noise.divisor = 0;
  noise.frequency = 0;
  noise.narrow = false;
  RunHelper(&noise, 32, "[0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10, 10, 0, 0, 0, "
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
  RunHelper(&noise, 32, "[0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10, 10, 0, 0, 0, "
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
  RunHelper(&noise, 32, "[0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10, "
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
  noise.envelopeFrequency = (uint3)5;
  noise.envelopePeriod = (uint3)1;
  noise.volume = (uint4)10;
  noise.clockEnvelope();
  EXPECT_EQ("Noise clockEnvelope", noise.envelopePeriod, (uint3)1);
  EXPECT_EQ("Noise clockEnvelope", noise.volume, (uint4)10);

  noise.power(true);
  noise.enable = true;
  noise.envelopeFrequency = (uint3)0;
  noise.envelopePeriod = (uint3)1;
  noise.volume = (uint4)10;
  noise.clockEnvelope();
  EXPECT_EQ("Noise clockEnvelope", noise.envelopePeriod, (uint3)1);
  EXPECT_EQ("Noise clockEnvelope", noise.volume, (uint4)10);

  noise.power(true);
  noise.enable = true;
  noise.envelopeFrequency = (uint3)5;
  noise.envelopePeriod = (uint3)5;
  noise.volume = (uint4)10;
  noise.clockEnvelope();
  EXPECT_EQ("Noise clockEnvelope", noise.envelopePeriod, (uint3)4);
  EXPECT_EQ("Noise clockEnvelope", noise.volume, (uint4)10);

  noise.power(true);
  noise.enable = true;
  noise.envelopeDirection = false;
  noise.envelopeFrequency = (uint3)5;
  noise.envelopePeriod = (uint3)1;
  noise.volume = (uint4)10;
  noise.clockEnvelope();
  EXPECT_EQ("Noise clockEnvelope", noise.envelopePeriod, (uint3)5);
  EXPECT_EQ("Noise clockEnvelope", noise.volume, (uint4)9);

  // volume already at min
  noise.power(true);
  noise.enable = true;
  noise.envelopeDirection = false;
  noise.envelopeFrequency = (uint3)5;
  noise.envelopePeriod = (uint3)1;
  noise.volume = (uint4)0;
  noise.clockEnvelope();
  EXPECT_EQ("Noise clockEnvelope", noise.envelopePeriod, (uint3)5);
  EXPECT_EQ("Noise clockEnvelope", noise.volume, (uint4)0);

  noise.power(true);
  noise.enable = true;
  noise.envelopeDirection = true;
  noise.envelopeFrequency = (uint3)5;
  noise.envelopePeriod = (uint3)1;
  noise.volume = (uint4)10;
  noise.clockEnvelope();
  EXPECT_EQ("Noise clockEnvelope", noise.envelopePeriod, (uint3)5);
  EXPECT_EQ("Noise clockEnvelope", noise.volume, (uint4)11);

  // volume already at max
  noise.power(true);
  noise.enable = true;
  noise.envelopeDirection = true;
  noise.envelopeFrequency = (uint3)5;
  noise.envelopePeriod = (uint3)1;
  noise.volume = (uint4)15;
  noise.clockEnvelope();
  EXPECT_EQ("Noise clockEnvelope", noise.envelopePeriod, (uint3)5);
  EXPECT_EQ("Noise clockEnvelope", noise.volume, (uint4)15);
}

void TestRead() {
  APU::Noise noise;

  noise.power(true);
  EXPECT_EQ("Noise read", noise.read(0xff1f), (uint8)0b11111111);

  noise.power(true);
  EXPECT_EQ("Noise read", noise.read(0xff20), (uint8)0b11111111);

  noise.power(true);
  noise.envelopeVolume = (uint4)0b1011;
  noise.envelopeDirection = true;
  noise.envelopeFrequency = (uint3)0b010;
  EXPECT_EQ("Noise read", noise.read(0xff21), (uint8)0b10111010);

  noise.power(true);
  noise.frequency = (uint4)0b1011;
  noise.narrow = true;
  noise.divisor = (uint3)0b010;
  EXPECT_EQ("Noise read", noise.read(0xff22), (uint8)0b10111010);

  noise.power(true);
  noise.counter = false;
  EXPECT_EQ("Noise read", noise.read(0xff23), (uint8)0b10111111);
}

void TestAll() {
  TestDacEnable();
  TestGetPeriod();
  TestRun();
  TestClockLength();
  TestClockEnvelope();
  TestRead();
}
}
