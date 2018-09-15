import os
import tarfile

from io import BytesIO
from os import path
from subprocess import run
from urllib.request import urlopen

LLVM_SRC_DIR = "llvm-6.0.1.src"
LLVM_URL = f"https://releases.llvm.org/6.0.1/{LLVM_SRC_DIR}.tar.xz"
LLVM_SYS_ENV_VAR = "LLVM_SYS_60_PREFIX"

LLVM_PATH = path.join(os.getcwd(), "llvm")
LLVM_SOURCE_PATH = path.join(LLVM_PATH, "source")

ARCH_X64 = "x64"
ARCH_X86 = "x86"

def llvm_members(tf):
  dir_len = len(LLVM_SRC_DIR) + 1 # +1 for the /
  for member in tf.getmembers():
    if member.path.startswith(LLVM_SRC_DIR):
      member.path = path.join(LLVM_SOURCE_PATH, member.path[dir_len:])
      yield member

def make_llvm_dir():
  if not path.exists(LLVM_PATH):
    os.mkdir(LLVM_PATH)

def download_source():
  print('Downloading sources')
  if not path.exists(LLVM_SOURCE_PATH):
    with urlopen(LLVM_URL) as f:
      with tarfile.open(mode="r:xz", fileobj=BytesIO(f.read())) as tf:
        print('Extracting sources')
        tf.extractall(members=llvm_members(tf))

def get_cmake_generator(arch):
  if os.name == 'nt':
    return "Ninja"
  else:
    raise RuntimeError("Non-windows platforms not yet supported")

def setup_environment():
  if os.name == 'nt':
    os.environ['CC'] = 'cl'
    os.environ['CXX'] = 'cl'

def build_for_arch(arch):
  print('building for', arch)
  cwd = os.getcwd()
  build_path = path.join(LLVM_PATH, f"build-{arch}")
  install_path = path.join(LLVM_PATH, arch)

  if not path.exists(build_path):
    os.mkdir(build_path)
    os.chdir(build_path)
    cmd_line = (
      "cmake", LLVM_SOURCE_PATH,
      f"-DCMAKE_INSTALL_PREFIX={install_path}",
      "-DCMAKE_BUILD_TYPE=Release",
      "-G", get_cmake_generator(arch),
      "-DLLVM_ENABLE_ASSERTIONS=1")
    out = run(cmd_line)
    if out.returncode != 0:
      exit(1)
  else:
    os.chdir(build_path)

  if not path.exists(install_path):
    cmd_line = (
      "cmake",
      "--build", ".",
      "--target", "install")
    out = run(cmd_line)
    if out.returncode != 0:
      exit(1)

  os.chdir(cwd)

def set_environment(host):
  prefix = path.join(LLVM_PATH, host)
  print(f"Please set {LLVM_SYS_ENV_VAR} to {prefix}")
  print()
  print(f"(in bash): {LLVM_SYS_ENV_VAR}=\"{prefix}\"")
  print(f"(in batch): set {LLVM_SYS_ENV_VAR}=\"{prefix}\"")
  print(f"(in powershell): $env:{LLVM_SYS_ENV_VAR} = \"{prefix}\"")

def main():
  host = ARCH_X64

  make_llvm_dir()
  download_source()
  build_for_arch(host)

  set_environment(host)

if __name__ == "__main__":
  main()