
use std::ffi::{ CStr, CString }; 
use std::os::raw::c_char;
use std::rc::Rc;
use std::mem;
use std::collections::{ HashSet, HashMap };



use model::*;

use super::context::*;
use super::bnfc;


mod rho_name;
mod rho_new;
mod rho_send;
mod rho_ground;
mod rho_par;

#[cfg(test)]
mod tests {
    include!("rho_new_test.rs");
    include!("rho_par_test.rs");
}

type RawProc = bnfc::Proc_;
type RawName = bnfc::Name_;
type RawGround = bnfc::Ground_;


pub fn from_root(p : bnfc::Proc) -> NormalizeResult {
    let mut normalizer = Normalizer::default();

    // unsafe{
    //     if p != 0 as bnfc::Proc {
    //         let s = CString::from_raw(bnfc::showProc(p));
    //         println!("{:?}", &s);
    //     }
    // }

    let mut result = NormalizeResult::default();
    match normalizer.normalize(p) {
        Err(e) => {
            result.compiliation_error = Some(e);
        },
        Ok(outputs) => {
            result.par = Some(outputs.par);
        },
    };
    result.syntax_errors = mem::replace( &mut normalizer.syntax_errors, Vec::new());
    result
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


pub struct NameVisitOutputs {
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
    pub syntax_errors : Vec<SyntaxError>,   

    traverse_completed : bool,
    proc_set : HashSet<bnfc::Proc>,

    pub par : Option<Par>,
}
impl Default for Normalizer {
    fn default() -> Self { 
        Normalizer {
            source_warnings : Vec::new(),
            syntax_errors : Vec::new(),
            environment : HashMap::new(),
            proc_set : HashSet::new(),
            traverse_completed : false,
            par : None,
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
            // we dont need care much about memory leak because now parser is running in a one-time process :)
        }
    }
}

impl Normalizer {

    fn normalize(&mut self, p : bnfc::Proc) -> Result<ProcVisitOutputs, CompiliationError> {
        let outputs = self.normalize_proc(p, &ProcVisitInputs::default())?;
        self.traverse_completed = true;
        Ok(outputs)
    }

    // traverse abstract syntax tree
    fn normalize_proc(&mut self, p : bnfc::Proc, input: &ProcVisitInputs) -> Result<ProcVisitOutputs, CompiliationError> {
        if p == 0 as bnfc::Proc {
            return Err(CompiliationError::new_null_pointer("proc_"));
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
    
            
            _ => Err(CompiliationError::new_unrecognized_token(proc.kind, proc.line_number, proc.char_number))
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