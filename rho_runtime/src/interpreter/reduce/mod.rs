use std::sync::Arc;


use super::*;


use model::*;

mod substitute;
pub mod eval_par;
mod eval_send;
mod eval_receive;
mod eval_expression;
mod eval_new;
mod eval_var;
mod eval_match;
mod environment;


#[cfg(test)] mod eval_receive_test;


use eval_expression::ParExpressionEvaluator;
use substitute::*;
pub use environment::*;

use crate::storage::Storage;

pub trait Evaluator<S, T = ()> 
    where S : Storage + std::marker::Send + std::marker::Sync, T : 'static
{
    fn evaluate(&mut self, context : &Arc<InterpreterContext<S>>, env : &Env) -> Result<T, ExecutionError>;
}








