namespace square2 {

void TestDacEnable() {
  APU::Square2 square2;
  square2.power(true);

  square2.envelopeVolume = 0;
  square2.envelopeDirection = false;
  EXPECT_FALSE("Square1 dacEnable", square2.dacEnable());

  square2.envelopeVolume = 3;
  square2.envelopeDirection = false;
  EXPECT_TRUE("Square1 dacEnable", square2.dacEnable());

  square2.envelopeVolume = 0;
  square2.envelopeDirection = true;
  EXPECT_TRUE("Square1 dacEnable", square2.dacEnable());

  square2.envelopeVolume = 3;
  square2.envelopeDirection = true;
  EXPECT_TRUE("Square1 dacEnable", square2.dacEnable());
}

void TestAll() { TestDacEnable(); }
}