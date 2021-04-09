use super::*;

impl Substitute for Par {
    fn substitute(self, reducer : &DebruijnInterpreter, depth : i32, env : &Env) -> Result<Self, ExecutionError> {
        Ok(self)
    }
    fn substitute_no_sort(self, reducer : &DebruijnInterpreter, depth : i32, env : &Env) -> Result<Self, ExecutionError> {
        Ok(self)
    }

    
}

fn substitute_expressions(mut vector : Vec<Expr>, depth : i32, env : &Env) -> Result<Par, ExecutionError> {
    // TODO: avoid extra allocation by replacing fold()
    vector.reverse();
    vector.into_iter().fold( Ok(Par::default()), |result, expression| {
        result.and_then( |par| {
            match expression.expr_instance {
                Some(ExprInstance::EVarBody(EVar{ v : Some(var) })) => {
                    unimplemented!("Some(ExprInstance::EVarBody(EVar))");
                },
                Some(ExprInstance::EVarBody(EVar{ v : None})) => {
                    Err(ExecutionError::new(ExecutionErrorKind::InvalidExpression,
                        "Expr::expr_instance::EVarBody::Var is None")
                    )
                },
                None => {
                    Err(ExecutionError::new(ExecutionErrorKind::InvalidExpression,
                        "Expr::expr_instance is None")
                    )
                }
                _ => {
                    unimplemented!("_");
                }
            }
        })
    })
}