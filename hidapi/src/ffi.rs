#![allow(dead_code)]

use std::ffi::*;

#[repr(C)]
pub struct CHidApiVersion {
    pub major: c_int,
    pub minor: c_int,
    pub patch: c_int,
}

#[repr(C)]
pub struct CHidDevice {
    _data: [u8; 0],
    _marker:
        core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/* see https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs */

#[repr(C)]
pub struct CHidDeviceInfo {
    pub path: *const c_char,
    pub vendor_id: c_ushort,
    pub product_id: c_ushort,
    pub serial_number: *const wchar_t,
    pub release_number: c_ushort,
    pub manufacturer_string: *const wchar_t,
    pub product_string: *const wchar_t,
    pub usage_page: c_ushort,
    pub usage: c_ushort,
    pub interface_number: c_int,
    pub next: *const CHidDeviceInfo,
}

extern "C" {
    pub fn hid_init() -> c_int;
    pub fn hid_exit() -> c_int;
    pub fn hid_enumerate(vendor_id: c_int, product_id: c_int) -> *const CHidDeviceInfo;
    pub fn hid_free_enumeration(devs: *const CHidDeviceInfo);
    pub fn hid_open(vendor_id: c_int, product_id: c_int, serial_number: *const wchar_t) -> *mut CHidDevice;
    pub fn hid_open_path(path: *const c_char) -> *mut CHidDevice;
    pub fn hid_write(dev: *mut CHidDevice, data: *const c_uchar, length: size_t) -> c_int;
    pub fn hid_read_timeout(dev: *mut CHidDevice, data: *mut c_uchar, length: size_t, milliseconds: c_int) -> c_int;
    pub fn hid_read(dev: *mut CHidDevice, data: *mut c_uchar, length: size_t) -> c_int;
    pub fn hid_set_nonblocking(dev: *mut CHidDevice, nonblock: c_int) -> c_int;
    pub fn hid_send_feature_report(dev: *mut CHidDevice, data: *const c_uchar, length: size_t) -> c_int;
    pub fn hid_get_feature_report(dev: *mut CHidDevice, data: *mut c_uchar, length: size_t) -> c_int;
    pub fn hid_get_input_report(dev: *mut CHidDevice, data: *mut c_uchar, length: size_t) -> c_int;
    pub fn hid_close(dev: *mut CHidDevice);
    pub fn hid_get_manufacturer_string(dev: *mut CHidDevice, string: *mut wchar_t, maxlen: size_t) -> c_int;
    pub fn hid_get_product_string(dev: *mut CHidDevice, string: *mut wchar_t, maxlen: size_t) -> c_int;
    pub fn hid_get_serial_number_string(dev: *mut CHidDevice, string: *mut wchar_t, maxlen: size_t) -> c_int;
    pub fn hid_get_device_info(dev: *mut CHidDevice) -> *const CHidDeviceInfo;
    pub fn hid_get_indexed_string(dev: *mut CHidDevice, string_index: c_int, string: *mut wchar_t, maxlen: size_t) -> c_int;
    pub fn hid_get_report_descriptor(dev: *mut CHidDevice, buf: *mut c_uchar, buf_size: size_t) -> c_int;
    pub fn hid_error(dev: *mut CHidDevice) -> *const wchar_t;
    pub fn hid_api_version() -> *const CHidApiVersion;
    pub fn hid_api_verison_str() -> *const c_char;
}
