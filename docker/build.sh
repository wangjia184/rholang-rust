#!/bin/bash

CARGO_PROFILE_RELEASE_LTO=true CARGO_PROFILE_RELEASE_PANIC=abort RUSTFLAGS="--emit=asm" cargo build --release

mkdir -p ./app
cp ../target/release/rho_runtime -f ./app/rho_runtime
cp ../target/release/rholang_parser -f ./app/rholang_parser

IMAGE_URI=wangjia184/rho_runtime:0.0.4

sudo docker build -t=$IMAGE_URI .
sudo docker push  $IMAGE_URI
