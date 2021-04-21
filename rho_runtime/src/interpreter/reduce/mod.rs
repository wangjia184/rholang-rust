use std::sync::Arc;


use super::*;
use async_trait::async_trait;


use model::*;

mod substitute;
pub mod eval_par;
mod eval_send;
mod eval_receive;
mod eval_expression;
mod eval_new;
mod environment;
mod hash_rand; use hash_rand::HashRand;

#[cfg(test)] mod eval_receive_test;


use eval_expression::AsyncParExpressionEvaluator;
use substitute::*;
pub use environment::*;

use crate::storage::Storage;

#[async_trait]
pub trait AsyncEvaluator<S> where S : Storage + std::marker::Send + std::marker::Sync {
    async fn evaluate(&mut self, context : &Arc<InterpreterContext<S>>, env : &Env) -> Result<(), ExecutionError>;
}








