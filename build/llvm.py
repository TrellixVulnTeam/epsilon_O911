import os
import tarfile

from io import BytesIO
from os import path
from subprocess import run
from urllib.request import urlopen

from platform import cmake_generator, setup_environment

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

def install_path(host):
  return path.join(LLVM_PATH, host)

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

def build(host):
  cwd = os.getcwd()
  build_path = path.join(LLVM_PATH, f"build-{host}")

  if not path.exists(build_path):
    print("Building for", host)
    os.mkdir(build_path)
    os.chdir(build_path)
    cmd_line = (
      "cmake", LLVM_SOURCE_PATH,
      f"-DCMAKE_INSTALL_PREFIX={install_path(host)}",
      "-DCMAKE_BUILD_TYPE=Release",
      "-G", cmake_generator(host),
      "-DLLVM_ENABLE_ASSERTIONS=1",
      "-Thost=x64")
    out = run(cmd_line)
    if out.returncode != 0:
      exit(1)
  else:
    print("Already built for", host)
    os.chdir(build_path)

  if not path.exists(install_path(host)):
    print("Installing LLVM for", host)
    cmd_line = (
      "cmake",
      "--build", ".",
      "--target", "install")
    out = run(cmd_line)
    if out.returncode != 0:
      exit(1)
  else:
    print("Already installed LLVM for", host)

  os.chdir(cwd)
