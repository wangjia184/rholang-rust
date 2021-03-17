

use std::rc::Rc;
use defer::defer;


use crate::model::rho_types::*;
use super::context::*;
use super::bnfc;
pub use super::CompliationError;

mod rho_name;
mod rho_new;
mod rho_send;

type RhoProc = bnfc::Proc_;
type RhoName = bnfc::Name_;


pub fn from_root(p : bnfc::Proc){
    let mut normalizer = Normalizer::default();
    normalizer.normalize(p, ProcVisitInputs::default());
}


// Input data to the normalizer
struct ProcVisitInputs {
    pub par : Par,
    pub env : Rc<IndexMapChain>,
    pub known_free : Rc<DeBruijnLevelMap>,
}

impl Default for ProcVisitInputs {
    fn default() -> Self { 
        ProcVisitInputs {
            par : Par::default(),
            env : Rc::new(IndexMapChain::empty()),
            known_free : Rc::new(DeBruijnLevelMap::empty()),
        }
    }
}

struct ProcVisitOutputs {
    pub par : Par,
    pub known_free : DeBruijnLevelMap,
}


struct NameVisitInputs {
    pub env : Rc<IndexMapChain>,
    pub known_free : Rc<DeBruijnLevelMap>,
}

impl Default for NameVisitInputs {
    fn default() -> Self { 
        NameVisitInputs {
            env : Rc::new(IndexMapChain::empty()),
            known_free : Rc::new(DeBruijnLevelMap::empty()),
        }
    }
}


struct NameVisitOutputs {
    pub par : Par,
    pub known_free : DeBruijnLevelMap,
}



struct Normalizer {
    // warning messages
    pub source_warnings : Vec<(SourcePosition, String)>,

    // error messages
    pub source_errors : Vec<(SourcePosition, String)>,

    pub faulty_errors : Vec<CompliationError>,
}
impl Default for Normalizer {
    fn default() -> Self { 
        Normalizer {
            source_warnings : Vec::new(),
            source_errors : Vec::new(),
            faulty_errors : Vec::new(),
        }
    }
}

impl Normalizer {
    // traverse abstract syntax tree
    pub fn normalize(&mut self, p : bnfc::Proc, input: ProcVisitInputs) -> Option<ProcVisitOutputs> {
        if p == 0 as bnfc::Proc {
            return None; // NULL pointer
        }

        // note that even if error occurs, still we need complete traverse to free all node's memory
        defer( || unsafe { libc::free(p as *mut libc::c_void); } );
    
        let proc = unsafe { *p };
    
        match proc.kind {
            bnfc::Proc__is_PNew => {
                return self.normalize_new(&proc, &input);
            },
            bnfc::Proc__is_PSend => {
                return self.normalize_send(&proc, &input);
            },
    
            
            _ => { warn!("Unknown token {:?}", &proc.kind); }
        };
        
        None
    }
}