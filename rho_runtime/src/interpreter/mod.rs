mod substitute;
use std::sync::Arc;

use model::*;

mod reduce;



pub async fn test_reduce() {

    let mut chan = Par::default();
    chan.exprs.push(Expr {
        expr_instance : Some( expr::ExprInstance::GString("channel".to_string()))
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

    let mut par = Par::default();
    par.sends.push(Send{
        chan : Some(chan),
        data : vec![ data1, data2, data3 ],
        locally_free : None,
        persistent : false,
        connective_used : false,
    });

    let reducer = Arc::new(reduce::DebruijnInterpreter::default());
    reducer.evaluate(par).await;
}