

#[cfg(test)]
mod ppar_tests {
    use model::*;
    use super::super::super::builder;


    

    #[test]
    fn ppar_should_compile_both_branches_into_a_par_object() {
        let rholang_code = "7|8";

        let mut normalizer = builder::build_ast_from_string(rholang_code);
        let root_par = normalizer.par.take().unwrap();
        assert_eq!( root_par.exprs.len(), 2);
        match &root_par.exprs[0] {
            Expr {
                expr_instance : Some(expr::ExprInstance::GInt(8))
            } => {
            },
            _ => panic!("{:?}", root_par.exprs[0]),
        };
        
        match &root_par.exprs[1] {
            Expr {
                expr_instance : Some(expr::ExprInstance::GInt(7))
            } => {
            },
            _ => panic!("{:?}", root_par.exprs[0]),
        };
    }


    

}