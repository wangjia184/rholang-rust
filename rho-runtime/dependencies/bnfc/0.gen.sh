#!/bin/bash

#https://bnfc.digitalgrammars.com/tutorial/bnfc-tutorial.html

rm -rf ./src

# generate C source files
bnfc -m --c ./rholang_mercury.cf -o ./src

# add a release function
sed --in-place '/#endif/i void freeProc(Proc proc);' ./src/Parser.h;

sed --in-place '/\/\* Global variables holding parse results for entrypoints. \*\//i void freeProc(Proc proc) { if(proc) free(proc); }' ./src/rholang_mercury.y

#printf "\n\nvoid freeProc(Proc proc);"  >> ./src/Absyn.h
#printf "\n\nvoid freeProc(Proc proc) { if(proc) free(proc); }"  >> ./src/Absyn.c



# copy header files
cp -rf ./src/*.h ../../include/bnfc/

# make love
cd ./src
make


# pack static library
ar rcs ../libbnfc.a Absyn.o Buffer.o Lexer.o Parser.o Printer.o

cd ..

# be careful the platform
#mv -f ./libbnfc.a ../../lib/x86_64-apple-darwin/
#mv -f ./libbnfc.a ../../lib/aarch64-apple-darwin/
mv -f ./libbnfc.a ../../lib/x86_64-unknown-linux-gnu/

