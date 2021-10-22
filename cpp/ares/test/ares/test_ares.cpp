#include "test_ares.hpp"
#include "gb/test_gb.hpp"

namespace test {
namespace ares {
namespace gb {
extern void TestGB();
}

void TestAres() { gb::TestGB(); }
} // namespace ares
} // namespace test