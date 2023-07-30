//  UniversalUI_Native - win32/event.rs
//  created by sebhall on 28/07/2023
//  
//  UniversalUI_Native contains native platform functionality
//  for the UniversalUI framework, such as windowing, events
//  and file management.
//
//  win32/event.rs provides functionality for polling win32 events.


use crate::UniversalUI_Base::general::*;
use crate::UniversalUI_Base::debug::*;
use crate::UniversalUI_Base::geometry::*;

use crate::libc::*;

use windows::{core::*, s};
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;

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

    unsafe {
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

}

pub fn set_global_callback(callback: WindowEventCallback) {
    unsafe {
        EVENT_CALLBACK = Some(callback);
    }
}

pub unsafe extern "system" fn wndproc(win32_window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {

    unsafe fn loword(x: u32) -> u16 {
        (x & 0xFFFF) as u16
    }

    unsafe fn hiword(x: u32) -> u16 {
        ((x >> 16) & 0xFFFF) as u16
    } 

    let callback = unsafe {
        match EVENT_CALLBACK {
            Some(callback) => callback,
            None => { 
                //  no callback bound, return default value
                return DefWindowProcW(win32_window, message, wparam, lparam);
            }
        }
    };

    let window: uID = win32_window.0 as uID;

    match message {
        WM_SIZE => {
            let width = loword(lparam.0 as u32);
            let height = hiword(lparam.0 as u32);
            (callback)(window, WINDOW_RESIZED, [width as uFloat, height as uFloat, 0.0, 0.0]);
        },
        WM_CLOSE => {
            (callback)(window, WINDOW_CLOSE_PRESSED, [0.0, 0.0, 0.0, 0.0]);
            // CHECK IF WINDOW SHOULD BE CLOSED - MAYBE ADD WINDOW CLOSE CALLBACK
            return LRESULT(0);
        },   
        _ => {
            
        }
    }
    
    return DefWindowProcW(win32_window, message, wparam, lparam);
}