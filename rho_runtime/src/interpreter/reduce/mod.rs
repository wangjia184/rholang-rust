use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use crossbeam::queue::SegQueue;

use async_trait::async_trait;

use tokio::task;

use model::*;

mod substitute;
mod eval_send;
mod eval_receive;
mod eval_expression;
mod environment;

use substitute::*;
pub use environment::*;

#[async_trait]
trait AsyncEvaluator {
    async fn evaluate(&mut self, reducer : Arc<DebruijnInterpreter>, env : Env);
}

type ThreadSafeEvaluator = Box<dyn AsyncEvaluator + std::marker::Send + std::marker::Sync>;

#[derive(Default)]
pub struct DebruijnInterpreter {

    aborted : AtomicBool,
    errors : SegQueue<ExecutionError>,
}


impl DebruijnInterpreter {

    pub fn push_error(&self, err : ExecutionError)
    {
        self.aborted.store(true, Ordering::Relaxed);
        self.errors.push(err);
    }

    pub fn add_error<S>(&self, e : ExecutionErrorKind, msg : S) -> ExecutionError 
        where S : Into<String> 
    {
        let err = ExecutionError{
            kind : e as i32,
            message : msg.into()
        };
        self.push_error(err.clone());
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

    pub async fn evaluate(self : Arc<Self>, par : Par, env : Env) {
        

        let evaluators : Vec<ThreadSafeEvaluator>;
        evaluators = par.sends.into_iter().map( |s| s.into() )
            .chain( par.receives.into_iter().map( |r| r.into() ) )
            .collect();


        let mut handles = Vec::new();

        for (_idx, mut evaluator) in evaluators.into_iter().enumerate() {
            let cloned_self = self.clone();
            let cloned_env = env.clone();
            handles.push(
                task::spawn( async move {
                    evaluator.evaluate(cloned_self, cloned_env).await;
                })
            );
        }

        for handle in handles {
            handle.await.expect("Panic in Evaluator");
        }

    }


}