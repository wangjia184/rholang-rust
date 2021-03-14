# Rholang Runtime

## Prerequisites

The following command line tools required:

* BNFC http://bnfc.digitalgrammars.com
* FLEX https://github.com/westes/flex
* BISON https://github.com/akimd/bison
* clang dev

## Files & Directories

* `/dependencies/` External dependencies
  * `bnfc/` BNFC generates lex and gramma parser
  * `protobuf/` Protobuf generator
* `/include` C header files
* `/lib` Static libraries
* `/src` Rust source code