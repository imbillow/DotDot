#ifndef WORKING_H
#define WORKING_H
#include "rule.hpp"
#include "files.hpp"

namespace Dotdot {

inline void Backups(const Rules &rules, const path &dst);
inline void Restores(const Rules &rules, const path &src);

inline void Backup(const ItemsType &items, const path &dst);
inline void Restore(const ItemsType &items, const path &src);

inline void Backups(const Rules &rules, const path &dst) {
  for (const auto &rule : rules) {
	Backup(rule.Items, dst / rule.Name);
  }
}

inline void Restores(const Rules &rules, const path &src) {
  for (const auto &rule : rules) {
	Restore(rule.Items, src / rule.Name);
  }
}

inline void Backup(const ItemsType &items, const path &dst) {
  auto home = GetHomePath();
  Files::MappersType mappers{};
  std::transform(items.begin(), items.end(), std::back_inserter(mappers),
				 [&](const Item &it) {
				   //    (from, to, type)
				   return std::make_tuple(home / it.Path, dst / it.Path, it.Type);
				 });
  Files::Copy(mappers);

  for (const auto &it : items) {
	fs::remove_all(home / it.Path);
  }

  Files::SoftLink(mappers);
}

inline void Restore(const ItemsType &items, const path &src) {
  auto home = GetHomePath();

  for (const auto &it : items) {
	fs::remove(home / it.Path);
  }

  Files::MappersType mappers{};
  std::transform(items.begin(), items.end(), std::back_inserter(mappers),
				 [&](const Item &it) {
				   return std::make_tuple(src / it.Path, home / it.Path, it.Type);
				 });
  Files::Copy(mappers);
}
}
#endif // WORKING_H
