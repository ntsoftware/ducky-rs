use std::fmt;
use std::ffi::CStr;
use crate::ffi::{hid_error, CHidDevice};

pub struct HidError {
    dev: *mut CHidDevice,
}

impl HidError {
   pub fn new() -> Self {
       Self { dev: std::ptr::null_mut() }
   }
}

impl fmt::Display for HidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cstr = unsafe { CStr::from_ptr(hid_error(self.dev)) };
        write!(f, "{}", String::from_utf8_lossy(cstr.to_bytes()))
    }
}
