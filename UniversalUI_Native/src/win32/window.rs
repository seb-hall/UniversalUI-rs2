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
use std::ffi::CString;

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

pub fn destroy_window(window: uID) {
    unsafe {
        DestroyWindow(HWND(window as isize));
    }
}

pub fn get_window_visibility(window: uID) -> bool {
    unsafe {
        return IsWindowVisible(HWND(window as isize)).into();
    }
}

pub fn set_window_visibility(window: uID, visibility: bool) {
    if visibility {
        unsafe { ShowWindow(HWND(window as isize), SW_SHOW); }
    } else {
        unsafe { ShowWindow(HWND(window as isize), SW_HIDE); }
    }
}

pub fn get_window_title(window: uID) -> *const c_char {
    unsafe {
        let mut buffer: [u16; 128] = [0; 128];
        let numCharacters: i32 = GetWindowTextW(HWND(window as isize), &mut buffer);
        
        
        let titleString: String = match String::from_utf16(&buffer[0..numCharacters as usize]) {
            Ok(val) => val,
            Err(_) => String::from(" ")
        };


        return CString::new(titleString.to_owned()).unwrap().into_raw();
    }
}

pub fn set_window_title(window: uID, title: *const c_char) {
    unsafe {
        SetWindowTextW(HWND(window as isize), PCWSTR(wide_null(CStr::from_ptr(title).to_str().unwrap()).as_ptr()));
    }
}

pub fn get_window_size(window: uID) -> uSize {

    unsafe {
        let mut rect: RECT = RECT::default();
        if(GetClientRect(HWND(window as isize), &mut rect)).into() {
            let raw_width = rect.right - rect.left;
            let raw_height = rect.bottom - rect.top;
            return uSize {
                width: raw_width as uFloat,
                height: raw_height as uFloat
            }
        } else {
            return uSize {
                width: 0.0,
                height: 0.0
            }
        }
    }

}

pub fn set_window_size(window: uID, size: uSize) {

    unsafe fn get_client_rect(window: uID) -> RECT {
        
        let mut client: RECT = RECT::default();

        if(GetClientRect(HWND(window as isize), &mut client)).into() {
            return client;
        }

        debug_error("failed to get client window rect");

        return client;
    }

    unsafe fn get_window_rect(window: uID) -> RECT {
        
        let mut rect: RECT = RECT::default();

        if(GetWindowRect(HWND(window as isize), &mut rect)).into() {
            return rect;
        }

        debug_error("failed to get window rect");
        return rect;
    }

    unsafe {
        
        let rect = get_window_rect(window);
        let client = get_client_rect(window);

        let padding_x = (rect.right - rect.left) - client.right;
        let padding_y = (rect.bottom - rect.top) - client.bottom; 

        if (SetWindowPos(HWND(window as isize), HWND_TOP, 0, 0, padding_x + size.width as i32, padding_y + size.height as i32, SWP_NOMOVE).into()) {
            return;
        }

        debug_error("failed to set window size");
    }


}