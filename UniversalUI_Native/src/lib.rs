//  UniversalUI_Native - lib.rs
//  created by sebhall on 28/07/2023
//  
//  UniversalUI_Native contains native platform functionality
//  for the UniversalUI framework, such as windowing, events
//  and file management.
//
//  lib.rs declares the crate submodules and init function.

extern crate UniversalUI_Base;
extern crate libc;

use UniversalUI_Base::debug::*;

#[cfg_attr(windows, path = "win32/init.rs")]
mod init;

pub mod window;
pub mod event;

#[no_mangle]
pub extern "C" fn native_init() -> bool { 

    debug_info("Initialising UniversalUI_Native");

    if !init::init() {
        return false;
    }
    
    return true;
}