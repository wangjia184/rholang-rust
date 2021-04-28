#!/bin/bash

#CARGO_PROFILE_RELEASE_LTO=true cargo build --release

mkdir ./app
cp ../target/release/rho_runtime -f ./app/rho_runtime
cp ../target/release/rholang_parser -f ./app/rholang_parser

sudo docker build .