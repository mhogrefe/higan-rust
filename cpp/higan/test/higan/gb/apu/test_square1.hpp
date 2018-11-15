#include "higan/gb/gb.hpp"
#include "test/test-utils.hpp"
#include <vector>

using GameBoy::APU;

void TestDacEnable() {
  APU::Square1 square1;
  square1.power(true);

  square1.envelopeVolume = 0;
  square1.envelopeDirection = false;
  EXPECT_FALSE("Square1 dacEnable", square1.dacEnable());

  square1.envelopeVolume = 3;
  square1.envelopeDirection = false;
  EXPECT_TRUE("Square1 dacEnable", square1.dacEnable());

  square1.envelopeVolume = 0;
  square1.envelopeDirection = true;
  EXPECT_TRUE("Square1 dacEnable", square1.dacEnable());

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

void TestPower() {
  APU::Square1 square1;
  square1.length = 0;
  square1.power(true);
  EXPECT_EQ("Square1 power", square1.length, 64u);

  square1.length = 0;
  square1.power(false);
  EXPECT_EQ("Square1 power", square1.length, 0u);
}

void TestSquare1() {
  TestDacEnable();
  TestRun();
  TestSweep();
  TestPower();
}
