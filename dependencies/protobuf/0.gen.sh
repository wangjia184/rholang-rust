#!/bin/bash

# https://github.com/stepancheg/rust-protobuf/tree/master/protobuf-codegen
# First install protoc
# $ arch -arm64 brew install protobuf
# or
# $ apt-get install protobuf-compiler


# Second, install plugin
# cargo install protobuf-codegen

PROTOC_GEN_RUST=$(which protoc-gen-rust)
echo $PROTOC_GEN_RUST

PATH="$(dirname "${PROTOC_GEN_RUST}"):$PATH"
protoc --rust_out . rho_types.proto

mv -f ./rho_types.rs ../../rho-runtime/model/