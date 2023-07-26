use crate::debug::*;
use std::borrow::Borrow;
use std::ops::Deref;

pub use crate::winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};



extern "C" {
    pub fn run_call_c();
}

#[no_mangle]
pub unsafe extern "C" fn run() -> bool {
    internal_debug_info("running loop");

    if let Some(event_loop) = crate::event_loop.take() {
        // Use the EventLoop here
        event_loop.run(move |event, event_loop, control_flow| {
            control_flow.set_wait();

            match event {
                Event::WindowEvent { event, window_id } => {
                    match event {
                        WindowEvent::CloseRequested => {
                            println!("Window {window_id:?} has received the signal to close");
                            println!("{}", u64::from(window_id));
                        }
                        _ => (),
                    }
                }
                Event::RedrawRequested(window_id) => {
                    internal_debug_info("window redraw requested");
                }
                _ => (),
            }
        });

        crate::event_loop = Some(event_loop);
    } else {
        // Handle the case when the EventLoop is not initialized.
    }

    run_call_c();

    return true;
}
