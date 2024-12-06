#!/bin/bash

rm -fr __pycache__
rm -f qsharp_bridge.dylib
rm -f qsharp_bridge.so
rm -f qsharp_bridge.py

cargo build --release --manifest-path ../../../Cargo.toml
if [[ "$OSTYPE" == "darwin"* ]]; then
    cp ../../../target/release/libqsharp_bridge.dylib .
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    cp ../../../target/release/libqsharp_bridge.so .
else
    echo "Unsupported OS: $OSTYPE"
    exit 1
fi
cp ../../../bindings/qsharp_bridge.py .