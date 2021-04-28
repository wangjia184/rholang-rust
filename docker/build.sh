#!/bin/bash

#CARGO_PROFILE_RELEASE_LTO=true cargo build --release

mkdir -p ./app
cp ../target/release/rho_runtime -f ./app/rho_runtime
cp ../target/release/rholang_parser -f ./app/rholang_parser

IMAGE_URI=wangjia184/rho_runtime:0.0.1

sudo docker build -t=$IMAGE_URI .
sudo docker push  $IMAGE_URI
