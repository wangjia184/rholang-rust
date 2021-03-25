
use std::ffi::{ CString }; 
use libc;


use super::bnfc;
use super::normalize;
use model::*;

pub fn build_ast_from_string(source : &str) -> NormalizeResult {
    let raw_source = match CString::new(source) {
        Err(e) => return NormalizeResult::from(e),
        Ok(s) => s.into_raw(),
    };
    let raw_mode = match CString::new("r") {
        Err(e) => return NormalizeResult::from(e),
        Ok(s) => s.into_raw(),
    };

    let file_handle : *mut libc::FILE;
    unsafe {
        let buf_ptr: *mut libc::c_void = raw_source as *mut libc::c_void;
        file_handle = libc::fmemopen( buf_ptr, source.len(), raw_mode);
    }

    let result = build_ast(file_handle);

    unsafe {
        if file_handle != 0 as *mut libc::FILE {
            libc::fclose(file_handle);
        }
        // retake ownership to avoid memory leak
        CString::from_raw(raw_source);
        CString::from_raw(raw_mode);
    }
    result
}


pub fn build_ast_from_file(filepath : &str) -> NormalizeResult {
    let raw_filepath = match CString::new(filepath) {
        Err(e) => return NormalizeResult::from(e),
        Ok(s) => s.into_raw(),
    };
    let raw_mode = match CString::new("r") {
        Err(e) => return NormalizeResult::from(e),
        Ok(s) => s.into_raw(),
    };

    let file_handle : *mut libc::FILE;
    unsafe {
        let buf_ptr: *mut libc::c_char = raw_filepath as *mut libc::c_char;
        file_handle = libc::fopen( buf_ptr, raw_mode);
    }

    let result = build_ast(file_handle);

    unsafe {
        if file_handle != 0 as *mut libc::FILE {
            libc::fclose(file_handle);
        }
        // retake ownership to avoid memory leak
        CString::from_raw(raw_filepath);
        CString::from_raw(raw_mode);
    }
    result
}

fn build_ast(file_handle : *mut libc::FILE) -> NormalizeResult {
    if file_handle == 0 as *mut libc::FILE {
        return NormalizeResult::new_fatal_error("null file pointer");
    }
    let proc : bnfc::Proc;
    unsafe {
        let io_file: *mut bnfc::FILE = file_handle as *mut _ as *mut bnfc::FILE;
        proc = bnfc::pProc(io_file);
    };

    normalize::from_root(proc)
}




pub fn parse(source : &str) -> Result<bnfc::Proc, std::ffi::NulError> {
    let raw_source = CString::new(source)?.into_raw();
    let raw_mode = CString::new("r")?.into_raw();
    let file_handle : *mut libc::FILE;
    let proc : bnfc::Proc;
    unsafe {
        let buf_ptr: *mut libc::c_void = raw_source as *mut libc::c_void;
        file_handle = libc::fmemopen( buf_ptr, source.len(), raw_mode);

        let io_file: *mut bnfc::FILE = file_handle as *mut _ as *mut bnfc::FILE;
        proc = bnfc::pProc(io_file);
        if file_handle != 0 as *mut libc::FILE {
            libc::fclose(file_handle);
        }
        // retake ownership to avoid memory leak
        CString::from_raw(raw_source);
        CString::from_raw(raw_mode);
    }
    unsafe{
        if proc != 0 as bnfc::Proc {
            let s = CString::from_raw(bnfc::showProc(proc));
            println!("{:?}", &s);
        }
    }
    Ok(proc)
}