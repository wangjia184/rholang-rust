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
impl Evaluator for SendEvaluator {
    async fn evaluate(&self, _reducer : Arc<DebruijnInterpreter>) {
 
        println!("{:#?}", &self.send);
    }
}