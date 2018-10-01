import os

from common import OPT_RELEASE, OPT_DEBUG
from llvm import install_path, LLVM_SYS_ENV_VAR
from platform import cargo_target
from subprocess import run

def build(args):
  target = cargo_target(args)

  if args.run:
    cargo_cmd = "run"
  else:
    cargo_cmd = "build"

  cmd = [
    "cargo", cargo_cmd,
    "--target", target ]

  if args.opt_level == OPT_RELEASE:
    cmd.append("--release")
  elif args.opt_level == OPT_DEBUG:
    pass
  else:
    assert False

  os.environ[LLVM_SYS_ENV_VAR] = install_path(args)

  out = run(cmd)
  if out.returncode != 0:
    exit(1)