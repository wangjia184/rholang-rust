use super::*;
use model::expr::*;
use model::var::*;

pub mod sub_par;
pub mod sub_expr;
pub mod sub_match;
pub mod sub_new;
pub mod sub_bundle;
pub mod sub_receive;
pub mod sub_send;

pub trait Substitutable<S> where S : Storage + std::marker::Send + std::marker::Sync {
    fn substitute(&mut self, context : &InterpreterContext<S>, depth : i32, env : &Env) 
        -> Result<(), ExecutionError> 
            where Self: std::marker::Sized;
    fn substitute_no_sort(&mut self, context : &InterpreterContext<S>, depth : i32, env : &Env) 
        -> Result<(), ExecutionError> 
            where Self: std::marker::Sized;
}



