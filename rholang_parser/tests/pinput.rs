
mod common;

use model::*;

fn validate_single_bound_variable (exprs : &Vec<Expr>, bound_var : i32) {
    assert_eq!( exprs.len(), 1);
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
            assert_eq!( *i, bound_var);
        },
        _ => panic!("{:#?}", &exprs[0]),
    }
}
fn validate_receive_bind_has_single_free_var( p : &Par, free_var : i32 ) {
            
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
                    validate_receive_bind_has_single_free_var( &patterns[0], 0);
                    validate_receive_bind_has_single_free_var( &patterns[1], 1);
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
                    validate_single_bound_variable( &exprs, 1);
                    assert_eq!( bs.len(), 1);
                    assert!( bs.contains(1) );
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
                    validate_single_bound_variable( &exprs, 0);
                    assert_eq!( bs.len(), 1);
                    assert!( bs.contains(0) );
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


#[test]
fn pinput_should_handle_a_more_complicated_receive() {
    let rholang_code = "for ( x1, @y1 <- @Nil ; x2, @y2 <- @1) { x1!(y2) | x2!(y1) }";

    let mut result = common::run_normalizer(rholang_code);
    //println!("{:#?}", &result);
    let root_par = result.par.take().unwrap();
    
    assert_eq!( root_par.receives.len(), 1);
    let rho_receive = &root_par.receives[0];
    let body = match &rho_receive {
        Receive {
            binds,
            persistent : false,
            peek : false,
            bind_count : 4,
            body : Some(body),
            locally_free,
            ..
        } => {
            assert!( locally_free.is_none() || (*locally_free).as_ref().unwrap().len() == 0);
            assert_eq!( binds.len(), 2);
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
                    validate_receive_bind_has_single_free_var( &patterns[0], 0);
                    validate_receive_bind_has_single_free_var( &patterns[1], 1);
                },
                _ => {
                    panic!("{:#?}", &receive_bind);
                }
            };

            let receive_bind = &binds[1];

            match receive_bind {
                ReceiveBind {
                    free_count : 2,
                    source : Some(Par {
                        connective_used : false,
                        locally_free : None,
                        exprs,
                        ..
                    }),
                    remainder : None,
                    patterns,
                    ..
                } => {
                    assert_eq!( patterns.len(), 2);
                    validate_receive_bind_has_single_free_var( &patterns[0], 0);
                    validate_receive_bind_has_single_free_var( &patterns[1], 1);
                    assert_eq!( exprs.len(), 1);
                    match &exprs[0] {
                        Expr {
                            expr_instance: Some( expr::ExprInstance::GInt(1) )
                        } => { },
                        _ => panic!("{:#?}", &exprs[0]),
                    };
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

    match body.locally_free {
        Some(bitset) => {
            assert_eq!( bitset.len(), 4);
            assert!( bitset.contains(0) && bitset.contains(1) && bitset.contains(2) && bitset.contains(3) );
        },
        None => panic!("{:#?}", &body.locally_free),
    };
    
    assert_eq!( body.sends.len(), 2);

    let rho_send = &body.sends[0];

    match rho_send {
        Send {
            chan : Some(Par {
                exprs,
                locally_free : Some(chan_bitset),
                connective_used : false,
                ..
            }),
            data,
            persistent : false,
            connective_used : false,
            locally_free : Some(bitset),
            ..
        } => {
            assert_eq!( bitset.len(), 2);
            assert!( bitset.contains(1) && bitset.contains(2) );
            validate_single_bound_variable( &exprs, 1);
            assert_eq!( chan_bitset.len(), 1);
            assert!( chan_bitset.contains(1) );
            assert_eq!( data.len(), 1);
            match &data[0] {
                Par {
                    exprs,
                    locally_free : Some(data_bitset),
                    ..
                } => {
                    validate_single_bound_variable(&exprs, 2);
                    assert_eq!( data_bitset.len(), 1);
                    assert!( data_bitset.contains(2) );
                },
                _ => {
                    panic!("{:#?}", &data[0]);
                }
            }
            

        }
        _ => {
            panic!("{:#?}", &rho_send);
        }
    }


    let rho_send = &body.sends[1];

    match rho_send {
        Send {
            chan : Some(Par {
                exprs,
                locally_free : Some(chan_bitset),
                connective_used : false,
                ..
            }),
            data,
            persistent : false,
            connective_used : false,
            locally_free : Some(bitset),
            ..
        } => {
            assert_eq!( bitset.len(), 2);
            assert!( bitset.contains(0) && bitset.contains(3) );
            validate_single_bound_variable( &exprs, 3);
            assert_eq!( chan_bitset.len(), 1);
            assert!( chan_bitset.contains(3) );
            assert_eq!( data.len(), 1);
            match &data[0] {
                Par {
                    exprs,
                    locally_free : Some(data_bitset),
                    ..
                } => {
                    validate_single_bound_variable(&exprs, 0);
                    assert_eq!( data_bitset.len(), 1);
                    assert!( data_bitset.contains(0) );
                },
                _ => {
                    panic!("{:#?}", &data[0]);
                }
            }
            

        }
        _ => {
            panic!("{:#?}", &rho_send);
        }
    }
}


#[test]
fn pinput_should_fail_if_free_variable_is_used_in_2_different_receives() {
    let rholang_code = "for ( x1, @y1 <- @Nil ; x2, @y1 <- @1) { Nil }";

    let result = common::run_normalizer(rholang_code);
    if !result.syntax_errors.iter().any( |ref err| {
        return err.kind == SyntaxErrorKind::UnexpectedReuseOfNameContextFree as i32;
    }) {
        panic!("{:#?}", &result);
    }
    
}

