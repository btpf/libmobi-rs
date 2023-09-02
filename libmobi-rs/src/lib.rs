#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CString;

include!(concat!(env!("OUT_DIR"), "/bindings.rs")); 

#[derive(Debug)]
pub enum MOBI_RET {
    MOBI_SUCCESS = 0, /**< Generic success return value */
    MOBI_ERROR = 1, /**< Generic error return value */
    MOBI_PARAM_ERR = 2, /**< Wrong function parameter */
    MOBI_DATA_CORRUPT = 3, /**< Corrupted data */
    MOBI_FILE_NOT_FOUND = 4, /**< File not found */
    MOBI_FILE_ENCRYPTED = 5, /**< Unsupported encrypted data */
    MOBI_FILE_UNSUPPORTED = 6, /**< Unsupported document type */
    MOBI_MALLOC_FAILED = 7, /**< Memory allocation error */
    MOBI_INIT_FAILED = 8, /**< Initialization error */
    MOBI_BUFFER_END = 9, /**< Out of buffer error */
    MOBI_XML_ERR = 10, /**< XMLwriter error */
    MOBI_DRM_PIDINV = 11,  /**< Invalid DRM PID */
    MOBI_DRM_KEYNOTFOUND = 12,  /**< Key not found */
    MOBI_DRM_UNSUPPORTED = 13, /**< DRM support not included */
    MOBI_WRITE_FAILED = 14, /**< Writing to file failed */
    MOBI_DRM_EXPIRED = 15, /**< DRM expired */
    MOBI_DRM_RANDOM_ERR = 16 /* DRM random bytes generation failed */
}
impl From<u32> for MOBI_RET {
    fn from(value: u32) -> Self {
        match value {
            0 => MOBI_RET::MOBI_SUCCESS,
            1 => MOBI_RET::MOBI_ERROR,
            2 => MOBI_RET::MOBI_PARAM_ERR,
            3 => MOBI_RET::MOBI_DATA_CORRUPT,
            4 => MOBI_RET::MOBI_FILE_NOT_FOUND,
            5 => MOBI_RET::MOBI_FILE_ENCRYPTED,
            6 => MOBI_RET::MOBI_FILE_UNSUPPORTED,
            7 => MOBI_RET::MOBI_MALLOC_FAILED,
            8 => MOBI_RET::MOBI_INIT_FAILED,
            9 => MOBI_RET::MOBI_BUFFER_END,
            10 => MOBI_RET::MOBI_XML_ERR,
            11 => MOBI_RET::MOBI_DRM_PIDINV,
            12 => MOBI_RET::MOBI_DRM_KEYNOTFOUND,
            13 => MOBI_RET::MOBI_DRM_UNSUPPORTED,
            14 => MOBI_RET::MOBI_WRITE_FAILED,
            15 => MOBI_RET::MOBI_DRM_EXPIRED,
            16 => MOBI_RET::MOBI_DRM_RANDOM_ERR,
            _ => {
                // Handle the case when myNum does not match any of the defined enum variants
                panic!("Invalid enum variant");
            }
    }
}
}

pub fn convertToEpubWrapper(c_str_src: &str, c_str_dest: &str) -> String{
    let c_str_src = CString::new(c_str_src).unwrap();
    let c_str_dest = CString::new(c_str_dest).unwrap();
    let c_str_src = c_str_src.as_ptr() as *mut i8;
    let c_str_dest = c_str_dest.as_ptr() as *mut i8;
    let mut returnCode: ::std::os::raw::c_int = -2;
    unsafe{
        returnCode = convertToEpub(c_str_src, c_str_dest);
    }
    let s: MOBI_RET = MOBI_RET::from(returnCode as u32);
    return String::from(format!("libmobi returned: {:?}", s));
}

#[cfg(test)]
mod tests {

    use std::{ffi::CString, env::{self, consts::OS}};
    use crate::{convertToEpub, MOBI_RET, convertToEpubWrapper};
    
    #[test]
    fn libMobiFFI() {
        let output = convertToEpubWrapper("./book.azw3",
        "./");
        println!("Ignore any errors above. Check unit test right below this returns \"ok\"");

        let expectedTestOutput = format!("libmobi returned: {:?}", MOBI_RET::MOBI_ERROR);

        assert_eq!(output, expectedTestOutput)
    }
}
