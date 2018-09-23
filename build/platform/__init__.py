import os

from . import windows

if os.name == "nt":
  cmake_generator = windows.cmake_generator
  setup_environment = windows.setup_environment
  cargo_target = windows.cargo_target
else:
  assert False