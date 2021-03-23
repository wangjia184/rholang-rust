

use std::cmp::Ordering;
use std::collections::hash_set::HashSet;

use super::*;
use super::super::bnfc;
use super::{ ProcVisitInputs, ProcVisitOutputs};


impl super::Normalizer {
    pub fn normalize_new(&mut self, proc : &RawProc, input: &ProcVisitInputs) -> Result<ProcVisitOutputs, CompiliationError> {
        let listnamedecl_ = unsafe { proc.u.pnew_.listnamedecl_ };
        let proc_ = unsafe { proc.u.pnew_.proc_ };

        if proc_ == 0 as bnfc::Proc {
            return Err(CompiliationError::new_null_pointer("pnew_.proc_"));
        }

        let mut list = self.list_name_decl(listnamedecl_)?;
    
        // This sorts the None's first, and the uris by lexicographical order.
        // We do this here because the sorting affects the numbering of variables inside the body.
        list.sort_by( |left, right| {
            match (&left.0, &right.0) {
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                (Some(ref l), Some(ref r)) => l.cmp(r),
                _ => left.1.cmp(&right.1),
            }
        });
    
        let uris : HashSet<String> = list
            .iter_mut()
            .filter_map(|tuple| tuple.0.take()).collect();
    
        let new_bindings : Vec<BoundVariable> = list
            .into_iter()
            .map(|tuple| (tuple.1, VarSort::Name, tuple.2))
            .collect();
    
        let new_env = (*input.env).clone().add_bindings_to_head(new_bindings);
        let new_binding_count = new_env.count() - input.env.count();
        let requires_deploy_id = uris.contains(constant::DEPLOY_ID_URI);
        let requires_deployer_id = uris.contains(constant::DEPLOYER_ID_URI);
    
        /* TODO: avoid hardcode
        def missingEnvElement(name: String, uri: String) =
            NormalizerError(s"`$uri` was used in rholang usage context where $name is not available.")
        if (requiresDeployId && env.get(deployIdUri).forall(_.singleDeployId().isEmpty))
            missingEnvElement("DeployId", deployIdUri).raiseError[M, ProcVisitOutputs]
        else if (requiresDeployerId && env.get(deployerIdUri).forall(_.singleDeployerId().isEmpty))
            missingEnvElement("DeployerId", deployerIdUri).raiseError[M, ProcVisitOutputs]
        */
    
        let proc_visit_inputs = ProcVisitInputs{
            par: Par::default(),
            env: Rc::new(new_env),
            known_free : input.known_free.clone(),
        };
        self.normalize_proc(proc_, &proc_visit_inputs).and_then( |body| {
            // val resultNew = New(
            //     bindCount = newCount,
            //     p = bodyResult.par,
            //     uri = uris,
            //     injections = env,
            //     locallyFree = bodyResult.par.locallyFree.from(newCount).map(x => x - newCount)
            //   )
            
            // Given this bitset [0 1 4 5 6 9]
            // suppose new_binding_count is 4
            // Create another bitset with [0 1 2 5]
            // The idea is to re-index of next level's variables 
            let locally_free : Option<BitSet> = body.par.locally_free.as_ref().map(|bitset| {
                bitset.iter().filter_map(|b| {
                    if b >= new_binding_count as usize {
                        Some(b - new_binding_count as usize)
                    } else {
                        None
                    }
                }).collect()
            });
            
            let mut rho_new = RhoNew::default();
            rho_new.bind_count = new_binding_count as i32;
            rho_new.p = Some(body.par);
            rho_new.injections = self.environment.clone();
            rho_new.uri = uris.into_iter().collect();
            rho_new.locally_free = locally_free;
    
            // TODO:
            // def prepend(n: New): Par =
            // p.copy(
            //   news = n +: p.news,
            //   locallyFree = p.locallyFree | n.locallyFree,
            //   connectiveUsed = p.connectiveUsed || n.connectiveUsed
            // )
            Ok(ProcVisitOutputs{ 
                par : input.par.clone_then_prepend_new(rho_new), // input.par.prepend(type_new),
                known_free : body.known_free 
            })
        })
    }


    fn list_name_decl(&mut self, mut listnamedecl : bnfc::ListNameDecl) -> Result< Vec<(Option<String>, String, SourcePosition)>, CompiliationError>
    {
        let mut list : Vec<(Option<String>, String, SourcePosition)> = Vec::new();
        while listnamedecl != 0 as bnfc::ListNameDecl
        {
            let p = unsafe { *listnamedecl };
            list.push(self.extract_name_decl(p.namedecl_)?);
            listnamedecl = p.listnamedecl_;
        }
        Ok(list)
    }


    fn extract_name_decl(&mut self, namedecl : bnfc::NameDecl) -> Result<(Option<String>, String, SourcePosition), CompiliationError>
    {
        if namedecl != 0 as bnfc::NameDecl {
            let p = unsafe { *namedecl };
            return match p.kind {
                bnfc::NameDecl__is_NameDeclSimpl => {
                    let var = unsafe { p.u.namedeclsimpl_.var_ };
                    
                    self.get_string(var)
                        .and_then(|name| {
                            let source_position = SourcePosition::new(p.line_number, p.char_number, name.len());
                            Ok((None, name, source_position))
                        })
                        .or_else(|e| {
                            Err( CompiliationError::new_utf8_error(&e, p.line_number, p.char_number) )
                        } )
                },
                bnfc::NameDecl__is_NameDeclUrn => {
                    let var = unsafe { p.u.namedeclurn_.var_ };
                    let uriliteral = unsafe { p.u.namedeclurn_.uriliteral_ };
                    match (self.get_string(var), self.get_string(uriliteral)) {
                        (Ok(name), Ok(uri)) => {
                            let source_position = SourcePosition::new(p.line_number, p.char_number, name.len());
                            Ok((Some(uri), name, source_position))
                        },
                        (Err(e), _) => {
                            Err( CompiliationError::new_utf8_error(&e, p.line_number, p.char_number) )
                        }
                        (_, Err(e)) => {
                            Err( CompiliationError::new_utf8_error(&e, p.line_number, p.char_number) )
                        }
                    }
                },
                _ => {
                    Err(CompiliationError::new_unrecognized_data("bnfc::NameDecl", p.kind, p.line_number, p.char_number) )
                }
            }
        }
        Err(CompiliationError::new_null_pointer("bnfc::NameDecl"))
    }

    
}





