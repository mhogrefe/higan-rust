#include "test_lr35902.hpp"

#include "higan/gb/gb.hpp"
#include "test/test-utils.hpp"

namespace test {
namespace higan {
namespace processor {
namespace lr35902 {

#include "higan/processor/lr35902/registers.cpp"

void TestPair() {
  {
    Processor::LR35902::Registers::Pair pair;
    pair.word = 123;
    EXPECT_EQ("LR35902 Pair", pair.word, (uint16)123);
  }
  {
    Processor::LR35902::Registers::Pair pair;
    pair.word = 0x12ab;
    EXPECT_EQ("LR35902 Pair", pair.byte.hi, (uint8)0x12);
    EXPECT_EQ("LR35902 Pair", pair.byte.lo, (uint8)0xab);

    pair.byte.hi = 0x34;
    EXPECT_EQ("LR35902 Pair", pair.byte.hi, (uint8)0x34);
    EXPECT_EQ("LR35902 Pair", pair.byte.lo, (uint8)0xab);
    EXPECT_EQ("LR35902 Pair", pair.word, (uint16)0x34ab);

    pair.byte.lo = 0xcd;
    EXPECT_EQ("LR35902 Pair", pair.byte.hi, (uint8)0x34);
    EXPECT_EQ("LR35902 Pair", pair.byte.lo, (uint8)0xcd);
    EXPECT_EQ("LR35902 Pair", pair.word, (uint16)0x34cd);
  }
}

#include "test_algorithms.cpp"
#include "test_registers.cpp"

void TestLR35902() {
  TestPair();
  algorithms::TestAll();
  registers::TestAll();
}
} // namespace lr35902
} // namespace processor
} // namespace higan
} // namespace test