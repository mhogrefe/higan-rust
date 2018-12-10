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
  std::vector<int16> output;
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
  EXPECT_EQ("Square1 run", square1.output, (int16)0);

  square1.power(true);
  square1.period = 0;
  square1.dutyOutput = true;
  square1.enable = false;
  square1.volume = 10;
  square1.run();
  EXPECT_EQ("Square1 run", square1.output, (int16)0);

  square1.power(true);
  square1.period = 0;
  square1.dutyOutput = true;
  square1.enable = true;
  square1.volume = 10;
  square1.run();
  EXPECT_EQ("Square1 run", square1.output, (int16)10);

  square1.power(true);
  square1.period = 30;
  square1.dutyOutput = false;
  square1.enable = false;
  square1.volume = 10;
  square1.run();
  EXPECT_EQ("Square1 run", square1.output, (int16)0);

  square1.power(true);
  square1.period = 30;
  square1.dutyOutput = true;
  square1.enable = false;
  square1.volume = 10;
  square1.run();
  EXPECT_EQ("Square1 run", square1.output, (int16)0);

  square1.power(true);
  square1.period = 30;
  square1.dutyOutput = true;
  square1.enable = true;
  square1.volume = 10;
  square1.run();
  EXPECT_EQ("Square1 run", square1.output, (int16)10);

  square1.power(true);
  square1.frequency = 2047;
  square1.duty = 0;
  square1.enable = true;
  square1.volume = 10;
  square1.period = 1;
  RunHelper(&square1, 32, "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 0, 0, 0, 0, "
                          "0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 0, 0, 0, 0]");

  square1.power(true);
  square1.frequency = 2047;
  square1.duty = 1;
  square1.enable = true;
  square1.volume = 10;
  square1.period = 1;
  RunHelper(&square1, 32, "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 0, "
                          "0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 0, "
                          "0]");

  square1.power(true);
  square1.frequency = 2047;
  square1.duty = 2;
  square1.enable = true;
  square1.volume = 10;
  square1.period = 1;
  RunHelper(&square1, 32, "[0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10, 10, "
                          "0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10, "
                          "10, 0, 0]");

  square1.power(true);
  square1.frequency = 2047;
  square1.duty = 3;
  square1.enable = true;
  square1.volume = 10;
  square1.period = 1;
  RunHelper(&square1, 32, "[10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 0, 0, 0, "
                          "0, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, "
                          "0, 0, 0, 0, 10, 10]");

  square1.power(true);
  square1.frequency = 2046;
  square1.duty = 0;
  square1.enable = true;
  square1.volume = 13;
  square1.period = 1;
  RunHelper(&square1, 32, "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "
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
  EXPECT_EQ("Square1 sweep", square1.frequency, (uint11)0);
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
  EXPECT_EQ("Square1 sweep", square1.frequency, (uint11)15);
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
  EXPECT_EQ("Square1 sweep", square1.frequency, (uint11)0);
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
  EXPECT_EQ("Square1 sweep", square1.frequency, (uint11)5);
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
  square1.sweepPeriod = (uint3)5;
  square1.clockSweep();
  EXPECT_EQ("Square1 clockSweep", square1.sweepPeriod, (uint3)4);

  square1.power(true);
  square1.sweepPeriod = (uint3)1;
  square1.sweepFrequency = 0;
  square1.clockSweep();
  EXPECT_EQ("Square1 clockSweep", square1.sweepPeriod, (uint3)0);

  square1.power(true);
  square1.enable = true;
  square1.sweepEnable = true;
  square1.sweepPeriod = (uint3)1;
  square1.sweepFrequency = 5;
  square1.frequencyShadow = 10;
  square1.sweepShift = 1;
  square1.clockSweep();
  EXPECT_EQ("Square1 clockSweep", square1.sweepPeriod, (uint3)5);
  EXPECT_TRUE("Square1 clockSweep", square1.enable);
  EXPECT_EQ("Square1 clockSweep", square1.frequencyShadow, 15);
  EXPECT_EQ("Square1 clockSweep", square1.frequency, (uint11)15);
  EXPECT_EQ("Square1 clockSweep", square1.period, 4066u);

  // sweepEnable is false
  square1.power(true);
  square1.enable = true;
  square1.sweepEnable = false;
  square1.sweepPeriod = (uint3)1;
  square1.sweepFrequency = 5;
  square1.frequencyShadow = 10;
  square1.sweepShift = 1;
  square1.clockSweep();
  EXPECT_EQ("Square1 clockSweep", square1.sweepPeriod, (uint3)5);
  EXPECT_TRUE("Square1 clockSweep", square1.enable);
  EXPECT_EQ("Square1 clockSweep", square1.frequencyShadow, 10);
  EXPECT_EQ("Square1 clockSweep", square1.frequency, (uint11)0);
  EXPECT_EQ("Square1 clockSweep", square1.period, 0u);
}

