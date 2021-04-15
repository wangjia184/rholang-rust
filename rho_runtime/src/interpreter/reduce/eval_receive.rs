use super::*;


#[async_trait]
impl AsyncEvaluator for Receive {
    async fn evaluate(&mut self, context : &Arc<InterpreterContext>, env : &Env) -> Result<(), ExecutionError> {
        context.may_raise_aborted_error()?;
        
        Ok(())
    }
}