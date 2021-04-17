use tokio_test;
use std::sync::Arc;
use super::*;

/* 

#[test]
fn expression_should_handle_simple_addition() {
    
    let mut p1 = Par::default();
    p1.exprs.push(Expr {
        expr_instance : Some( ExprInstance::GInt(7))
    });

    let mut p2 = Par::default();
    p2.exprs.push(Expr {
        expr_instance : Some( ExprInstance::GInt(8))
    });

    let mut par = Par::default();
    par.exprs.push(Expr {
        expr_instance : Some( ExprInstance::EPlusBody(EPlus {
            p1 : Some(p1),
            p2 : Some(p2),
        }))
    });

    let context = Arc::new(InterpreterContext::default());
    let env = Env::<Par>::default();
    tokio_test::block_on( par.evaluate_nested_expressions(&context, &env) ).unwrap();

    assert_eq!( par.exprs.len(), 1 );

    match &par.exprs[0] {
        Expr {
            expr_instance : Some(ExprInstance::GInt(num))
        } => {
            assert_eq!( *num, 15);
        },
        _ => {
            panic!("{:#?}", &par.exprs[0]);
        }
    }

    
}


#[test]
fn expression_should_handle_long_addition() {
    let mut p1 = Par::default();
    p1.exprs.push(Expr {
        expr_instance : Some( ExprInstance::GInt(i32::MAX as i64))
    });

    let mut p2 = Par::default();
    p2.exprs.push(Expr {
        expr_instance : Some( ExprInstance::GInt(i32::MAX as i64))
    });

    let mut par = Par::default();
    par.exprs.push(Expr {
        expr_instance : Some( ExprInstance::EPlusBody(EPlus {
            p1 : Some(p1),
            p2 : Some(p2),
        }))
    });

    let context = Arc::new(InterpreterContext::default());
    let env = Env::<Par>::default();
    tokio_test::block_on( par.evaluate_nested_expressions(&context, &env) ).unwrap();

    assert_eq!( par.exprs.len(), 1 );

    match &par.exprs[0] {
        Expr {
            expr_instance : Some(ExprInstance::GInt(num))
        } => {
            assert_eq!( *num, 2 * i32::MAX as i64);
        },
        _ => {
            panic!("{:#?}", &par.exprs[0]);
        }
    }
}



#[test]
fn expression_should_not_overflow_in_addition() {
    let mut p1 = Par::default();
    p1.exprs.push(Expr {
        expr_instance : Some( ExprInstance::GInt(i64::MAX))
    });

    let mut p2 = Par::default();
    p2.exprs.push(Expr {
        expr_instance : Some( ExprInstance::GInt(i64::MAX))
    });

    let mut par = Par::default();
    par.exprs.push(Expr {
        expr_instance : Some( ExprInstance::EPlusBody(EPlus {
            p1 : Some(p1),
            p2 : Some(p2),
        }))
    });

    let context = Arc::new(InterpreterContext::default());
    let env = Env::<Par>::default();
    tokio_test::block_on( par.evaluate_nested_expressions(&context, &env) ).unwrap();

    assert_eq!( par.exprs.len(), 1 );

    match &par.exprs[0] {
        Expr {
            expr_instance : Some(ExprInstance::GInt(num))
        } => {
            assert_eq!( *num, -2);
        },
        _ => {
            panic!("{:#?}", &par.exprs[0]);
        }
    }
    
}

*/