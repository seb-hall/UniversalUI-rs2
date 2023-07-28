//  UniversalUI_Native - win32/window.rs
//  created by sebhall on 28/07/2023
//  
//  UniversalUI_Native contains native platform functionality
//  for the UniversalUI framework, such as windowing, events
//  and file management.
//
//  win32/window.rs provides functionality for creating and 
//  managing windows on the win32 platform.

use crate::UniversalUI_Base::general::*;
use crate::UniversalUI_Base::debug::*;
use crate::UniversalUI_Base::geometry::*;

use crate::libc::*;

use std::ffi::CStr;

use windows::{core::*, s};
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::*;

pub fn wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

pub fn create_window(title: *const c_char, size: uSize) -> uID { 

    fn get_instance() -> Result<HMODULE> {
        unsafe {
            let instance = GetModuleHandleW(None)?;
            return Ok(instance);
        }
    }

    let instance: HMODULE = match get_instance() {
        Ok(inst) => inst,
        Err(_) => {
            debug_critical("couldn't get instance handle!"); 
            return 0;
        }
    };

    unsafe {

        let window_class = wide_null("window");
            
        let win32_window: HWND = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            PCWSTR(window_class.as_ptr()),
            PCWSTR(wide_null(CStr::from_ptr(title).to_str().unwrap()).as_ptr()),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            size.width as i32,
            size.height as i32,
            None,
            None,
            instance,
            None,
        );

        ShowWindow(win32_window, SW_SHOW);

        return win32_window.0 as uID;

    }
    
    
}