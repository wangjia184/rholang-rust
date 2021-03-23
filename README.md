# Rholang Runtime


A rholang runtime written in Rust

* `/model/` is a library crate containing protobuf models depended by `rho-runtime` and `rholang-parser`
* `/rholang-parser/` crate builds an executable program which accepts rholang code and outputs normalized AST
* `/rho-runtime/` is the primary executable program. It launches `rholang-parser` to parse rholang code, then handle reducing and storage
