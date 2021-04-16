

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

        let chan = match self.chan {
            Some(ref mut c) => c,
            None => return Err((ExecutionErrorKind::InvalidSend, "Send::chan is missing").into()),
        };


        // evalChan <- evalExpr(send.chan)
        chan.evaluate_nested_expressions(context, env).await?;

        // subChan  <- substituteAndCharge[Par, M](evalChan, 0, env)
        chan.substitute(&context, 0, &env)?;

        let unbunded = match chan.single_bundle() {
            Some(b) => {
                if !b.write_flag {
                    return Err((ExecutionErrorKind::NonWritableChannel, "Trying to send on non-writeable channel.").into());
                }
                if let Some(ref p) = b.body {
                    p
                } else {
                    return Err((ExecutionErrorKind::InvalidBundle, "Bundle::body is missing").into());
                }
            },
            None => chan,
        };

        for dataum in &mut self.data {
            dataum.evaluate_nested_expressions(context, env).await?;
            // substituteAndCharge
            dataum.substitute(context, 0, env)?;
        }
    

        

        println!("{:#?}", &self);

        Ok(())
    }
}

