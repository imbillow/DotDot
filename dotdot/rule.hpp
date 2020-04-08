#ifndef RULE_H
#define RULE_H

#include "dotdot.h"
#include "util.hpp"
#include <filesystem>

namespace Dotdot
{
    using std::filesystem::path;

    enum class ItemType
    {
        File,
        Dir,
    };

    inline std::ostream& operator<<(std::ostream& os, const ItemType& obj)
    {
        switch (obj)
        {
        case ItemType::File: os << "File";
            break;
        case ItemType::Dir: os << "Dir";
            break;
        default: ;
        }
        return os;
    }

    struct Item
    {
        ItemType Type;
        path Path;

        friend std::ostream& operator<<(std::ostream& os, const Item& obj)
        {
            return os
                << "Type: " << obj.Type
                << ", Path: " << obj.Path;
        }
    };

    struct Rule
    {
        std::string Name;
        std::vector<Item> Items;

        friend std::ostream& operator<<(std::ostream& os, const Rule& obj)
        {
            return os
                << "Name: " << obj.Name
                << ", Items: " << obj.Items;
        }
    };

    using Rules = std::vector<Rule>;

    inline Rules ResolveDirs(const std::vector<path>& dir)
    {
        // TODO

        return {};
    }

    inline void ResolveDir(const path& dir, const Rules& rulesOut)
    {
        // TODO

    }

    inline Rule ResolveFile(const path& file)
    {
        // TODO

        return {};
    }
}
#endif // RULE_H
