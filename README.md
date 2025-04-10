# Q# Bridge

A cross-platform library for Swift/Kotlin/C#/Python, surfacing features of the Q# compiler and resource estimator.

## Supported platforms and languages

|                        | **Swift**                                        | **.NET**                                                                                    | **Kotlin**                        | **Python**                                                  |
|------------------------|--------------------------------------------------|---------------------------------------------------------------------------------------------|-----------------------------------|-------------------------------------------------------------|
| **Package**            | Swift Package                                    | Nuget                                                                                       | N/A                               | Wheel                                                         |
| **Manual Integration** | Bindings + XCFramework Bindings + native library | Bindings + native library                                                                   | Bindings + native library         | Bindings + native library                                   |
| **Platforms**          | macOS arm64<br/>iOS                                  | Windows x64<br/>Windows arm64 (not via Nuget)<br/>Linux x64<br/>Linux arm64 (not via Nuget)<br/>macOS arm64 | Windows x64<br/>Linux x64<br/>macOS arm64 | Windows x64<br/>Windows arm64<br/>Linux x64<br/>Linux arm64<br/>macOS arm64 |

## Building instructions

### Swift

Build the Swift Package (arm64 Mac required).

```shell
./build_swift.sh
```

This builds:
 - the Swift Package under `platforms/swift/QSharp.Bridge`
 - XCFramework under `artifacts/swift/qsharp_bridge_framework.xcframework`

Now run the Swift console app:

```shell
cd examples/swuft/console
./run.sh
```

### C#

Install UniFFI C# bindings generator

```shell
cargo install uniffi-bindgen-cs --git https://github.com/NordSecurity/uniffi-bindgen-cs --tag v0.9.1+v0.28.3
```

Build the Nuget package for your platform:

```shell
cargo build --release
```

Now run the .NET console app:

```shell
cd examples/csharp/console
dotnet run -c Release
```

### Kotlin

Go the Kotlin sample and run it from there:

```shell
cd samples/kotlin
./run.sh
```

### Python

First build the Wheel:

```shell
cd platforms/python/qsharp-bridge
python -m pip install --upgrade pip setuptools wheel
python setup.py sdist bdist_wheel
```

You can then use the Jupyter Notebooks:

```shell
cd samples/python/jupyter
```

Make sure you have created a Virtual Environment or activated a Conda environment, and install the dependencies (including the Wheel):

```shell
pip install -r requirements.txt
```

Now open the Notebook and run the cells.

## Compatibility notes

### .NET

✅ Tested on Windows arm64

✅ Tested on Windows x64

✅ Tested on Linux arm64

✅ Tested on Linux x64

✅ Tested on macOS arm64

### Swift

✅ Tested on macOS arm64.

✅ Tested on iPadOS

✅ Tested on iOS

### Kotlin

✅ Tested on macOS arm64.

### Python

✅ Tested on Windows arm64

✅ Tested on macOS arm64