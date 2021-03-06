#!/bin/bash

#https://bnfc.digitalgrammars.com/tutorial/bnfc-tutorial.html

rm -rf ./src

# generate C source files
bnfc -m --c --line-numbers ./rholang_mercury.cf -o ./src

# copy header files
cp -rf ./src/*.h ../../rholang_parser/include/bnfc/
