



mod context;
mod bnfc;
mod normalize;
mod builder;
mod errors;

pub use errors::CompliationError;

pub use context::*;
use normalize::*;


pub struct ProcVisitInputs {

}

pub fn test() {
    builder::build_ast("new a1, a2, a3 in {
        a1!(1)
    }");
}