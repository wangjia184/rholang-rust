extern crate pretty_env_logger;
#[macro_use] extern crate log;
extern crate clap;

use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use prost::Message;
use bytes::BytesMut;

use model::*;

mod context;
mod bnfc;
mod normalize;
mod builder;



pub use context::*;


pub static VERSION: &str = "0.1.0";


fn main() {
    

    pretty_env_logger::init();

    let matches = clap::App::new("Rholang Parser")
        .version(VERSION)
        .author("++ vcer@qq.com")
        .about("Parse rholang source code and generate normalized AST/ADT")
        .arg(clap::Arg::with_name("input_file")
            .long("input")
            .short("i")
            .help("File path of rholang source to parse")
            .required(true)
            .takes_value(true))
        .arg(clap::Arg::with_name("output_file")
            .long("output")
            .short("o")
            .help("Path of file to store result. If this parameter is not supplied, warnings/errors are printed to stdout.")
            .required(false)
            .takes_value(true))
        .arg(clap::Arg::with_name("pid")
            .long("pid")
            .help("Process ID of parent process. When this parameter is supplied, rholang-parser watches the specified process and terminates itself if parent process exits.")
            .default_value("0")
            .required(false)
            .takes_value(true))
        .get_matches();
    
    let process_id : u32 = matches.value_of("pid").unwrap_or("0").parse().expect("`pid` parameter is invalid");
    let input_file = matches.value_of("input_file").unwrap();
    let output_file = matches.value_of("output_file").unwrap_or("");


    let result = builder::build_ast_from_file(input_file);
    if output_file.len() > 0 {
        match result {
            NormalizeResult { par : Some(ref p), .. } => save_to_file(output_file, p),
            _ => {},
        };
        
    } else {
        print_result(result);
    }
}


fn print_result(result : NormalizeResult) {
    if let Some(err) = result.compiliation_error {
        error!("Compiliation error : {}", err);
    } else if result.syntax_errors.len() > 0 {
        for syntax_error in result.syntax_errors.iter() {
            error!("{:?} - {:?} : {}", syntax_error.position, syntax_error.contra_position, &syntax_error.message);
        }
    } else {
        info!("Successfully compiled.");
        println!("{:?}", &result.par);
    }
    
}

fn save_to_file(filepath : &str, par : &Par) {
    {
        let mut file = match File::create(filepath) {
            Err(e) => panic!("Failed to create file at {}. {:?} {}", filepath, e, e),
            Ok(f) => f,
        };

        let mut buffer = BytesMut::new();
        par.encode(&mut buffer).unwrap();
        file.write_all(&buffer[..]).unwrap();
    }

    {
        let mut file = match File::open(filepath) {
            Err(e) => panic!("Failed to open file at {}. {:?} {}", filepath, e, e),
            Ok(f) => f,
        };
        let mut buffer = Vec::new();
        // read the whole file
        file.read_to_end(&mut buffer).unwrap();
        let mut cursor = Cursor::new(buffer);
        let par = Par::decode(&mut cursor).unwrap();
        println!("{:?}", &par);
    }
}
