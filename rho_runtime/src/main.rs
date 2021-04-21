extern crate pretty_env_logger;
#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;

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
    new x, y, stdout(`rho:stdout`) in {
        x!(7) | 
        y!(8) | 
        r!(9)
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
    
    let rt = runtime::Runtime::new().expect("Unable to setup tokio runtime");
    let future = run(par);
    rt.block_on(future);
}


async fn run(par : Par) {
    let (store,mut coordinator) = storage::Coordinator::create();

    tokio::task::spawn(async move{
        test(store, par).await
    });

    
    coordinator.run().await;
}

async fn test<S>(storage : S, par : Par) where S : Storage + std::marker::Send + std::marker::Sync + 'static {
    use model::*;

    
    /* 
    let mut par = Par::default();
    par.sends.push({
        let mut chan = Par::default();
        chan.exprs.push(Expr {
            expr_instance : Some( expr::ExprInstance::GInt(0))
        });

        let mut data1 = Par::default();
        data1.exprs.push(Expr {
            expr_instance : Some( expr::ExprInstance::GInt(7))
        });

        let mut data2 = Par::default();
        data2.exprs.push(Expr {
            expr_instance : Some( expr::ExprInstance::GInt(8))
        });

        let mut data3 = Par::default();
        data3.exprs.push(Expr {
            expr_instance : Some( expr::ExprInstance::GInt(9))
        });
        
        Send{
            chan : Some(chan),
            data : vec![ data1, data2, data3 ],
            locally_free : None,
            persistent : false,
            connective_used : false,
        }
    });


    par.receives.push({
        let mut receive = Receive::default();

        receive.binds.push({
            let mut receive_bind = ReceiveBind::default();

            receive_bind.patterns.push({
                let mut pattern_par = Par::default();
                pattern_par.exprs.push(Expr {
                    expr_instance : Some(expr::ExprInstance::EVarBody(EVar{
                        v : Some(Var{ 
                            var_instance : Some(var::VarInstance::FreeVar(0))
                        })
                    }))
                });
                pattern_par
            });

            receive_bind.patterns.push({
                let mut pattern_par = Par::default();
                 pattern_par.exprs.push(Expr {
                    expr_instance : Some(expr::ExprInstance::EVarBody(EVar{
                        v : Some(Var{ 
                            var_instance : Some(var::VarInstance::FreeVar(1))
                        })
                    }))
                });
                pattern_par
            });

            receive_bind.patterns.push({
                let mut pattern_par = Par::default();
                pattern_par.exprs.push(Expr {
                    expr_instance : Some(expr::ExprInstance::EVarBody(EVar{
                        v : Some(Var{ 
                            var_instance : Some(var::VarInstance::FreeVar(2))
                        })
                    }))
                });
                pattern_par
            });
            
            receive_bind.source = Some({
                let mut source_par = Par::default();
                source_par.exprs.push(Expr {
                    expr_instance : Some(expr::ExprInstance::GInt(0))
                });
                source_par
            });
            receive_bind            
        });

        receive.body = Some(
            {
                let mut body_par = Par::default();
                body_par
            }
        );
        receive.persistent = false;
        receive.peek = false;
        receive.bind_count = 3;

        receive
    });
    */
    let context = std::sync::Arc::new(interpreter::InterpreterContext::from(storage));

    let now = Instant::now();
    let errors = context.evaludate(par).await;
    println!("Reduction took {} ms", now.elapsed().as_millis());
    for err in errors {
        error!("Error #{} : {}", err.kind, err.message);
    }
    
    

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