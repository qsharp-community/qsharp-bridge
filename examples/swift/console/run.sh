#!/bin/bash

rm -f qsharp-bridge-swift-sample

cd ../../../
./build_swift.sh
cd examples/swift/console

swiftc *.swift \
     ../../../bindings/qsharp_bridge.swift \
    -I ../../../artifacts/swift/qsharp_bridge_framework.xcframework/macos-arm64/Headers \
    -L ../../../artifacts/swift/qsharp_bridge_framework.xcframework/macos-arm64 \
    -lqsharp_bridge \
    -O -whole-module-optimization \
    -cross-module-optimization \
    -enforce-exclusivity=unchecked \
    -o qsharp-bridge-swift-sample

./qsharp-bridge-swift-sample "$@"