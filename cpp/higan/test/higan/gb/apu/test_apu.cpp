#include "test_apu.hpp"

#include "higan/gb/gb.hpp"
#include "test/test-utils.hpp"
#include <vector>

using GameBoy::APU;

namespace test {
namespace higan {
namespace gb {
namespace apu {

#include "test_noise.cpp"
#include "test_sequencer.cpp"
#include "test_square1.cpp"
#include "test_square2.cpp"
#include "test_wave.cpp"

void TestAPU() {
  noise::TestAll();
  sequencer::TestAll();
  square1::TestAll();
  square2::TestAll();
  wave::TestAll();
}
}
}
}
}