

use super::super::bnfc;
use super::*;


impl super::Normalizer {

    pub fn normalize_pvar(&mut self, proc : &RawProc,  input: &ProcVisitInputs) -> Result<ProcVisitOutputs, CompiliationError> {
        let pvar = unsafe { *proc.u.pvar_.procvar_ };
        let var_ = unsafe { pvar.u.procvarvar_.var_ };
        let var_name : String = match self.get_string(var_) {
            Err(e) => {
                return Err( CompiliationError::new_utf8_error(&e, pvar.line_number, pvar.char_number) )
            },
            Ok(s) => s,
        };

        match pvar.kind {
            bnfc::ProcVar__is_ProcVarVar => {
                let source_position = SourcePosition::new(pvar.line_number, pvar.char_number, var_name.len());

                match input.env.get(&var_name) {
                    Some(IndexContext{ var_sort : VarSort::Process, index, .. }) => {
                        let expression = Expr {
                            expr_instance : Some(expr::ExprInstance::EVarBody(EVar{
                                v : Some(Var {
                                    var_instance : Some(var::VarInstance::BoundVar(index))
                                })
                            }))
                        };
                        let par = input.par.clone_then_prepend_expr(expression, input.env.depth());
                        Ok(ProcVisitOutputs { par : par, known_free : (*input.known_free).clone() })
                    },
                    Some(IndexContext{ var_sort : VarSort::Name, source_position : contra_position, .. }) => {
                        self.syntax_errors.push(
                            SyntaxError::new_unexpected_process_context(&var_name, source_position, contra_position)
                        );
                        // return a dummy result so that the AST tree can still be traversed
                        Ok(ProcVisitOutputs { par : Par::default(), known_free : (*input.known_free).clone() })
                    },
                    None => {
                        match input.known_free.get(&var_name) {
                            None => {
                                let new_binding_pair = input.known_free.clone_then_put((var_name, VarSort::Process, source_position));
                                let expression = Expr {
                                    expr_instance : Some(expr::ExprInstance::EVarBody(EVar{
                                        v : Some(Var {
                                            var_instance : Some(var::VarInstance::FreeVar(input.known_free.next_level))
                                        })
                                    }))
                                };
                                let mut par = input.par.clone_then_prepend_expr(expression, input.env.depth());
                                par.connective_used = true;
                                Ok(ProcVisitOutputs { par : par, known_free : new_binding_pair })
                            },
                            Some(rc) => {
                                let contra_position = (*rc.source_position).clone();
                                self.syntax_errors.push(
                                    SyntaxError::new_unexpected_reuse_of_process_context_free(&var_name, source_position, contra_position)
                                );
                                // return a dummy result so that the AST tree can still be traversed
                                Ok(ProcVisitOutputs { par : Par::default(), known_free : (*input.known_free).clone() })
                            }
                        }
                    }
                }

                
            },
            bnfc::ProcVar__is_ProcVarWildcard => {
                let source_position = SourcePosition::new(pvar.line_number, pvar.char_number, 1);

                let expression = Expr {
                    expr_instance : Some(expr::ExprInstance::EVarBody(EVar{
                        v : Some(Var {
                            var_instance : Some(var::VarInstance::Wildcard(var::WildcardMsg{}))
                        })
                    }))
                };
                let mut par = input.par.clone_then_prepend_expr(expression, input.env.depth());
                par.connective_used = true;
                let known_free = input.known_free.clone_then_add_wildcard(source_position);
                Ok(ProcVisitOutputs { par : par, known_free : known_free })
            },
            _ => {
                Err( CompiliationError::new_unrecognized_data("pvar_.procvar_.kind", pvar.kind, pvar.line_number, pvar.char_number) )
            }
        }
    }

    
    
}