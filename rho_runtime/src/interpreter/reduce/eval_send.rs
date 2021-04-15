

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
    async fn evaluate(&mut self, reducer : Arc<DebruijnInterpreter>, env : Env) {
 
        if reducer.is_aborted() {
            return; // abort the execution since error occured
        }

        if self.chan.is_none() {
            &reducer.add_error(ExecutionErrorKind::InvalidSend, "Send::chan is None");
            return;
        }

        // charge[M](SEND_EVAL_COST)

        if let Err(err) = evaluate_send(self, &reducer, env) {
            &reducer.push_error(err);
            return;
        }
    }
}

fn evaluate_send(send: &mut Send, reducer : &Arc<DebruijnInterpreter>, env : Env) -> Result<(), ExecutionError>{
 

    // charge[M](SEND_EVAL_COST)

    let chan = send.chan.take().unwrap();
    let mut evaluated_chan = reducer.evaluate_expressions_in_par(chan)?;

    evaluated_chan.substitute(&reducer, 0, &env)?;

    println!("{:#?}", &evaluated_chan);

    Ok(())
}