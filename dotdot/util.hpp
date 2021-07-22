#ifndef UTIL_HPP
#define UTIL_HPP
// ReSharper disable once CppInconsistentNaming
#define _CRT_SECURE_NO_WARNINGS

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

    inline path GetHomePath() {
        char *home{};
#ifdef _WIN32
        home = getenv("USERPROFILE");
#elif __linux__
        home = getenv("HOME");
#endif

        if (!home)
            throw std::exception();
        return path{home};
    }

    inline path NormalizePath(const std::string &ph) {
        if (ph.starts_with("~")) {
            return GetHomePath().concat(ph.substr(1));
        }
        return ph;
    }

}
#endif // UTIL_HPP
