#ifndef UTIL_HPP
#define UTIL_HPP
#include "dotdot.h"

namespace Dotdot {
template<typename T>
std::ostream &operator<<(std::ostream &os, const std::vector<std::vector<T>> &v) {
  using namespace std;

  if (v.empty()) {
	return os;
  }

  os << "[";
  for (auto i = 0; i < v.size(); ++i) {
	os << v[i] << "\n";
  }
  os << "]";
  return os;
}

template<typename T>
std::ostream &operator<<(std::ostream &os, const std::vector<T> &v) {
  using namespace std;

  if (v.empty()) {
	return os;
  }

  os << "[\n";
  copy(v.begin(), v.end(), ostream_iterator<T>(os, ",\n"));
  os << "]";
  return os;
}
}
#endif // UTIL_HPP
