//  UniversalUI_Native - x11/event.rs
//  created by sebhall on 28/07/2023
//  
//  UniversalUI_Native contains native platform functionality
//  for the UniversalUI framework, such as windowing, events
//  and file management.
//
//  x11/event.rs provides functionality for polling x11 events.

use crate::UniversalUI_Base::general::*;
use crate::UniversalUI_Base::debug::*;
use crate::UniversalUI_Base::geometry::*;

extern crate x11;

use crate::libc::*;
use x11::xlib;
use std::ptr;


pub fn get_events() {

     // Open the display
     let display = unsafe {
        xlib::XOpenDisplay(ptr::null())
    };

    if display.is_null() {
        panic!("Failed to open X11 display.");
    }

    debug_info("got X11 display.");

    // Create an event to listen for
    let mut event = xlib::XEvent { pad: [0; 24] };


        // Process X11 events
        unsafe {
            xlib::XNextEvent(display, &mut event);

            debug_info("got next event");
        }

        // Break the loop on any KeyPress event
        if event.get_type() == xlib::KeyPress {
            debug_info("key press!");
            return;
        }

        unsafe { xlib::XCloseDisplay(display); }


}
