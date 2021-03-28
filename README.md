# Rholang Runtime


A rholang runtime written in Rust. Development is still in progress...

* `/model/` is a library crate containing protobuf models depended by `rho_runtime` and `rholang_parser`
* `/rholang_parser/` crate builds an executable program which accepts rholang code and outputs normalized AST
* `/rho_runtime/` is the primary executable program. It launches `rholang_parser` to parse rholang code, then handle reducing and storage



## File Naming Convertion

Two kinds of naming convertions applied in this project.

* Filename using *snake_case* (e.g. `common.rs` / `rho_par.rs`) means it is an individual module, which is the default approach in Rust's module system.
* Filename using *PascalCase* (e.g. `Sortable.rs` / `DeBruijnIndexMap.rs`) means it is a part of its directory module.

The pascal cases style is used to keep the code structure similar as Scala edition without big single file or a lot of re-exports.

```
src/
├─ rho_types/
│  ├─ mod.rs
│  ├─ DeBruijnLevelMap.rs
│  ├─ DeBruijnIndexMap.rs
├─ normalize/
│  ├─ mod.rs
│  ├─ rho_par.rs
```
In the above example, `DeBruijnLevelMap.rs` and `DeBruijnIndexMap.rs` are included by `/src/context/mod.rd` as part of `context` module; 
While `rho_par.rs` defines `normalize::rho_par` module.