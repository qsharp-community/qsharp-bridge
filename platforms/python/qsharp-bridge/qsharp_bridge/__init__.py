import os
import sys
import ctypes
import platform

def _load_native_lib():
    pkg_dir = os.path.dirname(__file__)
    if sys.platform.startswith("darwin"):
        lib_name = "libqsharp_bridge.dylib"
    elif sys.platform.startswith("linux"):
        lib_name = "libqsharp_bridge.so"
    elif sys.platform.startswith("win"):
        lib_name = "qsharp_bridge.dll"
    else:
        raise RuntimeError(f"Unsupported platform: {sys.platform}")

    lib_path = os.path.join(pkg_dir, lib_name)
    if not os.path.exists(lib_path):
        raise RuntimeError(f"Native library not found: {lib_path}")
    return ctypes.CDLL(lib_path)

native_lib = _load_native_lib()

from .qsharp_bridge import *