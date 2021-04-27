extern crate pretty_env_logger;
#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;

#[cfg(not(any(
    target_env = "msvc", 
    target_vendor="apple"
)))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

use std::time::Instant;
use std::path::PathBuf;
use std::env;
use std::io::prelude::*;
use std::fs::{ File };
use std::io::{Write};
use std::process::{ self, Command };
use std::io::Cursor;
use prost::Message;
use tempfile::tempdir;

use storage::Storage;
use tokio::runtime;
use model::*;

mod interpreter;
mod storage;


fn main() {

    pretty_env_logger::init();

    let rholang_source = "
    new x, y, z, stdout(`rho:io:stdout`) in {
        x!(1) |
        for( a <= x ) {
            if( *a < 100000 ) {
                x!(*a+1)
            } else {
                stdout!(*a)
            }
        }
    }
    ";

    let now = Instant::now();
    let mut normalized_result = run_normalizer(rholang_source);
    info!("Normalizatin took {} ms", now.elapsed().as_millis());

    if let Some(err) = normalized_result.compiliation_error {
        error!("Normalization failed! #{} - {}", err.kind, err.message);
        return;
    }
    if !normalized_result.syntax_errors.is_empty() {
        for  syntax_error in normalized_result.syntax_errors {
            error!("Source code {} {},  #{} syntax error. {}"
                , match syntax_error.position { Some(pos) => pos.to_string(), _ => "".to_owned() }
                , match syntax_error.contra_position { Some(pos) => pos.to_string(), _ => "".to_owned() }
                , syntax_error.kind
                , syntax_error.message
                );
        }
        return;
    }
    let par = match normalized_result.par.take() {
        Some(p) => p,
        _ => panic!("Par is missing!"),
    };
    
    let rt = runtime::Builder::new_multi_thread()
                    .thread_stack_size(1024*1024*20)
                    .build()
                    .expect("Unable to setup runtime");
    let future = run(par);
    rt.block_on(future);
}


async fn run(par : Par) {
    let (store,mut coordinator) = storage::Coordinator::create();
    interpreter::system_process::setup(&store).await;
    tokio::task::spawn(async move{
        test(store, par).await
    });

    
    coordinator.run().await;
}

async fn test<S>(storage : S, par : Par) 
    where S : Storage + Clone + std::marker::Send + std::marker::Sync + 'static {
    
    let context = std::sync::Arc::new(interpreter::InterpreterContext::from(storage.clone()));

    let now = Instant::now();
    context.evaludate_par(par).await;
    info!("Reduction took {} ms", now.elapsed().as_millis());
    /*
    for err in errors {
        error!("Error #{} : {}", err.kind, err.message);
    }
    */
    storage.uninstall(); // stop
}



pub fn run_normalizer(source : &str) -> NormalizeResult {

    let parser_filepath = if cfg!(target_os = "windows") {
        get_file_path("rholang_parser.exe")
    } else {
        get_file_path("rholang_parser")
    };

    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.rho");
    let output_path = dir.path().join("output.bin");
    {
        let mut file = File::create(&input_path).unwrap();
        file.write_all(source.as_bytes()).unwrap();
    }

    
    let output = Command::new(&parser_filepath)
        .arg("--input")
        .arg(&input_path.to_str().unwrap())
        .arg("--output")
        .arg(&output_path.to_str().unwrap())
        .arg("--pid")
        .arg(process::id().to_string())
        .output()
        .expect(&format!("failed to execute process : {}", &parser_filepath.display()));

    
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


fn get_file_path(filename: &str) -> PathBuf {
    let mut dir = env::current_exe().unwrap();
    loop {
        if let Some(parent_dir) = dir.parent() {
            dir = parent_dir.to_path_buf();
            let filepath = dir.join(&filename);

            if filepath.exists() {
                return filepath;
            }
        } else {
            panic!("Cannot find the file {}", filename);
        }
    }
}