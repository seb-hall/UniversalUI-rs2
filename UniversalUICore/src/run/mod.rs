use crate::base::*;
use crate::debug::*;
use crate::graphics::*;
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
            //control_flow.set_wait();

            match event {
                Event::WindowEvent { event, window_id } => {
                    match event {
                        WindowEvent::CloseRequested => {
                            internal_debug_info(&format!("Window {} close button pressed", u64::from(window_id)));
                            println!("{}", u64::from(window_id));
                        }
                        WindowEvent::Resized(physicalSize) => {
                            internal_debug_info(&format!("Window {} resized {} {}", u64::from(window_id), physicalSize.width, physicalSize.height));
                            configure_window_surface(u64::from(window_id), uSize { width: physicalSize.width as f32, height: physicalSize.height as f32});
                            render_window(u64::from(window_id));
                        }
                        _ => (),
                    }
                }
                Event::RedrawRequested(window_id) => {
                    //internal_debug_info("window redraw requested");
                    render_window(u64::from(window_id));
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
