#!/bin/bash

NAME="qsharp_bridge"
HEADERPATH="bindings/qsharp_bridgeFFI.h"
TARGETDIR="target"
OUTDIR="artifacts/swift"
RELDIR="release"
STATIC_LIB_NAME="lib${NAME}.a"
NEW_HEADER_DIR="bindings/include"

IPHONEOS_DEPLOYMENT_TARGET=18.0 cargo build --target aarch64-apple-ios --release
cargo build --target aarch64-apple-ios-sim --release
MACOSX_DEPLOYMENT_TARGET=14.0 cargo build --target aarch64-apple-darwin --release

mkdir -p "${NEW_HEADER_DIR}"
cp "${HEADERPATH}" "${NEW_HEADER_DIR}/"
cp "bindings/qsharp_bridgeFFI.modulemap" "${NEW_HEADER_DIR}/module.modulemap"

rm -rf "${OUTDIR}/${NAME}_framework.xcframework"

xcodebuild -create-xcframework \
    -library "${TARGETDIR}/aarch64-apple-ios/${RELDIR}/${STATIC_LIB_NAME}" \
    -headers "${NEW_HEADER_DIR}" \
    -library "${TARGETDIR}/aarch64-apple-ios-sim/${RELDIR}/${STATIC_LIB_NAME}" \
    -headers "${NEW_HEADER_DIR}" \
    -library "${TARGETDIR}/aarch64-apple-darwin/${RELDIR}/${STATIC_LIB_NAME}" \
    -headers "${NEW_HEADER_DIR}" \
    -output "${OUTDIR}/${NAME}_framework.xcframework"

rm -rf "platforms/swift/Qsharp.Bridge/Libs"
mkdir -p "platforms/swift/Qsharp.Bridge/Libs"
cp -R "${OUTDIR}/${NAME}_framework.xcframework" "platforms/swift/Qsharp.Bridge/Libs"
cp "bindings/qsharp_bridge.swift" "platforms/swift/Qsharp.Bridge/Sources/Qsharp.Bridge"

swift build -c release --package-path platforms/swift/Qsharp.Bridge