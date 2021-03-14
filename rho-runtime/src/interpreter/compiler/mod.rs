



mod context;
mod bnfc;
mod normalize;
mod builder;
mod errors;

pub use errors::CompliationError;

use context::*;
use normalize::*;


pub struct ProcVisitInputs {

}

pub fn test() {
    builder::build_ast("Nil | new X in { X!(2) }");
}