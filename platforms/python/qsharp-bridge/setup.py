import os
import sys
import subprocess
import platform
from setuptools import setup, find_packages
from setuptools.command.build_py import build_py as _build_py

# Import the bdist_wheel command from wheel
try:
    from wheel.bdist_wheel import bdist_wheel as _bdist_wheel
except ImportError:
    _bdist_wheel = None

HERE = os.path.abspath(os.path.dirname(__file__))
CARGO_MANIFEST_PATH = os.path.abspath(os.path.join(HERE, "../../../Cargo.toml"))
CARGO_TARGET_DIR = os.path.abspath(os.path.join(HERE, "../../../target/release"))
BINDINGS_SRC = os.path.abspath(os.path.join(HERE, "../../../bindings/qsharp_bridge.py"))

def get_lib_filename():
    """
    Return the platform-specific filename for the shared library.
    For Windows, select the binary based on the machine architecture.
    """
    machine = platform.machine().lower()
    if sys.platform.startswith("darwin"):
        # for now we want to support arm64 only, but this could be a fat (lipo-ed) dylib
        return "libqsharp_bridge.dylib"
    elif sys.platform.startswith("linux"):
        return "libqsharp_bridge.so"
    elif sys.platform.startswith("win"):
        # what an ambition, arm64 Windows support. maybe it will work?
        if machine in ("amd64", "x86_64"):
            return "qsharp_bridge.dll"
        elif machine in ("arm64", "aarch64"):
            return "qsharp_bridge_arm64.dll"
        else:
            return "qsharp_bridge.dll"
    else:
        raise RuntimeError(f"Unsupported platform: {sys.platform}")

class build_py(_build_py):
    def run(self):
        # 1. build the Rust library.
        lib_filename = get_lib_filename()
        print("Building Rust library with Cargo …")
        subprocess.check_call([
            "cargo", "build", "--release",
            "--manifest-path", CARGO_MANIFEST_PATH
        ])

        # 2. copy the native library from Cargo's output folder into the Python package.
        src_lib = os.path.join(CARGO_TARGET_DIR, lib_filename)
        dst_dir = os.path.join(HERE, "qsharp_bridge")
        dst_lib = os.path.join(dst_dir, lib_filename)
        self.mkpath(dst_dir)
        print(f"Copying native library: {src_lib} -> {dst_lib}")
        self.copy_file(src_lib, dst_lib)

        # 3. copy uniFFI bindings file from the bindings/ folder.
        dst_binding = os.path.join(dst_dir, "qsharp_bridge.py")
        if os.path.exists(BINDINGS_SRC):
            print(f"Copying binding file: {BINDINGS_SRC} -> {dst_binding}")
            self.copy_file(BINDINGS_SRC, dst_binding)
        else:
            print("Warning: Binding file not found at", BINDINGS_SRC)

        # 4. continue with the standard build process.
        super().run()

if _bdist_wheel:
    class bdist_wheel(_bdist_wheel):
        def finalize_options(self):
            super().finalize_options()
            # mark the wheel as not pure so that a platform tag is used
            # I am not sure I know what I am doing, but this sounds right
            self.root_is_pure = False
else:
    bdist_wheel = None

setup(
    name="qsharp-bridge",
    version="0.1.0",
    description="Cross platform library for accessing Q# features in a simple way",
    author="Filip w",
    author_email="contact@strathweb.com",
    packages=find_packages(), 
    package_data={"qsharp_bridge": ["*.so", "*.dylib", "*.dll", "*.py"]},
    include_package_data=True,
    cmdclass={'build_py': build_py, 'bdist_wheel': bdist_wheel} if bdist_wheel else {'build_py': build_py},
    classifiers=[
        "Programming Language :: Python :: 3",
    ],
    python_requires=">=3.6",
)
