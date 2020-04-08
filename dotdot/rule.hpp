#ifndef RULE_H
#define RULE_H

#include "dotdot.h"
#include "util.hpp"
#include "yaml-cpp/yaml.h"

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

  Item(ItemType Type, path Path) : Type(Type), Path(std::move(Path)) {}

  explicit Item(const path &parent, const std::string &relative = (std::string &)"") {
	Type = ItemType::File;
	std::string relativePath{};

	if (relative.ends_with("/")) {
	  relativePath = relative.substr(0, relative.size() - 1);
	  Type = ItemType::Dir;
	}

	Path = parent / path{relativePath};
  }

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
using ItemsType = std::vector<Item>;

inline Rules ResolveDirs(const std::vector<path> &dirs);
inline void ResolveDir(const path &dir, Rules &rulesOut);
inline Rule ResolveFile(const path &file);
inline void ResolveNode(const YAML::Node &node, ItemsType &itemsOut);

inline Rules ResolveDirs(const std::vector<path> &dirs) {
  Rules rules{};
  for (const auto &dir:dirs) {
	if (!fs::exists(dir))
	  continue;
	ResolveDir(dir, rules);
  }
  return rules;
}

inline void ResolveDir(const path &dir, Rules &rulesOut) {
  for (const auto &item : fs::recursive_directory_iterator{dir}) {
	if (!item.is_regular_file() || item.path().extension().string() != ".yml") {
	  continue;
	}

	auto rule = ResolveFile(item.path());
	rulesOut.push_back(rule);
  }
}

inline Rule ResolveFile(const path &file) {
  std::cout << file << "\n";
  Rule rule{.Name=file.stem().string(), .Items=ItemsType{}};
  auto node = YAML::Load(file);

  for (auto it = node.begin(); it != node.end(); ++it) {
	std::cout << it->as<std::string>() << "\n";
  }
  std::cout << node["root"].as<std::string>() << "\n";

//  ResolveNode(node, rule.Items);

  #ifdef WINDOWS
  if (node["windows"]) {
	ResolveNode(node["windows"], rule.Items);
  }
  #elif LINUX
  if (node["linux"]) {
	ResolveNode(node["linux"], rule.Items);
  }
  #endif

  return rule;
}

inline void ResolveNode(const YAML::Node &node, ItemsType &itemsOut) {
  std::cout << node.Type() << "\n";
  std::cout << (node.Type() == YAML::NodeType::Scalar) << "\n";

  const path root = NormalizePath(node["root"].as<std::string>());
  if (node["include"]) {
	auto include = node["include"].as<std::vector<std::string >>();
	for (const auto &child : include) {
	  itemsOut.emplace_back(root, child);
	}
  } else {
	itemsOut.emplace_back(root);
  }
}
}
#endif // RULE_H
