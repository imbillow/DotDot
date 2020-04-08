#ifndef UTIL_HPP
#define UTIL_HPP
#include "dotdot.h"

namespace Dotdot {
template<typename T>
std::ostream &operator<<(std::ostream &os, const std::vector<std::vector<T>> &v) {
  using namespace std;

  if (v.empty()) {
	return os<< "[]";
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
	return os<< "[]";
  }

  os << "[\n";
  copy(v.begin(), v.end(), ostream_iterator<T>(os, ",\n"));
  os << "]";
  return os;
}

using std::filesystem::path;
namespace fs = std::filesystem;

path NormalizePath(const std::string &ph) {
  if (const auto home = getenv("HOME")) {
	auto homePath = path{home};
	path ret{};
	if (ph.starts_with("~")) {
	  ret = homePath.concat(ph.substr(1));
	} else {
	  ret = fs::canonical(ph);
	}
	return homePath;
  } else {
	throw std::exception();
  }
}
}
#endif // UTIL_HPP
