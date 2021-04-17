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

            _ => {
                panic!("ExprInstance::evaluate() : {:?}", &self)
            }
        }
    }
}


