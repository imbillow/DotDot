#include "dotdot.h"
#include "CLI11.hpp"


int main()
{
  CLI::App app = {.app_description = "Backup dotfiles", .app_name="dotdot"};
  app.add_flag("backup");
  app.add_flag("restore");
  return 0;
}
