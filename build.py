#!/usr/bin/env python3
"""Build a rust2cython extension without relying on a shell script.

Run from a Rust crate root after generating bindings, for example:
    python build.py --name mylib --bindings bindings
"""
from __future__ import annotations

import argparse
import shutil
import subprocess
import sys
from pathlib import Path


def run(*command: str, cwd: Path) -> None:
    print("+", " ".join(command))
    try:
        subprocess.run(command, cwd=cwd, check=True)
    except subprocess.CalledProcessError as error:
        raise SystemExit(f"build failed while running {' '.join(command)} (exit {error.returncode})")


def platform_name(value: str) -> str:
    if value != "auto":
        return value
    if sys.platform.startswith("win"):
        return "windows"
    if sys.platform == "darwin":
        return "macos"
    return "linux"


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--target-os", choices=("auto", "linux", "macos", "windows"), default="auto")
    parser.add_argument("--name", default="mylib", help="Rust cdylib and generated Python module name")
    parser.add_argument("--bindings", default="bindings", help="directory containing generated .pyx and setup.py")
    args = parser.parse_args()

    root = Path.cwd()
    bindings = (root / args.bindings).resolve()
    pyx = bindings / f"{args.name}.pyx"
    setup = bindings / "setup.py"
    if not pyx.is_file() or not setup.is_file():
        raise SystemExit(f"missing generated bindings in {bindings}; expected {pyx.name} and setup.py")

    target = platform_name(args.target_os)
    run("cargo", "build", "--release", cwd=root)
    release = root / "target" / "release"
    if target == "windows":
        library = release / f"{args.name}.dll"
        import_library = release / f"{args.name}.dll.lib"
        if not library.is_file() or not import_library.is_file():
            raise SystemExit(f"missing Windows cdylib outputs for {args.name} in {release}")
        shutil.copy2(library, bindings / library.name)
        shutil.copy2(import_library, bindings / f"{args.name}.lib")
    else:
        suffix = ".dylib" if target == "macos" else ".so"
        library = release / f"lib{args.name}{suffix}"
        if not library.is_file():
            raise SystemExit(f"missing cdylib {library}; ensure [lib] crate-type includes cdylib")
        shutil.copy2(library, bindings / library.name)

    run("cython", "--cplus", pyx.name, "-o", f"{args.name}.cpp", cwd=bindings)
    run(sys.executable, "setup.py", "build_ext", "--inplace", cwd=bindings)
    print(f"built {args.name} for {target}")


if __name__ == "__main__":
    main()
