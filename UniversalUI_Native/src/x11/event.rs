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

    let display = unsafe {
        // Open the display
        match crate::init::DISPLAY {
            Some(disp) => disp,
            None => { 
                debug_critical("Failed to unwrap X11 display");
                panic!(); 
            }
        }
    };

    unsafe {

        let mut event = xlib::XEvent { pad: [0; 24] };

        xlib::XNextEvent(display, &mut event);
        debug_info(&format!("Event: {}, Window ID {}", event.type_, event.any.window).as_str());
    }
    


}
