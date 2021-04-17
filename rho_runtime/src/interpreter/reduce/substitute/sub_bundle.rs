use super::*;

impl<S : Storage + std::marker::Send + std::marker::Sync> Substitutable<S> for Bundle {

    fn substitute(&mut self, context : &InterpreterContext<S>, depth : i32, env : &Env) -> Result<(), ExecutionError> {
       
        // substitutePar[M].substitute(term.body).map { subBundle =>
        //     subBundle.singleBundle() match {
        //       case Some(value) => term.merge(value)
        //       case None        => term.copy(body = subBundle)
        //     }
        //   }
        unimplemented!("Bundle::substitute")
        //Ok(self)
    }


    fn substitute_no_sort(&mut self, context : &InterpreterContext<S>, depth : i32, env : &Env) -> Result<(), ExecutionError> {


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
