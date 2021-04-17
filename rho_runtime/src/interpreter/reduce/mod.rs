use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use crossbeam::queue::SegQueue;


use async_trait::async_trait;
use tokio::task;

use model::*;

mod substitute;
pub mod eval_par;
mod eval_send;
mod eval_receive;
mod eval_expression;
mod environment;

#[cfg(test)] mod eval_receive_test;


use eval_expression::AsyncParExpressionEvaluator;
use substitute::*;
pub use environment::*;

use crate::storage::Storage;

#[async_trait]
pub trait AsyncEvaluator<S> where S : Storage + std::marker::Send + std::marker::Sync {
    async fn evaluate(&mut self, context : &Arc<InterpreterContext<S>>, env : &Env) -> Result<(), ExecutionError>;
}



#[derive(Default)]
pub struct InterpreterContext<S> where S : Storage + std::marker::Send + std::marker::Sync {
    storage : S,
    aborted : AtomicBool,
    errors : SegQueue<ExecutionError>,
}





impl<S : Storage + std::marker::Send + std::marker::Sync + 'static> InterpreterContext<S> {

    fn spawn_evaluation<T>(self : &Arc<Self>, t : T, env : &Env) -> tokio::task::JoinHandle<Result<(), ExecutionError>>
        where T : AsyncEvaluator<S> + std::marker::Send + 'static
    {
        let cloned_context = self.clone();
        let cloned_env = env.clone();
        task::spawn( async move {
            let mut evaluator = t;
            let reference = &mut evaluator;
            if let Err(err) = reference.evaluate(&cloned_context, &cloned_env).await {
                if err.kind != ExecutionErrorKind::Aborted as i32 {
                    cloned_context.aborted.store(true, Ordering::Relaxed);
                    cloned_context.errors.push(err.clone());
                }
                return Err(err);
            }
            Ok(())
        })
    }

    
    #[inline]
    pub fn may_raise_aborted_error(&self) -> Result<(), ExecutionError> {
        if self.aborted.load(Ordering::Relaxed) {
            Err(ExecutionError{
                kind : ExecutionErrorKind::Aborted as i32,
                message : "aborted".to_string(),
            })
        } else {
            Ok(())
        }
    }
}