use tokio_test;
use std::sync::Arc;
use super::*;


#[test]
fn eval_of_single_channel_should_place_something_in_tuplespace() {
    
    let mut par = Par::default();
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
                    expr_instance : Some(expr::ExprInstance::GString("channel".to_string()))
                });
                source_par
            });
            receive_bind            
        });

        receive.body = Some(Par::default());
        receive.persistent = false;
        receive.peek = false;
        receive.bind_count = 3;

        receive
    });
    

    //let context = Arc::new(InterpreterContext::default());
    //let env = Env::<Par>::default();
    //tokio_test::block_on( par.evaluate(&context, &env) ).unwrap();

    
}
