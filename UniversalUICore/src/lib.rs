#![allow(non_snake_case)]
#![allow(improper_ctypes_definitions)]

pub extern crate libc;
pub extern crate raw_window_handle;
pub extern crate winit;
pub extern crate lazy_static;




pub mod debug;
pub mod base;
pub mod window;
pub mod run;

use debug::*;
use base::*;

pub use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

pub use std::mem;
pub use std::sync::Mutex;
pub use std::sync::Arc;
pub use std::sync::RwLock;

pub use lazy_static::lazy_static;

//  globally visible so other functions can access it with
//  crate::event_loop
pub static mut event_loop: Option<EventLoop<()>> = None;


#[no_mangle]
pub extern "C" fn core_init() -> bool { 

    internal_debug_info("Initialising UniversalUICore");
    
    let e_loop = winit::event_loop::EventLoop::new();

    unsafe {
        event_loop = Some(e_loop);
    }
    
    return false;
}


#[no_mangle]
pub extern "C" fn check_point(mut point: uPoint) -> uPoint {

    println!("I got x: {} y: {}", point.x, point.y);

    point.x += 1.0;
    point.y += 1.0;

    return point;
}

