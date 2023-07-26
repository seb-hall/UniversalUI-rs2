extern crate libc;

use libc::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn core_init() {

    println!("Initialising UniversalUI corelib");

}

#[no_mangle]
pub unsafe extern "C" fn debug_info(input_string: *const c_char) {
    let c_string: &CStr = unsafe { CStr::from_ptr(input_string) };
    println!("UUI-INFO: {}", c_string.to_str().unwrap());
}



