
pub use crate::winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};


mod window_structs;

pub use self::window_structs::*;
pub use crate::debug::*;
pub use crate::graphics::*;
pub use crate::base::*;
use std::mem;

#[no_mangle]
pub unsafe extern "C" fn create_window(window_info: uWindowInfo) -> uWindowHandle {

    unsafe {
        if let Some(ref mut event_loop) = crate::event_loop {
            
            let window = Window::new(&event_loop).unwrap();
            
            let mut window_id: u64 = u64::from(window.id());

            

            debug_info(window_info.title);
            println!("{}",window_id);

            unsafe {
                if !setup_for_window(window_id, window, uSize { width: window_info.size.width, height: window_info.size.height }) {
                    return 0;
                }
            }

            //mem::forget(window);
            

            return window_id;

        } else {
            // Handle the case when the EventLoop is not initialized.
        }
    }

    return 0 ;
}

#[no_mangle]
pub unsafe extern "C" fn is_window_valid(window_handle: uWindowHandle) -> bool {
    return true;
}
