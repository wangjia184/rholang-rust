#!/bin/bash

#https://bnfc.digitalgrammars.com/tutorial/bnfc-tutorial.html

rm -rf ./src

# generate C source files
bnfc -m --c --line-numbers ./rholang_mercury.cf -o ./src

# add GCC's TLS extension __thread to global variables
# sed does not work the same in Mac OS and GNU Linux, so here use a stupid way to replace the following command
# sed --in-place --regexp-extended '/^([a-zA-Z]+)\s*YY_RESULT_\1_\s*\=/i __thread ' ./src/rholang_mercury.y
REGEX='^([a-zA-Z]+)[[:space:]]YY_RESULT_([a-zA-Z]+)_[[:space:]]\='
while IFS= read -r line
do
  if [[ $line =~ $REGEX ]] && [ "${BASH_REMATCH[1]}" = "${BASH_REMATCH[2]}" ] ; then
    echo "__thread $line" >> ./src/rholang_mercury.y.new
  else
    echo "$line" >> ./src/rholang_mercury.y.new
  fi
  
done < "./src/rholang_mercury.y"

mv -f ./src/rholang_mercury.y.new ./src/rholang_mercury.y

# copy header files
cp -rf ./src/*.h ../../include/bnfc/

# make love
cd ./src
make


# pack static library
ar rcs ../libbnfc.a Absyn.o Buffer.o Lexer.o Parser.o Printer.o

cd ..

# be careful with the platform
#mv -f ./libbnfc.a ../../lib/x86_64-apple-darwin/
#mv -f ./libbnfc.a ../../lib/aarch64-apple-darwin/
#mv -f ./libbnfc.a ../../lib/x86_64-unknown-linux-gnu/

