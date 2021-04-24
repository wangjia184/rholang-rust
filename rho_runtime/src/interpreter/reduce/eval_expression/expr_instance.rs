use super::*;


#[async_trait]
impl<S : Storage + std::marker::Send + std::marker::Sync> AsyncEvaluator<S> for Expr {
    async fn evaluate(&mut self, context : &Arc<InterpreterContext<S>>, env : &Env) -> Result<(), ExecutionError> {
  
        match &mut self.expr_instance {
            Some(ExprInstance::GBool(_)) => Ok(()),
            Some(ExprInstance::GInt(_)) => Ok(()),
            Some(ExprInstance::GString(_)) => Ok(()),
            Some(ExprInstance::GUri(_)) => Ok(()),
            Some(ExprInstance::GByteArray(_)) => Ok(()),
            Some(ExprInstance::EPlusBody(eplus)) => {
                self.expr_instance = Some(eplus.evaluate(context, env).await?);
                Ok(())
            },

            Some(ExprInstance::EVarBody(evar)) => {
                if let Some(ref mut var) = evar.v {
                    let mut par = var.evaluate(context, env).await?;
                    let expression = par.evaluate_single_expression(context, env).await?;
                    self.expr_instance = expression.expr_instance;
                }
                Ok(())
            },

            _ => {
                panic!("Unhandled branch in ExprInstance::evaluate() : {:?}", self.expr_instance)
            }
        }
    }
}


