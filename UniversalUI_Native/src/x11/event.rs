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

const UEVENT_NULL             : i32 = 0;
const WINDOW_CREATED          : i32 = 1;
const WINDOW_DESTROYED        : i32 = 2;
const WINDOW_MOVED            : i32 = 3;
const WINDOW_RESIZED          : i32 = 4;
const WINDOW_GAINED_FOCUS     : i32 = 5;
const WINDOW_LOST_FOCUS       : i32 = 6;
const WINDOW_NEEDS_REDRAW     : i32 = 7;
const WINDOW_CLOSE_PRESSED    : i32 = 8;
const CURSOR_MOVED            : i32 = 9;
const CURSOR_ENTERED          : i32 = 10;
const CURSOR_LEFT             : i32 = 11;
const BUTTON_DOWN             : i32 = 12;
const BUTTON_UP               : i32 = 13;
const KEY_DOWN                : i32 = 14;
const KEY_UP                  : i32 = 15;

pub type WindowEventCallback = extern "C" fn(window_id: uID, event_type: i32, parameters: [uFloat; 4]);

pub static mut EVENT_CALLBACK: Option<WindowEventCallback> = None;


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

pub fn set_global_callback(callback: WindowEventCallback) {
    unsafe {
        EVENT_CALLBACK = Some(callback);
    }
}

