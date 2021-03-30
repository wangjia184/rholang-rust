
mod common;

use model::*;


#[test]
fn pinput_should_handle_a_simple_receive() {
    let rholang_code = "for ( x, y <- @1 ) { x!(*y) }";

    let mut result = common::run_normalizer(rholang_code);
    let root_par = result.par.take().unwrap();

    assert_eq!( root_par.receives.len(), 1);
    let rho_receive = &root_par.receives[0];
    assert_eq!( rho_receive.binds.len(), 2);
    println!("{:?}", &rho_receive);
}



