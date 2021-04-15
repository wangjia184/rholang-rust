
use super::*;
use model::expr::ExprInstance;

mod expr_instance;
mod eval_eplus;
#[cfg(test)] mod eval_eplus_test;

#[async_trait]
pub trait AsyncExprInstanceEvaluator {
    async fn evaluate(&mut self, context : &Arc<InterpreterContext>, env : &Env) -> Result<ExprInstance, ExecutionError> ;

}

// AsyncExpressionEvaluator is implemented by Par
#[async_trait]
pub trait AsyncParExpressionEvaluator {
    async fn evaluate_nested_expressions(&mut self, context : &Arc<InterpreterContext>, env : &Env) -> Result<(), ExecutionError>;

    async fn evaluate_single_expression(&mut self, context : &Arc<InterpreterContext>, env : &Env) -> Result<Expr, ExecutionError>;
}

#[async_trait]
impl AsyncParExpressionEvaluator for Par {
    /**
    * evalExpr Evaluate any top level expressions in @param Par .
    */
    async fn evaluate_nested_expressions(&mut self, context : &Arc<InterpreterContext>, env : &Env) -> Result<(), ExecutionError> {

        //for {
        //      evaledExprs <- par.exprs.toList.traverse(evalExprToPar)
        //      result = evaledExprs.foldLeft(par.copy(exprs = Vector()))(_ ++ _)
        //} yield result

        // Note: the locallyFree cache in par could now be invalid, but given
        // that locallyFree is for use in the matcher, and the matcher uses
        // substitution, it will resolve in that case. AlwaysEqual makes sure
        // that this isn't an issue in the rest of cases.

        for expression in &mut self.exprs {

            match &mut expression.expr_instance {
                Some(ExprInstance::EVarBody(_)) => {
                    // evalExprToPar() is not implemented
                    unimplemented!("ExprInstance::EVarBody(_) in evaluate_nested_expressions()");
                },
                Some(ExprInstance::EMethodBody(_)) => {
                    // evalExprToPar() is not implemented
                    unimplemented!("ExprInstance::EMethodBody(_) in evaluate_nested_expressions()");
                },
                Some(_) => {
                    // in-place evaluate
                    expression.evaluate(context, env).await?;

                    if let Some(ref instance) = expression.expr_instance {
                        // merge locally free
                        if let Some(bitset) = ExprInstanceLocallyFree::locally_free(instance, 0) {
                            if let Some( ref mut locally_free ) = self.locally_free {
                                locally_free.union_with(&bitset)
                            } else {
                                self.locally_free = Some(bitset);
                            }
                        }

                        if ExprInstanceLocallyFree::connective_used(instance) {
                            self.connective_used = true;
                        }
                    }
                },
                None => {
                    return Err((ExecutionErrorKind::InvalidExpression, "`expression.expr_instance` is None in evaluate_nested_expressions() ").into());
                }
            }

            
        }

        Ok(())
    }


    async fn evaluate_single_expression(&mut self, context : &Arc<InterpreterContext>, env : &Env) -> Result<Expr, ExecutionError>
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

/*
#[async_trait]
pub trait AsyncExpressionToParEvaluator {
    async fn evaluate_to_par(self, context : &Arc<InterpreterContext>, env : &Env) -> Result<Par, ExecutionError>;
}

#[async_trait]
impl AsyncExpressionToParEvaluator for Expr {
    // evalExprToPar
    async fn evaluate_to_par(self, context : &Arc<InterpreterContext>, env : &Env) -> Result<Par, ExecutionError> {

        // expr.exprInstance match {
        //     case EVarBody(EVar(v)) =>
        //       for {
        //         p       <- eval(v)
        //         evaledP <- evalExpr(p)
        //       } yield evaledP
        //     case EMethodBody(EMethod(method, target, arguments, _, _)) => {
        //       for {
        //         _            <- charge[M](METHOD_CALL_COST)
        //         evaledTarget <- evalExpr(target)
        //         evaledArgs   <- arguments.toList.traverse(evalExpr)
        //         resultPar <- methodTable.get(method) match {
        //                       case None =>
        //                         ReduceError("Unimplemented method: " + method).raiseError[M, Par]
        //                       case Some(f) => f(evaledTarget, evaledArgs)
        //                     }
        //       } yield resultPar
        //     }
        //     case _ => evalExprToExpr(expr).map(fromExpr(_)(identity))

        match &self.expr_instance {
            Some(ExprInstance::EVarBody(EVar{ v : Some(var) })) => {
                
                todo!("Some(expr::ExprInstance::EVarBody(EVar))");
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
                todo!("Some(ExprInstance::EMethodBody(EMethod))");
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
                self.evaluate(context, env).await.and_then( |e| {
                    let mut par = Par::default();
                    par.exprs.push(e);
                    Ok(par)
                })
            }
        }
    }
}
*/

