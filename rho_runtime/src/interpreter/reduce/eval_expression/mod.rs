use std::sync::Arc;
use std::mem;
use async_trait::async_trait;

use super::*;
use model::expr::ExprInstance;

mod plus;
#[cfg(test)]
mod plus_test;

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
    async fn evaluate(&mut self, _reducer : Arc<DebruijnInterpreter>, env : Env) {
 
        
        
    }
}


impl DebruijnInterpreter {

    pub fn evaluate_expressions_in_par(&self, mut par : Par) -> Result<Par, ExecutionError> {
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
        evaluated_exprs.into_iter().fold(Ok(par), |result, mut exp| {
            result.and_then( |mut acc| {
                exp.append_mut(&mut acc);
                Ok(exp)
            })
            
        })
    }

    fn evaluate_expression_to_par(&self, exp : Expr) -> Result<Par, ExecutionError> {
        self.raise_error_if_aborted()?;

        match &exp.expr_instance {
            Some(ExprInstance::EVarBody(EVar{ v : Some(var) })) => {
                
                unimplemented!("Some(expr::ExprInstance::EVarBody(EVar))");
            },
            Some(ExprInstance::EVarBody(EVar{ v : None })) => {
                return Err(ExecutionError::new(ExecutionErrorKind::InvalidExpression,
                     "Expr::expr_instance::EVarBody::Var is None")
                );
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
                return Err(ExecutionError::new(ExecutionErrorKind::InvalidExpression, 
                    "Expr::expr_instance::EMethodBody::EMethod::target is None")
                );
            },
            None => {
                return Err(ExecutionError::new(ExecutionErrorKind::InvalidExpression, 
                    "Expr::expr_instance is None")
                );
            },
            _ => {
                self.evaluate_expression_to_expression(exp).and_then( |e| {
                    let mut par = Par::default();
                    par.exprs.push(e);
                    Ok(par)
                })
            }
        }
    }

    fn evaluate_expression_to_expression(&self, exp : Expr) -> Result<Expr, ExecutionError> {
        self.raise_error_if_aborted()?;

        match exp.expr_instance {
            Some(ExprInstance::GBool(_)) => Ok(exp),
            Some(ExprInstance::GInt(_)) => Ok(exp),
            Some(ExprInstance::GString(_)) => Ok(exp),
            Some(ExprInstance::GUri(_)) => Ok(exp),
            Some(ExprInstance::GByteArray(_)) => Ok(exp),
            Some(ExprInstance::EPlusBody(EPlus{ p1 : Some(p1), p2 : Some(p2) })) => {
                self.evaluate_plus(p1, p2)
            },
            Some(ExprInstance::EPlusBody(EPlus{ p1 : None, .. })) => {
                Err(ExecutionError::new( ExecutionErrorKind::InvalidExpression,
                    "EPlus::p1 is None")
                )
            },
            Some(ExprInstance::EPlusBody(EPlus{ p2 : None, .. })) => {
                Err(ExecutionError::new( ExecutionErrorKind::InvalidExpression,
                    "EPlus::p2 is None")
                )
            },

            
            None => {
                Err(ExecutionError::new(ExecutionErrorKind::InvalidExpression,
                    "Expr::expr_instance is None")
                )
            },
            _ => {
                panic!("evaluate_expression_to_expression() : {:?}", &exp.expr_instance)
            }
        }
    }

    fn evaluate_single_expression(&self, mut par : Par) -> Result<Expr, ExecutionError> {
        if !par.sends.is_empty() || !par.receives.is_empty() || !par.news.is_empty() || 
           !par.matches.is_empty() || !par.unforgeables.is_empty() || !par.bundles.is_empty() {
            return Err(ExecutionError::new( ExecutionErrorKind::InvalidExpression,
                "Parallel or non expression found where expression expected.")
            );
        }
        if par.exprs.len() != 1 {
            return Err(ExecutionError::new( ExecutionErrorKind::InvalidExpression,
                "Single expression is expected.")
            );
        }
        self.evaluate_expression_to_expression(par.exprs.remove(0))
    }

}
