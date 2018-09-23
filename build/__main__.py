import argparse
import cargo
import llvm

from common import ARCH_LIST, ARCH_X64, OPT_DEBUG, OPT_RELEASE

def get_arguments():
  parser = argparse.ArgumentParser(
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

  return parser.parse_args()

def main():
  args = get_arguments()

  llvm.download()
  llvm.build(args.host)

  cargo.build(args.host, args.opt_level)

if __name__ == "__main__":
  main()