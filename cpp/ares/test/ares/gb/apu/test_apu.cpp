#include "test_apu.hpp"

#include "ares/gb/gb.hpp"
#include "test/test-utils.hpp"
#include <vector>

using ares::GameBoy::APU;

namespace test {
namespace ares {
namespace gb {
namespace apu {

#include "test_noise.cpp"
#include "test_sequencer.cpp"
#include "test_square1.cpp"
#include "test_square2.cpp"
#include "test_wave.cpp"

void TestRunSequencer() {
  APU::Sequencer sequencer;

  sequencer.power();
  sequencer.center = 1;
  sequencer.left = 2;
  sequencer.right = 3;
  sequencer.run();
  EXPECT_EQ("Sequencer run", sequencer.center, (i16)0);
  EXPECT_EQ("Sequencer run", sequencer.left, (i16)0);
  EXPECT_EQ("Sequencer run", sequencer.right, (i16)0);

  sequencer.power();
  ::ares::GameBoy::apu.square1.power(true);
  ::ares::GameBoy::apu.square2.power(true);
  ::ares::GameBoy::apu.wave.power(true);
  ::ares::GameBoy::apu.noise.power(true);
  sequencer.enable = true;
  ::ares::GameBoy::apu.square1.output = 15;
  ::ares::GameBoy::apu.square2.output = 15;
  ::ares::GameBoy::apu.wave.output = 15;
  ::ares::GameBoy::apu.noise.output = 15;
  sequencer.run();
  EXPECT_EQ("Sequencer run", sequencer.center, (i16)7168);
  EXPECT_EQ("Sequencer run", sequencer.left, (i16)-1024);
  EXPECT_EQ("Sequencer run", sequencer.right, (i16)-1024);

  sequencer.power();
  ::ares::GameBoy::apu.square1.power(true);
  ::ares::GameBoy::apu.square2.power(true);
  ::ares::GameBoy::apu.wave.power(true);
  ::ares::GameBoy::apu.noise.power(true);
  sequencer.enable = true;
  ::ares::GameBoy::apu.square1.output = 15;
  ::ares::GameBoy::apu.square2.output = 15;
  ::ares::GameBoy::apu.wave.output = 15;
  ::ares::GameBoy::apu.noise.output = 15;
  sequencer.square1.leftEnable = true;
  sequencer.square2.leftEnable = true;
  sequencer.wave.leftEnable = true;
  sequencer.noise.leftEnable = true;
  sequencer.run();
  EXPECT_EQ("Sequencer run", sequencer.center, (i16)7168);
  EXPECT_EQ("Sequencer run", sequencer.left, (i16)896);
  EXPECT_EQ("Sequencer run", sequencer.right, (i16)-1024);

  sequencer.power();
  ::ares::GameBoy::apu.square1.power(true);
  ::ares::GameBoy::apu.square2.power(true);
  ::ares::GameBoy::apu.wave.power(true);
  ::ares::GameBoy::apu.noise.power(true);
  sequencer.enable = true;
  ::ares::GameBoy::apu.square1.output = 1;
  ::ares::GameBoy::apu.square2.output = 1;
  ::ares::GameBoy::apu.wave.output = 1;
  ::ares::GameBoy::apu.noise.output = 1;
  sequencer.square1.rightEnable = true;
  sequencer.square2.rightEnable = true;
  sequencer.wave.rightEnable = true;
  sequencer.noise.rightEnable = true;
  sequencer.run();
  EXPECT_EQ("Sequencer run", sequencer.center, (i16)-7168);
  EXPECT_EQ("Sequencer run", sequencer.left, (i16)-1024);
  EXPECT_EQ("Sequencer run", sequencer.right, (i16)-896);

  ::ares::GameBoy::apu.square1.power(true);
  ::ares::GameBoy::apu.square2.power(true);
  ::ares::GameBoy::apu.wave.power(true);
  ::ares::GameBoy::apu.noise.power(true);
}

void TestAPU() {
  TestRunSequencer();
  noise::TestAll();
  sequencer::TestAll();
  square1::TestAll();
  square2::TestAll();
  wave::TestAll();
}

} // namespace apu
} // namespace gb
} // namespace ares
} // namespace test