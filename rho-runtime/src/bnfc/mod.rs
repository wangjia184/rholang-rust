
#![allow(warnings, unused)]

include!(concat!(env!("OUT_DIR"), "/bnfc_bindings.rs"));


use std::ffi::{ CStr, CString };

// https://bnfc.digitalgrammars.com/tutorial/bnfc-tutorial.html

pub fn parse(source: &str) -> std::result::Result<&str, std::str::Utf8Error> {
    
    
    let raw_source = CString::new(source).expect("CString::new failed").into_raw();
    unsafe {
        let mut proc = psProc(raw_source);
        let ret = showProc(proc);
        // retake ownership to avoid memory leak
        CString::from_raw(raw_source);

        let c_str = CStr::from_ptr(ret);

        return c_str.to_str();
    }

}