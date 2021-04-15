use super::*;


#[async_trait]
impl AsyncEvaluator for Receive {
    async fn evaluate(&mut self, reducer : Arc<DebruijnInterpreter>, env : Env) {
        if reducer.is_aborted() {
            return; // abort the execution since error occured
        }
    }
}