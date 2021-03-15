extern crate bindgen;

use std::env;
use std::path::{ PathBuf };

fn main() {
    // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
    let target = env::var("TARGET").expect("Environment variable `TARGET` cannot be found.");
    let _manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Environment variable `CARGO_MANIFEST_DIR` cannot be found.");
    let _out_dir = env::var("OUT_DIR").expect("Environment variable `OUT_DIR` cannot be found.");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    //println!("cargo:rerun-if-changed=include/bnfc/bnfc.h");

    // static link to bnfc
    println!("cargo:rustc-link-lib=static=bnfc");

    // specify search path of libraries
    println!("cargo:rustc-link-search=native=lib/{}/", &target);

    let bindings = bindgen::Builder::default()
        .header("include/bnfc/bnfc.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bnfc_bindings.rs"))
        .expect("Couldn't write bindings!");
}