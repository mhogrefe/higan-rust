#include "test_ares.hpp"
#include "component/test_component.hpp"
#include "gb/test_gb.hpp"

namespace test {
namespace ares {
namespace component {
extern void TestComponent();
}
namespace gb {
extern void TestGB();
}

void TestAres() {
  component::TestComponent();
  gb::TestGB();
}
} // namespace ares
} // namespace test