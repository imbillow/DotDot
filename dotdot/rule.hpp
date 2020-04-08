#ifndef RULE_H
#define RULE_H

#include "dotdot.h"
#include "util.hpp"
#include <filesystem>

namespace Dotdot {

namespace fs = std::filesystem;
using std::filesystem::path;

enum class ItemType {
  File,
  Dir,
};

inline std::ostream &operator<<(std::ostream &os, const ItemType &obj) {
  switch (obj) {
  case ItemType::File: os << "File";
	break;
  case ItemType::Dir: os << "Dir";
	break;
  default:;
  }
  return os;
}

struct Item {
  ItemType Type;
  path Path;

  friend std::ostream &operator<<(std::ostream &os, const Item &obj) {
	return os
		<< "Type: " << obj.Type
		<< ", Path: " << obj.Path;
  }
};

struct Rule {
  std::string Name;
  std::vector<Item> Items;

  friend std::ostream &operator<<(std::ostream &os, const Rule &obj) {
	return os
		<< "Name: " << obj.Name
		<< ", Items: " << obj.Items;
  }
};

using Rules = std::vector<Rule>;

inline Rules ResolveDirs(const std::vector<path> &dirs);
inline void ResolveDir(const path &dir, Rules &rulesOut);
inline Rule ResolveFile(const path &file);

inline Rules ResolveDirs(const std::vector<path> &dirs) {
  Rules rules{};
  for (const auto &dir:dirs) {
	ResolveDir(dir, rules);
  }
  return rules;
}

using namespace std;

inline void ResolveDir(const path &dir, Rules &rulesOut) {
  for (const auto &item : fs::recursive_directory_iterator{dir}) {
	if (!item.is_regular_file() || item.path().extension().string() != "yml") {
	  continue;
	}

	auto rule = ResolveFile(item.path());
	rulesOut.push_back(rule);
  }
}

inline Rule ResolveFile(const path &file) {
  return {};
}
}
#endif // RULE_H
