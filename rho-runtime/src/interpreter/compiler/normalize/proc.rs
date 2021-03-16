

use std::ffi::{ CStr }; 
use std::os::raw::c_char;
use std::cmp::Ordering;
use std::collections::hash_set::HashSet;
use defer::defer;

use crate::model::constant;
use crate::model::rho_types;
use super::super::context::*;
use super::super::bnfc;

use super::super::errors::*;
use super::{ ProcVisitInputs, ProcVisitOutputs};



pub fn normalize_match(p : bnfc::Proc, input: ProcVisitInputs) -> Option<ProcVisitOutputs> {
    if p == 0 as bnfc::Proc {
        return None; // NULL pointer
    }

    defer( || unsafe { libc::free(p as *mut libc::c_void); } );

    let proc = unsafe { *p };

    match proc.kind {
        bnfc::Proc__is_PPar => {
            trace!("PPar at {}:{}", proc.line_number, proc.char_number);
            let proc_1 = unsafe { proc.u.ppar_.proc_1 };
            let proc_2 = unsafe { proc.u.ppar_.proc_2 };
            normalize_match(proc_1, input);
            //normalize_match(proc_2, input);
        },
        bnfc::Proc__is_PNew => {
            let listnamedecl = unsafe { proc.u.pnew_.listnamedecl_ };
            let sub_proc = unsafe { proc.u.pnew_.proc_ };
            return normalize_pnew(listnamedecl, sub_proc, &input);
        },

        
        _ => { warn!("Unknown token {:?}", proc.kind); }
    };
    
    None
}

fn normalize_pnew(listnamedecl : bnfc::ListNameDecl, sub_proc : bnfc::Proc, input: &ProcVisitInputs) -> Option<ProcVisitOutputs> {
    let mut list = match list_name_decl(listnamedecl) {
        Err(e) => { 
            error!("normalize_pnew failed. {}", e); 
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

    let new_env = input.env.clone_with_new_head(new_bindings);
    let bind_count = new_env.count() - input.env.count();
    let requires_deploy_id = uris.contains(constant::DEPLOY_ID_URI);
    let requires_deployer_id = uris.contains(constant::DEPLOYER_ID_URI);

    /* TODO:
    def missingEnvElement(name: String, uri: String) =
        NormalizerError(s"`$uri` was used in rholang usage context where $name is not available.")
    if (requiresDeployId && env.get(deployIdUri).forall(_.singleDeployId().isEmpty))
        missingEnvElement("DeployId", deployIdUri).raiseError[M, ProcVisitOutputs]
    else if (requiresDeployerId && env.get(deployerIdUri).forall(_.singleDeployerId().isEmpty))
        missingEnvElement("DeployerId", deployerIdUri).raiseError[M, ProcVisitOutputs]
    */

    normalize_match(sub_proc, ProcVisitInputs{
        par: rho_types::Par::default(),
        env: new_env,
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


fn list_name_decl(mut listnamedecl : bnfc::ListNameDecl) -> Result< Vec<(Option<String>, String, SourcePosition)>, CompliationError>
{
    let mut list : Vec<(Option<String>, String, SourcePosition)> = Vec::new();
    while listnamedecl != 0 as bnfc::ListNameDecl
    {
        let p = unsafe { *listnamedecl };
        match extract_name_decl(p.namedecl_) {
            Ok(tuple) => list.push(tuple),
            Err(e) => return Err(e),
        };
        listnamedecl = p.listnamedecl_;
    }
    Ok(list)
}


fn extract_name_decl(namedecl : bnfc::NameDecl) -> Result< (Option<String>, String, SourcePosition), CompliationError>
{
    if namedecl != 0 as bnfc::NameDecl {
        let p = unsafe { *namedecl };
        return match p.kind {
            bnfc::NameDecl__is_NameDeclSimpl => {
                let var = unsafe { p.u.namedeclsimpl_.var_ };
                get_string(var)
                    .and_then(|name| {
                        let source_position = SourcePosition::new(p.line_number, p.char_number, name.len());
                        Ok((None, name, source_position))
                    })
                    .or_else(|e| Err(CompliationError::SourceUtf8Error(e)) )
            },
            bnfc::NameDecl__is_NameDeclUrn => {
                let var = unsafe { p.u.namedeclurn_.var_ };
                let uriliteral = unsafe { p.u.namedeclurn_.uriliteral_ };
                match (get_string(var), get_string(uriliteral)) {
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
                Err(CompliationError::UnrecognizedNameDeclKind(p.kind))
            }
        }
    }
    Err(CompliationError::NullPointer("extract_name_decl".to_string()))
}

fn get_string(raw_str : bnfc::String) -> Result<String, std::str::Utf8Error> {
    unsafe {
        let raw_pointer = raw_str as *const _ as *const c_char;
        CStr::from_ptr(raw_pointer).to_str().and_then( |s| {
            Ok(s.to_owned())
        })
    }
}