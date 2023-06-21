use crate::ffi::{hid_init, hid_exit};
use crate::error::HidError;

pub struct HidApi {}

impl HidApi {
    fn new() -> Result<Self, HidError> {
        if unsafe { hid_init() } == 0 {
            Ok(Self {})
        } else {
            Err(HidError::new())
        }
    }
}

impl Drop for HidApi {
    fn drop(&mut self) {
        unsafe { hid_exit(); }
    }
}
