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
  std::vector<int16> output;
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
  EXPECT_EQ("Square2 run", square2.output, (int16)0);

  square2.power(true);
  square2.period = 0;
  square2.dutyOutput = true;
  square2.enable = false;
  square2.volume = 10;
  square2.run();
  EXPECT_EQ("Square2 run", square2.output, (int16)0);

  square2.power(true);
  square2.period = 0;
  square2.dutyOutput = true;
  square2.enable = true;
  square2.volume = 10;
  square2.run();
  EXPECT_EQ("Square2 run", square2.output, (int16)10);

  square2.power(true);
  square2.period = 30;
  square2.dutyOutput = false;
  square2.enable = false;
  square2.volume = 10;
  square2.run();
  EXPECT_EQ("Square2 run", square2.output, (int16)0);

  square2.power(true);
  square2.period = 30;
  square2.dutyOutput = true;
  square2.enable = false;
  square2.volume = 10;
  square2.run();
  EXPECT_EQ("Square2 run", square2.output, (int16)0);

  square2.power(true);
  square2.period = 30;
  square2.dutyOutput = true;
  square2.enable = true;
  square2.volume = 10;
  square2.run();
  EXPECT_EQ("Square2 run", square2.output, (int16)10);

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
  square2.envelopeFrequency = (uint3)5;
  square2.envelopePeriod = (uint3)1;
  square2.volume = (uint4)10;
  square2.clockEnvelope();
  EXPECT_EQ("Square2 clockEnvelope", square2.envelopePeriod, (uint3)1);
  EXPECT_EQ("Square2 clockEnvelope", square2.volume, (uint4)10);

  square2.power(true);
  square2.enable = true;
  square2.envelopeFrequency = (uint3)0;
  square2.envelopePeriod = (uint3)1;
  square2.volume = (uint4)10;
  square2.clockEnvelope();
  EXPECT_EQ("Square2 clockEnvelope", square2.envelopePeriod, (uint3)1);
  EXPECT_EQ("Square2 clockEnvelope", square2.volume, (uint4)10);

  square2.power(true);
  square2.enable = true;
  square2.envelopeFrequency = (uint3)5;
  square2.envelopePeriod = (uint3)5;
  square2.volume = (uint4)10;
  square2.clockEnvelope();
  EXPECT_EQ("Square2 clockEnvelope", square2.envelopePeriod, (uint3)4);
  EXPECT_EQ("Square2 clockEnvelope", square2.volume, (uint4)10);

  square2.power(true);
  square2.enable = true;
  square2.envelopeDirection = false;
  square2.envelopeFrequency = (uint3)5;
  square2.envelopePeriod = (uint3)1;
  square2.volume = (uint4)10;
  square2.clockEnvelope();
  EXPECT_EQ("Square2 clockEnvelope", square2.envelopePeriod, (uint3)5);
  EXPECT_EQ("Square2 clockEnvelope", square2.volume, (uint4)9);

  // volume already at min
  square2.power(true);
  square2.enable = true;
  square2.envelopeDirection = false;
  square2.envelopeFrequency = (uint3)5;
  square2.envelopePeriod = (uint3)1;
  square2.volume = (uint4)0;
  square2.clockEnvelope();
  EXPECT_EQ("Square2 clockEnvelope", square2.envelopePeriod, (uint3)5);
  EXPECT_EQ("Square2 clockEnvelope", square2.volume, (uint4)0);

  square2.power(true);
  square2.enable = true;
  square2.envelopeDirection = true;
  square2.envelopeFrequency = (uint3)5;
  square2.envelopePeriod = (uint3)1;
  square2.volume = (uint4)10;
  square2.clockEnvelope();
  EXPECT_EQ("Square2 clockEnvelope", square2.envelopePeriod, (uint3)5);
  EXPECT_EQ("Square2 clockEnvelope", square2.volume, (uint4)11);

  // volume already at max
  square2.power(true);
  square2.enable = true;
  square2.envelopeDirection = true;
  square2.envelopeFrequency = (uint3)5;
  square2.envelopePeriod = (uint3)1;
  square2.volume = (uint4)15;
  square2.clockEnvelope();
  EXPECT_EQ("Square2 clockEnvelope", square2.envelopePeriod, (uint3)5);
  EXPECT_EQ("Square2 clockEnvelope", square2.volume, (uint4)15);
}

void TestRead() {
  APU::Square2 square2;

  square2.power(true);
  EXPECT_EQ("Square2 read", square2.read(0), (uint8)0xff);

  square2.power(true);
  EXPECT_EQ("Square2 read", square2.read(0xff15), (uint8)0b11111111);

  square2.power(true);
  square2.duty = (uint2)0b01;
  EXPECT_EQ("Square2 read", square2.read(0xff16), (uint8)0b01111111);

  square2.power(true);
  square2.envelopeVolume = (uint4)0b1011;
  square2.envelopeDirection = true;
  square2.envelopeFrequency = (uint3)0b010;
  EXPECT_EQ("Square2 read", square2.read(0xff17), (uint8)0b10111010);

  square2.power(true);
  EXPECT_EQ("Square2 read", square2.read(0xff18), (uint8)0b11111111);

  square2.power(true);
  square2.counter = false;
  EXPECT_EQ("Square2 read", square2.read(0xff19), (uint8)0b10111111);
}

