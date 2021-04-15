

use super::*;



#[async_trait]
impl AsyncEvaluator for Send {

   /** Algorithm as follows:
    *
    * 1. Fully evaluate the channel in given environment.
    * 2. Substitute any variable references in the channel so that it can be
    *    correctly used as a key in the tuple space.
    * 3. Evaluate any top level expressions in the data being sent.
    * 4. Call produce
    * 5. If produce returned a continuation, evaluate it.
    *
    * @param send An output process
    * @param env An execution context
    *
    */
    async fn evaluate(&mut self, context : &Arc<InterpreterContext>, env : &Env) -> Result<(), ExecutionError> {
 
        context.may_raise_aborted_error()?;

        if self.chan.is_none() {
            return Err((ExecutionErrorKind::InvalidSend, "Send::chan is None").into());
        }

        // charge[M](SEND_EVAL_COST)

        // evalChan <- evalExpr(send.chan)
        if let Some(ref mut chan) = self.chan {
            chan.evaluate_nested_expressions(context, env).await?;

            // subChan  <- substituteAndCharge[Par, M](evalChan, 0, env)
            chan.substitute(&context, 0, &env)?;
        }

        

        println!("{:#?}", &self.chan);

        Ok(())
    }
}

