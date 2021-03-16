

use std::rc::Rc;
use crate::model::rho_types::*;
use super::context::*;

pub mod proc;


pub struct NormalizeResult {
    // warning messages
    pub warnings : Vec<(SourcePosition, String)>,

    // error messages
    pub errors : Vec<(SourcePosition, String)>,
}


// Input data to the normalizer
pub struct ProcVisitInputs {
    pub par : Par,
    pub env : IndexMapChain,
    pub known_free : Rc<DeBruijnLevelMap>,
}

impl Default for ProcVisitInputs {
    fn default() -> Self { 
        ProcVisitInputs {
            par : Par::default(),
            env : IndexMapChain::empty(),
            known_free : Rc::new(DeBruijnLevelMap::empty()),
        }
    }
}

pub struct ProcVisitOutputs {
    pub par : Par,
    pub known_free : DeBruijnLevelMap,
}
