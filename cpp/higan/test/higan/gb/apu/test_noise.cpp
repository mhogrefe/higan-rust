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

void TestAll() { TestDacEnable(); }
}
