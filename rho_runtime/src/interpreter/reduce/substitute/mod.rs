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
    fn substitute(self : Self, reducer : &DebruijnInterpreter, depth : i32, env : &Env) 
        -> Result<Self, ExecutionError> 
            where Self: std::marker::Sized;
    fn substitute_no_sort(self : Self, reducer : &DebruijnInterpreter, depth : i32, env : &Env) 
        -> Result<Self, ExecutionError> 
            where Self: std::marker::Sized;
}



