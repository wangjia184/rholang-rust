#!/bin/bash



# make love
cd ./src
make clean
make


# pack static library
ar rcs ../libbnfc.a Absyn.o Buffer.o Lexer.o Parser.o Printer.o

cd ..

# be careful with the platform
mv -f ./libbnfc.a ../../rholang_parser/lib/x86_64-apple-darwin/
#mv -f ./libbnfc.a ../../rholang_parser/lib/aarch64-apple-darwin/
#mv -f ./libbnfc.a ../../rholang_parser/lib/x86_64-unknown-linux-gnu/

