


use super::super::bnfc;
use super::super::errors::*;
use super::*;



impl super::Normalizer {

    pub fn normalize_send(&mut self, proc : &RawProc,  input: &ProcVisitInputs) -> Result<ProcVisitOutputs, CompliationError> {
        // normalize the name
        let name_ = unsafe { proc.u.psend_.name_ };
        let send_ = unsafe { proc.u.psend_.send_ };
        let listproc_ = unsafe { proc.u.psend_.listproc_ };

        if name_ == 0 as bnfc::Name {
            return Err(CompliationError::NullPointer("psend_.name_".to_string()));
        }
        if send_ == 0 as bnfc::Send {
            return Err(CompliationError::NullPointer("psend_.send_".to_string()));
        }
        if listproc_ == 0 as bnfc::ListProc {
            return Err(CompliationError::NullPointer("psend_.listproc_".to_string()));
        }
        let n = unsafe { *name_ };
        let name_visit_inputs = NameVisitInputs { env : input.env.clone(), known_free : input.known_free.clone() };
        let name_visit_outputs = self.normalize_name(&n, &name_visit_inputs)?;

        // send type
        let kind = unsafe { (*send_).kind };
        let persistent_send = match kind {
            bnfc::Send__is_SendSingle => false,
            bnfc::Send__is_SendMultiple => true,
            _ => {
                return Err(CompliationError::UnrecognizedKind(kind, "psend_.send_.kind".to_string()));
            }
        };

        let mut list = self.list_proc(listproc_)?;
        list.reverse();

        let init_acc : Result<_, CompliationError> = Ok((
            prost::alloc::vec::Vec::<Par>::new(),
            ProcVisitInputs { par : Par::default(), env : input.env.clone(), known_free : input.known_free.clone() },
            BitSet::default(), // locally_free
            false // is_connective_used
        ));

        let (data, proc_visit_inputs, mut locally_free, mut connective_used) = list.into_iter().fold( init_acc, | option, proc_| {
            option.and_then( | (mut list, proc_visit_inputs, mut locally_free, connective_used) | {
                self.normalize_proc(proc_, &proc_visit_inputs).map( |result| {
                    let connective_used = connective_used || result.par.connective_used;
                    // the nested
                    locally_free.union_with_option(result.par.locally_free.as_ref()); 
                    list.insert( 0, result.par);
                    (
                        list,
                        ProcVisitInputs { par : Par::default(), env : input.env.clone(), known_free : Rc::new(result.known_free) },
                        locally_free, 
                        connective_used,
                    )
                })
            })
        })?;

        locally_free.union_with_option(ParLocallyFree::locally_free( &name_visit_outputs.chan, input.env.depth()).as_ref());
        if ParLocallyFree::connective_used(&name_visit_outputs.chan) {
            connective_used = true;
        }

        let rho_send = RhoSend {
            chan : Some(name_visit_outputs.chan),
            data : data,
            persistent : persistent_send,
            locally_free : Some(locally_free),
            connective_used : connective_used,
        };
        

        let par = input.par.clone_then_prepend_send(rho_send);
        Ok( ProcVisitOutputs { par : par, known_free : (*proc_visit_inputs.known_free).clone() } )
    }


    pub fn list_proc(&mut self, mut listproc_ : bnfc::ListProc) -> Result< Vec<bnfc::Proc>, CompliationError>
    {
        let mut list : Vec<bnfc::Proc>= Vec::new();
        while listproc_ != 0 as bnfc::ListProc
        {
            let proc_ = unsafe { (*listproc_).proc_ };
            if listproc_ == 0 as bnfc::ListProc {
                return Err(CompliationError::NullPointer("psend_.listproc_.proc_".to_string()));
            } else {
                list.push(proc_);
            }

            listproc_ = unsafe { (*listproc_).listproc_ };
        }
        Ok(list)
    }
    
}