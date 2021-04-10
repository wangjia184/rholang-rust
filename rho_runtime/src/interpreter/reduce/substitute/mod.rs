use super::*;
use model::expr::*;


pub mod sub_par;
pub mod sub_expr;
pub mod sub_match;
pub mod sub_new;
pub mod sub_send;
pub mod sub_receive;
pub mod sub_bundle;

trait Substitutable {
    fn substitute(&mut self, reducer : &DebruijnInterpreter, depth : i32, env : &Env) 
        -> Result<(), ExecutionError> 
            where Self: std::marker::Sized;
    fn substitute_no_sort(&mut self, reducer : &DebruijnInterpreter, depth : i32, env : &Env) 
        -> Result<(), ExecutionError> 
            where Self: std::marker::Sized;
}



