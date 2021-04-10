use super::*;

impl Substitutable for Bundle {

    fn substitute(&mut self, reducer : &DebruijnInterpreter, depth : i32, env : &Env) -> Result<(), ExecutionError> {
       
        // substitutePar[M].substitute(term.body).map { subBundle =>
        //     subBundle.singleBundle() match {
        //       case Some(value) => term.merge(value)
        //       case None        => term.copy(body = subBundle)
        //     }
        //   }
        unimplemented!("Bundle::substitute")
        //Ok(self)
    }


    fn substitute_no_sort(&mut self, reducer : &DebruijnInterpreter, depth : i32, env : &Env) -> Result<(), ExecutionError> {


        // substitutePar[M].substituteNoSort(term.body).map { subBundle =>
        //     subBundle.singleBundle() match {
        //       case Some(value) => term.merge(value)
        //       case None        => term.copy(body = subBundle)
        //     }
        //   }

        unimplemented!("Bundle::substitute_no_sort")

        //Ok(self)
    }
    
}
