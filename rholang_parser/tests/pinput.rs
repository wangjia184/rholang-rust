
mod common;

use model::*;


fn validate_receive_bind_partner( p : &Par, free_var : i32 ) {
            
    let expression = match p {
        Par {
            connective_used: true,
            exprs,
            ..
        } if exprs.len() == 1 => &exprs[0],
        _ => panic!("{:#?}", p),
    };
    
    match expression {
        Expr {
            expr_instance : Some(
                expr::ExprInstance::EVarBody(
                    EVar {
                        v : Some(
                            Var {
                                var_instance : Some(
                                    var::VarInstance::FreeVar(i)
                                )
                            }
                        )
                    }
                )
            )
        }
         => {
             assert_eq!( *i, free_var);
        },
        _ => panic!("{:#?}", expression),
    };
}

#[test]
fn pinput_should_handle_a_simple_receive() {
    let rholang_code = "for ( x, y <- @Nil ) { x!(*y) }";

    let mut result = common::run_normalizer(rholang_code);
    let root_par = result.par.take().unwrap();


    assert_eq!( root_par.receives.len(), 1);
    let rho_receive = &root_par.receives[0];
    let body = match &rho_receive {
        Receive {
            binds,
            persistent : false,
            peek : false,
            body : Some(body),
            ..
        } => {
            assert_eq!( binds.len(), 1);
            let receive_bind = &binds[0];

            match receive_bind {
                ReceiveBind {
                    free_count : 2,
                    source : Some(Par {
                        connective_used : false,
                        locally_free : None,
                        ..
                    }),
                    remainder : None,
                    patterns,
                    ..
                } => {
                    assert_eq!( patterns.len(), 2);
                    validate_receive_bind_partner( &patterns[0], 0);
                    validate_receive_bind_partner( &patterns[1], 1);
                },
                _ => {
                    panic!("{:#?}", &receive_bind);
                }
            };

            body.clone()
        },
        _ => {
            panic!("{:#?}", &rho_receive);
        }
    };


    assert_eq!( body.sends.len(), 1);
    
    match &body.sends[0] {
        Send {
            chan : Some(chan),
            data,
            persistent : false,
            connective_used : false,
            locally_free : Some(bitset),
            ..
        } => {
            assert_eq!( bitset.len(), 2);
            assert!( bitset.contains(0) && bitset.contains(1) );
            match &chan {
                Par {
                    exprs,
                    locally_free : Some(bs),
                    connective_used : false,
                    ..
                } => {
                    assert_eq!( exprs.len(), 1);
                    assert_eq!( bs.len(), 1);
                    assert!( bs.contains(1) );
                    match &exprs[0] {
                        Expr {
                            expr_instance : Some(
                                expr::ExprInstance::EVarBody(
                                    EVar {
                                        v : Some(
                                            Var {
                                                var_instance : Some(
                                                    var::VarInstance::BoundVar(i)
                                                )
                                            }
                                        )
                                    }
                                )
                            )
                        }
                        => {
                            assert_eq!( *i, 1);
                        },
                        _ => panic!("{:#?}", &exprs[0]),
                    }
                },
                _ => {
                    panic!("{:#?}", &chan);
                }
            };
            assert_eq!( data.len(), 1);
            
            match &data[0] {
                Par {
                    exprs,
                    locally_free : Some(bs),
                    connective_used : false,
                    ..
                } => {
                    assert_eq!( exprs.len(), 1);
                    assert_eq!( bs.len(), 1);
                    assert!( bs.contains(0) );
                    match &exprs[0] {
                        Expr {
                            expr_instance : Some(
                                expr::ExprInstance::EVarBody(
                                    EVar {
                                        v : Some(
                                            Var {
                                                var_instance : Some(
                                                    var::VarInstance::BoundVar(i)
                                                )
                                            }
                                        )
                                    }
                                )
                            )
                        }
                        => {
                            assert_eq!( *i, 0);
                        },
                        _ => panic!("{:#?}", &exprs[0]),
                    }
                },
                _ => {
                    panic!("{:#?}", &data[0]);
                }
            };
        },
        _ => {
            panic!("{:#?}", &body.sends[0]);
        }
    }

}


#[test]
fn pinput_should_handle_peek() {
    let rholang_code = "for ( x, y <<- @Nil ) { x!(*y) }";

    let mut result = common::run_normalizer(rholang_code);
    let root_par = result.par.take().unwrap();

    assert_eq!( root_par.receives.len(), 1);
    let rho_receive = &root_par.receives[0];
    assert_eq!( rho_receive.peek, true);
}
