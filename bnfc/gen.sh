#!/bin/bash

#https://bnfc.digitalgrammars.com/tutorial/bnfc-tutorial.html

# generate C source files
bnfc -m -c ./rholang_mercury.cf -o ./src

# copy header files
cp -rf ./src/*.h ../rho-runtime/include/bnfc/

# make love
make

# pack static library
ar rcs libbnfc.a ./src/*.o

# be careful the platform
#cp ./libbnfc.a ../rho-runtime/lib/aarch64-apple-darwin/
