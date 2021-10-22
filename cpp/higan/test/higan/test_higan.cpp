#include "test_higan.hpp"
#include "gb/test_gb.hpp"
#include "processor/test_processor.hpp"

namespace test {
namespace higan {
namespace gb {
extern void TestGB();
}
namespace processor {
extern void TestProcessor();
}

void TestHigan() {
  gb::TestGB();
  processor::TestProcessor();
}
} // namespace higan
} // namespace test
