#!/bin/bash

#https://bnfc.digitalgrammars.com/tutorial/bnfc-tutorial.html

rm -rf ./src

# generate C source files
bnfc -m --c ./rholang_mercury.cf -o ./src

#printf "\n\nvoid freeProc(Proc proc);"  >> ./src/Absyn.h
#printf "\n\nvoid freeProc(Proc proc) { if(proc) free(proc); }"  >> ./src/Absyn.c



# copy header files
cp -rf ./src/*.h ../../rho-runtime/include/bnfc/

# make love
cd ./src
make
cd ..

# pack static library
ar rcs libbnfc.a ./src/*.o

# be careful the platform
mv -f ./libbnfc.a ../../rho-runtime/lib/aarch64-apple-darwin/
