
include!(concat!(env!("OUT_DIR"), "/bnfc_bindings.rs"));



use std::ffi::{ CStr, CString }; // https://bnfc.digitalgrammars.com/tutorial/bnfc-tutorial.html


use super::errors::*;


pub fn build_ast(source: &str) -> std::result::Result<(), CompliationError> {
    
    let raw_source = CString::new(source)?.into_raw();
    unsafe {
        let mut proc = psProc(raw_source);
        //let ret = showProc(proc);
        // retake ownership to avoid memory leak
        CString::from_raw(raw_source);

        //let c_str = CStr::from_ptr(ret);
    }

    Ok(())

}