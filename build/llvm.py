import os
import tarfile

from io import BytesIO
from os import path
from subprocess import run
from shutil import rmtree
from urllib.request import urlopen

import platform

LLVM_SRC_DIR = "llvm-6.0.1.src"
LLVM_URL = f"https://releases.llvm.org/6.0.1/{LLVM_SRC_DIR}.tar.xz"
LLVM_SYS_ENV_VAR = "LLVM_SYS_60_PREFIX"

LLVM_PATH = path.join(os.getcwd(), "llvm")
LLVM_SOURCE_PATH = path.join(LLVM_PATH, "source")

def _llvm_members(tf):
  dir_len = len(LLVM_SRC_DIR) + 1 # +1 for the /
  for member in tf.getmembers():
    if member.path.startswith(LLVM_SRC_DIR):
      member.path = path.join(LLVM_SOURCE_PATH, member.path[dir_len:])
      yield member

def install_path(args):
  return path.join(LLVM_PATH, args.host)

def download():
  if not path.exists(LLVM_PATH):
    os.mkdir(LLVM_PATH)

  if not path.exists(LLVM_SOURCE_PATH):
    print("Downloading sources")
    with urlopen(LLVM_URL) as f:
      with tarfile.open(mode="r:xz", fileobj=BytesIO(f.read())) as tf:
        print('Extracting sources')
        tf.extractall(members=_llvm_members(tf))
  else:
    print("Already downloaded LLVM sources")

def build(args):
  cwd = os.getcwd()
  build_path = path.join(LLVM_PATH, f"build-{args.host}")
  inst_path = install_path(args)

  env = platform.environment(args)

  if not path.exists(build_path):
    print("Building for", args.host)
    os.mkdir(build_path)
    os.chdir(build_path)
    cmd_line = (
      "cmake", LLVM_SOURCE_PATH,
      f"-DCMAKE_INSTALL_PREFIX={inst_path}",
      "-DCMAKE_BUILD_TYPE=Release",
      "-G", platform.cmake_generator(args),
      "-DLLVM_ENABLE_ASSERTIONS=1")
    out = run(cmd_line, env=env)
    if out.returncode != 0:
      rmtree(build_path)
      exit(1)
  else:
    print("Already built for", args.host)
    os.chdir(build_path)

  if not path.exists(inst_path):
    print("Installing LLVM for", args.host)
    cmd_line = (
      "cmake",
      "--build", ".",
      "--target", "install")
    out = run(cmd_line, env=env)
    if out.returncode != 0:
      rmtree(inst_path)
      exit(1)
  else:
    print("Already installed LLVM for", args.host)

  os.chdir(cwd)
