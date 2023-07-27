use crate::base::*;
use crate::debug::*;
use crate::graphics::*;
use std::borrow::Borrow;
use std::ops::Deref;

pub use crate::winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
    window::WindowId,
};

use std::time::{Duration, Instant};

use std::collections::HashMap;


extern "C" {
    //pub fn run_call_c();
}

#[no_mangle]
pub unsafe extern "C" fn run() -> bool {
    internal_debug_info("running loop");

    let mut window_size_map: HashMap<WindowId, uSize> = HashMap::new();

    if let Some(event_loop) = crate::event_loop.take() {
        // Use the EventLoop here
        event_loop.run(move |event, event_loop, control_flow| {
            control_flow.set_wait();

            match event {
                Event::WindowEvent { event, window_id } => {
                    match event {
                        WindowEvent::CloseRequested => {
                            internal_debug_info(&format!("Window {} close button pressed", u64::from(window_id)));
                            println!("{}", u64::from(window_id));
                        }
                        WindowEvent::Resized(physicalSize) => {
                            let now = Instant::now();
                            let mut duration: Duration = now.elapsed();
                            internal_debug_info(&format!("Window {} resized {} {}", u64::from(window_id), physicalSize.width, physicalSize.height));
                            duration = now.elapsed();

                            window_size_map.insert(window_id, uSize {width: physicalSize.width as f32, height: physicalSize.height as f32});
                            //internal_debug_info(&format!("{} us", duration.as_micros()));
                            //configure_window_surface(u64::from(window_id), uSize { width: physicalSize.width as f32, height: physicalSize.height as f32});
                            //duration = now.elapsed();
                            //internal_debug_info(&format!("{} us", duration.as_micros()));
                            //render_window(u64::from(window_id));
                            //duration = now.elapsed();
                            //internal_debug_info(&format!("{} us", duration.as_micros()));
                        }
                        _ => (),
                    }
                }
                Event::RedrawRequested(window_id) => {
                    //internal_debug_info("window redraw requested");
                    let size = window_size_map.get(&window_id).unwrap();
                    render_window(u64::from(window_id), uSize { width: size.width, height: size.height});
                }
                _ => (),
            }
        });

        crate::event_loop = Some(event_loop);

    } else {
        // Handle the case when the EventLoop is not initialized.
    }

    //run_call_c();

    return true;
}
