use std::env;
use std::fs::{ File };
use std::io::{Write};
use std::process::{ self, Command };
use std::io::prelude::*;
use std::io::Cursor;
use prost::Message;
use tempfile::tempdir;

use model::*;


pub fn run_normalizer(source : &str) -> NormalizeResult {

    let filename = if cfg!(target_os = "windows") {
        format!("{}.exe", env::var("CARGO_PKG_NAME").unwrap())
    } else {
        env::var("CARGO_PKG_NAME").unwrap()
    };

    let mut dir = env::current_exe().unwrap();
    let mut filepath = dir.join(&filename);
    loop {
        if let Some(parent_dir) = dir.parent() {
            dir = parent_dir.to_path_buf();
            filepath = dir.join(&filename);

            if filepath.exists() {
                break;
            }
        } else {
            panic!("Cannot find the file {}", filename);
        }
    }
    

    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.rho");
    let output_path = dir.path().join("output.bin");
    {
        let mut file = File::create(&input_path).unwrap();
        file.write_all(source.as_bytes()).unwrap();
    }

    
    let output = Command::new(&filepath)
        .arg("--input")
        .arg(&input_path.to_str().unwrap())
        .arg("--output")
        .arg(&output_path.to_str().unwrap())
        .arg("--pid")
        .arg(process::id().to_string())
        .output()
        .expect(&format!("failed to execute process : {}", &filepath.to_str().unwrap()));

    
    let stderr = String::from_utf8(output.stderr).unwrap();
    println!("{}", stderr);

    let result : NormalizeResult;
    {
        let mut file = match File::open(&output_path) {
            Err(e) => panic!("Failed to open file at {}. {:#?} {}", output_path.to_str().unwrap(), e, e),
            Ok(f) => f,
        };
        let mut buffer = Vec::new();
        // read the whole file
        file.read_to_end(&mut buffer).unwrap();
        let mut cursor = Cursor::new(buffer);
        result = NormalizeResult::decode(&mut cursor).unwrap();
    }

    dir.close().unwrap();

    result
}
