//  UniversalUI_Native - x11/window.rs
//  created by sebhall on 28/07/2023
//  
//  UniversalUI_Native contains native platform functionality
//  for the UniversalUI framework, such as windowing, events
//  and file management.
//
//  x11/window.rs provides functionality for creating and 
//  managing windows on the x11 platform.

use crate::UniversalUI_Base::general::*;
use crate::UniversalUI_Base::debug::*;
use crate::UniversalUI_Base::geometry::*;

extern crate x11;

use crate::libc::*;
use x11::xlib;
use x11::xlib::*;
use std::ptr;

pub fn create_window(title: *const c_char, size: uSize) -> uID { 

    // Open the display
    let display = unsafe {
        match crate::init::DISPLAY {
            Some(disp) => disp,
            None => { 
                debug_critical("Failed to unwrap X11 display");
                panic!(); 
            }
        }
    };
    

    // Get the root window
    let root_window = unsafe {
        xlib::XRootWindow(display, 0)
    };

    // Create a new window
    let window = unsafe {
        xlib::XCreateSimpleWindow(
            display,
            root_window,
            0, 0, 800, 600, // x, y, width, height
            0, 0, 0x00ff00ff,// border width and border color
        )
    };

    // Set the window title
    unsafe {
        xlib::XStoreName(display, window, title);
    }

    // Map the window on the screen
    unsafe {
        let result = unsafe { xlib::XMapWindow(display, window) };
        if result == 0 {
            debug_critical("Failed to map X11 display");
            panic!();
        }

        xlib::XSelectInput(display, window, 
            KeyPressMask | 
            ButtonPressMask |
            ButtonReleaseMask | 
            EnterWindowMask | 
            LeaveWindowMask |
            //PointerMotionMask | 
            FocusChangeMask |
            StructureNotifyMask |
            ExposureMask 
        );

    }
    
    return window.try_into().unwrap();


}

pub fn destroy_window(id: uID) {
    
    // Open the display
    let display = unsafe {
        match crate::init::DISPLAY {
            Some(disp) => disp,
            None => { 
                debug_critical("Failed to unwrap X11 display");
                panic!(); 
            }
        }
    };
    
    unsafe {
        xlib::XDestroyWindow(display, id.try_into().unwrap());
    }
}