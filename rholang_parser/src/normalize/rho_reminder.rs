use model::*;

use super::*;


impl super::Normalizer {

    pub fn normalize_name_reminder(&mut self, reminder : &RawNameRemainder, known_free: Rc<DeBruijnLevelMap>) -> Result<(Option<Var>, Rc<DeBruijnLevelMap>), CompiliationError> {
        match reminder.kind {
            bnfc::NameRemainder__is_NameRemainderEmpty => {
                Ok( (None, known_free) )
            },
            bnfc::NameRemainder__is_NameRemainderVar => {
                let procvar_ = unsafe { reminder.u.nameremaindervar_.procvar_ };
                if procvar_ == 0 as bnfc::ProcVar {
                    return Err(CompiliationError::new_null_pointer("nameremaindervar_.procvar_"));
                }
                let procvar = unsafe { *procvar_ };
                self.normalize_proc_var( procvar, known_free)
            },
            _ => { 
                Err(CompiliationError::new_unrecognized_data( "namereminder.kind", reminder.kind, reminder.line_number, reminder.char_number))
            }
        }
    }

    fn normalize_proc_var(&mut self, procvar : RawProcVar, known_free: Rc<DeBruijnLevelMap>) -> Result<(Option<Var>, Rc<DeBruijnLevelMap>), CompiliationError> {
        match procvar.kind {
            bnfc::ProcVar__is_ProcVarWildcard => {
                let var = Var {
                    var_instance : Some(var::VarInstance::Wildcard(var::WildcardMsg::default()))
                };
                let source_position = SourcePosition::new(procvar.line_number, procvar.char_number, 1);
                Ok((
                    Some(var), 
                    Rc::new( known_free.clone_then_add_wildcard(source_position) )
                ))
            },
            bnfc::ProcVar__is_ProcVarVar => {
                let var_ = unsafe { procvar.u.procvarvar_.var_ };
                if var_ == 0 as bnfc::Var {
                    return Err(CompiliationError::new_null_pointer("procvarvar_.var_"));
                }

                let var_name : String = match self.get_string(var_) {
                    Err(e) => {
                        return Err( CompiliationError::new_utf8_error(&e, procvar.line_number, procvar.char_number) )
                    },
                    Ok(s) => s,
                };

                let source_position = SourcePosition::new(procvar.line_number, procvar.char_number, var_name.len());
                match known_free.get(&var_name) {
                    None => {
                        let var = Var {
                            var_instance : Some(var::VarInstance::FreeVar(known_free.next_level))
                        };
                        Ok( (
                            Some(var),
                            Rc::new( known_free.clone_then_put((var_name, VarSort::Process, source_position)) )
                        ))
                    },
                    Some(rc) => {
                        let contra_position : SourcePosition = match *rc {
                            LevelContext { source_position : ref sp, .. } => {
                                (*(*sp)).clone()
                            }
                        };
                        self.syntax_errors.push(
                            SyntaxError::new_unexpected_reuse_of_process_context_free(&var_name, source_position, contra_position)
                        );
                        // return a dummy result so that the AST tree can still be traversed
                        Ok( (None, known_free) )
                    }
                }
            },
            _ => { 
                Err(CompiliationError::new_unrecognized_data( "procvar.kind", procvar.kind, procvar.line_number, procvar.char_number))
            }
        }
    }
}