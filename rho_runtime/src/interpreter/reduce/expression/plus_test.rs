use std::sync::Arc;
use super::super::DebruijnInterpreter;

use model::{ *, expr::* };

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

    let reducer = Arc::new(DebruijnInterpreter::default());
    let par = reducer.evaluate_expression(par).unwrap();

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

    let reducer = Arc::new(DebruijnInterpreter::default());
    let par = reducer.evaluate_expression(par).unwrap();

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

    let reducer = Arc::new(DebruijnInterpreter::default());
    let par = reducer.evaluate_expression(par).unwrap();

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