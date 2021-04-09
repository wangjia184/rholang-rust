use std::sync::Arc;

use async_trait::async_trait;

use super::*;


struct SendEvaluator {
    pub send : Send,
}

impl From<Send> for SendEvaluator {
    fn from(send: Send) -> Self {
        SendEvaluator { send : send }
    }
}

impl From<Send> for ThreadSafeEvaluator {
    fn from(send: Send) -> Self {
        Box::new(SendEvaluator { send : send })
    }
}

#[async_trait]
impl AsyncEvaluator for SendEvaluator {

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
    async fn evaluate(&mut self, reducer : Arc<DebruijnInterpreter>) {
 
        if reducer.is_aborted() {
            return; // abort the execution since error occured
        }

        if self.send.chan.is_none() {
            &reducer.add_error(ExecutionErrorKind::InvalidSend, "Send::chan is None");
            return;
        }

        // charge[M](SEND_EVAL_COST)
        let chan = self.send.chan.take().unwrap();
        let evaluated_chan = &reducer.evaluate_expressions_in_par(chan);

        println!("{:#?}", &self.send);
    }
}