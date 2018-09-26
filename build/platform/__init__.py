import os

from . import windows

if os.name == "nt":
  cmake_generator = windows.cmake_generator
  environment = windows.environment
  cargo_target = windows.cargo_target
  command_line_parser = windows.command_line_parser
else:
  assert False