void TestClockEnvelope() {
  APU::Square1 square1;

  square1.power(true);
  square1.enable = false;
  square1.envelopeFrequency = (uint3)5;
  square1.envelopePeriod = (uint3)1;
  square1.volume = (uint4)10;
  square1.clockEnvelope();
  EXPECT_EQ("Square1 clockEnvelope", square1.envelopePeriod, (uint3)1);
  EXPECT_EQ("Square1 clockEnvelope", square1.volume, (uint4)10);

  square1.power(true);
  square1.enable = true;
  square1.envelopeFrequency = (uint3)0;
  square1.envelopePeriod = (uint3)1;
  square1.volume = (uint4)10;
  square1.clockEnvelope();
  EXPECT_EQ("Square1 clockEnvelope", square1.envelopePeriod, (uint3)1);
  EXPECT_EQ("Square1 clockEnvelope", square1.volume, (uint4)10);

  square1.power(true);
  square1.enable = true;
  square1.envelopeFrequency = (uint3)5;
  square1.envelopePeriod = (uint3)5;
  square1.volume = (uint4)10;
  square1.clockEnvelope();
  EXPECT_EQ("Square1 clockEnvelope", square1.envelopePeriod, (uint3)4);
  EXPECT_EQ("Square1 clockEnvelope", square1.volume, (uint4)10);

  square1.power(true);
  square1.enable = true;
  square1.envelopeDirection = false;
  square1.envelopeFrequency = (uint3)5;
  square1.envelopePeriod = (uint3)1;
  square1.volume = (uint4)10;
  square1.clockEnvelope();
  EXPECT_EQ("Square1 clockEnvelope", square1.envelopePeriod, (uint3)5);
  EXPECT_EQ("Square1 clockEnvelope", square1.volume, (uint4)9);

  // volume already at min
  square1.power(true);
  square1.enable = true;
  square1.envelopeDirection = false;
  square1.envelopeFrequency = (uint3)5;
  square1.envelopePeriod = (uint3)1;
  square1.volume = (uint4)0;
  square1.clockEnvelope();
  EXPECT_EQ("Square1 clockEnvelope", square1.envelopePeriod, (uint3)5);
  EXPECT_EQ("Square1 clockEnvelope", square1.volume, (uint4)0);

  square1.power(true);
  square1.enable = true;
  square1.envelopeDirection = true;
  square1.envelopeFrequency = (uint3)5;
  square1.envelopePeriod = (uint3)1;
  square1.volume = (uint4)10;
  square1.clockEnvelope();
  EXPECT_EQ("Square1 clockEnvelope", square1.envelopePeriod, (uint3)5);
  EXPECT_EQ("Square1 clockEnvelope", square1.volume, (uint4)11);

  // volume already at max
  square1.power(true);
  square1.enable = true;
  square1.envelopeDirection = true;
  square1.envelopeFrequency = (uint3)5;
  square1.envelopePeriod = (uint3)1;
  square1.volume = (uint4)15;
  square1.clockEnvelope();
  EXPECT_EQ("Square1 clockEnvelope", square1.envelopePeriod, (uint3)5);
  EXPECT_EQ("Square1 clockEnvelope", square1.volume, (uint4)15);
}

void TestRead() {
  APU::Square1 square1;

  square1.power(true);
  EXPECT_EQ("Square1 read", square1.read(0), (uint8)0xff);

  square1.power(true);
  square1.sweepFrequency = (uint3)0b101;
  square1.sweepDirection = true;
  square1.sweepShift = (uint3)0b010;
  EXPECT_EQ("Square1 read", square1.read(0xff10), (uint8)0b11011010);

  square1.power(true);
  square1.duty = (uint2)0b01;
  EXPECT_EQ("Square1 read", square1.read(0xff11), (uint8)0b01111111);

  square1.power(true);
  square1.envelopeVolume = (uint4)0b1011;
  square1.envelopeDirection = true;
  square1.envelopeFrequency = (uint3)0b010;
  EXPECT_EQ("Square1 read", square1.read(0xff12), (uint8)0b10111010);

  square1.power(true);
  EXPECT_EQ("Square1 read", square1.read(0xff13), (uint8)0b11111111);

  square1.power(true);
  square1.counter = false;
  EXPECT_EQ("Square1 read", square1.read(0xff14), (uint8)0b10111111);
}

