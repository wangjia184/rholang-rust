

use super::*;




impl<S> Evaluator<S, Par> for Var 
    where S : Storage + std::marker::Send + std::marker::Sync 
{

    /**
    * Variable "evaluation" is an environment lookup, but
    * lookup of an unbound variable should be an error.
    *
    * @param valproc The variable to be evaluated
    * @param env  provides the environment (possibly) containing a binding for the given variable.
    * @return If the variable has a binding (par), lift the
    *                  binding into the monadic context, else signal
    *                  an exception.
    *
    */
    fn evaluate(&mut self, _context : &Arc<InterpreterContext<S>>, env : &Env) -> Result<Par, ExecutionError> {
 
        match self.var_instance {
            Some(var::VarInstance::BoundVar(level)) => {
                match env.get(level) {
                    Some(par) => Ok(par.clone()),
                    None => Err((ExecutionErrorKind::UnboundVariable, format!("Unbound variable: {} in {:?}", level, env)).into())
                }
            },
            Some(var::VarInstance::Wildcard(_)) => {
                Err((ExecutionErrorKind::UnboundVariable, "Unbound variable: attempting to evaluate a pattern").into())
            },
            Some(var::VarInstance::FreeVar(_)) => {
                Err((ExecutionErrorKind::UnboundVariable, "Unbound variable: attempting to evaluate a pattern").into())
            },
            None => {
                Err((ExecutionErrorKind::SystemFailure, "Var::var_instance is impossible to miss").into())
            }
        }

   
    }
}

