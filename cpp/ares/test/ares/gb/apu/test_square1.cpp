namespace square1 {

void TestDacEnable() {
  APU::Square1 square1;

  square1.power(true);
  square1.envelopeVolume = 0;
  square1.envelopeDirection = false;
  EXPECT_FALSE("Square1 dacEnable", square1.dacEnable());

  square1.power(true);
  square1.envelopeVolume = 3;
  square1.envelopeDirection = false;
  EXPECT_TRUE("Square1 dacEnable", square1.dacEnable());

  square1.power(true);
  square1.envelopeVolume = 0;
  square1.envelopeDirection = true;
  EXPECT_TRUE("Square1 dacEnable", square1.dacEnable());

  square1.power(true);
  square1.envelopeVolume = 3;
  square1.envelopeDirection = true;
  EXPECT_TRUE("Square1 dacEnable", square1.dacEnable());
}

void RunHelper(APU::Square1 *square1, uint cycles,
               const std::string &expected_output) {
  std::vector<i16> output;
  for (int i = 0; i < cycles; ++i) {
    square1->run();
    output.push_back(square1->output);
  }
  EXPECT_EQ("Square1 run", ToString(output), expected_output);
}

void TestRun() {
  APU::Square1 square1;

  square1.power(true);
  square1.period = 0;
  square1.dutyOutput = false;
  square1.enable = false;
  square1.run();
  EXPECT_EQ("Square1 run", square1.output, (i16)0);

  square1.power(true);
  square1.period = 0;
  square1.dutyOutput = true;
  square1.enable = false;
  square1.volume = 10;
  square1.run();
  EXPECT_EQ("Square1 run", square1.output, (i16)0);

  square1.power(true);
  square1.period = 0;
  square1.dutyOutput = true;
  square1.enable = true;
  square1.volume = 10;
  square1.run();
  EXPECT_EQ("Square1 run", square1.output, (i16)10);

  square1.power(true);
  square1.period = 30;
  square1.dutyOutput = false;
  square1.enable = false;
  square1.volume = 10;
  square1.run();
  EXPECT_EQ("Square1 run", square1.output, (i16)0);

  square1.power(true);
  square1.period = 30;
  square1.dutyOutput = true;
  square1.enable = false;
  square1.volume = 10;
  square1.run();
  EXPECT_EQ("Square1 run", square1.output, (i16)0);

  square1.power(true);
  square1.period = 30;
  square1.dutyOutput = true;
  square1.enable = true;
  square1.volume = 10;
  square1.run();
  EXPECT_EQ("Square1 run", square1.output, (i16)10);

  square1.power(true);
  square1.frequency = 2047;
  square1.duty = 0;
  square1.enable = true;
  square1.volume = 10;
  square1.period = 1;
  RunHelper(&square1, 32,
            "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 0, 0, 0, 0, "
            "0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 0, 0, 0, 0]");

  square1.power(true);
  square1.frequency = 2047;
  square1.duty = 1;
  square1.enable = true;
  square1.volume = 10;
  square1.period = 1;
  RunHelper(&square1, 32,
            "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 0, "
            "0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 0, "
            "0]");

  square1.power(true);
  square1.frequency = 2047;
  square1.duty = 2;
  square1.enable = true;
  square1.volume = 10;
  square1.period = 1;
  RunHelper(&square1, 32,
            "[0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10, 10, "
            "0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10, "
            "10, 0, 0]");

  square1.power(true);
  square1.frequency = 2047;
  square1.duty = 3;
  square1.enable = true;
  square1.volume = 10;
  square1.period = 1;
  RunHelper(&square1, 32,
            "[10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 0, 0, 0, "
            "0, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, "
            "0, 0, 0, 0, 10, 10]");

  square1.power(true);
  square1.frequency = 2046;
  square1.duty = 0;
  square1.enable = true;
  square1.volume = 13;
  square1.period = 1;
  RunHelper(&square1, 32,
            "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "
            "0, 0, 0, 13, 13, 13, 13, 0, 0, 0, 0, 0, 0, 0, 0]");
}

void TestSweep() {
  APU::Square1 square1;

  // sweepEnable false
  square1.power(true);
  square1.enable = true;
  square1.sweepEnable = false;
  square1.frequencyShadow = 10;
  square1.sweepShift = 1;
  square1.sweep(true);
  EXPECT_TRUE("Square1 sweep", square1.enable);
  EXPECT_EQ("Square1 sweep", square1.frequencyShadow, 10);
  EXPECT_EQ("Square1 sweep", square1.frequency, (n11)0);
  EXPECT_EQ("Square1 sweep", square1.period, 0u);

  // positive delta
  square1.power(true);
  square1.enable = true;
  square1.sweepEnable = true;
  square1.frequencyShadow = 10;
  square1.sweepShift = 1;
  square1.sweep(true);
  EXPECT_TRUE("Square1 sweep", square1.enable);
  EXPECT_EQ("Square1 sweep", square1.frequencyShadow, 15);
  EXPECT_EQ("Square1 sweep", square1.frequency, (n11)15);
  EXPECT_EQ("Square1 sweep", square1.period, 4066u);

  // freq exceeds 2047
  square1.power(true);
  square1.enable = true;
  square1.sweepEnable = true;
  square1.frequencyShadow = 2047;
  square1.sweepShift = 1;
  square1.sweep(true);
  EXPECT_FALSE("Square1 sweep", square1.enable);
  EXPECT_EQ("Square1 sweep", square1.frequencyShadow, 2047);
  EXPECT_EQ("Square1 sweep", square1.frequency, (n11)0);
  EXPECT_EQ("Square1 sweep", square1.period, 0u);

  // negative delta
  square1.power(true);
  square1.enable = true;
  square1.sweepDirection = true;
  square1.sweepEnable = true;
  square1.frequencyShadow = 10;
  square1.sweepShift = 1;
  square1.sweep(true);
  EXPECT_TRUE("Square1 sweep", square1.enable);
  EXPECT_EQ("Square1 sweep", square1.frequencyShadow, 5);
  EXPECT_EQ("Square1 sweep", square1.frequency, (n11)5);
  EXPECT_EQ("Square1 sweep", square1.period, 4086u);
}

void TestClockLength() {
  APU::Square1 square1;

  // counter is false
  square1.power(true);
  square1.counter = false;
  square1.enable = true;
  square1.length = 5;
  square1.clockLength();
  EXPECT_EQ("Square1 clockLength", square1.length, 5u);
  EXPECT_TRUE("Square1 clockLength", square1.enable);

  square1.power(true);
  square1.counter = true;
  square1.enable = true;
  square1.length = 5;
  square1.clockLength();
  EXPECT_EQ("Square1 clockLength", square1.length, 4u);
  EXPECT_TRUE("Square1 clockLength", square1.enable);

  // length is initially 0
  square1.power(true);
  square1.counter = true;
  square1.enable = true;
  square1.length = 0;
  square1.clockLength();
  EXPECT_EQ("Square1 clockLength", square1.length, 0u);
  EXPECT_TRUE("Square1 clockLength", square1.enable);

  // length is initially 1
  square1.power(true);
  square1.counter = true;
  square1.enable = true;
  square1.length = 1;
  square1.clockLength();
  EXPECT_EQ("Square1 clockLength", square1.length, 0u);
  EXPECT_FALSE("Square1 clockLength", square1.enable);
}

void TestClockSweep() {
  APU::Square1 square1;

  square1.power(true);
  square1.sweepPeriod = (n3)5;
  square1.clockSweep();
  EXPECT_EQ("Square1 clockSweep", square1.sweepPeriod, (n3)4);

  square1.power(true);
  square1.sweepPeriod = (n3)1;
  square1.sweepFrequency = 0;
  square1.clockSweep();
  EXPECT_EQ("Square1 clockSweep", square1.sweepPeriod, (n3)0);

  square1.power(true);
  square1.enable = true;
  square1.sweepEnable = true;
  square1.sweepPeriod = (n3)1;
  square1.sweepFrequency = 5;
  square1.frequencyShadow = 10;
  square1.sweepShift = 1;
  square1.clockSweep();
  EXPECT_EQ("Square1 clockSweep", square1.sweepPeriod, (n3)5);
  EXPECT_TRUE("Square1 clockSweep", square1.enable);
  EXPECT_EQ("Square1 clockSweep", square1.frequencyShadow, 15);
  EXPECT_EQ("Square1 clockSweep", square1.frequency, (n11)15);
  EXPECT_EQ("Square1 clockSweep", square1.period, 4066u);

  // sweepEnable is false
  square1.power(true);
  square1.enable = true;
  square1.sweepEnable = false;
  square1.sweepPeriod = (n3)1;
  square1.sweepFrequency = 5;
  square1.frequencyShadow = 10;
  square1.sweepShift = 1;
  square1.clockSweep();
  EXPECT_EQ("Square1 clockSweep", square1.sweepPeriod, (n3)5);
  EXPECT_TRUE("Square1 clockSweep", square1.enable);
  EXPECT_EQ("Square1 clockSweep", square1.frequencyShadow, 10);
  EXPECT_EQ("Square1 clockSweep", square1.frequency, (n11)0);
  EXPECT_EQ("Square1 clockSweep", square1.period, 0u);
}

void TestClockEnvelope() {
  APU::Square1 square1;

  square1.power(true);
  square1.enable = false;
  square1.envelopeFrequency = (n3)5;
  square1.envelopePeriod = (n3)1;
  square1.volume = (n4)10;
  square1.clockEnvelope();
  EXPECT_EQ("Square1 clockEnvelope", square1.envelopePeriod, (n3)1);
  EXPECT_EQ("Square1 clockEnvelope", square1.volume, (n4)10);

  square1.power(true);
  square1.enable = true;
  square1.envelopeFrequency = (n3)0;
  square1.envelopePeriod = (n3)1;
  square1.volume = (n4)10;
  square1.clockEnvelope();
  EXPECT_EQ("Square1 clockEnvelope", square1.envelopePeriod, (n3)1);
  EXPECT_EQ("Square1 clockEnvelope", square1.volume, (n4)10);

  square1.power(true);
  square1.enable = true;
  square1.envelopeFrequency = (n3)5;
  square1.envelopePeriod = (n3)5;
  square1.volume = (n4)10;
  square1.clockEnvelope();
  EXPECT_EQ("Square1 clockEnvelope", square1.envelopePeriod, (n3)4);
  EXPECT_EQ("Square1 clockEnvelope", square1.volume, (n4)10);

  square1.power(true);
  square1.enable = true;
  square1.envelopeDirection = false;
  square1.envelopeFrequency = (n3)5;
  square1.envelopePeriod = (n3)1;
  square1.volume = (n4)10;
  square1.clockEnvelope();
  EXPECT_EQ("Square1 clockEnvelope", square1.envelopePeriod, (n3)5);
  EXPECT_EQ("Square1 clockEnvelope", square1.volume, (n4)9);

  // volume already at min
  square1.power(true);
  square1.enable = true;
  square1.envelopeDirection = false;
  square1.envelopeFrequency = (n3)5;
  square1.envelopePeriod = (n3)1;
  square1.volume = (n4)0;
  square1.clockEnvelope();
  EXPECT_EQ("Square1 clockEnvelope", square1.envelopePeriod, (n3)5);
  EXPECT_EQ("Square1 clockEnvelope", square1.volume, (n4)0);

  square1.power(true);
  square1.enable = true;
  square1.envelopeDirection = true;
  square1.envelopeFrequency = (n3)5;
  square1.envelopePeriod = (n3)1;
  square1.volume = (n4)10;
  square1.clockEnvelope();
  EXPECT_EQ("Square1 clockEnvelope", square1.envelopePeriod, (n3)5);
  EXPECT_EQ("Square1 clockEnvelope", square1.volume, (n4)11);

  // volume already at max
  square1.power(true);
  square1.enable = true;
  square1.envelopeDirection = true;
  square1.envelopeFrequency = (n3)5;
  square1.envelopePeriod = (n3)1;
  square1.volume = (n4)15;
  square1.clockEnvelope();
  EXPECT_EQ("Square1 clockEnvelope", square1.envelopePeriod, (n3)5);
  EXPECT_EQ("Square1 clockEnvelope", square1.volume, (n4)15);
}

void TestPower() {
  APU::Square1 square1;
  square1.length = 0;
  square1.power(true);
  EXPECT_EQ("Square1 power", square1.length, 64u);

  square1.length = 0;
  square1.power(false);
  EXPECT_EQ("Square1 power", square1.length, 0u);
}

void TestAll() {
  TestDacEnable();
  TestRun();
  TestSweep();
  TestClockLength();
  TestClockSweep();
  TestClockEnvelope();
  // TestRead();
  // TestWrite();
  TestPower();
}
} // namespace square1
