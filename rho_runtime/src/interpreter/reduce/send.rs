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
    async fn evaluate(&self, _reducer : Arc<DebruijnInterpreter>) {
 
        println!("{:#?}", &self.send);
    }
}