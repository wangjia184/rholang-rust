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

use substitute::*;
pub use environment::*;

#[async_trait]
pub trait AsyncEvaluator {
    async fn evaluate(&mut self, reducer : Arc<DebruijnInterpreter>, env : Env);
}


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
}