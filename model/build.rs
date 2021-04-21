extern crate bindgen;

use std::env;
use std::fs;
use std::path::{ PathBuf };

fn main() {
    // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
    //let target = env::var("TARGET").expect("Environment variable `TARGET` cannot be found.");
    //let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Environment variable `CARGO_MANIFEST_DIR` cannot be found.");
    let out_dir = env::var("OUT_DIR").expect("Environment variable `OUT_DIR` cannot be found.");

    let out_path = PathBuf::from(out_dir);
    let mut config = prost_build::Config::new();
    config.bytes(&["."]);
    config.extern_path(".bitset", "crate");
    match config.compile_protos(&["src/rho_types/rho_types.proto"], &["src/"]) {
        Err(e) =>  { panic!("Failed to generate from rho_types.proto {}", e); },
        Ok(_) => {
            fs::copy( out_path.join("rho_types.rs"), "src/rho_types/RhoTypes.rs")
                .expect("Failed to copy generated file to src/rho_types/RhoTypes.rs");
        }
    }


}