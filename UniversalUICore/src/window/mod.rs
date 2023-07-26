
pub use crate::winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};


mod window_structs;

pub use self::window_structs::*;
pub use crate::debug::*;
use std::mem;

#[no_mangle]
pub unsafe extern "C" fn create_window(window_info: uWindowInfo) -> uWindowHandle {

    unsafe {
        if let Some(ref mut event_loop) = crate::event_loop {
            
            let window = Window::new(&event_loop).unwrap();
            
            let mut window_id: u64 = u64::from(window.id());

            mem::forget(window);

            debug_info(window_info.title);
            println!("{}",window_id);

            return uWindowHandle { raw_handle: window_id };

        } else {
            // Handle the case when the EventLoop is not initialized.
        }
    }

    return uWindowHandle { raw_handle: 0 };
}

#[no_mangle]
pub unsafe extern "C" fn is_window_valid(window_handle: uWindowHandle) -> bool {
    return true;
}
