#![allow(non_snake_case)]

pub mod debug;

use debug::*;

#[no_mangle]
pub extern "C" fn core_init() -> bool {

    internal_debug_info("Initialising UniversalUICore");

    return false;
}




