

use std::rc::Rc;
use defer::defer;


use crate::model::rho_types::*;
use super::context::*;
use super::bnfc;

mod rho_name;
mod rho_new;
mod rho_send;

type RhoProc = bnfc::Proc_;



pub fn from_root(p : bnfc::Proc){
    let mut normalizer = Normalizer::default();
    normalizer.normalize(p, ProcVisitInputs::default());
}


// Input data to the normalizer
struct ProcVisitInputs {
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

struct ProcVisitOutputs {
    pub par : Par,
    pub known_free : DeBruijnLevelMap,
}


struct NameVisitInputs {
     pub env : IndexMapChain,
    pub known_free : Rc<DeBruijnLevelMap>,
}

impl Default for NameVisitInputs {
    fn default() -> Self { 
        NameVisitInputs {
            env : IndexMapChain::empty(),
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
    pub warnings : Vec<(SourcePosition, String)>,

    // error messages
    pub errors : Vec<(SourcePosition, String)>,
}
impl Default for Normalizer {
    fn default() -> Self { 
        Normalizer {
            warnings : Vec::new(),
            errors : Vec::new(),
        }
    }
}

impl Normalizer {
    // traverse abstract syntax tree
    
    pub fn normalize(&mut self, p : bnfc::Proc, input: ProcVisitInputs) -> Option<ProcVisitOutputs> {
        if p == 0 as bnfc::Proc {
            return None; // NULL pointer
        }

        // note that even if error occurs, still we need complete the traverse to free all node's memory
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