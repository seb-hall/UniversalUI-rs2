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


pub static mut DISPLAY: Option<*mut xlib::Display> = None;



pub fn init() -> bool { 

    debug_info("Starting Test: X11");

    // Open the display
    let display = unsafe {
        xlib::XOpenDisplay(ptr::null())
    };

    if display.is_null() {
        debug_critical("Test Failed: X11 - failed to get a display"); 
        return false;
    }

    unsafe {
    // Store the X11 display in the global variable
    DISPLAY = Some(display);
    }
        
    debug_info("Test OK: X11");

    return true;
}

