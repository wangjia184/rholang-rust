extern crate pretty_env_logger;
#[macro_use] extern crate log;


use interpreter::AsyncEvaluator;
use storage::Storage;
use tokio::runtime;


mod interpreter;
mod storage;


fn main() {

    pretty_env_logger::init();

    
    let rt = runtime::Runtime::new().expect("Unable to setup tokio runtime");
    let future = run();
    rt.block_on(future);
}


async fn run() {
    let (store,mut coordinator) = storage::Coordinator::create();

    tokio::task::spawn(async move{
        test(store).await
    });

    
    coordinator.run().await;
}

async fn test<S>(storage : S) where S : Storage + std::marker::Send + std::marker::Sync + 'static {
    use model::*;

    

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

    let context = std::sync::Arc::new(interpreter::InterpreterContext::from(storage));
    let env = interpreter::Env::<Par>::default();
    par.evaluate(&context, &env).await.expect("");

}