void TestWrite() {
  APU::Square2 square2;

  square2.power(true);
  square2.write(0xff16, 0b01110010);
  EXPECT_EQ("Square2 write", square2.duty, (uint2)0b01);
  EXPECT_EQ("Square2 write", square2.length, 14u);

  square2.power(true);
  square2.enable = true;
  square2.write(0xff17, 0b10111010);
  EXPECT_EQ("Square2 write", square2.envelopeVolume, (uint4)0b1011);
  EXPECT_TRUE("Square2 write", square2.envelopeDirection);
  EXPECT_EQ("Square2 write", square2.envelopeFrequency, (uint3)0b010);
  EXPECT_TRUE("Square2 write", square2.enable);

  square2.power(true);
  square2.enable = true;
  square2.write(0xff17, 0);
  EXPECT_EQ("Square2 write", square2.envelopeVolume, (uint4)0);
  EXPECT_FALSE("Square2 write", square2.envelopeDirection);
  EXPECT_EQ("Square2 write", square2.envelopeFrequency, (uint3)0);
  EXPECT_FALSE("Square2 write", square2.enable);

  square2.power(true);
  square2.write(0xff18, 0b10110100);
  EXPECT_EQ("Square2 write", square2.frequency, (uint11)0b10110100);

  // data.bit(6) is false, data.bit(7) is true
  square2.power(true);
  square2.write(0xff19, 0b10110011);
  EXPECT_FALSE("Square2 write", square2.enable);
  EXPECT_FALSE("Square2 write", square2.counter);
  EXPECT_EQ("Square2 write", square2.frequency, (uint11)0b01100000000);
  EXPECT_EQ("Square2 write", square2.period, 2560u);
  EXPECT_EQ("Square2 write", square2.envelopePeriod, (uint3)0);
  EXPECT_EQ("Square2 write", square2.volume, (uint4)0);
  EXPECT_EQ("Square2 write", square2.length, 64u);

  // data.bit(6) is false, data.bit(7) is false. Length stays 0
  square2.power(true);
  square2.enable = true;
  square2.length = 0;
  square2.write(0xff19, 0b00110011);
  EXPECT_TRUE("Square2 write", square2.enable);
  EXPECT_FALSE("Square2 write", square2.counter);
  EXPECT_EQ("Square2 write", square2.frequency, (uint11)0b01100000000);
  EXPECT_EQ("Square2 write", square2.length, 0u);

  // data.bit(6) is true, data.bit(7) is true, enable stays true
  square2.power(true);
  square2.length = 1;
  square2.enable = true;
  square2.envelopeVolume = 5;
  square2.envelopeDirection = true;
  square2.write(0xff19, 0b11110011);
  EXPECT_TRUE("Square2 write", square2.enable);
  EXPECT_TRUE("Square2 write", square2.counter);
  EXPECT_EQ("Square2 write", square2.frequency, (uint11)0b01100000000);
  EXPECT_EQ("Square2 write", square2.period, 2560u);
  EXPECT_EQ("Square2 write", square2.envelopePeriod, (uint3)0);
  EXPECT_EQ("Square2 write", square2.volume, (uint4)5);
  EXPECT_EQ("Square2 write", square2.length, 1u);

  // same as previous, but length is initially 0 and becomes 64
  square2.power(true);
  square2.enable = true;
  square2.envelopeVolume = 5;
  square2.length = 0;
  square2.envelopeDirection = true;
  square2.write(0xff19, 0b11110011);
  EXPECT_TRUE("Square2 write", square2.enable);
  EXPECT_TRUE("Square2 write", square2.counter);
  EXPECT_EQ("Square2 write", square2.frequency, (uint11)0b01100000000);
  EXPECT_EQ("Square2 write", square2.period, 2560u);
  EXPECT_EQ("Square2 write", square2.envelopePeriod, (uint3)0);
  EXPECT_EQ("Square2 write", square2.volume, (uint4)5);
  EXPECT_EQ("Square2 write", square2.length, 64u);

  // same as previous, but length is initially 0 and becomes 63 because of
  // apu.phase
  GameBoy::apu.power();
  square2.power(true);
  GameBoy::apu.phase = 1;
  square2.enable = true;
  square2.envelopeVolume = 5;
  square2.length = 0;
  square2.envelopeDirection = true;
  square2.write(0xff19, 0b11110011);
  EXPECT_TRUE("Square2 write", square2.enable);
  EXPECT_TRUE("Square2 write", square2.counter);
  EXPECT_EQ("Square2 write", square2.frequency, (uint11)0b01100000000);
  EXPECT_EQ("Square2 write", square2.period, 2560u);
  EXPECT_EQ("Square2 write", square2.envelopePeriod, (uint3)0);
  EXPECT_EQ("Square2 write", square2.volume, (uint4)5);
  EXPECT_EQ("Square2 write", square2.length, 63u);
  // clear phase
  GameBoy::apu.power();

  // data.bit(6) is true, data.bit(7) is false, enable stays true
  square2.power(true);
  square2.length = 1;
  square2.enable = true;
  square2.write(0xff19, 0b01110011);
  EXPECT_TRUE("Square2 write", square2.enable);
  EXPECT_TRUE("Square2 write", square2.counter);
  EXPECT_EQ("Square2 write", square2.frequency, (uint11)0b01100000000);
  EXPECT_EQ("Square2 write", square2.length, 1u);

  // same as previous, but GameBoy::apu.phase = 1, so enable becomes false
  GameBoy::apu.power();
  square2.power(true);
  GameBoy::apu.phase = 1;
  square2.length = 1;
  square2.enable = true;
  square2.write(0xff19, 0b01110011);

  EXPECT_FALSE("Square2 write", square2.enable);
  EXPECT_TRUE("Square2 write", square2.counter);
  EXPECT_EQ("Square2 write", square2.frequency, (uint11)0b01100000000);
  EXPECT_EQ("Square2 write", square2.length, 0u);
  // clear phase
  GameBoy::apu.power();
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
  TestRead();
  TestWrite();
  TestPower();
}
} // namespace square2