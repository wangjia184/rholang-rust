use std::sync::Arc;

use async_trait::async_trait;

use super::*;


struct ReceiveEvaluator {
    pub receive : Receive,
}

impl From<Receive> for ReceiveEvaluator {
    fn from(receive: Receive) -> Self {
        ReceiveEvaluator { receive : receive }
    }
}

impl From<Receive> for ThreadSafeEvaluator {
    fn from(receive: Receive) -> Self {
        Box::new(ReceiveEvaluator { receive : receive })
    }
}

#[async_trait]
impl Evaluator for ReceiveEvaluator {
    async fn evaluate(&self, _reducer : Arc<DebruijnInterpreter>) {
 
    }
}