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

inline void Copy(const MappersType &mappers) {
  ForeachMapper(mappers,
				[](const path &src, const path &dst, const ItemType &type) {
                  if (!fs::exists(src)){
                      return;
                  }
				  std::cout << src << " -> " << dst << " copied\n";

				  if (type == ItemType::File) {
					fs::copy_file(src, dst, fs::copy_options::skip_existing);
				  } else {
					fs::copy(src, dst, fs::copy_options::recursive | fs::copy_options::skip_existing);
				  }
				});
}

inline void SoftLink(const MappersType &mappers) {
  ForeachMapper(mappers,
				[](const path &link, const path &to, const ItemType &type) {
                  if (!fs::exists(to)){
                    if (type==ItemType::File) {
                      std::ofstream{to.c_str()};
                    } else {
                     fs::create_directories(to);
                    }
                  }

				  std::cout << link << " -> " << to << " linked\n";

				  if (type == ItemType::File) {
					fs::create_symlink(to, link);
                     //CreateSymbolicLinkA(link.c_str(), to.c_str(), 0);
				  } else {
					fs::create_directory_symlink(to, link);
                     //CreateSymbolicLinkA(link.c_str(), to.c_str(), 1);
				  }
				});
}

inline void ForeachMapper(const MappersType &mappers,
                          const std::function<void(const path &, const path &, const ItemType &)> &fn) {
  for (const auto &[from, to, type] : mappers) {
	fs::create_directories(from.parent_path());
	fs::create_directories(to.parent_path());
	fn(from, to, type);
  }
}

}

#endif //DOTDOT_DOTDOT_DOTDOT_FILES_HPP
