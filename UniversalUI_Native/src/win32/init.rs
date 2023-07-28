//  UniversalUI_Native - win32/init.rs
//  created by sebhall on 28/07/2023
//  
//  UniversalUI_Native contains native platform functionality
//  for the UniversalUI framework, such as windowing, events
//  and file management.
//
//  win32/init.rs intialises and tests the win32 platform
//  implementation.

use crate::UniversalUI_Base::debug::*;

use windows::{core::*, s};
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::*;

pub fn wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}


pub fn init() -> bool {

    debug_info("Starting Test: Win32");

    fn get_instance() -> Result<HMODULE> {
        unsafe {
            let instance = GetModuleHandleW(None)?;
            return Ok(instance);
        }
    }

    fn create_class(instance: HMODULE) -> Result<WNDCLASSW> {
        unsafe {
            let window_class = wide_null("window");
            
            let wc = WNDCLASSW {
                hCursor: LoadCursorW(None, IDC_ARROW)?,
                hInstance: instance,
                lpszClassName: PCWSTR(window_class.as_ptr()),
                style: CS_HREDRAW | CS_VREDRAW | CS_OWNDC,
                lpfnWndProc: Some(wndproc),
                ..Default::default()
            };

            return Ok(wc);
        }
    }

    fn register_class(wc: WNDCLASSW) -> Result<bool> {
        unsafe {
            let atom = RegisterClassW(&wc);
            if atom == 0 {
                return Ok(false);
            }

            return Ok(true);
        }
    }


    let instance = match get_instance() {
        Ok(inst) => inst,
        Err(_) => {
            debug_critical("Test Failed: Win32 - couldn't get instance handle!"); 
            return false; 
        }
    };

    if instance.0 == 0 {
        return false;
    }

    let class = match create_class(instance) {
        Ok(wc) => wc,
        Err(_) => {
            debug_critical("Test Failed: Win32 - couldn't create a window class!"); 
            return false; 
        }
    };


    let _ = match register_class(class) {
        Ok(result) => if !result { return false; }
        Err(_) => {
            debug_critical("Test Failed: Win32 - couldn't register class!"); 
            return false; 
        }
    };

    debug_info("Test OK: Win32");

    return true;
}


unsafe extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT { 

    debug_info("an event occured!");
    
    return DefWindowProcW(window, message, wparam, lparam);
}