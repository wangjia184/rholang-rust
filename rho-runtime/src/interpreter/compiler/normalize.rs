use super::context::{ SourcePosition, IndexMapChain, DeBruijnLevelMap };
use super::bnfc;

use super::super::super::model::rho_types::*;


// Input data to the normalizer
pub struct ProcVisitInputs {
    pub par : Par,
    pub env : IndexMapChain,
    pub known_free : DeBruijnLevelMap,
}

pub struct ProcVisitOutputs {
    pub par : Par,
    pub known_free : DeBruijnLevelMap,
}



pub struct NormalizeResult {
    // warning messages
    pub warnings : Vec<(SourcePosition, String)>,

    // error messages
    pub errors : Vec<(SourcePosition, String)>,
}

impl NormalizeResult {
    fn normalize_proc(&mut self, p : bnfc::Proc, input : ProcVisitInputs)  {

    }
}
