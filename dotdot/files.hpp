//
// Created by iov on 4/8/20.
//

#ifndef DOTDOT_DOTDOT_DOTDOT_FILES_HPP
#define DOTDOT_DOTDOT_DOTDOT_FILES_HPP
#include "dotdot.h"
#include "rule.hpp"

namespace Dotdot::Files {
using MapperType = std::tuple<path, path, ItemType>;
using MappersType = std::vector<MapperType>;

void ForeachMapper(const MappersType &mappers,
				   const std::function<void(const path &, const path &, const ItemType &)> &fn);
void Copy(const MappersType &mappers);
void SoftLink(const MappersType &mappers);

void Copy(const MappersType &mappers) {
  ForeachMapper(mappers,
				[](const path &src, const path &dst, const ItemType &type) {
				  std::cout << src << " -> " << dst << "\n";
				  if (type == ItemType::File) {
					fs::copy_file(src, dst, fs::copy_options::skip_existing);
				  } else {
					fs::copy(src, dst, fs::copy_options::recursive | fs::copy_options::skip_existing);
				  }
				});
}

void SoftLink(const MappersType &mappers) {
  ForeachMapper(mappers,
				[](const path &link, const path &to, const ItemType &type) {
				  if (type == ItemType::File) {
					fs::create_symlink(to, link);
				  } else {
					fs::create_directory_symlink(to, link);
				  }
				});
}

void ForeachMapper(const MappersType &mappers,
				   const std::function<void(const path &, const path &, const ItemType &)> &fn) {
  for (const auto &[to, link, type] : mappers) {
	fs::create_directories(to.parent_path());
	fs::create_directories(link.parent_path());
	fn(to, link, type);
  }
}

}

#endif //DOTDOT_DOTDOT_DOTDOT_FILES_HPP
