import os

from common import OPT_RELEASE, OPT_DEBUG
from llvm import install_path, LLVM_SYS_ENV_VAR
from platform import cargo_target
from subprocess import run

def build(args):
  target = cargo_target(args)

  cmd = [ "cargo", args.action ]

  if args.requires_target:
    cmd.extend(("--target", target))

  if args.opt_level == OPT_RELEASE:
    cmd.append("--release")
  elif args.opt_level == OPT_DEBUG:
    pass
  else:
    assert False

  if args.extra_cargo_arguments:
    cmd.append("--")
    cmd.extend(args.extra_cargo_arguments)

  os.environ[LLVM_SYS_ENV_VAR] = install_path(args)

  out = run(cmd)
  if out.returncode != 0:
    exit(1)