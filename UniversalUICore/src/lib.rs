#![allow(non_snake_case)]
#![allow(improper_ctypes_definitions)]

pub extern crate libc;
pub extern crate raw_window_handle;


pub mod debug;
pub mod base;
pub mod window;


use debug::*;
use base::*;

#[no_mangle]
pub extern "C" fn core_init() -> bool {

    internal_debug_info("Initialising UniversalUICore");

    return false;
}


#[no_mangle]
pub extern "C" fn check_point(mut point: uPoint) -> uPoint {

    println!("I got x: {} y: {}", point.x, point.y);

    point.x += 1.0;
    point.y += 1.0;

    return point;
}

