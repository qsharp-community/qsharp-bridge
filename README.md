# qsharp-bridge
Language bindings for popular languages, surfacing Q# compiler and resource estimator APIs

## Supported Languages

- C#
- Swift
- Kotlin
- Python (though that is also natively supported by Q# already)

## Building

1. Install `uniffi-bindgen-cs` to support C# bindings generations. The other language bindings are supported without any extra dependencies.

```bash
cargo install uniffi-bindgen-cs --git https://github.com/NordSecurity/uniffi-bindgen-cs
```

2. Run the following command to build the project:

```bash
cargo build --release
```

3. The language bindings are generated into the `bindings` directory.

4. The native library is located under `./target/release/libqsharp_bridge.{so|dll|dylib|a}`.

## Examples

Explore the C#/Swift/Kotlin examples [here](./examples).