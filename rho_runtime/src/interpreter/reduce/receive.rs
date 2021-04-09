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
impl AsyncEvaluator for ReceiveEvaluator {
    async fn evaluate(&mut self, reducer : Arc<DebruijnInterpreter>) {
        if reducer.is_aborted() {
            return; // abort the execution since error occured
        }
    }
}