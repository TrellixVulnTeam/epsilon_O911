import argparse
import cargo
import llvm
import platform

from common import ARCH_LIST, ARCH_X64, OPT_DEBUG, OPT_RELEASE

ACTION_RUN = "run"
ACTION_BUILD = "build"
ACTION_FORMAT = "fmt"
ACTION_CLIPPY = "clippy"
ACTION_CLEAN = "clean"
ACTION_DOC = "doc"

ACTIONS = (
  ACTION_RUN,
  ACTION_BUILD,
  ACTION_FORMAT,
  ACTION_CLIPPY,
  ACTION_CLEAN,
  ACTION_DOC)

EXTRA_CARGO_ARGUMENTS = {}

for action in ACTIONS:
  EXTRA_CARGO_ARGUMENTS[action] = ()

EXTRA_CARGO_ARGUMENTS[ACTION_CLIPPY] = (
  "--",
  "-A", "clippy::cast_ptr_alignment",
  "-A", "clippy::let_unit_value",
  "-A", "clippy::mutex_atomic" )

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
    "--open",
    dest="open_docs",
    action="store_true",
    required=False,
    help="Open the docs in your browser")

  parser.add_argument(
    "action",
    metavar="ACTION",
    default=ACTION_BUILD,
    choices=ACTIONS,
    help="Action to take (by default, build)",
    nargs="?")

  args = parser.parse_args()

  args.requires_target = args.action in (ACTION_BUILD, ACTION_RUN)
  args.extra_cargo_arguments = EXTRA_CARGO_ARGUMENTS[args.action]

  if args.open_docs:
    assert args.action == ACTION_DOC
    args.extra_cargo_arguments = ("--open",)

  return args

def main():
  args = get_arguments()

  if args.action == ACTION_CLEAN:
    print("Are you sure? You will need to rebuild LLVM as well, and that takes a while")
    yn = input("type 'YES' to clean, type anything else to exit: ")
    if yn != "YES":
      print("Exiting")
      exit(0)
    else:
      from shutil import rmtree
      rmtree(llvm.LLVM_PATH)
      cargo.build(args)
      exit(0)


  llvm.download()
  llvm.build(args)

  cargo.build(args)

if __name__ == "__main__":
  main()