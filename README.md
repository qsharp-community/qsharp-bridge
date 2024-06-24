# qsharp-bridge
Language bindings for popular languages, surfacing Q# compiler and resource estimator APIs

## Supported Languages

- C#
- Swift
- Kotlin
- Python (though that is also natively supported by Q# already)

## Building

1. Run the following command to build the project:

```bash
cargo build --release
```

2. The language bindings are generated into the `bindings` directory.

3. The native library is located under `./target/release/libqsharp_bridge.{so|dll|dylib|a}`.