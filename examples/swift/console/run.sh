#!/bin/bash

rm -rf deps
rm -f qsharp-bridge-swift-sample
mkdir -p deps

cargo build --release
cp ../../../target/release/libqsharp_bridge.a deps/
cp ../../../bindings/qsharp_bridge.swift deps/
cp ../../../bindings/qsharp_bridgeFFI.h deps/
cp ../../../bindings/qsharp_bridgeFFI.modulemap deps/
swiftc *.swift deps/*.swift -I./deps -L./deps -lqsharp_bridge -Xcc -fmodule-map-file=$(pwd)/deps/qsharp_bridgeFFI.modulemap -o qsharp-bridge-swift-sample

./qsharp-bridge-swift-sample