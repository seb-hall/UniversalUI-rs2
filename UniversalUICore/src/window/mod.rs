

mod window_structs;


use self::window_structs::*;
use crate::debug::*;

#[no_mangle]
pub unsafe extern "C" fn create_window(window_info: uWindowInfo) -> uWindowHandle {
    debug_info(window_info.title);
    return uWindowHandle { raw_handle: std::ptr::null_mut() };
}
