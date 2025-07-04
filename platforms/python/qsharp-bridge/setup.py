import os
import sys
import subprocess
import platform
from setuptools import setup, find_packages
from setuptools.command.build_py import build_py as _build_py

try:
    from wheel.bdist_wheel import bdist_wheel as _bdist_wheel
except ImportError:
    _bdist_wheel = None

HERE = os.path.abspath(os.path.dirname(__file__))
CARGO_MANIFEST_PATH = os.path.abspath(os.path.join(HERE, "../../../Cargo.toml"))
CARGO_TARGET_DIR = os.path.abspath(os.path.join(HERE, "../../../target/release"))
BINDINGS_SRC = os.path.abspath(os.path.join(HERE, "../../../bindings/qsharp_bridge.py"))
VERSION = os.environ.get("PACKAGE_VERSION", "0.1.0")

def get_lib_filename():
    """
    Return the platform-specific filename for the shared library.
    """
    machine = platform.machine().lower()
    if sys.platform.startswith("darwin"):
        # for now we want to support arm64 only, but this could be a fat (lipo-ed) dylib
        return "libqsharp_bridge.dylib"
    elif sys.platform.startswith("linux"):
        return "libqsharp_bridge.so"
    elif sys.platform.startswith("win"):
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
            self.plat_name_supplied = True
            if sys.platform.startswith('linux'):
                self.plat_name = "py3-none-linux_x86_64"
            elif sys.platform.startswith('darwin'):
                self.plat_name = "py3-none-macosx_11_0_universal2"
            elif sys.platform.startswith('win'):
                machine = platform.machine().lower()
                if machine in ("arm64", "aarch64"):
                    self.plat_name = "py3-none-win_arm64"
                else:
                    self.plat_name = "py3-none-win_amd64"
            else:
                # Fall back to default behavior for unknown platforms
                self.plat_name_supplied = False
                
            super().finalize_options()
            # We still need to mark it as not pure Python
            self.root_is_pure = False
            
        def get_tag(self):
            # Override the tag generation to use our custom platform tag
            if self.plat_name_supplied:
                # Use py3 instead of cpXY to support any Python 3.x version
                return ('py3', 'none', self.plat_name.split('-')[-1])
            # Fall back to default behavior
            return super().get_tag()
else:
    bdist_wheel = None

setup(
    name="qsharp-bridge",
    version=VERSION,
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