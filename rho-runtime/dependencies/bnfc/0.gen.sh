#!/bin/bash

#https://bnfc.digitalgrammars.com/tutorial/bnfc-tutorial.html

rm -rf ./src

# generate C source files
bnfc -m --c ./rholang_mercury.cf -o ./src

# add TLS to global variables
sed --in-place --regexp-extended '/^([a-zA-Z]+)\s*YY_RESULT_\1_\s*\=/i __thread ' ./src/rholang_mercury.y



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

