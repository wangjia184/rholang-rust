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
    let (store,mut dispatcher) = storage::Dispatcher::create();

    tokio::task::spawn(async move{
        test(store).await
    });

    
    dispatcher.run().await;
}

async fn test<S>(storage : S) where S : Storage + std::marker::Send + std::marker::Sync + 'static {
    use model::*;

    let mut chan = Par::default();
    chan.exprs.push(Expr {
        expr_instance : Some( expr::ExprInstance::GInt(3))
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

    let context = std::sync::Arc::new(interpreter::InterpreterContext::from(storage));
    let env = interpreter::Env::<Par>::default();
    par.evaluate(&context, &env).await.expect("");

}