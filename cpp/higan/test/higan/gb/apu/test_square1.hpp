#include "higan/gb/gb.hpp"
#include "test/test-utils.hpp"
#include <vector>

using GameBoy::APU;

void TestDacEnable() {
  APU::Square1 square1;
  square1.power(true);

  square1.envelopeVolume = 0;
  square1.envelopeDirection = false;
  EXPECT_EQ("Square1 dacEnable", square1.dacEnable(), false);

  square1.envelopeVolume = 3;
  square1.envelopeDirection = false;
  EXPECT_EQ("Square1 dacEnable", square1.dacEnable(), true);

  square1.envelopeVolume = 0;
  square1.envelopeDirection = true;
  EXPECT_EQ("Square1 dacEnable", square1.dacEnable(), true);

  square1.envelopeVolume = 3;
  square1.envelopeDirection = true;
  EXPECT_EQ("Square1 dacEnable", square1.dacEnable(), true);
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

void TestPower() {
  APU::Square1 square1;
  square1.length = 0;
  square1.power(true);
  EXPECT_EQ("Testing Square1 power with initializeLength true", square1.length,
            64u);

  square1.length = 0;
  square1.power(false);
  EXPECT_EQ("Testing Square1 power with initializeLength false", square1.length,
            0u);
}

void TestSquare1() {
  TestDacEnable();
  TestRun();
  TestPower();
}
