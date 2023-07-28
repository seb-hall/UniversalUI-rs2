//  UniversalUI_Native - event.rs
//  created by sebhall on 28/07/2023
//  
//  UniversalUI_Native contains native platform functionality
//  for the UniversalUI framework, such as windowing, events
//  and file management.
//
//  event.rs declares functions for handling window events.

use crate::UniversalUI_Base::general::*;
use crate::UniversalUI_Base::debug::*;
use crate::UniversalUI_Base::geometry::*;

use crate::libc::*;

// Conditionally include the platform-specific modules
#[cfg(target_os = "windows")]
mod native_event {
    include!("win32/event.rs");
}

#[cfg(target_os = "linux")]
mod native_event {
    include!("x11/event.rs");
}

pub type WindowEventCallback = extern "C" fn(window_id: uID, event_type: i32, user_data: *mut std::ffi::c_void);

#[no_mangle]
pub extern "C" fn get_events() { 

    native_event::get_events();

}

#[no_mangle]
pub extern "C" fn registerWindowEventCallback(callback: WindowEventCallback, user_data: *mut std::ffi::c_void) { 

    println!("register callback");

    (callback)(0, 0, user_data);

}
