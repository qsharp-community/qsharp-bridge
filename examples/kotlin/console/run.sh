#!/bin/bash

rm -rf deps
rm -rf out
mkdir -p deps

cargo build --release
cp ../../../target/release/libqsharp_bridge.dylib deps/
cp ../../../bindings/qsharp/bridge/qsharp_bridge.kt deps/

kotlinc main.kt deps/qsharp_bridge.kt -include-runtime -cp lib/jna.jar -d out/main.jar
javac -cp out/main.jar:lib/jna.jar Main.java -d out
jar uf out/main.jar -C out Main.class

java -Djna.library.path=$(pwd)/deps -cp out/main.jar:lib/jna.jar MainKt
