#include "test_lr35902.hpp"

#include "higan/gb/gb.hpp"
#include "test/test-utils.hpp"

namespace test {
namespace higan {
namespace processor {
namespace lr35902 {

#include "test_registers.cpp"

void TestLR35902() { registers::TestAll(); }
}
}
}
}