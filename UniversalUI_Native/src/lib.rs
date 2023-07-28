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

// Conditionally include the platform-specific modules
#[cfg(target_os = "windows")]
mod init {
    include!("win32/init.rs");
}

#[cfg(target_os = "linux")]
mod init {
    include!("./x11/init.rs");
}

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