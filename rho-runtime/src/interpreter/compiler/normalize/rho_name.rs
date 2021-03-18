use protobuf::RepeatedField;



use super::super::bnfc;
use super::super::errors::*;
use super::*;

impl super::Normalizer {

    pub fn normalize_name(&mut self, n : &RhoName,  input: &NameVisitInputs) -> Option<NameVisitOutputs> {
        let source_position = SourcePosition::new(n.line_number, n.char_number, 1);

        match n.kind {
            bnfc::Name__is_NameWildcard => {
                let wildcard_bind_request = input.known_free.add_wildcard(source_position);
                let mut var = Var::new();
                var.set_wildcard(Var_WildcardMsg::default());
                return Some(NameVisitOutputs {
                    par : self.create_par_with_var(var),
                    known_free : Rc::new(wildcard_bind_request),
                })
            },
            bnfc::Name__is_NameVar => {
                let var_ = unsafe { n.u.namevar_.var_ };
                let var_name : String = match self.get_string(var_) {
                    Err(e) => {
                        self.faulty_errors.push(CompliationError::SourceUtf8Error(e));
                        return None;
                    },
                    Ok(s) => s,
                };
                match input.env.get(&var_name) {
                    Some(idx_ctx) => {
                        match idx_ctx.var_sort {
                            VarSort::Process => {
                                self.syntax_errors.push(
                                    (
                                        SyntaxError::UnexpectedNameContext(var_name),
                                        Some(idx_ctx.source_position),
                                        Some(source_position),
                                    )
                                );
                            },
                            VarSort::Name => {
                                let mut var = Var::new();
                                var.set_bound_var(idx_ctx.index);
                                return Some(NameVisitOutputs {
                                    par : self.create_par_with_var(var),
                                    known_free : input.known_free.clone(),
                                })
                            }
                        }
                    },
                    None => {
                        warn!("This branch in normalize_name() requires test!");
                        match input.known_free.get(&var_name) {
                            None => {
                                let source_position = SourcePosition::new(n.line_number, n.char_number, var_name.len());
                                let new_binding_pair = input.known_free.put((var_name, VarSort::Name, source_position));
                                let mut var = Var::new();
                                var.set_free_var(input.known_free.next_level);
                                return Some(NameVisitOutputs {
                                    par : self.create_par_with_var(var),
                                    known_free : Rc::new(new_binding_pair),
                                })
                            },
                            Some(level_ctx) => {
                                self.syntax_errors.push(
                                    (
                                        SyntaxError::UnexpectedReuseOfNameContextFree(var_name),
                                        Some((*(level_ctx.source_position)).clone()),
                                        Some(source_position),
                                    )
                                );
                            }
                        }
                    }
                };
            },
            bnfc::Name__is_NameQuote => {
                let proc_ = unsafe { n.u.namequote_.proc_ };
                return self.normalize(proc_, ProcVisitInputs{
                            par: Par::default(),
                            env: input.env.clone(),
                            known_free : input.known_free.clone(),
                        })
                        .and_then( |body| 
                            Some(NameVisitOutputs{
                                par : body.par,
                                known_free : Rc::new(body.known_free),
                            })
                        );
            },
            _ => {
                self.faulty_errors.push(CompliationError::UnrecognizedKind(n.kind, "bnfc::Name".to_string()));
            }
        };

 
        None
    }

    fn create_par_with_var(&self, var : Var) -> Par {
        let mut evar = EVar::new();
        evar.set_v(var);
        let mut expr = Expr::new();
        expr.set_e_var_body(evar);
        let mut par = Par::new();
        par.set_exprs(RepeatedField::from(vec![expr]));
        par
    }
    
}