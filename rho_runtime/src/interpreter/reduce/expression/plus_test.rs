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
    reducer.evaluate_expression(par).unwrap();
}