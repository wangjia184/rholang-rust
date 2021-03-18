

use std::cmp::Ordering;
use std::collections::hash_set::HashSet;

use super::*;
use crate::model::constant;
use crate::model::rho_types;
use super::super::bnfc;

use super::super::errors::*;
use super::{ ProcVisitInputs, ProcVisitOutputs};


impl super::Normalizer {
    pub fn normalize_new(&mut self, proc : &RhoProc, input: &ProcVisitInputs) -> Option<ProcVisitOutputs> {
        let listnamedecl_ = unsafe { proc.u.pnew_.listnamedecl_ };
        let proc_ = unsafe { proc.u.pnew_.proc_ };

        if proc_ == 0 as bnfc::Proc {
            self.faulty_errors.push(CompliationError::NullPointer("pnew_.proc_".to_string()));
            return None;
        }

        let mut list = match self.list_name_decl(listnamedecl_) {
            Err(e) => { 
                self.faulty_errors.push(e);
                return None;
            },
            Ok(l) => l,
        };
    
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
        let bind_count = new_env.count() - input.env.count();
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
    
        self.normalize(proc_, ProcVisitInputs{
            par: rho_types::Par::default(),
            env: Rc::new(new_env),
            known_free : input.known_free.clone(),
        }).and_then( |body| {
            let mut type_new = rho_types::New::new();
            type_new.set_bindCount(bind_count as i32);
            type_new.set_p(body.par);
            type_new.set_uri(protobuf::RepeatedField::from_vec(uris.into_iter().collect()));
            // TODO: type_new.set_injections(env);
    
            // bodyResult.par.locallyFree.from(newCount).map(x => x - newCount)
            // TODO: type_new.set_locallyFree(v: ::bytes::Bytes);
    
            // TODO:
            // def prepend(n: New): Par =
            // p.copy(
            //   news = n +: p.news,
            //   locallyFree = p.locallyFree | n.locallyFree,
            //   connectiveUsed = p.connectiveUsed || n.connectiveUsed
            // )
            Some(ProcVisitOutputs{ 
                par : rho_types::Par::default(), // TODO : input.par.prepend(type_new),
                known_free : body.known_free 
            })
        })
    }


    fn list_name_decl(&mut self, mut listnamedecl : bnfc::ListNameDecl) -> Result< Vec<(Option<String>, String, SourcePosition)>, CompliationError>
    {
        let mut list : Vec<(Option<String>, String, SourcePosition)> = Vec::new();
        while listnamedecl != 0 as bnfc::ListNameDecl
        {
            let p = unsafe { *listnamedecl };
            match self.extract_name_decl(p.namedecl_) {
                Ok(tuple) => list.push(tuple),
                Err(e) => return Err(e),
            };
            listnamedecl = p.listnamedecl_;
        }
        Ok(list)
    }


    fn extract_name_decl(&mut self, namedecl : bnfc::NameDecl) -> Result< (Option<String>, String, SourcePosition), CompliationError>
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
                        .or_else(|e| Err(CompliationError::SourceUtf8Error(e)) )
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
                            Err(CompliationError::SourceUtf8Error(e))
                        }
                        (_, Err(e)) => {
                            Err(CompliationError::SourceUtf8Error(e))
                        }
                    }
                },
                _ => {
                    Err(CompliationError::UnrecognizedKind(p.kind, "bnfc::NameDecl".to_string()))
                }
            }
        }
        Err(CompliationError::NullPointer("bnfc::NameDecl".to_string()))
    }

    
}





