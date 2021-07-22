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

    inline bool Copy(const MappersType &mappers) {
        for (const auto &[src, dst, type] : mappers) {
            if (!fs::exists(src)) {
                std::cout << "skipped " << src << " src not exists" << "\n";
                return false;
            }
            if (fs::exists(dst)) {
                std::cout << "skipped " << dst << " dst exists" << "\n";
                return false;
            }

            fs::create_directories(src.parent_path());
            fs::create_directories(dst.parent_path());

            if (type == ItemType::File) {
                fs::copy_file(src, dst, fs::copy_options::skip_existing);
            } else {
                fs::copy(src, dst, fs::copy_options::recursive | fs::copy_options::skip_existing);
            }
            std::cout << "copied " << src << " -> " << dst << "\n";
        }
        return true;
    }

    inline void SoftLink(const MappersType &mappers) {
        for (const auto &[link, to, type] : mappers) {
            if (fs::exists(link)) {
                std::cout << link << " exists, skip " << link << "\n";
                return;
            }

            if (!fs::exists(to)) {
                if (type == ItemType::File) {
                    // create a file
                    std::ofstream{to};
                } else {
                    fs::create_directories(to);
                }
            }

            fs::create_directories(link.parent_path());
            fs::create_directories(to.parent_path());

            if (type == ItemType::File) {
                fs::create_symlink(to, link);
            } else {
                fs::create_directory_symlink(to, link);
            }
            std::cout << "linked " << link << " -> " << to << "\n";
        }
    }

}

#endif //DOTDOT_DOTDOT_DOTDOT_FILES_HPP
