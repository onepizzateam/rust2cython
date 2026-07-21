#!/usr/bin/env python3
"""Cross-platform helper for building a rust2cython extension."""
import argparse
import subprocess
import sys
from pathlib import Path

def run(*args, cwd=None):
    print("+", " ".join(map(str, args)))
    subprocess.run(args, check=True, cwd=cwd)

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--target-os", choices=("auto", "linux", "macos", "windows"), default="auto")
    parser.add_argument("--name", default="mylib")
    parser.add_argument("--bindings", default="bindings")
    args = parser.parse_args()
    target = args.target_os if args.target_os != "auto" else ("windows" if sys.platform.startswith("win") else "macos" if sys.platform == "darwin" else "linux")
    root = Path(__file__).resolve().parent
    bindings = root / args.bindings
    try:
        run("cargo", "build", "--release")
        run("cython", "--cplus", str(bindings / f"{args.name}.pyx"))
        run(sys.executable, "setup.py", "build_ext", "--inplace", cwd=str(bindings))
    except subprocess.CalledProcessError as error:
        raise SystemExit(f"build failed for {target}: {error}")

if __name__ == "__main__":
    main()
