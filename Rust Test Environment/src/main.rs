use std::ffi::CString;

use libmobi_rs::{convertToEpub, MOBI_RET};

fn main() {
    println!("Hello, world!");
    let mut returnCode: ::std::os::raw::c_int = -2;
    unsafe{
        let c_str_src = CString::new("./book.azw3").unwrap();
        let c_str_dest = CString::new("./").unwrap();

        let c_str_src = (c_str_src.as_ptr() as *mut i8);
        let c_str_dest = (c_str_dest.as_ptr() as *mut i8);
        returnCode = convertToEpub(c_str_src, c_str_dest)
    }
    let s: MOBI_RET = MOBI_RET::from(returnCode as u32);

    println!("libmobi returned: {:?}", s);
}

