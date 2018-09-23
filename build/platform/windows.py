from os import path

from common import ARCH_X64, ARCH_X86

INSTALL_KIND_ENTERPRISE = "Enterprise"
INSTALL_KIND_PROFESSIONAL = "Professional"
INSTALL_KIND_COMMUNITY = "Community"

# Ordered by which install to use, from top to bottom
INSTALL_KINDS = (
  INSTALL_KIND_ENTERPRISE,
  INSTALL_KIND_PROFESSIONAL,
  INSTALL_KIND_COMMUNITY)

_CARGO_TARGET = {
  ARCH_X86: "i686-pc-windows-msvc",
  ARCH_X64: "x86_64-pc-windows-msvc" }

_CMAKE_GENERATOR = {
  ARCH_X86: "Visual Studio 15 2017",
  ARCH_X64: "Visual Studio 15 2017 Win64" }

def cargo_target(host):
  return _CARGO_TARGET[host]

def cmake_generator(host):
  return _CMAKE_GENERATOR[host]

def setup_environment(host):
  pass

def find_vc_install(drive, ver):
  base = f"{drive}:\\Program Files (x86)\\Microsoft Visual Studio\\{ver}"

  for kind in INSTALL_KINDS:
    kind_path = path.join(base, kind)
    if path.exists(kind_path):
      return kind_path