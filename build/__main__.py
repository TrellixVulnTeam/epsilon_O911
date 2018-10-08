import argparse
import cargo
import llvm
import platform

from common import ARCH_LIST, ARCH_X64, OPT_DEBUG, OPT_RELEASE

CARGO_RUN = "run"
CARGO_BUILD = "build"
CARGO_FORMAT = "fmt"
CARGO_CLIPPY = "clippy"
EXTRA_CARGO_ARGUMENTS = {
  CARGO_RUN: (),
  CARGO_BUILD: (),
  CARGO_FORMAT: (),
  CARGO_CLIPPY: (
    "-A", "clippy::cast_ptr_alignment",
    "-A", "clippy::let_unit_value",
    "-A", "clippy::mutex_atomic" ) }

def get_arguments():
  parser = argparse.ArgumentParser(
    parents=[platform.command_line_parser()],
    description=
      "Build newt - we have this instead of cargo because we can't build "
      "LLVM through build.rs",
    allow_abbrev=False)

  parser.add_argument(
    "--host",
    dest="host",
    action="store",
    metavar="ARCH",
    default=ARCH_X64,
    choices=ARCH_LIST,
    required=False,
    help="Which host to build for - similar to Cargo's --target")

  parser.add_argument(
    "--release",
    dest="opt_level",
    action="store_const",
    default=OPT_DEBUG,
    const=OPT_RELEASE,
    required=False,
    help=
      "Build in release mode "
      "(note: LLVM is always built with optimizations and assertions)")

  parser.add_argument(
    "action",
    metavar="ACTION",
    default=CARGO_BUILD,
    choices=(CARGO_BUILD, CARGO_RUN, CARGO_FORMAT, CARGO_CLIPPY),
    help="Action to take (by default, build)",
    nargs="?")

  args = parser.parse_args()
  args.requires_target = args.action in ["build", "run"]
  args.extra_cargo_arguments = EXTRA_CARGO_ARGUMENTS[args.action]

  return args

def main():
  args = get_arguments()

  llvm.download()
  llvm.build(args)

  cargo.build(args)

if __name__ == "__main__":
  main()