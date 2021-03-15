extern crate pretty_env_logger;
#[macro_use] extern crate log;

mod interpreter;
mod model;



fn main() {

    pretty_env_logger::init();

    interpreter::test();
    
 
}
