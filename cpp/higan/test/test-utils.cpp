#include "test-utils.h"

#include <sstream>
#include <stdexcept>
#include <string>

template <typename T> std::string ToString(T x) {
  std::stringstream ss;
  ss << x;
  return ss.str();
}

std::string ToString(uint8_t i) {
  std::stringstream ss;
  ss << unsigned(i);
  return ss.str();
}

std::string ToString(int8_t i) {
  std::stringstream ss;
  ss << signed(i);
  return ss.str();
}

template <typename T> std::string ToString(const std::vector<T> &xs) {
  std::stringstream ss;
  ss << '[';
  bool first_time = true;
  for (const T &x : xs) {
    if (first_time) {
      first_time = false;
    } else {
      ss << ", ";
    }
    ss << ToString(x);
  }
  ss << ']';
  return ss.str();
}

template <typename CAUSE, typename T>
void EXPECT_EQ(const CAUSE &cause, const T &a, const T &b) {
  if (a != b) {
    std::stringstream ss;
    ss << ToString(cause) << ": " << ToString(a) << " is not equal to "
       << ToString(b);
    throw std::runtime_error(ss.str());
  }
}

template <typename CAUSE, typename T>
void EXPECT_NOT_EQ(const CAUSE &cause, const T &a, const T &b) {
  if (a == b) {
    std::stringstream ss;
    ss << ToString(cause) << ": " << ToString(a) << " is equal to "
       << ToString(b);
    throw std::runtime_error(ss.str());
  }
}

template <typename CAUSE> void EXPECT_TRUE(const CAUSE &cause, bool b) {
  if (!b) {
    std::stringstream ss;
    ss << ToString(cause) << ": value should be true";
    throw std::runtime_error(ss.str());
  }
}

template <typename CAUSE> void EXPECT_FALSE(const CAUSE &cause, bool b) {
  if (b) {
    std::stringstream ss;
    ss << ToString(cause) << ": value should be false";
    throw std::runtime_error(ss.str());
  }
}

template <typename CAUSE> void FAIL(const CAUSE &cause) {
  std::stringstream ss;
  ss << ToString(cause) << ": failure";
  throw std::runtime_error(ss.str());
}

template <typename CAUSE>
void SHOULD_THROW(std::function<void(void)> f, const CAUSE &cause) {
  bool caught = false;
  try {
    f();
  } catch (const std::runtime_error &e) {
    caught = true;
  }
  if (!caught) {
    FAIL(cause);
  }
}
