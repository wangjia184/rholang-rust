use bit_set::BitSet;


use super::super::bnfc;
use super::super::errors::*;
use super::*;

impl super::Normalizer {

    pub fn normalize_send(&mut self, proc : &RawProc,  input: &ProcVisitInputs) -> Option<ProcVisitOutputs> {
        // normalize the name
        let name_ = unsafe { proc.u.psend_.name_ };
        let send_ = unsafe { proc.u.psend_.send_ };
        let listproc_ = unsafe { proc.u.psend_.listproc_ };

        if name_ == 0 as bnfc::Name {
            self.faulty_errors.push(CompliationError::NullPointer("psend_.name_".to_string()));
            return None;
        }
        if send_ == 0 as bnfc::Send {
            self.faulty_errors.push(CompliationError::NullPointer("psend_.send_".to_string()));
            return None;
        }
        if listproc_ == 0 as bnfc::ListProc {
            self.faulty_errors.push(CompliationError::NullPointer("psend_.listproc_".to_string()));
            return None;
        }
        let n = unsafe { *name_ };
        let name_visit_inputs = NameVisitInputs { env : input.env.clone(), known_free : input.known_free.clone() };
        let name_visit_outputs = match self.normalize_name(&n, &name_visit_inputs) {
            None => return None,
            Some(o) => o,
        };

        // send type
        let kind = unsafe { (*send_).kind };
        let persistent_send = match kind {
            bnfc::Send__is_SendSingle => false,
            bnfc::Send__is_SendMultiple => true,
            _ => {
                self.faulty_errors.push(CompliationError::UnrecognizedKind(kind, "psend_.send_.kind".to_string()));
                return None;
            }
        };

        let mut list = self.list_proc(listproc_);
        list.reverse();

        let init_acc = Some((
            Vec::<RhoPar>::new(),
            ProcVisitInputs { par : RhoPar::default(), env : input.env.clone(), known_free : input.known_free.clone() },
            BitSet::<u64>::default(), // locally_free
            false // is_connective_used
        ));

        list.into_iter().fold( init_acc, | option, proc_| {
            option.and_then( | (mut list, proc_visit_inputs, mut locally_free, connective_used) | {
                self.normalize(proc_, proc_visit_inputs).map( |result| {
                    let connective_used = connective_used || result.par.is_connective_used();
                    locally_free.union_with(&result.par.locally_free); // | procMatchResult.par.locallyFree,
                    list.insert( 0, result.par);
                    (
                        list,
                        ProcVisitInputs { par : RhoPar::default(), env : input.env.clone(), known_free : Rc::new(result.known_free) },
                        locally_free, 
                        connective_used,
                    )
                })
            })
            
        });

        None
    }


    pub fn list_proc(&mut self, mut listproc_ : bnfc::ListProc) -> Vec<bnfc::Proc>
    {
        let mut list : Vec<bnfc::Proc>= Vec::new();
        while listproc_ != 0 as bnfc::ListProc
        {
            let proc_ = unsafe { (*listproc_).proc_ };
            if listproc_ == 0 as bnfc::ListProc {
                self.faulty_errors.push(CompliationError::NullPointer("psend_.listproc_.proc_".to_string()));
            } else {
                list.push(proc_);
            }

            listproc_ = unsafe { (*listproc_).listproc_ };
        }
        list
    }
    
}