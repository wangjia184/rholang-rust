
use super::*;
use model::expr::ExprInstance;
use smallvec::SmallVec;

mod expr_instance;
mod eval_eplus;
#[cfg(test)] mod eval_eplus_test;

#[async_trait]
pub trait AsyncExprInstanceEvaluator<S> where  S : Storage + std::marker::Send + std::marker::Sync {
    async fn evaluate(&mut self, context : &Arc<InterpreterContext<S>>, env : &Env) -> Result<ExprInstance, ExecutionError> ;

}

// AsyncExpressionEvaluator is implemented by Par
#[async_trait]
pub trait AsyncParExpressionEvaluator<S> where  S : Storage + std::marker::Send + std::marker::Sync {
    async fn evaluate_nested_expressions(&mut self, context : &Arc<InterpreterContext<S>>, env : &Env) -> Result<(), ExecutionError>;

    async fn evaluate_single_expression(&mut self, context : &Arc<InterpreterContext<S>>, env : &Env) -> Result<Expr, ExecutionError>;
}

#[async_trait]
impl<S : Storage + std::marker::Send + std::marker::Sync> AsyncParExpressionEvaluator<S> for Par {
   /**
    * evalExpr Evaluate any top level expressions in @param Par .
    */
    async fn evaluate_nested_expressions(&mut self, context : &Arc<InterpreterContext<S>>, env : &Env) -> Result<(), ExecutionError> {

        //for {
        //      evaledExprs <- par.exprs.toList.traverse(evalExprToPar)
        //      result = evaledExprs.foldLeft(par.copy(exprs = Vector()))(_ ++ _)
        //} yield result

        // Note: the locallyFree cache in par could now be invalid, but given
        // that locallyFree is for use in the matcher, and the matcher uses
        // substitution, it will resolve in that case. AlwaysEqual makes sure
        // that this isn't an issue in the rest of cases.

        // avoid std::mem::replace to reduce heap allocation
        let mut expressions = SmallVec::<[Expr; 5]>::with_capacity(self.exprs.len());
        expressions.extend(self.exprs.drain(..));

        for mut expression in expressions {

            match expression.expr_instance {
                Some(ExprInstance::EVarBody(mut evar)) => {
                    let mut par = match evar.v {
                        Some(ref mut var) => var.evaluate(context, env).await?,
                        None => return Err((ExecutionErrorKind::InvalidExpression, "`expression.expr_instance` is None in evaluate_nested_expressions() ").into()),
                    };
                    par.evaluate_nested_expressions(context, env).await?;
                    self.append_mut(&mut par);
                },
                Some(ExprInstance::EMethodBody(_)) => {
                    // evalExprToPar() is not implemented
                    unimplemented!("ExprInstance::EMethodBody(_) in evaluate_nested_expressions()");
                },
                Some(_) => {
                    // in-place evaluate
                    expression.evaluate(context, env).await?;
                    self.append_expr(expression, 0);
                },
                None => {
                    return Err((ExecutionErrorKind::InvalidExpression, "`expression.expr_instance` is None in evaluate_nested_expressions() ").into());
                }
            }

            
        }

        Ok(())
    }


    async fn evaluate_single_expression(&mut self, context : &Arc<InterpreterContext<S>>, env : &Env) -> Result<Expr, ExecutionError>
    {
        if !self.sends.is_empty() || !self.receives.is_empty() || !self.news.is_empty() || 
           !self.matches.is_empty() || !self.unforgeables.is_empty() || !self.bundles.is_empty() {
            return Err((ExecutionErrorKind::InvalidExpression, "Parallel or non expression found where expression expected.").into());
        }
        
        if let Some(mut expression) = self.exprs.pop() {
            expression.evaluate(context, env).await?;
            Ok(expression)
        } else {
            Err((ExecutionErrorKind::InvalidExpression, "Single expression is expected.").into())
        }
    }
}


