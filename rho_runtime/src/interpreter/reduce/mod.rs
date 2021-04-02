use std::sync::Arc;

use async_trait::async_trait;

use tokio::task;

use model::*;

mod send;
mod receive;
mod expression;

#[async_trait]
trait AsyncEvaluator {
    async fn evaluate(&self, reducer : Arc<DebruijnInterpreter>);
}

type ThreadSafeEvaluator = Box<dyn AsyncEvaluator + std::marker::Send + std::marker::Sync>;


pub struct DebruijnInterpreter {

}


impl DebruijnInterpreter {

    pub async fn evaluate(self : Arc<Self>, par : Par) {

        let evaluators : Vec<ThreadSafeEvaluator>;
        evaluators = par.sends.into_iter().map( |s| s.into() )
            .chain( par.receives.into_iter().map( |r| r.into() ) )
            .collect();


        let mut handles = Vec::new();

        for (_idx, evaluator) in evaluators.into_iter().enumerate() {
            let cloned_self = self.clone();
            handles.push(
                task::spawn( async move {
                    evaluator.evaluate(cloned_self).await;
                })
            );
        }

        for handle in handles {
            handle.await.expect("Panic in Evaluator");
        }

    }


}