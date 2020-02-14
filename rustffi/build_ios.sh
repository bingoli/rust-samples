#!/bin/bash

OLD_PWD="`pwd`"
FIlE_DIR="`dirname $0`"

cd $FIlE_DIR

PROJ_NAME="rustffi"
PROJ_LIB="librustffi.a"
PROJ_FAT_LIB="librustffifat.a"

# Generate C bindings
cbindgen -l C -o target/$PROJ_NAME.h
# Build for iOS architectures
cargo build --target aarch64-apple-ios
cargo build --target x86_64-apple-ios

# Combine them into a universal library
lipo -create target/aarch64-apple-ios/debug/$PROJ_LIB target/x86_64-apple-ios/debug/$PROJ_LIB -output target/$PROJ_FAT_LIB

cd $OLD_PWD