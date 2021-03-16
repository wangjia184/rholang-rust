



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
    builder::build_ast("new deployerId(`rho:rchain:deployerId`), \n a2, stdout(`rho:io:stdout`), \n a3, a1, rl(`rho:registry:lookup`)  in {}");
}