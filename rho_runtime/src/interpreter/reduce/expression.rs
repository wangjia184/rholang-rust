use std::sync::Arc;
use std::mem;
use async_trait::async_trait;

use super::*;
use model::expr::ExprInstance;

struct ExprEvaluator {
    pub exp : Expr,
}

impl From<Expr> for ExprEvaluator {
    fn from(e: Expr) -> Self {
        ExprEvaluator { exp : e }
    }
}

impl From<Expr> for ThreadSafeEvaluator {
    fn from(e: Expr) -> Self {
        Box::new(ExprEvaluator { exp : e })
    }
}

#[async_trait]
impl AsyncEvaluator for ExprEvaluator {
    async fn evaluate(&self, _reducer : Arc<DebruijnInterpreter>) {
 
        
        
    }
}


impl DebruijnInterpreter {

    pub fn evaluate_expressions(&self, mut par : Par) -> Result<Par, ExecutionError> {
        let expressions = mem::replace( &mut par.exprs, Vec::new());
        let mut evaluated_exprs = vec![];
        for e in expressions.into_iter() {
            evaluated_exprs.push( self.evaluate_expression_to_par(e)? );
        } 
 
        // Note: the locallyFree cache in par could now be invalid, but given
        // that locallyFree is for use in the matcher, and the matcher uses
        // substitution, it will resolve in that case. AlwaysEqual makes sure
        // that this isn't an issue in the rest of cases.
        // evaledExprs.foldLeft(par.copy(exprs = Vector()))(_ ++ _)
        evaluated_exprs.into_iter().fold(Ok(par), |result, e| {
            Ok(result?)
        })
    }

    pub fn evaluate_expression_to_par(&self, exp : Expr) -> Result<Par, ExecutionError> {
        self.raise_error_if_aborted()?;

        match &exp.expr_instance {
            Some(ExprInstance::EVarBody(EVar{ v : Some(var) })) => {
                
                unimplemented!("Some(expr::ExprInstance::EVarBody(EVar))");
            },
            Some(ExprInstance::EVarBody(EVar{ v : None })) => {
                return Err(self.add_error(ExecutionErrorKind::InvalidExpression, "Expr::expr_instance::EVarBody::Var is None"));
            },
            Some(ExprInstance::EMethodBody(EMethod {
                method_name,
                target : Some(target),
                arguments,
                ..
            })) => {
                unimplemented!("Some(ExprInstance::EMethodBody(EMethod))");
            },
            Some(ExprInstance::EMethodBody(EMethod { target : None, ..})) => {
                return Err(self.add_error(ExecutionErrorKind::InvalidExpression, "Expr::expr_instance::EMethodBody::EMethod::target is None"));
            },
            None => {
                return Err(self.add_error(ExecutionErrorKind::InvalidExpression, "Expr::expr_instance is None"));
            },
            _ => {
                unimplemented!("Some(expr::ExprInstance::EVarBody(EVar))");
            }
        }
    }

    pub fn evaluate_expression_to_expression(&self, exp : Expr) -> Result<Expr, ExecutionError> {
        self.raise_error_if_aborted()?;

        Ok(exp)
    }
}