#include "test_apu.hpp"

#include "ares/gb/gb.hpp"
#include "test/test-utils.hpp"
#include <vector>

using ares::GameBoy::APU;

namespace test {
namespace ares {
namespace gb {
namespace apu {

#include "test_square1.cpp"

void TestAPU() { square1::TestAll(); }
} // namespace apu
} // namespace gb
} // namespace ares
} // namespace test