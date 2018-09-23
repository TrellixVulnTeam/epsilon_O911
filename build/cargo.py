import os

from common import OPT_RELEASE, OPT_DEBUG
from llvm import install_path, LLVM_SYS_ENV_VAR
from platform import cargo_target
from subprocess import run

def build(host, opt_level):
  target = cargo_target(host)

  cmd = [
    "cargo", "build",
    "--target", target ]

  if opt_level == OPT_RELEASE:
    cmd.append("--release")
  elif opt_level == OPT_DEBUG:
    pass
  else:
    assert False

  os.environ[LLVM_SYS_ENV_VAR] = install_path(host)

  out = run(cmd)
  if out.returncode != 0:
    exit(1)