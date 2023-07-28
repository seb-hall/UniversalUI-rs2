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

#[cfg_attr(windows, path = "win32/event.rs")]
pub mod native_event;

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
