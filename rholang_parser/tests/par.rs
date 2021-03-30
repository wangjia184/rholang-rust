
mod common;

use model::*;


#[test]
fn ppar_should_compile_both_branches_into_a_par_object() {
    let rholang_code = "7|8";

    let mut result = common::run_normalizer(rholang_code);
    let root_par = result.par.take().unwrap();
    assert_eq!( root_par.exprs.len(), 2);
    match &root_par.exprs[0] {
        Expr {
            expr_instance : Some(expr::ExprInstance::GInt(8))
        } => {
        },
        _ => panic!("{:#?}", root_par.exprs[0]),
    };
    
    match &root_par.exprs[1] {
        Expr {
            expr_instance : Some(expr::ExprInstance::GInt(7))
        } => {
        },
        _ => panic!("{:#?}", root_par.exprs[0]),
    };
}




#[test]
fn ppar_should_not_compile_if_both_branches_use_same_free_variable() {
    let rholang_code = "x | x";

    let result = common::run_normalizer(rholang_code);
    assert_eq!(result.syntax_errors.len(), 1);
    match &result.syntax_errors[0] {
        SyntaxError { kind, .. } => {
            assert_eq!( *kind, SyntaxErrorKind::UnexpectedReuseOfProcessContextFree as i32);
            },
        //_ => panic!("{:#?}", &result),
    }
}

