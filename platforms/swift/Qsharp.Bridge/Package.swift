// swift-tools-version: 5.9

import PackageDescription

let package = Package(
    name: "Qsharp.Bridge",
    platforms: [
        .iOS(.v15), .macOS(.v14)
    ],
    products: [
        .library(
            name: "Qsharp.Bridge",
            targets: ["Qsharp.Bridge"]),
    ],
    targets: [
        .target(
            name: "Qsharp.Bridge",
            dependencies: ["Qsharp.Bridge.FFI"],
            path: "Sources/Qsharp.Bridge"),
        .target(
            name: "Qsharp.Bridge.FFI",
            dependencies: ["qsharp_bridge_framework"],
            path: "Sources/FFI",
            publicHeadersPath: "include"),
        .binaryTarget(
            name: "qsharp_bridge_framework",
            path: "Libs/qsharp_bridge_framework.xcframework"),
    ]
)