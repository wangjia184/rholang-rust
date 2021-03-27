# Rholang Runtime


A rholang runtime written in Rust. Development is still in progress...

* `/model/` is a library crate containing protobuf models depended by `rho_runtime` and `rholang_parser`
* `/rholang_parser/` crate builds an executable program which accepts rholang code and outputs normalized AST
* `/rho_runtime/` is the primary executable program. It launches `rholang_parser` to parse rholang code, then handle reducing and storage

