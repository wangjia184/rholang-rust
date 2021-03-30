
mod common;

use model::*;


#[test]
fn pinput_should_handle_a_simple_receive() {
    let rholang_code = "for ( x, y <- @1 ) { x!(*y) }";

    let mut result = common::run_normalizer(rholang_code);
    println!("{:?}", &result);
}



