use super::*;

impl Substitutable for Match {

    fn substitute(self, reducer : &DebruijnInterpreter, depth : i32, env : &Env) -> Result<Self, ExecutionError> {
       
        Ok(self)
    }


    fn substitute_no_sort(self, reducer : &DebruijnInterpreter, depth : i32, env : &Env) -> Result<Self, ExecutionError> {

        Ok(self)
    }
    
}
