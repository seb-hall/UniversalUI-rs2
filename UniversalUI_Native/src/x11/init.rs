//  UniversalUI_Native - x11/init.rs
//  created by sebhall on 28/07/2023
//  
//  UniversalUI_Native contains native platform functionality
//  for the UniversalUI framework, such as windowing, events
//  and file management.
//
//  x11/init.rs intialises and tests the x11 platform
//  implementation.

use crate::UniversalUI_Base::debug::*;

extern crate x11;

use x11::xlib;
use std::ptr;

pub fn init() -> bool { 

    // Open the display
    let display = unsafe {
        xlib::XOpenDisplay(ptr::null())
    };

    if display.is_null() {
        debug_critical("failed to get X11 Display!");
        return false;
    }

    unsafe { xlib::XCloseDisplay(display); }

    return true;
}

