#ifndef UTIL_HPP
#define UTIL_HPP
#include "dotdot.h"

namespace Dotdot {
template<typename T>
std::ostream &operator<<(std::ostream &os, const std::vector<std::vector<T>> &v) {
  using namespace std;

  if (v.empty()) {
	return os << "[]";
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
	return os << "[]";
  }

  os << "[\n";
  copy(v.begin(), v.end(), ostream_iterator<T>(os, ",\n"));
  os << "]";
  return os;
}

using std::filesystem::path;
namespace fs = std::filesystem;

path GetHomePath() {
  auto home = getenv("HOME");
  if (!home)
	throw std::exception();
  return path{home};
}

path NormalizePath(const std::string &ph) {
  if (ph.starts_with("~")) {
	return GetHomePath().concat(ph.substr(1));
  }
  return ph;
}

}
#endif // UTIL_HPP
