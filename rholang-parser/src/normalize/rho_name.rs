

use super::super::bnfc;
use super::*;


impl super::Normalizer {

    pub fn normalize_name(&mut self, n : &RawName,  input: &NameVisitInputs) -> Result<NameVisitOutputs, CompiliationError> {
        
        match n.kind {
            bnfc::Name__is_NameWildcard => {
                let source_position = SourcePosition::new(n.line_number, n.char_number, 1);
                let wildcard_bind_request = input.known_free.clone_then_add_wildcard(source_position);
                
                Ok(NameVisitOutputs {
                    chan : Par::new_wildcard_var(),
                    known_free : Rc::new(wildcard_bind_request),
                })
            },
            bnfc::Name__is_NameVar => {
                let var_ = unsafe { n.u.namevar_.var_ };
                let var_name : String = match self.get_string(var_) {
                    Err(e) => {
                        return Err( CompiliationError::new_utf8_error(&e, n.line_number, n.char_number) )
                    },
                    Ok(s) => s,
                };
                match input.env.get(&var_name) {
                    Some(idx_ctx) => {
                        match idx_ctx.var_sort {
                            VarSort::Process => {
                                let source_position = SourcePosition::new(n.line_number, n.char_number, var_name.len());
                                self.syntax_errors.push( SyntaxError::new_unexpected_name_context(&var_name, source_position, idx_ctx.source_position) );
                                // return a default instance so that the traverse can continue
                                Ok(NameVisitOutputs::default())
                            },
                            VarSort::Name => {
                                Ok(NameVisitOutputs {
                                    chan : Par::new_bound_var(idx_ctx.index),
                                    known_free : input.known_free.clone(),
                                })
                            },
                            // _ => {
                            //     Err( CompiliationError::new_unrecognized_data("VarSort", idx_ctx.var_sort as u32, n.line_number, n.char_number) )
                            // }
                        }
                    },
                    None => {
                        let source_position = SourcePosition::new(n.line_number, n.char_number, var_name.len());
                        match input.known_free.get(&var_name) {
                            None => {
                                let new_binding_pair = input.known_free.clone_then_put((var_name, VarSort::Name, source_position));
                                Ok(NameVisitOutputs {
                                    chan : Par::new_free_var(input.known_free.next_level),
                                    known_free : Rc::new(new_binding_pair),
                                })
                            },
                            Some(level_ctx) => {
                                self.syntax_errors.push(SyntaxError::new_unexpected_reuse_of_name_context_free(&var_name, source_position, (*(level_ctx.source_position)).clone()));
                                Ok(NameVisitOutputs::default()) // return a dummy output so that the traverse will not be interrupted by syntax error
                            }
                        }
                    }
                }
            },
            bnfc::Name__is_NameQuote => {
                let proc_ = unsafe { n.u.namequote_.proc_ };
                self.normalize_proc(proc_, &ProcVisitInputs{
                    par: Par::default(),
                    env: input.env.clone(),
                    known_free : input.known_free.clone(),
                })
                .and_then( |body| 
                    Ok(NameVisitOutputs{
                        chan : body.par,
                        known_free : Rc::new(body.known_free),
                    })
                )
            },
            _ => {
                Err( CompiliationError::new_unrecognized_data("bnfc::Name", n.kind, n.line_number, n.char_number) )
            }
        }
    }

    
    
}