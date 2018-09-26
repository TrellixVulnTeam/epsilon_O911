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

_DEFAULT_VS_DIRECTORY = "C:\\Program Files (x86)\\Microsoft Visual Studio\\2017"

_CARGO_TARGET = {
  ARCH_X86: "i686-pc-windows-msvc",
  ARCH_X64: "x86_64-pc-windows-msvc" }

def command_line_parser():
  import argparse

  parser = argparse.ArgumentParser(add_help=False)

  parser.add_argument(
    "--vs-directory",
    dest="vs_directory",
    action="store",
    metavar="DIR",
    required=False,
    help=
      "Where Visual Studio is located - by default, "
      f"`{_DEFAULT_VS_DIRECTORY}\\[Kind]'")

  return parser

def cargo_target(args):
  return _CARGO_TARGET[args.host]

def cmake_generator(args):
  return "Ninja"

def environment(args):
  import re

  from subprocess import run

  vcvars_dir = path.join(find_vc_install(args), "VC", "Auxiliary", "Build")

  if args.host == ARCH_X64:
    vcvars = path.join(vcvars_dir, 'vcvars64.bat')
  elif args.host == ARCH_X86:
    vcvars = path.join(vcvars_dir, 'vcvars32.bat')
  else:
    assert False

  cmd_line = (vcvars, '&', 'set')

  out = run(cmd_line, shell=True, capture_output=True)
  env = {}
  if out.returncode != 0:
    assert False
  else:
    matcher = re.compile("^([a-zA-Z0-9]*)=(.*)$")
    stdout = out.stdout.decode('utf-8')
    for line in stdout.splitlines():
      match = matcher.match(line)
      if match:
        env[match[1]] = match[2]

  env['CC'] = 'cl'
  env['CXX'] = 'cl'

  return env


def find_vc_install(args):
  if args.vs_directory:
    return args.vs_directory

  base = _DEFAULT_VS_DIRECTORY

  for kind in INSTALL_KINDS:
    kind_path = path.join(base, kind)
    if path.exists(kind_path):
      return kind_path

  assert False