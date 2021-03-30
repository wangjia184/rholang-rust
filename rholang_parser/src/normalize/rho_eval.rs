
use model::*;

use super::*;


impl super::Normalizer {

    pub fn normalize_eval(&mut self, proc : &RawProc, input: &ProcVisitInputs) -> Result<ProcVisitOutputs, CompiliationError> {

        let name_ = unsafe { proc.u.peval_.name_ };
        if name_ == 0 as bnfc::Name {
            return Err(CompiliationError::new_null_pointer("peval_.name_"));
        }
        
        let name_visit_inputs = NameVisitInputs {
            env : input.env.clone(),
            known_free : input.known_free.clone(),
        };
        let name = unsafe { *name_ };
        let mut name_visit_output = self.normalize_name(&name, &name_visit_inputs)?;

        let known_free = name_visit_output.known_free;
        name_visit_output.chan.append(&input.par);

        Ok(ProcVisitOutputs{
            par : name_visit_output.chan,
            known_free : (*known_free).clone(),
        })
    }

}