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


// Conditionally include the platform-specific modules
#[cfg(target_os = "windows")]
mod native_window {
    include!("win32/window.rs");
}

#[cfg(target_os = "linux")]
mod native_window {
    include!("x11/window.rs");
}

#[no_mangle]
pub extern "C" fn create_window(title: *const c_char, size: uSize) -> uID { 

    debug_info("Creating a new Window");
    
    return native_window::create_window(title, size);
}

#[no_mangle]
pub extern "C" fn destroy_window(id: uID) { 

    debug_info("Destroying Window");
    native_window::destroy_window(id);
}


#[no_mangle]
pub extern "C" fn get_window_visibility(id: uID) -> bool { 

    debug_info("Getting window visibility");
    

    return native_window::get_window_visibility(id);
}

#[no_mangle]
pub extern "C" fn set_window_visibility(id: uID, visible: bool) { 

    debug_info("Setting window visibility");
    native_window::set_window_visibility(id, visible);
}

#[no_mangle]
pub extern "C" fn get_window_title(id: uID) -> *const c_char { 

    debug_info("Getting window title");
    
    return native_window::get_window_title(id);
}

#[no_mangle]
pub extern "C" fn set_window_title(id: uID, title: *const c_char) { 

    debug_info("Setting window title");
    native_window::set_window_title(id, title);
}

#[no_mangle]
pub extern "C" fn get_window_size(id: uID) -> uSize { 

    debug_info("Getting window size");

    return native_window::get_window_size(id);
    
}

#[no_mangle]
pub extern "C" fn set_window_size(id: uID, size: uSize) { 

    debug_info("Setting window size");
    native_window::set_window_size(id, size);
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