#!/bin/bash


# make love
cd ./src

rm -rf ./*.o
rm -rf ./*.gch

clang -target arm64-apple-macos11 -c -Wall  -I ./ Absyn.c Absyn.h
clang -target arm64-apple-macos11 -c -Wall  -I ./ Buffer.c Buffer.h
clang -target arm64-apple-macos11 -c -Wall  -I ./ Lexer.c
clang -target arm64-apple-macos11 -c -Wall  -I ./ Parser.c Absyn.h
clang -target arm64-apple-macos11 -c -Wall  -I ./ Printer.c Printer.h

# pack static library
ar rcs ../libbnfc.a Absyn.o Buffer.o Lexer.o Parser.o Printer.o

cd ..

mv -f ./libbnfc.a ../../rholang_parser/lib/aarch64-apple-darwin/
