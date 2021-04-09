use super::*;
use model::{ *, expr::* };


pub mod sub_par;

trait Substitute {
    fn substitute(self : Self, reducer : &DebruijnInterpreter, depth : i32, env : &Env) 
        -> Result<Self, ExecutionError> 
            where Self: std::marker::Sized;
    fn substitute_no_sort(self : Self, reducer : &DebruijnInterpreter, depth : i32, env : &Env) 
        -> Result<Self, ExecutionError> 
            where Self: std::marker::Sized;
}



