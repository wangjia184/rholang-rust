


use crate::model::rho_types;
use super::super::bnfc;
use super::super::errors::*;
use super::*;

impl super::Normalizer {

    pub fn normalize_send(&mut self, proc : &RhoProc,  input: &ProcVisitInputs) -> Option<ProcVisitOutputs> {
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
        self.normalize_name(&n, &NameVisitInputs { env : input.env.clone(), known_free : input.known_free.clone() });

        None
    }
    
}