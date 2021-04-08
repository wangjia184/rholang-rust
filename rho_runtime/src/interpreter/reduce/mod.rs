use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use crossbeam::queue::SegQueue;

use async_trait::async_trait;

use tokio::task;

use model::*;

mod substitute;
mod send;
mod receive;
mod expression;

#[async_trait]
trait AsyncEvaluator {
    async fn evaluate(&self, reducer : Arc<DebruijnInterpreter>);
}

type ThreadSafeEvaluator = Box<dyn AsyncEvaluator + std::marker::Send + std::marker::Sync>;

#[derive(Default)]
pub struct DebruijnInterpreter {

    aborted : AtomicBool,
    errors : SegQueue<ExecutionError>,
}

pub struct Env {
    
}

impl DebruijnInterpreter {

    pub fn add_error<S>(&self, e : ExecutionErrorKind, msg : S) -> ExecutionError 
        where S : Into<String> 
    {
        self.aborted.store(true, Ordering::Relaxed);
        let err = ExecutionError{
            kind : e as i32,
            message : msg.into()
        };
        self.errors.push(err.clone());
        err
    }

    pub fn is_aborted(&self) -> bool {
        self.aborted.load(Ordering::Relaxed)
    }

    pub fn raise_error_if_aborted(&self) -> Result<(), ExecutionError> {
        if self.is_aborted() {
            Err(ExecutionError{
                kind : ExecutionErrorKind::Aborted as i32,
                message : "aborted".to_string(),
            })
        } else {
            Ok(())
        }
    }

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