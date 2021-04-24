

use super::super::bnfc;
use super::*;


impl super::Normalizer {

    pub fn normalize_binary_expression<F>(&mut self,
         proc_1 : bnfc::Proc,
         proc_2 : bnfc::Proc,
         input: &ProcVisitInputs,
         func : F)
          -> Result<ProcVisitOutputs, CompiliationError> 
          where F : Fn(Par, Par) -> Expr
    {

        if proc_1 == 0 as bnfc::Proc {
            return Err(CompiliationError::new_null_pointer("proc_1"));
        }
        if proc_2 == 0 as bnfc::Proc {
            return Err(CompiliationError::new_null_pointer("proc_2"));
        }

        let left_input = ProcVisitInputs {
            par : Par::default(),
            env : input.env.clone(),
            known_free : input.known_free.clone(),
        };
        let left_output = self.normalize_proc(proc_1, &left_input)?;

        let right_input = ProcVisitInputs {
            par : Par::default(),
            env : input.env.clone(),
            known_free : Rc::new(left_output.known_free),
        };
        let right_output = self.normalize_proc(proc_2, &right_input)?;

        let expression = func(left_output.par, right_output.par);

        let par = input.par.clone_then_prepend_expr(expression, input.env.depth());
        Ok(
            ProcVisitOutputs {
                par : par, 
                known_free : (*input.known_free).clone() 
            }
        )
    }

    
}