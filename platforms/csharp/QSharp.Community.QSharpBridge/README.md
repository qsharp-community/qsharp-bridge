# QSharp.Community.QSharpBridge

NuGet package exposing the native functionalities of the `qsharp-bridge`.

## Building

1. Make sure the Rust project is built in release mode

    `cargo build --release`

2. Build the .NET library

    `dotnet build -c release`

The NuGet package will be located under `./bin/release/QSharp.Community.QSharpBridge.{version}.nupkg`