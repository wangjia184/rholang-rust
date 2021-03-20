

use super::super::errors::*;
use super::*;


impl super::Normalizer {

    pub fn normalize_par(&mut self, proc : &RawProc, input: &ProcVisitInputs) -> Result<ProcVisitOutputs, CompliationError> {

        let proc_1 = unsafe { proc.u.ppar_.proc_1 };
        let proc_2 = unsafe { proc.u.ppar_.proc_2 };
        
        let proc_visit_outputs = self.normalize_proc(proc_1, input)?;

        let input2 = ProcVisitInputs {
            par : proc_visit_outputs.par,
            env : input.env.clone(),
            known_free : Rc::new(proc_visit_outputs.known_free),
        };

        self.normalize_proc(proc_2, &input2)
    }

}