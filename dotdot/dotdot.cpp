#include "clipp.h"
#include "dotdot.h"
#include "rule.hpp"
#include "working.hpp"

namespace Dotdot {
enum class WorkingMode { Backup, Restore, Resolve };
}

int main(int argc, char *argv[]) {
  using namespace clipp;
  using namespace Dotdot;

  WorkingMode mode{WorkingMode::Resolve};
  std::string dataDir{"~/Dotdot/Dotfiles"};
  std::vector<std::string> rulesDir{};
  bool optShowHelp{false};

  const auto cli = (
	  command("backup").set(mode, WorkingMode::Backup)
		  | command("restore").set(mode, WorkingMode::Restore)
		  | command("resolve").set(mode, WorkingMode::Resolve)
		  | command("help").set(optShowHelp, true),
		  (option("-d", "--directory") & value("Directory", dataDir)) % "Directory of dotfiles",
		  (option("-r", "--rules") & values("Directory", rulesDir)) % "Sets addition rules directory"
  );

  if (const auto ret = !parse(argc, argv, cli) || optShowHelp) {
	if (ret) {
	  std::cerr << "Failed parse arguments: \n";
	  for (auto i = 1; i < argc; ++i) {
		std::cerr << argv[i] << "\n";
	  }
	}
	std::cout << "Usage:\n" << usage_lines(cli, argv[0]) << "\n\n"
			  << "Options:\n" << documentation(cli) << "\n\n";
	return 0;
  }

  const auto lst = std::unique(rulesDir.begin(), rulesDir.end());
  rulesDir.erase(lst, rulesDir.end());

  std::vector<path> rulesPath{};
  std::transform(rulesDir.begin(), rulesDir.end(), std::back_inserter(rulesPath),
				 [](const std::string &dir) {
				   return NormalizePath(dir);
				 });

  const auto rules = ResolveDirs(rulesPath);

  if (rules.empty()) {
	std::cout << "Empty rules!\n";
	return 0;
  }

  dataDir = Dotdot::NormalizePath(dataDir);

  switch (mode) {
  case WorkingMode::Backup:Backups(rules, dataDir);
	break;
  case WorkingMode::Restore:Restores(rules, dataDir);
	break;
  case WorkingMode::Resolve: std::cout << rules << "\n";
	break;
  default:
	// ignore
	;
  }

  return 0;
}
