//  UniversalUI_Native - window.rs
//  created by sebhall on 28/07/2023
//  
//  UniversalUI_Native contains native platform functionality
//  for the UniversalUI framework, such as windowing, events
//  and file management.
//
//  window.rs declares functions for creating and managing windows.

use crate::UniversalUI_Base::general::*;
use crate::UniversalUI_Base::debug::*;
use crate::UniversalUI_Base::geometry::*;

use crate::libc::*;

#[no_mangle]
pub extern "C" fn create_window(title: *const c_char, size: uSize) -> uID { 

    debug_info("Creating a new Window");
    
    return 0;
}

#[no_mangle]
pub extern "C" fn destroy_window(id: uID) { 

    debug_info("Destroying Window");

}


#[no_mangle]
pub extern "C" fn get_window_visibility(id: uID) -> bool { 

    debug_info("Getting window visibility");
    

    return false;
}

#[no_mangle]
pub extern "C" fn set_window_visibility(id: uID, visible: bool) { 

    debug_info("Setting window visibility");
    
}

#[no_mangle]
pub extern "C" fn get_window_title(id: uID) -> *const c_char { 

    debug_info("Getting window title");
    
    return std::ptr::null();
}

#[no_mangle]
pub extern "C" fn set_window_title(id: uID, title: *const c_char) { 

    debug_info("Setting window title");
    
}

#[no_mangle]
pub extern "C" fn get_window_size(id: uID) -> uSize { 

    debug_info("Getting window size");

    return uSize { width: 0.0, height: 0.0 };
    
}

#[no_mangle]
pub extern "C" fn set_window_size(id: uID, size: uSize) { 

    debug_info("Setting window size");
    
}

#[no_mangle]
pub extern "C" fn get_window_fullscreen(id: uID) -> bool { 

    debug_info("Setting window fullscreen");
    
    return false;
}

#[no_mangle]
pub extern "C" fn set_window_fullscreen(id: uID, fullscreen: bool) { 

    debug_info("Setting window fullscreen");
    
}