

#[cfg(test)]
mod ppar_tests {
    use model::*;
    use super::super::*;
    use super::super::super::*;


    

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

    #[test]
    fn ppar_should_compile_both_branches_with_same_environment() {
        let rholang_code = "x | x";

        let proc = builder::parse(rholang_code).unwrap();

        let inputs = ProcVisitInputs {
            par : Par::default(),
            env : Rc::new(IndexMapChain::empty().add_bindings_to_head(vec![("x".to_string(), VarSort::Process, SourcePosition::new(0,0,0))])),
            known_free : Rc::new(DeBruijnLevelMap::empty()),
        };

        let mut normalizer = Normalizer::default();
        let outputs = normalizer.normalize_proc(proc, &inputs).unwrap();
        assert_eq!( outputs.par.exprs.len(), 2);
        match &outputs.par.exprs[0] {
            Expr {
                expr_instance : Some(expr::ExprInstance::EVarBody(
                    EVar {
                        v : Some(Var {
                            var_instance : Some(var::VarInstance::BoundVar(0))
                        })
                    }
                ))
            } => {},
            _ => panic!("{:?}", &outputs.par.exprs[0]),
        };
        match &outputs.par.exprs[1] {
            Expr {
                expr_instance : Some(expr::ExprInstance::EVarBody(
                    EVar {
                        v : Some(Var {
                            var_instance : Some(var::VarInstance::BoundVar(0))
                        })
                    }
                ))
            } => {},
            _ => panic!("{:?}", &outputs.par.exprs[1]),
        };
    }
    

}