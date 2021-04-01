extern crate pretty_env_logger;
//#[macro_use] extern crate log;
use tokio::runtime;


mod interpreter;


// https://blog.logrocket.com/a-practical-guide-to-async-in-rust/
fn main() {

    pretty_env_logger::init();

    
    let rt = runtime::Runtime::new().expect("Unable to setup tokio runtime");
    let future = run();
    rt.block_on(future);
}


async fn run() {
    interpreter::test_reduce().await;
    // block to test
    std::thread::sleep(std::time::Duration::from_secs(9999));
}