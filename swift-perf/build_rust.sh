#!/bin/bash

OLD_PWD="`pwd`"
FIlE_DIR="`dirname $0`"

cd $FIlE_DIR

WORKSPACE="$FIlE_DIR/.."

# Location of rust project
RUST_PROJ="$WORKSPACE/rustffi"
# Location of the "Anvil" folder in the iOS project
IOS_RUST_FFI="$FIlE_DIR/rust-ffi"
# Provide access to Rust utilities
PATH="$PATH:~/.cargo/bin"

sh $RUST_PROJ/build_ios.sh

# Copy resources to the iOS folder, overwriting old ones
echo $RUST_PROJ/target/rustffi.h
echo $RUST_PROJ/target/librustffifat.a
echo "$IOS_RUST_FFI"
cp $RUST_PROJ/target/rustffi.h $RUST_PROJ/target/librustffifat.a "$IOS_RUST_FFI"

cd $OLD_PWD