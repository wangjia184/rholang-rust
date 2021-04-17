extern crate pretty_env_logger;
#[macro_use] extern crate log;
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
    //interpreter::test_reduce().await;

    let (store,mut dispatcher) = storage::Dispatcher::create();
    dispatcher.run().await;
}