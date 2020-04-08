#ifndef WORKING_H
#define WORKING_H
#include "rule.hpp"

namespace Dotdot {
inline void Backup(const Rule &rule, const path &dst);
inline void Restore(const Rule &rule, const path &src);
inline void Backup(const Rules &rules, const path &dst);
inline void Restore(const Rules &rules, const path &src);

inline void Backup(const Rules &rules, const path &dst) {
  for (const auto &rule : rules) {
	Backup(rule, dst);
  }
}

inline void Restore(const Rules &rules, const path &src) {
  for (const auto &rule : rules) {
	Restore(rule, src);
  }
}
void Dotdot::Backup(const Rule &rule, const path &dst) {
  // TODO
  std::cout << "Not impl\n";
}
void Dotdot::Restore(const Rule &rule, const path &src) {
  // TODO
  std::cout << "Not impl\n";
}
}
#endif // WORKING_H
