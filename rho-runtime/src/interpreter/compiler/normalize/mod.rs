
use std::ffi::{ CStr }; 
use std::os::raw::c_char;
use std::rc::Rc;
use std::collections::HashMap;
use defer::defer;


use crate::model::*;
use super::context::*;
use super::bnfc;
use super::errors::*;
pub use super::CompliationError;

mod rho_name;
mod rho_new;
mod rho_send;

type RawProc = bnfc::Proc_;
type RawName = bnfc::Name_;


pub fn from_root(p : bnfc::Proc){
    let mut normalizer = Normalizer::default();
    normalizer.normalize(p, ProcVisitInputs::default());
}


// Input data to the normalizer
struct ProcVisitInputs {
    pub par : RhoPar,
    pub env : Rc<IndexMapChain>,
    pub known_free : Rc<DeBruijnLevelMap>,
}

impl Default for ProcVisitInputs {
    fn default() -> Self { 
        ProcVisitInputs {
            par : RhoPar::default(),
            env : Rc::new(IndexMapChain::empty()),
            known_free : Rc::new(DeBruijnLevelMap::empty()),
        }
    }
}

struct ProcVisitOutputs {
    pub par : RhoPar,
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
    pub par : RhoPar,
    pub known_free : Rc<DeBruijnLevelMap>,
    
}



struct Normalizer {
    // warning messages
    pub source_warnings : Vec<(SourcePosition, String)>,

    // error messages
    pub syntax_errors : Vec<(SyntaxError, Option<SourcePosition>, Option<SourcePosition>)>,

    pub faulty_errors : Vec<CompliationError>,

    pub environment : HashMap<String, Par>,
}
impl Default for Normalizer {
    fn default() -> Self { 
        Normalizer {
            source_warnings : Vec::new(),
            syntax_errors : Vec::new(),
            faulty_errors : Vec::new(),
            environment : HashMap::new(),
        }
    }
}

impl Normalizer {
    // traverse abstract syntax tree
    pub fn normalize(&mut self, p : bnfc::Proc, input: ProcVisitInputs) -> Option<ProcVisitOutputs> {
        if p == 0 as bnfc::Proc {
            self.faulty_errors.push(CompliationError::NullPointer("proc_".to_string()));
            return None;
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


    fn get_string(&mut self, raw_str : bnfc::String) -> Result<String, std::str::Utf8Error> {
        unsafe {
            let raw_pointer = raw_str as *const _ as *const c_char;
            CStr::from_ptr(raw_pointer).to_str().and_then( |s| {
                Ok(s.to_owned())
            })
        }
    }
}