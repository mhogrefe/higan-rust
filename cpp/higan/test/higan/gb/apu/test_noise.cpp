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

void TestAll() {
  TestDacEnable();
  TestGetPeriod();
  TestRun();
}
}
