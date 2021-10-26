namespace io {
n8 read_helper(APU *apu, n16 address) { return apu->readIO(2, address, 0xff); }

void PowerAndZeroPattern(APU::Wave *wave) {
  wave->power(true);
  for (int i = 0; i < 16; ++i) {
    wave->pattern[i] = 0;
  }
}

void TestReadIO() {
  // Noise
  APU apu;

  apu.noise.power(true);
  EXPECT_EQ("Noise read", read_helper(&apu, 0), (n8)0xff);

  apu.noise.power(true);
  EXPECT_EQ("Noise read", read_helper(&apu, 0xff1f), (n8)0b11111111);

  apu.noise.power(true);
  EXPECT_EQ("Noise read", read_helper(&apu, 0xff20), (n8)0b11111111);

  apu.noise.power(true);
  apu.noise.envelopeVolume = (n4)0b1011;
  apu.noise.envelopeDirection = true;
  apu.noise.envelopeFrequency = (n3)0b010;
  EXPECT_EQ("Noise read", read_helper(&apu, 0xff21), (n8)0b10111010);

  apu.noise.power(true);
  apu.noise.frequency = (n4)0b1011;
  apu.noise.narrow = true;
  apu.noise.divisor = (n3)0b010;
  EXPECT_EQ("Noise read", read_helper(&apu, 0xff22), (n8)0b10111010);

  apu.noise.power(true);
  apu.noise.counter = false;
  EXPECT_EQ("Noise read", read_helper(&apu, 0xff23), (n8)0b10111111);

  // Sequencer
  apu.sequencer.power();
  EXPECT_EQ("Sequencer read", read_helper(&apu, 0), (n8)0xff);

  apu.sequencer.power();
  apu.sequencer.leftEnable = true;
  apu.sequencer.leftVolume = (n3)0b010;
  apu.sequencer.rightEnable = false;
  apu.sequencer.rightVolume = (n3)0b101;
  EXPECT_EQ("Sequencer read", read_helper(&apu, 0xff24), (n8)0b10100101);

  apu.sequencer.power();
  apu.sequencer.noise.leftEnable = true;
  apu.sequencer.wave.leftEnable = false;
  apu.sequencer.square2.leftEnable = true;
  apu.sequencer.square1.leftEnable = false;
  apu.sequencer.noise.rightEnable = false;
  apu.sequencer.wave.rightEnable = true;
  apu.sequencer.square2.rightEnable = false;
  apu.sequencer.square1.rightEnable = true;
  EXPECT_EQ("Sequencer read", read_helper(&apu, 0xff25), (n8)0b10100101);

  apu.sequencer.power();
  apu.sequencer.enable = true;
  apu.noise.enable = false;
  apu.wave.enable = true;
  apu.square2.enable = false;
  apu.square1.enable = true;
  EXPECT_EQ("Sequencer read", read_helper(&apu, 0xff26), (n8)0b11110101);

  apu.square1.power(true);
  apu.square2.power(true);
  apu.wave.power(true);
  apu.noise.power(true);

  // Square 1
  apu.square1.power(true);
  EXPECT_EQ("Square1 read", read_helper(&apu, 0), (n8)0xff);

  apu.square1.power(true);
  apu.square1.sweepFrequency = (n3)0b101;
  apu.square1.sweepDirection = true;
  apu.square1.sweepShift = (n3)0b010;
  EXPECT_EQ("Square1 read", read_helper(&apu, 0xff10), (n8)0b11011010);

  apu.square1.power(true);
  apu.square1.duty = (n2)0b01;
  EXPECT_EQ("Square1 read", read_helper(&apu, 0xff11), (n8)0b01111111);

  apu.square1.power(true);
  apu.square1.envelopeVolume = (n4)0b1011;
  apu.square1.envelopeDirection = true;
  apu.square1.envelopeFrequency = (n3)0b010;
  EXPECT_EQ("Square1 read", read_helper(&apu, 0xff12), (n8)0b10111010);

  apu.square1.power(true);
  EXPECT_EQ("Square1 read", read_helper(&apu, 0xff13), (n8)0b11111111);

  apu.square1.power(true);
  apu.square1.counter = false;
  EXPECT_EQ("Square1 read", read_helper(&apu, 0xff14), (n8)0b10111111);

  // Square 2
  apu.square2.power(true);
  EXPECT_EQ("Square2 read", read_helper(&apu, 0), (n8)0xff);

  apu.square2.power(true);
  EXPECT_EQ("Square2 read", read_helper(&apu, 0xff15), (n8)0b11111111);

  apu.square2.power(true);
  apu.square2.duty = (n2)0b01;
  EXPECT_EQ("Square2 read", read_helper(&apu, 0xff16), (n8)0b01111111);

  apu.square2.power(true);
  apu.square2.envelopeVolume = (n4)0b1011;
  apu.square2.envelopeDirection = true;
  apu.square2.envelopeFrequency = (n3)0b010;
  EXPECT_EQ("Square2 read", read_helper(&apu, 0xff17), (n8)0b10111010);

  apu.square2.power(true);
  EXPECT_EQ("Square2 read", read_helper(&apu, 0xff18), (n8)0b11111111);

  apu.square2.power(true);
  apu.square2.counter = false;
  EXPECT_EQ("Square2 read", read_helper(&apu, 0xff19), (n8)0b10111111);

  // Wave
  PowerAndZeroPattern(&apu.wave);
  EXPECT_EQ("Wave read", read_helper(&apu, 0), (n8)0xff);

  PowerAndZeroPattern(&apu.wave);
  apu.wave.dacEnable = false;
  EXPECT_EQ("Wave read", read_helper(&apu, 0xff1a), (n8)0b01111111);

  PowerAndZeroPattern(&apu.wave);
  EXPECT_EQ("Wave read", read_helper(&apu, 0xff1b), (n8)0b11111111);

  PowerAndZeroPattern(&apu.wave);
  apu.wave.volume = 0b10;
  EXPECT_EQ("Wave read", read_helper(&apu, 0xff1c), (n8)0b11011111);

  PowerAndZeroPattern(&apu.wave);
  EXPECT_EQ("Wave read", read_helper(&apu, 0xff1d), (n8)0b11111111);

  PowerAndZeroPattern(&apu.wave);
  apu.wave.counter = false;
  EXPECT_EQ("Wave read", read_helper(&apu, 0xff1e), (n8)0b10111111);

  // Model::GameBoyColor() is false, patternHold is zero
  PowerAndZeroPattern(&apu.wave);
  apu.wave.enable = true;
  apu.wave.patternHold = 0;
  apu.wave.patternOffset = 3;
  apu.wave.pattern[1] = 0xab;
  EXPECT_EQ("Wave read", read_helper(&apu, 0xff3a), (n8)0xff);

  // Model::GameBoyColor() is false, patternHold is nonzero
  PowerAndZeroPattern(&apu.wave);
  apu.wave.enable = true;
  apu.wave.patternHold = 5;
  apu.wave.patternOffset = 3;
  apu.wave.pattern[1] = 0xab;
  EXPECT_EQ("Wave read", read_helper(&apu, 0xff3a), (n8)0xab);

  // Model::GameBoyColor() is true, patternHold is zero
  PowerAndZeroPattern(&apu.wave);
  auto old_model = ::ares::GameBoy::system.information.model;
  ::ares::GameBoy::system.information.model =
      ::ares::GameBoy::System::Model::GameBoyColor;
  apu.wave.enable = true;
  apu.wave.patternHold = 0;
  apu.wave.patternOffset = 3;
  apu.wave.pattern[1] = 0xab;
  EXPECT_EQ("Wave read", read_helper(&apu, 0xff3a), (n8)0xab);
  ::ares::GameBoy::system.information.model = old_model;

  // enable is false
  PowerAndZeroPattern(&apu.wave);
  apu.wave.enable = false;
  apu.wave.patternHold = 0;
  apu.wave.patternOffset = 3;
  apu.wave.pattern[5] = 0xab;
  EXPECT_EQ("Wave read", read_helper(&apu, 0xff35), (n8)0xab);
}

void write_helper(APU *apu, n16 address, n8 data) {
  apu->writeIO(2, address, data);
}

void write_helper_with_cycle(APU *apu, u32 cycle, n16 address, n8 data) {
  apu->writeIO(cycle, address, data);
}

void TestWriteIO() {
  APU *apu = &::ares::GameBoy::apu;
  apu->power();
  apu->sequencer.enable = true;

  // Noise
  apu->noise.power(true);
  write_helper(apu, 0xff20, 0b10110100);
  EXPECT_EQ("Noise write", apu->noise.length, 12u);

  apu->noise.power(true);
  apu->noise.enable = true;
  write_helper(apu, 0xff21, 0b10111010);
  EXPECT_EQ("Noise write", apu->noise.envelopeVolume, (n4)0b1011);

  EXPECT_TRUE("Noise write", apu->noise.envelopeDirection);
  EXPECT_EQ("Noise write", apu->noise.envelopeFrequency, (n3)0b010);
  EXPECT_TRUE("Noise write", apu->noise.enable);

  apu->noise.power(true);
  apu->noise.enable = true;
  write_helper(apu, 0xff21, 0);
  EXPECT_EQ("Noise write", apu->noise.envelopeVolume, (n4)0);
  EXPECT_FALSE("Noise write", apu->noise.envelopeDirection);
  EXPECT_EQ("Noise write", apu->noise.envelopeFrequency, (n3)0);
  EXPECT_FALSE("Noise write", apu->noise.enable);

  apu->noise.power(true);
  apu->noise.enable = true;
  write_helper(apu, 0xff22, 0b10111010);
  EXPECT_EQ("Noise write", apu->noise.frequency, (n4)0b1011);
  EXPECT_TRUE("Noise write", apu->noise.narrow);
  EXPECT_EQ("Noise write", apu->noise.divisor, (n3)0b010);

  // data.bit(6) is false, data.bit(7) is true
  apu->noise.power(true);
  write_helper_with_cycle(apu, 4, 0xff23, 0b10110011);
  EXPECT_FALSE("Noise write", apu->noise.enable);
  EXPECT_FALSE("Noise write", apu->noise.counter);
  EXPECT_EQ("Noise write", apu->noise.envelopePeriod, (n3)0);
  EXPECT_EQ("Noise write", apu->noise.lfsr, (n15)0x7fff);
  EXPECT_EQ("Noise write", apu->noise.volume, (n4)0);
  EXPECT_EQ("Noise write", apu->noise.length, 64u);

  // data.bit(6) is false, data.bit(7) is false. Length stays 0
  apu->noise.power(true);
  apu->noise.enable = true;
  apu->noise.length = 0;
  write_helper_with_cycle(apu, 4, 0xff23, 0b00110011);
  EXPECT_TRUE("Noise write", apu->noise.enable);
  EXPECT_FALSE("Noise write", apu->noise.counter);
  EXPECT_EQ("Noise write", apu->noise.length, 0u);

  // data.bit(6) is true, data.bit(7) is true, enable stays true
  apu->noise.power(true);
  apu->noise.length = 1;
  apu->noise.enable = true;
  apu->noise.envelopeVolume = 5;
  apu->noise.envelopeDirection = true;
  write_helper_with_cycle(apu, 4, 0xff23, 0b11110011);
  EXPECT_TRUE("Noise write", apu->noise.enable);
  EXPECT_TRUE("Noise write", apu->noise.counter);
  EXPECT_EQ("Noise write", apu->noise.envelopePeriod, (n3)0);
  EXPECT_EQ("Noise write", apu->noise.lfsr, (n15)0x7fff);
  EXPECT_EQ("Noise write", apu->noise.volume, (n4)5);
  EXPECT_EQ("Noise write", apu->noise.length, 1u);

  // same as previous, but length is initially 0 and becomes 64
  apu->noise.power(true);
  apu->noise.enable = true;
  apu->noise.envelopeVolume = 5;
  apu->noise.length = 0;
  apu->noise.envelopeDirection = true;
  write_helper_with_cycle(apu, 4, 0xff23, 0b11110011);
  EXPECT_TRUE("Noise write", apu->noise.enable);
  EXPECT_TRUE("Noise write", apu->noise.counter);
  EXPECT_EQ("Noise write", apu->noise.envelopePeriod, (n3)0);
  EXPECT_EQ("Noise write", apu->noise.lfsr, (n15)0x7fff);
  EXPECT_EQ("Noise write", apu->noise.volume, (n4)5);
  EXPECT_EQ("Noise write", apu->noise.length, 64u);

  // same as previous, but length is initially 0 and becomes 63 because of
  // apu->phase
  apu->power();
  apu->noise.power(true);
  apu->sequencer.enable = true;
  apu->phase = 1;
  apu->noise.enable = true;
  apu->noise.envelopeVolume = 5;
  apu->noise.length = 0;
  apu->noise.envelopeDirection = true;
  write_helper_with_cycle(apu, 4, 0xff23, 0b11110011);
  EXPECT_TRUE("Noise write", apu->noise.enable);
  EXPECT_TRUE("Noise write", apu->noise.counter);
  EXPECT_EQ("Noise write", apu->noise.envelopePeriod, (n3)0);
  EXPECT_EQ("Noise write", apu->noise.lfsr, (n15)0x7fff);
  EXPECT_EQ("Noise write", apu->noise.volume, (n4)5);
  EXPECT_EQ("Noise write", apu->noise.length, 63u);
  // clear phase
  apu->power();

  // data.bit(6) is true, data.bit(7) is false, enable stays true
  apu->noise.power(true);
  apu->sequencer.enable = true;
  apu->noise.length = 1;
  apu->noise.enable = true;
  write_helper_with_cycle(apu, 4, 0xff23, 0b01110011);
  EXPECT_TRUE("Noise write", apu->noise.enable);
  EXPECT_TRUE("Noise write", apu->noise.counter);
  EXPECT_EQ("Noise write", apu->noise.length, 1u);

  // same as previous, but GameBoy::apu->phase = 1
  apu->power();
  apu->noise.power(true);
  apu->sequencer.enable = true;
  apu->phase = 1;
  apu->noise.length = 1;
  apu->noise.enable = true;
  write_helper_with_cycle(apu, 4, 0xff23, 0b01110011);

  EXPECT_FALSE("Noise write", apu->noise.enable);
  EXPECT_TRUE("Noise write", apu->noise.counter);
  EXPECT_EQ("Noise write", apu->noise.length, 0u);
  // clear phase
  apu->power();

  // Sequencer
  apu->sequencer.power();
  apu->sequencer.enable = true;
  write_helper(apu, 0xff24, 0b10100101);
  EXPECT_TRUE("Sequencer write", apu->sequencer.leftEnable);
  EXPECT_EQ("Sequencer write", apu->sequencer.leftVolume, (n3)0b010);
  EXPECT_FALSE("Sequencer write", apu->sequencer.rightEnable);
  EXPECT_EQ("Sequencer write", apu->sequencer.rightVolume, (n3)0b101);

  apu->sequencer.power();
  apu->sequencer.enable = true;
  write_helper(apu, 0xff25, 0b10100101);
  EXPECT_TRUE("Sequencer write", apu->sequencer.noise.leftEnable);
  EXPECT_FALSE("Sequencer write", apu->sequencer.wave.leftEnable);
  EXPECT_TRUE("Sequencer write", apu->sequencer.square2.leftEnable);
  EXPECT_FALSE("Sequencer write", apu->sequencer.square1.leftEnable);
  EXPECT_FALSE("Sequencer write", apu->sequencer.noise.rightEnable);
  EXPECT_TRUE("Sequencer write", apu->sequencer.wave.rightEnable);
  EXPECT_FALSE("Sequencer write", apu->sequencer.square2.rightEnable);
  EXPECT_TRUE("Sequencer write", apu->sequencer.square1.rightEnable);

  // enable and data.bit(7) both false, so nothing happens
  apu->sequencer.power();
  apu->square1.power(true);
  apu->square1.period = 5;
  write_helper_with_cycle(apu, 4, 0xff26, 0);
  EXPECT_EQ("Sequencer write", apu->square1.period, 5u);
  apu->square1.power(true);

  // enable and data.bit(7) both true, so nothing happens
  apu->sequencer.power();
  apu->square1.power(true);
  apu->square1.period = 5;
  apu->sequencer.enable = true;
  write_helper_with_cycle(apu, 4, 0xff26, 0b10000000);
  EXPECT_EQ("Sequencer write", apu->square1.period, 5u);
  apu->square1.power(true);

  // enable is false and data.bit(7) is true, so apu phase is set to 0
  apu->sequencer.power();
  apu->power();
  apu->phase = 5;
  write_helper_with_cycle(apu, 4, 0xff26, 0b10000000);
  EXPECT_EQ("Sequencer write", apu->phase, (n3)0);
  apu->power();

  // enable is true, data.bit(7) is false, and model is not GBC, so APU
  // components are powered without initializing length
  apu->sequencer.power();
  apu->square1.power(true);
  apu->square1.period = 5;
  apu->square1.length = 5;
  apu->sequencer.enable = true;
  write_helper_with_cycle(apu, 4, 0xff26, 0);
  EXPECT_EQ("Sequencer write", apu->square1.period, 0u);
  EXPECT_EQ("Sequencer write", apu->square1.length, 5u);
  apu->square1.power(true);

  // enable is true, data.bit(7) is false, and model is GBC, so APU components
  // are powered, initializing length
  apu->sequencer.power();
  auto old_model = ::ares::GameBoy::system.information.model;
  ::ares::GameBoy::system.information.model =
      ::ares::GameBoy::System::Model::GameBoyColor;
  apu->square1.power(true);
  apu->square1.period = 5;
  apu->square1.length = 5;
  apu->sequencer.enable = true;
  write_helper_with_cycle(apu, 4, 0xff26, 0);
  EXPECT_EQ("Sequencer write", apu->square1.period, 0u);
  EXPECT_EQ("Sequencer write", apu->square1.length, 64u);
  ::ares::GameBoy::system.information.model = old_model;
  apu->square1.power(true);

  // Square 1
  apu->square1.power(true);
  apu->square1.enable = true;
  apu->square1.sweepEnable = true;
  apu->square1.sweepNegate = true;
  apu->sequencer.enable = true;

  write_helper(apu, 0xff10, 0b11011010);
  EXPECT_EQ("Square1 write", apu->square1.sweepFrequency, (n3)0b101);
  EXPECT_TRUE("Square1 write", apu->square1.sweepDirection);
  EXPECT_EQ("Square1 write", apu->square1.sweepShift, (n3)0b010);
  EXPECT_TRUE("Square1 write", apu->square1.enable);

  apu->square1.power(true);
  apu->square1.enable = true;
  apu->square1.sweepEnable = true;
  apu->square1.sweepNegate = true;
  write_helper(apu, 0xff10, 0b11010010);
  EXPECT_EQ("Square1 write", apu->square1.sweepFrequency, (n3)0b101);
  EXPECT_FALSE("Square1 write", apu->square1.sweepDirection);
  EXPECT_EQ("Square1 write", apu->square1.sweepShift, (n3)0b010);
  EXPECT_FALSE("Square1 write", apu->square1.enable);

  apu->square1.power(true);
  write_helper(apu, 0xff11, 0b01110010);
  EXPECT_EQ("Square1 write", apu->square1.duty, (n2)0b01);
  EXPECT_EQ("Square1 write", apu->square1.length, 14u);

  apu->square1.power(true);
  apu->square1.enable = true;
  write_helper(apu, 0xff12, 0b10111010);
  EXPECT_EQ("Square1 write", apu->square1.envelopeVolume, (n4)0b1011);
  EXPECT_TRUE("Square1 write", apu->square1.envelopeDirection);
  EXPECT_EQ("Square1 write", apu->square1.envelopeFrequency, (n3)0b010);
  EXPECT_TRUE("Square1 write", apu->square1.enable);

  apu->square1.power(true);
  apu->square1.enable = true;
  write_helper(apu, 0xff12, 0);
  EXPECT_EQ("Square1 write", apu->square1.envelopeVolume, (n4)0);
  EXPECT_FALSE("Square1 write", apu->square1.envelopeDirection);
  EXPECT_EQ("Square1 write", apu->square1.envelopeFrequency, (n3)0);
  EXPECT_FALSE("Square1 write", apu->square1.enable);

  apu->square1.power(true);
  write_helper(apu, 0xff13, 0b10110100);
  EXPECT_EQ("Square1 write", apu->square1.frequency, (n11)0b10110100);

  // data.bit(6) is false, data.bit(7) is true
  apu->square1.power(true);
  write_helper_with_cycle(apu, 4, 0xff14, 0b10110011);
  EXPECT_FALSE("Square1 write", apu->square1.enable);
  EXPECT_FALSE("Square1 write", apu->square1.counter);
  EXPECT_EQ("Square1 write", apu->square1.frequency, (n11)0b01100000000);
  EXPECT_EQ("Square1 write", apu->square1.period, 2560u);
  EXPECT_EQ("Square1 write", apu->square1.envelopePeriod, (n3)0);
  EXPECT_EQ("Square1 write", apu->square1.volume, (n4)0);
  EXPECT_EQ("Square1 write", apu->square1.length, 64u);
  EXPECT_EQ("Square1 write", apu->square1.frequencyShadow, 768);
  EXPECT_FALSE("Square1 write", apu->square1.sweepNegate);
  EXPECT_EQ("Square1 write", apu->square1.sweepPeriod, (n3)0);
  EXPECT_FALSE("Square1 write", apu->square1.sweepEnable);

  // data.bit(6) is false, data.bit(7) is false. Length stays 0
  apu->square1.power(true);
  apu->square1.enable = true;
  apu->square1.length = 0;
  write_helper_with_cycle(apu, 4, 0xff14, 0b00110011);
  EXPECT_TRUE("Square1 write", apu->square1.enable);
  EXPECT_FALSE("Square1 write", apu->square1.counter);
  EXPECT_EQ("Square1 write", apu->square1.frequency, (n11)0b01100000000);
  EXPECT_EQ("Square1 write", apu->square1.length, 0u);

  // data.bit(6) is true, data.bit(7) is true, enable stays true
  apu->square1.power(true);
  apu->square1.length = 1;
  apu->square1.enable = true;
  apu->square1.envelopeVolume = 5;
  apu->square1.envelopeDirection = true;
  write_helper_with_cycle(apu, 4, 0xff14, 0b11110011);
  EXPECT_TRUE("Square1 write", apu->square1.enable);
  EXPECT_TRUE("Square1 write", apu->square1.counter);
  EXPECT_EQ("Square1 write", apu->square1.frequency, (n11)0b01100000000);
  EXPECT_EQ("Square1 write", apu->square1.period, 2560u);
  EXPECT_EQ("Square1 write", apu->square1.envelopePeriod, (n3)0);
  EXPECT_EQ("Square1 write", apu->square1.volume, (n4)5);
  EXPECT_EQ("Square1 write", apu->square1.length, 1u);
  EXPECT_EQ("Square1 write", apu->square1.frequencyShadow, 768);
  EXPECT_FALSE("Square1 write", apu->square1.sweepNegate);
  EXPECT_EQ("Square1 write", apu->square1.sweepPeriod, (n3)0);
  EXPECT_FALSE("Square1 write", apu->square1.sweepEnable);

  // same as previous, but length is initially 0 and becomes 64
  apu->square1.power(true);
  apu->square1.enable = true;
  apu->square1.envelopeVolume = 5;
  apu->square1.length = 0;
  apu->square1.envelopeDirection = true;
  write_helper_with_cycle(apu, 4, 0xff14, 0b11110011);
  EXPECT_TRUE("Square1 write", apu->square1.enable);
  EXPECT_TRUE("Square1 write", apu->square1.counter);
  EXPECT_EQ("Square1 write", apu->square1.frequency, (n11)0b01100000000);
  EXPECT_EQ("Square1 write", apu->square1.period, 2560u);
  EXPECT_EQ("Square1 write", apu->square1.envelopePeriod, (n3)0);
  EXPECT_EQ("Square1 write", apu->square1.volume, (n4)5);
  EXPECT_EQ("Square1 write", apu->square1.length, 64u);
  EXPECT_EQ("Square1 write", apu->square1.frequencyShadow, 768);
  EXPECT_FALSE("Square1 write", apu->square1.sweepNegate);
  EXPECT_EQ("Square1 write", apu->square1.sweepPeriod, (n3)0);
  EXPECT_FALSE("Square1 write", apu->square1.sweepEnable);

  // same as previous, but length is initially 0 and becomes 63 because of
  // apu->phase
  apu->power();
  apu->square1.power(true);
  apu->phase = 1;
  apu->square1.enable = true;
  apu->square1.envelopeVolume = 5;
  apu->square1.length = 0;
  apu->square1.envelopeDirection = true;
  apu->sequencer.enable = true;
  write_helper_with_cycle(apu, 4, 0xff14, 0b11110011);
  EXPECT_TRUE("Square1 write", apu->square1.enable);
  EXPECT_TRUE("Square1 write", apu->square1.counter);
  EXPECT_EQ("Square1 write", apu->square1.frequency, (n11)0b01100000000);
  EXPECT_EQ("Square1 write", apu->square1.period, 2560u);
  EXPECT_EQ("Square1 write", apu->square1.envelopePeriod, (n3)0);
  EXPECT_EQ("Square1 write", apu->square1.volume, (n4)5);
  EXPECT_EQ("Square1 write", apu->square1.length, 63u);
  EXPECT_EQ("Square1 write", apu->square1.frequencyShadow, 768);
  EXPECT_FALSE("Square1 write", apu->square1.sweepNegate);
  EXPECT_EQ("Square1 write", apu->square1.sweepPeriod, (n3)0);
  EXPECT_FALSE("Square1 write", apu->square1.sweepEnable);
  // clear phase
  apu->power();

  // data.bit(6) is true, data.bit(7) is false, enable stays true
  apu->square1.power(true);
  apu->square1.length = 1;
  apu->square1.enable = true;
  apu->sequencer.enable = true;
  write_helper_with_cycle(apu, 4, 0xff14, 0b01110011);
  EXPECT_TRUE("Square1 write", apu->square1.enable);
  EXPECT_TRUE("Square1 write", apu->square1.counter);
  EXPECT_EQ("Square1 write", apu->square1.frequency, (n11)0b01100000000);
  EXPECT_EQ("Square1 write", apu->square1.length, 1u);

  // same as previous, but apu->phase = 1, so enable becomes false
  apu->power();
  apu->square1.power(true);
  apu->phase = 1;
  apu->square1.length = 1;
  apu->square1.enable = true;
  apu->sequencer.enable = true;
  write_helper_with_cycle(apu, 4, 0xff14, 0b01110011);

  EXPECT_FALSE("Square1 write", apu->square1.enable);
  EXPECT_TRUE("Square1 write", apu->square1.counter);
  EXPECT_EQ("Square1 write", apu->square1.frequency, (n11)0b01100000000);
  EXPECT_EQ("Square1 write", apu->square1.length, 0u);
  // clear phase
  apu->power();

  // Square 2
  apu->sequencer.enable = true;
  apu->square2.power(true);
  write_helper(apu, 0xff16, 0b01110010);
  EXPECT_EQ("Square2 write", apu->square2.duty, (n2)0b01);
  EXPECT_EQ("Square2 write", apu->square2.length, 14u);

  apu->square2.power(true);
  apu->square2.enable = true;
  write_helper(apu, 0xff17, 0b10111010);
  EXPECT_EQ("Square2 write", apu->square2.envelopeVolume, (n4)0b1011);
  EXPECT_TRUE("Square2 write", apu->square2.envelopeDirection);
  EXPECT_EQ("Square2 write", apu->square2.envelopeFrequency, (n3)0b010);
  EXPECT_TRUE("Square2 write", apu->square2.enable);

  apu->square2.power(true);
  apu->square2.enable = true;
  write_helper(apu, 0xff17, 0);
  EXPECT_EQ("Square2 write", apu->square2.envelopeVolume, (n4)0);
  EXPECT_FALSE("Square2 write", apu->square2.envelopeDirection);
  EXPECT_EQ("Square2 write", apu->square2.envelopeFrequency, (n3)0);
  EXPECT_FALSE("Square2 write", apu->square2.enable);

  apu->square2.power(true);
  write_helper(apu, 0xff18, 0b10110100);
  EXPECT_EQ("Square2 write", apu->square2.frequency, (n11)0b10110100);

  // data.bit(6) is false, data.bit(7) is true
  apu->square2.power(true);
  write_helper_with_cycle(apu, 4, 0xff19, 0b10110011);
  EXPECT_FALSE("Square2 write", apu->square2.enable);
  EXPECT_FALSE("Square2 write", apu->square2.counter);
  EXPECT_EQ("Square2 write", apu->square2.frequency, (n11)0b01100000000);
  EXPECT_EQ("Square2 write", apu->square2.period, 2560u);
  EXPECT_EQ("Square2 write", apu->square2.envelopePeriod, (n3)0);
  EXPECT_EQ("Square2 write", apu->square2.volume, (n4)0);
  EXPECT_EQ("Square2 write", apu->square2.length, 64u);

  // data.bit(6) is false, data.bit(7) is false. Length stays 0
  apu->square2.power(true);
  apu->square2.enable = true;
  apu->square2.length = 0;
  write_helper_with_cycle(apu, 4, 0xff19, 0b00110011);
  EXPECT_TRUE("Square2 write", apu->square2.enable);
  EXPECT_FALSE("Square2 write", apu->square2.counter);
  EXPECT_EQ("Square2 write", apu->square2.frequency, (n11)0b01100000000);
  EXPECT_EQ("Square2 write", apu->square2.length, 0u);

  // data.bit(6) is true, data.bit(7) is true, enable stays true
  apu->square2.power(true);
  apu->square2.length = 1;
  apu->square2.enable = true;
  apu->square2.envelopeVolume = 5;
  apu->square2.envelopeDirection = true;
  write_helper_with_cycle(apu, 4, 0xff19, 0b11110011);
  EXPECT_TRUE("Square2 write", apu->square2.enable);
  EXPECT_TRUE("Square2 write", apu->square2.counter);
  EXPECT_EQ("Square2 write", apu->square2.frequency, (n11)0b01100000000);
  EXPECT_EQ("Square2 write", apu->square2.period, 2560u);
  EXPECT_EQ("Square2 write", apu->square2.envelopePeriod, (n3)0);
  EXPECT_EQ("Square2 write", apu->square2.volume, (n4)5);
  EXPECT_EQ("Square2 write", apu->square2.length, 1u);

  // same as previous, but length is initially 0 and becomes 64
  apu->square2.power(true);
  apu->square2.enable = true;
  apu->square2.envelopeVolume = 5;
  apu->square2.length = 0;
  apu->square2.envelopeDirection = true;
  write_helper_with_cycle(apu, 4, 0xff19, 0b11110011);
  EXPECT_TRUE("Square2 write", apu->square2.enable);
  EXPECT_TRUE("Square2 write", apu->square2.counter);
  EXPECT_EQ("Square2 write", apu->square2.frequency, (n11)0b01100000000);
  EXPECT_EQ("Square2 write", apu->square2.period, 2560u);
  EXPECT_EQ("Square2 write", apu->square2.envelopePeriod, (n3)0);
  EXPECT_EQ("Square2 write", apu->square2.volume, (n4)5);
  EXPECT_EQ("Square2 write", apu->square2.length, 64u);

  // same as previous, but length is initially 0 and becomes 63 because of
  // apu->phase
  apu->power();
  apu->square2.power(true);
  apu->phase = 1;
  apu->square2.enable = true;
  apu->square2.envelopeVolume = 5;
  apu->square2.length = 0;
  apu->square2.envelopeDirection = true;
  apu->sequencer.enable = true;
  write_helper_with_cycle(apu, 4, 0xff19, 0b11110011);
  EXPECT_TRUE("Square2 write", apu->square2.enable);
  EXPECT_TRUE("Square2 write", apu->square2.counter);
  EXPECT_EQ("Square2 write", apu->square2.frequency, (n11)0b01100000000);
  EXPECT_EQ("Square2 write", apu->square2.period, 2560u);
  EXPECT_EQ("Square2 write", apu->square2.envelopePeriod, (n3)0);
  EXPECT_EQ("Square2 write", apu->square2.volume, (n4)5);
  EXPECT_EQ("Square2 write", apu->square2.length, 63u);
  // clear phase
  apu->power();

  // data.bit(6) is true, data.bit(7) is false, enable stays true
  apu->square2.power(true);
  apu->square2.length = 1;
  apu->square2.enable = true;
  apu->sequencer.enable = true;
  write_helper_with_cycle(apu, 4, 0xff19, 0b01110011);
  EXPECT_TRUE("Square2 write", apu->square2.enable);
  EXPECT_TRUE("Square2 write", apu->square2.counter);
  EXPECT_EQ("Square2 write", apu->square2.frequency, (n11)0b01100000000);
  EXPECT_EQ("Square2 write", apu->square2.length, 1u);

  // same as previous, but apu->phase = 1, so enable becomes false
  apu->power();
  apu->square2.power(true);
  apu->phase = 1;
  apu->square2.length = 1;
  apu->square2.enable = true;
  apu->sequencer.enable = true;
  write_helper_with_cycle(apu, 4, 0xff19, 0b01110011);

  EXPECT_FALSE("Square2 write", apu->square2.enable);
  EXPECT_TRUE("Square2 write", apu->square2.counter);
  EXPECT_EQ("Square2 write", apu->square2.frequency, (n11)0b01100000000);
  EXPECT_EQ("Square2 write", apu->square2.length, 0u);
  // clear phase
  apu->power();

  // Wave
  PowerAndZeroPattern(&apu->wave);
  apu->wave.dacEnable = false;
  apu->sequencer.enable = true;
  write_helper(apu, 0xff1a, 0b10000000);
  EXPECT_TRUE("Wave write", apu->wave.dacEnable);

  PowerAndZeroPattern(&apu->wave);
  apu->wave.dacEnable = true;
  apu->wave.enable = true;
  write_helper(apu, 0xff1a, 0);
  EXPECT_FALSE("Wave write", apu->wave.dacEnable);
  EXPECT_FALSE("Wave write", apu->wave.enable);

  PowerAndZeroPattern(&apu->wave);
  write_helper(apu, 0xff1b, 100);
  EXPECT_EQ("Wave write", apu->wave.length, 156u);

  PowerAndZeroPattern(&apu->wave);
  write_helper(apu, 0xff1c, 0b01000000);
  EXPECT_EQ("Wave write", apu->wave.volume, (n2)0b10);

  PowerAndZeroPattern(&apu->wave);
  write_helper(apu, 0xff1d, 0b10101010);
  EXPECT_EQ("Wave write", apu->wave.frequency, (n11)0b00010101010);

  // apu->phase.bit(0) is false so enable stays true
  PowerAndZeroPattern(&apu->wave);
  apu->wave.enable = true;
  apu->wave.length = 1;
  write_helper_with_cycle(apu, 4, 0xff1e, 0b01000101);
  EXPECT_TRUE("Wave write", apu->wave.enable);
  EXPECT_EQ("Wave write", apu->wave.length, 1u);
  EXPECT_TRUE("Wave write", apu->wave.counter);
  EXPECT_EQ("Wave write", apu->wave.frequency, (n11)0b10100000000);

  // apu->phase.bit(0) is true so enable becomes false
  apu->power();
  PowerAndZeroPattern(&apu->wave);
  apu->phase = 1;
  apu->wave.enable = true;
  apu->wave.length = 1;
  apu->sequencer.enable = true;
  write_helper_with_cycle(apu, 4, 0xff1e, 0b01000000);
  EXPECT_FALSE("Wave write", apu->wave.enable);
  EXPECT_EQ("Wave write", apu->wave.length, 0u);
  // clear phase
  apu->power();

  // pattern[0] corrupted
  apu->phase = 1;
  PowerAndZeroPattern(&apu->wave);
  for (int i = 0; i < 16; ++i) {
    apu->wave.pattern[i] = i;
  }
  apu->wave.patternHold = 5;
  apu->wave.patternOffset = 2;
  apu->sequencer.enable = true;
  write_helper_with_cycle(apu, 4, 0xff1e, 0b11000101);
  EXPECT_EQ("Wave write", apu->wave.pattern[0], (n8)1);
  EXPECT_EQ("Wave write", apu->wave.pattern[1], (n8)1);
  EXPECT_EQ("Wave write", apu->wave.pattern[2], (n8)2);
  EXPECT_EQ("Wave write", apu->wave.pattern[3], (n8)3);
  EXPECT_EQ("Wave write", apu->wave.pattern[4], (n8)4);

  // pattern[0-3] corrupted
  PowerAndZeroPattern(&apu->wave);
  for (int i = 0; i < 16; ++i) {
    apu->wave.pattern[i] = i;
  }
  apu->wave.patternHold = 5;
  apu->wave.patternOffset = 9;
  write_helper_with_cycle(apu, 4, 0xff1e, 0b11000101);
  EXPECT_EQ("Wave write", apu->wave.pattern[0], (n8)4);
  EXPECT_EQ("Wave write", apu->wave.pattern[1], (n8)5);
  EXPECT_EQ("Wave write", apu->wave.pattern[2], (n8)6);
  EXPECT_EQ("Wave write", apu->wave.pattern[3], (n8)7);
  EXPECT_EQ("Wave write", apu->wave.pattern[4], (n8)4);

  // no corruption when system is Game Boy Color
  PowerAndZeroPattern(&apu->wave);
  old_model = ::ares::GameBoy::system.information.model;
  ::ares::GameBoy::system.information.model =
      ::ares::GameBoy::System::Model::GameBoyColor;
  for (int i = 0; i < 16; ++i) {
    apu->wave.pattern[i] = i;
  }
  apu->wave.patternHold = 5;
  apu->wave.patternOffset = 9;
  write_helper_with_cycle(apu, 4, 0xff1e, 0b11000101);
  EXPECT_EQ("Wave write", apu->wave.pattern[0], (n8)0);
  EXPECT_EQ("Wave write", apu->wave.pattern[1], (n8)1);
  EXPECT_EQ("Wave write", apu->wave.pattern[2], (n8)2);
  EXPECT_EQ("Wave write", apu->wave.pattern[3], (n8)3);
  EXPECT_EQ("Wave write", apu->wave.pattern[4], (n8)4);
  ::ares::GameBoy::system.information.model = old_model;

  // no corruption when data.bit(7) is false
  PowerAndZeroPattern(&apu->wave);
  for (int i = 0; i < 16; ++i) {
    apu->wave.pattern[i] = i;
  }
  apu->wave.patternHold = 5;
  apu->wave.patternOffset = 9;
  write_helper_with_cycle(apu, 4, 0xff1e, 0b01000101);
  EXPECT_EQ("Wave write", apu->wave.pattern[0], (n8)0);
  EXPECT_EQ("Wave write", apu->wave.pattern[1], (n8)1);
  EXPECT_EQ("Wave write", apu->wave.pattern[2], (n8)2);
  EXPECT_EQ("Wave write", apu->wave.pattern[3], (n8)3);
  EXPECT_EQ("Wave write", apu->wave.pattern[4], (n8)4);

  PowerAndZeroPattern(&apu->wave);
  apu->wave.patternOffset = 9;
  apu->wave.frequency = 1;
  apu->wave.patternSample = 1;
  apu->wave.patternHold = 5;
  apu->wave.dacEnable = true;
  write_helper_with_cycle(apu, 4, 0xff1e, 0b11000000);
  EXPECT_TRUE("Wave write", apu->wave.enable);
  EXPECT_EQ("Wave write", apu->wave.period, 2049u);
  EXPECT_EQ("Wave write", apu->wave.patternOffset, (n5)0);
  EXPECT_EQ("Wave write", apu->wave.patternSample, (n4)0);
  EXPECT_EQ("Wave write", apu->wave.patternHold, 0u);

  PowerAndZeroPattern(&apu->wave);
  write_helper_with_cycle(apu, 4, 0xff1e, 0b11000000);
  EXPECT_EQ("Wave write", apu->wave.length, 255u);

  PowerAndZeroPattern(&apu->wave);
  apu->phase = 0;
  apu->wave.length = 100;
  write_helper_with_cycle(apu, 4, 0xff1e, 0b11000000);
  EXPECT_EQ("Wave write", apu->wave.length, 100u);

  apu->power();
  PowerAndZeroPattern(&apu->wave);
  apu->phase = 1;
  apu->wave.length = 100;
  apu->sequencer.enable = true;
  write_helper_with_cycle(apu, 4, 0xff1e, 0b11000000);
  EXPECT_EQ("Wave write", apu->wave.length, 99u);
  // clear phase
  apu->power();

  // passes up to here

  PowerAndZeroPattern(&apu->wave);
  apu->sequencer.enable = true;
  write_helper(apu, 0xff3a, 123);
  EXPECT_EQ("Wave write", apu->wave.pattern[0xa], (n8)123);

  PowerAndZeroPattern(&apu->wave);
  apu->wave.patternOffset = 5;
  apu->wave.enable = true;
  apu->wave.patternHold = 5;
  write_helper(apu, 0xff3a, 123);
  EXPECT_EQ("Wave write", apu->wave.pattern[2], (n8)123);

  apu->power();
  PowerAndZeroPattern(&apu->wave);
  apu->phase = 1;
  apu->wave.patternOffset = 5;
  apu->wave.enable = true;
  apu->sequencer.enable = true;
  write_helper(apu, 0xff3a, 123);
  EXPECT_EQ("Wave write", apu->wave.pattern[2], (n8)0);
  // clear phase
  apu->power();
}

void TestAll() {
  TestReadIO();
  TestWriteIO();
}
} // namespace io
