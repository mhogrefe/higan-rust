#include "test_sm83.hpp"

#include "ares/gb/gb.hpp"
#include "test/test-utils.hpp"

namespace test {
namespace ares {
namespace component {
namespace processor {
namespace sm83 {

void TestPair() {
  {
    ::ares::SM83::Registers::Pair pair;
    pair.word = 123;
    EXPECT_EQ("SM83 Pair", pair.word, (n16)123);
  }
  {
    ::ares::SM83::Registers::Pair pair;
    pair.word = 0x12ab;
    EXPECT_EQ("SM83 Pair", pair.byte.hi, (n8)0x12);
    EXPECT_EQ("SM83 Pair", pair.byte.lo, (n8)0xab);

    pair.byte.hi = 0x34;
    EXPECT_EQ("SM83 Pair", pair.byte.hi, (n8)0x34);
    EXPECT_EQ("SM83 Pair", pair.byte.lo, (n8)0xab);
    EXPECT_EQ("SM83 Pair", pair.word, (n16)0x34ab);

    pair.byte.lo = 0xcd;
    EXPECT_EQ("SM83 Pair", pair.byte.hi, (n8)0x34);
    EXPECT_EQ("SM83 Pair", pair.byte.lo, (n8)0xcd);
    EXPECT_EQ("SM83 Pair", pair.word, (n16)0x34cd);
  }
}

// #include "test_algorithms.cpp"
// #include "test_registers.cpp"

void TestSM83() {
  TestPair();
  // algorithms::TestAll();
  // registers::TestAll();
}
} // namespace sm83
} // namespace processor
} // namespace component
} // namespace ares
} // namespace test