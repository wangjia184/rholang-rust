

#[cfg(test)]
mod ppar_tests {
    use model::*;
    use super::super::*;
    use super::super::super::*;


    

    #[test]
    fn ppar_should_compile_both_branches_into_a_par_object() {
        let rholang_code = "7|8";

        let mut result = builder::build_ast_from_string(rholang_code);
        let root_par = result.par.take().unwrap();
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


    #[test]
    fn ppar_should_not_compile_if_both_branches_use_same_free_variable() {
        let rholang_code = "x | x";

        let result = builder::build_ast_from_string(rholang_code);
        assert_eq!(result.syntax_errors.len(), 1);
        match &result.syntax_errors[0] {
            SyntaxError { kind, .. } => {
                assert_eq!( *kind, SyntaxErrorKind::UnexpectedReuseOfProcessContextFree as i32);
             },
            _ => panic!("{:?}", &result),
        }
    }
    
    #[test]
    fn ppar_should_accumulate_free_counts_from_both_branches() {
        let rholang_code = "x | y";

        let proc = builder::parse(rholang_code).unwrap();
        let mut normalizer = Normalizer::default();
        let outputs = normalizer.normalize(proc).unwrap();
        let par = outputs.par;
        assert_eq!(par.connective_used, true);
        assert_eq!( par.exprs.len(), 2);
        match &par.exprs[0] {
            Expr {
                expr_instance : Some(expr::ExprInstance::EVarBody(
                    EVar {
                        v : Some(Var {
                            var_instance : Some(var::VarInstance::FreeVar(1))
                        })
                    }
                ))
            } => {},
            _ => panic!("{:?}", &par.exprs[0]),
        };
        match &par.exprs[1] {
            Expr {
                expr_instance : Some(expr::ExprInstance::EVarBody(
                    EVar {
                        v : Some(Var {
                            var_instance : Some(var::VarInstance::FreeVar(0))
                        })
                    }
                ))
            } => {},
            _ => panic!("{:?}", &par.exprs[0]),
        }

        match outputs.known_free.get("x") {
            Some(ctx) => {
                assert_eq!(ctx.level, 0);
                assert_eq!(ctx.var_sort, VarSort::Process);
            },
            _ => panic!("x not found"),
        };
        match outputs.known_free.get("y") {
            Some(ctx) => {
                assert_eq!(ctx.level, 1);
                assert_eq!(ctx.var_sort, VarSort::Process);
            },
            _ => panic!("y not found"),
        };
    }
}