void TestWrite() {
  APU::Square1 square1;

  square1.power(true);
  square1.enable = true;
  square1.sweepEnable = true;
  square1.sweepNegate = true;
  square1.write(0xff10, 0b11011010);
  EXPECT_EQ("Square1 write", square1.sweepFrequency, (uint3)0b101);
  EXPECT_TRUE("Square1 write", square1.sweepDirection);
  EXPECT_EQ("Square1 write", square1.sweepShift, (uint3)0b010);
  EXPECT_TRUE("Square1 write", square1.enable);

  square1.power(true);
  square1.enable = true;
  square1.sweepEnable = true;
  square1.sweepNegate = true;
  square1.write(0xff10, 0b11010010);
  EXPECT_EQ("Square1 write", square1.sweepFrequency, (uint3)0b101);
  EXPECT_FALSE("Square1 write", square1.sweepDirection);
  EXPECT_EQ("Square1 write", square1.sweepShift, (uint3)0b010);
  EXPECT_FALSE("Square1 write", square1.enable);

  square1.power(true);
  square1.write(0xff11, 0b01110010);
  EXPECT_EQ("Square1 write", square1.duty, (uint2)0b01);
  EXPECT_EQ("Square1 write", square1.length, 14u);

  square1.power(true);
  square1.enable = true;
  square1.write(0xff12, 0b10111010);
  EXPECT_EQ("Square1 write", square1.envelopeVolume, (uint4)0b1011);
  EXPECT_TRUE("Square1 write", square1.envelopeDirection);
  EXPECT_EQ("Square1 write", square1.envelopeFrequency, (uint3)0b010);
  EXPECT_TRUE("Square1 write", square1.enable);

  square1.power(true);
  square1.enable = true;
  square1.write(0xff12, 0);
  EXPECT_EQ("Square1 write", square1.envelopeVolume, (uint4)0);
  EXPECT_FALSE("Square1 write", square1.envelopeDirection);
  EXPECT_EQ("Square1 write", square1.envelopeFrequency, (uint3)0);
  EXPECT_FALSE("Square1 write", square1.enable);

  square1.power(true);
  square1.write(0xff13, 0b10110100);
  EXPECT_EQ("Square1 write", square1.frequency, (uint11)0b10110100);

  // data.bit(6) is false, data.bit(7) is true
  square1.power(true);
  square1.write(0xff14, 0b10110011);
  EXPECT_FALSE("Square1 write", square1.enable);
  EXPECT_FALSE("Square1 write", square1.counter);
  EXPECT_EQ("Square1 write", square1.frequency, (uint11)0b01100000000);
  EXPECT_EQ("Square1 write", square1.period, 2560u);
  EXPECT_EQ("Square1 write", square1.envelopePeriod, (uint3)0);
  EXPECT_EQ("Square1 write", square1.volume, (uint4)0);
  EXPECT_EQ("Square1 write", square1.length, 64u);
  EXPECT_EQ("Square1 write", square1.frequencyShadow, 768);
  EXPECT_FALSE("Square1 write", square1.sweepNegate);
  EXPECT_EQ("Square1 write", square1.sweepPeriod, (uint3)0);
  EXPECT_FALSE("Square1 write", square1.sweepEnable);

  // data.bit(6) is false, data.bit(7) is false. Length stays 0
  square1.power(true);
  square1.enable = true;
  square1.length = 0;
  square1.write(0xff14, 0b00110011);
  EXPECT_TRUE("Square1 write", square1.enable);
  EXPECT_FALSE("Square1 write", square1.counter);
  EXPECT_EQ("Square1 write", square1.frequency, (uint11)0b01100000000);
  EXPECT_EQ("Square1 write", square1.length, 0u);

  // data.bit(6) is true, data.bit(7) is true, enable stays true
  square1.power(true);
  square1.length = 1;
  square1.enable = true;
  square1.envelopeVolume = 5;
  square1.envelopeDirection = true;
  square1.write(0xff14, 0b11110011);
  EXPECT_TRUE("Square1 write", square1.enable);
  EXPECT_TRUE("Square1 write", square1.counter);
  EXPECT_EQ("Square1 write", square1.frequency, (uint11)0b01100000000);
  EXPECT_EQ("Square1 write", square1.period, 2560u);
  EXPECT_EQ("Square1 write", square1.envelopePeriod, (uint3)0);
  EXPECT_EQ("Square1 write", square1.volume, (uint4)5);
  EXPECT_EQ("Square1 write", square1.length, 1u);
  EXPECT_EQ("Square1 write", square1.frequencyShadow, 768);
  EXPECT_FALSE("Square1 write", square1.sweepNegate);
  EXPECT_EQ("Square1 write", square1.sweepPeriod, (uint3)0);
  EXPECT_FALSE("Square1 write", square1.sweepEnable);

  // same as previous, but length is initially 0 and becomes 64
  square1.power(true);
  square1.enable = true;
  square1.envelopeVolume = 5;
  square1.length = 0;
  square1.envelopeDirection = true;
  square1.write(0xff14, 0b11110011);
  EXPECT_TRUE("Square1 write", square1.enable);
  EXPECT_TRUE("Square1 write", square1.counter);
  EXPECT_EQ("Square1 write", square1.frequency, (uint11)0b01100000000);
  EXPECT_EQ("Square1 write", square1.period, 2560u);
  EXPECT_EQ("Square1 write", square1.envelopePeriod, (uint3)0);
  EXPECT_EQ("Square1 write", square1.volume, (uint4)5);
  EXPECT_EQ("Square1 write", square1.length, 64u);
  EXPECT_EQ("Square1 write", square1.frequencyShadow, 768);
  EXPECT_FALSE("Square1 write", square1.sweepNegate);
  EXPECT_EQ("Square1 write", square1.sweepPeriod, (uint3)0);
  EXPECT_FALSE("Square1 write", square1.sweepEnable);

  // same as previous, but length is initially 0 and becomes 63 because of
  // apu.phase
  GameBoy::apu.power();
  square1.power(true);
  GameBoy::apu.phase = 1;
  square1.enable = true;
  square1.envelopeVolume = 5;
  square1.length = 0;
  square1.envelopeDirection = true;
  square1.write(0xff14, 0b11110011);
  EXPECT_TRUE("Square1 write", square1.enable);
  EXPECT_TRUE("Square1 write", square1.counter);
  EXPECT_EQ("Square1 write", square1.frequency, (uint11)0b01100000000);
  EXPECT_EQ("Square1 write", square1.period, 2560u);
  EXPECT_EQ("Square1 write", square1.envelopePeriod, (uint3)0);
  EXPECT_EQ("Square1 write", square1.volume, (uint4)5);
  EXPECT_EQ("Square1 write", square1.length, 63u);
  EXPECT_EQ("Square1 write", square1.frequencyShadow, 768);
  EXPECT_FALSE("Square1 write", square1.sweepNegate);
  EXPECT_EQ("Square1 write", square1.sweepPeriod, (uint3)0);
  EXPECT_FALSE("Square1 write", square1.sweepEnable);
  // clear phase
  GameBoy::apu.power();

  // data.bit(6) is true, data.bit(7) is false, enable stays true
  square1.power(true);
  square1.length = 1;
  square1.enable = true;
  square1.write(0xff14, 0b01110011);
  EXPECT_TRUE("Square1 write", square1.enable);
  EXPECT_TRUE("Square1 write", square1.counter);
  EXPECT_EQ("Square1 write", square1.frequency, (uint11)0b01100000000);
  EXPECT_EQ("Square1 write", square1.length, 1u);

  // same as previous, but GameBoy::apu.phase = 1, so enable becomes false
  GameBoy::apu.power();
  square1.power(true);
  GameBoy::apu.phase = 1;
  square1.length = 1;
  square1.enable = true;
  square1.write(0xff14, 0b01110011);

  EXPECT_FALSE("Square1 write", square1.enable);
  EXPECT_TRUE("Square1 write", square1.counter);
  EXPECT_EQ("Square1 write", square1.frequency, (uint11)0b01100000000);
  EXPECT_EQ("Square1 write", square1.length, 0u);
  // clear phase
  GameBoy::apu.power();
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
  TestRead();
  TestWrite();
  TestPower();
}
}
