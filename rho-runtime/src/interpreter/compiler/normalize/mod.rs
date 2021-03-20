
use std::ffi::{ CStr }; 
use std::os::raw::c_char;
use std::rc::Rc;
use std::mem;
use std::collections::{ HashSet, HashMap };



use crate::model::*;
use super::context::*;
use super::bnfc;
use super::errors::*;
pub use super::CompliationError;

mod rho_name;
mod rho_new;
mod rho_send;
mod rho_ground;
mod rho_par;

#[cfg(test)]
mod tests {
    include!("rho_new_test.rs");
}

type RawProc = bnfc::Proc_;
type RawName = bnfc::Name_;
type RawGround = bnfc::Ground_;


pub fn from_root(p : bnfc::Proc) -> Result<(), CompliationError>{
    let mut normalizer = Normalizer::default();
    normalizer.normalize(p)?;
    Ok(())
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
    pub chan : Par,
    pub known_free : Rc<DeBruijnLevelMap>,
    
}
impl Default for NameVisitOutputs {
    fn default() -> Self { 
        NameVisitOutputs {
            chan : Par::default(),
            known_free : Rc::new(DeBruijnLevelMap::empty()),
        }
    }
}



struct Normalizer {
    pub environment : HashMap<String, Par>,

    // warning messages
    pub source_warnings : Vec<(SourcePosition, String)>,

    // error messages
    pub syntax_errors : Vec<(SyntaxError, Option<SourcePosition>, Option<SourcePosition>)>,   

    traverse_completed : bool,
    proc_set : HashSet<bnfc::Proc>,
}
impl Default for Normalizer {
    fn default() -> Self { 
        Normalizer {
            source_warnings : Vec::new(),
            syntax_errors : Vec::new(),
            environment : HashMap::new(),
            proc_set : HashSet::new(),
            traverse_completed : false,
        }
    }
}

impl Drop for Normalizer {
    fn drop(&mut self) {
        if self.traverse_completed {
            let set = mem::replace(&mut self.proc_set, HashSet::new());
            for p in set {
                unsafe { libc::free(p as *mut libc::c_void); }
            }
        } else {
            //unimplemented!("call C function to traverse again to free");
        }
    }
}

impl Normalizer {

    fn normalize(&mut self, p : bnfc::Proc) -> Result<ProcVisitOutputs, CompliationError> {
        let outputs = match self.normalize_proc(p, &ProcVisitInputs::default()) {
            Err(e) => unimplemented!("{:?} : {}", e, e),
            Ok(o) => o,
        };
        //let outputs = self.normalize_proc(p, ProcVisitInputs::default())?;
        self.traverse_completed = true;
        Ok(outputs)
    }

    // traverse abstract syntax tree
    fn normalize_proc(&mut self, p : bnfc::Proc, input: &ProcVisitInputs) -> Result<ProcVisitOutputs, CompliationError> {
        if p == 0 as bnfc::Proc {
            return Err(CompliationError::NullPointer("proc_".to_string()));
        }
        
        self.proc_set.insert(p); // collect pointers to be freed 
    
        let proc = unsafe { *p };
    
        match proc.kind {
            bnfc::Proc__is_PGround => {
                self.normalize_ground(&proc, input)
            },
            bnfc::Proc__is_PPar => {
                self.normalize_par(&proc, input)
            },
            bnfc::Proc__is_PNew => {
                self.normalize_new(&proc, input)
            },
            bnfc::Proc__is_PSend => {
                self.normalize_send(&proc, input)
            },
    
            
            _ => Err(CompliationError::UnrecognizedToken(proc.kind))
        }        
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